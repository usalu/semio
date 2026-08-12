//! 📦 `bounds` — one named inference: 3d bounding box across every pane's object origins and
//! brep vertex positions, plus object/vertex counts.
//!
//! ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `CadSnapshot` no longer inlines
//! `objects`/`*Geometry` per pane — that data now lives inside composed `s.stdio.semio.model` CHILD
//! documents (own document, own history; resolving one is a host/composition concern, never
//! something a pure `CadSnapshot`-only function can do — see `🔖️Composition` in
//! `🏪️store/🦀️component.rs`). This inference therefore degrades to counting/bounding only the
//! FOUR fixed model-child SLOTS themselves (present vs. absent), not their resolved contents — a
//! real, honest, reduced-fidelity signal (non-zero `object_count` again once a real per-child
//! element/vertex inference exists over the composed children), not a silently wrong one.

use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 📦Bounds
/// 📦 Axis-aligned 3d bounding box.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦 3d bounding box across every pane's object origins and brep vertex positions. `None`
/// unconditionally now — real bounds require resolving the composed model children's content,
/// which is out of this pure inference's reach (see module doc comment).
pub(crate) fn scene_bounds(_snapshot: &CadSnapshot) -> Option<CadBounds> {
    None
}

/// 📦 Number of the four fixed model-child SLOTS that are occupied (0..=4) — a real, cheap signal
/// over what `CadSnapshot` itself can see; NOT a count of elements inside those children.
pub(crate) fn object_count(snapshot: &CadSnapshot) -> usize {
    [&snapshot.shape_model, &snapshot.building_model, &snapshot.energy_model, &snapshot.structure_classic_model].into_iter().filter(|slot| slot.is_some()).count()
}

/// 📦 Vertex counting requires resolved child content (see module doc comment) — `0` unconditionally.
pub(crate) fn vertex_count(_snapshot: &CadSnapshot) -> usize {
    0
}
//#endregion 📦Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::cad::{empty_cad_snapshot, testkit::sample_model_child};

    #[test]
    fn empty_scene_has_no_bounds() {
        let snapshot = empty_cad_snapshot();
        assert!(scene_bounds(&snapshot).is_none());
        assert_eq!(object_count(&snapshot), 0);
        assert_eq!(vertex_count(&snapshot), 0);
    }

    #[test]
    fn object_count_reflects_occupied_model_slots() {
        let mut snapshot = empty_cad_snapshot();
        snapshot.shape_model = Some(sample_model_child("bounds-law-1"));
        assert_eq!(object_count(&snapshot), 1);
        snapshot.building_model = Some(sample_model_child("bounds-law-2"));
        assert_eq!(object_count(&snapshot), 2);
    }
}
//#endregion 🧪️Tests
