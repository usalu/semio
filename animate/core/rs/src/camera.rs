//! 📷 Scene cameras: static, moving, 3D, and zoomed views.

use crate::color::Color;
use mathematical_geometry::{Affine, Point, Vec2};

/// 📸 Base camera framing the scene.
#[derive(Clone, Debug)]
pub struct Camera {
    pub frame_center: Point,
    pub frame_width: f64,
    pub frame_height: f64,
    pub background: Color,
    pub transform: Affine,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            frame_center: Point::ZERO,
            frame_width: 14.0,
            frame_height: 8.0,
            background: Color::BLACK,
            transform: Affine::IDENTITY,
        }
    }
}

impl Camera {
    pub fn new(frame_width: f64, frame_height: f64) -> Self {
        Self {
            frame_width,
            frame_height,
            ..Self::default()
        }
    }

    pub fn pixel_coords_to_scene(&self, px: f64, py: f64, pixel_width: u32, pixel_height: u32) -> Point {
        let u = px / pixel_width as f64;
        let v = 1.0 - py / pixel_height as f64;
        let x = self.frame_center.x() + (u - 0.5) * self.frame_width;
        let y = self.frame_center.y() + (v - 0.5) * self.frame_height;
        self.transform * Point::new(x, y)
    }

    pub fn scene_to_pixel(&self, p: Point, pixel_width: u32, pixel_height: u32) -> (f64, f64) {
        let p = self.transform * p;
        let u = (p.x() - self.frame_center.x()) / self.frame_width + 0.5;
        let v = (p.y() - self.frame_center.y()) / self.frame_height + 0.5;
        (u * pixel_width as f64, (1.0 - v) * pixel_height as f64)
    }
}

/// 🎥 Camera that can pan and zoom over time.
#[derive(Clone, Debug)]
pub struct MovingCamera {
    pub camera: Camera,
    pub target_center: Point,
    pub target_width: f64,
}

impl MovingCamera {
    pub fn new(camera: Camera) -> Self {
        let target_center = camera.frame_center;
        let target_width = camera.frame_width;
        Self {
            camera,
            target_center,
            target_width,
        }
    }

    pub fn interpolate(&mut self, alpha: f64) {
        let a = alpha.clamp(0.0, 1.0);
        let c0 = self.camera.frame_center;
        let c1 = self.target_center;
        self.camera.frame_center = Point::new(
            c0.x() + (c1.x() - c0.x()) * a,
            c0.y() + (c1.y() - c0.y()) * a,
        );
        self.camera.frame_width = self.camera.frame_width + (self.target_width - self.camera.frame_width) * a;
        self.camera.frame_height = self.camera.frame_width * self.camera.frame_height / self.camera.frame_width.max(1e-9);
    }

    pub fn set_target(&mut self, center: Point, width: f64) {
        self.target_center = center;
        self.target_width = width;
    }
}

/// 🧊 Perspective camera for 3D scenes.
#[derive(Clone, Debug)]
pub struct ThreeDCamera {
    pub camera: Camera,
    pub phi: f64,
    pub theta: f64,
    pub distance: f64,
    pub gamma: f64,
}

impl ThreeDCamera {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            phi: 0.0,
            theta: -std::f64::consts::FRAC_PI_2,
            distance: 10.0,
            gamma: 0.0,
        }
    }

    pub fn project(&self, x: f64, y: f64, z: f64) -> Point {
        let cy = self.phi.cos();
        let sy = self.phi.sin();
        let ct = self.theta.cos();
        let st = self.theta.sin();
        let x1 = x * cy - z * sy;
        let z1 = x * sy + z * cy;
        let y1 = y * ct - z1 * st;
        let z2 = y * st + z1 * ct + self.distance;
        let scale = 1.0 / z2.max(0.1);
        Point::new(x1 * scale, y1 * scale)
    }
}

/// 🔍 Picture-in-picture zoomed camera region.
#[derive(Clone, Debug)]
pub struct ZoomedCamera {
    pub camera: Camera,
    pub zoom_factor: f64,
    pub display_corner: Vec2,
    pub display_size: (f64, f64),
}

impl ZoomedCamera {
    pub fn new(camera: Camera, zoom_factor: f64) -> Self {
        Self {
            camera,
            zoom_factor,
            display_corner: Vec2::new(1.0, 1.0),
            display_size: (3.0, 2.0),
        }
    }

    pub fn effective_frame_width(&self) -> f64 {
        self.camera.frame_width / self.zoom_factor.max(1e-9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_camera_interpolates_center() {
        let mut cam = MovingCamera::new(Camera::default());
        cam.set_target(Point::new(2.0, 2.0), 8.0);
        cam.interpolate(0.5);
        assert!(cam.camera.frame_center.x().abs() < 2.0);
    }

    #[test]
    fn three_d_camera_projects_finite_point() {
        let cam = ThreeDCamera::new(Camera::default());
        let p = cam.project(1.0, 1.0, 1.0);
        assert!(p.x().is_finite() && p.y().is_finite());
    }
}
