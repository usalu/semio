use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type PlaybookApp = VcsArtifactApp<EditorApp<PlaybookPlayApp>>;

/// 🧪️ An app instance with its concrete command registry and retained job proofs.
pub async fn playbook_app() -> PlaybookApp {
    new_app_with_registry::<EditorApp<PlaybookPlayApp>>(playbook_manifest_for_testkit).await
}

/// 🧪️ Adapts `create_playbook_play_app`'s `AppDefinition` (contract §2.4) into the `App {
/// definition, examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
/// still expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
/// packet's lease).
pub fn playbook_manifest_for_testkit() -> App {
    App { definition: create_playbook_play_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
/// `kind` default declared on `addBlock` materializes host-side.
pub async fn playbook_app_with_registry() -> PlaybookApp {
    new_app_with_registry::<EditorApp<PlaybookPlayApp>>(playbook_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut PlaybookApp, command: PlaybookCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut PlaybookApp, body_key: &str) -> String {
    serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("render json")
}
