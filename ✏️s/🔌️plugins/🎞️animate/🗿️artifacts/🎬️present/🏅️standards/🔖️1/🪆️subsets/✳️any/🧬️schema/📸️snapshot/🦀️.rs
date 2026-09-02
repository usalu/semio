//! 🧬️ Present snapshot schema — persistent fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`,
//! `animate→C:presentation,animation`): `PresentSnapshot` now carries two owned composed-child
//! handles (`presentation`/`animation`) instead of the old inline `source: FigureTileSource` +
//! `tiles: Vec<FigureTileDraft>` fields — `store::ArtifactChild<S>` has no `DslField` impl, so the
//! old `dsl::DslRecord`-derived mirror (`PresentSnapshotDsl`) is gone. The hand-rolled codec itself
//! (hex/bracket text + LEB128-length-prefixed binary child-handle convention) now lives in
//! `../../../../🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` (ticket
//! `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` §1 CORRECTION) — this file keeps only the
//! type + its pure transforms.

use crate::artifacts::present::{AnimationChild, PresentationChild};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted present document snapshot — a composed `presentation` deck (shared source figure +
/// named tile crops, see `crate::artifacts::present::presentation_snapshot_from_source_tiles`) plus a
/// composed `animation` set (currently always empty — see `crate::artifacts::present::animation_child_handle`'s
/// doc comment for the honest gap). Both slots are bare (never absent) — this artifact always
/// composes exactly one of each, matching writer's `document: WriterDocumentChild` single-`Option`-in-
/// the-diff convention rather than lowpoly's optional-slot double-`Option` shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.presentation")]
    pub presentation: PresentationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.animation")]
    pub animation: AnimationChild,
}

impl Default for PresentSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> PresentSnapshot {
    crate::artifacts::present::present_snapshot_with_tiles(&crate::artifacts::present::default_figure_tile_source(), &[])
}
//#endregion 🔖️Snapshot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips() {
        let snap = PresentSnapshot::default();
        let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <PresentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = PresentSnapshot::default();
        let text = <PresentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <PresentSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn populated_snapshot_pack_and_dsl_round_trip() {
        let source = crate::artifacts::present::default_figure_tile_source();
        let tiles = vec![crate::artifacts::present::FigureTileDraft { id: "t1".into(), name: "Tile One".into(), crop: crate::artifacts::present::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }];
        let snap = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
        let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <PresentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);

        let text = <PresentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <PresentSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🧪️Tests
