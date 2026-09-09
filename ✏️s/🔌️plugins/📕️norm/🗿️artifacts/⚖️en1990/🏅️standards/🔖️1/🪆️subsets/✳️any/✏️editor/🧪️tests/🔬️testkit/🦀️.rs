use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ Adapts `create_en1990_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::new_app_with_registry` still expects (framework testkit gap,
/// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
pub fn en1990_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_en1990_app(), examples: Vec::new() }
}

pub type NormApp = VcsArtifactApp<EditorApp<En1990PlayApp>>;

/// ð§¬ï¸ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
pub async fn app_with_registry() -> NormApp {
    new_app_with_registry::<EditorApp<En1990PlayApp>>(en1990_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut NormApp, command: En1990Command) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut NormApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render projection")
}
