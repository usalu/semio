//! 🌐️ Animate present app commands — grid seeding: seed-grid, reset-grid, clear-tiles.

use crate::apps::present::config::{PresentConfig, PresentConfigOperation};
use crate::artifacts::present::engine::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::op::PresentOperation;
use crate::artifacts::present::PresentDeck;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SeedGrid
pub mod seed_grid {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "seed-grid")]
    pub struct SeedGrid {
        pub rows: u32,
        pub columns: u32,
    }

    pub fn handle(payload: &SeedGrid, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        let deck = doc.projection;
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: payload.rows, columns: payload.columns, gap: 0.0, key_prefix: "tile" });
        let selected = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
        Ok(Emit { document_operations: vec![PresentOperation::SetTiles { tiles }], config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: selected }], ..Default::default() })
    }
}
//#endregion 🔖️SeedGrid

//#region 🔖️ResetGrid
pub mod reset_grid {
    use super::*;

    /// 🎛️ App-scope command — mirrors the pre-B1 `handle_command`-only `"animate.resetGrid"` action.
    /// Its manifest action id (`animate.resetGrid`) diverges from what the wire keyword (`reset-grid`)
    /// would suggest, which is exactly what `app_commands!`'s `"id" as "wire-key"` two-literal row
    /// exists for — see `crate::apps::present`'s invocation.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reset-grid")]
    pub struct ResetGrid {}

    pub fn handle(_payload: &ResetGrid, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        let deck = doc.projection;
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
        let selected = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
        Ok(Emit { document_operations: vec![PresentOperation::SetTiles { tiles }], config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: selected }], ..Default::default() })
    }
}
//#endregion 🔖️ResetGrid

//#region 🔖️ClearTiles
pub mod clear_tiles {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-tiles")]
    pub struct ClearTiles {}

    pub fn handle(_payload: &ClearTiles, _doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        Ok(Emit { document_operations: vec![PresentOperation::SetTiles { tiles: Vec::new() }], config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: Vec::new() }], ..Default::default() })
    }
}
//#endregion 🔖️ClearTiles

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app};
    use crate::apps::present::PresentCommand;
    use semio_framework_plugin::testkit::meta;

    #[test]
    fn seed_grid_action_adds_tiles() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }));
        assert_eq!(app.projection().expect("projection").tiles.len(), 4);
    }

    #[test]
    fn set_active_example_demo_resets_to_default_deck_after_seed() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }));
        app.dispatch_typed(PresentCommand::SetActiveExample(crate::apps::present::commands::source::set_active_example::SetActiveExample { example_id: "demo".into() }), &meta("local")).expect("reset demo");
        assert!(app.projection().expect("projection").tiles.is_empty(), "resetting to demo clears seeded tiles");
    }

    #[test]
    fn clear_tiles_action_empties_tiles_and_selection() {
        use crate::apps::present::PRESENT_PLAY_BODY_DETAILS;
        use semio_framework_plugin::{PluginApp, ViewState};
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::SeedGrid(seed_grid::SeedGrid { rows: 2, columns: 2 }));
        let first_id = app.projection().expect("projection").tiles[0].id.clone();
        dispatch(&mut app, PresentCommand::SetSelectedIds(crate::apps::present::commands::view::set_selected_ids::SetSelectedIds { ids: vec![first_id] }));
        dispatch(&mut app, PresentCommand::ClearTiles(clear_tiles::ClearTiles {}));
        assert!(app.projection().expect("projection").tiles.is_empty());
        let node = app.render(PRESENT_PLAY_BODY_DETAILS, None, &ViewState::default()).expect("render details");
        let json_str = serde_json::to_string(&node).unwrap();
        assert!(json_str.contains("Select a tile"), "selection was cleared alongside tiles");
    }
}
//#endregion 🧪️Tests
