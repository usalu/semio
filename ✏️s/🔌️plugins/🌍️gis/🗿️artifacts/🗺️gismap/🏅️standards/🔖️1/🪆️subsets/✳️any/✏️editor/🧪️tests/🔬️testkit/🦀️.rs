
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Gis2dApp = VcsArtifactApp<EditorApp<Gis2dPlayApp>>;

pub async fn app() -> Gis2dApp {
    new_app::<EditorApp<Gis2dPlayApp>>().await
}

/// ✏️ Adapts `create_gis2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
/// framework testkit gap, not modifiable here.
pub fn gis2d_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_gis2d_app(), examples: Vec::new() }
}

/// 🧬️ A wrapper carrying the real registry so kind discipline (View/Shell-emits-operations rejection) runs.
pub async fn app_with_registry() -> Gis2dApp {
    new_app_with_registry::<EditorApp<Gis2dPlayApp>>(gis2d_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Gis2dApp, command: Gis2dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Gis2dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render projection")
}

pub async fn main_window_measures(app: &mut Gis2dApp) -> Vec<WindowMeasure> {
    app.window_measures().await.get(map::GIS2D_PLAY_WINDOW_MAIN).cloned().unwrap_or_default()
}
