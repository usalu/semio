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
    // 🪪️ Every workshop mutation addresses a machine by ID, so the expectation must locate the same
    // machine the replay does. It used to edit `machines.first_mut()` — the initial workshop already
    // carries the three generic machines (saw, drill, attacher) BEFORE the one this law appends, so a
    // positional expectation renamed `saw` while the real id-keyed replay renamed `machine`. The
    // label, icon and capability are read off the mutations themselves for the same reason.
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
    let envelope = store::os_store::document_materialize::create_document_envelope(semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA, "process3d-production-mounted-law", snapshot, None);
    let owners = store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Process3dSnapshot, Process3dMutation>().expect("Process3d fixture funds its bounded catalog");
    let funded = semio_s_artifact_process_process3d_host::process3d_demand_grant(owners.uninstalled_envelope_retirement_demands(&envelope));
    let (mut retirement, _) = owners.retire_envelope_uninstalled(envelope, funded).map_err(|(error, _, _)| error).expect("Process3d fixture envelope retirement admission");
    for _ in 0..100_000 {
        let demand = retirement.next_demand(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Process3d fixture envelope retirement demand");
        let step = retirement.close_step(semio_s_artifact_process_process3d_host::process3d_demand_grant(demand)).expect("Process3d fixture envelope retirement");
        if matches!(step, semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) && retirement.terminal_is_empty() {
            drop(retirement);
            return (wire, expected, expected_digest);
        }
    }
    panic!("Process3d fixture envelope retirement did not reach terminal")
}

fn admit_production_envelope(app: &mut crate::context::Process3dRawApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Process3d production ingress credits");
    semio_s_artifact_process_process3d_host::process3d_admit_publication_authority(
        handle.operation,
        handle.generation,
        handle.generation.0,
        handle.generation.0,
        handle.generation.0,
        semio_s_artifact_process_process3d_host::Process3dPublicationLimits { maximum_items: 8_192, maximum_output_pages: semio_s_artifact_process_process3d_host::PROCESS3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: semio_s_artifact_process_process3d_host::PROCESS3D_MOUNTED_CONTROL_CREDITS },
    )
    .expect("Process3d production publication authority");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Process3d production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Process3d production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Process3d production envelope seal"));
    handle
}

fn drive_production_envelope(app: &mut crate::context::Process3dRawApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..200_000 {
        semio_s_artifact_process_process3d_host::process3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0).expect("Process3d authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Process3d production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Process3d production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Process3d production envelope load did not reach terminal")
}

/// 🔐️ LAW: the real `VcsArtifactApp` maintenance branch cannot swap around Process3d's
/// atomic authority hook; accepted and every hostile lease retire their displaced/candidate owner.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed() {
    // 🛣️ The publication lease table is a process-global four-slot DIRECT-MAPPED registry shared by the
    // whole test binary; two laws admitting concurrently collide on a slot and report
    // `process3d-publication.saturated`, which is scheduling, never the property under test.
    let _lane = semio_s_artifact_process_process3d_host::process3d_publication_authority_lane();
    let accepted_label = "accepted-production-swap";
    let mut accepted = crate::context::unseeded_app_with_registry();
    let base_generation = accepted.artifact_generation_now();
    let (accepted_wire, expected_snapshot, expected_digest) = production_envelope_wire(accepted_label);
    let accepted_handle = admit_production_envelope(&mut accepted, &accepted_wire);
    assert_eq!(drive_production_envelope(&mut accepted, accepted_handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let accepted_snapshot = accepted.snapshot().expect("accepted Process3d production snapshot");
    assert_eq!(&accepted_snapshot, &expected_snapshot, "real maintenance replay must publish the complete deep semantic state");
    assert_eq!(production_semantic_digest(&accepted_snapshot), expected_digest, "real maintenance replay must publish the deterministic semantic digest");
    // 🪪️ The SAME id-keyed lookup `production_envelope_wire` builds its expectation with: the initial
    // workshop declares three generic machines (saw, drill, attacher) BEFORE the one this law renames,
    // so `machines.first()` reads `saw` — a machine the replay never touches — and the literal
    // `("machine", "Renamed Machine", "drill")` was a positional accident, not the published state.
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
    assert!(semio_s_artifact_process_process3d_host::process3d_release_publication_authority(accepted_handle.operation, accepted_handle.generation));

    use semio_s_artifact_process_process3d_host::{Process3dPublicationHostile::Missing, Process3dPublicationHostile::WrongBase, Process3dPublicationHostile::WrongGeneration, Process3dPublicationHostile::WrongOperation, Process3dPublicationHostile::WrongParent};
    for (hostile, expected_code) in [
        (Missing, "process3d-publication.authority-missing"),
        (WrongOperation, "process3d-publication.wrong-operation"),
        (WrongGeneration, "process3d-publication.wrong-generation"),
        (WrongBase, "process3d-publication.wrong-base"),
        (WrongParent, "process3d-publication.wrong-parent"),
    ] {
        let mut app = crate::context::unseeded_app_with_registry();
        let last_valid = app.snapshot().expect("last-valid Process3d snapshot");
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (hostile_wire, hostile_snapshot, hostile_digest) = production_envelope_wire("rejected-production-candidate");
        assert_eq!(production_semantic_digest(&hostile_snapshot), hostile_digest);
        let handle = admit_production_envelope(&mut app, &hostile_wire);
        semio_s_artifact_process_process3d_host::process3d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(semio_s_artifact_process_process3d_host::process3d_take_publication_hostile_observed(handle.operation), Some(expected_code), "removing or bypassing the real validator must fail this law");
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = app.snapshot().expect("last-valid snapshot after rejected candidate");
        assert_eq!(retained, last_valid);
        assert_eq!(production_semantic_digest(&retained), last_valid_digest, "hostile candidate must not change the last-valid digest");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected Process3d terminal ACK after candidate retirement"));
        assert!(semio_s_artifact_process_process3d_host::process3d_release_publication_authority(handle.operation, handle.generation));
    }
}
