use super::*;
use semio_framework_plugin::PluginApp;

#[test]
fn retained_command_dispositions_match_the_language_neutral_oracle() {
    crate::app_surface::retained_disposition_oracle::assert_fixture(VARIANT);
    let definition = create_din4108_app();
    let mut classified = 0usize;
    for window in definition.window_kinds.iter() {
        for id in crate::app_surface::NORM_RETAINED_TOOL_IDS {
            let action = window.actions.iter().find(|action| action.id == *id).unwrap_or_else(|| panic!("window {} must declare {id}", window.id));
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
            classified += 1;
        }
    }
    assert_eq!(classified, definition.window_kinds.len() * crate::app_surface::NORM_RETAINED_TOOL_IDS.len());
}

//#region ðï¸CommandSurface
/// ð¯ï¸ One value per `Din4108Command` row â the whole-command-surface laws below iterate it, so a new row
/// that is not listed here fails `command_ids_cover_every_row`.
fn every_command() -> Vec<Din4108Command> {
    vec![
        Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() }),
        Din4108Command::Evaluate(evaluate::Evaluate {}),
        Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_cover_every_row_and_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Din4108Command::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids, vec!["setSnapshot", "evaluate", "setSelectedCheckIndex"]);
}

/// ð§·ï¸ The permanent wire guard: every row round-trips textâbinary and prints under its own declared
/// kebab wire keyword (which is deliberately NOT the camelCase `command_id`).
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    let keywords = ["set-snapshot", "evaluate", "selected-check"];
    for (command, keyword) in every_command().into_iter().zip(keywords) {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        assert!(printed.starts_with(keyword), "row {} printed {printed:?}, expected keyword {keyword}", command.command_id());
    }
}

/// ð§·ï¸ Pins the exact pre-migration bytes for the rows whose shape the `app_commands!` decomposition
/// could have silently rewritten â the fieldless `Evaluate` (was a unit variant) and both `Option`
/// cases of `SetSelectedCheckIndex`. Hex copied verbatim from the ticket's
/// `ð§ªï¸wire-baseline-before.txt`; these bytes are identical for all fifteen norm apps because none
/// of the three payload shapes involves the per-standard `Din4108Snapshot`.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &Din4108Command| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex(&Din4108Command::Evaluate(evaluate::Evaluate {})), "01010000");
    assert_eq!(hex(&Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01020001000402");
    assert_eq!(hex(&Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01020000");
}
//#endregion ðï¸CommandSurface

//#region ðï¸Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_din4108_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 2);
    for body_key in [document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == crate::app_surface::artifact_kind_id(VARIANT)));
}

/// ðï¸ Port recipe: every norm app declares `model:in`/`report:out` alongside the implicit document
/// ports, and `report:out` is pinned to this family's already-declared artifact kind.
#[semio_framework_async_macros::async_test]
async fn declares_model_in_and_report_out_ports() {
    let ports = create_din4108_app().io.ports;
    assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
    let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
    assert_eq!(report_out.kind_id.as_deref(), Some(crate::app_surface::artifact_kind_id(VARIANT).as_str()));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = testkit::app_with_registry().await;
    assert!(testkit::render(&mut app, "norm.din4108.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn every_declared_body_key_renders() {
    let mut app = testkit::app_with_registry().await;
    for body_key in [inputs::BODY_INPUTS, results::BODY_RESULTS, document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
        assert!(!testkit::render(&mut app, body_key).await.contains("Unknown body"), "{body_key} must render its own node");
    }
}
//#endregion ðï¸Manifest

//#region ðï¸Behavior
#[semio_framework_async_macros::async_test]
async fn set_snapshot_commits_a_host_backed_report() {
    let mut app = testkit::app_with_registry().await;
    testkit::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() })).await;
    let host = NormHost::<Din4108Family>::from_document(app.snapshot().expect("projection"));
    assert!(!host.report().checks.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn host_updates_report_after_document_replace() {
    let mut host = Host::default();
    assert!(host.report().all_pass());
    let mut document = Din4108Snapshot::default();
    document.layers.clear();
    host.replace_document(document);
    assert!(!host.report().all_pass());
}

#[semio_framework_async_macros::async_test]
async fn evaluate_recommits_the_current_projection_without_changing_it() {
    let mut app = testkit::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    testkit::dispatch(&mut app, Din4108Command::Evaluate(evaluate::Evaluate {})).await;
    assert_eq!(before, app.snapshot().expect("projection"));
}

/// 🧵️ The migrated route end to end: `dispatch_typed` passes the UI-dispatch classification gate,
/// `build_norm_tool_job` hands the command to the shared owned factory, and repeated
/// `maintenance_step` turns stage every `from_snapshot` field mutation into ONE batched Artifact
/// edit through the exact per-item preparation authority — proving both the wiring and that the
/// authored bundle order survives the staging. `app_with_registry` + a bound instance id is mandatory:
/// a registry-less wrapper fails closed with `interactive-job.catalog-authority` once proofs exist.
#[semio_framework_async_macros::async_test]
async fn set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document() {
    let mut app = testkit::app_with_registry().await;
    semio_framework::io::resolve_ready(app.bind_instance_id(1));
    let mut target = Din4108Snapshot::default();
    target.layers.clear();
    assert_ne!(target, app.snapshot().expect("projection"));
    testkit::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: target.clone() })).await;
    let mut ticks = 0usize;
    while ticks < 5_000 && app.snapshot().expect("projection") != target {
        app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert_eq!(app.snapshot().expect("projection"), target, "setSnapshot did not publish the payload document after {ticks} maintenance turns");
}

/// 🧵️ Three-way drift guard between the retained id list, the publication contracts and the proof
/// catalog every norm editor forwards.
#[test]
fn the_proof_catalog_covers_exactly_the_shared_retained_tool_ids() {
    use semio_framework_plugin::ArtifactEditor;
    assert_eq!(crate::app_surface::NORM_RETAINED_TOOL_IDS.len(), crate::app_surface::NORM_PUBLICATION_CONTRACTS.len());
    assert_eq!(Din4108PlayApp::bounded_first_step_tool_proofs().len(), crate::app_surface::NORM_RETAINED_TOOL_IDS.len());
    assert_eq!(crate::app_surface::NORM_PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), crate::app_surface::NORM_RETAINED_TOOL_IDS.to_vec());
}

/// ð§®ï¸ `setSelectedCheckIndex` is config-only â it must dispatch cleanly and never touch the document.
#[semio_framework_async_macros::async_test]
async fn selected_check_index_is_a_config_only_edit() {
    let mut app = testkit::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    let result = testkit::dispatch(&mut app, Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })).await;
    assert!(result.mutations.is_empty(), "a config-only command must emit no document operations");
    assert_eq!(before, app.snapshot().expect("projection"), "a config-only command must never mutate the document");
}

/// ð§¬ï¸ Kind-discipline wrapper: the real registry enforces that View actions never emit document
/// operations.
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = testkit::app_with_registry().await;
    let result = testkit::dispatch(&mut app, Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) })).await;
    assert!(result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = testkit::app_with_registry().await;
    testkit::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() })).await;
    app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("undo");
    app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("projection"), Din4108Snapshot::default());
}

/// ðï¸ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
#[semio_framework_async_macros::async_test]
async fn report_out_exports_the_computed_check_report() {
    let mut app = testkit::app_with_registry().await;
    let media = semio_framework_plugin::resolve_ready(PluginApp::export_media(&mut app, "report:out")).expect("export report:out");
    let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
    assert_eq!(schema, crate::app_surface::artifact_kind_id(VARIANT));
    let report: crate::document::CheckReport = serde_json::from_str(&json).expect("report json parses");
    assert!(!report.checks.is_empty());
}
//#endregion ðï¸Behavior
