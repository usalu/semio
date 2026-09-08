
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Gis3dApp = VcsArtifactApp<EditorApp<Gis3dPlayApp>>;

pub async fn app() -> Gis3dApp {
    new_app::<EditorApp<Gis3dPlayApp>>().await
}

/// ✏️ Adapts `create_gis3d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
/// framework testkit gap, not modifiable here.
pub fn gis3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_gis3d_app(), examples: Vec::new() }
}

/// 🧬️ A wrapper carrying the real registry so kind discipline (View/Shell-emits-operations rejection) runs.
pub async fn app_with_registry() -> Gis3dApp {
    new_app_with_registry::<EditorApp<Gis3dPlayApp>>(gis3d_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Gis3dApp, command: Gis3dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Gis3dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render projection")
}
