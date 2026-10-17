//! ⚖️ Laws for the flow editor's interactive-job catalog: every declared action is `Migrated`, is
//! owned by exactly one app-owned factory, and carries exactly one bounded-first-step proof — the
//! bijection `AppActionRegistry::validate_tool_job_rows` faults on with
//! `interactive-job.catalog-authority`/`interactive-job.catalog-incomplete`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use crate::editor::flow::commands::{connect_media_ports, evaluate, flow_eval_tick, rename_flow_widget, reorganize, run_extension_action, set_active_example, toggle_extension};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::editor::flow::modes::edit::windows::{compiled, main};
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel};
use crate::editor::flow::terminology::flow_play_labels;
use crate::editor::flow::base::{FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_PLAY_WINDOW_MAIN, FLOW_STORE_MAX_SCENE_ITEMS};
use crate::FlowSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_test_fixtures::fixture_text;
use serde_json::Value;
#[allow(unused_imports)]
use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
#[allow(unused_imports)]
use crate::editor::flow::commands::{add_widget, context_menu_at, delete_selection, disconnect, duplicate_widget, flow_eval_resolve, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, open_spotlight, patch_flow_widgets, remove_widget, replace_image, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode, set_contributions, set_preview_off, set_proximity_distance, spotlight_commit};
#[allow(unused_imports)]
use semio_framework_plugin::{NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
#[allow(unused_imports)]
use crate::editor::flow::modes::{edit, generate};
#[allow(unused_imports)]
use crate::editor::flow::panels::inspection as inspection_panel;
#[allow(unused_imports)]
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
#[allow(unused_imports)]
use crate::editor::flow::terminology::FlowPlayLabels;
#[allow(unused_imports)]
use crate::editor::flow::base::{flow_graph_edge_target_id, flow_graph_node_target_id, flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[allow(unused_imports)]
use crate::editor::flow::base::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
#[allow(unused_imports)]
use crate::editor::flow::host::{cold_grant, flow_content_leaves_emit, flow_removal_leaves, focus_selection_camera, seed_host_catalogue, value_fault};
#[allow(unused_imports)]
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
#[allow(unused_imports)]
use crate::{FlowMutation, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use flow::{flow_host_with_session, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::{CameraJson, Widget};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::{SynapseSpec, WidgetLayout};
#[allow(unused_imports)]
use semio_framework_os_infinite_directed_presentation::DagDrawLod;
#[allow(unused_imports)]
use semio_framework_plugin::app::{ChildEmitPreparation, InteractionView};
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppActionRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::AppDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::CommandDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuItemSpec;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuRequest;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DomainTopology;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTopology;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::TopologyNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, PortRef, SemioFlowSnapshot};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::editor::flow::{FLOW_DIRECT_STORE_TOOL_IDS, FlowDirectStoreJobFactory, FLOW_CHILD_GROUP_TOOL_IDS, FlowChildGroupJobFactory, FLOW_HOST_ONLY_TOOL_IDS, FlowHostEffectJobFactory, FLOW_GRAPH_OPERATION_TOOL_IDS, FLOW_GRAPH_OPERATION_CAPACITY, FlowGraphOperationJobFactory, flow_graph_operation_contract, FLOW_CONTRIBUTIONS_TOOL_IDS, FlowContributionsJobFactory};
use crate::editor::flow::*;
use crate::editor::flow::unit_tests::context::flow_app_closing;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use std::collections::{BTreeMap, BTreeSet};

static FLOW_INTERACTIVE_JOB_FIXTURE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧫️fixtures/🧮️interactive-job/🔣️.json"));

/// 🧫️ The language-agnostic declaration this whole lane is measured against.
fn fixture() -> Value {
    serde_json::from_str((*FLOW_INTERACTIVE_JOB_FIXTURE)).expect("flow interactive-job fixture")
}

/// 🧾️ The declared migrated ids, owned so callers can hold them past the fixture temporary.
fn migrated_ids() -> BTreeSet<String> {
    fixture()["migrated"].as_array().expect("fixture migrated").iter().map(|id| id.as_str().expect("fixture id").to_string()).collect()
}

fn fixture_tools(factory: &str) -> Vec<String> {
    fixture()["factories"]
        .as_array()
        .expect("fixture factories")
        .iter()
        .find(|row| row["factory"] == factory)
        .unwrap_or_else(|| panic!("fixture row for {factory}"))["tools"]
        .as_array()
        .expect("fixture factory tools")
        .iter()
        .map(|tool| tool.as_str().expect("fixture tool id").to_string())
        .collect()
}

/// 🏷️ Every `id`/`semantics.execution.interactiveJob` pair the built manifest declares, at any depth —
/// app actions, window-kind actions, app commands and mode commands alike, exactly the four places
/// `AppActionRegistry::migrated_tool_ids` reads.
fn declared_classifications() -> BTreeMap<String, String> {
    fn walk(value: &Value, out: &mut BTreeMap<String, String>) {
        match value {
            Value::Object(map) => {
                if let (Some(Value::String(id)), Some(Value::String(job))) = (map.get("id"), value.pointer("/semantics/execution/interactiveJob")) {
                    out.insert(id.clone(), job.clone());
                }
                for nested in map.values() {
                    walk(nested, out);
                }
            }
            Value::Array(rows) => rows.iter().for_each(|row| walk(row, out)),
            _ => {}
        }
    }
    let definition = serde_json::to_value(create_flow_app()).expect("flow app definition json");
    let mut out = BTreeMap::new();
    walk(&definition, &mut out);
    out
}

/// ⚖️ LAW: the five app-owned factories partition every declared `FlowCommand` tool id exactly —
/// no id owned twice, none left to the batch `handle` fallback — and each declares an exact,
/// nonempty publication-lane contract for every id it owns. Mirrors generation3d's
/// `retained_route_dispositions_are_exact_and_exhaustive`.
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    let routes: [(&str, &[&str], &[semio_framework_plugin::ArtifactToolPublicationContract]); 5] = [
        ("FlowDirectStoreJobFactory", FLOW_DIRECT_STORE_TOOL_IDS, FlowDirectStoreJobFactory::PUBLICATION_CONTRACTS),
        ("FlowChildGroupJobFactory", FLOW_CHILD_GROUP_TOOL_IDS, FlowChildGroupJobFactory::PUBLICATION_CONTRACTS),
        ("FlowHostEffectJobFactory", FLOW_HOST_ONLY_TOOL_IDS, FlowHostEffectJobFactory::PUBLICATION_CONTRACTS),
        ("FlowGraphOperationJobFactory", FLOW_GRAPH_OPERATION_TOOL_IDS, FlowGraphOperationJobFactory::PUBLICATION_CONTRACTS),
        ("FlowContributionsJobFactory", FLOW_CONTRIBUTIONS_TOOL_IDS, FlowContributionsJobFactory::PUBLICATION_CONTRACTS),
    ];
    let mut owned: BTreeMap<&str, &str> = BTreeMap::new();
    for (factory, tool_ids, publication) in routes {
        assert_eq!(tool_ids.iter().map(|id| (*id).to_string()).collect::<Vec<_>>(), fixture_tools(factory), "{factory} tool ids must equal the fixture declaration");
        assert_eq!(publication.len(), tool_ids.len(), "{factory} publishes one lane contract per tool");
        for contract in publication {
            assert!(tool_ids.contains(&contract.tool_id), "{factory} publishes a lane contract for an unowned tool {}", contract.tool_id);
            assert!(!contract.lanes.is_empty(), "{factory} tool {} declares no publication lane", contract.tool_id);
        }
        for tool_id in tool_ids {
            assert!(owned.insert(tool_id, factory).is_none(), "tool {tool_id} is owned by more than one factory");
        }
    }
    let declared = FlowCommand::TOOL_JOB_IDS.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(owned.keys().copied().collect::<BTreeSet<_>>(), declared, "every declared FlowCommand row must be owned by exactly one app-owned factory");
    assert_eq!(declared.len(), fixture()["migrated"].as_array().expect("fixture migrated").len());
}

/// ⚖️ LAW: the manifest declares `Migrated` for every one of those ids and
/// `BatchOnlyPendingRewrite` for none — the `expected` half of the catalog join. A single
/// `BatchOnlyPendingRewrite` row here is what panicked `FlowPlayApp` at construction on the wgpu
/// shell (`📓️runtime-verification-2026-09-09.md` boot #1).
#[test]
fn every_declared_flow_action_is_migrated() {
    let declared = declared_classifications();
    let migrated = migrated_ids();
    let batch_only = declared.iter().filter(|(_, job)| *job == "batchOnlyPendingRewrite").map(|(id, _)| id.clone()).collect::<Vec<_>>();
    assert!(batch_only.is_empty(), "flow still declares batch-only actions: {batch_only:?}");
    assert_eq!(fixture()["batchOnlyPendingRewrite"].as_array().expect("fixture batch-only").len(), 0);
    for id in &migrated {
        assert_eq!(declared.get(id).map(String::as_str), Some("migrated"), "action {id} must be declared Migrated");
    }
}

/// ⚖️ LAW: the aggregated bounded-first-step proof set is a bijection onto those ids — one proof per
/// tool, none duplicated, none missing. `validate_tool_job_rows` compares exactly these two sets
/// (`seen != expected` → `interactive-job.catalog-incomplete`).
#[test]
fn bounded_first_step_proofs_are_a_bijection_onto_the_migrated_ids() {
    let proofs = <FlowPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs();
    let ids = proofs.iter().map(|proof| proof.tool_id()).collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), proofs.len(), "a tool may carry at most one proof row");
    let expected = migrated_ids();
    assert_eq!(ids.iter().map(|id| (*id).to_string()).collect::<BTreeSet<_>>(), expected, "proof rows and migrated declarations must be one set");
    assert_eq!(proofs.len(), FLOW_DIRECT_STORE_TOOL_IDS.len() + FLOW_HOST_ONLY_TOOL_IDS.len() + FLOW_CHILD_GROUP_TOOL_IDS.len() + FLOW_GRAPH_OPERATION_TOOL_IDS.len() + FLOW_CONTRIBUTIONS_TOOL_IDS.len());
}

/// ⚖️ LAW: the graph-operation route declares ONE capacity — the same `ArtifactRetainedWorkCapacity`
/// its payload carries as `maximum_work_items`, its extent answers in, and its contract publishes
/// (`📓️work-capacity-2026-09-10.md`).
#[test]
fn graph_operation_route_declares_one_work_capacity() {
    let contract = flow_graph_operation_contract();
    assert_eq!(contract.max_raw_wire_bytes, FLOW_GRAPH_OPERATION_RAW_BYTES);
    assert_eq!(semio_framework::ToolJobFactory::execution_contract(&FlowGraphOperationJobFactory::new("s.flow.flow@1/*#editor")), contract);
    assert_eq!(semio_framework::ToolJobFactory::classification(&FlowGraphOperationJobFactory::new("s.flow.flow@1/*#editor")), semio_framework_plugin::InteractiveJobClassification::Migrated);
    assert_eq!(FLOW_GRAPH_OPERATION_CAPACITY.work_items(), FLOW_GRAPH_OPERATION_CAPACITY.rows(FLOW_STORE_MAX_SCENE_ITEMS + 1));
    assert_eq!(FLOW_GRAPH_OPERATION_CAPACITY.rows_for_items(1), Some(FLOW_GRAPH_OPERATION_CAPACITY.rows(1)));
    assert!(FLOW_GRAPH_OPERATION_CAPACITY.admits(FLOW_GRAPH_OPERATION_CAPACITY.rows(FLOW_STORE_MAX_SCENE_ITEMS + 1)));
    assert!(!FLOW_GRAPH_OPERATION_CAPACITY.admits(FLOW_GRAPH_OPERATION_CAPACITY.rows(FLOW_STORE_MAX_SCENE_ITEMS + 2)));
}

/// ⚖️ LAW: `FlowPlayApp` instantiates through the REAL manifest registry — the exact construction
/// that ran `validate_tool_job_rows` and faulted `interactive-job.catalog-authority` on the wgpu
/// playground, aborting the process before the first step. A booted app renders its main body, so
/// the app is live and not merely constructed.
#[semio_framework_async_macros::async_test]
async fn flow_play_app_boots_through_the_real_registry_without_a_catalog_authority_fault() {
    let mut app = flow_app_closing().await;
    assert!(!semio_framework_plugin::PluginApp::has_pending_typed_operations(&*app), "a freshly booted flow app owns no in-flight typed operation");
    let demand = semio_framework_plugin::PluginApp::maintenance_retirement_demands(&*app, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("a booted flow app quotes its own maintenance turn");
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
    assert!(matches!(semio_framework_plugin::PluginApp::maintenance_step(&mut *app, grant), Ok(_)), "a booted flow app runs its own maintenance turn");
}

/// ⚖️ LAW: every graph-operation route is admitted by `FlowGraphOperationJobFactory` and returns a
/// retained admission receipt (`{operationId, generation}`) — never the batch `handle`, which fails
/// closed for all four retained families now, and never an `interactive-job.catalog-authority`
/// fault. Every admitted operation must complete before the next route is probed so this law also
/// proves that each factory relinquishes its retained owner.
#[semio_framework_async_macros::async_test]
async fn every_graph_operation_route_is_admitted_by_its_own_retained_factory() {
    use crate::editor::flow::unit_tests::context::{dispatch, settle};
    let mut app = flow_app_closing().await;
    dispatch(&mut app, FlowCommand::ToggleExtension(toggle_extension::ToggleExtension { id: "auto-layout".into(), enabled: true })).await;
    settle(&mut app).await;
    let commands = [
        FlowCommand::Reorganize(reorganize::Reorganize {}),
        FlowCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "b".into() }),
        FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "preview".into(), value: "graph-operation-probe".into() }),
        FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }),
        FlowCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() }),
    ];
    assert_eq!(commands.len(), FLOW_GRAPH_OPERATION_TOOL_IDS.len(), "one probe per graph-operation route");
    for command in commands {
        let tool_id = command.command_id();
        assert!(FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id));
        let result = dispatch(&mut app, command).await;
        assert!(!format!("{result:?}").contains("legacy-dispatch"), "{tool_id} must not fall back to the batch handle: {result:?}");
        assert!(!format!("{result:?}").contains("catalog-authority"), "{tool_id} must not fault the catalog: {result:?}");
        assert!(result.output.as_object().is_some_and(|fields| fields.iter().any(|(key, _)| key == "operationId")), "{tool_id} must return a retained admission receipt, not a batch emit: {result:?}");
        settle(&mut app).await;
    }
}

/// ⚖️ LAW: the batch `handle` fallback is unreachable for every declared route — every id is
/// retained-owned, so the legacy path fails closed rather than silently diverging from the job.
#[test]
fn the_batch_handle_fallback_is_closed_for_every_declared_route() {
    for tool_id in FlowCommand::TOOL_JOB_IDS {
        assert!(
            FLOW_DIRECT_STORE_TOOL_IDS.contains(tool_id) || FLOW_CHILD_GROUP_TOOL_IDS.contains(tool_id) || FLOW_HOST_ONLY_TOOL_IDS.contains(tool_id) || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(tool_id) || FLOW_CONTRIBUTIONS_TOOL_IDS.contains(tool_id),
            "tool {tool_id} would reach the batch handle fallback"
        );
    }
}

/// ⚖️ LAW (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP): the live boot of the flow editor —
/// render every host-backed body (main, compiled, catalogue, document panel), refuse an unservable
/// boot evaluation, and probe its first tick — never drops a live `FlowHost`/`FlowHostSnapshot`/`FlowEvalSession`. Each
/// of those owns an `OrderedMap` layout root (or a close-witnessed session) that aborts the guest on a
/// bare drop ("ordered-map root must be explicitly retired before drop"); in the browser that abort
/// happened inside the instance owner's `with_mut`, so every later call answered "runtime instance
/// authority is busy". Driven below the app so the law is independent of the app's close ladder.
#[test]
fn booting_renders_and_evaluates_without_dropping_a_live_flow_owner() {
    use semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree;
    let snapshot = FlowSnapshot::default();
    let config = main::config::FlowMainWindowConfig::default();
    let mut session = FlowEvalSession::new();
    let view_state = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let labels = flow_play_labels(&view_state);
    let bodies = [
        main::render(&snapshot, &config, &session, &[]),
        compiled::render(&snapshot, &config, &session),
        catalogue_panel::render(&snapshot, &config, &session, labels, &semio_framework_plugin::TreeWindows::for_body(&view_state, FLOW_PLAY_BODY_CATALOGUE)),
        document_panel::render(&snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(&view_state, FLOW_PLAY_BODY_ARTIFACT)),
    ];
    for body in bodies {
        let tree = project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(body.expect("flow body renders"))).expect("rendered flow body retires");
        assert!(!tree.is_empty());
    }
    let armed = evaluate::evaluate_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN);
    assert!(armed.effects.len() <= 1, "one probe arms at most this window's single flowEvalTick continuation");
    // ♻️ The anti-spin law, stated so it does not depend on WHO ran first: the flow extension
    // registry is process-global, so whether the starter graph's `math.add` is servable depends on
    // whether a sibling law already installed the `math` fixture module. Asserting "no effect at all"
    // was a claim about a bare registry this law does not own — it passed alone and failed in the
    // suite (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). What must hold either way is that a
    // SECOND probe of the same unchanged snapshot arms nothing: that is the latch the spin fix added.
    let rearmed = evaluate::evaluate_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN);
    assert!(rearmed.effects.is_empty(), "a second probe of the same unchanged snapshot must not arm a second tick");
    let _ = flow_eval_tick::tick_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN);
    session.retire_cold();
}
