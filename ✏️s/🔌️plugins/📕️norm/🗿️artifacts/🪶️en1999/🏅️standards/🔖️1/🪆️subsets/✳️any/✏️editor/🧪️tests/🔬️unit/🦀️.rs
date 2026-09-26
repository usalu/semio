//! 🔬️ Editor unit-test harness for EN 1999.

pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    pub fn en1999_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_en1999_app(), examples: Vec::new() }
    }

    pub type NormApp = VcsArtifactApp<EditorApp<En1999PlayApp>>;

    pub async fn app_with_registry() -> NormApp {
        let mut app = new_app_with_registry::<EditorApp<En1999PlayApp>>(en1999_manifest_for_tests).await;
        semio_framework::io::resolve_ready(app.bind_instance_id(meta("local").instance_id));
        app
    }

    pub fn close(app: &mut NormApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(app);
    }

    pub async fn dispatch(app: &mut NormApp, command: En1999Command) -> InvocationResult {
        let mut action_meta = meta("local");
        if command.command_id() == "setSelectedCheckIndex" {
            action_meta.view_state = Some(ViewModel {
                window_id: Some(results::WINDOW_RESULTS.into()),
                focused_window_id: Some(results::WINDOW_RESULTS.into()),
                window_instances: vec![ViewWindowInstance { id: results::WINDOW_RESULTS.into(), window_kind_id: results::WINDOW_RESULTS.into() }],
                ..Default::default()
            });
        }
        let result = app.dispatch_typed(command, &action_meta).await.expect("dispatch");
        settle(app).await;
        result
    }

    pub async fn settle(app: &mut NormApp) {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id).await.expect("settle");
    }

    pub async fn render(app: &mut NormApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render projection")
    }
}

use super::*;

#[test]
fn editor_module_links() {
    let _ = create_en1999_app();
}
