//! 🧪️ The one cad-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
//! instead of re-deriving a store/dispatch/render scaffold of its own.
use super::*;
use protocol::{Mutation, MutationDiff};
use semio_framework_plugin::app::EditorApp;
use semio_framework_plugin::{ActionMeta, HistoryView, UiMenuRef, VcsArtifactApp};

pub fn meta(actor: &str) -> ActionMeta {
    semio_framework_plugin::testkit::meta(actor)
}

/// ✏️ `CadPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime `ArtifactApp`
/// — `EditorApp<CadPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
/// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<CadPlayApp>` builds it.
pub async fn new_app() -> VcsArtifactApp<EditorApp<CadPlayApp>> {
    semio_framework_plugin::testkit::new_app::<EditorApp<CadPlayApp>>().await
}

/// ✏️ Adapts `create_cad_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
/// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
/// framework testkit gap, not modifiable here (`🧰️framework/**` is outside this packet's lease).
pub fn cad_app_manifest_for_testkit() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_cad_app(), examples: Vec::new() }
}

pub fn empty_history() -> HistoryView {
    HistoryView::empty()
}

/// 🔀️ Keeps the legacy test-harness call shape while exercising the production action bridge —
/// `cad_command_from_action` speaks `DslValue`, so this bridges the `pack::json::Value`-shaped
/// test-harness `args` via `protocol::json::to_dsl_value` right at the call site.
pub fn command_from_action(action: &str, args: Option<&Value>) -> CadCommand {
    cad_command_from_action(action, args.map(json::to_dsl_value).as_ref()).unwrap_or_else(|error| panic!("command_from_action: {error:?}"))
}

/// 🕹️ Drives one action against a bare `CadPlayApp` (unwrapped, config defaulted) so tests can
/// inspect the emitted document/config operations directly.
pub fn drive(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>) -> Emit<CadMutation, CadConfigMutation> {
    drive_with_config(app, scene, action, args, &CadConfig::default())
}

/// 🧪️ `args` stays owned so every ported test keeps the pre-migration `(action id, json!(..))`
/// call shape verbatim; `command_from_action` only ever reads it.
///
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): dispatches straight through
/// `CadCommand::dispatch` instead of the `ArtifactApp::handle` trait method — `handle`'s
/// `interaction: &semio_framework_plugin::app::InteractionView<'_>` parameter has `pub(crate)`
/// fields in that crate, so this crate's own tests cannot construct one; `dispatch` only needs
/// the app-owned `CadDispatchCtx` (whose `interaction: CadInteractionSnapshot` field IS plain
/// and cad-owned), so tests build that by hand and skip the adaptation `handle` exists for.
#[allow(clippy::needless_pass_by_value)]
pub fn drive_with_config(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig) -> Emit<CadMutation, CadConfigMutation> {
    let operation = CadPreviewOperationIdentity { app_instance_id: 1, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) };
    drive_with_operation(app, scene, action, args, config, Some(operation)).expect("cad command handled")
}

/// 🪪️ Production-dispatch harness with an explicit public operation identity, including the
/// missing-context case used by fail-closed transition fixtures.
#[allow(clippy::needless_pass_by_value)]
pub fn drive_with_operation(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig, preview_operation: Option<CadPreviewOperationIdentity>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
    let _ = app;
    let history = empty_history();
    let doc = ArtifactView::new(scene, &history);
    let cfg = ConfigView { snapshot: config, window: None };
    let command = command_from_action(action, args.as_ref());
    let mut ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation, view_state: None };
    command.dispatch(&doc, &cfg, &mut ctx)
}

/// 🪟️ Dispatches one command with a host-authenticated concrete CAD window instance.
pub fn drive_in_window(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig, window_id: &str, window_kind_id: &str) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
    let _ = app;
    let history = empty_history();
    let doc = ArtifactView::new(scene, &history);
    let cfg = ConfigView { snapshot: config, window: None };
    let command = command_from_action(action, args.as_ref());
    let view_state = ViewModel {
        window_id: Some(window_id.into()),
        active_window_kind_id: Some(window_kind_id.into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: window_id.into(), window_kind_id: window_kind_id.into() }],
        ..ViewModel::default()
    };
    let mut ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: None, view_state: Some(view_state) };
    command.dispatch(&doc, &cfg, &mut ctx)
}

pub fn render_direct(_app: &CadPlayApp, body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let cfg = ConfigView { snapshot: config, window: None };
    CadPlayApp::render(body_key, doc, &cfg, view_state).map(|tree| tree.root)
}

pub fn window_measures_direct(_app: &CadPlayApp, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
    let cfg = ConfigView { snapshot: config, window: None };
    CadPlayApp::window_measures(doc, &cfg, view_state)
}

pub fn context_menu_direct(_app: &CadPlayApp, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
    let cfg = ConfigView { snapshot: config, window: None };
    let request = ContextMenuRequest { menu: UiMenuRef { id: "world3d".into(), args: None }, surface: None, window_instance_id: None, point: None };
    CadPlayApp::context_menu(&request, doc, &cfg, view_state, registry)
}

/// 🧮️ Folds a list of `CadMutation`s onto a scene via the core `Mutation`/`MutationDiff` impls —
/// mirrors what the wrapping `VcsArtifactApp` store does when it dispatches the emitted operations.
pub fn apply_mutations(scene: &CadSnapshot, operations: &[CadMutation]) -> CadSnapshot {
    let mut next = scene.clone();
    for operation in operations {
        next = operation.diff(&next).diff().apply(&next).expect("valid mutation diff");
    }
    next
}

/// 🧮️ `apply_mutations`'s config-targeted twin — folds an `Emit`'s `config_mutations` onto a base
/// `CadConfig` (mirrors what `VcsArtifactApp`'s config store does when it dispatches them).
pub fn config_after(emit: &Emit<CadMutation, CadConfigMutation>, base: &CadConfig) -> CadConfig {
    let mut next = base.clone();
    for operation in &emit.config_mutations {
        next = operation.diff(&next).diff().clone();
    }
    next
}

/// 🧮️ `config_after` plus the `CadConfig -> CadPlayRuntime` boundary conversion — the direct
/// replacement for the pre-B1 `app.runtime.borrow()` most tests below inspected after `drive(..)`.
pub fn runtime_after(emit: &Emit<CadMutation, CadConfigMutation>, base: &CadConfig) -> CadPlayRuntime {
    cad_runtime_from_config(&config_after(emit, base))
}

pub fn view(scene: CadSnapshot, runtime: CadPlayRuntime) -> CadPlayView {
    CadPlayView { document: scene, runtime }
}
