//! 📦 `bounds` — one named inference: the stock's world-space axis-aligned bounding box. Whole-
//! snapshot scalar, not per-entity, so this leaf holds plain pure functions rather than an
//! `InferredField` chain — the family root's `impl protocol::Inference<Process3dSnapshot>` calls
//! `stock_bounding_box` directly.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: the bound is computed straight
//! from a resolved `SemioBrepSnapshot`'s own `vertices` list (the AABB of its point set) rather than
//! from `SolidSpec`'s (now-deleted) per-variant analytic-extent switch — a strictly MORE general
//! technique (works for any brep content, not just the three known primitive kinds) that happens to
//! also be simpler. A brep with no vertices (the sphere/cylinder-lateral-face case, or the honest
//! empty placeholder minted for `WorkingSolid::ImportedMesh`/`ImportedSolid`) degenerates to a
//! single point at `pose.position` — honest given what the brep content alone can tell us.

use crate::Pose;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️BoundingBox
/// 📦️ Axis-aligned world-space bounding box.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0] }
    }
}
//#endregion 🔖️BoundingBox

//#region 🔖️Derivation
/// 🌀️ Rodrigues' rotation formula — rotates `p` by `angle` radians around `axis` (normalized
/// internally; a zero-length axis or zero angle is a no-op).
fn rotate_axis_angle(p: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let norm = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    if norm < 1e-12 || angle == 0.0 {
        return p;
    }
    let a = [axis[0] / norm, axis[1] / norm, axis[2] / norm];
    let (sin_a, cos_a) = angle.sin_cos();
    let dot = p[0] * a[0] + p[1] * a[1] + p[2] * a[2];
    let cross = [a[1] * p[2] - a[2] * p[1], a[2] * p[0] - a[0] * p[2], a[0] * p[1] - a[1] * p[0]];
    [p[0] * cos_a + cross[0] * sin_a + a[0] * dot * (1.0 - cos_a), p[1] * cos_a + cross[1] * sin_a + a[1] * dot * (1.0 - cos_a), p[2] * cos_a + cross[2] * sin_a + a[2] * dot * (1.0 - cos_a)]
}

/// 📦️ World-space AABB of `solid` (a resolved `SemioBrepSnapshot`) at `pose` — the AABB of the
/// rotated+translated vertex set (a standard, slightly-loose-but-honest OBB-corners-then-AABB
/// technique). No vertices (untrimmed sphere/cylinder faces, or an empty placeholder) degenerates
/// to a single point at `pose.position`.
pub fn brep_bounding_box(solid: &SemioBrepSnapshot, pose: &Pose) -> BoundingBox {
    if solid.vertices.is_empty() {
        return BoundingBox { min: pose.position, max: pose.position };
    }
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for vertex in &solid.vertices {
        let corner = [vertex.point.x, vertex.point.y, vertex.point.z];
        let rotated = rotate_axis_angle(corner, pose.axis, pose.angle);
        let world = [rotated[0] + pose.position[0], rotated[1] + pose.position[1], rotated[2] + pose.position[2]];
        for i in 0..3 {
            min[i] = min[i].min(world[i]);
            max[i] = max[i].max(world[i]);
        }
    }
    BoundingBox { min, max }
}
//#endregion 🔖️Derivation

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
