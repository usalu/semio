//! ⌨️ Animate present app commands — the engagement bar: engagement-submit, engagement-input.

use crate::apps::present::config::{PresentConfig, PresentConfigOperation};
use crate::apps::present::{new_tile_id, tile_morph_prompt_effect};
use crate::artifacts::present::engine::{parse_grid_engagement, populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::op::PresentOperation;
use crate::artifacts::present::{FigureTileDraft, FigureTileFrame, PresentDeck};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️EngagementSubmit
pub mod engagement_submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-submit")]
    pub struct EngagementSubmit {
        pub value: String,
    }

    pub fn handle(payload: &EngagementSubmit, doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        let deck = doc.projection;
        let trimmed = payload.value.trim();
        if let Some((rows, columns)) = parse_grid_engagement(trimmed) {
            let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck.source, rows, columns, gap: 0.0, key_prefix: "tile" });
            let selected = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
            return Ok(Emit {
                document_operations: vec![PresentOperation::SetTiles { tiles }],
                config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: selected }, PresentConfigOperation::SetEngagementInput { value: String::new() }],
                ..Default::default()
            });
        }
        match trimmed.to_lowercase().as_str() {
            "add" => {
                let id = new_tile_id("tile");
                let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
                Ok(Emit {
                    document_operations: vec![PresentOperation::Tiles(protocol::CollectionOperation::Add { id: tile.id.clone(), at: deck.tiles.len(), item: tile })],
                    config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: vec![id] }, PresentConfigOperation::SetEngagementInput { value: String::new() }],
                    ..Default::default()
                })
            }
            "clear" => Ok(Emit {
                document_operations: vec![PresentOperation::SetTiles { tiles: Vec::new() }],
                config_operations: vec![PresentConfigOperation::SetSelectedIds { ids: Vec::new() }, PresentConfigOperation::SetEngagementInput { value: String::new() }],
                ..Default::default()
            }),
            "copy" | "copy prompt" => Ok(Emit { config_operations: vec![PresentConfigOperation::SetEngagementInput { value: String::new() }], effects: vec![tile_morph_prompt_effect(deck)], ..Default::default() }),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️EngagementSubmit

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &EngagementInput, _doc: &DocumentView<'_, PresentDeck>, _cfg: &ConfigView<'_, PresentConfig>) -> Result<Emit<PresentOperation, PresentConfigOperation>, Fault> {
        Ok(Emit::config(vec![PresentConfigOperation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app};
    use crate::apps::present::PresentCommand;
    use semio_framework_plugin::HostEffect;

    #[test]
    fn engagement_input_stores_draft_and_submit_parses_grid_pattern() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::EngagementInput(engagement_input::EngagementInput { value: "2x3".into() }));
        dispatch(&mut app, PresentCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "2x3".into() }));
        assert_eq!(app.projection().expect("projection").tiles.len(), 6, "2x3 grid pattern seeds 6 tiles");
    }

    #[test]
    fn engagement_submit_add_clear_and_copy_keywords() {
        use semio_framework_plugin::testkit::meta;
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "add".into() }));
        assert_eq!(app.projection().expect("projection").tiles.len(), 1);

        dispatch(&mut app, PresentCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "clear".into() }));
        assert!(app.projection().expect("projection").tiles.is_empty());

        app.dispatch_typed(PresentCommand::AddTile(crate::apps::present::commands::tile::add_tile::AddTile { crop: None }), &meta("local")).expect("seed for copy");
        let copy_result = app.dispatch_typed(PresentCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "copy prompt".into() }), &meta("local")).expect("copy keyword");
        assert!(matches!(copy_result.requested_effects.as_slice(), [HostEffect::DownloadMediaExport { .. }]));
    }

    #[test]
    fn engagement_submit_unrecognized_input_is_a_no_op() {
        use semio_framework_plugin::testkit::meta;
        let mut app = present_app();
        let result = app.dispatch_typed(PresentCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "gibberish".into() }), &meta("local")).expect("unrecognized");
        assert!(result.operations.is_empty());
        assert!(result.requested_effects.is_empty());
    }
}
//#endregion 🧪️Tests
