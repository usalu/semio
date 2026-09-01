//! 📦 `bounds` — one named inference: the 3d extent and entity counts derivable from a fem3d
//! snapshot's `nodes`/`elements` alone. A whole-snapshot scalar (not per-entity), so this leaf
//! holds a plain pure function rather than an `InferredField` dependency chain — nothing here
//! benefits from per-entity incremental caching, unlike `puzzle3d`'s `flatPosition`.

use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Bounds
/// 📦️ Axis-aligned 3d bounding box in meters (empty snapshot: both corners at the origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Fem3dBoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦️ `bounds` — 3d extent plus node/element counts.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Fem3dBounds {
    pub bounding_box: Fem3dBoundingBox,
    pub node_count: u32,
    pub element_count: u32,
}

/// 📦️ Computes `bounds` from a fem3d snapshot's `nodes`/`elements` — the min/max extent of every
/// node's `(x, y, z)`, plus their counts. Empty `nodes` yields the origin-degenerate box.
pub fn compute_fem3d_bounds(snapshot: &Fem3dSnapshot) -> Fem3dBounds {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for node in &snapshot.nodes {
        min[0] = min[0].min(node.x);
        min[1] = min[1].min(node.y);
        min[2] = min[2].min(node.z);
        max[0] = max[0].max(node.x);
        max[1] = max[1].max(node.y);
        max[2] = max[2].max(node.z);
    }
    if snapshot.nodes.is_empty() {
        min = [0.0; 3];
        max = [0.0; 3];
    }
    Fem3dBounds { bounding_box: Fem3dBoundingBox { min, max }, node_count: snapshot.nodes.len() as u32, element_count: snapshot.elements.len() as u32 }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::fem3d::{FemElement, FemNode};

    //#region 🧸️Fixtures
    fn sample_snapshot() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: -2.0, y: 1.0, z: 0.0 }, FemNode { id: "n2".into(), x: 5.0, y: 1.0, z: -3.0 }, FemNode { id: "n3".into(), x: 5.0, y: 7.5, z: 6.0 }],
            elements: vec![FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() }],
            ..Default::default()
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(compute_fem3d_bounds(&snapshot), compute_fem3d_bounds(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_fem3d_bounds(&Fem3dSnapshot::default()), Fem3dBounds::default());
    }

    #[test]
    fn bounds_matches_hand_built_node_extent() {
        let bounds = compute_fem3d_bounds(&sample_snapshot());
        assert_eq!(bounds.bounding_box.min, [-2.0, 1.0, -3.0]);
        assert_eq!(bounds.bounding_box.max, [5.0, 7.5, 6.0]);
        assert_eq!(bounds.node_count, 3);
        assert_eq!(bounds.element_count, 1);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
