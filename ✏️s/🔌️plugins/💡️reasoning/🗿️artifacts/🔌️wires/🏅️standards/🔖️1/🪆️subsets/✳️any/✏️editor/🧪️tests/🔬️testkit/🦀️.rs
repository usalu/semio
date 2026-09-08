
use super::*;
use semio_framework_plugin::testkit::{meta, new_app as new_test_app, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type WiresApp = VcsArtifactApp<EditorApp<ReasoningWiresPlayApp>>;

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn new_app() -> WiresApp {
    new_test_app::<EditorApp<ReasoningWiresPlayApp>>().await
}

/// 🧪️ Framework testkit gap (SDK GAP, see this ticket's `📓️w0-f-report.md` handoff #3):
/// `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take `fn() -> App`,
/// unchanged for this ticket, while `create_wires_app` now returns `AppDefinition` — this tiny
/// local wrapper bridges the two shapes with an empty `examples` list (dropped per `create_wires_app`'s
/// own doc comment).
fn wires_manifest_for_testkit() -> App {
    App { definition: create_wires_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — required to resolve the "graph" interaction
/// domain's declaration when dispatching a framework-injected verb like `interactionSelect`.
pub async fn app_with_registry() -> WiresApp {
    new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(wires_manifest_for_testkit).await
}

/// 🧪️ An app pre-loaded with the metabolism example document, for tests exercising a populated board.
pub async fn metabolism_app() -> WiresApp {
    let mut app = new_app().await;
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
    let envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document, None);
    let files = store::print_document_pack(&envelope).await.expect("print document pack");
    app.load_document_pack(&files).await.expect("load metabolism");
    app
}

pub async fn dispatch(app: &mut WiresApp, command: WiresCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut WiresApp, body_key: &str) -> String {
    let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("retire rendered tree")
}
