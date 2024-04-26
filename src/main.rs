use z_console_canvas_lib::*;
use std::time::{Instant, Duration};
use random::*;


struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub dx: f32,
    pub dy: f32,
}

fn lerpf(min: f32, max: f32, t: f32) -> f32 {
    assert!(max > min);
    assert!(t >= 0.0000000000001f32 && t <= 1.0000000000000001f32);
    min + t * (max - min)
}

fn rand_float(rand: &mut Xorshift128Plus, min: f32, max: f32) -> f32 {
    let t: f32 = rand.read_f64() as f32;
    lerpf(min, max, t)
}

fn clampf(a: f32, min: f32, max: f32) -> f32 {
    if a < min { return min; }
    if a > max { return max; }
    a
}

impl Ball {
    pub fn random(
            rand: &mut Xorshift128Plus,
            max_x: f32,
            max_y: f32,
            max_r: f32,
            max_dx: f32,
            max_dy: f32
        ) -> Ball {
        
        let radius: f32 = rand_float(rand, 2f32, max_r);
        Ball {
            x: rand_float(rand, radius, max_x - radius),
            y: rand_float(rand, radius, max_y - radius),
            r: radius,
            dx: rand_float(rand, -1f32 * max_dx, max_dx),
            dy: rand_float(rand, -1f32 * max_dy, max_dy),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.fill_circle(self.x as usize, 
                           self.y as usize, 
                           self.r as usize, 
                           0xff0000
        );
    }

    pub fn update(&mut self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        let check_x: f32 = self.x + self.dx;
        let check_y: f32 = self.y + self.dy;
        if check_x - self.r <= min_x || check_x + self.r >= max_x {
            self.dx *= -1f32;
        }
        if check_y - self.r <= min_y || check_y + self.r >= max_y {
            self.dy *= -1f32;
            self.dx *= 0.9f32;
        }
        self.dy += 0.89f32;
        self.x += self.dx;
        self.x = clampf(self.x, min_x + self.r, max_x - self.r);
        self.y += self.dy;
        self.y = clampf(self.y, min_y + self.r, max_y - self.r);
    }
}

#[allow(unused_assignments)]
fn main() {
    const WIDTH: usize = 60;
    const HEIGHT: usize = 60;
    let mut canvas: Canvas = Canvas::new(WIDTH, HEIGHT); 
    let mut rand: Xorshift128Plus = random::default(69);
    unsafe {
        let r = core::arch::x86_64::_rdtsc();
        rand = random::default(r);
    }
    let mut ball: Ball = Ball::random(&mut rand,
                                      50f32,
                                      50f32,
                                      10f32,
                                      5f32,
                                      5f32
                                      );

    let mut old_time: Instant = std::time::Instant::now();
    loop {
        canvas.clear(0x000000);
        canvas.draw_rect(00, 00, 60, 60, 0xFFFFFF);
        

        let dt: Duration = old_time.elapsed();
        if dt > Duration::from_millis(1000/15) {
            ball.update(0f32, WIDTH as f32, 0f32, HEIGHT as f32);
            ball.draw(&mut canvas);
            canvas.render_canvas();
            old_time = std::time::Instant::now();
        }
    }
}
