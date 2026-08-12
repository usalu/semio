//! 📦 `bounds` — one named inference: the stock's world-space axis-aligned bounding box. Whole-
//! snapshot scalar, not per-entity, so this leaf holds plain pure functions rather than an
//! `InferredField` chain — the family root's `impl protocol::Inference<Process3dSnapshot>` calls
//! `stock_bounding_box` directly.
//!
//! 🎯️ First-cut correctness-over-performance, documented not hidden: the box is the AABB of the
//! stock solid's LOCAL analytic extents (exact for `Box`/`Cylinder`/`Sphere`, since those are the
//! only solids with known parametric extents — see `⚙️engine/🦀️component.rs`'s primitive kernel
//! calls) rotated by `pose.axis`/`pose.angle` and translated by `pose.position`. An
//! `ImportedMesh`/`ImportedSolid` stock has no extents available without resolving it through the
//! app's kernel session, so it degenerates to a single point at `pose.position` — honest given
//! what the snapshot alone can tell us, not a guess at unknown geometry. `steps` (cuts/drills/
//! attaches already applied) are NOT folded in — this is the bound of the starting stock, not the
//! current machined state; a step-aware bound is future work once kernel replay is threaded
//! through an `InferredField` chain here.

use crate::artifacts::process3d::{SolidSpec, Stock};
use serde::{Deserialize, Serialize};

//#region 🔖️BoundingBox
/// 📦️ Axis-aligned world-space bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
/// 📐️ Local-space AABB corners for a solid spec, before pose transform. `Box` spans `[0,w]×[0,d]×
/// [0,h]` (matches `⚙️engine`'s `make_box`'s corner-at-origin convention); `Cylinder`/`Sphere` are
/// centered on the local origin (matches `make_cylinder`/`make_sphere`). Imported solids have no
/// known extents, so they degenerate to the origin point.
fn local_corners(solid: &SolidSpec) -> Vec<[f64; 3]> {
    match solid {
        SolidSpec::Box { width, depth, height } => {
            let (w, d, h) = (*width, *depth, *height);
            vec![[0.0, 0.0, 0.0], [w, 0.0, 0.0], [w, d, 0.0], [0.0, d, 0.0], [0.0, 0.0, h], [w, 0.0, h], [w, d, h], [0.0, d, h]]
        }
        SolidSpec::Cylinder { radius, height } => {
            let (r, h) = (*radius, *height);
            vec![[-r, -r, 0.0], [r, -r, 0.0], [r, r, 0.0], [-r, r, 0.0], [-r, -r, h], [r, -r, h], [r, r, h], [-r, r, h]]
        }
        SolidSpec::Sphere { radius } => {
            let r = *radius;
            vec![[-r, -r, -r], [r, -r, -r], [r, r, -r], [-r, r, -r], [-r, -r, r], [r, -r, r], [r, r, r], [-r, r, r]]
        }
        SolidSpec::ImportedMesh { .. } | SolidSpec::ImportedSolid { .. } => vec![[0.0, 0.0, 0.0]],
    }
}

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
    [
        p[0] * cos_a + cross[0] * sin_a + a[0] * dot * (1.0 - cos_a),
        p[1] * cos_a + cross[1] * sin_a + a[1] * dot * (1.0 - cos_a),
        p[2] * cos_a + cross[2] * sin_a + a[2] * dot * (1.0 - cos_a),
    ]
}

/// 📦️ World-space AABB of `stock.solid` at `stock.pose` — the AABB of the rotated+translated
/// local-corner set (a standard, slightly-loose-but-honest OBB-corners-then-AABB technique).
pub fn stock_bounding_box(stock: &Stock) -> BoundingBox {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for corner in local_corners(&stock.solid) {
        let rotated = rotate_axis_angle(corner, stock.pose.axis, stock.pose.angle);
        let world = [rotated[0] + stock.pose.position[0], rotated[1] + stock.pose.position[1], rotated[2] + stock.pose.position[2]];
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
mod tests {
    use super::*;
    use crate::artifacts::process3d::Pose;

    #[test]
    fn default_box_stock_bounds_are_unit_cube() {
        let stock = Stock::default();
        let bounds = stock_bounding_box(&stock);
        assert_eq!(bounds.min, [0.0, 0.0, 0.0]);
        assert_eq!(bounds.max, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn translated_box_shifts_bounds() {
        let stock = Stock { id: "s".into(), label: "S".into(), solid: SolidSpec::Box { width: 2.0, depth: 1.0, height: 1.0 }, pose: Pose { position: [5.0, 0.0, 0.0], axis: [0.0, 0.0, 1.0], angle: 0.0 } };
        let bounds = stock_bounding_box(&stock);
        assert_eq!(bounds.min, [5.0, 0.0, 0.0]);
        assert_eq!(bounds.max, [7.0, 1.0, 1.0]);
    }

    #[test]
    fn sphere_bounds_are_centered_cube() {
        let stock = Stock { id: "s".into(), label: "S".into(), solid: SolidSpec::Sphere { radius: 2.0 }, pose: Pose::default() };
        let bounds = stock_bounding_box(&stock);
        assert_eq!(bounds.min, [-2.0, -2.0, -2.0]);
        assert_eq!(bounds.max, [2.0, 2.0, 2.0]);
    }

    #[test]
    fn imported_solid_degenerates_to_a_point() {
        let stock = Stock { id: "s".into(), label: "S".into(), solid: SolidSpec::ImportedSolid { solid_handle: "h1".into() }, pose: Pose { position: [3.0, 4.0, 5.0], axis: [0.0, 0.0, 1.0], angle: 0.0 } };
        let bounds = stock_bounding_box(&stock);
        assert_eq!(bounds.min, [3.0, 4.0, 5.0]);
        assert_eq!(bounds.max, [3.0, 4.0, 5.0]);
    }
}
//#endregion 🧪️Tests
