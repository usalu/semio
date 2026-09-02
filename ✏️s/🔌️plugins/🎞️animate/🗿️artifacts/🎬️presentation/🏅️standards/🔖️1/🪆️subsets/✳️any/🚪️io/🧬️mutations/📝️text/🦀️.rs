//! ⚡️ presentation artifact — OpText/OpBinary codecs + grammar for `PresentationMutation`. `PresentationMutation`
//! derives `dsl::DslEnum` directly on its dispatch enum (every variant wraps a payload struct that
//! itself derives `dsl::DslRecord` with its own `#[dsl(keyword = "...")]`), so no separate mirror
//! enum is needed — unlike the retired generic whole-collection `Tiles(...)` variant, every
//! payload here is a plain struct declared in this crate, so `dsl::DslRecord` applies directly.

pub use crate::artifacts::presentation::schema::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for PresentationMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
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

impl protocol::OpBinary for PresentationMutation {
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
    use crate::artifacts::presentation::schema::mutations::{create_tile, delete_tile, delete_tiles, rename_tile, reorder_tiles, replace_source, replace_tiles, resize_source_frame, resize_tile_crop};
    use crate::artifacts::presentation::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
    use crate::artifacts::presentation::{default_figure_tile_source, default_presentation_snapshot, FigureTileDraft, FigureTileFrame, PresentationSnapshot};
    use store::os_store::test_support;

    async fn round_trip(deck: &PresentationSnapshot, operation: &PresentationMutation) -> PresentationSnapshot {
        let (forward, _messages) = vcs::apply_mutation(deck, operation).await.expect("valid mutation");
        let mut restored = forward.clone();
        for back in protocol::Mutation::inverse(operation, deck) {
            let (next, _messages) = vcs::apply_mutation(&restored, &back).await.expect("valid inverse mutation");
            restored = next;
        }
        assert_eq!(&restored, deck, "inverse() must exactly restore the pre-operation deck");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_tiles_and_clear_round_trip() {
        let deck = default_presentation_snapshot();
        let (source, _) = crate::artifacts::presentation::presentation_working_scene(&deck);
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let seeded = round_trip(&deck, &PresentationMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: tiles })).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&seeded).1.len(), 4);
        let cleared = round_trip(&seeded, &PresentationMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() })).await;
        assert!(crate::artifacts::presentation::presentation_working_scene(&cleared).1.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn tile_create_rename_resize_delete_round_trip() {
        let deck = default_presentation_snapshot();
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        let added = round_trip(&deck, &PresentationMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile })).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&added).1.len(), 1);
        let renamed = round_trip(&added, &PresentationMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Renamed".into() })).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&renamed).1[0].name, "Renamed");
        let recropped = round_trip(&renamed, &PresentationMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "t1".into(), new_crop: FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 } })).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&recropped).1[0].crop.width, 0.4);
        let removed = round_trip(&recropped, &PresentationMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "t1".into() })).await;
        assert!(crate::artifacts::presentation::presentation_working_scene(&removed).1.is_empty());
    }

    //#region 🔖️OpTextTests
    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_create_tile() {
        let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
        test_support::assert_op_line_round_trip(&PresentationMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_delete_tile() {
        test_support::assert_op_line_round_trip(&PresentationMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "t1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_delete_tiles() {
        test_support::assert_op_line_round_trip(&PresentationMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["t1".into(), "t2".into()] }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_reorder_tiles() {
        test_support::assert_op_line_round_trip(&PresentationMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "t1".into(), to_index: 2 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_rename_tile() {
        test_support::assert_op_line_round_trip(&PresentationMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Renamed".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_resize_tile_crop() {
        let new_crop = FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 };
        test_support::assert_op_line_round_trip(&PresentationMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "t1".into(), new_crop }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_replace_source() {
        test_support::assert_op_line_round_trip(&PresentationMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: default_figure_tile_source() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_resize_source_frame() {
        test_support::assert_op_line_round_trip(&PresentationMutation::ResizeSourceFrame(resize_source_frame::mutation::ResizeSourceFrame { new_frame: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_replace_tiles() {
        let source = default_figure_tile_source();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        test_support::assert_op_line_round_trip(&PresentationMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: tiles }));
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests
