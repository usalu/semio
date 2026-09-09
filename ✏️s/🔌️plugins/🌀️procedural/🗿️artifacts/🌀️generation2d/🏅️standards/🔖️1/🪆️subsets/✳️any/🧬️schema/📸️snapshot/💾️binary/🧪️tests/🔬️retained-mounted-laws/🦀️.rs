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

fn close(session: &mut Generation2dMountedPackSession) {
    session.request_cancel();
    for _ in 0..100_000 {
        if session.close_step(1, mounted::RETAINED_PACK_PAGE_BYTES).expect("P2 retained session close") {
            assert!(session.terminal_is_empty());
            return;
        }
    }
    panic!("P2 retained session did not reach terminal-empty close");
}

#[test]
fn non_empty_canonical_snapshot_round_trips_one_grant_at_a_time() {
    let mut expected = Generation2dSnapshot::default();
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
    let mut values: semio_framework_artifact_playbook_playbook::PlaybookValues = std::collections::HashMap::new();
    values.insert(
        "nested".into(),
        dsl::DslValue::object([("array".to_string(), dsl::DslValue::Array(vec![dsl::DslValue::Bool(true), dsl::DslValue::Null, dsl::DslValue::float(3.5)])), ("text".to_string(), dsl::DslValue::String("retained".to_string()))]),
    );
    expected.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: "retained-generation".into(), name: "Generation".into(), values });
    expected.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("retained-generation".into());
    expected.generation.cold_builder_mut().expect("unique cold generation owner").preview_text = Some("preview".into());
    assert!(!expected.fixture.widgets.is_empty());
    assert!(!expected.fixture.synapses.is_empty());
    let bytes = encode(&expected);
    assert_eq!(&bytes[..4], &GENERATION2D_MOUNTED_PREFIX);
    let expected_ledger = bytes[4..].iter().fold(0xcbf2_9ce4_8422_2325u64, |ledger, byte| (ledger ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3));
    let mut session = Generation2dMountedPackSession::new(bytes.len(), 8_192).expect("P2 retained snapshot preflight");
    for byte in bytes {
        session.admit_byte(byte).expect("one admitted snapshot byte");
    }
    assert_eq!(session.canonical_ingress_ledger(), expected_ledger, "bytes after P2D2 must be the unchanged canonical SPK stream");
    session.seal().expect("exact snapshot seal");
    let mut ready = false;
    for _ in 0..1_000_000 {
        if session.grant().expect("one retained snapshot grant") {
            ready = true;
            break;
        }
    }
    assert!(ready, "P2 retained canonical route must converge");
    let actual = session.take().expect("typed snapshot handoff");
    assert_eq!(actual.fixture.synapses.len(), expected.fixture.synapses.len(), "typed synapse owner must retain the exact row census");
    assert_eq!(actual.fixture.synapses.last(), expected.fixture.synapses.last(), "typed synapse owner must retain the exact non-empty appended row");
    assert_eq!(synapse_digest(actual.fixture.synapses.last().expect("typed retained synapse")), synapse_digest(expected.fixture.synapses.last().expect("expected retained synapse")));
    assert_eq!(actual, expected, "all typed snapshot owners must round-trip exactly");
    close(&mut session);
}

#[test]
fn p3d3_is_rejected_before_semantic_allocation() {
    let mut session = Generation2dMountedPackSession::new(8, 8).expect("P2 hostile discriminator preflight");
    session.admit_byte(b'P').expect("shared first discriminator byte");
    assert!(!session.semantic_allocated());
    assert_eq!(session.admit_byte(b'3'), Err(b'3'));
    assert!(!session.semantic_allocated());
    close(&mut session);
}
