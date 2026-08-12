//! 📦 `bounds` — one named inference: 3d bounding box across every object's transform position,
//! plus object count. `LowpolyObject::mesh_json` is a raw half-edge-mesh JSON blob decoded by the
//! `⚙️engine`'s own `HalfedgeMesh` codec (a whole separate geometry crate) — deliberately left
//! unparsed here, this facet reads only the typed `transform.position` field every object already
//! carries. Simple whole-snapshot scalar: no `InferredField` caching.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolySnapshot};
use serde::{Deserialize, Serialize};

//#region 📦Bounds
/// 📦 Axis-aligned 3d bounding box.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyBounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

fn grow(bounds: Option<LowpolyBounds>, point: [f32; 3]) -> LowpolyBounds {
    match bounds {
        Some(bounds) => LowpolyBounds {
            min: [bounds.min[0].min(point[0]), bounds.min[1].min(point[1]), bounds.min[2].min(point[2])],
            max: [bounds.max[0].max(point[0]), bounds.max[1].max(point[1]), bounds.max[2].max(point[2])],
        },
        None => LowpolyBounds { min: point, max: point },
    }
}

/// 📦 3d bounding box across every object's `transform.position`, or `None` for an empty document.
pub(crate) fn scene_bounds(snapshot: &LowpolySnapshot) -> Option<LowpolyBounds> {
    snapshot.objects.iter().fold(None, |bounds, object: &LowpolyObject| Some(grow(bounds, object.transform.position)))
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::{LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};

    fn object(id: &str, position: [f32; 3]) -> LowpolyObject {
        LowpolyObject {
            id: id.into(),
            name: id.into(),
            transform: LowpolyTransform { position, ..LowpolyTransform::default() },
            smooth_shading: false,
            mesh_json: "{}".into(),
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        }
    }

    #[test]
    fn empty_document_has_no_bounds() {
        assert!(scene_bounds(&LowpolySnapshot::default()).is_none());
    }

    #[test]
    fn two_objects_produce_their_enclosing_box() {
        let snapshot = LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![object("a", [-1.0, 0.0, 2.0]), object("b", [3.0, -4.0, 5.0])] };
        let bounds = scene_bounds(&snapshot).expect("two objects bound");
        assert_eq!(bounds, LowpolyBounds { min: [-1.0, -4.0, 2.0], max: [3.0, 0.0, 5.0] });
    }
}
//#endregion 🧪️Tests
