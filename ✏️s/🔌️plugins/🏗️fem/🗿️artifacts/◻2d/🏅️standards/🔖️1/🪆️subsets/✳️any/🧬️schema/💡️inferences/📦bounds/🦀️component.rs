//! 📦 `bounds` — one named inference: the plan-view extent and entity counts derivable from a
//! fem2d snapshot's `nodes`/`elements` alone. A whole-snapshot scalar (not per-entity), so this
//! leaf holds a plain pure function rather than an `InferredField` dependency chain — nothing here
//! benefits from per-entity incremental caching, unlike `puzzle3d`'s `flatPosition`.

use crate::artifacts::fem2d::Fem2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Axis-aligned plan-view bounding box in meters (empty snapshot: both corners at the origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dBoundingBox {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

/// 📦️ `bounds` — plan-view extent plus node/element counts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dBounds {
    pub bounding_box: Fem2dBoundingBox,
    pub node_count: u32,
    pub element_count: u32,
}

/// 📦️ Computes `bounds` from a fem2d snapshot's `nodes`/`elements` — the min/max plan-view extent
/// of every node's `(x, y)`, plus their counts. Empty `nodes` yields the origin-degenerate box.
pub async fn compute_fem2d_bounds(snapshot: &Fem2dSnapshot) -> Fem2dBounds {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    for node in &snapshot.nodes {
        min[0] = min[0].min(node.x);
        min[1] = min[1].min(node.y);
        max[0] = max[0].max(node.x);
        max[1] = max[1].max(node.y);
    }
    if snapshot.nodes.is_empty() {
        min = [0.0; 2];
        max = [0.0; 2];
    }
    Fem2dBounds {
        bounding_box: Fem2dBoundingBox { min, max },
        node_count: snapshot.nodes.len() as u32,
        element_count: snapshot.elements.len() as u32,
    }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemElement, FemNode};

    //#region 🧸️Fixtures
    async fn sample_snapshot() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![
                FemNode { id: "n1".into(), x: -2.0, y: 1.0 },
                FemNode { id: "n2".into(), x: 5.0, y: 1.0 },
                FemNode { id: "n3".into(), x: 5.0, y: 7.5 },
            ],
            elements: vec![FemElement::Bar {
                id: "e1".into(),
                start: "n1".into(),
                end: "n2".into(),
                material_id: "m1".into(),
                section_id: "s1".into(),
            }],
            ..Default::default()
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    async fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(compute_fem2d_bounds(&snapshot), compute_fem2d_bounds(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_fem2d_bounds(&Fem2dSnapshot::default()), Fem2dBounds::default());
    }

    #[test]
    async fn bounds_matches_hand_built_node_extent() {
        let bounds = compute_fem2d_bounds(&sample_snapshot());
        assert_eq!(bounds.bounding_box.min, [-2.0, 1.0]);
        assert_eq!(bounds.bounding_box.max, [5.0, 7.5]);
        assert_eq!(bounds.node_count, 3);
        assert_eq!(bounds.element_count, 1);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
