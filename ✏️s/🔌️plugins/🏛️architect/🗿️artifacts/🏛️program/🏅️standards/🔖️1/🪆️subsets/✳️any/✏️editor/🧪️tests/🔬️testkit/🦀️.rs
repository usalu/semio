
use super::*;
use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, HistoryView, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// ✏️ `ArchitectPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
/// `ArtifactApp` — `EditorApp<ArchitectPlayApp>` (SDK adapter, contract §2.1) is the real
/// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
/// `PluginBuilder::editor::<ArchitectPlayApp>` builds it.
pub type ArchitectApp = VcsArtifactApp<EditorApp<ArchitectPlayApp>>;

pub async fn new_app() -> ArchitectApp {
    sdk_new_app::<EditorApp<ArchitectPlayApp>>().await
}

/// 🚧️ SDK GAP (w0-f-report Gap 3): `new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
/// still take `fn() -> App` (the pre-migration manifest wrapper), unchanged for this ticket —
/// `create_architect_app` now returns `AppDefinition`, so wrap it in a throwaway `App` (empty
/// examples) rather than widen the framework testkit signature.
pub fn architect_app_manifest_for_testkit() -> App {
    App { definition: create_architect_app(), examples: Vec::new() }
}

/// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
pub async fn app_with_registry() -> ArchitectApp {
    new_app_with_registry::<EditorApp<ArchitectPlayApp>>(architect_app_manifest_for_testkit).await
}

pub async fn dispatch(app: &mut ArchitectApp, command: ArchitectCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut ArchitectApp, body_key: &str) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("retire app render")
}

/// 🔀️ Drives a typed `ArchitectCommand` straight through `ArchitectCommand::dispatch` — mirrors
/// `cad`'s `drive`/`drive_with_config` harness.
///
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches through
/// `ArchitectCommand::dispatch` directly instead of `ArtifactEditor::handle` — `handle`'s
/// `interaction: &semio_framework_plugin::app::InteractionView<'_>` parameter has `pub(crate)`
/// fields in that crate, so this crate's own tests cannot construct one; no `ArchitectCommand`
/// row reads the "program" domain's live selection (see `handle`'s own doc comment), so
/// `dispatch`'s plain 2-arg shape (no `ctx`) already carries everything every handler needs.
pub fn drive(command: &ArchitectCommand, program: &ProgramSnapshot) -> Emit<ProgramMutation, ArchitectConfigMutation> {
    drive_with_config(command, program, &ArchitectPlayApp::initial_config())
}

pub fn drive_with_config(command: &ArchitectCommand, program: &ProgramSnapshot, config: &ArchitectConfig) -> Emit<ProgramMutation, ArchitectConfigMutation> {
    let history = HistoryView::empty();
    let doc = ArtifactView::new(program, &history);
    let cfg = ConfigView { snapshot: config };
    command.dispatch(&doc, &cfg).expect("dispatch")
}

/// 🧮️ Folds an `Emit`'s `config_mutations` onto a base `ArchitectConfig` — mirrors what
/// `VcsArtifactApp`'s config store does when it dispatches them.
pub fn config_after(emit: &Emit<ProgramMutation, ArchitectConfigMutation>, base: &ArchitectConfig) -> ArchitectConfig {
    use protocol::Mutation;
    let mut next = base.clone();
    for operation in &emit.config_mutations {
        next = operation.diff(&next).into_parts().0;
    }
    next
}

pub fn project_render(node: semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>) -> String {
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node.expect("render"))).expect("retire semantic tree")
}

pub fn render_direct(body_key: &str, program: &ProgramSnapshot, config: &ArchitectConfig) -> String {
    let history = HistoryView::empty();
    let tree = ArchitectPlayApp::render(body_key, &ArtifactView::new(program, &history), &ConfigView { snapshot: config }, &semio_framework_plugin::ViewModel::default()).expect("editor render");
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("retire editor tree")
}
