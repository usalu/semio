use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type SequenceApp = VcsArtifactApp<EditorApp<SequencePlayApp>>;

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn new_app() -> SequenceApp {
    semio_framework_plugin::testkit::new_app::<EditorApp<SequencePlayApp>>().await
}

/// 🧩️ `create_sequence_app` now returns `AppDefinition` (contract §2.4), not the runtime-shaped
/// `App { definition, examples }` `new_app_with_registry` still expects (SDK gap, unchanged by
/// this ticket — `testkit::assert_declared_actions_bridge_to_commands` carries the identical gap
/// per `📓️w0-f-report.md` Gap 3) — wraps it with an empty `examples` list rather than porting one.
fn sequence_manifest_for_testkit() -> App {
    App { definition: create_sequence_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn new_app_with_registry_wired() -> SequenceApp {
    new_app_with_registry::<EditorApp<SequencePlayApp>>(sequence_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut SequenceApp, command: SequenceCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut SequenceApp, body_key: &str) -> String {
    let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
    let tree = semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("retire rendered tree");
    tree
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
/// injected `interactionSelect` verb, dispatched against the "steps" domain declared on this app —
/// requires `new_app_with_registry_wired().await` (a bare `new_app().await` has no declared interaction
/// domains to select against). `ids` are the steps' own raw document ids — the SAME ids the
/// "steps" domain's topology/the document panel tree/the main node-graph canvas all use.
pub async fn select_steps(app: &mut SequenceApp, ids: &[&str]) {
    let target_list: Vec<Value> = ids.iter().map(|id| serde_json::json!({ "granularity": "step", "id": id })).collect();
    let targets = serde_json::to_string(&target_list).expect("targets json");
    app.handle_action("interactionSelect", semio_framework_plugin::optional_json_to_dsl(Some(serde_json::json!({ "domainId": SEQUENCE_INTERACTION_STEPS, "targets": targets, "merge": "replace" }))).as_ref(), &meta("test"))
        .await
        .expect("interactionSelect");
}
