//! 🏭️ Process3d editor unit tests: the production envelope helpers and the maintenance swap law.

use super::*;

fn production_initial_snapshot(label: &str) -> Process3dSnapshot {
    let mut snapshot = semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot();
    snapshot.stock_label = label.into();
    snapshot.workshop.machines.push(WorkshopMachine { id: "machine".into(), label: "Original Machine".into(), icon_id: "original-tool".into(), catalog_id: Some("original-catalog".into()), capabilities: Vec::new() });
    snapshot
}

fn production_semantic_digest(snapshot: &Process3dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"process3d.production-law.semantic");
    digest.observe(&snapshot.encode_pack());
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Process3dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mutations = semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::process3d_all_retained_mutation_fixtures_for_test();
    assert_eq!(mutations.len(), semio_s_artifact_process_process3d_mutations::KINDS.len(), "production ingress carries every Process3d mutation variant");
    let mutation_hex: Vec<String> = mutations.iter().map(|mutation| semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::encode_op(mutation).expect("deep Process3d mutation encoding").iter().map(|byte| format!("{byte:02x}")).collect()).collect();
    let mut expected = production_initial_snapshot(label);
    let (expected_machine_id, expected_machine_label) = match &mutations[9] {
        Process3dMutation::RenameMachine(value) => (value.id.clone(), value.new_label.clone()),
        _ => unreachable!("fixed all-variant fixture order"),
    };
    let expected_machine_icon = match &mutations[10] {
        Process3dMutation::ChangeMachineIcon(value) => value.new_icon_id.clone(),
        _ => unreachable!("fixed all-variant fixture order"),
    };
    let expected_capability = match &mutations[11] {
        Process3dMutation::ReplaceMachineCapabilities(value) => value.new_capabilities[0].clone(),
        _ => unreachable!("fixed all-variant fixture order"),
    };
    let machine = expected.workshop.machines.iter_mut().find(|machine| machine.id == expected_machine_id).expect("production law renames a machine the initial workshop declares");
    machine.label = expected_machine_label;
    machine.icon_id = expected_machine_icon;
    machine.capabilities = vec![expected_capability];
    if let Process3dMutation::MoveStock(value) = &mutations[12] {
        expected.stock_pose = value.new_pose.clone();
    }
    if let Process3dMutation::ChangeStockLabel(value) = &mutations[13] {
        expected.stock_label = value.new_label.clone();
    }
    if let Process3dMutation::ReplaceStockSolid(value) = &mutations[14] {
        expected.stock_solid = value.new_solid.clone();
    }
    let mut step = match &mutations[0] {
        Process3dMutation::CreateStep(value) => value.step.clone(),
        _ => unreachable!("fixed timeline fixture order"),
    };
    if let Process3dMutation::RenameStep(value) = &mutations[2] { step.label = value.new_label.clone(); }
    if let Process3dMutation::ChangeStepEnabled(value) = &mutations[3] { step.enabled = value.new_enabled; }
    if let Process3dMutation::ChangeStepOrigin(value) = &mutations[4] { step.origin = value.new_origin.clone(); }
    if let Process3dMutation::ReplaceStepMeasure(value) = &mutations[5] { step.measure = value.new_measure.clone(); }
    let solid = match &step.measure {
        ProcessMeasure::Attach { component, .. } => component,
        _ => unreachable!("production timeline ends with the authored attach measure"),
    };
    let tool = semio_s_artifact_process_process3d_model::snapshot::working_solid_child_handle(&format!("tool-{}", step.id), solid);
    let tool_ids = std::collections::BTreeMap::from([(step.id.clone(), tool.child_id.clone())]);
    expected.steps = semio_s_artifact_process_process3d_model::snapshot::flow_child_handle(&semio_s_artifact_process_process3d_model::snapshot::flow_snapshot_for_steps(std::slice::from_ref(&step), &tool_ids));
    expected.step_payloads = vec![step];
    expected.tool_solids = vec![tool];
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA,
        "id": "process3d-production-mounted-law",
        "vcs": {
            "initialPack": snapshot_hex,
            "edits": [{
                "id": "process3d-production-deep-edit",
                "actor": "process3d-production-law",
                "line": null,
                "forwards": mutation_hex,
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "1"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first Process3d production fixture envelope");
    let envelope = store::os_store::document_materialize::create_document_envelope(semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA, "process3d-production-mounted-law", snapshot, None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let owners = store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Process3dSnapshot, Process3dMutation>().expect("Process3d fixture funds its bounded catalog");
    let funded = semio_plugin::plugin_self_funded_grant(owners.uninstalled_envelope_retirement_demands(&envelope), semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
    let (mut retirement, _) = owners.retire_envelope_uninstalled(envelope, funded).map_err(|(error, _, _)| error).expect("Process3d fixture envelope retirement admission");
    let mut caller = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let page_bytes = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    for _ in 0..100_000 {
        let demand = retirement.next_demand(page_bytes).expect("Process3d fixture envelope retirement demand");
        let grant = semio_plugin::plugin_self_funded_grant(demand, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
        let mut complete = false;
        caller.cx().with_retained_loan(grant, |cx| {
            semio_framework_job::close_original_effect_step(cx, demand, |effect| {
                let poll = retirement.close_step(page_bytes, effect);
                use semio_framework_value::{RetainedCloneStep, RetainedWorkPoll};
                complete = matches!(&poll, RetainedWorkPoll::Complete(turn) if matches!(&turn.result, Ok(RetainedCloneStep::Complete(_))));
                poll
            })
        }).expect("Process3d fixture envelope retirement");
        if complete {
            assert!(retirement.terminal_is_empty());
            drop(retirement);
            return (wire, expected, expected_digest);
        }
    }
    panic!("Process3d fixture envelope retirement did not reach terminal")
}

fn admit_production_envelope(app: &mut super::context::Process3dApp, wire: &[u8]) -> semio_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Process3d production ingress credits");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Process3d production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Process3d production envelope page admission failed: {fault:?}"));
    }
    assert!(app.0.seal_artifact_envelope_ingress(handle, app.1.cx()).expect("Process3d production envelope seal"));
    handle
}

async fn drive_production_envelope(app: &mut super::context::Process3dApp, handle: semio_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_plugin::ArtifactEnvelopeDecodeOperationPoll {
    use semio_plugin::artifact_app_laws::{OriginalFixtureCaller, fixture_identity, fixture_renew_turn};
    use semio_plugin::ArtifactEnvelopeDecodeOperationPoll::{Pending, Ready, Cancelled, Fault};
    let mut identity = fixture_identity();
    let fuel = OriginalFixtureCaller::turn_fuel();
    let grant = app.1.cx().retained_grant();
    let mut poll = Pending;
    for _ in 0..OriginalFixtureCaller::maximum_turns() {
        fixture_renew_turn(app.1.cx(), fuel, grant).expect("Process3d production caller renewal");
        PluginApp::advance_typed_operation_publication(&mut app.0, &mut identity, app.1.cx()).await.expect("Process3d production publication turn");
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("one Process3d production maintenance turn");
        poll = app.0.advance_artifact_envelope_load(handle, app.1.cx()).expect("Process3d production load advancement");
        if matches!(poll, Ready | Cancelled | Fault) {
            return poll;
        }
        semio_framework_async::yield_once().await;
    }
    panic!("Process3d production envelope load did not reach terminal ({poll:?}): {}", PluginApp::close_diagnostic(&app.0))
}

/// 🔐️ LAW: the real `VcsArtifactApp` maintenance branch publishes the complete deep Process3d document through the framework-minted publication token.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_production_maintenance_swap_publishes_the_deep_document() {
    let accepted_label = "accepted-production-swap";
    let mut accepted = super::context::unseeded_app_with_registry();
    let base_generation = accepted.artifact_generation_now();
    let (accepted_wire, expected_snapshot, expected_digest) = production_envelope_wire(accepted_label);
    let accepted_handle = admit_production_envelope(&mut accepted, &accepted_wire);
    assert_eq!(drive_production_envelope(&mut accepted, accepted_handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let accepted_snapshot = accepted.snapshot().expect("accepted Process3d production snapshot");
    assert_eq!(&accepted_snapshot, &expected_snapshot, "real maintenance replay must publish the complete deep semantic state");
    assert_eq!(production_semantic_digest(&accepted_snapshot), expected_digest, "real maintenance replay must publish the deterministic semantic digest");
    let fixtures = semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::process3d_all_retained_mutation_fixtures_for_test();
    let (renamed_id, renamed_label) = match &fixtures[9] {
        Process3dMutation::RenameMachine(value) => (value.id.clone(), value.new_label.clone()),
        _ => unreachable!("fixed all-variant fixture order"),
    };
    let renamed_icon = match &fixtures[10] {
        Process3dMutation::ChangeMachineIcon(value) => value.new_icon_id.clone(),
        _ => unreachable!("fixed all-variant fixture order"),
    };
    let machine = accepted_snapshot.workshop.machines.iter().find(|machine| machine.id == renamed_id).expect("deep production machine");
    assert_eq!((machine.label.as_str(), machine.icon_id.as_str()), (renamed_label.as_str(), renamed_icon.as_str()));
    let capability = machine.capabilities.first().expect("deep production capability");
    assert!(matches!(&capability.recipe, MeasureRecipe::BoxAttach { width, depth, height } if (width.as_str(), depth.as_str(), height.as_str()) == ("width", "depth", "height")));
    assert_eq!((capability.parameters.len(), capability.rules.len(), accepted_snapshot.stock_label.as_str()), (3, 2, "Beam"));
    assert!(accepted.acknowledge_artifact_store_replacement(accepted_handle).expect("accepted Process3d terminal ACK"));
}

/// 🧹️ Valid history arrays settle before an invalid genesis triggers terminal decode retirement.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_production_retires_history_before_faulting_on_genesis() {
    let wire = include_bytes!("../../🧫️fixtures/production-envelope-history-fault/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_slice(wire).expect("production history fault fixture JSON");
    let vcs = fixture["vcs"].as_object().expect("production history fault VCS object");
    for key in ["alternatives", "changes", "checkpoints", "edits"] {
        assert!(vcs[key].as_array().is_some_and(Vec::is_empty));
    }
    assert!(vcs.get("initialPack").is_some_and(serde_json::Value::is_null));

    let mut app = super::context::unseeded_app_with_registry();
    let base_generation = app.artifact_generation_now();
    let handle = admit_production_envelope(&mut app, wire);
    assert_eq!(drive_production_envelope(&mut app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert!(app.0.artifact_envelope_decode_worker_owners_are_terminal(handle));
    assert_eq!(app.artifact_generation_now(), base_generation);
}

#[test]
fn process3d_archived_mutation_wire_matches_its_native_receiver() {
    use ::semio_os_kernel::os_dsl::original_receiving::check_receiving_round_trip;
    use ::semio_os_kernel::OpBinary;
    use ::semio_framework_value::DslValue;

    let wire = include_str!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/❌delete-machine/➖️empties/🦠️mutation/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(wire).expect("Process3d archive operation fixture");
    let mutation: Process3dMutation = semio_framework_pack_json::from_json_str(wire, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Process3d archive operation");
    let encoded = mutation.encode_op().expect("persisted DeleteMachine payload");
    let packed = ::semio_os_kernel::pack_rt::decode_pack_value(&encoded).expect("one complete mutation Pack value");
    for field in ["mutation", "id"] {
        assert_eq!(packed.get(field).and_then(DslValue::as_str), fixture[field].as_str());
    }
    assert!(Process3dMutation::decode_op(&encoded[..encoded.len() - 1]).is_err());
    assert!(Process3dMutation::decode_op(&[2, 8, 3, b's', b'a', b'w']).is_err());

    let mut mutations = semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::process3d_all_retained_mutation_fixtures_for_test();
    mutations.push(mutation);
    check_receiving_round_trip(&mutations).expect("persisted operation bytes must be accepted by the original native operation receiver");
}
