//! 🧰️ The fixture app of the editor unit suites: the real registry, one original caller scope, dispatch, settle, render and close.

use crate::modes::edit::windows::checks;
use crate::*;
use semio_plugin::artifact_app_laws::{meta, new_app_with_registry};
use semio_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

/// ✏️ Adapts `create_en1993_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `context::new_app_with_registry` still expects (framework test context gap,
/// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
pub fn en1993_manifest_for_tests() -> semio_plugin::App {
    semio_plugin::App { definition: create_en1993_app(), examples: Vec::new() }
}

/// 📜️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the app's paid close.
pub struct NormApp(pub VcsArtifactApp<EditorApp<En1993PlayApp>>, pub semio_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for NormApp {
    type Target = VcsArtifactApp<EditorApp<En1993PlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for NormApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
pub async fn app_with_registry() -> NormApp {
    let scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = new_app_with_registry::<EditorApp<En1993PlayApp>>(en1993_manifest_for_tests, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(meta("local").instance_id));
    NormApp(app, scope)
}

/// 🧹️ Closes every store the wrapper opened. A live `ArtifactStore` asserts in `Drop` unless it was
/// driven to its terminal-empty shallow shell, so every fixture that mounts an app must end here.
pub fn close(app: &mut NormApp) {
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

pub async fn dispatch(app: &mut NormApp, command: En1993Command) -> InvocationResult {
    let mut action_meta = meta("local");
    if command.command_id() == "setSelectedCheckIndex" {
        action_meta.view_state = Some(ViewModel {
            window_id: Some(checks::WINDOW_CHECKS.into()),
            focused_window_id: Some(checks::WINDOW_CHECKS.into()),
            window_instances: vec![ViewWindowInstance { id: checks::WINDOW_CHECKS.into(), window_kind_id: checks::WINDOW_CHECKS.into() }],
            ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
        });
    }
    let result = semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &action_meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
    settle(app).await;
    result
}

/// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does, draining
/// EVERY result page: a bare `maintenance_step` loop retires nothing and turns a publication fault
/// into a silent timeout, which is exactly how ticket 26/09/18 slice B2b lost the real cause.
pub async fn settle(app: &mut NormApp) {
    semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
}

pub async fn render(app: &mut NormApp, body_key: &str) -> String {
    render_with_windows(app, body_key, Vec::new()).await
}

pub async fn render_with_windows(app: &mut NormApp, body_key: &str, tree_windows: Vec<semio_framework::TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows, ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) };
    semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, &view, app.1.cx()).await.expect("render projection")
}
