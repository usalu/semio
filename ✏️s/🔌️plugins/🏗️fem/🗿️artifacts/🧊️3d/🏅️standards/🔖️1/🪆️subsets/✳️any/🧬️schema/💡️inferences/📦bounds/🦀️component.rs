//! 📦 `bounds` — one named inference: the 3d extent and entity counts derivable from a fem3d
//! snapshot's `nodes`/`elements` alone. A whole-snapshot scalar (not per-entity), so this leaf
//! holds a plain pure function rather than an `InferredField` dependency chain — nothing here
//! benefits from per-entity incremental caching, unlike `puzzle3d`'s `flatPosition`.

use crate::artifacts::fem3d::Fem3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Axis-aligned 3d bounding box in meters (empty snapshot: both corners at the origin).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem3dBoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦️ `bounds` — 3d extent plus node/element counts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    Fem3dBounds {
        bounding_box: Fem3dBoundingBox { min, max },
        node_count: snapshot.nodes.len() as u32,
        element_count: snapshot.elements.len() as u32,
    }
}
//#endregion 🔖️Bounds
