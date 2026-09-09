use super::*;
use crate::standards::v1::subsets::any::schema::mutations::{create_tile, delete_tile, delete_tiles, rename_tile, reorder_tiles, replace_source, replace_tiles, resize_source_frame, resize_tile_crop};
use crate::standards::v1::subsets::any::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::{default_figure_tile_source, default_presentation_snapshot, FigureTileDraft, FigureTileFrame, PresentationSnapshot};
use store::os_store::test_support;

async fn round_trip(deck: &PresentationSnapshot, operation: &PresentationMutation) -> PresentationSnapshot {
    let (forward, _messages) = vcs::apply_mutation(deck, operation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in protocol::Mutation::inverse(operation, deck) {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, deck, "inverse() must exactly restore the pre-operation deck");
    forward
}

#[semio_framework_async_macros::async_test]
async fn replace_tiles_and_clear_round_trip() {
    let deck = default_presentation_snapshot();
    let (source, _) = crate::presentation_working_scene(&deck);
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
    let seeded = round_trip(&deck, &PresentationMutation::ReplaceTiles(replace_tiles::ReplaceTiles { new_tiles: tiles })).await;
    assert_eq!(crate::presentation_working_scene(&seeded).1.len(), 4);
    let cleared = round_trip(&seeded, &PresentationMutation::ReplaceTiles(replace_tiles::ReplaceTiles { new_tiles: Vec::new() })).await;
    assert!(crate::presentation_working_scene(&cleared).1.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn tile_create_rename_resize_delete_round_trip() {
    let deck = default_presentation_snapshot();
    let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
    let added = round_trip(&deck, &PresentationMutation::CreateTile(create_tile::CreateTile { index: 0, tile })).await;
    assert_eq!(crate::presentation_working_scene(&added).1.len(), 1);
    let renamed = round_trip(&added, &PresentationMutation::RenameTile(rename_tile::RenameTile { id: "t1".into(), new_name: "Renamed".into() })).await;
    assert_eq!(crate::presentation_working_scene(&renamed).1[0].name, "Renamed");
    let recropped = round_trip(&renamed, &PresentationMutation::ResizeTileCrop(resize_tile_crop::ResizeTileCrop { id: "t1".into(), new_crop: FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 } })).await;
    assert_eq!(crate::presentation_working_scene(&recropped).1[0].crop.width, 0.4);
    let removed = round_trip(&recropped, &PresentationMutation::DeleteTile(delete_tile::DeleteTile { id: "t1".into() })).await;
    assert!(crate::presentation_working_scene(&removed).1.is_empty());
}

//#region 🔖️OpTextTests
#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_create_tile() {
    let tile = FigureTileDraft { id: "t1".into(), name: "A".into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
    test_support::assert_op_line_round_trip(&PresentationMutation::CreateTile(create_tile::CreateTile { index: 0, tile }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_delete_tile() {
    test_support::assert_op_line_round_trip(&PresentationMutation::DeleteTile(delete_tile::DeleteTile { id: "t1".into() }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_delete_tiles() {
    test_support::assert_op_line_round_trip(&PresentationMutation::DeleteTiles(delete_tiles::DeleteTiles { ids: vec!["t1".into(), "t2".into()] }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_reorder_tiles() {
    test_support::assert_op_line_round_trip(&PresentationMutation::ReorderTiles(reorder_tiles::ReorderTiles { id: "t1".into(), to_index: 2 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_rename_tile() {
    test_support::assert_op_line_round_trip(&PresentationMutation::RenameTile(rename_tile::RenameTile { id: "t1".into(), new_name: "Renamed".into() }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_resize_tile_crop() {
    let new_crop = FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 };
    test_support::assert_op_line_round_trip(&PresentationMutation::ResizeTileCrop(resize_tile_crop::ResizeTileCrop { id: "t1".into(), new_crop }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_replace_source() {
    test_support::assert_op_line_round_trip(&PresentationMutation::ReplaceSource(replace_source::ReplaceSource { new_source: default_figure_tile_source() }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_resize_source_frame() {
    test_support::assert_op_line_round_trip(&PresentationMutation::ResizeSourceFrame(resize_source_frame::ResizeSourceFrame { new_frame: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_replace_tiles() {
    let source = default_figure_tile_source();
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
    test_support::assert_op_line_round_trip(&PresentationMutation::ReplaceTiles(replace_tiles::ReplaceTiles { new_tiles: tiles }));
}
//#endregion 🔖️OpTextTests
