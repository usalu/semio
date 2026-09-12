use super::*;

fn synapse_digest(synapse: &semio_framework_artifact_flow_flow::SynapseSpec) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for field in [&synapse.id, &synapse.from, &synapse.from_port, &synapse.to, &synapse.to_port] {
        digest ^= field.len() as u64;
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in field.as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}

fn admit(session: &mut Generation3dMountedPackSession, value: u8) {
    loop {
        if let Some(exact) = session.next_source_allocation_bytes().expect("P3 source allocation query") {
            let step = session.reserve_source_page(exact).expect("P3 source allocation grant");
            assert!(step.progressed);
            continue;
        }
        session.admit_byte(value).expect("one admitted snapshot byte");
        return;
    }
}

fn close(session: &mut Generation3dMountedPackSession) {
    session.request_cancel();
    let admitted_allocation_bytes = session.progress().map_or(0, |progress| progress.allocated_bytes);
    let mut retained_allocation_bytes = admitted_allocation_bytes;
    let mut released_allocation_bytes = 0;
    for _ in 0..100_000 {
        let maximum_bytes = session.next_source_release_allocation_bytes().unwrap_or(0);
        let step = session.close_step(1, maximum_bytes).expect("P3 retained session close");
        let next_retained_allocation_bytes = session.progress().map_or(0, |progress| progress.allocated_bytes);
        released_allocation_bytes += retained_allocation_bytes - next_retained_allocation_bytes;
        retained_allocation_bytes = next_retained_allocation_bytes;
        if step == Generation3dMountedPackCloseStep::Complete {
            assert!(session.terminal_is_empty());
            assert_eq!(released_allocation_bytes, admitted_allocation_bytes);
            return;
        }
    }
    panic!("P3 retained session did not reach terminal-empty close");
}

#[test]
fn non_empty_canonical_snapshot_round_trips_one_grant_at_a_time() {
    let mut expected = Generation3dSnapshot::default();
    let nested = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("enabled", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(true)));
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new()
        .insert("gain", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(2.5)))
        .insert("nested", semio_framework_artifact_flow_flow::neural::Value::Dictionary(nested));
    expected.fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::Neuron { id: "retained-neuron".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true });
    let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
    expanded.insert("answer".into());
    let preview = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("answer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("visible".into())));
    expected.fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::OutputPreview { id: "retained-preview".into(), preview, expanded });
    expected.fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::Cluster { id: "retained-cluster".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() });
    expected.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-neuron".into(), to: "retained-preview".into(), from_port: "out".into(), to_port: String::new() });
    expected.fixture.layout.insert("retained-neuron".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 12.5, y: -8.25 });
    expected.fixture.layout.insert("retained-preview".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 36.0, y: -8.25 });
    let mut values: semio_framework_artifact_playbook_playbook::PlaybookValues = std::collections::HashMap::new();
    values.insert(
        "nested".into(),
        dsl::DslValue::object([("array".to_string(), dsl::DslValue::Array(vec![dsl::DslValue::Bool(true), dsl::DslValue::Null, dsl::DslValue::float(3.5)])), ("text".to_string(), dsl::DslValue::String("retained".to_string()))]),
    );
    expected.generation = semio_framework_artifact_playbook_playbook::GenerationPlayState {
        generations: vec![semio_framework_artifact_playbook_playbook::FormGeneration { id: "retained-generation".into(), name: "Generation".into(), values }],
        selected_generation_id: Some("retained-generation".into()),
        preview_text: Some("preview".into()),
    }
    .into();
    assert!(!expected.fixture.widgets.is_empty());
    assert!(!expected.fixture.synapses.is_empty());
    assert!(!expected.fixture.layout.is_empty());
    let bytes = encode_mounted(&expected);
    assert_eq!(&bytes[..4], &GENERATION3D_MOUNTED_PREFIX);
    let expected_ledger = bytes[4..].iter().fold(0xcbf2_9ce4_8422_2325u64, |ledger, byte| (ledger ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3));
    let mut session = Generation3dMountedPackSession::new(bytes.len(), 8_192).expect("P3 retained snapshot preflight");
    for (index, byte) in bytes.into_iter().enumerate() {
        if index == GENERATION3D_MOUNTED_PREFIX.len() {
            let before = session.progress().expect("P3 source exists after discriminator");
            assert_eq!(session.admit_byte(byte), Err(byte));
            assert_eq!(session.progress(), Some(before));
        }
        admit(&mut session, byte);
    }
    assert_eq!(session.canonical_ingress_ledger(), expected_ledger, "bytes after P3D3 must be the unchanged canonical SPK stream");
    session.seal().expect("exact snapshot seal");
    let mut ready = false;
    for _ in 0..1_000_000 {
        if session.grant().expect("one retained snapshot grant") {
            ready = true;
            break;
        }
    }
    assert!(ready, "P3 retained canonical route must converge");
    let actual = session.take().expect("typed snapshot handoff");
    assert_eq!(actual.fixture.synapses.len(), expected.fixture.synapses.len(), "typed synapse owner must retain the exact row census");
    assert_eq!(actual.fixture.synapses.last(), expected.fixture.synapses.last(), "typed synapse owner must retain the exact non-empty appended row");
    assert_eq!(synapse_digest(actual.fixture.synapses.last().expect("typed retained synapse")), synapse_digest(expected.fixture.synapses.last().expect("expected retained synapse")));
    assert_eq!(actual.fixture.layout, expected.fixture.layout, "typed layout owner must retain the semantically attached widget positions");
    assert_eq!(actual, expected, "all typed snapshot owners must round-trip exactly");
    close(&mut session);
    actual.retire_cold();
    expected.retire_cold();
}

#[test]
fn p2d2_is_rejected_before_semantic_allocation() {
    let mut session = Generation3dMountedPackSession::new(8, 8).expect("P3 hostile discriminator preflight");
    session.admit_byte(b'P').expect("shared first discriminator byte");
    assert!(!session.semantic_allocated());
    assert_eq!(session.admit_byte(b'2'), Err(b'2'));
    assert!(!session.semantic_allocated());
    close(&mut session);
}
