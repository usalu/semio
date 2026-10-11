use super::*;
use crate::retirement_driver::{drive_erased_released, exact_grant, retire_owned_for_test, retire_shared_for_test, Stepped, STEP_GRANT};
use crate::standards::v1::subsets::any::io::binary::mutations::{decode_op, encode_op};

//#region 🔮️ThirdPartyOracle
#[derive(Debug, PartialEq)]
struct Generation3dSemanticResult {
    widget_count: usize,
    synapse_count: usize,
    layout_count: usize,
    moved_id: String,
    x_bits: u64,
    y_bits: u64,
    synapse_id: String,
    from_port: String,
    to_port: String,
}

/// 🧩️ Owned test boundary shielding production and exported APIs from an oracle library.
trait Generation3dSemanticOracle {
    fn evaluate(&self, source: &[u8]) -> Result<Generation3dSemanticResult, String>;
}

struct SerdeJsonMoveOracle;

impl Generation3dSemanticOracle for SerdeJsonMoveOracle {
    fn evaluate(&self, source: &[u8]) -> Result<Generation3dSemanticResult, String> {
        let root: serde_json::Value = serde_json::from_slice(source).map_err(|error| error.to_string())?;
        let input = root.get("input").ok_or("oracle.input")?;
        let widgets = input.get("widgets").and_then(serde_json::Value::as_array).ok_or("oracle.widgets")?;
        let synapses = input.get("synapses").and_then(serde_json::Value::as_array).ok_or("oracle.synapses")?;
        let layout = input.get("layout").and_then(serde_json::Value::as_object).ok_or("oracle.layout")?;
        let mutation = root.get("mutation").ok_or("oracle.mutation")?;
        if mutation.get("kind").and_then(serde_json::Value::as_str) != Some("move-widget") {
            return Err("oracle.mutation-kind".into());
        }
        let moved_id = mutation.get("id").and_then(serde_json::Value::as_str).ok_or("oracle.moved-id")?;
        if !widgets.iter().any(|widget| widget.get("id").and_then(serde_json::Value::as_str) == Some(moved_id)) || !layout.contains_key(moved_id) {
            return Err("oracle.moved-owner".into());
        }
        let synapse = synapses.first().ok_or("oracle.synapse")?;
        let from = synapse.get("from").and_then(serde_json::Value::as_str).ok_or("oracle.synapse-from")?;
        let to = synapse.get("to").and_then(serde_json::Value::as_str).ok_or("oracle.synapse-to")?;
        if !widgets.iter().any(|widget| widget.get("id").and_then(serde_json::Value::as_str) == Some(from)) || !widgets.iter().any(|widget| widget.get("id").and_then(serde_json::Value::as_str) == Some(to)) {
            return Err("oracle.synapse-owner".into());
        }
        let position = mutation.get("layout").ok_or("oracle.mutation-layout")?;
        Ok(Generation3dSemanticResult {
            widget_count: widgets.len(),
            synapse_count: synapses.len(),
            layout_count: layout.len(),
            moved_id: moved_id.into(),
            x_bits: position.get("x").and_then(serde_json::Value::as_f64).ok_or("oracle.x")?.to_bits(),
            y_bits: position.get("y").and_then(serde_json::Value::as_f64).ok_or("oracle.y")?.to_bits(),
            synapse_id: synapse.get("id").and_then(serde_json::Value::as_str).ok_or("oracle.synapse-id")?.into(),
            from_port: synapse.get("fromPort").and_then(serde_json::Value::as_str).ok_or("oracle.from-port")?.into(),
            to_port: synapse.get("toPort").and_then(serde_json::Value::as_str).ok_or("oracle.to-port")?.into(),
        })
    }
}

fn semantic_result(snapshot: &Generation3dSnapshot, moved_id: &str) -> Generation3dSemanticResult {
    let position = snapshot.host_snapshot.layout.get(moved_id).expect("P3 small-feature moved layout");
    let synapse = snapshot.host_snapshot.synapses.first().expect("P3 small-feature synapse");
    Generation3dSemanticResult {
        widget_count: snapshot.host_snapshot.widgets.len(),
        synapse_count: snapshot.host_snapshot.synapses.len(),
        layout_count: snapshot.host_snapshot.layout.len(),
        moved_id: moved_id.into(),
        x_bits: position.x.to_bits(),
        y_bits: position.y.to_bits(),
        synapse_id: synapse.id.clone(),
        from_port: synapse.from_port.clone(),
        to_port: synapse.to_port.clone(),
    }
}

fn semantic_digest(result: &Generation3dSemanticResult) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for bytes in [
        (result.widget_count as u64).to_be_bytes().to_vec(),
        (result.synapse_count as u64).to_be_bytes().to_vec(),
        (result.layout_count as u64).to_be_bytes().to_vec(),
        result.moved_id.as_bytes().to_vec(),
        result.x_bits.to_be_bytes().to_vec(),
        result.y_bits.to_be_bytes().to_vec(),
        result.synapse_id.as_bytes().to_vec(),
        result.from_port.as_bytes().to_vec(),
        result.to_port.as_bytes().to_vec(),
    ] {
        digest ^= bytes.len() as u64;
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in bytes {
            digest ^= u64::from(byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}

#[test]
fn small_move_widget_feature_matches_the_test_only_third_party_oracle() {
    let source = include_bytes!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔬️p8yz-b-third-party-oracle-laws.json");
    let oracle = SerdeJsonMoveOracle.evaluate(source).expect("third-party P3 semantic oracle");
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_snapshot.widgets = vec![
        semio_framework_artifact_flow_flow::Widget::Neuron { id: "source".into(), neuron_kind: "law".into(), params: Default::default(), input_ports: vec!["in".into()], output_ports: vec!["solid".into()], preview: true },
        semio_framework_artifact_flow_flow::Widget::OutputPreview { id: "preview".into(), preview: Default::default(), expanded: Default::default() },
    ];
    snapshot.host_snapshot.synapses = vec![semio_framework_artifact_flow_flow::SynapseSpec { id: "source-preview".into(), from: "source".into(), from_port: "solid".into(), to: "preview".into(), to_port: String::new() }];
    snapshot.host_snapshot.layout = [("source".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 }), ("preview".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 8.0, y: 3.0 })].into_iter().collect();
    generation3d_apply_retained_mutations_for_test(&mut snapshot, &[Generation3dMutation::MoveWidget(MoveWidget { id: "source".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 12.5, y: -8.25 } })]);
    let owned = semantic_result(&snapshot, "source");
    assert_eq!(owned, oracle, "owned P3 move result must equal the independent serde_json projection");
    assert_eq!(semantic_digest(&owned), semantic_digest(&oracle), "owned and oracle semantic digests must match exactly");
    snapshot.retire_cold();
}
//#endregion 🔮️ThirdPartyOracle

//#region ⏱️BoundedInitializer
/// 🔐️ The lease table is process-global and four slots deep — every initializer law holds this for
/// its whole body (`crate::publication_authority`).
fn initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Generation3dStoreInitializationAuthority {
    generation3d_admit_publication_authority(operation, generation, generation.0, generation.0, generation.0, crate::host::Generation3dPublicationCredits { maximum_items: GENERATION3D_MAXIMUM_DOMAIN_ITEMS, maximum_output_pages: GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: GENERATION3D_MOUNTED_CONTROL_CREDITS })
        .expect("P3 initializer law publication authority");
    Generation3dStoreInitializationAuthority::new(store::create_document_envelope(crate::GENERATION_3D_SCHEMA, "generation3d-bounded-initializer", Generation3dSnapshot::default(), None), operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
}

fn close_initializer(authority: &mut Generation3dStoreInitializationAuthority) {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    for _ in 0..100_000 {
        let demand = authority.retirement_demands(GENERATION3D_OWNER_BYTES).expect("P3 initializer close quote");
        if matches!(authority.close_step(exact_grant(demand)).expect("P3 initializer bounded close"), store::RetainedCloneStep::Complete(_)) {
            assert!(authority.terminal_is_empty());
            return;
        }
    }
    panic!("P3 initializer did not reach terminal-empty close");
}

/// 🦶️ Drives one step of the initializer under `fuel` and `deadline_us` and answers what it lent.
fn step_initializer(authority: &mut Generation3dStoreInitializationAuthority, fuel: u64, deadline_us: u64, cancel: &semio_framework_job::CancelToken, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, sequence: &mut u64) -> Stepped {
    use semio_framework_job::JobOutcomeBorrow;
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    let mut receipt = store::RetainedCloneProgress::default();
    let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(fuel, deadline_us, STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, sequence, &mut receipt);
    match authority.step(&mut cx).expect("P3 initializer step admission") {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Stepped::Yield,
        Some(JobOutcomeBorrow::PreviewReady { .. }) => Stepped::Preview(Vec::new()),
        Some(JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Checkpoint,
        Some(JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
        Some(JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
    }
}

#[test]
fn insufficient_fuel_and_expired_deadline_yield_before_initializer_progress() {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    let _serial = crate::publication_authority::lock();
    let operation = semio_framework_job::OperationId(u64::MAX - 301);
    let generation = semio_framework_job::Generation(301);
    let mut authority = initializer(operation, generation);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    assert_eq!(step_initializer(&mut authority, 0, u64::MAX, &cancel, operation, generation, &mut sequence), Stepped::Yield);
    assert!(matches!(authority.phase, Generation3dStoreInitializationPhase::ValidateEnvelope));
    assert_eq!(step_initializer(&mut authority, 1, 0, &cancel, operation, generation, &mut sequence), Stepped::Yield);
    assert!(matches!(authority.phase, Generation3dStoreInitializationPhase::ValidateEnvelope));
    close_initializer(&mut authority);
    assert!(generation3d_release_publication_authority(operation, generation));
}

#[test]
fn cancelled_and_stale_aba_initializers_retire_to_terminal_empty() {
    use semio_framework_plugin::ArtifactStoreInitializationAuthority;
    let _serial = crate::publication_authority::lock();
    let cancelled_operation = semio_framework_job::OperationId(u64::MAX - 302);
    let cancelled_generation = semio_framework_job::Generation(302);
    let mut cancelled = initializer(cancelled_operation, cancelled_generation);
    cancelled.request_cancel();
    let mut cancelled_sequence = 0;
    let cancelled_token = semio_framework_job::CancelToken::root_now();
    let mut cancelled_outcome = None;
    for _ in 0..100_000 {
        let outcome = step_initializer(&mut cancelled, 1, u64::MAX, &cancelled_token, cancelled_operation, cancelled_generation, &mut cancelled_sequence);
        if outcome != Stepped::Yield {
            cancelled_outcome = Some(outcome);
            break;
        }
    }
    assert_eq!(cancelled_outcome.expect("cancelled P3 initializer must terminate within its bounded owner budget"), Stepped::Cancelled);
    assert!(cancelled.terminal_is_empty());
    assert!(generation3d_release_publication_authority(cancelled_operation, cancelled_generation));

    let stale_operation = semio_framework_job::OperationId(u64::MAX - 303);
    let stale_generation = semio_framework_job::Generation(303);
    let mut stale = initializer(stale_operation, stale_generation);
    let mut stale_sequence = 0;
    let stale_token = semio_framework_job::CancelToken::root_now();
    let mut stale_outcome = None;
    for _ in 0..100_000 {
        let outcome = step_initializer(&mut stale, 1, u64::MAX, &stale_token, stale_operation, semio_framework_job::Generation(stale_generation.0 + 1), &mut stale_sequence);
        if outcome != Stepped::Yield {
            stale_outcome = Some(outcome);
            break;
        }
    }
    assert_eq!(stale_outcome.expect("stale P3 initializer must terminate within its bounded owner budget"), Stepped::Fault);
    assert!(stale.terminal_is_empty());
    assert!(generation3d_release_publication_authority(stale_operation, stale_generation));
}
//#endregion ⏱️BoundedInitializer

fn close_session(session: &mut Generation3dMutationSession) {
    let admitted = session.retained_allocated_bytes();
    let mut released = 0;
    let mut refused_subexact = false;
    for _ in 0..GENERATION3D_MAXIMUM_DOMAIN_ITEMS {
        let maximum_bytes = session.next_retained_release_allocation_bytes().unwrap_or(0);
        if maximum_bytes > 0 && !refused_subexact {
            let before = session.retained_allocated_bytes();
            assert_eq!(
                session.close_step(1, maximum_bytes - 1).expect("P3 subexact retained mutation close"),
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 }
            );
            assert_eq!(session.retained_allocated_bytes(), before);
            refused_subexact = true;
        }
        match session.close_step(1, maximum_bytes).expect("P3 retained mutation close") {
            store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                assert!(session.terminal_is_empty());
                assert_eq!(released, admitted);
                assert!(refused_subexact || admitted == 0);
                return;
            }
            store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_bytes, .. } => released += released_bytes,
        }
    }
    panic!("P3 retained mutation session did not close");
}

#[test]
fn every_variant_decodes_through_retained_structural_grants() {
    let mutations = generation3d_all_retained_mutation_fixtures_for_test();
    assert_eq!(mutations.iter().map(std::mem::discriminant).collect::<std::collections::HashSet<_>>().len(), GENERATION3D_MUTATION_VARIANT_COUNT, "the fixtures cover every variant");
    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("P3 retained mutation fixture encode");
        let mut session = Generation3dMutationSession::new(bytes.len(), GENERATION3D_MAXIMUM_DOMAIN_ITEMS).expect("P3 retained mutation preflight");
        let mut refused_subexact = false;
        for byte in bytes {
            assert!(session.ingress_ready());
            session.admit_byte(byte).expect("one retained mutation byte");
            for _ in 0..GENERATION3D_OWNER_BYTES {
                if let Some(exact) = session.next_retained_allocation_bytes().expect("P3 retained mutation allocation query") {
                    if exact > 0 && !refused_subexact {
                        let before = session.retained_allocated_bytes();
                        assert_eq!(session.reserve_retained_allocation(exact - 1).expect("P3 subexact retained mutation allocation"), (false, 0));
                        assert_eq!(session.retained_allocated_bytes(), before);
                        refused_subexact = true;
                    }
                    let (progressed, _) = session.reserve_retained_allocation(exact).expect("P3 retained mutation allocation");
                    assert!(progressed);
                } else {
                    session.grant().expect("one retained mutation ingress grant");
                }
                if session.ingress_ready() {
                    break;
                }
            }
            assert!(session.ingress_ready(), "symbol expansion must hand input ownership back before the next byte");
        }
        assert!(refused_subexact, "every retained mutation must pre-admit real record-body backing");
        session.seal().expect("exact retained mutation seal");
        let mut ready = false;
        for _ in 0..100_000 {
            if let Some(exact) = session.next_retained_allocation_bytes().expect("P3 retained mutation allocation query") {
                let (progressed, _) = session.reserve_retained_allocation(exact).expect("P3 retained mutation allocation");
                assert!(progressed);
            } else if session.grant().expect("one retained semantic grant") {
                ready = true;
                break;
            }
        }
        assert!(ready, "retained P3 mutation owner must converge");
        let decoded = session.take().expect("typed P3 mutation handoff");
        assert_eq!(decoded, mutation, "the retained ingress route must recover the exact mutation it was handed");
        decoded.retire_cold();
        mutation.retire_cold();
        close_session(&mut session);
    }
}

//#region 🧹️FlowFrontierOwnership
/// 🧊️ Every widget, synapse, layout row and generation this app's document owns is retired through
/// a `semio_framework_artifact_flow_flow::retained::FlowRetirement`, which is a RESERVE-then-CLOSE
/// frontier: each turn must be granted exactly the capacity, release and depth its quote names. Both
/// document retirement routes therefore run under exactly their quoted grants.
///
/// The oracle is the framework's OWN generic route to the same value — the `Arc` retirement
/// `SharedValueRetirementFactory` hands the document store — which must release byte-for-byte
/// what the owned route releases.
#[test]
fn every_document_retirement_pays_its_own_flow_frontier_under_its_quoted_grants() {
    let mut owned = retire_owned_for_test(Generation3dSnapshot::default(), "owned document snapshot");
    let owned_bytes = drive_erased_released(owned.as_mut(), "owned document snapshot");
    let mut aliased = retire_shared_for_test(std::sync::Arc::new(Generation3dSnapshot::default()), "aliased document snapshot");
    let aliased_bytes = drive_erased_released(aliased.as_mut(), "aliased document snapshot");
    assert_eq!(owned_bytes, aliased_bytes, "the owned and Arc retirement routes must release the same exact document backing");
    assert!(owned_bytes > 0, "a populated document fixture owns real backing");
}

/// 🔁️ The same law for the replay displacement route: every `generation3d_apply_initialization_mutation`
/// that displaces a widget, a synapse or a string hands the initializer a `Generation3dReplayDisplaced`,
/// which its close ladder drives under the same quoted grants.
#[test]
fn every_displaced_replay_owner_pays_its_own_flow_frontier_under_its_quoted_grants() {
    let mut snapshot = Generation3dSnapshot::default();
    let mutations = generation3d_all_retained_mutation_fixtures_for_test();
    let mut displaced = 0usize;
    for mutation in &mutations {
        let Ok(Some(value)) = generation3d_apply_initialization_mutation(&mut snapshot, mutation) else { continue };
        let mut owner = retire_owned_for_test(value, "displaced replay owner");
        drive_erased_released(owner.as_mut(), "displaced replay owner");
        displaced += 1;
    }
    for mutation in mutations {
        mutation.retire_cold();
    }
    assert!(displaced > 0, "the retained mutation fixtures must displace at least one owner");
    let mut replayed = retire_owned_for_test(snapshot, "replayed document snapshot");
    drive_erased_released(replayed.as_mut(), "replayed document snapshot");
}
//#endregion 🧹️FlowFrontierOwnership

/// 🧬️ Authored semantic vectors traverse compact and retained wire paths and directly replay addressed fields — on a replica
/// WITHOUT any flow extension installed (design §20.9): every input lands, or is refused, from the operator record alone, whose
/// params state each declared input's default literal.
#[test]
fn semantic_wire_vectors_match_independent_json_oracle() {
    use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{WidgetInputPlane, WidgetInputValue};
    use crate::standards::v1::subsets::any::schema::mutations::{generation3d_number_literal,generation3d_param_number,generation3d_param_vector,generation3d_vector_literal};

    use semio_framework_artifact_flow_flow::{Widget, WidgetLayout};
    let record = |id: &str, kind: &str, params: Vec<(&str, semio_framework_value::DslValue)>| Widget::Neuron {
        id: id.into(),
        neuron_kind: kind.into(),
        params: params.into_iter().fold(semio_framework_artifact_flow_flow::neural::Dictionary::new(), |record, (key, literal)| record.insert(key, <semio_framework_artifact_flow_flow::neural::Value as semio_framework_value::FromValue>::from_value(literal).expect("a typed literal is a neural value"))),
        input_ports: Vec::new(),
        output_ports: Vec::new(),
        preview: false,
    };
    every_variant_decodes_through_retained_structural_grants();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧫️fixtures/🧬️semantic-wire/🔣️.json")).expect("independent serde corpus");
    for case in corpus["cases"].as_array().expect("wire vectors") {
        let mutation = <Generation3dMutation as semio_framework_value::FromValue>::from_value(case["mutation"].clone().into()).expect("first-party mutation decoder");
        let text = protocol::OpText::print_op(&mutation);
        assert_eq!(<Generation3dMutation as protocol::OpText>::parse_op(&text).expect("text decode"), mutation);
        let bytes = encode_op(&mutation).expect("binary encode");
        assert_eq!(bytes[1], case["tag"].as_u64().expect("independent tag") as u8);
        assert_eq!(decode_op(&bytes).expect("binary decode"), mutation);
        let mut snapshot = Generation3dSnapshot::default();
        for widget in std::mem::take(&mut snapshot.host_snapshot.widgets) { widget.retire_cold(); }
        snapshot.host_snapshot.widgets.push(Widget::InputSlider { id: "slider".into(), label: "Slider".into(), value: 0.0, min: -10.0, max: 10.0, step: 0.5 });
        snapshot.host_snapshot.widgets.push(record("translate", "brep.xform.translate", vec![("offset", generation3d_vector_literal("vector", [0.0; 3])), ("label", WidgetInputValue::Text(String::new()).literal())]));
        snapshot.host_snapshot.widgets.push(record("rotate", "brep.xform.rotate", vec![("axis", generation3d_vector_literal("vector", [0.0, 0.0, 1.0])), ("angle", generation3d_number_literal(0.0))]));
        snapshot.host_snapshot.widgets.push(record("scale", "brep.xform.scale", vec![("factor", generation3d_vector_literal("vector", [1.0; 3])), ("center", generation3d_vector_literal("point", [0.0; 3])), ("uniform", WidgetInputValue::Boolean(false).literal())]));
        snapshot.host_snapshot.widgets.push(record("shape", "semantic-wire-law.collections", vec![("items", semio_framework_value::DslValue::Object(vec![("$schema".into(), semio_framework_value::DslValue::String("list".into()))]))]));
        snapshot.host_snapshot.widgets.push(record("section", "semantic-wire-law.sections", vec![("plane", WidgetInputValue::Plane(WidgetInputPlane { origin: [0.0; 3], normal: [0.0, 0.0, 1.0] }).literal())]));
        snapshot.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: 0.0, y: 0.0 });
        let before_schema = snapshot.host_snapshot.schema.clone();
        if let Some(displaced) = generation3d_apply_initialization_mutation(&mut snapshot, &mutation).expect("direct semantic replay") {
            let mut owner = retire_owned_for_test(displaced, "semantic replay displacement");
            drive_erased_released(owner.as_mut(), "semantic replay displacement");
        }
        assert_eq!(snapshot.host_snapshot.widgets.len(), 6);
        assert_eq!(snapshot.host_snapshot.schema, before_schema);
        let expected = &case["expected"];
        match &mutation {
            Generation3dMutation::ChangeSliderValue(_) => {
                let Widget::InputSlider { value, .. } = &snapshot.host_snapshot.widgets[0] else { panic!("slider owner") };
                assert_eq!(*value, expected["value"].as_f64().expect("oracle slider"));
            }
            Generation3dMutation::MoveNodes(_) => {
                let layout = snapshot.host_snapshot.layout.get("slider").expect("addressed layout");
                assert_eq!([layout.x, layout.y], [expected["layout"][0].as_f64().unwrap(), expected["layout"][1].as_f64().unwrap()]);
            }
            Generation3dMutation::ChangeWidgetInput(payload) => {
                let params = snapshot.host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == payload.id).map(semio_framework_value::ToValue::to_value).and_then(|widget| widget.get("params").cloned()).expect("addressed operator params");
                let literal: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(params.get(&payload.channel).expect("the addressed input"))).expect("literal json");
                assert_eq!(literal, expected["input"], "{}.{}", payload.id, payload.channel);
            }
            _ => {
                let (index, key) = match &mutation { Generation3dMutation::DragTransforms(_) => (1, "offset"), Generation3dMutation::RotateTransforms(_) => (2, "axis"), Generation3dMutation::ScaleTransforms(_) => (3, "factor"), _ => unreachable!() };
                let params = semio_framework_value::ToValue::to_value(&snapshot.host_snapshot.widgets[index]).get("params").cloned().expect("operator params");
                let axes: [f64; 3] = std::array::from_fn(|axis| expected[key][axis].as_f64().expect("independent vector"));
                assert_eq!(generation3d_param_vector(&params, key, [f64::NAN; 3]), axes);
                if key == "axis" { assert_eq!(generation3d_param_number(&params, "angle", f64::NAN), expected["angle"].as_f64().expect("oracle angle")); }
            }
        }
        mutation.retire_cold();
        let mut replayed = retire_owned_for_test(snapshot, "semantic replay snapshot");
        drive_erased_released(replayed.as_mut(), "semantic replay snapshot");
    }
}

/// 🚪️ Cancelled semantic target lists hand their dynamic owners to the bounded Flow close frontier.
#[test]
fn semantic_wire_vectors_cancelled_target_lists_reach_terminal_empty() {
    let mutation = Generation3dMutation::DragTransforms(DragTransforms { targets: (0..128).map(|index| format!("translate-{index}")).collect(), dx: 2.0, dy: -1.0, dz: 4.0 });
    let bytes = encode_op(&mutation).expect("semantic list frame");
    for prefix in [bytes.len() / 2, bytes.len() - 9, bytes.len()] {
        let mut session = Generation3dMutationSession::new(bytes.len(), GENERATION3D_MAXIMUM_DOMAIN_ITEMS).expect("cancelled semantic owner");
        for byte in &bytes[..prefix] {
            assert!(session.ingress_ready());
            session.admit_byte(*byte).expect("retained semantic byte");
            for _ in 0..GENERATION3D_OWNER_BYTES {
                if let Some(exact) = session.next_retained_allocation_bytes().expect("semantic reservation") { assert!(session.reserve_retained_allocation(exact).expect("semantic reserve").0); }
                else { session.grant().expect("semantic ingress grant"); }
                if session.ingress_ready() { break; }
            }
            assert!(session.ingress_ready());
        }
        if prefix == bytes.len() {
            session.seal().expect("semantic seal");
            for _ in 0..100_000 {
                if let Some(exact) = session.next_retained_allocation_bytes().expect("semantic terminal reservation") { assert!(session.reserve_retained_allocation(exact).expect("semantic terminal reserve").0); }
                else if session.grant().expect("semantic terminal grant") { break; }
            }
        }
        close_session(&mut session);
    }
    mutation.retire_cold();
}
