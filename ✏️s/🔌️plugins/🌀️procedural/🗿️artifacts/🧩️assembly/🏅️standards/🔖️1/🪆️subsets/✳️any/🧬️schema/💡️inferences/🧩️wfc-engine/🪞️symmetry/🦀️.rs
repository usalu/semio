//! 🔄️ 2D symmetry: the dihedral group D4 (identity, 3 rotations, 4 reflections) and its action on
//! stencil offsets and on rectangular tile windows. The single source of truth is each element's
//! 2×2 integer matrix — offset transform and window transform both derive from it, which is what
//! guarantees a pattern's rotated pixel grid and its rotated neighbor directions stay consistent
//! with each other (the invariant symmetry-aware extraction and orbit expansion depend on).

use crate::wfc_engine::ids::TileId;

// #region 🔖️Transform
/// 🔄️ One element of the dihedral group D4.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Transform2d {
    Identity,
    Rot90,
    Rot180,
    Rot270,
    FlipH,
    FlipV,
    FlipDiag,
    FlipAntiDiag,
}

type Mat2 = (i32, i32, i32, i32);

impl Transform2d {
    /// 🔄️ All 8 elements, identity first.
    pub const ALL: [Transform2d; 8] = [Transform2d::Identity, Transform2d::Rot90, Transform2d::Rot180, Transform2d::Rot270, Transform2d::FlipH, Transform2d::FlipV, Transform2d::FlipDiag, Transform2d::FlipAntiDiag];

    fn matrix(self) -> Mat2 {
        match self {
            Transform2d::Identity => (1, 0, 0, 1),
            Transform2d::Rot90 => (0, -1, 1, 0),
            Transform2d::Rot180 => (-1, 0, 0, -1),
            Transform2d::Rot270 => (0, 1, -1, 0),
            Transform2d::FlipH => (-1, 0, 0, 1),
            Transform2d::FlipV => (1, 0, 0, -1),
            Transform2d::FlipDiag => (0, 1, 1, 0),
            Transform2d::FlipAntiDiag => (0, -1, -1, 0),
        }
    }

    fn from_matrix(m: Mat2) -> Self {
        Self::ALL.into_iter().find(|t| t.matrix() == m).expect("matrix is not a D4 element")
    }

    /// 🔄️ `self` applied first, then `other` (i.e. `other ∘ self`).
    pub fn semio_compose_rs(self, other: Transform2d) -> Transform2d {
        let (a1, b1, c1, d1) = self.matrix();
        let (a2, b2, c2, d2) = other.matrix();
        Self::from_matrix((a2 * a1 + b2 * c1, a2 * b1 + b2 * d1, c2 * a1 + d2 * c1, c2 * b1 + d2 * d1))
    }

    pub fn inverse(self) -> Transform2d {
        match self {
            Transform2d::Rot90 => Transform2d::Rot270,
            Transform2d::Rot270 => Transform2d::Rot90,
            other => other,
        }
    }

    /// 🔄️ Whether this transform swaps width and height when applied to a window.
    pub fn swaps_dimensions(self) -> bool {
        matches!(self, Transform2d::Rot90 | Transform2d::Rot270 | Transform2d::FlipDiag | Transform2d::FlipAntiDiag)
    }

    /// 🔄️ Transforms a relative grid offset (e.g. a stencil direction).
    pub fn apply_offset(self, (dx, dy): (i32, i32)) -> (i32, i32) {
        let (a, b, c, d) = self.matrix();
        (a * dx + b * dy, c * dx + d * dy)
    }

    /// 🔄️ Transforms a `width × height` row-major tile window, returning the new `(width, height)`
    /// (swapped for the four dimension-swapping elements) and the remapped tile content.
    pub fn apply_window(self, width: usize, height: usize, tiles: &[TileId]) -> (usize, usize, Vec<TileId>) {
        debug_assert_eq!(tiles.len(), width * height);
        let (nw, nh) = if self.swaps_dimensions() { (height, width) } else { (width, height) };
        let (a, b, c, d) = self.inverse().matrix();
        let mut out = vec![TileId(0); nw * nh];
        for oy in 0..nh {
            for ox in 0..nw {
                let cx = 2 * ox as i32 - (nw as i32 - 1);
                let cy = 2 * oy as i32 - (nh as i32 - 1);
                let sx2 = a * cx + b * cy;
                let sy2 = c * cx + d * cy;
                let sx = (sx2 + (width as i32 - 1)) / 2;
                let sy = (sy2 + (height as i32 - 1)) / 2;
                out[oy * nw + ox] = tiles[sy as usize * width + sx as usize];
            }
        }
        (nw, nh, out)
    }
}
// #endregion 🔖️Transform

// #region 🔖️Group
/// 🔄️ A subgroup of D4 to expand patterns/tiles under.
#[derive(Clone, PartialEq, Debug)]
pub enum SymmetryGroup2d {
    /// 🔄️ Just the identity — no expansion.
    None,
    /// 🔄️ The four rotations only.
    C4,
    /// 🔄️ Identity, 180° rotation, and both axis flips (no 90°/270°).
    D2,
    /// 🔄️ The full 8-element dihedral group.
    D4,
    Custom(Vec<Transform2d>),
}

impl SymmetryGroup2d {
    pub fn elements(&self) -> Vec<Transform2d> {
        use Transform2d::*;
        match self {
            SymmetryGroup2d::None => vec![Identity],
            SymmetryGroup2d::C4 => vec![Identity, Rot90, Rot180, Rot270],
            SymmetryGroup2d::D2 => vec![Identity, Rot180, FlipH, FlipV],
            SymmetryGroup2d::D4 => Transform2d::ALL.to_vec(),
            SymmetryGroup2d::Custom(v) => v.clone(),
        }
    }
}
// #endregion 🔖️Group

// #region 🔖️Transform3d
type Mat3 = [[i32; 3]; 3];

fn mat3_mul(a: Mat3, b: Mat3) -> Mat3 {
    let mut r = [[0i32; 3]; 3];
    for (i, row) in r.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    r
}

fn mat3_identity() -> Mat3 {
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
}

fn mat3_transpose(a: Mat3) -> Mat3 {
    let mut r = [[0i32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[j][i] = a[i][j];
        }
    }
    r
}

/// 🔄️ One element of a cube's rotation/reflection symmetry group, represented by its 3×3 integer
/// orthogonal matrix (every entry in `{-1,0,1}`, exactly one nonzero per row/column). Constructed
/// only via [`cube_rotations_24`]/[`cube_symmetries_48`]'s closure computation, never by hand, so
/// every instance is guaranteed to actually be a symmetry of the cube.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Transform3d(Mat3);

impl Transform3d {
    pub fn identity() -> Self {
        Transform3d(mat3_identity())
    }

    /// 🔄️ `self` applied first, then `other`.
    pub fn semio_compose_rs(self, other: Transform3d) -> Transform3d {
        Transform3d(mat3_mul(other.0, self.0))
    }

    /// 🔄️ Orthogonal matrices' inverse is their transpose.
    pub fn inverse(self) -> Transform3d {
        Transform3d(mat3_transpose(self.0))
    }

    pub fn apply_offset(self, (dx, dy, dz): (i32, i32, i32)) -> (i32, i32, i32) {
        let m = self.0;
        (m[0][0] * dx + m[0][1] * dy + m[0][2] * dz, m[1][0] * dx + m[1][1] * dy + m[1][2] * dz, m[2][0] * dx + m[2][1] * dy + m[2][2] * dz)
    }

    pub fn determinant(self) -> i32 {
        let m = self.0;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
}

fn rot_x90() -> Mat3 {
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]]
}

fn rot_z90() -> Mat3 {
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]]
}

fn reflect_x() -> Mat3 {
    [[-1, 0, 0], [0, 1, 0], [0, 0, 1]]
}

fn closure(generators: &[Mat3]) -> Vec<Mat3> {
    let mut group = vec![mat3_identity()];
    let mut frontier = vec![mat3_identity()];
    while !frontier.is_empty() {
        let mut next_frontier = Vec::new();
        for m in &frontier {
            for g in generators {
                let candidate = mat3_mul(*g, *m);
                if !group.contains(&candidate) {
                    group.push(candidate);
                    next_frontier.push(candidate);
                }
            }
        }
        frontier = next_frontier;
    }
    group
}

/// 🔄️ The 24 proper (orientation-preserving, determinant `+1`) rotations of a cube, generated by
/// closure from two 90° generators rather than hand-enumerated.
pub fn cube_rotations_24() -> Vec<Transform3d> {
    closure(&[rot_x90(), rot_z90()]).into_iter().map(Transform3d).collect()
}

/// 🔄️ The full 48-element octahedral symmetry group (24 rotations plus their mirror images).
pub fn cube_symmetries_48() -> Vec<Transform3d> {
    closure(&[rot_x90(), rot_z90(), reflect_x()]).into_iter().map(Transform3d).collect()
}

/// 🔄️ A subgroup of the cube's symmetry group to expand patterns/tiles under.
#[derive(Clone, Debug)]
pub enum SymmetryGroup3d {
    /// 🔄️ Just the identity — no expansion.
    None,
    /// 🔄️ All 24 proper rotations.
    Rot24,
    /// 🔄️ All 48 rotations and reflections.
    Full48,
    /// 🔄️ The four rotations about the Z axis only.
    ZRot4,
    Custom(Vec<Transform3d>),
}

impl SymmetryGroup3d {
    pub fn elements(&self) -> Vec<Transform3d> {
        match self {
            SymmetryGroup3d::None => vec![Transform3d::identity()],
            SymmetryGroup3d::Rot24 => cube_rotations_24(),
            SymmetryGroup3d::Full48 => cube_symmetries_48(),
            SymmetryGroup3d::ZRot4 => {
                let mut t = Transform3d::identity();
                let z90 = Transform3d(rot_z90());
                (0..4)
                    .map(|_| {
                        let cur = t;
                        t = t.semio_compose_rs(z90);
                        cur
                    })
                    .collect()
            }
            SymmetryGroup3d::Custom(v) => v.clone(),
        }
    }
}
// #endregion 🔖️Transform3d

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
