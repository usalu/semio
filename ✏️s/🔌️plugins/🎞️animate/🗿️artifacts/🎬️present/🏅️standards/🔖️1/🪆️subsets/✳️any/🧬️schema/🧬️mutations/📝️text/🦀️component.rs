//! ⚡️ present artifact — OpText/OpBinary codecs + grammar for `PresentMutation`. `PresentMutation`
//! derives `dsl::DslEnum` directly on its dispatch enum (every variant wraps a payload struct that
//! itself derives `dsl::DslRecord` with its own `#[dsl(keyword = "...")]`), so no separate mirror
//! enum is needed — unlike the retired generic whole-collection `Tiles(...)` variant, every
//! payload here is a plain struct declared in this crate, so `dsl::DslRecord` applies directly.

pub use crate::artifacts::present::schema::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for PresentMutation {
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

impl protocol::OpBinary for PresentMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::schema::mutations::{create_tile, delete_tile, delete_tiles, rename_tile, reorder_tiles, replace_source, replace_tiles, resize_source_frame, resize_tile_crop};
    use crate::artifacts::present::{default_figure_tile_source, default_present_snapshot, FigureTileDraft, FigureTileFrame, PresentSnapshot};
    use crate::artifacts::present::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use store::os_store::test_support;

    fn round_trip(deck: &PresentSnapshot, operation: &PresentMutation) -> PresentSnapshot {
        let (forward, _messages) = vcs::apply_mutation(deck, operation);
        let mut restored = forward.clone();
        for back in protocol::Mutation::inverse(operation, deck) {
            let (next, _messages) = vcs::apply_mutation(&restored, &back);
            restored = next;
        }
        assert_eq!(&restored, deck, "inverse() must exactly restore the pre-operation deck");
        forward
    }

    #[test]
    fn replace_tiles_and_clear_round_trip() {
        let deck = default_present_snapshot();
        let (source, _) = crate::artifacts::present::present_working_scene(&deck);
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let seeded = round_trip(&deck, &PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: tiles }));
        assert_eq!(crate::artifacts::present::present_working_scene(&seeded).1.len(), 4);
        let cleared = round_trip(&seeded, &PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() }));
        assert!(crate::artifacts::present::present_working_scene(&cleared).1.is_empty());
    }

    #[test]
    fn tile_create_rename_resize_delete_round_trip() {
        let deck = default_present_snapshot();
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        let added = round_trip(&deck, &PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile }));
        assert_eq!(crate::artifacts::present::present_working_scene(&added).1.len(), 1);
        let renamed = round_trip(&added, &PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Renamed".into() }));
        assert_eq!(crate::artifacts::present::present_working_scene(&renamed).1[0].name, "Renamed");
        let recropped = round_trip(
            &renamed,
            &PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "t1".into(), new_crop: FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 } }),
        );
        assert_eq!(crate::artifacts::present::present_working_scene(&recropped).1[0].crop.width, 0.4);
        let removed = round_trip(&recropped, &PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "t1".into() }));
        assert!(crate::artifacts::present::present_working_scene(&removed).1.is_empty());
    }

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_create_tile() {
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        test_support::assert_op_line_round_trip(&PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile }));
    }

    #[test]
    fn op_text_round_trip_delete_tile() {
        test_support::assert_op_line_round_trip(&PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "t1".into() }));
    }

    #[test]
    fn op_text_round_trip_delete_tiles() {
        test_support::assert_op_line_round_trip(&PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["t1".into(), "t2".into()] }));
    }

    #[test]
    fn op_text_round_trip_reorder_tiles() {
        test_support::assert_op_line_round_trip(&PresentMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "t1".into(), to_index: 2 }));
    }

    #[test]
    fn op_text_round_trip_rename_tile() {
        test_support::assert_op_line_round_trip(&PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Renamed".into() }));
    }

    #[test]
    fn op_text_round_trip_resize_tile_crop() {
        let new_crop = FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 };
        test_support::assert_op_line_round_trip(&PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "t1".into(), new_crop }));
    }

    #[test]
    fn op_text_round_trip_replace_source() {
        test_support::assert_op_line_round_trip(&PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: default_figure_tile_source() }));
    }

    #[test]
    fn op_text_round_trip_resize_source_frame() {
        test_support::assert_op_line_round_trip(&PresentMutation::ResizeSourceFrame(resize_source_frame::mutation::ResizeSourceFrame { new_frame: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }));
    }

    #[test]
    fn op_text_round_trip_replace_tiles() {
        let source = default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        test_support::assert_op_line_round_trip(&PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: tiles }));
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests
