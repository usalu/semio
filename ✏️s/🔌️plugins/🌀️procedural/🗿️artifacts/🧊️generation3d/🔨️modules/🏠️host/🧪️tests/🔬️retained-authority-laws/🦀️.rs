use semio_s_artifact_procedural_3d_host::*;
use semio_s_artifact_procedural_3d_envelope::*;
use semio_s_artifact_procedural_3d_testkit::retirement_driver::drive_erased_released;
use semio_s_artifact_procedural_3d_testkit::retirement_driver::close_job;
use semio_s_artifact_procedural_3d_testkit::retirement_driver::step_once;
use semio_s_artifact_procedural_3d_testkit::retirement_driver::retire_owned_for_test;
use semio_s_artifact_procedural_3d_testkit::retirement_driver::retire_shared_for_test;
use semio_s_artifact_procedural_3d_testkit::retirement_driver::Stepped;
use semio_s_artifact_procedural_3d_io_binary::mutations::decode_op;
use semio_s_artifact_procedural_3d_io_binary::mutations::encode_op;
#[cfg(test)] use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
#[cfg(test)] use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;
#[cfg(test)] use semio_s_artifact_procedural_3d_mut_transforms::drag_transforms::DragTransforms;

const SEMANTIC_WIRE_CORPUS: &str = include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🧫️fixtures/🧬️semantic-wire/🔣️.json");

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
    let source = include_bytes!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔬️p8yz-b-third-party-oracle-laws.json");
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
type Generation3dStoreInitializationAuthority = store::ArtifactStoreReplayInitializer<Generation3dSnapshot, Generation3dMutation, store::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>;

fn initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Generation3dStoreInitializationAuthority {
    Generation3dStoreInitializationAuthority::new(store::os_store::document_materialize::create_document_envelope(semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA, "generation3d-bounded-initializer", Generation3dSnapshot::default(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
}

/// 🧩️ Lends the initializer authority to the original fixture caller as the interactive job its store wrapper makes of it.
struct AuthorityJob<'a>(&'a mut Generation3dStoreInitializationAuthority);

impl semio_framework_job::InteractiveJob for AuthorityJob<'_> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        Ok(self.0.retirement_demands(GENERATION3D_OWNER_BYTES)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        Ok(self.0.retirement_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        Ok(self.0.retirement_demands(GENERATION3D_OWNER_BYTES)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        Ok(self.0.retirement_demands(GENERATION3D_OWNER_BYTES)?.depth)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>, _: &mut semio_framework_job::OriginalJobReturnClaim<'_>) -> semio_framework_job::InteractiveJobStep<'a> {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        semio_framework_job::InteractiveJobStep::from_callback(self.0.step(cx))
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        self.0.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        self.0.begin_close();
    }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::InteractiveJobCloseStep as Close;
        use semio_plugin::ArtifactStoreInitializationAuthority;
        if self.0.terminal_is_empty() {
            return Ok(Close::Complete { progress: Default::default() });
        }
        Ok(match self.0.close_step(cx)? {
            store::RetainedCloneStep::Complete(progress) => Close::Complete { progress },
            store::RetainedCloneStep::Progress(progress) => Close::Pending { progress },
        })
    }

    fn terminal_is_empty(&self) -> bool {
        use semio_plugin::ArtifactStoreInitializationAuthority;
        self.0.terminal_is_empty()
    }
}

fn close_initializer(authority: &mut Generation3dStoreInitializationAuthority) {
    close_job(&mut AuthorityJob(authority), "P3 initializer");
}

/// 🦶️ Drives one step of the initializer under `fuel` and `deadline_us` through the original fixture caller and answers what it lent.
fn step_initializer(authority: &mut Generation3dStoreInitializationAuthority, fuel: u64, deadline_us: u64, cancel: &semio_framework_job::CancelToken, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, sequence: &mut u64) -> Stepped {
    step_once(&mut AuthorityJob(authority), fuel, deadline_us, cancel, operation, generation, sequence)
}

#[test]
fn insufficient_fuel_and_expired_deadline_yield_before_initializer_progress() {
    use semio_plugin::ArtifactStoreInitializationAuthority;
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
    let operation = semio_framework_job::OperationId(u64::MAX - 301);
    let generation = semio_framework_job::Generation(301);
    let mut authority = initializer(operation, generation);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut sequence = 0;
    assert_eq!(step_initializer(&mut authority, 0, u64::MAX, &cancel, operation, generation, &mut sequence), Stepped::Yield);
    assert_eq!(format!("{:?}", authority.phase), "ValidateEnvelope");
    assert_eq!(step_initializer(&mut authority, 1, 0, &cancel, operation, generation, &mut sequence), Stepped::Yield);
    assert_eq!(format!("{:?}", authority.phase), "ValidateEnvelope");
    close_initializer(&mut authority);
}

#[test]
fn cancelled_and_stale_aba_initializers_retire_to_terminal_empty() {
    use semio_plugin::ArtifactStoreInitializationAuthority;
    let _serial = semio_s_artifact_procedural_3d_testkit::test_serial::lock();
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
}
//#endregion ⏱️BoundedInitializer

/// 🧹️ Decodes the pack wire and retires the decoded owner before judging that it equals the mutation it came from.
fn assert_decodes_to(bytes: &[u8], mutation: &Generation3dMutation) {
    let decoded = decode_op(bytes).expect("the pack wire decodes");
    let same = decoded == *mutation;
    decoded.retire_cold();
    assert!(same, "the pack wire must recover the exact mutation it was handed");
}

#[test]
fn every_variant_decodes_through_retained_structural_grants() {
    let mutations = generation3d_all_retained_mutation_fixtures_for_test(SEMANTIC_WIRE_CORPUS);
    assert_eq!(mutations.iter().map(std::mem::discriminant).collect::<std::collections::HashSet<_>>().len(), GENERATION3D_MUTATION_VARIANT_COUNT, "the fixtures cover every variant");
    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("P3 retained mutation fixture encode");
        assert_decodes_to(&bytes, &mutation);
        store::os_store::test_support::assert_op_text_binary_equivalence_cold(&mutation, |decoded| decoded.retire_cold());
        mutation.retire_cold();
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
/// what the owned route releases, plus the `Arc` allocation itself (counters and inline value) that only it frees.
#[test]
fn every_document_retirement_pays_its_own_flow_frontier_under_its_quoted_grants() {
    let mut owned = retire_owned_for_test(Generation3dSnapshot::default(), "owned document snapshot");
    let owned_bytes = drive_erased_released(owned.as_mut(), "owned document snapshot");
    let mut aliased = retire_shared_for_test(std::sync::Arc::new(Generation3dSnapshot::default()), "aliased document snapshot");
    let aliased_bytes = drive_erased_released(aliased.as_mut(), "aliased document snapshot");
    eprintln!("[DEBUG] owned={owned_bytes} aliased={aliased_bytes} T={} arc={} birth={}", std::mem::size_of::<Generation3dSnapshot>(), semio_framework_value::shared_retirement_allocation_bytes::<Generation3dSnapshot>(), semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<Generation3dSnapshot>());
    assert_eq!(owned_bytes + semio_framework_value::shared_retirement_allocation_bytes::<Generation3dSnapshot>(), aliased_bytes, "the Arc route must release exactly the owned document backing plus its own shared allocation");
    assert!(owned_bytes > 0, "a populated document fixture owns real backing");
}

/// 🔁️ The same law for the replay displacement route: every `generation3d_apply_initialization_mutation`
/// that displaces a widget, a synapse or a string hands the initializer a `Generation3dReplayDisplaced`,
/// which its close ladder drives under the same quoted grants.
#[test]
fn every_displaced_replay_owner_pays_its_own_flow_frontier_under_its_quoted_grants() {
    let mut snapshot = Generation3dSnapshot::default();
    let mutations = generation3d_all_retained_mutation_fixtures_for_test(SEMANTIC_WIRE_CORPUS);
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
    use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputPlane;
    use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
    use semio_s_artifact_procedural_3d_mut_transforms::generation3d_number_value;
    use semio_s_artifact_procedural_3d_mut_transforms::generation3d_param_number;
    use semio_s_artifact_procedural_3d_mut_transforms::generation3d_param_vector;
    use semio_s_artifact_procedural_3d_mut_transforms::generation3d_vector_value;

    use semio_framework_artifact_flow_flow::{Widget, WidgetLayout};
    let record = |id: &str, kind: &str, params: Vec<(&str, semio_framework_artifact_flow_flow::neural::Value)>| Widget::Neuron {
        id: id.into(),
        neuron_kind: kind.into(),
        params: params.into_iter().fold(semio_framework_artifact_flow_flow::neural::Dictionary::new(), |record, (key, literal)| record.insert(key, literal)),
        input_ports: Vec::new(),
        output_ports: Vec::new(),
        preview: false,
    };
    every_variant_decodes_through_retained_structural_grants();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🧫️fixtures/🧬️semantic-wire/🔣️.json")).expect("independent serde corpus");
    for case in corpus["cases"].as_array().expect("wire vectors") {
        let mutation = <Generation3dMutation as semio_framework_value::FromValue>::from_value(case["mutation"].clone().into()).expect("first-party mutation decoder");
        let text = protocol::OpText::print_op(&mutation);
        let parsed = <Generation3dMutation as protocol::OpText>::parse_op(&text).expect("text decode");
        let same = parsed == mutation;
        parsed.retire_cold();
        assert!(same, "text decode");
        let bytes = encode_op(&mutation).expect("binary encode");
        assert_decodes_to(&bytes, &mutation);
        let mut snapshot = Generation3dSnapshot::default();
        for widget in std::mem::take(&mut snapshot.host_snapshot.widgets) { widget.retire_cold(); }
        snapshot.host_snapshot.widgets.push(Widget::InputSlider { id: "slider".into(), label: "Slider".into(), value: 0.0, min: -10.0, max: 10.0, step: 0.5 });
        snapshot.host_snapshot.widgets.push(record("translate", "brep.xform.translate", vec![("offset", generation3d_vector_value("vector", [0.0; 3])), ("label", WidgetInputValue::Text(String::new()).neural_value())]));
        snapshot.host_snapshot.widgets.push(record("rotate", "brep.xform.rotate", vec![("axis", generation3d_vector_value("vector", [0.0, 0.0, 1.0])), ("angle", generation3d_number_value(0.0))]));
        snapshot.host_snapshot.widgets.push(record("scale", "brep.xform.scale", vec![("factor", generation3d_vector_value("vector", [1.0; 3])), ("center", generation3d_vector_value("point", [0.0; 3])), ("uniform", WidgetInputValue::Boolean(false).neural_value())]));
        snapshot.host_snapshot.widgets.push(record("shape", "semantic-wire-law.collections", vec![("items", semio_framework_artifact_flow_flow::neural::Value::Dictionary(semio_framework_artifact_flow_flow::neural::Dictionary::with_schema("list")))]));
        snapshot.host_snapshot.widgets.push(record("section", "semantic-wire-law.sections", vec![("plane", WidgetInputValue::Plane(WidgetInputPlane { origin: [0.0; 3], normal: [0.0, 0.0, 1.0] }).neural_value())]));
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
                let params = snapshot.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget) == payload.id).map(semio_framework_value::ToValue::to_value).and_then(|widget| widget.get("params").cloned()).expect("addressed operator params");
                let literal: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(params.get(&payload.channel).expect("the addressed input"))).expect("literal json");
                assert_eq!(literal, expected["input"], "{}.{}", payload.id, payload.channel);
            }
            _ => {
                let (index, key) = match &mutation { Generation3dMutation::DragTransforms(_) => (1, "offset"), Generation3dMutation::RotateTransforms(_) => (2, "axis"), Generation3dMutation::ScaleTransforms(_) => (3, "factor"), _ => unreachable!() };
                let Widget::Neuron { params, .. } = &snapshot.host_snapshot.widgets[index] else { panic!("operator params") };
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

/// 🚪️ A long semantic target list round-trips and every truncated wire is refused instead of half-decoded.
#[test]
fn semantic_wire_vectors_long_target_lists_receive_and_truncated_wire_is_refused() {
    let mutation = Generation3dMutation::DragTransforms(DragTransforms { targets: (0..128).map(|index| format!("translate-{index}")).collect(), dx: 2.0, dy: -1.0, dz: 4.0 });
    let bytes = encode_op(&mutation).expect("semantic list frame");
    for prefix in [0, 1, bytes.len() / 2, bytes.len() - 9, bytes.len() - 1] {
        assert!(decode_op(&bytes[..prefix]).is_err(), "a wire truncated to {prefix} of {} bytes must be refused", bytes.len());
    }
    assert_decodes_to(&bytes, &mutation);
    mutation.retire_cold();
}
