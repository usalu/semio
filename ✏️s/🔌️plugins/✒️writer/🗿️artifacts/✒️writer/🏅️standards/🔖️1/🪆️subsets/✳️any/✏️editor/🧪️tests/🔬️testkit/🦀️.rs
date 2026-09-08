
use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry as framework_new_app_with_registry};
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// WriterPlayApp implements the AUTHORING trait ArtifactEditor, not the runtime ArtifactApp --
/// EditorApp<WriterPlayApp> (SDK adapter, contract 2.1) is the real ArtifactApp implementor
/// VcsArtifactApp wraps, the same way PluginBuilder::editor::<WriterPlayApp> builds it.
pub type WriterApp = VcsArtifactApp<EditorApp<WriterPlayApp>>;

/// 🧪️ Constructs the Writer app with its declared command registry.
pub async fn new_app() -> WriterApp {
    framework_new_app_with_registry::<EditorApp<WriterPlayApp>>(writer_app_manifest_for_testkit).await
}

/// Adapts create_writer_app's AppDefinition (contract 2.4) into the App { definition, examples }
/// shape testkit::new_app_with_registry/assert_declared_actions_bridge_to_commands still expect --
/// framework testkit gap (framework crate outside this packet's lease), not modifiable here.
fn writer_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_writer_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn new_app_with_registry() -> WriterApp {
    framework_new_app_with_registry::<EditorApp<WriterPlayApp>>(writer_app_manifest_for_testkit).await
}

/// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
/// 🌱️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
/// see `reset_document_effect`'s doc comment), so `setActiveExample` no longer lands via
/// `dispatch_typed` alone; this loads the same document pack a real host would apply from that
/// command's `Effect::LoadDocument`, via `PluginApp::load_document_pack` directly — the same
/// technique `📐️cad`'s own `two_instances_converge_disjoint_edits_via_backbone` test uses.
pub async fn app_with_jack() -> WriterApp {
    let mut app = new_app().await;
    let document = crate::dsl::jack_example_document();
    let (schema, id) = (document.schema.clone(), document.id.clone());
    let envelope = store::create_document_envelope::<WriterSnapshot, WriterMutation>(&schema, &id, document, None);
    let files = store::print_document_pack(&envelope).await.expect("print jack document pack");
    app.load_document_pack(&files).await.expect("load jack");
    app
}

pub async fn dispatch(app: &mut WriterApp, command: WriterCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut WriterApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
}

pub async fn main_window_measures(app: &mut WriterApp) -> Vec<WindowMeasure> {
    app.window_measures().await.get(WRITER_PLAY_WINDOW_KIND).cloned().expect("main window measures")
}
