use super::*;
use crate::editor::architect::catalog::{analysis_kind_from_str, register_entities};
use crate::editor::architect::testkit;
use crate::registers::{AdjacencyKind, AnalysisKind};
use crate::standards::v1::subsets::any::schema::inferences::export_registers_csv;
use semio_framework_plugin::PluginApp;
use serde_json::json;

//#region 🔖️CommandSurface
/// 🎯️ One value per `app_commands!` row — the fixture behind the wire laws below.
fn every_command() -> Vec<ArchitectCommand> {
    vec![
        ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "risks".into() }),
        ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: None }),
        ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem { register_id: "elements".into(), entity_id: "e1".into() }),
        ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem { register_id: "elements".into(), entity_id: "e1".into(), patch_json: "{\"name\":\"X\"}".into() }),
        ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField { entity_id: "a1".into(), field: "kind".into(), value_json: "\"required\"".into() }),
        ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: "t1".into() }),
        ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {}),
        ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv: "a,b".into(), strategy: "upsert".into() }),
        ArchitectCommand::AddElement(add_element::AddElement { name: "Room".into() }),
        ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: "e1".into() }),
        ArchitectCommand::RunValidation(run_validation::RunValidation {}),
        ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: "gap".into() }),
        ArchitectCommand::RunReport(run_report::RunReport { report_kind: "executiveSummary".into() }),
        ArchitectCommand::ExportProgram(export_program::ExportProgram {}),
        ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {}),
        ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: "text".into() }),
        ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }),
        ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: None, cycle: true }),
        ArchitectCommand::Search(query::Search { query: "hall".into() }),
        ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(ArchitectCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 21, "every ArchitectCommand row must be covered by every_command()");
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        let keyword = printed.split_whitespace().next().unwrap_or_default().to_string();
        assert!(keyword.contains('-') || keyword == "search", "row {} printed a non-kebab keyword {printed:?}", command.command_id());
    }
}

/// 🧷️ Pins the exact pre-migration bytes for every row whose `Option`/`bool` fields make the
/// `None`/`Some` cases distinct on the wire — copied verbatim out of the ticket's
/// `🧪️wire-baseline-before.txt`, captured from the pre-migration hand-written `ArchitectCommand`.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &ArchitectCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex(&ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: None })), "01010204526f6f6d08656c656d656e747302000601010600");
    assert_eq!(hex(&ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: Some("t1".into()) })), "01010304526f6f6d08656c656d656e747302743103000601010600020602");
    assert_eq!(hex(&ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})), "01060000");
    assert_eq!(hex(&ArchitectCommand::RunValidation(run_validation::RunValidation {})), "010a0000");
    assert_eq!(hex(&ArchitectCommand::ExportProgram(export_program::ExportProgram {})), "010d0000");
    assert_eq!(hex(&ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})), "010e0000");
    assert_eq!(hex(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: None, cycle: true })), "01120201610162030006000106010302");
    assert_eq!(
        hex(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: Some("required".into()), cycle: false })),
        "01120301610162087265717569726564040006000106010206020301"
    );
    assert_eq!(hex(&ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None })), "01140000");
    assert_eq!(hex(&ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: Some("required".into()) })), "01140108726571756972656401000600");
}

/// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
/// `command_id`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<semio_framework_plugin::EditorApp<ArchitectPlayApp>>(testkit::architect_app_manifest_for_testkit).await;
    assert!(ArchitectPlayApp::command_from_action("notARealAction", None).is_err());
}

/// 🎯️ Spot-check a representative sample of action ids round-tripping into the expected typed
/// `ArchitectCommand` variant.
#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_declared_actions() {
    assert!(matches!(ArchitectPlayApp::command_from_action("runValidation", None), Ok(ArchitectCommand::RunValidation(_))));
    assert!(matches!(ArchitectPlayApp::command_from_action("search", Some(&dsl::json::to_dsl_value(&dsl::json!({ "query": "hall" })))), Ok(ArchitectCommand::Search(query::Search { query })) if query == "hall"));
    assert!(matches!(
        ArchitectPlayApp::command_from_action("selectRegister", Some(&dsl::json::to_dsl_value(&dsl::json!({ "registerId": "risks" })))),
        Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id })) if register_id == "risks"
    ));
}
//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_architect_app();
    assert_eq!(definition.modes.len(), 3);
    assert_eq!(definition.window_kinds.len(), 5);
    for body_key in [document_panel::ARCHITECT_BODY_DOCUMENT, catalogue_panel::ARCHITECT_BODY_CATALOGUE, inspection_panel::ARCHITECT_BODY_INSPECTION] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    for window in [adjacency_window::ARCHITECT_WINDOW_ADJACENCY, graph_window::ARCHITECT_WINDOW_GRAPH, register_window::ARCHITECT_WINDOW_REGISTER, report_window::ARCHITECT_WINDOW_REPORT, trace_window::ARCHITECT_WINDOW_TRACE] {
        assert!(definition.window_kinds.iter().any(|kind| kind.id == window), "window kind {window} is stitched into the manifest");
    }
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = testkit::new_app().await;
    assert!(testkit::render(&mut app, "architect.nope").await.contains("Unknown body"));
}
//#endregion 🔖️Manifest

//#region 🔖️Behavior
#[semio_framework_async_macros::async_test]
async fn adjacency_matrix_renders_triangle_strip() {
    let program = sample_plugin();
    let json = testkit::render_direct(adjacency_window::ARCHITECT_BODY_ADJACENCY, &program, &ArchitectPlayApp::initial_config());
    assert!(json.contains('▲'));
    assert!(json.contains("Reception"));
}

#[semio_framework_async_macros::async_test]
async fn graph_body_emits_node_graph_scene() {
    let program = sample_plugin();
    let history = semio_framework_plugin::HistoryView::empty();
    let cfg = ArchitectPlayApp::initial_config();
    let tree = ArchitectPlayApp::render(graph_window::ARCHITECT_BODY_GRAPH, &ArtifactView::new(&program, &history), &ConfigView { snapshot: &cfg, window: None }, &semio_framework_plugin::ViewModel::default()).expect("graph render");
    let semio_framework_plugin::Component::Surface(props) = &tree.root.component else { panic!("graph surface") };
    let scene: semio_framework_plugin::NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.nodes.len(), program.elements.len());
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("retire graph tree");
}

#[semio_framework_async_macros::async_test]
async fn set_adjacency_kind_cycles_required_to_preferred() {
    let program = sample_plugin();
    let adjacency = program.adjacencies.first().expect("adjacency");
    let emit = testkit::drive(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: adjacency.element_a_id.0.clone(), element_b_id: adjacency.element_b_id.0.clone(), kind: None, cycle: true }), &program);
    assert!(matches!(
        emit.artifact_mutations.first(),
        Some(ProgramMutation::ConnectAdjacency(payload)) if payload.adjacency.kind == AdjacencyKind::Preferred
    ));
}

#[semio_framework_async_macros::async_test]
async fn run_validation_populates_last_result_json() {
    let program = sample_plugin();
    let initial = ArchitectPlayApp::initial_config();
    let emit = testkit::drive_with_config(&ArchitectCommand::RunValidation(run_validation::RunValidation {}), &program, &initial);
    assert!(!testkit::config_after(&emit, &initial).last_result_json.is_empty());
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: search no longer auto-selects
/// its hits (selection is framework-owned `InteractionState` now, only ever mutated by the
/// framework's own injected `interactionSelect` handling, never by an app command's `Emit` —
/// mirrors `note`'s `add-block` precedent) — it still records the hits in `last_result_json` and
/// the query in `search_history_json`.
#[semio_framework_async_macros::async_test]
async fn search_finds_sample_elements() {
    let program = sample_plugin();
    let initial = ArchitectPlayApp::initial_config();
    let emit = testkit::drive_with_config(&ArchitectCommand::Search(query::Search { query: "Reception".into() }), &program, &initial);
    let config = testkit::config_after(&emit, &initial);
    assert!(!config.last_result_json.is_empty());
    assert!(!config.search_history_json.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn select_register_switches_active_register() {
    let program = sample_plugin();
    let initial = ArchitectPlayApp::initial_config();
    let emit = testkit::drive_with_config(&ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "stakeholders".into() }), &program, &initial);
    assert_eq!(testkit::config_after(&emit, &initial).active_register, "stakeholders");
    assert!(!register_entities(&program, "stakeholders").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn patch_register_item_updates_element_name() {
    let program = sample_plugin();
    let element_id = program.elements[0].header.id.clone();
    let emit = testkit::drive(&ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem { register_id: "elements".into(), entity_id: element_id.0, patch_json: json!({ "name": "Updated Reception" }).to_string() }), &program);
    assert!(matches!(
        emit.artifact_mutations.first(),
        Some(ProgramMutation::ReplaceProgramElement(payload)) if payload.program_element.header.name == "Updated Reception"
    ));
}

#[semio_framework_async_macros::async_test]
async fn formatted_report_renders_section_headings() {
    let program = sample_plugin();
    let initial = ArchitectPlayApp::initial_config();
    let emit = testkit::drive_with_config(&ArchitectCommand::RunReport(run_report::RunReport { report_kind: "executiveSummary".into() }), &program, &initial);
    let config = testkit::config_after(&emit, &initial);
    let json = testkit::render_direct(report_window::ARCHITECT_BODY_REPORT, &program, &config);
    assert!(json.contains("Overview"));
    assert!(json.contains("architect-report.section"));
}

#[semio_framework_async_macros::async_test]
async fn analysis_kind_picker_maps_all_variants() {
    let options = analysis_kind_picker_options();
    assert_eq!(options.len(), 20);
    for option in &options {
        let kind = analysis_kind_from_str(&option.value);
        assert!(!format!("{kind:?}").is_empty(), "missing mapping for {}", option.value);
    }
    assert_eq!(analysis_kind_from_str("relationshipAnalysis"), AnalysisKind::RelationshipAnalysis);
}

#[semio_framework_async_macros::async_test]
async fn import_registers_csv_action_sets_plugin() {
    let program = sample_plugin();
    let csv = export_registers_csv(&program).expect("export csv");
    let emit = testkit::drive(&ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv, strategy: "upsert".into() }), &program);
    assert!(emit.artifact_mutations.is_empty(), "whole-document load must not go through the Mutation enum");
    assert!(matches!(emit.effects.first(), Some(semio_framework_plugin::Effect::LoadDocument { .. })), "importRegistersCsv must emit a LoadDocument effect");
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = testkit::new_app().await;
    let before = app.snapshot().expect("projection").elements.len();
    testkit::dispatch(&mut app, ArchitectCommand::AddElement(add_element::AddElement { name: "Ward".into() })).await;
    assert_eq!(app.snapshot().expect("projection").elements.len(), before + 1);
    app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("projection").elements.len(), before);
    app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("projection").elements.len(), before + 1);
}

/// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
/// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
/// `testkit::app_with_registry` exists.
///
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `setSelection` (the old
/// app-declared View action this test used to dispatch) is deleted — selection is the
/// framework's own injected `interactionSelect` verb now, never an app command. `selectRegister`
/// is the remaining view action closest in shape (config-only, no document mutation).
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = testkit::app_with_registry().await;
    let result = testkit::dispatch(&mut app, ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "risks".into() })).await;
    assert!(result.mutations.is_empty(), "selectRegister is a view action and must never reach document operations under kind discipline");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: end-to-end proof the "program"
/// domain's real pick surface (the document panel's element rows) actually drives the framework's
/// injected `interactionSelect` — dispatches it directly (the only way a downstream crate can
/// populate a genuine `InteractionView`, see `testkit::drive`'s own doc comment), then confirms
/// the SAME element row renders `"selected":true` (mirrors `note`'s `select_blocks` proof).
#[semio_framework_async_macros::async_test]
async fn interaction_select_stamps_the_picked_element_as_selected_in_the_document_panel() {
    let mut app = testkit::app_with_registry().await;
    let element_id = app.snapshot().expect("snapshot").elements[0].header.id.to_string();
    let targets = serde_json::to_string(&[serde_json::json!({ "granularity": ARCHITECT_INTERACTION_GRANULARITY_ENTITY, "id": element_id })]).expect("targets json");
    app.handle_action("interactionSelect", Some(&dsl::json::to_dsl_value(&dsl::json!({ "domainId": ARCHITECT_INTERACTION_PROGRAM, "targets": targets, "merge": "replace" }))), &semio_framework_plugin::testkit::meta("test"))
        .await
        .expect("interactionSelect");
    let rendered = testkit::render(&mut app, document_panel::ARCHITECT_BODY_DOCUMENT).await;
    assert!(rendered.contains(&element_id), "the rendered tree must still list the picked element");
    assert!(rendered.contains("\"selected\":true"), "the picked element must be stamped selected by the framework wrapper");
}
//#endregion 🔖️Behavior
