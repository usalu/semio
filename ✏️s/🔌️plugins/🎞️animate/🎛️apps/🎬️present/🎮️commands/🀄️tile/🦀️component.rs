//! 🀄️ Animate present app commands — tile CRUD: add, delete, delete-selection, rename, patch-crops.

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

//#region 🔖️AddTile
pub mod add_tile {
    use super::*;

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
        Ok(Emit {
            artifact_mutations: vec![PresentMutation::CreateTile(CreateTile { index: deck.tiles.len(), tile })],
            config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: vec![id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddTile

//#region 🔖️DeleteTile
pub mod delete_tile {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-tile")]
    pub struct DeleteTile {
        pub id: String,
    }

    pub fn handle(payload: &DeleteTile, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        let deck = doc.snapshot;
        let config = cfg.snapshot;
        let targets = valid_tile_ids(deck, vec![payload.id.clone()]);
        if targets.is_empty() {
            return Ok(Emit::default());
        }
        let remaining: Vec<String> = config.selected_ids.iter().filter(|selected| !targets.contains(selected)).cloned().collect();
        Ok(Emit {
            artifact_mutations: targets.into_iter().map(|id| PresentMutation::DeleteTile(DeleteTileMutation { id })).collect(),
            config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: remaining }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️DeleteTile

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, PresentSnapshot>, cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        let deck = doc.snapshot;
        let config = cfg.snapshot;
        let targets = valid_tile_ids(deck, config.selected_ids.clone());
        if targets.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit {
            artifact_mutations: vec![PresentMutation::DeleteTiles(DeleteTiles { ids: targets })],
            config_mutations: vec![PresentConfigMutation::SetSelectedIds { ids: Vec::new() }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️RenameTiles
pub mod rename_tiles {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rename-tiles")]
    pub struct RenameTiles {
        pub ids: Vec<String>,
        pub value: String,
    }

    pub fn handle(payload: &RenameTiles, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        let deck = doc.snapshot;
        let name = payload.value.trim();
        if name.is_empty() {
            return Ok(Emit::default());
        }
        let valid = valid_tile_ids(deck, payload.ids.clone());
        if valid.is_empty() {
            return Ok(Emit::default());
        }
        Ok(Emit::mutations(valid.into_iter().map(|id| PresentMutation::RenameTile(RenameTile { id, new_name: name.to_string() })).collect()))
    }
}
//#endregion 🔖️RenameTiles

//#region 🔖️PatchTileCrops
pub mod patch_tile_crops {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-tile-crops")]
    pub struct PatchTileCrops {
        pub ids: Vec<String>,
        pub field: String,
        pub value: f64,
    }

    pub fn handle(payload: &PatchTileCrops, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
        let deck = doc.snapshot;
        let targets: HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
        let operations: Vec<PresentMutation> = deck
            .tiles
            .iter()
            .filter(|tile| targets.contains(tile.id.as_str()))
            .map(|tile| {
                let mut crop = tile.crop.clone();
                match payload.field.as_str() {
                    "x" => crop.x = payload.value,
                    "y" => crop.y = payload.value,
                    "width" => crop.width = payload.value,
                    "height" => crop.height = payload.value,
                    _ => {}
                }
                PresentMutation::ResizeTileCrop(ResizeTileCrop { id: tile.id.clone(), new_crop: clamp_tile_crop(&crop) })
            })
            .collect();
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(operations))
        }
    }
}
//#endregion 🔖️PatchTileCrops

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app, present_app_with_registry};
    use crate::apps::present::PresentCommand;
    use semio_framework_plugin::testkit::meta;

    fn seed_2x2(app: &mut crate::apps::present::testkit::PresentApp) {
        dispatch(app, PresentCommand::SeedGrid(crate::apps::present::commands::grid::seed_grid::SeedGrid { rows: 2, columns: 2 }));
    }

    #[test]
    fn add_delete_and_rename_tile_round_trip_through_operations() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = app.snapshot().expect("projection").tiles[0].id.clone();
        app.dispatch_typed(PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec![tile_id.clone()], value: "Hero".into() }), &meta("local")).expect("rename");
        assert_eq!(app.snapshot().expect("projection").tiles[0].name, "Hero");
        app.dispatch_typed(PresentCommand::DeleteTile(delete_tile::DeleteTile { id: tile_id }), &meta("local")).expect("delete");
        assert!(app.snapshot().expect("projection").tiles.is_empty());
    }

    #[test]
    fn patch_tile_crop_clamps_and_is_reversible() {
        use semio_framework_plugin::PluginApp;
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = app.snapshot().expect("projection").tiles[0].id.clone();
        app.dispatch_typed(PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec![tile_id], field: "width".into(), value: 0.5 }), &meta("local")).expect("patch crop");
        assert_eq!(app.snapshot().expect("projection").tiles[0].crop.width, 0.5);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("projection").tiles[0].crop.width, 0.2);
    }

    #[test]
    fn delete_selection_removes_selected_tiles_and_clears_selection() {
        let mut app = present_app();
        seed_2x2(&mut app);
        let first_id = app.snapshot().expect("projection").tiles[0].id.clone();
        app.dispatch_typed(PresentCommand::SetSelectedIds(crate::apps::present::commands::view::set_selected_ids::SetSelectedIds { ids: vec![first_id] }), &meta("local")).expect("select");
        app.dispatch_typed(PresentCommand::DeleteSelection(delete_selection::DeleteSelection {}), &meta("local")).expect("delete selection");
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 3, "only the selected tile is removed");
    }

    #[test]
    fn delete_selection_with_no_selection_is_a_no_op() {
        let mut app = present_app();
        seed_2x2(&mut app);
        app.dispatch_typed(PresentCommand::SetSelectedIds(crate::apps::present::commands::view::set_selected_ids::SetSelectedIds { ids: vec![] }), &meta("local")).expect("clear selection");
        app.dispatch_typed(PresentCommand::DeleteSelection(delete_selection::DeleteSelection {}), &meta("local")).expect("delete selection");
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 4, "nothing selected means nothing deleted");
    }

    #[test]
    fn delete_tile_with_unknown_id_is_a_no_op() {
        let mut app = present_app();
        seed_2x2(&mut app);
        app.dispatch_typed(PresentCommand::DeleteTile(delete_tile::DeleteTile { id: "does-not-exist".into() }), &meta("local")).expect("delete missing");
        assert_eq!(app.snapshot().expect("projection").tiles.len(), 4, "unknown ids are filtered out before dispatch");
    }

    #[test]
    fn rename_tiles_with_blank_value_leaves_name_unchanged() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: None }), &meta("local")).expect("add tile");
        let tile_id = app.snapshot().expect("projection").tiles[0].id.clone();
        let before = app.snapshot().expect("projection").tiles[0].name.clone();
        app.dispatch_typed(PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec![tile_id], value: "   ".into() }), &meta("local")).expect("rename blank");
        assert_eq!(app.snapshot().expect("projection").tiles[0].name, before, "whitespace-only rename is rejected");
    }

    #[test]
    fn rename_tiles_with_unknown_ids_is_a_no_op() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::AddTile(add_tile::AddTile { crop: None }), &meta("local")).expect("add tile");
        app.dispatch_typed(PresentCommand::RenameTiles(rename_tiles::RenameTiles { ids: vec!["nope".into()], value: "Hero".into() }), &meta("local")).expect("rename unknown");
        assert_ne!(app.snapshot().expect("projection").tiles[0].name, "Hero");
    }

    #[test]
    fn patch_tile_crops_covers_all_fields_across_multiple_tiles() {
        let mut app = present_app();
        seed_2x2(&mut app);
        let ids: Vec<String> = app.snapshot().expect("projection").tiles.iter().map(|tile| tile.id.clone()).collect();
        for field in ["x", "y", "width", "height"] {
            app.dispatch_typed(PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: ids.clone(), field: field.into(), value: 0.4 }), &meta("local")).expect("patch field");
        }
        for tile in &app.snapshot().expect("projection").tiles {
            assert_eq!(tile.crop.width, 0.4);
            assert_eq!(tile.crop.height, 0.4);
        }
    }

    #[test]
    fn patch_tile_crops_targeting_no_existing_tile_is_a_no_op() {
        let mut app = present_app();
        app.dispatch_typed(PresentCommand::PatchTileCrops(patch_tile_crops::PatchTileCrops { ids: vec!["ghost".into()], field: "width".into(), value: 0.4 }), &meta("local")).expect("patch ghost");
        assert!(app.snapshot().expect("projection").tiles.is_empty());
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
