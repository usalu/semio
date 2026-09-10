//! ⚖️ Laws for the flow editor's interactive-job catalog: every declared action is `Migrated`, is
//! owned by exactly one app-owned factory, and carries exactly one bounded-first-step proof — the
//! bijection `AppActionRegistry::validate_tool_job_rows` faults on with
//! `interactive-job.catalog-authority`/`interactive-job.catalog-incomplete`
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::*;
use crate::editor::flow::testkit::flow_app_closing;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use std::collections::{BTreeMap, BTreeSet};

const FLOW_INTERACTIVE_JOB_FIXTURE: &str = include_str!("../../🧫️fixtures/🧮️interactive-job/🔣️.json");

/// 🧫️ The language-agnostic declaration this whole lane is measured against.
fn fixture() -> Value {
    serde_json::from_str(FLOW_INTERACTIVE_JOB_FIXTURE).expect("flow interactive-job fixture")
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

/// ⚖️ LAW: the four app-owned factories partition every declared `FlowCommand` tool id exactly —
/// no id owned twice, none left to the batch `handle` fallback — and each declares an exact,
/// nonempty publication-lane contract for every id it owns. Mirrors generation3d's
/// `retained_route_dispositions_are_exact_and_exhaustive`.
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    let routes: [(&str, &[&str], &[semio_framework_plugin::ArtifactToolPublicationContract]); 4] = [
        ("FlowDirectStoreJobFactory", FLOW_DIRECT_STORE_TOOL_IDS, FlowDirectStoreJobFactory::PUBLICATION_CONTRACTS),
        ("FlowChildGroupJobFactory", FLOW_CHILD_GROUP_TOOL_IDS, FlowChildGroupJobFactory::PUBLICATION_CONTRACTS),
        ("FlowHostEffectJobFactory", FLOW_HOST_ONLY_TOOL_IDS, FlowHostEffectJobFactory::PUBLICATION_CONTRACTS),
        ("FlowGraphOperationJobFactory", FLOW_GRAPH_OPERATION_TOOL_IDS, FlowGraphOperationJobFactory::PUBLICATION_CONTRACTS),
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
    eprintln!("[DEBUG] flow retained dispositions: {} tool ids partitioned over 4 factories", declared.len());
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
    eprintln!("[DEBUG] flow manifest declares {} migrated actions, 0 batch-only", migrated.len());
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
    assert_eq!(proofs.len(), FLOW_DIRECT_STORE_TOOL_IDS.len() + FLOW_HOST_ONLY_TOOL_IDS.len() + FLOW_CHILD_GROUP_TOOL_IDS.len() + FLOW_GRAPH_OPERATION_TOOL_IDS.len());
    eprintln!("[DEBUG] flow bounded-first-step proofs: {} rows, exact bijection", proofs.len());
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
    eprintln!("[DEBUG] flow graph-operation capacity: work_items={} rows(1)={}", FLOW_GRAPH_OPERATION_CAPACITY.work_items(), FLOW_GRAPH_OPERATION_CAPACITY.rows(1));
}

/// ⚖️ LAW: `FlowPlayApp` instantiates through the REAL manifest registry — the exact construction
/// that ran `validate_tool_job_rows` and faulted `interactive-job.catalog-authority` on the wgpu
/// playground, aborting the process before the first step. A booted app renders its main body, so
/// the app is live and not merely constructed.
#[semio_framework_async_macros::async_test]
async fn flow_play_app_boots_through_the_real_registry_without_a_catalog_authority_fault() {
    let mut app = flow_app_closing().await;
    assert!(!semio_framework_plugin::PluginApp::has_pending_typed_operations(&*app), "a freshly booted flow app owns no in-flight typed operation");
    assert!(matches!(semio_framework_plugin::PluginApp::maintenance_step(&mut *app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(_)), "a booted flow app runs its own maintenance turn");
    eprintln!("[DEBUG] FlowPlayApp booted through the real registry: no interactive-job.catalog-authority fault");
}

/// ⚖️ LAW: every graph-operation route is admitted by `FlowGraphOperationJobFactory` and returns a
/// retained admission receipt (`{operationId, generation}`) — never the batch `handle`, which fails
/// closed for all four retained families now, and never an `interactive-job.catalog-authority`
/// fault. The publication half of the ladder is deliberately not driven here: settling one of these
/// operations reaches the store's own `Recipe::advance`, which leaks an `OrderedMap` root on a
/// pre-existing path this lane does not own (`📓️flow-catalog-authority-2026-09-10.md` §7).
#[semio_framework_async_macros::async_test]
async fn every_graph_operation_route_is_admitted_by_its_own_retained_factory() {
    use crate::editor::flow::testkit::dispatch;
    let mut app = flow_app_closing().await;
    let widget_id = "graph-operation-probe".to_string();
    let commands = [
        FlowCommand::Reorganize(reorganize::Reorganize {}),
        FlowCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: widget_id.clone(), source_port_id: String::new(), target_node_id: widget_id.clone(), target_port_id: String::new() }),
        FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: widget_id.clone(), value: widget_id.clone() }),
        FlowCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations: Vec::new() }),
        FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: Vec::new() }),
        FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }),
    ];
    assert_eq!(commands.len(), FLOW_GRAPH_OPERATION_TOOL_IDS.len(), "one probe per graph-operation route");
    for command in commands {
        let tool_id = command.command_id();
        assert!(FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id));
        let result = dispatch(&mut app, command).await;
        assert!(!format!("{result:?}").contains("legacy-dispatch"), "{tool_id} must not fall back to the batch handle: {result:?}");
        assert!(!format!("{result:?}").contains("catalog-authority"), "{tool_id} must not fault the catalog: {result:?}");
        assert!(result.output.as_object().is_some_and(|fields| fields.iter().any(|(key, _)| key == "operationId")), "{tool_id} must return a retained admission receipt, not a batch emit: {result:?}");
        eprintln!("[DEBUG] graph-operation route {tool_id} admitted by its own retained factory: {result:?}");
    }
}

/// ⚖️ LAW: the batch `handle` fallback is unreachable for every declared route — every id is
/// retained-owned, so the legacy path fails closed rather than silently diverging from the job.
#[test]
fn the_batch_handle_fallback_is_closed_for_every_declared_route() {
    for tool_id in FlowCommand::TOOL_JOB_IDS {
        assert!(
            FLOW_DIRECT_STORE_TOOL_IDS.contains(tool_id) || FLOW_CHILD_GROUP_TOOL_IDS.contains(tool_id) || FLOW_HOST_ONLY_TOOL_IDS.contains(tool_id) || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(tool_id),
            "tool {tool_id} would reach the batch handle fallback"
        );
    }
    eprintln!("[DEBUG] flow batch handle fallback is unreachable for all {} declared routes", FlowCommand::TOOL_JOB_IDS.len());
}
