//! 📦 `bounds` — one named inference: 3d bounding box across every object's transform position,
//! plus object count. `LowpolyObject` carries no live mesh content field at all (the half-edge-mesh
//! JSON a session's compute engine works with lives session-side, `✏️editor/🖌️session::LowpolyScratch`'s
//! `mesh_workspace` cache — round 2 of ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's
//! round-trip law fix); this facet reads only the typed `transform.position` field every object
//! already carries. Simple whole-snapshot scalar: no `InferredField` caching.

use crate::{LowpolyObject, LowpolySnapshot};

//#region 📦Bounds
/// 📦 Axis-aligned 3d bounding box.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyBounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

fn grow(bounds: Option<LowpolyBounds>, point: [f32; 3]) -> LowpolyBounds {
    match bounds {
        Some(bounds) => LowpolyBounds { min: [bounds.min[0].min(point[0]), bounds.min[1].min(point[1]), bounds.min[2].min(point[2])], max: [bounds.max[0].max(point[0]), bounds.max[1].max(point[1]), bounds.max[2].max(point[2])] },
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
