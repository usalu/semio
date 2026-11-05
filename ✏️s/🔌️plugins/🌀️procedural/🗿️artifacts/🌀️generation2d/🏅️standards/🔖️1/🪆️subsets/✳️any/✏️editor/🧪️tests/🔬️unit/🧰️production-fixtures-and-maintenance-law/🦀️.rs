/// 🧰️ Production fixtures of the generation2d editor routes: the seeded snapshot, mutations, envelope helpers and the non-empty retained maintenance swap law.

fn production_initial_snapshot(label: &str) -> Generation2dSnapshot {
    let mut snapshot = Generation2dSnapshot::default();
    snapshot.host_snapshot.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.host_snapshot.widgets.push(Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.host_snapshot.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.host_snapshot.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation2dMutation> {
    use semio_s_artifact_procedural_2d_mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        create_widget(0, Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
        replace_widget(Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() }),
        delete_widget("delete-target".into()),
        connect_synapse(0, semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() }),
        replace_synapse(semio_framework_artifact_flow_flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() }),
        disconnect_synapse("disconnect-synapse".into()),
        move_widget("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 }),
        clear_widget_layout("clear-target".into()),
        update_camera(semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 }),
        change_schema("flow.host_snapshot.production-retained".into()),
        create_generation(semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() }),
        delete_generation("delete-generation".into()),
        rename_generation("rename-generation".into(), "After Rename".into()),
        change_generation_value("change-generation".into(), "deep-answer".into(), serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into()),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P2 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

/// 📸️ Every owned production projection this law holds is a CLOSING read — `fixture.layout` is a
/// live `OrderedMap` root that aborts the process on a bare drop.
fn production_read(snapshot: Generation2dSnapshot) -> semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead {
    semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead::new(snapshot)
}

fn production_semantic_digest(snapshot: &Generation2dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation2d.production-law.semantic");
    digest.observe(&semio_s_artifact_procedural_2d_io_binary::snapshot::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation2dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P2 mutation variant including clear-widget-layout");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P2 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&semio_s_artifact_procedural_2d_io_binary::mutations::encode_op(mutation).expect("P2 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    semio_s_artifact_procedural_2d_mutations::central_apply::generation2d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    semio_s_artifact_procedural_2d_mutations::central_apply::generation2d_retire_mutations_cold(mutations);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_2D_SCHEMA,
        "id": "generation2d-production-mounted-law",
        "vcs": {
            "initialPack": production_hex(&semio_s_artifact_procedural_2d_io_binary::snapshot::encode(&snapshot)),
            "edits": [{
                "id": "generation2d-production-all14-edit",
                "actor": "generation2d-production-law",
                "forwards": mutation_hex,
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "1",
                "line": null
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first P2 production fixture envelope");
    snapshot.retire_cold();
    (wire, expected, expected_digest)
}

fn admit_production_envelope(app: &mut Generation2dApp, wire: &[u8]) -> semio_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P2 production ingress credits");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P2 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P2 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.0.seal_artifact_envelope_ingress(handle, app.1.cx()).expect("P2 production envelope seal"));
    handle
}

fn drive_production_envelope(app: &mut Generation2dApp, handle: semio_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("one P2 production maintenance turn");
        let poll = app.0.advance_artifact_envelope_load(handle, app.1.cx()).expect("P2 production load advancement");
        if matches!(poll, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P2 production envelope load did not reach terminal");
}

/// 🔐️ LAW: non-empty P2D2 canonical ingress reaches the real VCS maintenance replacement,
/// and the accepted and displaced stores remain owned until explicit terminal ACK/close.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_non_empty_retained_maintenance_swap_publishes_through_the_token() {
    let mut accepted = app_with_registry().await;
    let base_generation = accepted.artifact_generation_now();
    let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
    let handle = admit_production_envelope(&mut accepted, &wire);
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = production_read(accepted.snapshot().expect("accepted P2 production snapshot"));
    let expected = production_read(expected);
    assert_eq!(&*snapshot, &*expected, "real maintenance must publish all P2 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.host_snapshot.layout.contains_key("move-target"));
    assert!(!snapshot.host_snapshot.layout.contains_key("clear-target"), "2D-only clear-widget-layout must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P2 terminal ACK"));
    close(accepted);
}

/// 🔐️ LAW: two replacements admitted on the same live generation cannot both publish through the real VCS maintenance swap; the loser faults, the published store and generation stay the winner's, and its candidate is retired by terminal ACK.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_maintenance_swap_rejects_the_stale_replacement_and_keeps_the_published_store() {
    let mut app = app_with_registry().await;
    let base_generation = app.artifact_generation_now();
    let (first_wire, first_candidate, first_digest) = production_envelope_wire("stale-race-first");
    let (second_wire, second_candidate, second_digest) = production_envelope_wire("stale-race-second");
    first_candidate.retire_cold();
    second_candidate.retire_cold();
    let first = admit_production_envelope(&mut app, &first_wire);
    let second = admit_production_envelope(&mut app, &second_wire);
    let polls = [drive_production_envelope(&mut app, first), drive_production_envelope(&mut app, second)];
    let ready = semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready;
    let fault = semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault;
    assert_eq!(polls.iter().filter(|poll| **poll == ready).count(), 1, "exactly one racing replacement publishes: {polls:?}");
    assert_eq!(polls.iter().filter(|poll| **poll == fault).count(), 1, "the stale replacement faults: {polls:?}");
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1, "the rejected replacement does not advance the live generation");
    let winner_digest = if polls[0] == ready { first_digest } else { second_digest };
    let published = production_read(app.snapshot().expect("published P2 snapshot after the rejected race"));
    assert_eq!(production_semantic_digest(&published), winner_digest, "the live store is the winner's, never the rejected candidate");
    assert!(app.acknowledge_artifact_store_replacement(first).expect("first P2 terminal ACK"));
    assert!(app.acknowledge_artifact_store_replacement(second).expect("second P2 terminal ACK after candidate retirement"));
    close(app);
}
