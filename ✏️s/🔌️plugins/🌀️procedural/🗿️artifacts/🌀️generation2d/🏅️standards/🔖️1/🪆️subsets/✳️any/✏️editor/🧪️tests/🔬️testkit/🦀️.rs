
use super::*;
use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type Generation2dApp = VcsArtifactApp<EditorApp<Generation2dPlayApp>>;

pub async fn app() -> Generation2dApp {
    new_app::<EditorApp<Generation2dPlayApp>>().await
}

pub async fn app_with_registry() -> Generation2dApp {
    new_app_with_registry::<EditorApp<Generation2dPlayApp>>(generation2d_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut Generation2dApp, command: Generation2dCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut Generation2dApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

/// ✏️ Adapts `create_generation2d_app`'s `AppDefinition` (contract §2.4) into the `App {
/// definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still
/// expects — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn generation2d_manifest_for_testkit() -> App {
    App { definition: create_generation2d_app(), examples: Vec::new() }
}
