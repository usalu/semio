//! 📐️ The frame of a vertical view: the plane of a section or elevation as the coordinate system the drawing lives in. The plane stands on a line of the plan from `start` to `end`;
//! `u` runs along that line, `z` is the elevation above the building datum and `w` is the distance behind the plane, measured along the viewing direction, which is the left normal of
//! the line (the viewer stands on its right). A point is drawn at `(u, z)` and is the nearer to the viewer the smaller its `w`.

use crate::standards::v1::subsets::any::schema::inferences::element_solids::SolidBounds;
use crate::ViewPlane;

/// 🧭️ A point or vector in building coordinates `(x, y, z)`.
pub type World = [f64; 3];

/// 🖼️ A point in view coordinates `(u, z, w)`.
pub type Drawn = [f64; 3];

/// 🖼️ The coordinate system of a section or elevation plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub origin: [f64; 2],
    pub along: [f64; 2],
    pub look: [f64; 2],
    pub length: f64,
}

impl Frame {
    /// 🖼️ The frame of `plane`, none when the plane has no length.
    pub fn of(plane: &ViewPlane) -> Option<Self> {
        let (dx, dy) = (plane.end.x - plane.start.x, plane.end.y - plane.start.y);
        let length = dx.hypot(dy);
        (length.is_finite() && length > 1e-9).then(|| Self { origin: [plane.start.x, plane.start.y], along: [dx / length, dy / length], look: [-dy / length, dx / length], length })
    }

    /// ➡️ The distance of `point` along the plane from its start.
    pub fn u(&self, point: World) -> f64 {
        (point[0] - self.origin[0]) * self.along[0] + (point[1] - self.origin[1]) * self.along[1]
    }

    /// 🔭️ The distance of `point` behind the plane, negative in front of it.
    pub fn w(&self, point: World) -> f64 {
        (point[0] - self.origin[0]) * self.look[0] + (point[1] - self.origin[1]) * self.look[1]
    }

    /// 🖼️ The view coordinates of a building point.
    pub fn draw(&self, point: World) -> Drawn {
        [self.u(point), point[2], self.w(point)]
    }

    /// 🧭️ The viewing direction as a building vector.
    pub fn look3(&self) -> World {
        [self.look[0], self.look[1], 0.0]
    }

    /// 📦️ The ranges `(u_min, u_max, w_min, w_max)` the eight corners of `bounds` cover in view coordinates.
    pub fn ranges(&self, bounds: &SolidBounds) -> (f64, f64, f64, f64) {
        let corners = [bounds.min.x, bounds.max.x].into_iter().flat_map(|x| [bounds.min.y, bounds.max.y].into_iter().map(move |y| [x, y, 0.0]));
        corners.fold((f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY), |(u0, u1, w0, w1), corner| {
            let (u, w) = (self.u(corner), self.w(corner));
            (u0.min(u), u1.max(u), w0.min(w), w1.max(w))
        })
    }

    /// 🔭️ Whether `bounds` reaches into the slab of the view: along the plane within its length and behind it within `depth`.
    pub fn sees(&self, bounds: &SolidBounds, depth: f64) -> bool {
        let (u0, u1, w0, w1) = self.ranges(bounds);
        u1 >= 0.0 && u0 <= self.length && w1 >= 0.0 && w0 <= depth
    }

    /// ✂️ Whether `bounds` straddles the plane itself.
    pub fn crosses(&self, bounds: &SolidBounds) -> bool {
        let (u0, u1, w0, w1) = self.ranges(bounds);
        u1 >= 0.0 && u0 <= self.length && w0 <= 0.0 && w1 >= 0.0
    }
}
