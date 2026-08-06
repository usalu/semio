//! 📐️ Plain-`f64` 2D/3D vectors and points — no external linear-algebra crate. Points and vectors
//! are kept as distinct newtypes (a point minus a point is a vector; a vector has no fixed origin)
//! so geometric code cannot silently add two points or translate a direction. Every operation here
//! is exact IEEE-754 arithmetic; tolerance-aware comparison lives in [`crate::brep::tolerance`].

// #region 🔖️Scalars

/// 📐️ True when `a` and `b` are within `ulps` representable steps of each other (handles the
/// `0.1+0.2 != 0.3` class of rounding noise without hiding genuinely different values).
pub fn nearly_equal_ulps(a: f64, b: f64, ulps: i64) -> bool {
    if a == b {
        return true;
    }
    if a.is_nan() || b.is_nan() {
        return false;
    }
    let ia = a.to_bits() as i64;
    let ib = b.to_bits() as i64;
    (ia - ib).unsigned_abs() as i64 <= ulps
}

/// 📐️ Normalizes an angle into `[0, 2π)`.
pub fn normalize_angle(theta: f64) -> f64 {
    let two_pi = std::f64::consts::TAU;
    let wrapped = theta % two_pi;
    if wrapped < 0.0 {
        wrapped + two_pi
    } else {
        wrapped
    }
}

/// 📐️ Signed angular difference `b - a`, wrapped into `(-π, π]`.
pub fn angle_diff(a: f64, b: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let d = (b - a) % std::f64::consts::TAU;
    if d > pi {
        d - std::f64::consts::TAU
    } else if d <= -pi {
        d + std::f64::consts::TAU
    } else {
        d
    }
}

// #endregion 🔖️Scalars

// #region 🔖️Vec2

/// 📐️ A free 2D direction/displacement (no fixed origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// 📐️ A located 2D point, e.g. a parametric-domain (u, v) coordinate.
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pnt2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
    pub fn dot(self, o: Vec2) -> f64 {
        self.x * o.x + self.y * o.y
    }
    /// 📐️ The z-component of the 3D cross product `(x,y,0) × (o.x,o.y,0)`; its sign is the
    /// orientation of the turn from `self` to `o`.
    pub fn cross(self, o: Vec2) -> f64 {
        self.x * o.y - self.y * o.x
    }
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn norm_sq(self) -> f64 {
        self.dot(self)
    }
    /// 📐️ Returns `None` when the vector is (numerically) zero-length rather than dividing by
    /// zero into a `NaN`/`inf` direction.
    pub fn normalized(self) -> Option<Vec2> {
        let n = self.norm();
        if n <= f64::EPSILON {
            None
        } else {
            Some(self * (1.0 / n))
        }
    }
    pub fn perp(self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }
    pub fn lerp(self, o: Vec2, t: f64) -> Vec2 {
        self * (1.0 - t) + o * t
    }
    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}
impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl Pnt2 {
    pub fn new(x: f64, y: f64) -> Self {
        Pnt2 { x, y }
    }
    pub fn to_vec(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn lerp(self, o: Pnt2, t: f64) -> Pnt2 {
        Pnt2::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t)
    }
    pub fn distance(self, o: Pnt2) -> f64 {
        (self - o).norm()
    }
    pub fn distance_sq(self, o: Pnt2) -> f64 {
        (self - o).norm_sq()
    }
}
impl std::ops::Sub for Pnt2 {
    type Output = Vec2;
    fn sub(self, o: Pnt2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}
impl std::ops::Add<Vec2> for Pnt2 {
    type Output = Pnt2;
    fn add(self, v: Vec2) -> Pnt2 {
        Pnt2::new(self.x + v.x, self.y + v.y)
    }
}

// #endregion 🔖️Vec2

// #region 🔖️Vec3

/// 📐️ A free 3D direction/displacement (no fixed origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 📐️ A located 3D point in model space.
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pnt3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn from_array(a: [f64; 3]) -> Self {
        Vec3::new(a[0], a[1], a[2])
    }
    pub fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn dot(self, o: Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn norm_sq(self) -> f64 {
        self.dot(self)
    }
    pub fn normalized(self) -> Option<Vec3> {
        let n = self.norm();
        if n <= f64::EPSILON {
            None
        } else {
            Some(self * (1.0 / n))
        }
    }
    pub fn lerp(self, o: Vec3, t: f64) -> Vec3 {
        self * (1.0 - t) + o * t
    }
    /// 📐️ Any unit vector orthogonal to `self` — used to seed orthonormal frames. Picks the
    /// world axis least aligned with `self` before cross-producting, so the result stays
    /// well-conditioned even when `self` is nearly axis-aligned.
    pub fn any_orthogonal(self) -> Vec3 {
        let ax = self.x.abs();
        let ay = self.y.abs();
        let az = self.z.abs();
        let seed = if ax <= ay && ax <= az {
            Vec3::X
        } else if ay <= az {
            Vec3::Y
        } else {
            Vec3::Z
        };
        self.cross(seed).normalized().unwrap_or(Vec3::X)
    }
    pub fn angle_to(self, o: Vec3) -> f64 {
        let cross = self.cross(o).norm();
        let dot = self.dot(o);
        cross.atan2(dot)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Pnt3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pnt3 { x, y, z }
    }
    pub fn from_array(a: [f64; 3]) -> Self {
        Pnt3::new(a[0], a[1], a[2])
    }
    pub fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn to_vec(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn lerp(self, o: Pnt3, t: f64) -> Pnt3 {
        Pnt3::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
    }
    pub fn distance(self, o: Pnt3) -> f64 {
        (self - o).norm()
    }
    pub fn distance_sq(self, o: Pnt3) -> f64 {
        (self - o).norm_sq()
    }
}
impl std::ops::Sub for Pnt3 {
    type Output = Vec3;
    fn sub(self, o: Pnt3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Add<Vec3> for Pnt3 {
    type Output = Pnt3;
    fn add(self, v: Vec3) -> Pnt3 {
        Pnt3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}
impl std::ops::Sub<Vec3> for Pnt3 {
    type Output = Pnt3;
    fn sub(self, v: Vec3) -> Pnt3 {
        Pnt3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

// #endregion 🔖️Vec3

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product_is_orthogonal_to_both_operands() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 4.0);
        let c = a.cross(b);
        assert!(c.dot(a).abs() < 1e-12);
        assert!(c.dot(b).abs() < 1e-12);
    }

    #[test]
    fn normalized_returns_none_for_zero_vector() {
        assert_eq!(Vec3::ZERO.normalized(), None);
        assert_eq!(Vec2::ZERO.normalized(), None);
    }

    #[test]
    fn normalized_has_unit_length() {
        let v = Vec3::new(3.0, 4.0, 0.0).normalized().unwrap();
        assert!((v.norm() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn any_orthogonal_is_perpendicular_for_all_axis_aligned_inputs() {
        for v in [Vec3::X, Vec3::Y, Vec3::Z, -Vec3::X, -Vec3::Y, -Vec3::Z, Vec3::new(1.0, 1.0, 1.0)] {
            let o = v.any_orthogonal();
            assert!(v.dot(o).abs() < 1e-9, "not orthogonal for {v:?}: dot={}", v.dot(o));
            assert!((o.norm() - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn vec2_cross_sign_matches_turn_direction() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert!(a.cross(b) > 0.0);
        assert!(b.cross(a) < 0.0);
    }

    #[test]
    fn point_minus_point_is_vector_and_point_plus_vector_is_point() {
        let p = Pnt3::new(1.0, 2.0, 3.0);
        let q = Pnt3::new(4.0, 6.0, 8.0);
        let v: Vec3 = q - p;
        assert_eq!(v, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(p + v, q);
    }

    #[test]
    fn normalize_angle_wraps_into_0_tau() {
        assert!((normalize_angle(-0.1) - (std::f64::consts::TAU - 0.1)).abs() < 1e-12);
        assert!((normalize_angle(std::f64::consts::TAU + 0.5) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn angle_diff_wraps_into_minus_pi_pi() {
        let d = angle_diff(0.1, std::f64::consts::TAU - 0.1);
        assert!((d - (-0.2)).abs() < 1e-9);
    }

    #[test]
    fn lerp_at_endpoints_returns_endpoints() {
        let a = Pnt3::new(0.0, 0.0, 0.0);
        let b = Pnt3::new(10.0, 20.0, 30.0);
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
    }

    mod quick {
        use super::*;

        #[test]
        fn cross_product_antisymmetric_on_random_vectors() {
            let mut rng = semio_framework_math::random::Rng::from_seed(1);
            for _ in 0..200 {
                let a = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let b = Vec3::new(rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0, rng.next_f64() * 20.0 - 10.0);
                let ab = a.cross(b);
                let ba = b.cross(a);
                assert!((ab + ba).norm() < 1e-9);
            }
        }
    }
}
// #endregion 🔖️Tests
