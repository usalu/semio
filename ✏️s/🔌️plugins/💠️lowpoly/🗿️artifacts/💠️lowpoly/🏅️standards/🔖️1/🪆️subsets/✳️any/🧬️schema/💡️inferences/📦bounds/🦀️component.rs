//! 📦 `bounds` — one named inference: 3d bounding box across every object's transform position,
//! plus object count. `LowpolyObject` carries no live mesh content field at all (the half-edge-mesh
//! JSON a session's compute engine works with lives session-side, `✏️editor/🖌️session::LowpolyScratch`'s
//! `mesh_workspace` cache — round 2 of ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's
//! round-trip law fix); this facet reads only the typed `transform.position` field every object
//! already carries. Simple whole-snapshot scalar: no `InferredField` caching.

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

async fn grow(bounds: Option<LowpolyBounds>, point: [f32; 3]) -> LowpolyBounds {
    match bounds {
        Some(bounds) => LowpolyBounds {
            min: [bounds.min[0].min(point[0]), bounds.min[1].min(point[1]), bounds.min[2].min(point[2])],
            max: [bounds.max[0].max(point[0]), bounds.max[1].max(point[1]), bounds.max[2].max(point[2])],
        },
        None => LowpolyBounds { min: point, max: point },
    }
}

/// 📦 3d bounding box across every object's `transform.position`, or `None` for an empty document.
pub(crate) async fn scene_bounds(snapshot: &LowpolySnapshot) -> Option<LowpolyBounds> {
    snapshot.objects.iter().fold(None, |bounds, object: &LowpolyObject| Some(grow(bounds, object.transform.position)))
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::{LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};

    async fn object(id: &str, position: [f32; 3]) -> LowpolyObject {
        LowpolyObject {
            id: id.into(),
            name: id.into(),
            transform: LowpolyTransform { position, ..LowpolyTransform::default() },
            smooth_shading: false,
            mesh: None,
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        }
    }

    #[test]
    async fn empty_document_has_no_bounds() {
        assert!(scene_bounds(&LowpolySnapshot::default()).is_none());
    }

    #[test]
    async fn two_objects_produce_their_enclosing_box() {
        let snapshot = LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: vec![object("a", [-1.0, 0.0, 2.0]), object("b", [3.0, -4.0, 5.0])] };
        let bounds = scene_bounds(&snapshot).expect("two objects bound");
        assert_eq!(bounds, LowpolyBounds { min: [-1.0, -4.0, 2.0], max: [3.0, 0.0, 5.0] });
    }
}
//#endregion 🧪️Tests
