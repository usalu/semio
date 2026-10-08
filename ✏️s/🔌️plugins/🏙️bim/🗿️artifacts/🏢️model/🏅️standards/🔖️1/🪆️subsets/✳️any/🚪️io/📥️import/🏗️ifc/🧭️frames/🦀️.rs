//! 🧭️ Rigid placements of the IFC import: the composition and inversion of `IfcLocalPlacement` chains and bulge recovery for trimmed circles.

/// 📐️ A right-handed rigid placement: an origin and three orthonormal axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rigid {
    pub origin: [f64; 3],
    pub x: [f64; 3],
    pub y: [f64; 3],
    pub z: [f64; 3],
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit(a: [f64; 3]) -> Option<[f64; 3]> {
    let length = dot(a, a).sqrt();
    (length > 1e-12).then(|| [a[0] / length, a[1] / length, a[2] / length])
}

impl Rigid {
    /// 🌍️ The identity placement.
    pub const IDENTITY: Rigid = Rigid { origin: [0.0; 3], x: [1.0, 0.0, 0.0], y: [0.0, 1.0, 0.0], z: [0.0, 0.0, 1.0] };

    /// 📐️ The placement of an `IfcAxis2Placement3D`: `axis` is the local z (default +z), `ref_direction` the local x made orthogonal to it.
    pub fn from_axes(origin: [f64; 3], axis: Option<[f64; 3]>, ref_direction: Option<[f64; 3]>) -> Self {
        let z = axis.and_then(unit).unwrap_or([0.0, 0.0, 1.0]);
        let seed = ref_direction.unwrap_or(if z[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] });
        let along = dot(seed, z);
        let x = unit([seed[0] - along * z[0], seed[1] - along * z[1], seed[2] - along * z[2]]).unwrap_or([1.0, 0.0, 0.0]);
        Self { origin, x, y: cross(z, x), z }
    }

    /// 📍️ A local point in the parent frame.
    pub fn point(&self, local: [f64; 3]) -> [f64; 3] {
        [0, 1, 2].map(|row| self.origin[row] + self.x[row] * local[0] + self.y[row] * local[1] + self.z[row] * local[2])
    }

    /// 🧭️ A local vector in the parent frame.
    pub fn vector(&self, local: [f64; 3]) -> [f64; 3] {
        [0, 1, 2].map(|row| self.x[row] * local[0] + self.y[row] * local[1] + self.z[row] * local[2])
    }

    /// 🔗️ This placement moved by `frame`: the placement of `self` (relative to `frame`) in the space `frame` itself lives in.
    pub fn transformed_by(&self, frame: &Rigid) -> Rigid {
        Rigid { origin: frame.point(self.origin), x: frame.vector(self.x), y: frame.vector(self.y), z: frame.vector(self.z) }
    }

    /// 🔁️ The inverse placement.
    pub fn inverse(&self) -> Rigid {
        let rows = Rigid { origin: [0.0; 3], x: [self.x[0], self.y[0], self.z[0]], y: [self.x[1], self.y[1], self.z[1]], z: [self.x[2], self.y[2], self.z[2]] };
        let moved = rows.vector(self.origin);
        Rigid { origin: [-moved[0], -moved[1], -moved[2]], ..rows }
    }

    /// 📍️ This world placement expressed relative to `base` (also a world placement).
    pub fn relative_to(&self, base: &Rigid) -> Rigid {
        self.transformed_by(&base.inverse())
    }

    /// 🧭️ The angle of the local x axis in the plane, counter-clockwise from +X.
    pub fn heading(&self) -> f64 {
        snap(self.x[1].atan2(self.x[0]))
    }
}

/// 🧲️ A derived quantity (bulge, angle, arc length) snapped to a micro-unit: the file stores reals rounded to 1e-9, so values recovered through trigonometry carry noise that would otherwise flip the last written digit on re-export.
pub fn snap(value: f64) -> f64 {
    (value * 1e6).round() / 1e6
}

/// 🌙️ The bulge `tan(sweep / 4)` of the arc from `start` to `end` on the circle around `centre`, counter-clockwise when `ccw`.
pub fn bulge_of(centre: [f64; 2], start: [f64; 2], end: [f64; 2], ccw: bool) -> f64 {
    let (from, to) = ((start[1] - centre[1]).atan2(start[0] - centre[0]), (end[1] - centre[1]).atan2(end[0] - centre[0]));
    let tau = std::f64::consts::TAU;
    let sweep = if ccw { (to - from).rem_euclid(tau) } else { -((from - to).rem_euclid(tau)) };
    let sweep = if sweep == 0.0 { if ccw { tau } else { -tau } } else { sweep };
    snap((sweep / 4.0).tan())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
