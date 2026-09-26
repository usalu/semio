pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    /// ✏️ Adapts `create_din4108_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry` still expects (framework test context gap,
    /// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
    pub fn din4108_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_din4108_app(), examples: Vec::new() }
    }
    
    pub type NormApp = VcsArtifactApp<EditorApp<Din4108PlayApp>>;
    
    /// ð§¬ï¸ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub async fn app_with_registry() -> NormApp {
        let mut app = new_app_with_registry::<EditorApp<Din4108PlayApp>>(din4108_manifest_for_tests).await;
        semio_framework::io::resolve_ready(app.bind_instance_id(meta("local").instance_id));
        app
    }

    /// 🧹️ Closes every store the wrapper opened. A live `ArtifactStore` asserts in `Drop` unless it was
    /// driven to its terminal-empty shallow shell, so every fixture that dispatches must end here.
    pub fn close(app: &mut NormApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(app);
    }
    
    pub async fn dispatch(app: &mut NormApp, command: Din4108Command) -> InvocationResult {
        let mut action_meta = meta("local");
        if command.command_id() == "setSelectedCheckIndex" {
            action_meta.view_state = Some(ViewModel {
                window_id: Some(results::WINDOW_RESULTS.into()),
                focused_window_id: Some(results::WINDOW_RESULTS.into()),
                window_instances: vec![ViewWindowInstance { id: results::WINDOW_RESULTS.into(), window_kind_id: results::WINDOW_RESULTS.into() }],
                ..Default::default()
            });
        }
        let result = app.dispatch_typed(command, &action_meta).await.expect("dispatch");
        settle(app).await;
        result
    }
    
    /// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does, draining
    /// EVERY result page: a bare `maintenance_step` loop retires nothing and turns a publication fault
    /// into a silent timeout, which is exactly how ticket 26/09/18 slice B2b lost the real cause.
    pub async fn settle(app: &mut NormApp) {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id).await.expect("settle the typed operation");
    }

    pub async fn render(app: &mut NormApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render projection")
    }
}

use super::*;
use semio_framework_plugin::PluginApp;

#[test]
fn retained_command_dispositions_match_the_language_neutral_oracle() {
    crate::app_surface::retained_disposition_oracle::assert_fixture(VARIANT);
    let definition = create_din4108_app();
    let mut classified = 0usize;
    for window in definition.window_kinds.iter() {
        // 🕹️ `window.actions` holds only what a window kind claims FOR ITSELF. Since the app-wide
        // roster stopped being cloned into every window kind, the dispatchable set of a window is
        // `semio_framework::window_kind_actions` — the same predicate the framework's own
        // command-bridge law uses. Norm declares all three retained tools at app level, so reading
        // `window.actions` here saw an empty roster.
        let dispatchable = semio_framework::window_kind_actions(&definition, window);
        for id in crate::app_surface::NORM_RETAINED_TOOL_IDS {
            let action = dispatchable.iter().find(|action| action.id == *id).unwrap_or_else(|| panic!("window {} must dispatch {id}", window.id));
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
        Din4108Command::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }),
        Din4108Command::SetField(set_field::SetField { path: "airtightnessN50".into(), value_json: "1.5".into() }),
        Din4108Command::InsertItem(insert_item::InsertItem { path: "zones".into(), index: 0, value_json: None }),
        Din4108Command::RemoveItem(remove_item::RemoveItem { path: "zones".into(), index: 0 }),
        Din4108Command::ApplyRemedy(apply_remedy::ApplyRemedy { check_id: String::new(), remedy_index: 0 }),
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
    assert_eq!(ids, vec!["setSnapshot", "evaluate", "setSelectedCheckIndex", "setActiveExample", "setField", "insertItem", "removeItem", "applyRemedy"]);
}

/// ð§·ï¸ The permanent wire guard: every row round-trips textâbinary and prints under its own declared
/// kebab wire keyword (which is deliberately NOT the camelCase `command_id`).
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    let keywords = ["set-snapshot", "evaluate", "selected-check", "set-active-example", "set-field", "insert-item", "remove-item", "apply-remedy"];
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
    for body_key in [document_panel::BODY_ARTIFACT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
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
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, "norm.din4108.play.nope").await.contains("Unknown body"));
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_body_key_renders() {
    let mut app = context::app_with_registry().await;
    for body_key in [inputs::BODY_INPUTS, results::BODY_RESULTS, document_panel::BODY_ARTIFACT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
        assert!(!context::render(&mut app, body_key).await.contains("Unknown body"), "{body_key} must render its own node");
    }
    context::close(&mut app);
}
//#endregion ðï¸Manifest

//#region ðï¸Behavior
#[semio_framework_async_macros::async_test]
async fn set_snapshot_commits_a_host_backed_report() {
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() })).await;
    context::settle(&mut app).await;
    let mut host = NormHost::<Din4108Family>::from_artifact(app.snapshot().expect("projection"));
    host.evaluate();
    assert!(!host.report().checks.is_empty());
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn host_updates_report_after_document_replace() {
    let mut host = Host::default();
    host.evaluate();
    assert!(host.report().complies());
    let mut document = Din4108Snapshot::default();
    document.elements.clear(); document.zones.clear();
    host.replace_document(document);
    host.evaluate();
    assert!(!host.report().complies());
}

#[semio_framework_async_macros::async_test]
async fn evaluate_recommits_the_current_projection_without_changing_it() {
    let mut app = context::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    context::dispatch(&mut app, Din4108Command::Evaluate(evaluate::Evaluate {})).await;
    assert_eq!(before, app.snapshot().expect("projection"));
    context::close(&mut app);
}

/// 🧵️ The migrated route end to end: `dispatch_typed` passes the UI-dispatch classification gate,
/// `build_norm_tool_job` hands the command to the shared owned factory, and repeated
/// `maintenance_step` turns stage every `from_snapshot` field mutation into ONE batched Artifact
/// edit through the exact per-item preparation authority — proving both the wiring and that the
/// authored bundle order survives the staging. `app_with_registry` + a bound instance id is mandatory:
/// a registry-less wrapper fails closed with `interactive-job.catalog-authority` once proofs exist.
#[semio_framework_async_macros::async_test]
async fn set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document() {
    let mut app = context::app_with_registry().await;
    let mut target = Din4108Snapshot::default();
    target.elements.clear(); target.zones.clear();
    assert_ne!(target, app.snapshot().expect("projection"));
    context::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: target.clone() })).await;
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), target, "setSnapshot did not publish the payload document");
    context::close(&mut app);
}

/// 🔬️ The one link inside the artifact-lane preparation that `setSnapshot` cannot be debugged
/// through the store for: `prepare_norm_one_item` takes the mutation, asks it for a diff and applies
/// that diff to the base. If any of the three steps refuses, the preparation has already consumed
/// the mutation and the next `advance` reports the *downstream* `norm-mutation-owner-missing`
/// instead of the real cause, which is exactly what ticket 26/09/18 slice B2b saw in the shell.
#[test]
fn from_snapshot_yields_one_diff_appliable_mutation_for_a_one_field_change() {
    let base = Din4108Snapshot::default();
    let mut target = base.clone();
    target.t_int_c = 22.5;
    let mutations = Din4108Mutation::from_snapshot(&base, &target);
    assert_eq!(mutations.len(), 1, "one changed field must decompose into exactly one mutation: {mutations:?}");
    let diff = ::protocol::Mutation::diff(&mutations[0], &base).into_parts().0;
    let post = ::protocol::MutationDiff::apply(&diff, &base).expect("the from_snapshot mutation's own diff must apply to the base it was derived from");
    assert_eq!(post, target);
}

/// 🌡️ The narrowest `setSnapshot` there is: one field moved, so `Din4108Mutation::from_snapshot`
/// yields exactly ONE mutation and the artifact lane's one-item preparation authority
/// (`NormOneItemPreparationFactory`) has exactly one item to stage. Ticket 26/09/18 slice B2b
/// measured the shell dispatching this verb and the operation failing
/// `norm-mutation-owner-missing`; this test is the native witness of the same path, and it is the
/// sibling of `set_snapshot_dispatches_…`, whose target differs in two fields.
#[semio_framework_async_macros::async_test]
async fn set_snapshot_publishes_a_one_field_change_through_the_tool_job_path() {
    let mut app = context::app_with_registry().await;
    let mut target = app.snapshot().expect("projection");
    target.t_int_c = 22.5;
    assert_ne!(target, app.snapshot().expect("projection"));
    context::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: target.clone() })).await;
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), target, "a one-field setSnapshot did not publish");
    context::close(&mut app);
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

/// ð§®ï¸ `setSelectedCheckIndex` is Results-window-config-only â it must dispatch cleanly and never touch the document.
#[semio_framework_async_macros::async_test]
async fn selected_check_index_is_a_config_only_edit() {
    let mut app = context::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    let result = context::dispatch(&mut app, Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })).await;
    assert!(result.mutations.is_empty(), "a Results-window-config-only command must emit no document operations");
    assert_eq!(before, app.snapshot().expect("projection"), "a Results-window-config-only command must never mutate the document");
    context::close(&mut app);
}

/// ð§¬ï¸ Kind-discipline wrapper: the real registry enforces that View actions never emit document
/// operations.
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = context::app_with_registry().await;
    let result = context::dispatch(&mut app, Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) })).await;
    assert!(result.mutations.is_empty());
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() })).await;
    app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo");
    context::settle(&mut app).await;
    app.handle_action("redo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("redo");
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), Din4108Snapshot::default());
    context::close(&mut app);
}

/// ðï¸ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
#[semio_framework_async_macros::async_test]
async fn report_out_exports_the_computed_check_report() {
    let mut app = context::app_with_registry().await;
    let mut host = NormHost::<Din4108Family>::from_artifact(app.snapshot().expect("projection"));
    host.evaluate();
    let media = semio_framework_plugin::resolve_ready(PluginApp::export_media(&mut app, "report:out")).expect("export report:out");
    let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
    assert_eq!(schema, crate::app_surface::artifact_kind_id(VARIANT));
    let value: serde_json::Value = serde_json::from_str(&json).expect("report json parses");
    let checks = value.get("checks").and_then(|c| c.as_array()).expect("checks array");
    assert!(!checks.is_empty(), "exported report must contain computed checks");
    context::close(&mut app);
}
//#endregion ðï¸Behavior
