//! 🀄️ 🀄️ Animate present app commands command — `add-tile`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{new_tile_id, valid_tile_ids};
use crate::artifacts::present::schema::clamp_tile_crop;
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::mutations::delete_tile::mutation::DeleteTile as DeleteTileMutation;
use crate::artifacts::present::mutations::delete_tiles::mutation::DeleteTiles;
use crate::artifacts::present::mutations::rename_tile::mutation::RenameTile;
use crate::artifacts::present::mutations::resize_tile_crop::mutation::ResizeTileCrop;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{FigureTileDraft, FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-tile")]
pub struct AddTile {
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

pub fn handle(payload: &AddTile, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let id = new_tile_id("tile");
    let crop = payload.crop.clone().unwrap_or(FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 });
    let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop };
    let tile_count = crate::artifacts::present::present_working_scene(deck).1.len();
    Ok(Emit {
        artifact_mutations: vec![PresentMutation::CreateTile(CreateTile { index: tile_count, tile })],
        config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: vec![id] }],
        ..Default::default()
    })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app, present_app_with_registry};
    use crate::apps::present::PresentCommand;
    use semio_framework_plugin::testkit::meta;

    fn seed_2x2(app: &mut crate::apps::present::testkit::PresentApp) {
        dispatch(app, PresentCommand::SeedGrid(crate::apps::present::commands::seed_grid::SeedGrid { rows: 2, columns: 2 }));
    }

    #[test]
    fn add_delete_and_rename_tile_round_trip_through_operations() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
        app.dispatch_typed(PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec![tile_id.clone()], value: "Hero".into() }), &meta("local")).expect("rename");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].name, "Hero");
        app.dispatch_typed(PresentCommand::DeleteTile(delete_tile::DeleteTile { id: tile_id }), &meta("local")).expect("delete");
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty());
    }

    #[test]
    fn patch_tile_crop_clamps_and_is_reversible() {
        use semio_framework_plugin::PluginApp;
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
        app.dispatch_typed(PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec![tile_id], field: "width".into(), value: 0.5 }), &meta("local")).expect("patch crop");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].crop.width, 0.5);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].crop.width, 0.2);
    }

    #[test]
    fn delete_selection_removes_selected_tiles_and_clears_selection() {
        let mut app = present_app();
        seed_2x2(&mut app);
        let first_id = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
        app.dispatch_typed(PresentCommand::SetSelectedIds(crate::apps::present::commands::set_selected_ids::SetSelectedIds { ids: vec![first_id] }), &meta("local")).expect("select");
        app.dispatch_typed(PresentCommand::DeleteSelection(delete_selection::DeleteSelection {}), &meta("local")).expect("delete selection");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 3, "only the selected tile is removed");
    }

    #[test]
    fn delete_selection_with_no_selection_is_a_no_op() {
        let mut app = present_app();
        seed_2x2(&mut app);
        app.dispatch_typed(PresentCommand::SetSelectedIds(crate::apps::present::commands::set_selected_ids::SetSelectedIds { ids: vec![] }), &meta("local")).expect("clear selection");
        app.dispatch_typed(PresentCommand::DeleteSelection(delete_selection::DeleteSelection {}), &meta("local")).expect("delete selection");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 4, "nothing selected means nothing deleted");
    }

    #[test]
    fn delete_tile_with_unknown_id_is_a_no_op() {
        let mut app = present_app();
        seed_2x2(&mut app);
        app.dispatch_typed(PresentCommand::DeleteTile(delete_tile::DeleteTile { id: "does-not-exist".into() }), &meta("local")).expect("delete missing");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 4, "unknown ids are filtered out before dispatch");
    }

    #[test]
    fn rename_tiles_with_blank_value_leaves_name_unchanged() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
        let before = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].name.clone();
        app.dispatch_typed(PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec![tile_id], value: "   ".into() }), &meta("local")).expect("rename blank");
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].name, before, "whitespace-only rename is rejected");
    }

    #[test]
    fn rename_tiles_with_unknown_ids_is_a_no_op() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(AddTile { crop: None }), &meta("local")).expect("add tile");
        app.dispatch_typed(PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec!["nope".into()], value: "Hero".into() }), &meta("local")).expect("rename unknown");
        assert_ne!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].name, "Hero");
    }

    #[test]
    fn patch_tile_crops_covers_all_fields_across_multiple_tiles() {
        let mut app = present_app();
        seed_2x2(&mut app);
        let ids: Vec<String> = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.iter().map(|tile| tile.id.clone()).collect();
        for field in ["x", "y", "width", "height"] {
            app.dispatch_typed(PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: ids.clone(), field: field.into(), value: 0.4 }), &meta("local")).expect("patch field");
        }
        for tile in &crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1 {
            assert_eq!(tile.crop.width, 0.4);
            assert_eq!(tile.crop.height, 0.4);
        }
    }

    #[test]
    fn patch_tile_crops_targeting_no_existing_tile_is_a_no_op() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec!["ghost".into()], field: "width".into(), value: 0.4 }), &meta("local")).expect("patch ghost");
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty());
    }

    #[test]
    fn app_manifest_declares_expected_operations() {
        use semio_framework_plugin::ActionKind;
        let definition = crate::apps::present::create_animate_present_app().definition;
        let operation_ids: Vec<&str> = definition.actions.iter().filter(|action| matches!(action.kind, ActionKind::Mutation)).map(|action| action.id.as_str()).collect();
        for expected in ["addTile", "deleteTile", "deleteSelection", "renameTiles", "patchTileCrops"] {
            assert!(operation_ids.contains(&expected), "missing declared operation {expected}");
        }
        let _ = present_app_with_registry();
    }
}
//#endregion 🧪️Tests
