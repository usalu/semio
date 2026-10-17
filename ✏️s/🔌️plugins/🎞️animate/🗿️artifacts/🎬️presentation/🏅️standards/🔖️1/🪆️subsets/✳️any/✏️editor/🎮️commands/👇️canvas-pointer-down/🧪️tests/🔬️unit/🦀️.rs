#[allow(unused_imports)]
use crate::{editor::animate::config::PresentationConfig, editor::animate::config::PresentationConfigMutation};
#[allow(unused_imports)]
use crate::{editor::animate::support::interaction_select_effect, editor::animate::support::PresentationDispatchCtx};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
#[allow(unused_imports)]
use crate::PresentationSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::animate::commands::canvas_pointer_down::*;
use crate::editor::animate::unit_tests::context::{presentation_app_with_registry, PresentationApp};
use crate::editor::animate::{commands::add_tile, PresentationCommand};
use crate::editor::animate::support::{PRESENTATION_INTERACTION_DOMAIN};
use semio_framework_plugin::{artifact_app_laws, Effect, PluginApp};

/// 🔁️ One dispatch settled the way the plugin host settles it (ticket 26/09/16/INPUT-CAUSALITY-LEDGER
/// §2 C): the `interactionSelect` this gesture emits as `Effect::ReplayShellCommand` is folded
/// in-reactor by the typed-operation ladder, so the receipt's `effects` are exactly what the host
/// would have been handed and the selection snapshot is the witness.
async fn settled(app: &mut PresentationApp, command: PresentationCommand) -> artifact_app_laws::TypedOperationFixtureReceipt {
    let instance_id = artifact_app_laws::meta("local").instance_id;
    app.dispatch_typed(command, &artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("dispatch");
    artifact_app_laws::settle_registered_typed_operation(&mut **app, instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("retained publication settles")
}

async fn selected_tiles(app: &PresentationApp) -> Vec<String> {
    app.interaction_state().await.selection.get(PRESENTATION_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_selects_a_hit_inline_and_clears_on_miss() {
    let mut app = presentation_app_with_registry().await;
    app.bind_instance_id(artifact_app_laws::meta("local").instance_id).await;
    settled(&mut app, PresentationCommand::AddTile(add_tile::AddTile { crop: None })).await;
    let tile_id = crate::presentation_working_scene(&app.snapshot().expect("projection")).1[0].id.clone();

    let hit = settled(&mut app, PresentationCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some(tile_id.clone()) })).await;
    assert!(!hit.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "interactionSelect is folded in-reactor, never handed to the host: {:?}", hit.effects);
    assert_eq!(selected_tiles(&app).await, vec![tile_id], "the hit tile is selected inside the carrying operation");

    let miss = settled(&mut app, PresentationCommand::CanvasPointerDown(CanvasPointerDown { layer_id: Some("source-frame".into()) })).await;
    assert!(!miss.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { .. })), "the clearing interactionSelect is folded too: {:?}", miss.effects);
    assert!(selected_tiles(&app).await.is_empty(), "a miss clears the tiles selection inside the carrying operation");
    artifact_app_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
}
