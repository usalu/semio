//! ⏪️ Part 1 of 2 of the history-edit session laws of the plugin runtime (cut from `🧪️time-travel`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_plugin_decl_emission::HostEvent;
#[cfg(test)]
use semio_framework_ui_locale::Locale;
use semio_framework_plugin_time_travel::TIME_TRAVEL_TURN_WALL_US;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
#[cfg(test)]
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_tests_06::time_travel_support_1::{TOY_HOST_EVENTS, ToyApp, ToyHistoryApp, apply_long_edit, close, collect_original_history_command_receipt, commit_scenario, dsl, edit_authors, find_node, fixture, head, meta, original_time_travel_fixture_caller, publish_as, pump_until, rejected, render_body, render_history, row_actions, run_step, seed_op, seeded_app, seeded_app_as, seeded_mutation, text, toy_manifest, verb};
use semio_framework_plugin_time_travel::TimeTravelActionRefusal;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::{artifact_app_laws, with_authoring_identity};
use semio_framework_plugin_instance_app as artifact_instance;
use semio_framework_plugin_actor_host as host;
use semio_framework_plugin_extension::describe;
use semio_framework_plugin_command::lifted_command::history_mutation_entry;
use semio_framework_os_kernel::io;
use semio_framework::*;
use semio_framework::kernel::{ActorId, HybridLogicalTimestamp, InvocationResult, MutationId, UiDirtyScope};
use store::os_store::store_schema::{ArtifactCommand, HistoryLane};
use semio_framework_artifact_reference::ArtifactDialect;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, MemberFactory, Mutation, SpaceMember};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::DslValue;
use semio_framework::{FaultCode, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use std::future::Future;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use protocol::LOCAL_ACTOR_ID;
#[cfg(test)]
use semio_framework_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
#[cfg(test)]
use semio_framework_plugin_decl_emission::TransactionPhase;
use store::os_store::document_materialize::create_document_envelope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_plugin_time_travel as time_travel;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_time_travel::TimeTravelStage;
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};


/// ⚖️ LAW: every fixture scenario ends in its status and renders its body; accepted verbs never error, the committed
/// document only changes through a commit, and pending/superseded/withdrawn rows are exactly the fixture's.
#[semio_framework_async_macros::async_test]
async fn every_fixture_scenario_reaches_its_status_body_and_rows() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    for scenario in fixture["scenarios"].as_array().expect("scenarios") {
        let id = text(&scenario["id"]);
        let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
        let generation = app.stores.store.generation();
        let mut commits = false;
        for step in scenario["steps"].as_array().expect("steps") {
            commits |= step.get("commit").is_some();
            if let Some(result) = run_step(&mut app, &fixture, step, mounted_policy, &mut identity,cx).await {
                assert_eq!(rejected(&result), None, "{id}: step {step} was refused: {:?}", result.output);
            }
        }
        let status = app.time_travel.status();
        match scenario["status"].as_object() {
            None => assert!(status.is_none(), "{id}: the session closed"),
            Some(expected) => {
                let status = status.unwrap_or_else(|| panic!("{id}: a session is open"));
                assert_eq!(serde_json::to_value(status.stage).expect("stage"), expected["stage"], "{id}: stage");
                assert_eq!(status.blocking, expected["blocking"].as_bool().expect("blocking"), "{id}: blocking");
                assert_eq!(u64::from(status.accepted_count), expected["acceptedCount"].as_u64().expect("accepted"), "{id}: accepted drafts");
                let problem = expected.get("nextProblem").and_then(Value::as_u64).map(|index| seeded_mutation(&app, index as usize));
                assert_eq!(status.next_problem.as_ref().map(|problem| (problem.mutation_id.clone(), problem.store.clone())), problem.map(|mutation| (mutation, None)), "{id}: the next problem");
                assert_eq!(status.next_problem.is_some(), status.blocking, "{id}: a review names its next problem exactly while it blocks");
            }
        }
        if let Some(refusal) = scenario.get("finalizeRefusal") {
            assert_eq!(app.time_travel.session().finalize_refusal().map(|refusal| refusal.code()), refusal.as_str(), "{id}: finalize refusal");
        }
        assert_eq!(render_body(&mut app).await.contains(text(&scenario["body"])), true, "{id}: body {}", render_body(&mut app).await);
        if !commits {
            assert_eq!(app.stores.store.generation(), generation, "{id}: no commit, no store change");
        }
        let patch = app.render_ctx().history_patch(true).await.expect("history patch");
        let rows: Vec<&semio_framework::kernel::HistoryMutationEntry> = patch.upserts.iter().rev().flat_map(|entry| entry.mutations.iter()).collect();
        let mutations: Vec<String> = (0..fixture["seed"].as_array().expect("seed").len()).map(|index| seeded_mutation(&app, index)).collect();
        for (key, flag) in [("pending", 0usize), ("superseded", 1), ("withdrawn", 2)] {
            let Some(expected) = scenario.get(key).and_then(Value::as_array) else { continue };
            let expected: Vec<&str> = expected.iter().map(|index| mutations[index.as_u64().expect("index") as usize].as_str()).collect();
            let actual: Vec<&str> = rows.iter().filter(|row| [row.pending, row.superseded, row.withdrawn][flag]).map(|row| row.mutation_id.as_str()).collect();
            assert_eq!(actual, expected, "{id}: {key} rows");
        }
        if let Some(name) = scenario["alternative"].as_str() {
            let envelope = app.stores.store.envelope();
            let alternative = envelope.vcs.alternatives.iter().find(|alternative| alternative.name == name).unwrap_or_else(|| panic!("{id}: alternative {name} exists"));
            assert_eq!(envelope.active_alternative_id.as_deref(), Some(alternative.id.as_str()), "{id}: the new alternative is active");
            assert!(app.stores.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(alternative.id.as_str())), "{id}: the supersession is scoped to the new alternative");
        }
        if scenario["superseded"].as_array().is_some_and(|superseded| !superseded.is_empty()) && scenario["alternative"].is_null() {
            assert!(app.stores.store.supersessions().values().all(|supersession| supersession.scope.is_none()), "{id}: overwrite is unscoped");
        }
        pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
        close(&mut app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: while a history edit is open, artifact emits, child emits and history-lane verbs are refused with
/// `timeTravel.frozen`; view verbs keep working; a stale generation and an invalid input value are silent refusals
/// that leave the draft untouched.
#[semio_framework_async_macros::async_test]
async fn an_open_session_freezes_document_verbs_and_refuses_stale_or_invalid_drafts() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let refusals = &fixture["refusals"];
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let emitted = app.dispatch_emit("setCount", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { artifact_mutations: vec![SetCount { value: 3 }.into()], ..Default::default() }, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    assert_eq!(emitted.err().map(|fault| fault.code), Some(FaultCode::new(text(&refusals["frozen"]))), "an artifact emit is frozen");
    for action in refusals["frozenVerbs"].as_array().expect("frozen verbs") {
        let refused = app.handle_action(text(action), None, &meta(&fixture), &mut identity).await;
        assert_eq!(refused.err().map(|fault| fault.code), Some(FaultCode::new(text(&refusals["frozen"]))), "{action} is frozen");
    }
    let filtered = app.handle_action(SET_HISTORY_COMMAND_FILTER_ACTION_ID, Some(&DslValue::object([("value".to_string(), DslValue::String("all".into()))])), &meta(&fixture), &mut identity).await;
    assert!(filtered.is_ok(), "a view verb keeps working: {filtered:?}");
    let generation = app.time_travel.session().generation;
    let stale = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(9)), ("generation".into(), DslValue::uint(u64::from(generation) + 1))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&stale), Some(text(&refusals["stale"])));
    let invalid = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), dsl(&refusals["invalidInputValue"]))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&invalid), Some(text(&refusals["invalidInput"])));
    assert!(app.time_travel.editor().is_some_and(|editor| editor.refused.is_some()), "the refused value names its reason on the editor");
    assert_eq!(app.time_travel.session().generation, generation, "refusals leave the session untouched");
    let history = render_history(&mut app, Locale::De).await;
    assert!(find_node(&history, "framework.history.timeTravel").is_some(), "the band leads the history body");
    assert!(find_node(&history, "framework.history.editor").is_some(), "the draft editor is shown while editing");
    assert!(find_node(&history, "framework.history.editor.input.value").is_some(), "the input's control is derived from its descriptor");
    assert!(history.to_string().contains("Verlaufsbearbeitung"), "German copy in a German shell");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    assert!(app.handle_action("undo", None, &meta(&fixture), &mut identity).await.is_ok(), "exit lifts the freeze");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: cancelling a running replay keeps the drafts and reviews without a report; a remote edit arriving while
/// reviewing moves the base and replays again, and the reviewed head includes the remote edit.
#[semio_framework_async_macros::async_test]
async fn cancel_keeps_drafts_and_a_remote_edit_replays_again() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let (backbone, mut probe) = MemoryBackbone::pair("time-travel-remote", "time-travel-remote").await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("attach probe backbone");
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&cancelled), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    assert!(app.time_travel.session().report.is_none() && app.time_travel.session().accepted.len() == 1, "the draft stays, no report");
    let status = app.time_travel.status().expect("a reviewing session");
    assert_eq!(
        (status.review, status.rerunnable, status.fault.as_deref()),
        (Some(semio_framework::kernel::HistoryTimeTravelReview::NeedsReplay), true, Some(semio_framework_time_travel::TIME_TRAVEL_CANCELLED_CODE)),
        "a cancelled replay needs a rerun, never reads as no changes"
    );
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.timeTravel.rerun").is_some_and(|rerun| rerun["disabled"] != Value::Bool(true)), "the band offers Replay again");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&rerun), None);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying, "rerun replays the kept drafts");
    run_step(&mut app, &fixture, &serde_json::json!({ "replay": "clean" }), mounted_policy, &mut identity,cx).await;
    assert_eq!(app.time_travel.status().and_then(|status| status.review), Some(semio_framework::kernel::HistoryTimeTravelReview::Ready));
    let mut remote = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("remote".into()), mounted_policy, &mut identity).await;
    let (remote_backbone, mut remote_probe) = MemoryBackbone::pair("time-travel-remote-peer", "time-travel-remote-peer").await;
    remote.attach_backbone(store::os_store::backbone::Backbones::Memory(remote_backbone)).await.expect("attach remote probe");
    assert_eq!(remote.stores.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }, &mut identity).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.stores.store.generation();
    app.tick_backbone(&mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("ingest remote edit");
    assert!(app.stores.store.generation() > generation, "the remote edit is ingested while the session reviews");
    pump_until(&mut app, "the moved base replays again", |app| app.time_travel.session().stage == TimeTravelStage::Reviewing && app.time_travel.session().report.is_some() && app.time_travel.session().base.content_revision == app.stores.store.content_revision(), mounted_policy, &mut identity,cx)
        .await;
    assert!(render_body(&mut app).await.contains("label=b"), "the replayed head keeps the draft");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    drop((probe, remote_probe));
    close(&mut app,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🪟️ The history body as a host shows it with history row `seq`'s mutation window open on `rows` rows from `offset`.
async fn render_history_window(app: &mut ToyApp, seq: u64, offset: u32, rows: u32) -> Value {
    let request = semio_framework::TreeWindowRequest { body_key: FRAMEWORK_HISTORY_BODY_KEY.into(), node_key: time_travel::history_row_window_path(seq), open: Some(true), offset, rows };
    let view = ViewModel { tree_windows: vec![request], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let tree = app.render(FRAMEWORK_HISTORY_BODY_KEY, None, &view).await.unwrap_or_else(|fault| panic!("history render: {fault:?}"));
    serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).expect("history projection")).expect("history projection parses")
}

/// 🧒️ The mutation ids of the mutation rows a rendered history row materialised, in row order.
fn mutation_row_ids(row: &Value) -> Vec<String> {
    row["children"].as_array().into_iter().flatten().filter_map(|child| child["key"].as_str()?.strip_prefix("framework.history.mutation.").map(str::to_string)).collect()
}

/// ⚖️ LAW (gaps N1, N15): every mutation of a long transaction is reachable from its history row — the row is a tree window
/// over all of them (the flagged one first, then the projected ones, then the rest of its edit read on demand) while the
/// wire stays bounded; a row holding a warning opens by default with the warning first, a row without one stays closed,
/// and a window opened past the projection shows the 33rd…40th mutations with their Edit action, which opens the editor on
/// them. The Edit action is disabled, naming why and activating nothing, exactly where `Begin` is refused: a changed draft
/// still open, a running replay; it is enabled again in the review.
#[semio_framework_async_macros::async_test]
async fn every_mutation_of_a_long_transaction_is_reachable_and_edit_follows_the_begin_law() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let mut ops: Vec<TestMutation> = (1..40).map(|value| SetCount { value }.into()).collect();
    ops.push(SetLabel { value: head(&app).1 }.into());
    let transaction = protocol::TransactionRef { id: "tx-0000000000000040".into(), tool: format!("{}#select", ToyHistoryApp::APP_ID) };
    app.dispatch_emit("select", Emit::<TestMutation, TestConfigMutation, NoDraftMutation>::commit_transaction(transaction.clone(), ops), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the long transaction publishes");
    let row = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().find(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == transaction.id)).expect("the transaction row");
    let ids: Vec<String> = app.stores.store.mutation_ops().expect("applied operations").iter().filter(|op| row.edit_id.as_deref() == Some(op.edit_id)).map(|op| op.mutation_id.0.clone()).collect();
    assert_eq!((row.op_count, ids.len()), (40, 40), "one edit of forty operations");
    assert!(
        row.mutations.len() < 40 && row.mutations.iter().any(|mutation| mutation.mutation_id == ids[39] && mutation.worst == Some(semio_framework_diagnostic::Severity::Warning)),
        "the wire stays bounded and keeps the flagged no-op: {}",
        row.mutations.len()
    );
    let key = format!("framework.history.entry.{}", row.seq);

    let unrequested = render_history(&mut app, Locale::En).await;
    let opened = find_node(&unrequested, &key).unwrap_or_else(|| panic!("the transaction row: {unrequested}"));
    assert_eq!((opened["component"]["window"]["total"].as_u64(), mutation_row_ids(opened).first()), (Some(40), Some(&ids[39])), "a row holding a warning opens by default, the warning first, announcing all forty: {opened}");
    let seed_row = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().find(|entry| entry.edit_id.is_some() && entry.transaction.is_none() && !entry.mutations.is_empty()).expect("a seed row");
    assert!(mutation_row_ids(find_node(&unrequested, &format!("framework.history.entry.{}", seed_row.seq)).expect("the seed row")).is_empty(), "a row without an outcome stays closed");
    let first = render_history_window(&mut app, row.seq, 0, 8).await;
    assert_eq!(mutation_row_ids(find_node(&first, &key).expect("the open row")), [&ids[39..], &ids[..7]].concat(), "the flagged mutation leads, then op order");
    let last = render_history_window(&mut app, row.seq, 32, 8).await;
    let tail = find_node(&last, &key).expect("the row opened past its projection");
    assert_eq!(mutation_row_ids(tail), ids[31..39].to_vec(), "the 33rd to 40th rows: the last projected one, then the rest of the edit");
    let edited = find_node(tail, &format!("framework.history.mutation.{}", ids[38])).expect("the 39th mutation row");
    assert_eq!(
        (edited["component"]["rowActions"][0]["verb"].as_str(), edited["component"]["target"]["args"]["mutationId"].as_str(), edited["component"]["target"]["activation"].as_str()),
        (Some("historyEditBegin"), Some(ids[38].as_str()), Some("historyEditBegin")),
        "{edited}"
    );

    let begun = app.handle_action("historyEditBegin", Some(&dsl(&edited["component"]["target"]["args"])), &meta(&fixture), &mut identity).await.expect("Edit dispatches");
    assert_eq!(rejected(&begun), None, "{:?}", begun.output);
    assert_eq!(app.time_travel.editor().map(|editor| editor.target.0.clone()), Some(ids[38].clone()), "Edit opens the 39th mutation");
    let edit_action = |body: &Value, id: &str| {
        find_node(body, &format!("framework.history.mutation.{id}")).map(|row| (row["component"]["rowActions"][0].clone(), row["component"]["target"]["activation"].clone())).unwrap_or_else(|| panic!("the row of {id}: {body}"))
    };
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] != Value::Bool(true) && activation.as_str() == Some("historyEditBegin"), "an unchanged draft lets Edit switch targets: {action}");
    verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::uint(77))], mounted_policy, &mut identity).await;
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(
        action["disabled"] == Value::Bool(true) && activation.is_null() && action["label"].as_str() == Some("Edit") && action["reason"].as_str().is_some_and(|reason| reason.starts_with("Blocked")),
        "a changed draft blocks Edit, naming why: {action}"
    );
    verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] == Value::Bool(true) && activation.is_null() && action["label"].as_str() == Some("Edit") && action["reason"].as_str() == Some("Not possible right now"), "a running replay disables Edit, naming why: {action}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    let (action, activation) = edit_action(&render_history_window(&mut app, row.seq, 32, 8).await, &ids[37]);
    assert!(action["disabled"] != Value::Bool(true) && activation.as_str() == Some("historyEditBegin") && action["label"].as_str() == Some("Edit"), "the review edits again: {action}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🌊️ LAW (design §15, transaction-scoped amend): a streamed tool transaction's ticks grow ONE edit and ONE history row
/// carrying its ref; while it is open a plain edit is refused `toolTransaction.open` and time travel `timeTravel.busy`; an
/// empty commit closes it with every op stamped; an aborted one leaves no edit, no row and the revision of before; streams
/// naming no transaction and aborts carrying mutations break the shape rule.
#[semio_framework_async_macros::async_test]
async fn a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    type ToyEmit = Emit<TestMutation, TestConfigMutation, NoDraftMutation>;
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let edits = app.stores.store.envelope().vcs.edits.len();
    let streamed = protocol::TransactionRef { id: "tx-00000000000000aa".into(), tool: format!("{}#import", ToyHistoryApp::APP_ID) };
    for value in [2, 3, 4] {
        app.dispatch_emit("select", ToyEmit::stream_transaction(streamed.clone(), vec![SetCount { value }.into()]), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("a tick streams");
    }
    assert_eq!((app.stores.store.envelope().vcs.edits.len(), app.stores.store.open_transaction().map(|open| open.transaction.clone())), (edits + 1, Some(streamed.clone())), "the ticks grow one open edit");
    let plain = app.dispatch_emit("select", ToyEmit::mutations(vec![SetCount { value: 9 }.into()]), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
    assert_eq!(plain.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.open")), "a plain edit waits for the open transaction");
    let first = seeded_mutation(&app, 0);
    let busy = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&busy), Some("timeTravel.busy"), "time travel waits for the open transaction");
    app.dispatch_emit("select", ToyEmit::commit_transaction(streamed.clone(), Vec::new()), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("an empty commit closes the edit");
    assert!(app.stores.store.open_transaction().is_none());
    let tail = app.stores.store.envelope().vcs.edits.last().expect("the committed edit");
    assert!(tail.forwards.len() == 3 && tail.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&streamed)), "one edit, every op stamped");
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    let rows: Vec<_> = patch.upserts.iter().filter(|entry| entry.transaction.as_ref().is_some_and(|reference| reference.id == streamed.id)).collect();
    assert_eq!((rows.len(), rows.first().map(|row| row.mutations.len())), (1, Some(3)), "one row with one mutation row per op");
    let (edits, revision) = (app.stores.store.envelope().vcs.edits.len(), app.stores.store.content_revision_now());
    let aborted = protocol::TransactionRef { id: "tx-00000000000000bb".into(), tool: streamed.tool.clone() };
    for value in [5, 6] {
        app.dispatch_emit("select", ToyEmit::stream_transaction(aborted.clone(), vec![SetCount { value }.into()]), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("a tick streams");
    }
    app.dispatch_emit("select", ToyEmit::abort_transaction(aborted.clone()), &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the abort reverts the open edit");
    assert_eq!((app.stores.store.envelope().vcs.edits.len(), app.stores.store.content_revision_now(), app.stores.store.open_transaction().is_none()), (edits, revision, true), "zero trace");
    let patch = app.render_ctx().history_patch(true).await.expect("history patch");
    let held: HashSet<&str> = app.stores.store.envelope().vcs.edits.iter().map(|edit| edit.id.as_str()).collect();
    assert!(patch.upserts.iter().all(|entry| entry.transaction.as_ref().is_none_or(|reference| reference.id != aborted.id) && entry.edit_id.as_deref().is_none_or(|edit_id| held.contains(edit_id))), "no row of the aborted transaction remains");
    let unnamed = ToyEmit { transaction: None, transaction_phase: TransactionPhase::Stream, ..ToyEmit::mutations(vec![SetCount { value: 7 }.into()]) };
    assert_eq!(app.dispatch_emit("select", unnamed, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
    let carrying = ToyEmit { artifact_mutations: vec![SetCount { value: 8 }.into()], ..ToyEmit::abort_transaction(aborted) };
    assert_eq!(app.dispatch_emit("select", carrying, &meta(&fixture), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.err().map(|fault| fault.code), Some(FaultCode::new("toolTransaction.shape")));
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🌲️ LAW: the band and the draft editor speak the one tree vocabulary both hosts mount — tree sections of rows, each row
/// holding at most one child that is not a row, and that child one single control (the kinds a wgpu row mounts as its
/// control; React mounts any): no text, progress bar or group is a row's child, and a second control is a nested row.
#[semio_framework_async_macros::async_test]
async fn the_band_and_the_editor_rows_hold_at_most_one_single_control() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    const SINGLE_CONTROLS: [&str; 9] = ["input", "select", "toggle", "button", "keyValueList", "slider", "numberStepper", "ring", "iconSelect"];
    fn violations(node: &Value, found: &mut Vec<String>) {
        let children = node["children"].as_array().map(Vec::as_slice).unwrap_or_default();
        if node["component"]["type"].as_str() == Some("treeItem") {
            let controls: Vec<&str> = children.iter().filter_map(|child| child["component"]["type"].as_str()).filter(|kind| *kind != "treeItem").collect();
            if controls.len() > 1 || controls.iter().any(|kind| !SINGLE_CONTROLS.contains(kind)) {
                found.push(format!("{}: {controls:?}", node["key"]));
            }
        }
        for child in children {
            violations(child, found);
        }
    }
    fn sections(history: &Value) -> Vec<String> {
        let mut found = Vec::new();
        for key in ["framework.history.timeTravel", "framework.history.editor", "framework.history.editor.inputs"] {
            if let Some(section) = find_node(history, key) {
                assert_eq!(section["component"]["type"].as_str(), Some("treeSection"), "{key} is a tree section");
                violations(section, &mut found);
            }
        }
        found
    }
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs =
        vec![semio_framework::ActionArgDef { nullable: true, ..semio_framework::ActionArgDef::number("/scale", LocalizedLabel::native("Scale", "Maßstab")) }, semio_framework::ActionArgDef::number("/width", LocalizedLabel::native("Width", "Breite"))];
    editor.value = dsl(&serde_json::json!({ "scale": 2.0, "width": 1.0 }));
    let history = render_history(&mut app, Locale::En).await;
    assert!(find_node(&history, "framework.history.editor.input.scale.clear").is_some(), "the nullable input keeps its Clear: {history}");
    assert_eq!(sections(&history), Vec::<String>::new(), "editing rows");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let history = render_history(&mut app, Locale::De).await;
    assert!(find_node(&history, "framework.history.timeTravel.finalize").is_some(), "the review offers Finalize: {history}");
    assert_eq!(sections(&history), Vec::<String>::new(), "reviewing rows");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧾️ LAW: unchanged Accept is refused through the app route, preserving the draft, preview and committed store.
#[semio_framework_async_macros::async_test]
async fn accepting_an_unchanged_input_retains_the_open_editor_and_preview() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 }), mounted_policy, &mut identity,cx).await;
    let session = app.time_travel.session().clone();
    let pending = session.pending.as_ref().expect("the open draft");
    assert!(session.unchanged(pending), "opening preserves the draft baseline: {pending:?}");
    assert!(!app.time_travel.panel().expect("native history panel").editor.expect("native editor availability").changed, "unchanged availability comes from the authoritative draft baseline");
    let input = app.time_travel.editor().expect("the draft editor").value.clone();
    let body = render_body(&mut app).await;
    let generation = app.stores.store.generation();
    for (locale, reason) in [(Locale::En, "Change an input before accepting"), (Locale::De, "Vor dem Übernehmen eine Eingabe ändern")] {
        let history = render_history(&mut app, locale).await;
        let accept = find_node(&history, "framework.history.editor.accept").expect("the Accept control");
        let row = find_node(&history, "framework.history.editor.accept.row").expect("the Accept row");
        assert_eq!(accept["disabled"], Value::Bool(true), "{locale:?}: {accept}");
        assert!(row.to_string().contains(reason), "{locale:?}: {row}");
        assert_ne!(find_node(&history, "framework.history.editor.discard").expect("the Discard control")["disabled"], Value::Bool(true));
    }
    let refused = verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&refused), Some(text(&fixture["refusals"]["unchanged"])));
    assert_eq!(app.time_travel.session(), &session);
    assert_eq!(app.time_travel.editor().expect("the draft editor stays open").value, input);
    assert_eq!(render_body(&mut app).await, body);
    assert_eq!(app.stores.store.generation(), generation);
    for (value, changed) in [(7, true), (1, false)] {
        run_step(&mut app, &fixture, &serde_json::json!({ "input": { "path": "/value", "value": value } }), mounted_policy, &mut identity,cx).await;
        let history = render_history(&mut app, Locale::En).await;
        let accept = find_node(&history, "framework.history.editor.accept").expect("the Accept control");
        assert_eq!(accept["disabled"] == Value::Bool(true), !changed, "{accept}");
    }
    let discarded = verb(&mut app, &fixture, "historyEditDiscard", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&discarded), None);
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    assert!(app.time_travel.status().is_none());
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a stage change ships its session status in the turn it happens — Begin from a review bumps the generation,
/// and the editing stage with that generation rides both the command reply and the unsolicited UI progress frame of the
/// same turn (the one carrier of a retained-surface intent, whose reply has no patch), even while the review's snapshots
/// still retire; a verb stamped with the published generation applies, one stamped with the previous is stale.
#[semio_framework_async_macros::async_test]
async fn a_stage_change_ships_its_generation_in_the_same_turn_and_a_verb_stamped_with_it_applies() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    while app.take_typed_operation_ui_progress().is_some() {}
    let reviewed = app.time_travel.session().generation;
    let mutation = seeded_mutation(&app, 2);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))], mounted_policy, &mut identity).await;
    let generation = app.time_travel.session().generation;
    assert_ne!(generation, reviewed, "Begin from a review bumps the generation");
    let status = |patch: &semio_framework::kernel::HistoryPatch| patch.time_travel.as_ref().map(|status| (status.stage, status.generation));
    let editing = Some((semio_framework::kernel::HistoryTimeTravelStage::Editing, generation));
    assert_eq!(begun.history_patch.as_ref().and_then(status), editing, "the reply publishes the new generation with the stage");
    let progress = app.take_typed_operation_ui_progress().expect("the stage change owes this turn a UI progress frame");
    assert_eq!(progress.history_patch.as_ref().and_then(status), editing, "the progress frame publishes it before any driver turn");
    assert!(progress.history_patch.as_ref().is_some_and(|patch| !patch.upserts.is_empty()), "it carries the rows the session overlay moved");
    assert!(app.time_travel.has_pending_work(), "the review's snapshots still retire behind it");
    let generation_arg = |value: u32| vec![(semio_framework::history_edit::HISTORY_EDIT_ARG_GENERATION.to_string(), DslValue::uint(u64::from(value)))];
    let stale = verb(&mut app, &fixture, "historyEditDiscard", generation_arg(reviewed), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&stale), Some("timeTravel.stale"), "the previous generation is stale");
    let discarded = verb(&mut app, &fixture, "historyEditDiscard", generation_arg(generation), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&discarded), None, "the published generation applies: {:?}", discarded.output);
    let deadline=std::time::Instant::now()+std::time::Duration::from_secs(30);
    let mut reviewing_progress=false;
    while std::time::Instant::now()<deadline {
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("original Discard bounded publication");
        collect_original_history_command_receipt(&mut app);
        while let Some(progress)=app.take_typed_operation_ui_progress(){reviewing_progress|=progress.history_patch.as_ref().and_then(status)==Some((semio_framework::kernel::HistoryTimeTravelStage::Reviewing,app.time_travel.session().generation));}
        if !app.time_travel.has_pending_work(){break}
    }
    assert!(!app.time_travel.has_pending_work(),"original Discard effects reach terminal custody");
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Reviewing);
    assert!(reviewing_progress,"the actual bounded return publishes its reviewing generation");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🌿️ LAW: the history body lists every line — the trunk (read "Main line", the original history) and the alternative a
/// finalize kept — with its current marker, author and branch time and whether it carries history edits, in English and
/// German; the other line offers Switch, whose rendered target dispatches `switchAlternative{alternativeId}`, and
/// switching reprojects: the edited alternative applies its history edit, the trunk shows the original.
#[semio_framework_async_macros::async_test]
async fn the_alternatives_section_lists_each_alternative_and_switching_reprojects_its_history_edits() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = replica(&fixture, "history-alternatives", &actor, true, mounted_policy, &mut identity).await;
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original", mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "b", "the edited alternative is current");
    let history = app.render_ctx().test_history().await;
    let listed: Vec<(&str, bool, bool)> = history.alternatives.iter().map(|alternative| (alternative.name.as_str(), alternative.current, alternative.edited)).collect();
    assert_eq!(listed, vec![("", false, false), ("Variant", true, true)], "the trunk first, then the kept edit");
    let (trunk, variant) = (history.alternatives[0].id.clone(), history.alternatives[1].id.clone());
    assert_eq!(history.alternatives[1].author.as_deref(), Some(actor.as_str()));
    assert!(history.alternatives[1].branched_at.as_deref().is_some_and(|at| at.ends_with('Z')), "{:?}", history.alternatives[1]);
    let row_key = |id: &str| format!("framework.history.alternative.{id}");
    for (locale, main_line, current, edited, switch) in [(Locale::En, "Main line", "Current", "Edited history", "Switch"), (Locale::De, "Hauptlinie", "Aktuell", "Bearbeiteter Verlauf", "Wechseln")] {
        let body = render_history(&mut app, locale).await;
        let section = find_node(&body, "framework.history.alternatives").unwrap_or_else(|| panic!("the alternatives section: {body}"));
        assert_eq!(section["component"]["type"].as_str(), Some("treeSection"));
        let trunk_row = find_node(section, &row_key(&trunk)).expect("the trunk row");
        assert!(trunk_row.to_string().contains(main_line), "{locale:?}: {trunk_row}");
        let action = &trunk_row["component"]["rowActions"][0];
        assert_eq!((action["verb"].as_str(), action["label"].as_str()), (Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID), Some(switch)), "{locale:?}: {trunk_row}");
        assert_eq!(trunk_row["component"]["target"]["activation"].as_str(), Some(semio_framework::SWITCH_ALTERNATIVE_ACTION_ID), "Enter on the row switches");
        let variant_row = find_node(section, &row_key(&variant)).expect("the variant row");
        assert!(variant_row.to_string().contains(current) && variant_row.to_string().contains(edited) && variant_row.to_string().contains(&actor), "{locale:?}: {variant_row}");
        assert!(variant_row["component"]["rowActions"].as_array().is_none_or(Vec::is_empty), "the current line offers no switch");
    }
    let body = render_history(&mut app, Locale::En).await;
    let args = find_node(&body, &row_key(&trunk)).expect("the trunk row")["component"]["target"]["args"].clone();
    assert_eq!(args[semio_framework::SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID].as_str(), Some(trunk.as_str()));
    history_verb(&mut app, &actor, semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, Some(dsl(&args)), mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "a", "the trunk shows the original history");
    let history = app.render_ctx().test_history().await;
    assert_eq!(history.alternatives.iter().map(|alternative| alternative.current).collect::<Vec<_>>(), vec![true, false]);
    history_verb(&mut app, &actor, semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, Some(dsl(&serde_json::json!({ "alternativeId": variant }))), mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "b", "switching back applies the history edit again");
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 📨️ LAW: opening a history edit delivers `TimeTravelFrozen` to every window of the view once (retargeting the open
/// session delivers none), and a remote edit moving the base delivers `BaseMoved` to every window of the last roster.
#[semio_framework_async_macros::async_test]
async fn opening_a_history_edit_and_a_remote_edit_deliver_host_events_to_every_window() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "host-events-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "host-events-remote", "remote", false, mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let roster = vec![semio_framework::ViewWindowInstance { id: "pane-a".into(), window_kind_id: "main".into() }, semio_framework::ViewWindowInstance { id: "pane-b".into(), window_kind_id: "main".into() }];
    let under = ActionMeta { view_state: Some(ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }), ..artifact_app_laws::meta(&actor) };
    let every = |event: fn(String) -> HostEvent| vec![event("pane-a".into()), event("pane-b".into())];
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    for index in [0, 1] {
        let mutation = seeded_mutation(&local, index);
        let begun = local.handle_action("historyEditBegin", Some(&DslValue::Object(vec![("mutationId".into(), DslValue::String(mutation))])), &under, &mut identity).await.expect("begin");
        assert_eq!(rejected(&begun), None);
    }
    assert_eq!(TOY_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut())), every(|window_id| HostEvent::TimeTravelFrozen { window_id }), "only opening the session freezes the windows");
    local.handle_action("historyEditExit", None, &under, &mut identity).await.expect("exit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }, &mut identity).await.expect("a remote edit");
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    relay(&mut remote_probe, &mut local_probe, &mut local, mounted_policy, &mut identity).await;
    assert_eq!(TOY_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut())), every(|window_id| HostEvent::BaseMoved { window_id }), "the remote edit moved the base");
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

//#region 🪞️Replicas
/// 🪞️ A registered toy replica on its own memory backbone, authoring as `actor`; `seed` dispatches the fixture seed
/// through its store, so every seed edit leaves in its outbox.
async fn replica(fixture: &Value, uri: &str, actor: &str, seed: bool, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (ToyApp, MemoryBackbone) {
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.into()), mounted_policy, identity).await;
    let (backbone, probe) = MemoryBackbone::pair(uri, uri).await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("attach the replica backbone");
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(actor.into()));
    if seed {
        for op in fixture["seed"].as_array().expect("seed") {
            app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![seed_op(op)], transaction: None }, identity).await.expect("seed edit applies");
        }
    }
    app.render_ctx().refresh_cache().await.expect("the command log backfills");
    (app, probe)
}

/// 🔀️ Delivers every event batch `from` published since the last relay into `to` (through its probe `into`) and ingests it.
#[doc(hidden)]
pub async fn relay(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut ToyApp, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    for message in from.receive().await.expect("replica outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        into.send(message).await.expect("relay an event batch");
    }
    to.tick_backbone(identity).await.expect("ingest the relayed events");
    to.render_ctx().refresh_cache().await.expect("the command log backfills the relayed events");
}

/// 🔀️ [`relay`], then drives `to`'s turns until the relayed change is adopted — its replay may span turns (gap N17).
async fn relay_adopted(from: &mut MemoryBackbone, into: &mut MemoryBackbone, to: &mut ToyApp, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    relay(from, into, to, mounted_policy, identity).await;
    pump_until(to, "the relayed change is adopted", |app| app.stores.store.reprojection_progress().is_none(), mounted_policy, identity,cx).await;
    to.render_ctx().refresh_cache().await.expect("the command log backfills the adopted change");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ↩️ Runs a framework history verb (`undo`, `redo`, `revertToCommand`) as `actor` through its reserved admission.
async fn history_verb(app: &mut ToyApp, actor: &str, action: &str, args: Option<DslValue>, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let admitted = app.handle_action(action, args.as_ref(), &meta, identity).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
    semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted).await.unwrap_or_else(|fault| panic!("{action} settles: {fault:?}"));
    app.render_ctx().refresh_cache().await.expect("history refresh");
}

/// 🧾️ The history-transition rows of `app`, newest first.
async fn history_edit_rows(app: &mut ToyApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().filter(|row| row.transition_id.is_some()).collect()
}

fn english(rows: &[semio_framework::kernel::HistoryEntry]) -> Vec<&str> {
    rows.iter().map(|row| row.label.resolve(Terminology::Native, Locale::En)).collect()
}

/// ✏️ The history mutation row of seeded op `index`.
async fn seeded_mutation_row(app: &mut ToyApp, index: usize) -> semio_framework::kernel::HistoryMutationEntry {
    let target = seeded_mutation(app, index);
    app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == target).expect("the seeded mutation row")
}
//#endregion 🪞️Replicas

/// ⚖️ LAW: a finalized history edit is its own history row, derived from the store's `Supersede` transition on the
/// authoring replica, on a replica that ingested it and after a text or pack reload — labelled in every locale with its
/// scope and mutation count, its author and time — and the edited mutation reads superseded from the store alike.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_is_its_own_row_locally_remotely_and_after_reload() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "history-edit-row-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "history-edit-row-remote", "remote", false, mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica holds the seed");
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone().into()), mounted_policy, &mut identity).await;
    assert_eq!(reloaded.stores.store.local_actor_id(), &protocol::ActorId(actor.clone().into()));
    artifact_app_laws::load_document_text(&mut reloaded, &local.document_text().await.expect("document text"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("text reload");
    reloaded.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into()), mounted_policy, &mut identity).await;
    assert_eq!(repacked.stores.store.local_actor_id(), &protocol::ActorId("reader".into()));
    artifact_app_laws::load_document(&mut repacked, &local.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    repacked.render_ctx().refresh_cache().await.expect("the repacked log backfills");
    for (who, app, revertible) in [("local", &mut local, true), ("remote", &mut remote, false), ("text reload", &mut reloaded, true), ("pack reload", &mut repacked, false)] {
        let rows = history_edit_rows(app).await;
        assert_eq!(english(&rows), ["History edited — overwrite: 1 mutation"], "{who}: one row per history edit, never a runtime duplicate");
        let row = &rows[0];
        assert_eq!(row.label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — überschrieben: 1 Mutation", "{who}");
        assert_eq!(row.author.as_deref(), Some(actor.as_str()), "{who}: the author");
        assert!(row.timestamp.parse::<u64>().is_ok_and(|ms| ms > 0), "{who}: the time, {}", row.timestamp);
        assert_eq!((row.kind.as_str(), row.action_id.as_str(), row.applied, row.revertible), ("history", semio_framework::history_edit::HISTORY_EDIT_COMMIT_ACTION_ID, true, revertible), "{who}");
        assert_eq!(row.key(), format!("transition:{}", row.transition_id.as_deref().expect("transition id")), "{who}: the row folds under its transition");
        let mutation = seeded_mutation_row(app, 1).await;
        assert!(mutation.superseded && !mutation.withdrawn, "{who}: the edited mutation reads superseded from the store");
        assert_eq!(head(app), (5, "b".to_string()), "{who}: the edited head");
        let history = render_history(app, Locale::De).await;
        assert!(find_node(&history, &format!("framework.history.entry.{}", row.seq)).is_some(), "{who}: the panel shows the row");
    }
    drop((local_probe, remote_probe));
    for app in [&mut local, &mut remote, &mut reloaded, &mut repacked] {
        close(app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: undo of a finalize is a new `Supersede` restoring the previous effective input (design §2), redo re-authors
/// the edit, both through the store's one supersede path and ingested alike by another replica; a newer own document
/// edit is undone first and redone last; the row's Backwards takes the edit back without touching document edits; and
/// another author can neither undo nor revert it.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_of_a_finalize_author_restoring_supersedes_on_both_replicas() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "history-edit-undo-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "history-edit-undo-remote", "remote", false, mounted_policy, &mut identity).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let edits = local.stores.store.envelope().vcs.edits.len();

    history_verb(&mut local, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "a".to_string()), "the undo restores the original input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[0].revertible && !rows[1].applied && !rows[1].revertible, "the undone history edit reads undone");
    assert!(!seeded_mutation_row(&mut local, 1).await.superseded, "an input restored to its original is not superseded");
    assert_eq!(local.stores.store.envelope().vcs.edits.len(), edits, "the undo is a history transition, never a document edit");
    assert!(local.render_ctx().history_patch(true).await.expect("patch").can_redo, "the undone history edit can be redone");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (5, "a".to_string()), "the remote replica folds the restoring supersede");
    assert_eq!(english(&history_edit_rows(&mut remote).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);

    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the redo re-authors the edited input");
    let rows = history_edit_rows(&mut local).await;
    assert_eq!(english(&rows), ["History edit redone — overwrite: 1 mutation", "History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    assert!(rows[0].applied && !rows[1].applied && rows[2].applied && rows[2].revertible, "the redone history edit is in effect again");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (5, "b".to_string()));

    local.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 7 }.into()], transaction: None }, &mut identity).await.expect("a newer document edit");
    local.render_ctx().refresh_cache().await.expect("the newer edit is logged");
    history_verb(&mut local, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the newer own document edit is undone first");
    history_verb(&mut local, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "a".to_string()), "then the history edit");
    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (5, "b".to_string()), "the history edit undone last is redone first");
    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (7, "b".to_string()), "then the document edit");

    let edited = history_edit_rows(&mut local).await.into_iter().rev().find(|row| row.revertible).expect("the revertible history edit row");
    history_verb(&mut local, &actor, REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::object([("entrySeq".to_string(), DslValue::float(edited.seq as f64))])), mounted_policy, &mut identity).await;
    assert_eq!(head(&local), (7, "a".to_string()), "Backwards takes the history edit back and leaves the document edits");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (7, "a".to_string()), "the replicas converge");

    history_verb(&mut local, &actor, "redo", None, mounted_policy, &mut identity).await;
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!((head(&local), head(&remote)), ((7, "b".to_string()), (7, "b".to_string())));
    let foreign = history_edit_rows(&mut remote).await;
    assert!(foreign.iter().all(|row| !row.revertible && row.author.as_deref() == Some(actor.as_str())), "another author's history edits are never revertible here");
    history_verb(&mut remote, "remote", "undo", None, mounted_policy, &mut identity).await;
    let original = foreign.iter().rev().find(|row| row.label.resolve(Terminology::Native, Locale::En).starts_with("History edited")).expect("the history edit row");
    history_verb(&mut remote, "remote", REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::object([("entrySeq".to_string(), DslValue::float(original.seq as f64))])), mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), (7, "b".to_string()), "another author can neither undo nor revert the history edit");
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a history edit committed as a new alternative is undone and redone within that alternative: its rows name
/// the alternative, the restoring supersede is scoped to it, and the alternative stays active.
#[semio_framework_async_macros::async_test]
async fn an_alternative_history_edit_is_undone_within_its_alternative() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = replica(&fixture, "history-edit-alternative", &actor, true, mounted_policy, &mut identity).await;
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original", mounted_policy, &mut identity).await;
    let variant = app.stores.store.envelope().active_alternative_id.clone().expect("the new alternative is active");
    assert_eq!(english(&history_edit_rows(&mut app).await), ["History edited — alternative Variant: 1 mutation"]);
    history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
    pump_until(&mut app, "the alternative undo lands", |app| !app.time_travel.authoring_pending(), mounted_policy, &mut identity,cx).await;
    assert_eq!(head(&app), (5, "a".to_string()), "the undo restores the original within the alternative");
    let rows = history_edit_rows(&mut app).await;
    assert_eq!(english(&rows), ["History edit undone — alternative Variant: 1 mutation", "History edited — alternative Variant: 1 mutation"]);
    assert_eq!(rows[1].label.resolve(Terminology::Native, Locale::De), "Verlauf bearbeitet — Alternative Variant: 1 Mutation");
    assert!(
        app.stores.store.supersessions().values().all(|supersession| supersession.scope.as_deref() == Some(variant.as_str()) && Some(supersession.transition_id.as_str()) == rows[0].transition_id.as_deref()),
        "the restoring supersede is scoped to the alternative"
    );
    assert_eq!(app.stores.store.envelope().active_alternative_id.as_deref(), Some(variant.as_str()), "the alternative stays active");
    history_verb(&mut app, &actor, "redo", None, mounted_policy, &mut identity).await;
    pump_until(&mut app, "the alternative redo lands", |app| !app.time_travel.authoring_pending(), mounted_policy, &mut identity,cx).await;
    assert_eq!(head(&app), (5, "b".to_string()));
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

//#region 🧭️AcceptanceLaws
/// 🖱️ Selects `children` in the toy's `test.slot` domain at the `child` granularity, as an interaction verb would.
async fn select_children(app: &mut ToyApp, children: &[String], mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
    let selection = protocol::DomainSelection { granularity: "child".into(), ids: children.to_vec(), anchor_id: None };
    let state = protocol::InteractionState { selection: BTreeMap::from([("test.slot".to_string(), selection)]), ..protocol::InteractionState::default() };
    app.stores.interaction_store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![InteractionConfigMutation::set_state(state)], lane: HistoryLane::Interaction, transaction: None }, identity).await.expect("the selection lands");
}

/// ⚖️ LAW (design §16.4, R14, R31): a blocked review is resolved by editing the failing mutation's targets — Next problem
/// opens the first blocking mutation from the review; "Use selection" refuses while nothing is selected, then drafts the
/// selected ids; the chips read the app's entity labels (German shell: German labels), every id is a chip row of the
/// reference's tree window (gap N2), a chip's remove binding removes its item; Accept replays and the review is ready.
#[semio_framework_async_macros::async_test]
async fn editing_targets_through_the_selection_resolves_a_blocked_review() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 2 }), serde_json::json!({ "input": { "path": "/children", "value": ["not a uri"] } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "fatal" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let failing = seeded_mutation(&app, 2);
    let panel = app.time_travel.panel().expect("a blocked review");
    assert_eq!((panel.review, panel.next_problem.as_deref()), (Some(semio_framework_time_travel::TimeTravelReview::Blocked), Some(failing.as_str())), "Next problem names the first blocking mutation");
    let history = render_history(&mut app, Locale::En).await;
    let next = find_node(&history, "framework.history.timeTravel.nextProblem").unwrap_or_else(|| panic!("the Next problem button: {history}"));
    assert_eq!(next["bindings"][0]["action"]["name"].as_str(), Some("historyEditBegin"));
    let begun = app.handle_action("historyEditBegin", Some(&dsl(&next["bindings"][0]["args"])), &meta(&fixture), &mut identity).await.expect("Next problem dispatches");
    assert_eq!(rejected(&begun), None, "{:?}", begun.output);
    assert_eq!(app.time_travel.editor().map(|editor| editor.target.0.as_str()), Some(failing.as_str()), "Next problem opens the failing mutation");
    let path = || vec![("path".to_string(), DslValue::String("/children".into()))];
    assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditUseSelection", path(), mounted_policy, &mut identity).await), Some("timeTravel.no-selection"), "nothing is selected yet");
    let children: Vec<String> = (1..=10).map(|index| format!("child-{index}!s.test.child@native/*")).collect();
    select_children(&mut app, &children, mounted_policy, &mut identity).await;
    let drafted = verb(&mut app, &fixture, "historyEditUseSelection", path(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&drafted), None, "{:?}", drafted.output);
    let drafted_children = |app: &ToyApp| app.time_travel.editor().and_then(|editor| editor.value.get("children").cloned());
    assert_eq!(drafted_children(&app), Some(DslValue::Array(children.iter().map(|child| DslValue::String(child.clone())).collect())), "the selection is the draft");
    let history = render_history(&mut app, Locale::De).await;
    let first = find_node(&history, "framework.history.editor.input.children.chip.0.row").unwrap_or_else(|| panic!("the first chip: {history}"));
    assert_eq!(first["component"]["label"].as_str(), Some("Kind child-1"), "a chip reads the entity label in the shell's locale: {first}");
    let references = find_node(&history, "framework.history.editor.input.children.row").unwrap_or_else(|| panic!("the reference row: {history}"));
    assert!(references["component"]["window"]["total"].as_u64().is_some_and(|total| total >= 11), "Use selection and one chip per id, all ten reachable: {references}");
    assert!(find_node(&history, "framework.history.editor.input.children.chip.9.row").is_some_and(|chip| chip.to_string().contains("Kind child-10")), "the tenth chip is a row too: {history}");
    let chip = find_node(&history, "framework.history.editor.input.children.chip.0").unwrap_or_else(|| panic!("the first chip's Remove: {history}"));
    assert_eq!(chip["bindings"][0]["args"]["edit"].as_str(), Some("remove"), "a chip removes its item by index: {chip}");
    let removed = app.handle_action("historyEditInput", Some(&dsl(&chip["bindings"][0]["args"])), &meta(&fixture), &mut identity).await.expect("the chip removes");
    assert_eq!(rejected(&removed), None, "{:?}", removed.output);
    assert_eq!(drafted_children(&app), Some(DslValue::Array(children[1..].iter().map(|child| DslValue::String(child.clone())).collect())), "removing a chip drafts the list without it");
    for step in [serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let status = app.time_travel.status().expect("a review");
    assert_eq!((status.review, status.blocking, status.worst), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, None), "edited targets resolve the review: {status:?}");
    assert!(seeded_mutation_row(&mut app, 2).await.worst.is_none(), "the edited mutation applies cleanly");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §16.5, R20): an upstream edit that turns a downstream mutation into a no-op reviews `ready` (a Warning
/// never blocks) with that mutation's Warning marked as introduced by the edit; finalizing keeps the Warning on its
/// history row — on the authoring replica, after a text reload and after a pack reload, in English and German.
#[semio_framework_async_macros::async_test]
async fn a_warning_an_edit_introduces_stays_visible_after_finalize_and_reload() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone().into()), mounted_policy, &mut identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(actor.clone().into()));
    for value in ["a", "b"] {
        app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetLabel { value: value.into() }.into()], transaction: None }, &mut identity).await.expect("a label edit applies");
    }
    app.render_ctx().refresh_cache().await.expect("the command log backfills");
    assert!(seeded_mutation_row(&mut app, 1).await.worst.is_none(), "before the edit the downstream label changes the document");
    for step in [serde_json::json!({ "begin": 0 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } }), serde_json::json!({ "accept": null }), serde_json::json!({ "replay": "clean" })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let status = app.time_travel.status().expect("a review");
    assert_eq!((status.review, status.blocking, status.worst), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, Some(semio_framework_diagnostic::Severity::Warning)), "a warning never blocks finalizing");
    let downstream = seeded_mutation_row(&mut app, 1).await;
    assert!(downstream.introduced && downstream.worst == Some(semio_framework_diagnostic::Severity::Warning) && downstream.messages.iter().any(|message| message.code == "mutation.no-op"), "the edit introduced the no-op: {downstream:?}");
    assert!(!seeded_mutation_row(&mut app, 0).await.introduced, "the edited mutation itself raises nothing new");
    let history = render_history(&mut app, Locale::En).await.to_string();
    assert!(history.contains("New since this edit") && history.contains("Warning: No change"), "{history}");
    for step in [serde_json::json!({ "finalize": null }), serde_json::json!({ "commit": { "choice": "overwrite" } })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    pump_until(&mut app, "the finalize retires", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    let mut reloaded = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(actor.clone().into()), mounted_policy, &mut identity).await;
    assert_eq!(reloaded.stores.store.local_actor_id(), &protocol::ActorId(actor.clone().into()));
    artifact_app_laws::load_document_text(&mut reloaded, &app.document_text().await.expect("document text"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("text reload");
    reloaded.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    let mut repacked = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into()), mounted_policy, &mut identity).await;
    assert_eq!(repacked.stores.store.local_actor_id(), &protocol::ActorId("reader".into()));
    artifact_app_laws::load_document(&mut repacked, &app.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    repacked.render_ctx().refresh_cache().await.expect("the repacked log backfills");
    for (who, app) in [("finalized", &mut app), ("text reload", &mut reloaded), ("pack reload", &mut repacked)] {
        let row = seeded_mutation_row(app, 1).await;
        assert!(row.worst == Some(semio_framework_diagnostic::Severity::Warning) && row.messages.iter().any(|message| message.code == "mutation.no-op") && !row.introduced, "{who}: the warning persists without a session: {row:?}");
        assert_eq!(head(app).1, "b", "{who}: the edited head");
        for (locale, line) in [(Locale::En, "Warning: No change"), (Locale::De, "Warnung: Keine Änderung")] {
            let history = render_history(app, locale).await.to_string();
            assert!(history.contains(line), "{who} {locale:?}: the row names the warning in words: {history}");
        }
    }
    for app in [&mut app, &mut reloaded, &mut repacked] {
        close(app,cx);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧱️ `replica` with one long edit after the seed, so a replay downstream of the seed exceeds one turn's budget.
async fn long_replica(fixture: &Value, uri: &str, actor: &str, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (ToyApp, MemoryBackbone) {
    let (mut app, probe) = replica(fixture, uri, actor, true, mounted_policy, identity).await;
    apply_long_edit(&mut app, 3_000, identity).await;
    app.render_ctx().refresh_cache().await.expect("the long edit is logged");
    (app, probe)
}

/// ⚖️ LAW (design §16.6, G9): a remote history change whose replay exceeds one turn's budget waits while the replica shows
/// the history before it, its progress rides the history wire (`reprojection`) and the body's reprojection section; Cancel replay
/// pauses it (driver turns leave it alone), Replay again resumes it, and the adoption equals the author's head. The remote
/// store replays one operation per turn, so the seed's downstream suffix already spans several turns.
#[semio_framework_async_macros::async_test]
async fn a_long_remote_history_change_replays_over_turns_and_pauses_on_cancel() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-replay-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-replay-remote", "remote", false, mounted_policy, &mut identity).await;
    remote.stores.store.defer_remote_replays(Some(store::os_store::store_schema::ReplayTurnBudget::operations(1)));
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), head(&local), "the remote replica holds the seed");
    commit_scenario(&mut local, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    let status = remote.render_ref().reprojection_status().expect("the remote history change waits for its replay");
    assert!(status.total > 0 && status.done < status.total && !status.paused && status.kind == semio_framework::kernel::HistoryReprojectionKind::Remote && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a", "the replica shows the history before the change");
    assert_eq!(remote.render_ctx().history_patch(true).await.expect("history patch").reprojection.as_ref().map(|remote| remote.total), Some(status.total), "the wire carries the remote replay");
    let body = render_history(&mut remote, Locale::En).await;
    assert!(find_node(&body, "framework.history.reprojection.status").is_some_and(|row| row.to_string().contains("Replaying a remote history change")), "{body}");
    let cancelled = verb(&mut remote, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&cancelled), None, "{:?}", cancelled.output);
    assert!(remote.time_travel.reprojection_paused(), "the remote replay is paused");
    for _ in 0..4 {
        remote.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
        collect_original_history_command_receipt(&mut remote);
        while remote.take_typed_operation_ui_progress().is_some() {}
    }
    assert!(remote.time_travel.reprojection_paused() && !remote.time_travel_ref().time_travel_has_pending_work(), "once its patch shipped, a paused remote replay is no driver work");
    assert_eq!(head(&remote).1, "a", "paused, the change is not adopted");
    let body = render_history(&mut remote, Locale::De).await;
    assert!(find_node(&body, "framework.history.reprojection.rerun").is_some() && body.to_string().contains("pausiert"), "{body}");
    let resumed = verb(&mut remote, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&resumed), None, "{:?}", resumed.output);
    assert_eq!(rejected(&verb(&mut remote, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await), Some("timeTravel.illegal"), "nothing is paused any more");
    pump_until(&mut remote, "the remote change is adopted", |app| app.stores.store.reprojection_progress().is_none(), mounted_policy, &mut identity,cx).await;
    assert_eq!(head(&remote), head(&local), "the adoption equals the author's head");
    assert!(remote.render_ref().reprojection_status().is_none(), "nothing waits any more");
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §16.6, G9): the undo of a finalize over a long downstream history is authored resumably — the verb answers
/// at once, history editing is busy while its replay runs, and the driver commits it: the original input is back and the
/// undo row reads like every other.
#[semio_framework_async_macros::async_test]
async fn the_undo_of_a_finalize_over_a_long_history_lands_through_the_driver() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut app, probe) = long_replica(&fixture, "long-undo", &actor, mounted_policy, &mut identity).await;
    commit_scenario(&mut app, &fixture, "overwrite-supersedes-in-place", mounted_policy, &mut identity).await;
    assert_eq!(head(&app).1, "b");
    history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
    if app.time_travel.authoring_pending() {
        let first = seeded_mutation(&app, 0);
        let busy = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))], mounted_policy, &mut identity).await;
        assert_eq!(rejected(&busy), Some("timeTravel.busy"), "history editing waits for the authoring");
    }
    pump_until(&mut app, "the undo lands", |app| !app.time_travel.authoring_pending(), mounted_policy, &mut identity,cx).await;
    app.render_ctx().refresh_cache().await.expect("history refresh");
    assert_eq!(head(&app).1, "a", "the undo restores the original input");
    assert_eq!(english(&history_edit_rows(&mut app).await), ["History edit undone — overwrite: 1 mutation", "History edited — overwrite: 1 mutation"]);
    drop(probe);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
/// 🔁️ A history verb as `actor`, admitted and settled, answering the fault code of a refusal (`None`: applied).
async fn history_verb_refusal(app: &mut ToyApp, actor: &str, action: &str, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Option<String> {
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(actor) };
    let settled = match app.handle_action(action, None, &meta, identity).await {
        Ok(admitted) => semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted).await,
        Err(fault) => Err(fault),
    };
    settled.err().map(|fault| fault.code.0)
}

/// 🧮️ What a history step may change: the head, the content revision, the edits, the transitions and the history rows.
async fn history_trace(app: &mut ToyApp) -> ((i32, String), [u8; 32], usize, usize, usize) {
    let rows = app.render_ctx().history_patch(true).await.expect("history patch").upserts.len();
    (head(app), app.stores.store.content_revision_now(), app.stores.store.envelope().vcs.edits.len(), app.stores.store.envelope().transitions.len(), rows)
}

/// ✏️ Publishes a foreign downstream edit per value and drains displaced owners under bounded pressure.
async fn apply_other_edits(app: &mut ToyApp, count: i32) {
    for value in 0..count {
        publish_as(app, "downstream", SetCount { value }.into());
        for _ in 0..4_096 {
            if !app.stores.store.maintenance_retirements_under_pressure(){break;}
            let grant=app.mounted_policy.maintenance;
            let step=app.stores.store.maintenance_retirements_step(grant).expect("the store retires what the edits displaced");assert!(step.progress().fits(grant));
            if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){break;}
        }
    }
}

/// ⚰️ One edit of `author` ([`publish_as`]) under `count` one-operation edits of the fixture actor
/// ([`apply_other_edits`]): the history an interior undo of `author` replays through. Proves what it built: exactly one
/// history row is `author`'s and it is not the newest.
async fn bury_edit_of(app: &mut ToyApp, author: &str, count: i32) {
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId(author.into()), "the buried edit belongs to the opened actor");
    publish_as(app, author, SetLabel { value: "buried".into() }.into());
    apply_other_edits(app, count).await;
    app.render_ctx().refresh_cache().await.expect("backfill");
    let authors: Vec<Option<String>> = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().map(|row| row.author).collect();
    let own = authors.iter().filter(|row| row.as_deref() == Some(author)).count();
    assert!(own == 1 && authors.first().and_then(|row| row.as_deref()) != Some(author), "one interior row is {author}'s: {own} of {} rows, newest {:?}", authors.len(), authors.first());
}

/// ⚖️ LAW (gap N17): an interior undo over a long downstream history — the author's own edit under 600 edits of another
/// author — is a local history step the runtime replays over driver turns:
/// - the undo answers at once, the head, the edits and the history rows unchanged;
/// - the wire carries `reprojection.kind = step`, the body reads "Replaying history" / "Verlauf wird neu angewendet";
/// - each turn replays at most `TIME_TRAVEL_REPLAY_OPERATIONS`;
/// - meanwhile undo and redo answer `history.replaying` and `historyEditBegin` answers `timeTravel.busy`;
/// - the adoption equals the undo of a store that replays inside the dispatch, and records the undo row.
/// Cancel replay instead drops the step with zero trace: head, revision, edits, transitions and history rows as before.
#[semio_framework_async_macros::async_test]
async fn an_interior_undo_over_a_long_history_replays_over_turns_and_cancel_leaves_zero_trace() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = "author".to_string();
    let mut undeferred = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
    undeferred.store.defer_local_replays(None);
    bury_edit_of(&mut undeferred, &actor, 600).await;
    history_verb(&mut undeferred, &actor, "undo", None, mounted_policy, &mut identity).await;
    let undone = undeferred.store.snapshot().expect("the undeferred undo");
    for cancel in [true, false] {
        let mut app = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
        bury_edit_of(&mut app, &actor, 600).await;
        let before = history_trace(&mut app).await;
        history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
        assert!(app.stores.store.local_step_pending(), "the interior undo waits for its replay");
        assert_eq!(history_trace(&mut app).await, before, "nothing of the step shows before its adoption");
        let status = app.render_ctx().history_patch(true).await.expect("history patch").reprojection.expect("the wire carries the waiting step");
        assert!(status.kind == semio_framework::kernel::HistoryReprojectionKind::Step && status.total > 0 && !status.paused && status.fault.is_none(), "{status:?}");
        for (locale, line) in [(Locale::En, "Replaying history"), (Locale::De, "Verlauf wird neu angewendet")] {
            let body = render_history(&mut app, locale).await;
            assert!(find_node(&body, "framework.history.reprojection.status").is_some_and(|row| row.to_string().contains(line)), "{locale:?}: {body}");
        }
        assert_eq!(history_verb_refusal(&mut app, &actor, "undo", mounted_policy, &mut identity).await.as_deref(), Some("history.replaying"), "a further history step waits");
        assert_eq!(history_verb_refusal(&mut app, &actor, "redo", mounted_policy, &mut identity).await.as_deref(), Some("history.replaying"));
        let first = seeded_mutation(&app, 0);
        assert_eq!(rejected(&verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(first))], mounted_policy, &mut identity).await), Some("timeTravel.busy"), "history editing waits for the step");
        if cancel {
            let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
            assert_eq!(rejected(&cancelled), None, "{:?}", cancelled.output);
            assert!(!app.stores.store.local_step_pending() && app.render_ref().reprojection_status().is_none(), "the step is gone");
            for _ in 0..4 {
                app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
                collect_original_history_command_receipt(&mut app);
            }
            assert_eq!(history_trace(&mut app).await, before, "a cancelled step leaves zero trace");
        } else {
            let mut turns = 0;
            while app.stores.store.local_step_pending() {
                let done = app.stores.store.reprojection_progress().map_or(0, |progress| progress.done);
                app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
                collect_original_history_command_receipt(&mut app);
                while app.take_typed_operation_ui_progress().is_some() {}
                let after = app.stores.store.reprojection_progress().map_or(u32::MAX, |progress| progress.done);
                assert!(after == u32::MAX || after.saturating_sub(done) as usize <= time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, "one turn replays at most one budget: {done} → {after}");
                turns += 1;
                assert!(turns < 64, "the step is adopted");
            }
            assert!(turns >= 2, "a 600-operation downstream spans several turns ({turns})");
            assert_eq!(app.stores.store.snapshot().expect("the adopted undo"), undone, "the adoption equals the undeferred undo");
            let (_, _, edits, _, rows) = history_trace(&mut app).await;
            assert_eq!((edits, rows), (before.2, before.4 + 1), "the adoption records the undo row");
        }
        close(&mut app,cx);
    }
    close(&mut undeferred,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (decision §4 of `📓️api-stepped-document-load.md`): a pure command's document lane hydrates the HEAD snapshot of
/// a 240-edit document without its history (`PluginApp::hydrate_pure_head` takes the head pack alone, so no history can
/// ride it) — the store holds no edit, the head is the source head, nothing folds; in that head-only mode undo, a history
/// edit and the history query refuse `pure.history-unavailable`, and the whole-document archive load of the history (the
/// only load path) lifts it over real polls, landing the source head and all 240 edits.
#[semio_framework_async_macros::async_test]
async fn a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut source = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
    for value in 0..240 {
        source.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], transaction: None }, &mut identity).await.expect("a source edit");
    }
    let head = source.stores.store.snapshot().expect("the source head");
    let files = source.document_pack().await.expect("the source pair");
    let mut pure = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId(text(&fixture["actor"]).into()), mounted_policy, &mut identity).await;
    let code = |fault: Fault| fault.code.0;
    PluginApp::hydrate_pure_head(&mut pure, &head.encode_pack()).await.expect("the head-only lane hydrates");
    assert_eq!(pure.stores.store.snapshot().expect("the pure head"), head, "the pure document is the source head");
    assert!(pure.stores.store.envelope().vcs.edits.is_empty() && pure.time_travel.history_unavailable(), "no history was folded or kept");
    assert_eq!(history_verb_refusal(&mut pure, &actor, "undo", mounted_policy, &mut identity).await.as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE));
    let meta = ActionMeta { view_state: Some(ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), ..artifact_app_laws::meta(&actor) };
    let begin = pure.handle_action("historyEditBegin", Some(&DslValue::object([("mutationId".to_string(), DslValue::String("any".into()))])), &meta, &mut identity).await;
    assert_eq!(begin.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "a history edit needs the history");
    assert_eq!(pure.history_snapshot().await.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "so does the history query");
    let envelope = pure.stores.store.envelope();
    let dialect: ArtifactDialect = <ToyHistoryApp as ArtifactApp>::DIALECT.into();
    let parent_spr = store::os_store::text_format::stamp_document_spr_identity(&files.spr, &envelope.id, <ToyHistoryApp as ArtifactApp>::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref()).await.expect("the load identity stamp");
    let operation = 0x51;
    PluginApp::begin_document_archive_load(&mut pure, operation, protocol::DocumentArchivePack { parent_pack: files.pack.clone(), parent_spr, members: Vec::new() }).expect("the archive load is admitted");
    let mut polls = 0;
    let status = loop {
        let status = PluginApp::poll_document_archive_load(&mut pure, operation, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("the archive load status");
        if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
            break status;
        }
        polls += 1;
        assert!(polls < 1_000_000, "the archive load reaches a terminal state");
    };
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "the archive load lands: {}", if status.fault.is_empty() { String::new() } else { semio_framework_diagnostic::decode_fault_bytes(&status.fault).describe() });
    PluginApp::acknowledge_document_archive_load(&mut pure, operation).expect("the terminal load is released");
    assert!(!pure.time_travel.history_unavailable() && pure.history_snapshot().await.is_ok(), "the archive load with history lifts head-only mode");
    assert_eq!((pure.stores.store.snapshot().expect("the loaded head"), pure.stores.store.envelope().vcs.edits.len()), (head, source.stores.store.envelope().vcs.edits.len()), "the archive load lands the source head and its whole history (seed + 240 edits)");
    close(&mut pure,cx);
    close(&mut source,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🩹️HistoryViewPatch

//#region ⏱️WallDeadline
thread_local! {
    /// ⏱️ A clock on this test thread that advances one millisecond per read: every replayed operation "costs" 1 ms.
    static COUNTED_CLOCK_US: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// ⏱️ Reads [`COUNTED_CLOCK_US`], advancing it by one millisecond.
fn counted_clock() -> Option<u64> {
    Some(COUNTED_CLOCK_US.with(|now| {
        now.set(now.get() + 1_000);
        now.get()
    }))
}

/// ⚖️ LAW (design §20.14, audit W2A-3): a deferred history step ends its driver turn at the turn's wall deadline, never at
/// the operation cap — with every clock read costing 1 ms, an interior undo over 600 downstream edits replays a handful of
/// them per 4 ms turn (well under the 256-operation cap), over many turns, and still lands.
#[semio_framework_async_macros::async_test]
async fn a_deferred_history_step_ends_its_turn_at_the_wall_deadline_not_an_operation_count() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = "author".to_string();
    let mut app = seeded_app_as(&fixture, &actor, mounted_policy, &mut identity).await;
    app.time_travel.set_turn_clock(counted_clock);
    app.stores.store.defer_local_replays(Some(store::os_store::store_schema::ReplayTurnBudget { wall_us: TIME_TRAVEL_TURN_WALL_US, operations: time_travel::TIME_TRAVEL_REPLAY_OPERATIONS, now_us: counted_clock }));
    bury_edit_of(&mut app, &actor, 600).await;
    history_verb(&mut app, &actor, "undo", None, mounted_policy, &mut identity).await;
    assert!(app.stores.store.local_step_pending(), "the interior undo waits for its replay");
    let mut turns = 0usize;
    while app.stores.store.local_step_pending() {
        let done = app.stores.store.reprojection_progress().map_or(0, |progress| progress.done);
        app.advance_typed_operation_publication(&mut identity,cx).await.expect("driver turn");
        collect_original_history_command_receipt(&mut app);
        while app.take_typed_operation_ui_progress().is_some() {}
        let after = app.stores.store.reprojection_progress().map_or(u32::MAX, |progress| progress.done);
        assert!(after == u32::MAX || after.saturating_sub(done) <= 8, "a 4 ms turn of 1 ms operations replays a handful: {done} → {after}");
        turns += 1;
        assert!(turns < 1_000, "the step is adopted");
    }
    assert!(turns >= 600 / 8, "600 operations of 1 ms span many 4 ms turns ({turns})");
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §22.6, runtime half of §22.34): an opened instance acts as its admitted actor — at its open, across a
/// reload and on the revert route.
/// - construction binds the admitted actor before genesis; [`PluginApp::bind_actor`] verifies that same authority;
/// - a whole-document reload keeps the instance's actor although the loaded history ends in another author's edit, so an
///   undo there takes nothing back; a head-only pure hydrate keeps it too;
/// - the revert route acts as the reverting actor: the store acts as that actor afterwards and another actor's edit stays
///   applied; an undo takes back the acting actor's own edit.
#[semio_framework_async_macros::async_test]
async fn an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let mut app = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("ada".into()), mounted_policy, &mut identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId("ada".into()), "construction binds the admitted actor");
    PluginApp::bind_actor(&mut app, "ada").await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId("ada".into()), "the admitted actor is the instance's");
    publish_as(&mut app, "ada", SetCount { value: 1 }.into());
    app.render_ctx().refresh_cache().await.expect("the log backfills the instance's own edit");
    publish_as(&mut app, "grace", SetCount { value: 2 }.into());
    app.render_ctx().refresh_cache().await.expect("the log backfills the other actor's edit");
    assert_eq!(edit_authors(&app), [Some("ada".to_string()), Some("grace".to_string())], "each edit names the actor that published it");
    let other = app.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).expect("the other actor's edit");

    let mut reader = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), protocol::ActorId("reader".into()), mounted_policy, &mut identity).await;
    PluginApp::bind_actor(&mut reader, "reader").await;
    artifact_app_laws::load_document(&mut reader, &app.document_pack().await.expect("document pack"), &mut identity).await.expect("pack reload");
    reader.render_ctx().refresh_cache().await.expect("the reloaded log backfills");
    assert_eq!((reader.store.local_actor_id(), head(&reader).0), (&protocol::ActorId("reader".into()), 2), "a reload keeps the instance's actor although the history ends in another author's edit");
    history_verb(&mut reader, "reader", "undo", None, mounted_policy, &mut identity).await;
    assert_eq!((head(&reader).0, reader.store.applied_edit_ids().len()), (2, 2), "an undo never takes back another author's edit");
    let pure_head = reader.store.snapshot().expect("the reloaded head").encode_pack();
    PluginApp::hydrate_pure_head(&mut reader, &pure_head).await.expect("the head-only lane hydrates");
    assert_eq!(reader.store.local_actor_id(), &protocol::ActorId("reader".into()), "a head-only hydrate keeps the instance's actor");

    let own = app.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().find(|row| row.author.as_deref() == Some("ada")).expect("the instance's own row").seq;
    history_verb(&mut app, "ada", "revertToCommand", Some(DslValue::object([("entrySeq".to_string(), DslValue::uint(own))])), mounted_policy, &mut identity).await;
    assert_eq!(app.stores.store.local_actor_id(), &protocol::ActorId("ada".into()), "the revert route acts as the reverting actor");
    assert!(app.stores.store.applied_edit_ids().iter().any(|id| *id == other), "a revert never takes back another actor's edit");
    history_verb(&mut app, "grace", "undo", None, mounted_policy, &mut identity).await;
    pump_until(&mut app, "the undo lands", |app| !app.stores.store.local_step_pending(), mounted_policy, &mut identity,cx).await;
    assert!(!app.stores.store.applied_edit_ids().iter().any(|id| *id == other), "an undo takes back the acting actor's own edit");
    close(&mut reader,cx);
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion ☑️LongOptionRows

//#region 📡️RemoteEditWhileEditing
/// ⚖️ LAW (live fault F4, session half; design §7: remote ingests keep arriving → `BaseMoved`): a remote edit ingested
/// while a draft is being edited moves the base and nothing else. The session stays `Editing` with its draft, its editor
/// and its generation, so an input stamped with the generation the editor's controls carried before the base move is
/// not stale; the remote edit is listed downstream of the edited mutation as not applied while editing, and nothing
/// waits for a replay (a tail edit is adopted at ingest); Accept replays the remote edit too — the reviewed head holds
/// the draft and the remote edit — and the overwrite converges on the other replica.
#[semio_framework_async_macros::async_test]
async fn a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let (mut local, mut local_probe) = replica(&fixture, "remote-while-editing-local", &actor, true, mounted_policy, &mut identity).await;
    let (mut remote, mut remote_probe) = replica(&fixture, "remote-while-editing-remote", "remote", false, mounted_policy, &mut identity).await;
    relay(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut local, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let (target, generation) = (seeded_mutation(&local, 1), local.time_travel.session().generation);
    let drafted = local.time_travel.session().pending.clone().expect("a pending draft");

    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: 9 }.into()], transaction: None }, &mut identity).await.expect("the remote edit");
    relay_adopted(&mut remote_probe, &mut local_probe, &mut local, mounted_policy, &mut identity).await;
    pump_until(&mut local, "the base move reaches the session", |app| app.time_travel.session().base.content_revision == app.stores.store.content_revision(), mounted_policy, &mut identity,cx).await;
    let arrived = local.stores.store.mutation_ops().expect("applied operations").last().map(|op| op.mutation_id.0.clone()).expect("the remote operation");
    assert!(arrived != seeded_mutation(&local, 3), "the remote edit is applied in this replica's store");

    let session = local.time_travel.session();
    assert_eq!((session.stage, session.generation, session.pending.as_ref()), (TimeTravelStage::Editing, generation, Some(&drafted)), "the base move keeps the stage, the generation and the draft");
    assert_eq!(local.time_travel.editor().map(|editor| (editor.target.0.clone(), editor.value.get("value").and_then(DslValue::as_str).map(str::to_string))), Some((target.clone(), Some("b".to_string()))), "the editor keeps its drafted value");
    assert!(local.render_ref().reprojection_status().is_none(), "a tail edit is adopted at ingest: nothing waits for a replay");
    let wire = local.render_ctx().history_patch(true).await.expect("history patch").upserts.into_iter().flat_map(|row| row.mutations).find(|mutation| mutation.mutation_id == arrived).expect("the remote edit is a history row");
    assert!(wire.pending && wire.label.resolve(Terminology::Native, Locale::En) == "Set count to 9", "the remote edit is downstream and not applied in the preview: {wire:?}");
    let seq = local.render_ctx().history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == arrived)).map(|entry| entry.seq).expect("the history row of the remote edit");
    let body = render_history_window(&mut local, seq, 0, 8).await;
    let row = find_node(&body, &format!("framework.history.mutation.{arrived}")).unwrap_or_else(|| panic!("the mutation row of the remote edit: {body}"));
    assert!(row.to_string().contains("Not applied while editing"), "the row says so in words: {row}");
    assert!(render_body(&mut local).await.contains("count=1 label=b"), "the preview stays the state before the edited mutation with the draft");

    let typed = verb(&mut local, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("c".into())), ("generation".into(), DslValue::uint(u64::from(generation)))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&typed), None, "the user's next input, stamped with the generation from before the base move, is not stale");

    let accepted = verb(&mut local, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(rejected(&accepted), None, "{:?}", accepted.output);
    pump_until(&mut local, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    let status = local.time_travel.status().expect("a reviewing session");
    assert_eq!((status.review, status.blocking, status.accepted_count), (Some(semio_framework::kernel::HistoryTimeTravelReview::Ready), false, 1), "the replay over the remote edit is clean");
    assert!(render_body(&mut local).await.contains("count=9 label=c"), "the reviewed head holds the draft and the remote edit");

    for (action, args) in [("historyEditFinalize", Vec::new()), ("historyEditCommit", vec![("choice".to_string(), DslValue::String("overwrite".into()))])] {
        let result = verb(&mut local, &fixture, action, args, mounted_policy, &mut identity).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    assert_eq!(head(&local), (9, "c".to_string()), "the overwrite lands on the remote edit");
    relay_adopted(&mut local_probe, &mut remote_probe, &mut remote, mounted_policy, &mut identity).await;
    assert_eq!(head(&remote), head(&local), "the other replica adopts the history edit");
    pump_until(&mut local, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    drop((local_probe, remote_probe));
    close(&mut local,cx);
    close(&mut remote,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 📡️RemoteEditWhileEditing

//#region 🎞️SessionEdgeScope
/// ⚖️ LAW (live probe: the `choosing` edge; design §20.14, per-render work is O(change)): a history-edit verb re-publishes
/// every window body only when the document a window shows was swapped — opening the session (committed → preview), a
/// draft (a new preview), a finished replay (preview → head), leaving (→ committed). The edges that swap nothing —
/// an accepted draft starting its replay, the finalize prompt opening and closing, a replay run again — re-publish the
/// history body alone; and a replay run again from a review keeps showing the head reviewed last, never the committed
/// document.
#[semio_framework_async_macros::async_test]
async fn a_session_edge_republishes_every_window_only_when_the_shown_document_swaps() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let full = |result: &InvocationResult| matches!(result.ui_scope, UiDirtyScope::Full);
    let history_only = |result: &InvocationResult| matches!(&result.ui_scope, UiDirtyScope::Partial { window_bodies, panel_bodies, .. } if window_bodies.is_empty() && panel_bodies.len() == 1 && panel_bodies[0] == FRAMEWORK_HISTORY_BODY_KEY);
    let mutation = seeded_mutation(&app, 1);
    let begun = verb(&mut app, &fixture, "historyEditBegin", vec![("mutationId".into(), DslValue::String(mutation))], mounted_policy, &mut identity).await;
    assert!(rejected(&begun).is_none() && full(&begun), "opening shows the preview in every window: {:?}", begun.ui_scope);
    let drafted = verb(&mut app, &fixture, "historyEditInput", vec![("path".into(), DslValue::String("/value".into())), ("value".into(), DslValue::String("b".into()))], mounted_policy, &mut identity).await;
    assert!(rejected(&drafted).is_none() && full(&drafted), "a draft is a new preview: {:?}", drafted.ui_scope);
    let accepted = verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&accepted).is_none() && history_only(&accepted), "an accepted draft keeps its preview while it replays: {:?}", accepted.ui_scope);
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the preview stays while the replay runs");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let prompted = verb(&mut app, &fixture, "historyEditFinalize", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&prompted).is_none() && history_only(&prompted), "the finalize prompt swaps no document: {:?}", prompted.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Choosing);
    let back = verb(&mut app, &fixture, "historyEditBack", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&back).is_none() && history_only(&back), "leaving the prompt swaps no document: {:?}", back.ui_scope);

    let first = seeded_mutation(&app, 0);
    for (action, args) in [("historyEditBegin", vec![("mutationId".to_string(), DslValue::String(first))]), ("historyEditInput", vec![("path".to_string(), DslValue::String("/value".into())), ("value".to_string(), DslValue::uint(7))]), ("historyEditAccept", Vec::new())] {
        let result = verb(&mut app, &fixture, action, args, mounted_policy, &mut identity).await;
        assert_eq!(rejected(&result), None, "{action}: {:?}", result.output);
    }
    let cancelled = verb(&mut app, &fixture, "historyEditCancelReplay", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&cancelled).is_none() && full(&cancelled), "a cancelled replay leaves its preview for the head reviewed last: {:?}", cancelled.ui_scope);
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the cancelled review shows the head reviewed last");
    let rerun = verb(&mut app, &fixture, "historyEditRerun", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&rerun).is_none() && history_only(&rerun), "a replay run again keeps the head reviewed last: {:?}", rerun.ui_scope);
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = render_body(&mut app).await;
    assert!(replaying.contains("count=5 label=b") && !replaying.contains(text(&fixture["committedBody"])), "no window falls back to the committed document while it replays again: {replaying}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    assert!(render_body(&mut app).await.contains("count=5 label=b"), "the review shows the replayed head");

    let exited = verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    assert!(rejected(&exited).is_none() && full(&exited), "leaving shows the committed document in every window: {:?}", exited.ui_scope);
    assert!(render_body(&mut app).await.contains(text(&fixture["committedBody"])));
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🎞️SessionEdgeScope

//#region 🚫️RowWithdraw
/// 📍️ The rendered mutation row of seeded edit `index`, its history row opened as a host window shows it.
async fn seeded_mutation_node(app: &mut ToyApp, index: usize) -> Value {
    let id = seeded_mutation(app, index);
    let seq = app.render_ctx().history_patch(true).await.expect("history patch").upserts.iter().find(|entry| entry.mutations.iter().any(|mutation| mutation.mutation_id == id)).map(|entry| entry.seq).expect("the history row of the seeded edit");
    let body = render_history_window(app, seq, 0, 8).await;
    find_node(&body, &format!("framework.history.mutation.{id}")).cloned().unwrap_or_else(|| panic!("the mutation row of seeded edit {index}: {body}"))
}

/// ⚖️ LAW (design §22.1, coordinator decision "Restore"): every applied mutation row offers Withdraw beside Edit on the one
/// row target. A row's Withdraw opens the session on a withdrawn draft (the preview is the state before the mutation, the
/// store untouched); on the mutation being edited it stays the editor's own while every other row names why it is blocked;
/// a running replay refuses it naming why; a mutation holding an accepted draft offers Restore in Withdraw's place and is
/// no longer withdrawable on the wire; restoring the only accepted draft leaves history editing with zero trace.
#[semio_framework_async_macros::async_test]
async fn a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let generation = app.stores.store.generation();
    let (first, target) = (seeded_mutation(&app, 0), seeded_mutation(&app, 3));
    let enabled = |verb: &str, label: &str| (verb.to_string(), label.to_string(), false, None);
    let idle = seeded_mutation_node(&mut app, 3).await;
    assert_eq!(row_actions(&idle), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "Edit stays first, Withdraw follows: {idle}");
    assert_eq!((idle["component"]["target"]["args"]["mutationId"].as_str(), idle["component"]["target"]["activation"].as_str()), (Some(target.as_str()), Some("historyEditBegin")), "one row target for both actions: {idle}");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.editable && wire.withdrawable && !wire.withdrawn, "the wire row is withdrawable");

    let withdrawn = app.handle_action("historyEditWithdraw", Some(&dsl(&idle["component"]["target"]["args"])), &meta(&fixture), &mut identity).await.expect("the row's Withdraw dispatches");
    assert_eq!(rejected(&withdrawn), None, "{:?}", withdrawn.output);
    let session = app.time_travel.session();
    assert_eq!((session.stage, session.pending.as_ref().map(|pending| (pending.target.mutation.0.clone(), pending.replacement.clone()))), (TimeTravelStage::Editing, Some((target.clone(), protocol::InputReplacement::Withdrawn))), "the session opens on a withdrawn draft");
    assert_eq!((render_body(&mut app).await.contains("count=1 label=a"), app.stores.store.generation()), (true, generation), "the preview is the state before the mutation; the store is untouched");
    let editing = row_actions(&seeded_mutation_node(&mut app, 3).await);
    assert_eq!((editing[0].2, editing[0].3.as_deref().is_some_and(|reason| reason.starts_with("Blocked")), &editing[1]), (true, true, &enabled("historyEditWithdraw", "Withdraw")), "the withdrawn draft blocks Edit; Withdraw stays the editor's own: {editing:?}");
    let other = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(other.iter().all(|action| action.2 && action.3.as_deref().is_some_and(|reason| reason.starts_with("Blocked"))), "another row names the open draft: {other:?}");
    let blocked = verb(&mut app, &fixture, "historyEditWithdraw", vec![("mutationId".into(), DslValue::String(first.clone()))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&blocked), Some("timeTravel.blocked"), "the verb refuses what the row disables");

    verb(&mut app, &fixture, "historyEditAccept", Vec::new(), mounted_policy, &mut identity).await;
    assert_eq!(app.time_travel.session().stage, TimeTravelStage::Replaying);
    let replaying = row_actions(&seeded_mutation_node(&mut app, 0).await);
    assert!(replaying.iter().all(|action| action.2 && action.3.as_deref() == Some("Not possible right now")), "a running replay disables Edit and Withdraw, naming why: {replaying:?}");
    pump_until(&mut app, "the replay completes", |app| app.time_travel.session().stage != TimeTravelStage::Replaying, mounted_policy, &mut identity,cx).await;
    let status = app.time_travel.status().expect("a session is open");
    assert_eq!((status.accepted_count, status.blocking, status.next_problem.is_none()), (1, false, true), "the withdrawal replays clean");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 3).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditRestore", "Restore")], "an accepted draft offers Restore in Withdraw's place");
    assert_eq!(row_actions(&seeded_mutation_node(&mut app, 0).await), vec![enabled("historyEditBegin", "Edit"), enabled("historyEditWithdraw", "Withdraw")], "the review withdraws other rows again");
    let wire = seeded_mutation_row(&mut app, 3).await;
    assert!(wire.withdrawn && !wire.withdrawable && wire.edited, "the replayed row reads withdrawn and is no longer withdrawable");
    let unaccepted = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(first.clone()))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&unaccepted), Some("timeTravel.illegal"), "a mutation without an accepted draft has nothing to restore");

    let restored = verb(&mut app, &fixture, "historyEditRestore", vec![("mutationId".into(), DslValue::String(target.clone()))], mounted_policy, &mut identity).await;
    assert_eq!(rejected(&restored), None, "{:?}", restored.output);
    assert!(app.time_travel.status().is_none(), "restoring the only accepted draft leaves history editing");
    assert_eq!((render_body(&mut app).await.contains(text(&fixture["committedBody"])), app.stores.store.generation()), (true, generation), "zero trace: the committed document, the store untouched");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🫥️ A payload schema whose one input is hidden: a draft editor would hold no row for it (design §22.20).
const HIDDEN_ONLY_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["value"],"properties":{"value":{"type":"string","x-semio-ui":{"widget":"hidden","label":{"en":"Label","de":"Beschriftung"}}}}}"#;

/// 🪵️ A document operation over the toy kinds with two kinds an editor cannot repair — the label kind, whose payload
/// schema hides its only input and which refuses to apply on a negative count, and the children kind, which declares no
/// input schema and has no inverse under a count below -1: the smallest operations an upstream edit breaks without inputs
/// to edit.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
struct InertLabelOp(TestMutation);

impl semio_framework_value::ToValue for InertLabelOp {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, semio_framework_value::ValueError> {
        semio_framework_value::ToValue::to_value_controlled(&self.0, control)
    }
    fn to_value(&self) -> DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}

impl semio_framework_value::FromValue for InertLabelOp {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value_controlled(value, control).map(Self)
    }
    fn retire_decoded(self) { <TestMutation as semio_framework_value::FromValue>::retire_decoded(self.0) }
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <TestMutation as semio_framework_value::FromValue>::from_value(value).map(Self)
    }
}

impl ::protocol::OpBinary for InertLabelOp {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::protocol::OpBinary::encode_op(&self.0)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        <TestMutation as ::protocol::OpBinary>::decode_op(bytes).map(Self)
    }
}

impl ::protocol::OpText for InertLabelOp {
    fn print_op(&self) -> String {
        ::protocol::OpText::print_op(&self.0)
    }

    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        <TestMutation as ::protocol::OpText>::parse_op(line).map(Self)
    }
}

impl store::Mutation<TestSnapshot> for InertLabelOp {
    type Diff = <TestMutation as store::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as store::Mutation<TestSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        store::Mutation::<TestSnapshot>::descriptor(&self.0)
    }

    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::Diff> {
        match &self.0 {
            TestMutation::SetLabel(_) if base.count < 0 => ::protocol::MutationOutcome::error("mutation.target-mismatch", "a label needs a count that is not negative", ["count"]),
            operation => store::Mutation::<TestSnapshot>::diff(operation, base),
        }
    }

    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if matches!(self.0, TestMutation::SetSlotChildren(_)) && base.count < -1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "children have no inverse under a count below -1"));
        }
        Ok(store::Mutation::<TestSnapshot>::inverse(&self.0, base)?.into_iter().map(Self).collect())
    }

    fn conflict_target(&self) -> Vec<String> {
        store::Mutation::<TestSnapshot>::conflict_target(&self.0)
    }

    fn may_emit_foreign_steps(&self) -> bool {
        false
    }

    fn input_schema(&self) -> Option<&'static str> {
        match &self.0 {
            TestMutation::SetLabel(_) => Some(HIDDEN_ONLY_SCHEMA),
            TestMutation::SetSlotChildren(_) => None,
            operation => store::Mutation::<TestSnapshot>::input_schema(operation),
        }
    }

    fn payload_value(&self) -> DslValue {
        store::Mutation::<TestSnapshot>::payload_value(&self.0)
    }

    fn with_payload_value(&self, value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        store::Mutation::<TestSnapshot>::with_payload_value(&self.0, value).map(Self)
    }
}

/// ⏭️ Replays `drafts` from `from` on `document` to completion through its history-edit owners and answers the report.
async fn replayed_report(owners: &mut time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, document: &mut ArtifactStore<TestSnapshot, InertLabelOp>, drafts: &protocol::HistoryInputDrafts, from: &MutationId) -> protocol::ReplayReport {
    let started = owners.run(document, time_travel::TimeTravelStoreCommand::StartReplay { drafts: drafts.clone(), from: from.clone() }).await.expect("the replay starts");
    assert!(matches!(started, time_travel::TimeTravelStoreOutput::ReplayStarted(true)), "the store starts the Report replay");
    for _ in 0..65_536 {
        match owners.run(document, time_travel::TimeTravelStoreCommand::StepReplay { deadline_us: u64::MAX, clock: semio_framework_job::default_now_us }).await.expect("a replay slice") {
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Completed(report)) => return report,
            time_travel::TimeTravelStoreOutput::Stepped(time_travel::TimeTravelReplayStep::Pending { .. }) => {}
            _ => panic!("the replay faulted"),
        }
    }
    panic!("the replay never completed");
}

/// 🌰️ The schema id of the [`InertLabelOp`] documents.
const INERT_SCHEMA: &str = "semio.test.inert-label/v1";

/// 🥊️ A document store over [`InertLabelOp`] holding count 1, label a, no children and count 5 — one edit each, authored
/// by `local` — with the mutation ids of those four operations.
async fn inert_document(mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (ArtifactStore<TestSnapshot, InertLabelOp>, Vec<MutationId>) {
    let genesis = store::os_store::document_materialize::create_document_envelope::<TestSnapshot, InertLabelOp>(INERT_SCHEMA, "inert-label", TestSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut document = Box::pin(ArtifactStore::new(genesis, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("the document store");
    document.install_operation_wire(store::ArtifactOperationWire::canonical());
    let (mut catalog, receipt)=bounded_document_store_owners::<TestSnapshot, InertLabelOp>(mounted_policy.preparation).unwrap_or_else(|_|panic!("original inert catalog admission")); let mut catalog_receipts=vec![(mounted_policy.preparation,receipt)]; while !catalog.constructor_is_complete(){let receipt=catalog.admit_constructor(mounted_policy.preparation).unwrap_or_else(|_|panic!("original inert catalog construction")); assert!(receipt.fits(mounted_policy.preparation)); catalog_receipts.push((mounted_policy.preparation,receipt));} assert!(catalog_receipts.iter().all(|(grant,receipt)|receipt.fits(*grant))); document.install_document_store_owners_exact(catalog);
    assert_eq!(document.local_actor_id(), &protocol::ActorId("local".into()));
    for operation in [TestMutation::SetCount(SetCount { value: 1 }), TestMutation::SetLabel(SetLabel { value: "a".into() }), TestMutation::SetSlotChildren(SetSlotChildren { children: Vec::new() }), TestMutation::SetCount(SetCount { value: 5 })] {
        Box::pin(document.dispatch(ArtifactCommand::Apply { mutations: vec![InertLabelOp(operation)], transaction: None }, identity)).await.expect("a seed edit applies");
    }
    let ids = document.mutation_ops().expect("applied operations").iter().map(|op| op.mutation_id.clone()).collect();
    (document, ids)
}

/// 🪤️ Settles and retires the history-edit owners of an [`inert_document`], then closes it under its exact grant.
fn close_inert(mut owners: time_travel::TimeTravelStoreState<TestSnapshot, InertLabelOp>, mut document: ArtifactStore<TestSnapshot, InertLabelOp>, mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) {
    use time_travel::TimeTravelOwners;
    owners.settle(&document, TimeTravelStage::Inactive).expect("the owners settle");
    for _ in 0..65_536 {
        let grant=mounted_policy.close;
        if owners.retire_step(grant).expect("the owners retire").inspect(|step|assert!(step.progress().fits(grant))).is_none() {
            break;
        }
    }
    assert!(owners.terminal_is_empty(), "the history-edit owners retire everything they held");
    for _ in 0..65_536 {
        let grant=mounted_policy.close;
        let step=document.close_owned_step(grant).expect("the document store closes under its exact grant");assert!(step.progress().fits(grant));
        match step {
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress==Default::default()=>panic!("the document store has no external owner"),
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(_) => {}
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => {
                assert!(document.close_owned_terminal_is_empty());
                return;
            }
        }
    }
    panic!("the document store did not close");
}

/// ⚖️ LAW (design §22.1, §22.20): a blocking mutation without editable inputs is always resolvable. A mutation whose
/// payload schema hides every input, and one that declares no input schema, are not editable and still withdrawable (never
/// for a viewer). An upstream edit makes the hidden-only one fail; opening it for editing is refused
/// `timeTravel.not-editable` (an editor with zero rows never opens); opening it to withdraw it is admitted by the store's
/// supersede law on an editor without inputs; the replay with the withdrawal no longer blocks and finalizes as one
/// overwrite whose head holds the edit and skips the withdrawn operation.
#[semio_framework_async_macros::async_test]
async fn a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready() {
    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    use time_travel::{TimeTravelActionRefusal, TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let schema = INERT_SCHEMA;
    let (mut document, ids) = inert_document(mounted_policy, &mut identity).await;
    assert!(semio_framework::mutation_input_defs(HIDDEN_ONLY_SCHEMA, &semio_framework::registered_input_schema_document).is_ok_and(|inputs| inputs.len() == 1 && inputs[0].presentation == Some(semio_framework::ArgPresentation::Hidden)), "the hidden-only schema reads as one hidden input");
    let (count, label, children) = (ids[0].clone(), ids[1].clone(), ids[2].clone());
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let row = |index: usize, viewer: bool| {
            let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[index], None, &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), viewer, None);
            (view.editable, view.withdrawable)
        };
        assert_eq!((row(0, false), row(1, false), row(2, false)), ((true, true), (false, true), (false, true)), "hidden-only inputs and no input schema: not editable, still withdrawable");
        assert_eq!((row(0, true), row(1, true), row(2, true)), ((false, false), (false, false), (false, false)), "a viewer edits and withdraws nothing");
    }

    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let negative = protocol::InputReplacement::Input { schema: schema.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -1 }))).expect("the edited count encodes") };
    let mut drafts = protocol::HistoryInputDrafts::from([(count.clone(), negative)]);
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let problem = blocked.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.clone());
    assert_eq!((blocked.blocks_finalize(), problem), (true, Some(label.clone())), "the edited count breaks the label operation downstream: {blocked:?}");

    for (target, why) in [(&label, "an editor with zero rows never opens"), (&children, "an operation without an input schema has no editor")] {
        let refused = owners.run(&mut document, TimeTravelStoreCommand::Open { target: target.clone(), current: None, withdraw: false }).await.expect("the open runs");
        assert!(matches!(refused, TimeTravelStoreOutput::Opened(Err(TimeTravelActionRefusal::NotEditable))), "{why}");
    }
    let TimeTravelStoreOutput::Opened(Ok((editor, original))) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: label.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing an operation without editable inputs");
    };
    assert!(editor.inputs.is_empty() && editor.inputs_refused.is_none() && editor.target == label, "its editor holds no inputs and names no schema fault");
    assert!(matches!(original, protocol::InputReplacement::Input { .. }), "its original input is its recorded operation");
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");

    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize() && ready.outcomes.iter().any(|outcome| outcome.mutation_id == label && outcome.withdrawn), "withdrawing the failing operation leaves a report that finalizes: {ready:?}");
    let committed = owners.commit_finished(&mut document, drafts.clone(), store::HistoryFinalization::Overwrite, None, &mut identity).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelCommit::Finalized { .. }), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operation");

    close_inert(owners, document,mounted_policy);
}

/// ⚖️ LAW (design §22.5, §22.17): one inverse failure is one mutation's fatal, never a faulted session. An upstream edit
/// under which a downstream operation has no inverse leaves the Report replay complete: that operation's outcome is one
/// `Fatal` `mutation.inverse-refused`, the operation after it is still replayed and applies, and the report blocks
/// finalizing. Its row says so in words — "Fatal: Cannot be reversed" / "Kritisch: Nicht umkehrbar", never the code — and
/// is the way out: withdrawing it, with the other operation the same edit broke, leaves a report that finalizes as one
/// overwrite.
#[semio_framework_async_macros::async_test]
async fn an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves() {
    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    use time_travel::{TimeTravelCommit, TimeTravelStoreCommand, TimeTravelStoreOutput, TimeTravelStoreState};
    let (mut document, ids) = inert_document(mounted_policy, &mut identity).await;
    let (count, label, children, tail) = (ids[0].clone(), ids[1].clone(), ids[2].clone(), ids[3].clone());
    let mut owners = TimeTravelStoreState::<TestSnapshot, InertLabelOp>::new(|_| LocalizedLabel::data("operation"));
    let lower = protocol::InputReplacement::Input { schema: INERT_SCHEMA.to_string(), payload: ::protocol::OpBinary::encode_op(&InertLabelOp(TestMutation::SetCount(SetCount { value: -2 }))).expect("the edited count encodes") };
    let mut drafts = protocol::HistoryInputDrafts::from([(count.clone(), lower)]);
    let outcome_of = |report: &protocol::ReplayReport, id: &MutationId| report.outcomes.iter().find(|outcome| outcome.mutation_id == *id).cloned().expect("an outcome per replayed operation");
    let blocked = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    let refused = outcome_of(&blocked, &children);
    assert_eq!((refused.worst, refused.messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>()), (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.inverse-refused"]), "the refused inverse is one fatal on its mutation: {blocked:?}");
    assert!(blocked.blocks_finalize() && outcome_of(&blocked, &tail).worst.is_none(), "the replay went on past the refusal and the report blocks: {blocked:?}");
    {
        let ops = document.mutation_ops().expect("applied operations");
        let unlabelled = HashMap::new();
        let view = time_travel::history_mutation_view_of::<TestSnapshot, InertLabelOp>(&ops[2], Some(&refused), &unlabelled, &|_: &InertLabelOp| LocalizedLabel::data("operation"), false, None);
        let row = time_travel::history_mutation_entry(&view, None);
        assert_eq!((time_travel::history_mutation_description(&row, Locale::En), time_travel::history_mutation_description(&row, Locale::De)), ("Fatal: Cannot be reversed".to_string(), "Kritisch: Nicht umkehrbar".to_string()), "the row names the refusal in words, never the code");
        assert!(view.withdrawable && !view.editable, "the row offers the way out");
    }

    let TimeTravelStoreOutput::Opened(Ok(_)) = owners.run(&mut document, TimeTravelStoreCommand::Open { target: children.clone(), current: Some(protocol::InputReplacement::Withdrawn), withdraw: true }).await.expect("the open runs") else {
        panic!("the store's supersede law admits withdrawing the operation without an inverse");
    };
    owners.run(&mut document, TimeTravelStoreCommand::Adopt(true)).await.expect("the session adopts the kind");
    drafts.insert(children.clone(), protocol::InputReplacement::Withdrawn);
    let half = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(half.blocks_finalize() && outcome_of(&half, &children).withdrawn && outcome_of(&half, &children).worst.is_none(), "withdrawn, the refusal is gone and the other broken operation still blocks: {half:?}");
    drafts.insert(label.clone(), protocol::InputReplacement::Withdrawn);
    let ready = replayed_report(&mut owners, &mut document, &drafts, &count).await;
    assert!(!ready.blocks_finalize(), "every broken operation withdrawn, the report finalizes: {ready:?}");
    let committed = owners.commit_finished(&mut document, drafts.clone(), store::HistoryFinalization::Overwrite, None, &mut identity).await.expect("the commit runs");
    assert!(matches!(committed, TimeTravelCommit::Finalized { .. }), "the finished replay commits as one overwrite");
    let head = document.snapshot().expect("head");
    assert_eq!((head.count, head.label.as_str()), (5, ""), "the head holds the edit and skips the withdrawn operations");
    close_inert(owners, document,mounted_policy);
}

/// ⚖️ LAW (design §22.6): the user-input path answers, it never panics. Every history-edit verb with missing, unknown
/// and wrongly typed arguments is answered — a result or a fault with a code — without a session, where the stage, the
/// document and its revision stay what they were, and inside one that edits a draft, where the session stays what it was.
#[semio_framework_async_macros::async_test]
async fn hostile_history_edit_input_is_answered_never_panicked_on() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let text_arg = |key: &str, value: &str| (key.to_string(), DslValue::String(value.to_string()));
    let garbage = || vec![("mutationId".to_string(), DslValue::uint(7)), ("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::Null), ("generation".to_string(), DslValue::String("x".into())), text_arg("choice", "sideways"), text_arg("name", ""), text_arg("store", "no-such-store")];
    let answered = |action: &str, answer: Result<InvocationResult, Fault>| {
        if let Err(fault) = answer {
            assert!(!fault.code.0.is_empty(), "{action}: a refusal carries its code: {fault:?}");
        }
    };

    let before = (head(&app), app.stores.store.content_revision_now());
    for action in semio_framework::history_edit::HISTORY_EDIT_ACTION_IDS {
        for args in [Vec::new(), vec![text_arg("mutationId", "no-such-mutation")], vec![("mutationId".to_string(), DslValue::uint(7))], vec![text_arg("mutationId", "no-such-mutation"), text_arg("store", "no-such-store")], garbage()] {
            let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture), &mut identity).await;
            answered(action, answer);
            assert_eq!((app.time_travel.session().stage, head(&app), app.stores.store.content_revision_now()), (TimeTravelStage::Inactive, before.0.clone(), before.1), "{action} {args:?}: nothing opened and nothing changed");
        }
    }

    for step in [serde_json::json!({ "begin": 1 }), serde_json::json!({ "input": { "path": "/value", "value": "b" } })] {
        let result = run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await.expect("a verb result");
        assert_eq!(rejected(&result), None, "{step}: {:?}", result.output);
    }
    let session = app.time_travel.session().clone();
    let far = u64::from(session.generation) + 4_000_000;
    let hostile: Vec<(&str, Vec<(String, DslValue)>)> = vec![
        ("historyEditBegin", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditBegin", vec![("mutationId".to_string(), DslValue::uint(7))]),
        ("historyEditBegin", garbage()),
        ("historyEditWithdraw", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditRestore", Vec::new()),
        ("historyEditRestore", vec![text_arg("mutationId", "no-such-mutation")]),
        ("historyEditInput", Vec::new()),
        ("historyEditInput", vec![text_arg("path", "/no/such/input"), ("value".to_string(), DslValue::Null)]),
        ("historyEditInput", vec![("path".to_string(), DslValue::uint(3)), ("value".to_string(), DslValue::uint(3))]),
        ("historyEditInput", vec![text_arg("path", "/value"), text_arg("value", "c"), ("generation".to_string(), DslValue::uint(far))]),
        ("historyEditInput", garbage()),
        ("historyEditCommit", vec![text_arg("choice", "sideways")]),
        ("historyEditFinalize", Vec::new()),
        ("historyEditRerun", Vec::new()),
        ("historyEditCancelReplay", Vec::new()),
    ];
    for (action, args) in hostile {
        let answer = app.handle_action(action, Some(&DslValue::Object(args.clone())), &meta(&fixture), &mut identity).await;
        answered(action, answer);
        assert_eq!(app.time_travel.session(), &session, "{action} {args:?}: the session is what it was");
    }
    assert!(render_body(&mut app).await.contains("count=1 label=b"), "the draft preview stands");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🚫️RowWithdraw

//#region 🖐️GestureSlot
#[path = "../🧪️gesture/🦀️.rs"]
mod gesture_laws;
//#endregion 🖐️GestureSlot

#[path = "../🧪️folder-reload-route/🦀️.rs"]
mod folder_reload_route;