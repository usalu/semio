//! 🧰️ Flow app fixture harness shared by the unit test crates: the self-closing app fixture, dispatch, settle and render helpers and the registered laws.

use crate::editor::flow::*;
use crate::editor::flow::commands::{add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode, set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use crate::editor::flow::modes::edit::windows::compiled;
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use crate::editor::flow::modes::{edit, generate};
use crate::editor::flow::editor_vocabulary::{flow_graph_edge_target_id, flow_graph_node_target_id, FLOW_INTERACTION_GRAPH, FLOW_PLAY_WINDOW_MAIN};
use crate::editor::flow::editor_vocabulary::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
use crate::editor::flow::host::flow_removal_leaves;
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
use crate::{FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
use flow::FlowEvalSession;
use semio_framework_artifact_flow_flow::{CameraJson, Widget};
use semio_framework_artifact_flow_flow::{SynapseSpec, WidgetLayout};
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::Fault;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::WindowMeasure;
use serde_json::json;
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use serde_json::Value;
use flow::neural::ColdRetire;
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
use semio_s_artifact_stdio_semio::SemioMembers;
use pack::snapshot::ArtifactPack;

pub type FlowApp = VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>;

/// 🧹️ A live app fixture that CLOSES itself. `VcsArtifactApp`'s `ArtifactStore` owns an
/// `ArtifactStoreCursorDisposer` whose `Drop` asserts terminal-empty ownership, so a plainly-dropped
/// fixture panics with "artifact store reached Drop without its exact terminal-empty shallow-shell
/// witness". Dereferences to the app and drains the exact retained close ladder
/// (`PluginApp::close_step`) on the way out — the same law the runtime uses. Mirrors generation3d's
/// `Generation3dAppFixture` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct FlowAppFixture(pub FlowApp, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

impl std::ops::Deref for FlowAppFixture {
    type Target = FlowApp;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FlowAppFixture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for FlowAppFixture {
    fn drop(&mut self) {
        if std::thread::panicking() || self.0.close_terminal_is_empty() {
            return;
        }
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
    }
}

/// 🧪️ A self-closing app wired to the real manifest registry — the ONE construction a law that only
/// proves boot/dispatch should use, so the store's own terminal-empty witness is honoured. The
/// `content` child is composed by the runtime itself from `FlowPlayApp::genesis_child_pack`, the
/// same way a live shell composes it, so no fixture registers a member by hand any more.
pub async fn flow_app_closing() -> FlowAppFixture {
    install_first_party_light_flow_extensions_for_tests();
    let definition = create_flow_app();
    let registry = AppActionRegistry::from_definition(&definition);
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = VcsArtifactApp::<EditorApp<FlowPlayApp>, SemioMembers>::with_registry(EditorApp::default(), registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(meta("local").instance_id).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
    FlowAppFixture(app, scope)
}

semio_framework_plugin::history_edit_acceptance_law!("flow", FlowPlayApp, || semio_framework_plugin::App { definition: create_flow_app(), examples: Vec::new() }, "../..");
semio_framework_plugin::composed_reload_law!("flow", FlowPlayApp, || semio_framework_plugin::App { definition: create_flow_app(), examples: Vec::new() }, "../..");
semio_framework_plugin::composed_child_history_law!("flow", FlowPlayApp, || semio_framework_plugin::App { definition: create_flow_app(), examples: Vec::new() }, [("nodeGraphEdit", r#"{"operations":[{"operation":"move","gestureId":"node-drag:1","nodeIds":["add"],"dx":25.0,"dy":5.0}]}"#)]);

/// 🧪️ Installs a hand-authored `flow.extension` manifest fixture (a "math" module contributing the
/// `math.add` operator) so tests exercising the catalogue/extension surfaces have something real
/// installed — deliberately NOT the production `flow-extension-*` crates: flow-core must not
/// dev-depend on its own extensions (audit finding C1, see ticket
/// `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`'s `w3-flow.md`). Each real extension crate already
/// exhaustively tests its own manifest/operator content in its own `#[cfg(test)] mod tests` (e.g.
/// `flow-extension-math`'s `manifest_lists_math_operators_and_schemas`); this fixture only covers
/// what flow-core's own tests assert on (`catalogue_lists_module_operators`).
pub fn install_first_party_light_flow_extensions_for_tests() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let manifest = flow::FlowExtensionManifest {
            schema: "flow.extension".into(),
            id: "math".into(),
            name: "Math".into(),
            version: "0.0.0-test-fixture".into(),
            activation_events: vec!["onStartup".into()],
            contributes: flow::FlowExtensionContributes {
                schemas: vec![],
                operators: vec![flow::neural::OperatorInfo {
                    id: "math.add".into(),
                    extension: "math".into(),
                    name: "Add".into(),
                    abbreviation: "Add".into(),
                    inputs: vec![
                        flow::neural::ChannelSpec::number("a", &["core.number"]),
                        flow::neural::ChannelSpec::number_default("b", 0.0, &["core.number"]),
                    ],
                    outputs: vec![flow::neural::ChannelSpec::named("S", "Sum", "sum", "Sum")],
                    ..Default::default()
                }],
                widgets: vec![],
                commands: vec![],
                settings: vec![],
            },
        };
        let manifest_json = semio_framework_pack_json::to_json_string(&manifest);
        for schema in manifest.contributes.schemas { schema.retire_cold(); }
        for operator in manifest.contributes.operators { operator.retire_cold(); }
        flow::install_flow_extension_manifest("flow-core-test-fixture", &manifest_json).expect("fixture extension admission");
    });
}

/// 🧪️ Uses the real registered application so every concrete tool factory has declared authority.
pub async fn flow_app() -> FlowAppFixture {
    flow_app_with_registry().await
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn flow_app_with_registry() -> FlowAppFixture {
    flow_app_with_actor(semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await
}

/// 🎭️ Constructs a registered fixture whose retained sessions belong to the explicit actor.
pub async fn flow_app_with_actor(actor: semio_framework_os_kernel::ActorId) -> FlowAppFixture {
    install_first_party_light_flow_extensions_for_tests();
    let definition = create_flow_app();
    let registry = AppActionRegistry::from_definition(&definition);
    let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = VcsArtifactApp::<EditorApp<FlowPlayApp>, SemioMembers>::with_registry(EditorApp::default(), registry, actor, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(meta("local").instance_id).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
    FlowAppFixture(app, scope)
}

pub fn flow_main_window_meta() -> semio_framework_plugin::ActionMeta {
    let window = semio_framework_plugin::ViewWindowInstance { id: FLOW_PLAY_WINDOW_MAIN.into(), window_kind_id: FLOW_PLAY_WINDOW_MAIN.into() };
    semio_framework_plugin::ActionMeta {
        view_state: Some(ViewModel {
            window_id: Some(window.id.clone()),
            active_window_kind_id: Some(window.window_kind_id.clone()),
            window_instances: vec![window],
            ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        }),
        ..meta("local")
    }
}

pub async fn dispatch(app: &mut FlowAppFixture, command: FlowCommand) -> InvocationResult {
    app.0.dispatch_typed(command, &flow_main_window_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch")
}

pub async fn dispatch_with_registry(app: &mut FlowAppFixture, command: FlowCommand) -> InvocationResult {
    app.0.dispatch_typed(command, &flow_main_window_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch")
}

pub async fn settle(app: &mut FlowAppFixture) {
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("retained Flow command publication");
}

/// 🚫️ Settles a refused retained command while preserving its typed code and message.
pub async fn settle_refusal(app: &mut FlowAppFixture) -> semio_framework_plugin::Fault {
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.err().expect("the handler refuses by name")
}

/// 🪆️ The scene the window renders and every verb edits: the composed content child, decoded from its own store —
/// never the parent's genesis owner (see `crate::flow_composed_snapshot`).
pub async fn composed_scene(app: &FlowApp) -> semio_framework_artifact_flow_flow::FlowHostSnapshot {
    use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
    use store::SpaceMember;
    let snapshot = app.snapshot().expect("snapshot");
    let child = SemioFlowSnapshot::decode_pack(&app.child_store("content", &snapshot.content.child_id).await.expect("composed content child").document_pack_bytes().await.expect("content child pack")).expect("content child snapshot");
    let (widgets, synapses, layout) = crate::working_from_flow_content_snapshot(&child);
    FlowSnapshot { schema: snapshot.schema.clone(), content: crate::flow_content_child_handle_and_cache(widgets, synapses, layout) }.to_host_snapshot()
}

pub async fn render_with_view(app: &mut FlowAppFixture, body_key: &str, view: &ViewModel) -> String {
    semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, view, app.1.cx()).await.expect("rendered fixture observation and retirement")
}

pub async fn render(app: &mut FlowAppFixture, body_key: &str) -> String {
    let view = flow_main_window_meta().view_state.expect("Flow main window view");
    render_with_view(app, body_key, &view).await
}

pub async fn main_window_measures(app: &mut FlowAppFixture) -> Vec<WindowMeasure> {
    let window = semio_framework_plugin::ViewWindowInstance { id: FLOW_PLAY_WINDOW_MAIN.into(), window_kind_id: FLOW_PLAY_WINDOW_MAIN.into() };
    let view = ViewModel { window_id: Some(window.id.clone()), active_window_kind_id: Some(window.window_kind_id.clone()), window_instances: vec![window], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    app.window_measures(&view).await.get(FLOW_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is the framework's injected
/// `interactionSelect` verb now, dispatched against the "graph" domain declared on this app —
/// requires `flow_app_with_registry()` (a bare `flow_app()` has no declared interaction domains to
/// select against). `node_ids`/`edge_ids` are raw widget/synapse ids, converted to the row-id-
/// prefixed `InteractionTarget` ids the document panel tree/`interaction_topology` both use (see
/// `flow_graph_node_target_id`/`flow_graph_edge_target_id`).
pub async fn select_graph(app: &mut FlowAppFixture, node_ids: &[&str], edge_ids: &[&str]) {
    let mut targets: Vec<Value> = node_ids.iter().map(|id| serde_json::json!({ "granularity": "node", "id": flow_graph_node_target_id(id) })).collect();
    targets.extend(edge_ids.iter().map(|id| serde_json::json!({ "granularity": "edge", "id": flow_graph_edge_target_id(id) })));
    let targets_json = serde_json::to_string(&targets).expect("targets json");
    let args = semio_framework_value::DslValue::from(serde_json::json!({ "domainId": FLOW_INTERACTION_GRAPH, "targets": targets_json, "merge": "replace" }));
    let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "interactionSelect", Some(args.clone()), &flow_main_window_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("interactionSelect admission");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await.expect("interactionSelect reserved-job commit");
}
