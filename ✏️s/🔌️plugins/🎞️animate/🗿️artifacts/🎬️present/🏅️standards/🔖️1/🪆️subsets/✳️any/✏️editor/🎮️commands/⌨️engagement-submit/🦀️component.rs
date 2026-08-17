//! ⌨️ ⌨️ Animate present app commands command — `engagement-submit`.

use crate::editor::animate::config::{PresentConfig, PresentConfigMutation};
use crate::editor::animate::{interaction_select_effect, new_tile_id, tile_morph_prompt_effect, PresentDispatchCtx};
use crate::artifacts::present::schema::{parse_grid_engagement, populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::mutations::replace_tiles::mutation::ReplaceTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::{FigureTileDraft, FigureTileFrame, PresentSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: String,
}

pub fn handle(payload: &EngagementSubmit, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, _ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let trimmed = payload.value.trim();
    let (deck_source, deck_tiles) = crate::artifacts::present::present_working_scene(deck);
    if let Some((rows, columns)) = parse_grid_engagement(trimmed) {
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &deck_source, rows, columns, gap: 0.0, key_prefix: "tile" });
        let selected: Vec<String> = tiles.first().map(|tile| vec![tile.id.clone()]).unwrap_or_default();
        return Ok(Emit {
            artifact_mutations: vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })],
            config_mutations: vec![PresentConfigMutation::SetEngagementInput { value: String::new() }],
            effects: vec![interaction_select_effect(&selected, "replace")],
            ..Default::default()
        });
    }
    match trimmed.to_lowercase().as_str() {
        "add" => {
            let id = new_tile_id("tile");
            let tile = FigureTileDraft { id: id.clone(), name: id.clone(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } };
            Ok(Emit {
                artifact_mutations: vec![PresentMutation::CreateTile(CreateTile { index: deck_tiles.len(), tile })],
                config_mutations: vec![PresentConfigMutation::SetEngagementInput { value: String::new() }],
                effects: vec![interaction_select_effect(&[id], "replace")],
                ..Default::default()
            })
        }
        "clear" => Ok(Emit {
            artifact_mutations: vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: Vec::new() })],
            config_mutations: vec![PresentConfigMutation::SetEngagementInput { value: String::new() }],
            effects: vec![interaction_select_effect(&[], "replace")],
            ..Default::default()
        }),
        "copy" | "copy prompt" => Ok(Emit { config_mutations: vec![PresentConfigMutation::SetEngagementInput { value: String::new() }], effects: vec![tile_morph_prompt_effect(deck)], ..Default::default() }),
        _ => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::commands::engagement_input;
    use crate::editor::animate::testkit::{dispatch, present_app};
    use crate::editor::animate::PresentCommand;
    use semio_framework_plugin::HostEffect;

    #[test]
    fn engagement_input_stores_draft_and_submit_parses_grid_pattern() {
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::EngagementInput(engagement_input::EngagementInput { value: "2x3".into() }));
        dispatch(&mut app, PresentCommand::EngagementSubmit(EngagementSubmit { value: "2x3".into() }));
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 6, "2x3 grid pattern seeds 6 tiles");
    }

    #[test]
    fn engagement_submit_add_clear_and_copy_keywords() {
        use semio_framework_plugin::testkit::meta;
        let mut app = present_app();
        dispatch(&mut app, PresentCommand::EngagementSubmit(EngagementSubmit { value: "add".into() }));
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 1);

        dispatch(&mut app, PresentCommand::EngagementSubmit(EngagementSubmit { value: "clear".into() }));
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty());

        app.dispatch_typed(PresentCommand::AddTile(crate::editor::animate::commands::add_tile::AddTile { crop: None }), &meta("local")).expect("seed for copy");
        let copy_result = app.dispatch_typed(PresentCommand::EngagementSubmit(EngagementSubmit { value: "copy prompt".into() }), &meta("local")).expect("copy keyword");
        assert!(matches!(copy_result.requested_effects.as_slice(), [HostEffect::DownloadMediaExport { .. }]));
    }

    #[test]
    fn engagement_submit_unrecognized_input_is_a_no_op() {
        use semio_framework_plugin::testkit::meta;
        let mut app = present_app();
        let result = app.dispatch_typed(PresentCommand::EngagementSubmit(EngagementSubmit { value: "gibberish".into() }), &meta("local")).expect("unrecognized");
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
    }
}
//#endregion 🧪️Tests
