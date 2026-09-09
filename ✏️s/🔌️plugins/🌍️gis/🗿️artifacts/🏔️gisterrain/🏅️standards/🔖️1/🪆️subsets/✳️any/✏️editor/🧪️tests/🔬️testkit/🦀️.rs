use super::*;
use semio_framework_plugin::testkit::{close_registered_fixture_app, meta, new_app_with_registry, settle_registered_typed_operation, TypedOperationFixtureReceipt};
use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

pub type Gis3dApp = VcsArtifactApp<EditorApp<Gis3dPlayApp>>;

/// 🧬️ Builds the real registered fixture and binds the instance addressed by [`meta`].
pub async fn app() -> Gis3dApp {
    let mut app = new_app_with_registry::<EditorApp<Gis3dPlayApp>>(gis3d_app_manifest_for_testkit).await;
    app.bind_instance_id(meta("local").instance_id).await;
    app
}

/// ✏️ Adapts `create_gis3d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
/// framework testkit gap, not modifiable here.
pub fn gis3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_gis3d_app(), examples: Vec::new() }
}

/// 🪟️ Targets the real Terrain window instance for render and command authority.
pub fn main_window_view() -> ViewModel {
    ViewModel {
        window_id: Some(modes::view::windows::terrain::GIS3D_PLAY_WINDOW_MAIN.into()),
        window_instances: vec![ViewWindowInstance { id: modes::view::windows::terrain::GIS3D_PLAY_WINDOW_MAIN.into(), window_kind_id: modes::view::windows::terrain::GIS3D_PLAY_WINDOW_MAIN.into() }],
        ..Default::default()
    }
}

/// 🧹️ Drives a Terrain fixture to its exact terminal-empty ownership witness.
pub fn close(app: &mut Gis3dApp) {
    close_registered_fixture_app(app);
}

/// 🎯️ Dispatches a typed Terrain command and completes its bounded host publication protocol.
pub async fn dispatch(app: &mut Gis3dApp, command: Gis3dCommand) -> TypedOperationFixtureReceipt {
    let mut action_meta = meta("local");
    action_meta.view_state = Some(main_window_view());
    let admission = app.dispatch_typed(command, &action_meta).await.expect("dispatch");
    assert!(admission.mutations.is_empty(), "retained Terrain commands publish only through acknowledged result pages");
    settle_registered_typed_operation(app, action_meta.instance_id).await.expect("settle Terrain dispatch")
}

pub async fn render(app: &mut Gis3dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &main_window_view()).await.expect("render")).expect("render projection")
}
