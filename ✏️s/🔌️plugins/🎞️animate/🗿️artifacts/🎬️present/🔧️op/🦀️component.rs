//! 🔧 present artifact — OpText/OpBinary for `PresentMutation`.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::present::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};

use crate::artifacts::present::snapshot::schema::{present_snapshot_from_dsl, present_snapshot_to_dsl, PresentSnapshotDsl};
use crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentSnapshot};
use protocol::CollectionMutation;


//#region 🔖️OpText
/// ⚡️ DSL-facing mirror of `PresentMutation`, declared purely so `#[derive(dsl::DslEnum)]` has
/// something to attach to: `PresentMutation::Tiles` wraps `protocol::CollectionMutation<..>`, a
/// foreign generic type the derive can't classify (and can't gain a `DslField` impl here either — both
/// the trait and the type live outside this crate, so Rust's orphan rules forbid it). Every `Tiles(...)`
/// case is flattened into its own tagged variant instead; `SetSource`/`SetTiles`/`SetSnapshot` carry
/// straight through unchanged. `From`/`Into` below keep this an implementation detail — nothing
/// outside `impl protocol::OpText for PresentMutation` ever names it.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum PresentMutationDsl {
    TilesAdd {
        index: usize,
        #[dsl(block)]
        item: FigureTileDraft,
    },
    TilesRemove {
        id: String,
    },
    TilesMove {
        id: String,
        to_index: usize,
    },
    TilesPatch {
        id: String,
        #[dsl(block)]
        patch: FigureTileDraftPatch,
    },
    SetSource {
        #[dsl(block)]
        source: FigureTileSource,
    },
    SetTiles {
        #[dsl(table)]
        tiles: Vec<FigureTileDraft>,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: PresentSnapshotDsl,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for PresentMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for PresentMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




impl From<&PresentMutation> for PresentMutationDsl {
    fn from(operation: &PresentMutation) -> Self {
        match operation {
            PresentMutation::Tiles(CollectionMutation::Add { index: at, item }) => PresentMutationDsl::TilesAdd { index: *at, item: item.clone() },
            PresentMutation::Tiles(CollectionMutation::Remove { id }) => PresentMutationDsl::TilesRemove { id: id.clone() },
            PresentMutation::Tiles(CollectionMutation::Move { id, to_index: to }) => PresentMutationDsl::TilesMove { id: id.clone(), to_index: *to },
            PresentMutation::Tiles(CollectionMutation::Patch { id, patch }) => PresentMutationDsl::TilesPatch { id: id.clone(), patch: patch.clone() },
            PresentMutation::SetSource { source } => PresentMutationDsl::SetSource { source: source.clone() },
            PresentMutation::SetTiles { tiles } => PresentMutationDsl::SetTiles { tiles: tiles.clone() },
            PresentMutation::SetSnapshot { snapshot } => PresentMutationDsl::SetSnapshot { snapshot: present_snapshot_to_dsl(snapshot) },
        }
    }
}

impl From<PresentMutationDsl> for PresentMutation {
    fn from(operation: PresentMutationDsl) -> Self {
        match operation {
            PresentMutationDsl::TilesAdd { index, item } => PresentMutation::Tiles(CollectionMutation::Add { index: index, item }),
            PresentMutationDsl::TilesRemove { id } => PresentMutation::Tiles(CollectionMutation::Remove { id }),
            PresentMutationDsl::TilesMove { id, to_index } => PresentMutation::Tiles(CollectionMutation::Move { id, to_index: to_index }),
            PresentMutationDsl::TilesPatch { id, patch } => PresentMutation::Tiles(CollectionMutation::Patch { id, patch }),
            PresentMutationDsl::SetSource { source } => PresentMutation::SetSource { source },
            PresentMutationDsl::SetTiles { tiles } => PresentMutation::SetTiles { tiles },
            PresentMutationDsl::SetSnapshot { snapshot } => PresentMutation::SetSnapshot { snapshot: present_snapshot_from_dsl(snapshot) },
        }
    }
}

/// ⚡️ One-line op-text for every `PresentMutation` variant, routed through {@link PresentMutationDsl}


/// ⚡️ Binary mirror of the `OpText` bridge above — `PresentMutationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslEnum)]`, so this is a pure `From`/`Into` forward.
impl protocol::OpText for PresentMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        PresentMutationDsl::parse_op(line).map(Into::into)
    }
    fn print_op(&self) -> String {
        PresentMutationDsl::from(self).print_op()
    }
}

impl protocol::OpBinary for PresentMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        PresentMutationDsl::from(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        PresentMutationDsl::decode_op(bytes).map(PresentMutationDsl::into)
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;
    use crate::artifacts::present::{default_figure_tile_source, default_present_snapshot, FigureTileDraft, FigureTileDraftPatch, FigureTileFrame, PresentSnapshot};
    use crate::artifacts::present::engine::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use store::os_store::test_support;

    fn round_trip(deck: &PresentSnapshot, operation: &PresentMutation) -> PresentSnapshot {
        let forward = vcs::apply_mutation(deck, operation);
        let mut restored = forward.clone();
        for back in protocol::Mutation::inverse(operation, deck) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, deck, "backwards() must exactly restore the pre-operation deck");
        forward
    }

    #[test]
    fn set_tiles_and_clear_round_trip() {
        let deck = default_present_snapshot();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let seeded = round_trip(&deck, &PresentMutation::SetTiles { tiles });
        assert_eq!(seeded.tiles.len(), 4);
        let cleared = round_trip(&seeded, &PresentMutation::SetTiles { tiles: Vec::new() });
        assert!(cleared.tiles.is_empty());
    }

    #[test]
    fn tile_add_patch_remove_round_trip() {
        let deck = default_present_snapshot();
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        let added = round_trip(&deck, &PresentMutation::Tiles(CollectionMutation::Add { index: 0, item: tile }));
        assert_eq!(added.tiles.len(), 1);
        let renamed = round_trip(&added, &PresentMutation::Tiles(CollectionMutation::Patch { id: "t1".into(), patch: FigureTileDraftPatch { name: Some("Renamed".into()), crop: None } }));
        assert_eq!(renamed.tiles[0].name, "Renamed");
        let recropped = round_trip(&renamed, &PresentMutation::Tiles(CollectionMutation::Patch { id: "t1".into(), patch: FigureTileDraftPatch { name: None, crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) } }));
        assert_eq!(recropped.tiles[0].crop.width, 0.4);
        let removed = round_trip(&recropped, &PresentMutation::Tiles(CollectionMutation::Remove { id: "t1".into() }));
        assert!(removed.tiles.is_empty());
    }

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_tiles_add() {
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        test_support::assert_op_line_round_trip(&PresentMutation::Tiles(CollectionMutation::Add { index: 0, item: tile }));
    }

    #[test]
    fn op_text_round_trip_tiles_remove() {
        test_support::assert_op_line_round_trip(&PresentMutation::Tiles(CollectionMutation::Remove { id: "t1".into() }));
    }

    #[test]
    fn op_text_round_trip_tiles_move() {
        test_support::assert_op_line_round_trip(&PresentMutation::Tiles(CollectionMutation::Move { id: "t1".into(), to_index: 2 }));
    }

    #[test]
    fn op_text_round_trip_tiles_patch_full() {
        let patch = FigureTileDraftPatch { name: Some("Renamed".into()), crop: Some(FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 }) };
        test_support::assert_op_line_round_trip(&PresentMutation::Tiles(CollectionMutation::Patch { id: "t1".into(), patch }));
    }

    #[test]
    fn op_text_round_trip_tiles_patch_empty() {
        let patch = FigureTileDraftPatch { name: None, crop: None };
        test_support::assert_op_line_round_trip(&PresentMutation::Tiles(CollectionMutation::Patch { id: "t1".into(), patch }));
    }

    #[test]
    fn op_text_round_trip_set_source() {
        test_support::assert_op_line_round_trip(&PresentMutation::SetSource { source: default_figure_tile_source() });
    }

    #[test]
    fn op_text_round_trip_set_tiles() {
        let source = default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        test_support::assert_op_line_round_trip(&PresentMutation::SetTiles { tiles });
    }

    #[test]
    fn op_text_round_trip_set_snapshot() {
        test_support::assert_op_line_round_trip(&PresentMutation::SetSnapshot { snapshot: default_present_snapshot() });
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests
