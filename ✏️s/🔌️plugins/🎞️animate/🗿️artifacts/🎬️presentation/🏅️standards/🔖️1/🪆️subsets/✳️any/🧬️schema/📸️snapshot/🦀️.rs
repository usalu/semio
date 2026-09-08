//! 🧬️ Presentation snapshot schema — persistent fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`,
//! `animate→C:presentation,animation`): `PresentationSnapshot` now carries two owned composed-child
//! handles (`presentation`/`animation`) instead of the old inline `source: FigureTileSource` +
//! `tiles: Vec<FigureTileDraft>` fields — `store::ArtifactChild<S>` has no `DslField` impl, so the
//! old `dsl::DslRecord`-derived mirror (`PresentationSnapshotDsl`) is gone. The hand-rolled codec itself
//! (hex/bracket text + LEB128-length-prefixed binary child-handle convention) now lives in
//! `../../../../🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` (ticket
//! `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` §1 CORRECTION) — this file keeps only the
//! type + its pure transforms.

use crate::{AnimationChild, PresentationChild};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted presentation document snapshot — a composed `presentation` deck (shared source figure +
/// named tile crops, see `crate::presentation_snapshot_from_source_tiles`) plus a
/// composed `animation` set (currently always empty — see `crate::animation_child_handle`'s
/// doc comment for the honest gap). Both slots are bare (never absent) — this artifact always
/// composes exactly one of each, matching writer's `document: WriterDocumentChild` single-`Option`-in-
/// the-diff convention rather than lowpoly's optional-slot double-`Option` shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.presentation")]
pub struct PresentationSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.presentation")]
    pub presentation: PresentationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.animation")]
    pub animation: AnimationChild,
}

impl Default for PresentationSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> PresentationSnapshot {
    crate::presentation_snapshot_with_tiles(&crate::default_figure_tile_source(), &[])
}
//#endregion 🔖️Snapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
