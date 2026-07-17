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

// #region 🔖VecD
/// 📏 Heap-allocated f64 vector for element and system-level numerics (loads, displacements, residuals).
#[derive(Clone, Debug, PartialEq)]
pub struct VecD(pub Vec<f64>);

impl VecD {
    pub fn zeros(n: usize) -> Self {
        Self(vec![0.0; n])
    }

    pub fn from_vec(data: Vec<f64>) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, i: usize) -> f64 {
        self.0[i]
    }

    pub fn set(&mut self, i: usize, value: f64) {
        self.0[i] = value;
    }

    pub fn add_at(&mut self, i: usize, value: f64) {
        self.0[i] += value;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn scale(&self, s: f64) -> Self {
        Self(self.0.iter().map(|v| v * s).collect())
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b).collect())
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b).collect())
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn norm_inf(&self) -> f64 {
        self.0.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()))
    }
}
// #endregion 🔖VecD

// #region 🔖MatD
/// 🧮 Dynamic dense f64 matrix, row-major storage; sized for element stiffness matrices and small global systems.
#[derive(Clone, Debug, PartialEq)]
pub struct MatD {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl MatD {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn add_at(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    pub fn matmul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows, "matmul dimension mismatch");
        let mut out = Self::zeros(self.rows, other.cols);
        for row in 0..self.rows {
            for k in 0..self.cols {
                let a = self.get(row, k);
                if a == 0.0 {
                    continue;
                }
                for col in 0..other.cols {
                    out.add_at(row, col, a * other.get(k, col));
                }
            }
        }
        out
    }

    pub fn mul_vec(&self, x: &VecD) -> VecD {
        assert_eq!(self.cols, x.len(), "mul_vec dimension mismatch");
        let mut out = VecD::zeros(self.rows);
        for row in 0..self.rows {
            let mut sum = 0.0;
            for col in 0..self.cols {
                sum += self.get(row, col) * x.get(col);
            }
            out.set(row, sum);
        }
        out
    }

    /// 🧮 `Bᵀ D B` scaled by `weight`, accumulated into `self` — the element-stiffness Gauss-point kernel.
    pub fn add_triple_product(&mut self, b: &MatD, d: &MatD, weight: f64) {
        let btdb = b.transpose().matmul(d).matmul(b);
        for i in 0..self.data.len() {
            self.data[i] += weight * btdb.data[i];
        }
    }

    /// 🧮 Solves `Ax = b` via Gaussian elimination with partial pivoting; `None` if `A` is singular.
    pub fn lu_solve(&self, b: &VecD) -> Option<VecD> {
        assert_eq!(self.rows, self.cols, "lu_solve requires a square matrix");
        assert_eq!(self.rows, b.len(), "lu_solve dimension mismatch");
        let n = self.rows;
        let mut a = self.data.clone();
        let mut x = b.0.clone();
        for pivot in 0..n {
            let (mut best_row, mut best_val) = (pivot, a[pivot * n + pivot].abs());
            for row in (pivot + 1)..n {
                let val = a[row * n + pivot].abs();
                if val > best_val {
                    best_row = row;
                    best_val = val;
                }
            }
            if best_val < 1e-12 {
                return None;
            }
            if best_row != pivot {
                for col in 0..n {
                    a.swap(pivot * n + col, best_row * n + col);
                }
                x.swap(pivot, best_row);
            }
            let pivot_value = a[pivot * n + pivot];
            for row in (pivot + 1)..n {
                let factor = a[row * n + pivot] / pivot_value;
                if factor == 0.0 {
                    continue;
                }
                for col in pivot..n {
                    a[row * n + col] -= factor * a[pivot * n + col];
                }
                x[row] -= factor * x[pivot];
            }
        }
        for row in (0..n).rev() {
            let mut sum = x[row];
            for col in (row + 1)..n {
                sum -= a[row * n + col] * x[col];
            }
            x[row] = sum / a[row * n + row];
        }
        Some(VecD(x))
    }
}
// #endregion 🔖MatD

// #region 🔖Mat3d
/// 🧊 3x3 f64 matrix for element local frames and rotation transforms, column-major storage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3d {
    pub cols: [[f64; 3]; 3],
}

impl Mat3d {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    /// 🧭 Rotation matrix from an orthonormal local basis, columns `(x, y, z)` expressed in global coordinates.
    pub fn from_axes(x: [f64; 3], y: [f64; 3], z: [f64; 3]) -> Self {
        Self { cols: [x, y, z] }
    }

    pub fn transpose(self) -> Self {
        Self {
            cols: [
                [self.cols[0][0], self.cols[1][0], self.cols[2][0]],
                [self.cols[0][1], self.cols[1][1], self.cols[2][1]],
                [self.cols[0][2], self.cols[1][2], self.cols[2][2]],
            ],
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| {
            self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2]
        };
        Self {
            cols: [
                [entry(0, 0), entry(1, 0), entry(2, 0)],
                [entry(0, 1), entry(1, 1), entry(2, 1)],
                [entry(0, 2), entry(1, 2), entry(2, 2)],
            ],
        }
    }

    pub fn mul_vec3(self, v: [f64; 3]) -> [f64; 3] {
        [
            self.cols[0][0] * v[0] + self.cols[1][0] * v[1] + self.cols[2][0] * v[2],
            self.cols[0][1] * v[0] + self.cols[1][1] * v[1] + self.cols[2][1] * v[2],
            self.cols[0][2] * v[0] + self.cols[1][2] * v[1] + self.cols[2][2] * v[2],
        ]
    }
}

pub fn vec3d_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn vec3d_length(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn vec3d_normalize(v: [f64; 3]) -> [f64; 3] {
    let len = vec3d_length(v);
    if len < 1e-12 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn vec3d_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
// #endregion 🔖Mat3d

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

    #[test]
    fn vecd_dot_and_norm() {
        let a = VecD::from_vec(vec![3.0, 4.0]);
        let b = VecD::from_vec(vec![1.0, 0.0]);
        assert!((a.dot(&b) - 3.0).abs() < 1e-12);
        assert!((a.norm2() - 5.0).abs() < 1e-12);
        assert!((a.norm_inf() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn vecd_add_sub_scale_round_trip() {
        let a = VecD::from_vec(vec![1.0, 2.0, 3.0]);
        let b = VecD::from_vec(vec![0.5, 0.5, 0.5]);
        let sum = a.add(&b);
        let back = sum.sub(&b);
        for i in 0..3 {
            assert!((back.get(i) - a.get(i)).abs() < 1e-12);
        }
        assert!((a.scale(2.0).get(1) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn matd_matmul_identity_is_noop() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 2.0);
        m.set(1, 0, 3.0);
        m.set(1, 1, 4.0);
        let id = MatD::identity(2);
        let out = m.matmul(&id);
        assert_eq!(out, m);
    }

    #[test]
    fn matd_transpose_round_trips() {
        let mut m = MatD::zeros(2, 3);
        for row in 0..2 {
            for col in 0..3 {
                m.set(row, col, (row * 3 + col) as f64);
            }
        }
        assert_eq!(m.transpose().transpose(), m);
    }

    #[test]
    fn matd_mul_vec_matches_matrix_vector_multiply() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 2.0);
        m.set(0, 1, 0.0);
        m.set(1, 0, 0.0);
        m.set(1, 1, 3.0);
        let x = VecD::from_vec(vec![1.0, 1.0]);
        let y = m.mul_vec(&x);
        assert!((y.get(0) - 2.0).abs() < 1e-12);
        assert!((y.get(1) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn matd_lu_solve_matches_hand_solved_system() {
        // 2x + y = 5, x + 3y = 10  =>  x = 1, y = 3
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 3.0);
        let b = VecD::from_vec(vec![5.0, 10.0]);
        let x = a.lu_solve(&b).expect("solvable");
        assert!((x.get(0) - 1.0).abs() < 1e-9);
        assert!((x.get(1) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn matd_lu_solve_detects_singular_matrix() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 4.0);
        let b = VecD::from_vec(vec![1.0, 2.0]);
        assert!(a.lu_solve(&b).is_none());
    }

    #[test]
    fn matd_add_triple_product_matches_btdb() {
        let mut b = MatD::zeros(1, 2);
        b.set(0, 0, 1.0);
        b.set(0, 1, 2.0);
        let mut d = MatD::zeros(1, 1);
        d.set(0, 0, 3.0);
        let mut ke = MatD::zeros(2, 2);
        ke.add_triple_product(&b, &d, 1.0);
        // Bᵀ D B = [1;2] * 3 * [1 2] = [[3,6],[6,12]]
        assert!((ke.get(0, 0) - 3.0).abs() < 1e-12);
        assert!((ke.get(0, 1) - 6.0).abs() < 1e-12);
        assert!((ke.get(1, 1) - 12.0).abs() < 1e-12);
    }

    #[test]
    fn mat3d_identity_axes_is_identity() {
        let m = Mat3d::from_axes([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(m, Mat3d::IDENTITY);
    }

    #[test]
    fn mat3d_transpose_is_inverse_for_orthonormal_basis() {
        let x = vec3d_normalize([1.0, 1.0, 0.0]);
        let z = [0.0, 0.0, 1.0];
        let y = vec3d_cross(z, x);
        let m = Mat3d::from_axes(x, y, z);
        let round = m.mul(m.transpose());
        for row in 0..3 {
            for col in 0..3 {
                let expected = if row == col { 1.0 } else { 0.0 };
                assert!((round.cols[col][row] - expected).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn mat3d_mul_vec3_transforms_basis_vector() {
        let m = Mat3d::from_axes([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let out = m.mul_vec3([1.0, 0.0, 0.0]);
        assert!((out[0] - 0.0).abs() < 1e-12);
        assert!((out[1] - 1.0).abs() < 1e-12);
        assert!((out[2] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn vec3d_cross_is_perpendicular_to_inputs() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let c = vec3d_cross(a, b);
        assert!((c[2] - 1.0).abs() < 1e-12);
    }
}
// #endregion 🔖Tests
