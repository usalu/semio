//! 🧮 Linear algebra: 3D vector/matrix math for scenes and cameras, plus a 2D matrix for planar transforms.

// #region 🔖Vec3
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_array(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len < 1e-8 {
            return Self::ZERO;
        }
        self.scale(1.0 / len)
    }
}
// #endregion 🔖Vec3

// #region 🔖Mat4
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            cols: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let gl_z = (far + near) / (near - far);
        let gl_w = (2.0 * far * near) / (near - far);
        Self {
            cols: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, 0.5 * gl_z - 0.5, -1.0],
                [0.0, 0.0, 0.5 * gl_w, 0.0],
            ],
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = target.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Self {
            cols: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0],
            ],
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let mut out = Self::identity();
        for col in 0..4 {
            for row in 0..4 {
                out.cols[col][row] = self.cols[0][row] * other.cols[col][0]
                    + self.cols[1][row] * other.cols[col][1]
                    + self.cols[2][row] * other.cols[col][2]
                    + self.cols[3][row] * other.cols[col][3];
            }
        }
        out
    }

    pub fn transform_point(self, p: Vec3) -> Vec3 {
        let x = p.x * self.cols[0][0] + p.y * self.cols[1][0] + p.z * self.cols[2][0] + self.cols[3][0];
        let y = p.x * self.cols[0][1] + p.y * self.cols[1][1] + p.z * self.cols[2][1] + self.cols[3][1];
        let z = p.x * self.cols[0][2] + p.y * self.cols[1][2] + p.z * self.cols[2][2] + self.cols[3][2];
        let w = p.x * self.cols[0][3] + p.y * self.cols[1][3] + p.z * self.cols[2][3] + self.cols[3][3];
        if w.abs() < 1e-8 {
            return Vec3::new(x, y, z);
        }
        Vec3::new(x / w, y / w, z / w)
    }

    pub fn transform_direction(self, dir: Vec3) -> Vec3 {
        let x = dir.x * self.cols[0][0] + dir.y * self.cols[1][0] + dir.z * self.cols[2][0];
        let y = dir.x * self.cols[0][1] + dir.y * self.cols[1][1] + dir.z * self.cols[2][1];
        let z = dir.x * self.cols[0][2] + dir.y * self.cols[1][2] + dir.z * self.cols[2][2];
        Vec3::new(x, y, z).normalize()
    }

    /// 🧮 Full 4x4 inverse via Gauss-Jordan elimination on an augmented `[A | I]` matrix.
    /// Indexed as `a[row][col]`; `self.cols[c][r]` is read/written as `a[r][c]` throughout.
    pub fn inverse(self) -> Self {
        let mut a = [[0.0f32; 8]; 4];
        for row in 0..4 {
            for col in 0..4 {
                a[row][col] = self.cols[col][row];
            }
            a[row][4 + row] = 1.0;
        }
        for pivot in 0..4 {
            let (mut best_row, mut best_val) = (pivot, a[pivot][pivot].abs());
            for row in (pivot + 1)..4 {
                if a[row][pivot].abs() > best_val {
                    best_row = row;
                    best_val = a[row][pivot].abs();
                }
            }
            if best_val < 1e-8 {
                return Self::identity();
            }
            if best_row != pivot {
                a.swap(pivot, best_row);
            }
            let pivot_value = a[pivot][pivot];
            for col in 0..8 {
                a[pivot][col] /= pivot_value;
            }
            for row in 0..4 {
                if row == pivot {
                    continue;
                }
                let factor = a[row][pivot];
                if factor == 0.0 {
                    continue;
                }
                for col in 0..8 {
                    a[row][col] -= factor * a[pivot][col];
                }
            }
        }
        let mut inv = [[0.0f32; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                inv[col][row] = a[row][4 + col];
            }
        }
        Self { cols: inv }
    }

    pub fn translation(v: Vec3) -> Self {
        let mut m = Self::identity();
        m.cols[3] = [v.x, v.y, v.z, 1.0];
        m
    }

    pub fn scale_vec(v: Vec3) -> Self {
        Self {
            cols: [
                [v.x, 0.0, 0.0, 0.0],
                [0.0, v.y, 0.0, 0.0],
                [0.0, 0.0, v.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_quat(x: f32, y: f32, z: f32, w: f32) -> Self {
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        Self {
            cols: [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
                [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
                [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn to_cols_array(self) -> [f32; 16] {
        let mut out = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = self.cols[col][row];
            }
        }
        out
    }
}
// #endregion 🔖Mat4

// #region 🔖Mat2
/// 🧮 2x2 matrix, column-major storage; `new(a, b, c, d)` takes row-major entries of `[[a, b], [c, d]]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2 {
    pub cols: [[f64; 2]; 2],
}

impl Mat2 {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0], [0.0, 1.0]] };

    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { cols: [[a, c], [b, d]] }
    }

    pub fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1];
        Self { cols: [[entry(0, 0), entry(1, 0)], [entry(0, 1), entry(1, 1)]] }
    }

    pub fn apply(self, v: (f64, f64)) -> (f64, f64) {
        (self.cols[0][0] * v.0 + self.cols[1][0] * v.1, self.cols[0][1] * v.0 + self.cols[1][1] * v.1)
    }

    pub fn det(self) -> f64 {
        self.cols[0][0] * self.cols[1][1] - self.cols[1][0] * self.cols[0][1]
    }

    pub fn trace(self) -> f64 {
        self.cols[0][0] + self.cols[1][1]
    }

    pub fn transpose(self) -> Self {
        Self::new(self.cols[0][0], self.cols[0][1], self.cols[1][0], self.cols[1][1])
    }

    pub fn inverse(self) -> Option<Self> {
        let d = self.det();
        if d.abs() < 1e-12 {
            return None;
        }
        let inv_d = 1.0 / d;
        Some(Self::new(self.cols[1][1] * inv_d, -self.cols[1][0] * inv_d, -self.cols[0][1] * inv_d, self.cols[0][0] * inv_d))
    }

    /// 🧮 Real eigenvalues (if any) of a 2x2 matrix via the characteristic polynomial `λ² - tr·λ + det = 0`.
    pub fn eigenvalues(self) -> Option<(f64, f64)> {
        let t = self.trace();
        let d = self.det();
        let disc = t * t - 4.0 * d;
        if disc < 0.0 {
            return None;
        }
        let sq = disc.sqrt();
        Some(((t + sq) * 0.5, (t - sq) * 0.5))
    }
}
// #endregion 🔖Mat2

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_normalize_zero_stays_zero() {
        assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
    }

    #[test]
    fn vec3_cross_is_perpendicular() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert!((c.dot(a)).abs() < 1e-6);
        assert!((c.dot(b)).abs() < 1e-6);
        assert!((c.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn mat4_identity_transforms_point_unchanged() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        let out = Mat4::identity().transform_point(p);
        assert!((out.x - p.x).abs() < 1e-6 && (out.y - p.y).abs() < 1e-6 && (out.z - p.z).abs() < 1e-6);
    }

    #[test]
    fn mat4_inverse_round_trips_translation() {
        let m = Mat4::translation(Vec3::new(3.0, -2.0, 5.0));
        let inv = m.inverse();
        let p = Vec3::new(1.0, 1.0, 1.0);
        let round = inv.transform_point(m.transform_point(p));
        assert!((round.x - p.x).abs() < 1e-4 && (round.y - p.y).abs() < 1e-4 && (round.z - p.z).abs() < 1e-4);
    }

    #[test]
    fn mat2_identity_apply_is_noop() {
        assert_eq!(Mat2::IDENTITY.apply((3.0, -4.0)), (3.0, -4.0));
    }

    #[test]
    fn mat2_apply_matches_matrix_vector_multiply() {
        let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
        assert_eq!(m.apply((1.0, 1.0)), (2.0, 3.0));
    }

    #[test]
    fn mat2_det_of_scale_matrix() {
        let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
        assert!((m.det() - 6.0).abs() < 1e-9);
    }

    #[test]
    fn mat2_inverse_round_trips() {
        let m = Mat2::new(2.0, 1.0, 1.0, 3.0);
        let inv = m.inverse().expect("invertible");
        let round = m.mul(inv);
        assert!((round.cols[0][0] - 1.0).abs() < 1e-9);
        assert!((round.cols[1][1] - 1.0).abs() < 1e-9);
        assert!(round.cols[1][0].abs() < 1e-9);
        assert!(round.cols[0][1].abs() < 1e-9);
    }

    #[test]
    fn mat2_singular_matrix_has_no_inverse() {
        let m = Mat2::new(1.0, 2.0, 2.0, 4.0);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn mat2_transpose_swaps_off_diagonal() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let t = m.transpose();
        assert_eq!(t.apply((1.0, 0.0)), (1.0, 2.0));
        assert_eq!(t.apply((0.0, 1.0)), (3.0, 4.0));
    }

    #[test]
    fn mat2_eigenvalues_of_diagonal_matrix_are_the_diagonal() {
        let m = Mat2::new(2.0, 0.0, 0.0, 5.0);
        let (l1, l2) = m.eigenvalues().expect("real eigenvalues");
        let mut vals = [l1, l2];
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((vals[0] - 2.0).abs() < 1e-9);
        assert!((vals[1] - 5.0).abs() < 1e-9);
    }

    #[test]
    fn mat2_rotation_like_matrix_has_complex_eigenvalues() {
        let m = Mat2::new(0.0, -1.0, 1.0, 0.0);
        assert!(m.eigenvalues().is_none());
    }
}
// #endregion 🔖Tests
