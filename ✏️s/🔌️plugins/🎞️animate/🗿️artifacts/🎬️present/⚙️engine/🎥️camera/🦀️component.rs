//! 🎞️ Animate engine facet: 🎥️camera

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod camera {
    //! 📷️ Scene cameras: static, moving, 3D, and zoomed views.

    use crate::artifacts::present::engine::animate::color::Color;
    use math::geometry::{Affine, Point, Vec2};

    /// 📸️ Base camera framing the scene.
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
            Self { frame_center: Point::ZERO, frame_width: 14.0, frame_height: 8.0, background: Color::BLACK, transform: Affine::IDENTITY }
        }
    }

    impl Camera {
        pub fn new(frame_width: f64, frame_height: f64) -> Self {
            Self { frame_width, frame_height, ..Self::default() }
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

    /// 🎥️ Camera that can pan and zoom over time.
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
            Self { camera, target_center, target_width }
        }

        pub fn interpolate(&mut self, alpha: f64) {
            let a = alpha.clamp(0.0, 1.0);
            let c0 = self.camera.frame_center;
            let c1 = self.target_center;
            self.camera.frame_center = Point::new(c0.x() + (c1.x() - c0.x()) * a, c0.y() + (c1.y() - c0.y()) * a);
            self.camera.frame_width = self.camera.frame_width + (self.target_width - self.camera.frame_width) * a;
            self.camera.frame_height = self.camera.frame_width * self.camera.frame_height / self.camera.frame_width.max(1e-9);
        }

        pub fn set_target(&mut self, center: Point, width: f64) {
            self.target_center = center;
            self.target_width = width;
        }
    }

    /// 🧊️ Perspective camera for 3D scenes.
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
            Self { camera, phi: 0.0, theta: -std::f64::consts::FRAC_PI_2, distance: 10.0, gamma: 0.0 }
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

    /// 🔍️ Picture-in-picture zoomed camera region.
    #[derive(Clone, Debug)]
    pub struct ZoomedCamera {
        pub camera: Camera,
        pub zoom_factor: f64,
        pub display_corner: Vec2,
        pub display_size: (f64, f64),
    }

    impl ZoomedCamera {
        pub fn new(camera: Camera, zoom_factor: f64) -> Self {
            Self { camera, zoom_factor, display_corner: Vec2::new(1.0, 1.0), display_size: (3.0, 2.0) }
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
}

pub mod matrix {
    //! 🔢️ Matrix types as gridded Sobject groups.

    use crate::artifacts::present::engine::animate::color::Color;
    use crate::artifacts::present::engine::animate::geometry::rectangle;
    use crate::artifacts::present::engine::animate::sobject::{arrange, Group, Sobject};
    use crate::artifacts::present::engine::animate::text::{MathText, Text};
    use math::geometry::{Point, Vec2};

    fn arrange_grid(group: &mut Group, rows: usize, cols: usize, cell_size: (f64, f64)) {
        if group.children.is_empty() || rows == 0 || cols == 0 {
            return;
        }
        let origin = group.children[0].center();
        for (idx, child) in group.children.iter_mut().enumerate() {
            let row = idx / cols;
            let col = idx % cols;
            let x = origin.x() + col as f64 * cell_size.0;
            let y = origin.y() - row as f64 * cell_size.1;
            child.move_to(Point::new(x, y));
        }
    }

    /// 📊️ Matrix of string entries with optional brackets.
    pub struct Matrix {
        pub group: Group,
        pub rows: usize,
        pub cols: usize,
    }

    impl Matrix {
        pub fn from_rows(rows: Vec<Vec<String>>, cell_size: (f64, f64), color: Color) -> Self {
            let nrows = rows.len();
            let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for row in rows {
                for cell in row {
                    let t = Text::new(cell, color);
                    children.push(Box::new(t.inner));
                }
            }
            let mut group = Group::new(children);
            arrange_grid(&mut group, nrows, ncols, cell_size);
            Self { group, rows: nrows, cols: ncols }
        }

        pub fn math(entries: &[&str], cell_size: (f64, f64), color: Color) -> Self {
            let children: Vec<Box<dyn Sobject>> = entries
                .iter()
                .map(|e| {
                    let m = MathText::new(*e, color);
                    Box::new(m.inner) as Box<dyn Sobject>
                })
                .collect();
            let cols = (entries.len() as f64).sqrt().ceil() as usize;
            let rows = entries.len().div_ceil(cols);
            let mut group = Group::new(children);
            arrange(&mut group, Vec2::new(1.0, 0.0), cell_size.0 * 0.15);
            Self { group, rows, cols }
        }

        pub fn with_brackets(mut self, color: Color, padding: f64) -> Self {
            let b = self.group.bounds();
            let w = b.width() + padding * 2.0;
            let h = b.height() + padding * 2.0;
            let c = b.center();
            let frame = rectangle(w, h, c, Color::TRANSPARENT, Some(color), 3.0);
            self.group.add_child(Box::new(frame));
            self
        }
    }

    /// 📋️ Table with header row and body rows in a 2D grid.
    pub struct Table {
        pub group: Group,
        pub rows: usize,
        pub cols: usize,
    }

    impl Table {
        pub fn new(headers: Vec<String>, rows: &[Vec<String>], cell_size: (f64, f64), color: Color) -> Self {
            let ncols = headers.len().max(rows.iter().map(|r| r.len()).max().unwrap_or(0));
            let nrows = rows.len() + 1;
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for header in headers {
                children.push(Box::new(Text::new(header, color).inner));
            }
            for row in rows {
                for cell in row {
                    children.push(Box::new(Text::new(cell.clone(), color).inner));
                }
                let pad = ncols.saturating_sub(row.len());
                for _ in 0..pad {
                    children.push(Box::new(Text::new("", color).inner));
                }
            }
            let mut group = Group::new(children);
            arrange_grid(&mut group, nrows, ncols, cell_size);
            Self { group, rows: nrows, cols: ncols }
        }

        pub fn with_frame(mut self, color: Color, padding: f64) -> Self {
            let b = self.group.bounds();
            let frame = rectangle(b.width() + padding * 2.0, b.height() + padding * 2.0, b.center(), Color::TRANSPARENT, Some(color), 2.0);
            self.group.add_child(Box::new(frame));
            self
        }
    }

    /// 🧮️ Decimal matrix for numeric interpolation animations.
    #[derive(Clone, Debug)]
    pub struct DecimalMatrix {
        pub values: Vec<Vec<f64>>,
    }

    impl DecimalMatrix {
        pub fn new(values: Vec<Vec<f64>>) -> Self {
            Self { values }
        }

        pub fn lerp(&self, other: &Self, t: f64) -> Self {
            let rows = self.values.len().min(other.values.len());
            let mut out = Vec::with_capacity(rows);
            for r in 0..rows {
                let cols = self.values[r].len().min(other.values[r].len());
                let mut row = Vec::with_capacity(cols);
                for c in 0..cols {
                    let a = self.values[r][c];
                    let b = other.values[r][c];
                    row.push(a + (b - a) * t);
                }
                out.push(row);
            }
            Self { values: out }
        }

        pub fn to_matrix_sobject(&self, cell_size: (f64, f64), color: Color) -> Matrix {
            let rows: Vec<Vec<String>> = self.values.iter().map(|row| row.iter().map(|v| format!("{v:.2}")).collect()).collect();
            Matrix::from_rows(rows, cell_size, color)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn decimal_matrix_lerps() {
            let a = DecimalMatrix::new(vec![vec![0.0, 1.0]]);
            let b = DecimalMatrix::new(vec![vec![2.0, 3.0]]);
            let m = a.lerp(&b, 0.5);
            assert!((m.values[0][0] - 1.0).abs() < 1e-9);
        }

        #[test]
        fn matrix_grid_layout() {
            let m = Matrix::from_rows(vec![vec!["a".into(), "b".into()], vec!["c".into(), "d".into()]], (1.0, 1.0), Color::WHITE);
            assert_eq!(m.rows, 2);
            assert_eq!(m.cols, 2);
            assert_eq!(m.group.children.len(), 4);
        }

        #[test]
        fn table_has_header_and_rows() {
            let t = Table::new(vec!["x".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
            assert_eq!(t.rows, 2);
            assert_eq!(t.cols, 1);
        }

        #[test]
        fn table_with_frame_adds_border_child() {
            let t = Table::new(vec!["a".into(), "b".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
            let before = t.group.children.len();
            let framed = t.with_frame(Color::WHITE, 0.2);
            assert_eq!(framed.group.children.len(), before + 1);
        }

        #[test]
        fn matrix_math_lays_out_entries() {
            let m = Matrix::math(&["1", "2", "3", "4"], (1.0, 1.0), Color::WHITE);
            assert_eq!(m.group.children.len(), 4);
            assert_eq!(m.cols, 2);
            assert_eq!(m.rows, 2);
        }

        #[test]
        fn matrix_with_brackets_adds_frame_child() {
            let m = Matrix::from_rows(vec![vec!["a".into()]], (1.0, 1.0), Color::WHITE);
            let before = m.group.children.len();
            let bracketed = m.with_brackets(Color::WHITE, 0.1);
            assert_eq!(bracketed.group.children.len(), before + 1);
        }

        #[test]
        fn decimal_matrix_to_matrix_sobject_formats_values() {
            let d = DecimalMatrix::new(vec![vec![1.5, 2.25]]);
            let m = d.to_matrix_sobject((1.0, 1.0), Color::WHITE);
            assert_eq!(m.group.children.len(), 2);
        }
    }
}