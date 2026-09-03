//! 🀄️ 🀄️ Animate presentation app commands command — `delete-selection`.

#![allow(clippy::result_large_err)]

use crate::artifacts::presentation::mutations::delete_tiles::mutation::DeleteTiles;
use crate::artifacts::presentation::op::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{valid_tile_ids, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Reads the `tiles` domain's current selection (`ctx.selected_ids`, resolved once by
/// `ArtifactApp::handle` from `InteractionView`) instead of a deleted config field — no config
/// mutation needed afterwards: `tiles` is declared `HierarchyProvider::Flat`, so the framework never
/// auto-prunes a Flat domain's selection (see the plugin SDK's `validate_state` doc); a deleted tile's
/// stale id simply stays selected until the next real pick, a documented, accepted gap matching
/// `🖍️draw`'s `delete-layer`.
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    let deck = doc.snapshot;
    let targets = valid_tile_ids(deck, ctx.selected_ids.clone());
    if targets.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![PresentationMutation::DeleteTiles(DeleteTiles { ids: targets })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{dispatch, presentation_app_with_registry};
    use crate::editor::animate::{commands::add_tile, PresentationCommand, PRESENTATION_INTERACTION_DOMAIN, PRESENTATION_INTERACTION_GRANULARITY};
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::{PluginApp, INTERACTION_SELECT_ACTION_ID};

    /// 🕹️ End-to-end proof the `tiles` domain's live selection actually drives `deleteSelection` —
    /// adds a tile, selects it via the framework's real `interactionSelect` action (the only way a
    /// downstream crate can populate a genuine `InteractionView`), then confirms `deleteSelection`
    /// removes exactly that tile.
    #[semio_framework_async_macros::async_test]
    async fn delete_selection_removes_the_live_selected_tile() {
        let mut app = presentation_app_with_registry().await;
        dispatch(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
        let tile_id = crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).1[0].id.clone();
        let targets = dsl::os_pack::json::to_string(&dsl::os_pack::json::Value::Array(vec![dsl::os_pack::json::object([
            ("granularity".to_string(), dsl::os_pack::json::Value::from(PRESENTATION_INTERACTION_GRANULARITY)),
            ("id".to_string(), dsl::os_pack::json::Value::from(tile_id.clone())),
        ])]));
        let args = dsl::DslValue::object([
            ("domainId".to_string(), dsl::DslValue::String(PRESENTATION_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), dsl::DslValue::String(targets)),
            ("merge".to_string(), dsl::DslValue::String("replace".into())),
            ("method".to_string(), dsl::DslValue::String("pick".into())),
        ]);
        app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interactionSelect");
        dispatch(&mut app, PresentationCommand::DeleteSelection(DeleteSelection {})).await;
        assert!(crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).1.is_empty(), "selected tile must be deleted");
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_selection_with_no_selection_is_a_no_op() {
        let mut app = presentation_app_with_registry().await;
        dispatch(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
        dispatch(&mut app, PresentationCommand::DeleteSelection(DeleteSelection {})).await;
        assert_eq!(crate::artifacts::presentation::presentation_working_scene(&app.snapshot().await.expect("projection")).1.len(), 1, "nothing selected means nothing deleted");
    }
}
//#endregion 🧪️Tests
