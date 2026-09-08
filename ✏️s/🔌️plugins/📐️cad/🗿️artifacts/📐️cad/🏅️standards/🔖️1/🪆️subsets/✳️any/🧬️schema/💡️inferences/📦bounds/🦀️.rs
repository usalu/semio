//! 📦 `bounds` — one named inference: 3d bounding box across every pane's object origins and
//! brep vertex positions, plus object/vertex counts.
//!
//! ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `CadSnapshot` no longer inlines
//! `objects`/`*Geometry` per pane — that data now lives inside composed `s.stdio.semio.model` CHILD
//! documents (own document, own history; resolving one is a host/composition concern, never
//! something a pure `CadSnapshot`-only function can do — see `🔖️Composition` in
//! `🏪️store/🦀️.rs`). This inference therefore degrades to counting/bounding only the
//! FOUR fixed model-child SLOTS themselves (present vs. absent), not their resolved contents — a
//! real, honest, reduced-fidelity signal (non-zero `object_count` again once a real per-child
//! element/vertex inference exists over the composed children), not a silently wrong one.

use crate::CadSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 📦Bounds
/// 📦 Axis-aligned 3d bounding box.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
