//! 🀄️ 🀄️ Animate present app commands command — `delete-selection`.

use crate::apps::present::config::{PresentConfig, PresentConfigMutation};
use crate::apps::present::{valid_tile_ids, PresentDispatchCtx};
use crate::artifacts::present::mutations::delete_tiles::mutation::DeleteTiles;
use crate::artifacts::present::op::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Reads the `tiles` domain's current selection (`ctx.selected_ids`, resolved once by
/// `ArtifactApp::handle` from `InteractionView`) instead of a deleted config field — no config
/// mutation needed afterwards: `tiles` is declared `HierarchyProvider::Flat`, so the framework never
/// auto-prunes a Flat domain's selection (see the plugin SDK's `validate_state` doc); a deleted tile's
/// stale id simply stays selected until the next real pick, a documented, accepted gap matching
/// `🖍️draw`'s `delete-layer`.
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, PresentSnapshot>, _cfg: &ConfigView<'_, PresentConfig>, ctx: &mut PresentDispatchCtx) -> Result<Emit<PresentMutation, PresentConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let targets = valid_tile_ids(deck, ctx.selected_ids.clone());
    if targets.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![PresentMutation::DeleteTiles(DeleteTiles { ids: targets })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{dispatch, present_app_with_registry};
    use crate::apps::present::{commands::add_tile, PresentCommand, PRESENT_INTERACTION_DOMAIN, PRESENT_INTERACTION_GRANULARITY};
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::{InteractionTarget, PluginApp, INTERACTION_SELECT_ACTION_ID};
    use serde_json::json;

    /// 🕹️ End-to-end proof the `tiles` domain's live selection actually drives `deleteSelection` —
    /// adds a tile, selects it via the framework's real `interactionSelect` action (the only way a
    /// downstream crate can populate a genuine `InteractionView`), then confirms `deleteSelection`
    /// removes exactly that tile.
    #[test]
    fn delete_selection_removes_the_live_selected_tile() {
        let mut app = present_app_with_registry();
        dispatch(&mut app, PresentCommand::AddTile(add_tile::AddTile { crop: None }));
        let tile_id = crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: PRESENT_INTERACTION_GRANULARITY.into(), id: tile_id.clone() }]).expect("targets");
        app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&json!({ "domainId": PRESENT_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), &meta("local")).expect("interactionSelect");
        dispatch(&mut app, PresentCommand::DeleteSelection(DeleteSelection {}));
        assert!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.is_empty(), "selected tile must be deleted");
    }

    #[test]
    fn delete_selection_with_no_selection_is_a_no_op() {
        let mut app = present_app_with_registry();
        dispatch(&mut app, PresentCommand::AddTile(add_tile::AddTile { crop: None }));
        dispatch(&mut app, PresentCommand::DeleteSelection(DeleteSelection {}));
        assert_eq!(crate::artifacts::present::present_working_scene(&app.snapshot().expect("projection")).1.len(), 1, "nothing selected means nothing deleted");
    }
}
//#endregion 🧪️Tests
