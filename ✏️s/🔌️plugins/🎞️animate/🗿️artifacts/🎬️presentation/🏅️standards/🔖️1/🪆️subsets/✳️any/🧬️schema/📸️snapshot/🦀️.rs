//! 🧬️ Presentation snapshot schema — persistent fields only.
//!
//! `PresentationSnapshot` carries the shared `source` figure, its `tiles`, and two owned composed-child
//! handles (`presentation`/`animation`). `ArtifactPack` encodes its derived `dsl::DslRecord` spec; the
//! hex/bracket text codec lives in `../../../../🚪️io/📸️snapshot/📝️text/🦀️.rs`. This file keeps only the
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "presentation")]
#[artifact_schema(id = "s.animate.presentation")]
pub struct PresentationSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🖼️ The shared source figure the composed `presentation` deck is derived from — the PERSISTED
    /// payload of that child slot (`🏭️process`'s `stock_payload` pattern). A composed child's content
    /// never travels inside `store::ArtifactChild`, and the react shell answers `Effect::LoadDocument`
    /// with an EMPTY member roster, so `crate::genesis_presentation_child_pack` can only derive the deck
    /// from fields the parent's own pack/DSL round-trips.
    #[state(artifact)]
    pub source: crate::FigureTileSource,
    /// 🧱 The named tile crops of [`PresentationSnapshot::source`] — one composed slide each. See
    /// [`PresentationSnapshot::source`].
    #[state(artifact)]
    pub tiles: Vec<crate::FigureTileDraft>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub presentation: PresentationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub animation: AnimationChild,
}

impl Default for PresentationSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app — the shared source figure with no tile crops
/// yet. The `demo` EXAMPLE is [`demo_snapshot`], not this.
pub fn default_snapshot() -> PresentationSnapshot {
    crate::presentation_snapshot_with_tiles(&crate::default_figure_tile_source(), &[])
}

/// 🎬️ The `demo` example document — the shared source figure already cropped into the same 3×5 tile
/// grid `resetGrid` seeds, so the booted pane shows a real deck instead of an empty tile editor. The
/// committed `🖼️assets/🎬️demo` asset is this snapshot printed by this crate's own DSL printer.
pub fn demo_snapshot() -> PresentationSnapshot {
    let source = crate::default_figure_tile_source();
    let tiles = crate::standards::v1::subsets::any::schema::populate_tile_drafts_from_grid(crate::standards::v1::subsets::any::schema::FigureTileGridSeedSpec { source: &source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
    crate::presentation_snapshot_with_tiles(&source, &tiles)
}
//#endregion 🔖️Snapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
