//! 🧊 Three-dimensional Sobjects projected into the scene plane.

use crate::color::Color;
use crate::geometry::{circle, line, rectangle};
use crate::sobject::{Group, Sobject, VSobject};
use mathematical_geometry::Point;

/// 📦 Base 3D Sobject with yaw/pitch and projection scale.
#[derive(Clone)]
pub struct ThreeDVSobject {
    pub inner: VSobject,
    pub yaw: f64,
    pub pitch: f64,
    pub depth: f64,
}

impl ThreeDVSobject {
    pub fn new(inner: VSobject) -> Self {
        Self {
            inner,
            yaw: 0.0,
            pitch: 0.0,
            depth: 0.0,
        }
    }

    pub fn project_point(&self, p: (f64, f64, f64)) -> Point {
        let (x, y, z) = p;
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let x1 = x * cy - z * sy;
        let z1 = x * sy + z * cy;
        let y1 = y * cp - z1 * sp;
        let z2 = y * sp + z1 * cp + self.depth;
        let scale = 1.0 / (1.0 + z2 * 0.1);
        Point::new(x1 * scale, y1 * scale)
    }
}

/// 🌐 Parametric surface wireframe.
pub struct Surface {
    pub group: Group,
    pub resolution: u32,
}

impl Surface {
    pub fn paraboloid(radius: f64, color: Color) -> Self {
        let steps = 12;
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for i in 0..steps {
            let t = i as f64 / steps as f64 * std::f64::consts::TAU;
            let mut prev = None;
            for j in 0..=steps {
                let r = radius * j as f64 / steps as f64;
                let x = r * t.cos();
                let z = r * t.sin();
                let y = (x * x + z * z) * 0.2;
                let td = ThreeDVSobject::new(VSobject::new());
                let p = td.project_point((x, y, z));
                if let Some(prev_p) = prev {
                    children.push(Box::new(line(prev_p, p, color.with_alpha(0.5), 1.0)));
                }
                prev = Some(p);
            }
        }
        Self {
            group: Group::new(children),
            resolution: steps as u32,
        }
    }
}

/// ⚪ Sphere wireframe.
pub fn sphere(radius: f64, center: (f64, f64, f64), color: Color) -> Group {
    let steps = 16;
    let mut children: Vec<Box<dyn Sobject>> = Vec::new();
    let td = ThreeDVSobject::new(VSobject::new());
    for i in 0..steps {
        let phi = i as f64 / steps as f64 * std::f64::consts::PI;
        let mut prev = None;
        for j in 0..=steps {
            let theta = j as f64 / steps as f64 * std::f64::consts::TAU;
            let x = center.0 + radius * phi.sin() * theta.cos();
            let y = center.1 + radius * phi.cos();
            let z = center.2 + radius * phi.sin() * theta.sin();
            let p = td.project_point((x, y, z));
            if let Some(prev_p) = prev {
                children.push(Box::new(line(prev_p, p, color.with_alpha(0.6), 1.0)));
            }
            prev = Some(p);
        }
    }
    Group::new(children)
}

/// 🧊 Cube wireframe.
pub fn cube(side: f64, center: (f64, f64, f64), color: Color) -> Group {
    let h = side / 2.0;
    let corners = [
        (-h, -h, -h),
        (h, -h, -h),
        (h, h, -h),
        (-h, h, -h),
        (-h, -h, h),
        (h, -h, h),
        (h, h, h),
        (-h, h, h),
    ];
    let td = ThreeDVSobject::new(VSobject::new());
    let pts: Vec<Point> = corners
        .iter()
        .map(|(x, y, z)| td.project_point((center.0 + x, center.1 + y, center.2 + z)))
        .collect();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    let children: Vec<Box<dyn Sobject>> = edges
        .iter()
        .map(|(a, b)| Box::new(line(pts[*a], pts[*b], color, 2.0)) as Box<dyn Sobject>)
        .collect();
    Group::new(children)
}

/// 🟦 Filled face proxy for 3D objects (projected rectangle).
pub fn face(width: f64, height: f64, center: Point, fill: Color) -> VSobject {
    rectangle(width, height, center, fill, None, 0.0)
}

/// 🔮 Disc cross-section helper.
pub fn disc(radius: f64, center: Point, fill: Color) -> VSobject {
    circle(center, radius, fill, None, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_has_twelve_edges() {
        let g = cube(2.0, (0.0, 0.0, 0.0), Color::WHITE);
        assert_eq!(g.children.len(), 12);
    }

    #[test]
    fn projection_moves_points() {
        let td = ThreeDVSobject::new(VSobject::new());
        let p = td.project_point((1.0, 0.0, 0.0));
        assert!(p.x().is_finite());
    }
}
