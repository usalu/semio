//! 🧪️ Laws of the generation3d gesture leaves (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, design §5, §13, §19): the
//! ABSOLUTE `change-slider-value` a scrub commits and `change-widget-input` an inspector field or a mesh edit commits, and
//! the RELATIVE `drag-`/`rotate-`/`scale-transforms` and `move-nodes` a gumball or node drag commits. A relative leaf reads its BASE operator, so an edited gesture replays on any base; its
//! inverse is the absolute base rows, never a negated delta.

use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::change_slider_value;
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{change_widget_input, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::drag_transforms;
use crate::standards::v1::subsets::any::schema::mutations::move_nodes::move_nodes;
use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::rotate_transforms;
use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::scale_transforms;
use semio_framework_artifact_flow_flow::neural::{Dictionary, Value};
use semio_framework_artifact_flow_flow::{Widget, WidgetLayout};

const TRANSLATE: &str = "shape__gumball_translate";
const ROTATE: &str = "shape__gumball_rotate";
const SCALE: &str = "shape__gumball_scale";

fn neuron(id: &str, kind: &str, params: Vec<(&'static str, dsl::DslValue)>) -> Widget {
    let mut dictionary = Dictionary::new();
    for (key, value) in params {
        dictionary = dictionary.insert(key, <Value as dsl::FromValue>::from_value(value).expect("a typed literal is a neural value"));
    }
    Widget::Neuron { id: id.into(), neuron_kind: kind.into(), params: dictionary, input_ports: Vec::new(), output_ports: Vec::new(), preview: false }
}

/// 🧱️ A base with one operator of each gumball family, one slider, one note and two placed nodes.
fn base(offset: [f64; 3]) -> Generation3dSnapshot {
    let mut snapshot = empty_generation3d_snapshot();
    snapshot.host_snapshot.widgets.push(neuron(TRANSLATE, "brep.mesh.translate", vec![("offset", generation3d_vector_literal("vector", offset))]));
    snapshot.host_snapshot.widgets.push(neuron(ROTATE, "brep.xform.rotate", vec![("axis", generation3d_vector_literal("vector", [0.0, 0.0, 1.0])), ("angle", generation3d_number_literal(std::f64::consts::FRAC_PI_2))]));
    snapshot.host_snapshot.widgets.push(neuron(SCALE, "brep.mesh.scale", vec![("factor", generation3d_vector_literal("vector", [2.0, 2.0, 2.0])), ("center", generation3d_vector_literal("point", [0.0; 3]))]));
    snapshot.host_snapshot.widgets.push(Widget::InputSlider { id: "height".into(), label: "Height".into(), value: 6.0, min: 0.0, max: 10.0, step: 0.5 });
    snapshot.host_snapshot.widgets.push(Widget::InputNote { id: "note".into(), text: "Note".into() });
    for (id, x, y) in [("height", 10.0, 20.0), ("note", -5.0, 0.0)] {
        let _ = snapshot.host_snapshot.layout.insert(id.to_string(), WidgetLayout { x, y });
    }
    snapshot
}

fn params_of(snapshot: &Generation3dSnapshot, id: &str) -> dsl::DslValue {
    let widget = snapshot.host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == id).expect("operator present");
    dsl::ToValue::to_value(widget).get("params").cloned().unwrap_or(dsl::DslValue::Null)
}

fn outcome_codes(mutation: &Generation3dMutation, base: &Generation3dSnapshot) -> Vec<(protocol::Severity, String)> {
    let (delta, messages) = protocol::Mutation::diff(mutation, base).into_parts();
    delta.retire_cold();
    messages.into_iter().map(|message| (message.level, message.code.0)).collect()
}

/// ♻️ Applies `mutation` to a copy of `base`, then its inverse, and answers the applied copy; the inverse must restore
/// `base` exactly.
fn applied_and_restored(mutation: &Generation3dMutation, base: &Generation3dSnapshot) -> Generation3dSnapshot {
    let inverse = inverse_generation3d_mutation(base, mutation);
    let mut applied = base.clone();
    apply_generation3d_mutation(&mut applied, mutation).expect("the gesture leaf applies to its base");
    let mut restored = applied.clone();
    for step in &inverse {
        apply_generation3d_mutation(&mut restored, step).expect("every inverse row applies");
    }
    assert_eq!(&restored, base, "{} must invert exactly to its base", <Generation3dMutation as protocol::SemanticMutation<Generation3dSnapshot>>::semantics(mutation).kind);
    restored.retire_cold();
    for step in inverse {
        step.retire_cold();
    }
    applied
}

/// ✋️ A drag adds its offset to the operator's BASE offset; the inverse restores the base operator whole.
#[test]
fn a_drag_composes_onto_the_base_offset_and_inverts_exactly() {
    let base = base([1.0, 0.0, 0.0]);
    let applied = applied_and_restored(&drag_transforms(vec![TRANSLATE.into()], [2.0, 3.0, -1.0]), &base);
    assert_eq!(generation3d_param_vector(&params_of(&applied, TRANSLATE), "offset", [0.0; 3]), [3.0, 3.0, -1.0]);
    applied.retire_cold();
    base.retire_cold();
}

/// 🔁️ Replay determinism: the SAME edited leaf replays relative to whatever base it lands on — editing its offset in
/// history moves the operator by the edited offset from the operator's own base, never to a stored absolute.
#[test]
fn an_edited_drag_replays_relative_to_any_base() {
    for (offset, edited) in [([1.0, 0.0, 0.0], [5.0, 0.0, 0.0]), ([-4.0, 2.0, 7.0], [0.5, 0.5, 0.5])] {
        let base = base(offset);
        let applied = applied_and_restored(&drag_transforms(vec![TRANSLATE.into()], edited), &base);
        assert_eq!(generation3d_param_vector(&params_of(&applied, TRANSLATE), "offset", [0.0; 3]), std::array::from_fn::<f64, 3, _>(|axis| offset[axis] + edited[axis]));
        applied.retire_cold();
        base.retire_cold();
    }
}

/// 🔃️ A rotation composes after the operator's base rotation (`AxisAngle::then`): a quarter turn after a quarter turn
/// about the same axis is a half turn.
#[test]
fn a_rotation_composes_after_the_base_rotation() {
    let base = base([0.0; 3]);
    let applied = applied_and_restored(&rotate_transforms(vec![ROTATE.into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2), &base);
    let angle = generation3d_param_number(&params_of(&applied, ROTATE), "angle", 0.0);
    assert!((angle - std::f64::consts::PI).abs() < 1e-12, "a quarter turn after a quarter turn is a half turn, got {angle}");
    applied.retire_cold();
    base.retire_cold();
}

/// 📏️ A scaling multiplies its factors into the operator's base factors.
#[test]
fn a_scaling_multiplies_the_base_factors() {
    let base = base([0.0; 3]);
    let applied = applied_and_restored(&scale_transforms(vec![SCALE.into()], [0.5, 3.0, 1.0]), &base);
    assert_eq!(generation3d_param_vector(&params_of(&applied, SCALE), "factor", [1.0; 3]), [1.0, 6.0, 2.0]);
    applied.retire_cold();
    base.retire_cold();
}

/// 🎯️ Missing and foreign targets are skipped with `mutation.partial`; none left is `target-missing` or `target-mismatch`;
/// the identity is `mutation.no-op` with no inverse.
#[test]
fn skipped_targets_and_the_identity_report_their_vocabulary_codes() {
    use protocol::Severity::{Error, Warning};
    let base = base([0.0; 3]);
    assert_eq!(outcome_codes(&drag_transforms(vec![TRANSLATE.into(), "ghost".into(), ROTATE.into()], [1.0, 0.0, 0.0]), &base), vec![(Warning, "mutation.partial".into()), (Warning, "mutation.partial".into())]);
    assert_eq!(outcome_codes(&drag_transforms(vec!["ghost".into()], [1.0, 0.0, 0.0]), &base), vec![(Error, "mutation.target-missing".into())]);
    assert_eq!(outcome_codes(&drag_transforms(vec![SCALE.into(), "height".into()], [1.0, 0.0, 0.0]), &base), vec![(Error, "mutation.target-mismatch".into())]);
    assert_eq!(outcome_codes(&drag_transforms(vec![TRANSLATE.into()], [0.0; 3]), &base), vec![(Warning, "mutation.no-op".into())]);
    assert!(inverse_generation3d_mutation(&base, &drag_transforms(vec![TRANSLATE.into()], [0.0; 3])).is_empty(), "an identity drag owes no inverse");
    base.retire_cold();
}

/// 🚫️ Payload-intrinsic invariants are Fatal `mutation.invariant`: no target, a repeated target, a zero rotation axis
/// (`axis-nonzero`), a non-positive scale factor.
#[test]
fn payload_invariants_are_fatal() {
    use protocol::Severity::Fatal;
    let base = base([0.0; 3]);
    let invariant = vec![(Fatal, "mutation.invariant".to_string())];
    assert_eq!(outcome_codes(&drag_transforms(Vec::new(), [1.0, 0.0, 0.0]), &base), invariant);
    assert_eq!(outcome_codes(&drag_transforms(vec![TRANSLATE.into(), TRANSLATE.into()], [1.0, 0.0, 0.0]), &base), invariant);
    assert_eq!(outcome_codes(&rotate_transforms(vec![ROTATE.into()], [0.0; 3], 1.0), &base), invariant);
    assert_eq!(outcome_codes(&scale_transforms(vec![SCALE.into()], [0.0, 1.0, 1.0]), &base), invariant);
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into(), "height".into()], 1.0, 1.0), &base), invariant);
    base.retire_cold();
}

/// 🎚️ A slider value is absolute: in range it sets the value, out of range it widens the range like the canvas knob, the
/// value it holds is `no-op`, a non-slider is `target-mismatch`, a missing widget `target-missing`; the inverse restores the
/// base slider whole (range included).
#[test]
fn a_slider_value_is_absolute_and_inverts_to_the_whole_base_slider() {
    use protocol::Severity::{Error, Warning};
    let base = base([0.0; 3]);
    for (value, range) in [(7.5, (0.0, 10.0)), (42.0, (0.0, 50.0))] {
        let applied = applied_and_restored(&change_slider_value("height", value), &base);
        let slider = applied.host_snapshot.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, min, max, .. } if id == "height" => Some((*value, (*min, *max))),
            _ => None,
        });
        assert_eq!(slider, Some((value, range)));
        applied.retire_cold();
    }
    assert_eq!(outcome_codes(&change_slider_value("height", 6.0), &base), vec![(Warning, "mutation.no-op".into())]);
    assert_eq!(outcome_codes(&change_slider_value("note", 1.0), &base), vec![(Error, "mutation.target-mismatch".into())]);
    assert_eq!(outcome_codes(&change_slider_value("ghost", 1.0), &base), vec![(Error, "mutation.target-missing".into())]);
    base.retire_cold();
}

/// 🚚️ A node drag moves every placed node by its offset from the BASE position and inverts to the absolute base positions;
/// an unplaced node is skipped with `mutation.partial`, a zero offset is `mutation.no-op`.
#[test]
fn a_node_drag_moves_placed_nodes_relative_to_their_base_position() {
    use protocol::Severity::Warning;
    let base = base([0.0; 3]);
    let applied = applied_and_restored(&move_nodes(vec!["height".into(), "note".into()], 40.0, -12.5), &base);
    assert_eq!(applied.host_snapshot.layout.get("height").map(|layout| (layout.x, layout.y)), Some((50.0, 7.5)));
    assert_eq!(applied.host_snapshot.layout.get("note").map(|layout| (layout.x, layout.y)), Some((35.0, -12.5)));
    applied.retire_cold();
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into(), TRANSLATE.into()], 1.0, 1.0), &base), vec![(Warning, "mutation.partial".into())]);
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into()], 0.0, 0.0), &base), vec![(Warning, "mutation.no-op".into())]);
    base.retire_cold();
}

/// 🏷️ Every gesture leaf labels its history row in English and German (decimal comma).
#[test]
fn gesture_leaves_label_their_rows_in_english_and_german() {
    let label = |mutation: Generation3dMutation| serde_json::to_string(&<Generation3dMutation as protocol::SemanticMutation<Generation3dSnapshot>>::label(&mutation)).expect("label serializes");
    for (mutation, english, german) in [
        (drag_transforms(vec![TRANSLATE.into()], [1.5, -2.0, 0.25]), "Drag 1 shape(s) by (1.5, -2, 0.25)", "1 Form(en) um (1,5; -2; 0,25) ziehen"),
        (rotate_transforms(vec![ROTATE.into()], [0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2), "Rotate 1 shape(s) by 90°", "1 Form(en) um 90° drehen"),
        (scale_transforms(vec![SCALE.into(), TRANSLATE.into()], [2.0, 2.0, 0.5]), "Scale 2 shape(s) by (2, 2, 0.5)", "2 Form(en) um (2; 2; 0,5) skalieren"),
        (change_slider_value("height", 7.5), "Set slider \\\"height\\\" to 7.5", "Schieberegler \\\"height\\\" auf 7,5 setzen"),
        (move_nodes(vec!["a".into(), "b".into()], 40.0, -12.5), "Move 2 node(s) by (40, -12.5)", "2 Knoten um (40; -12,5) verschieben"),
        (change_widget_input(SCALE, "center", WidgetInputValue::Point([0.5, -1.0, 2.0])), "Set input \\\"center\\\" of \\\"shape__gumball_scale\\\" to (0.5, -1, 2)", "Eingang \\\"center\\\" von \\\"shape__gumball_scale\\\" auf (0,5; -1; 2) setzen"),
        (change_widget_input("note", "text", WidgetInputValue::Text("Hi".into())), "Set input \\\"text\\\" of \\\"note\\\" to \\\"Hi\\\"", "Eingang \\\"text\\\" von \\\"note\\\" auf \\\"Hi\\\" setzen"),
    ] {
        let label = label(mutation);
        assert!(label.contains(english) && label.contains(german), "{label}");
    }
}

/// 🧾️ Every committed wire witness decodes to its leaf and round-trips the binary op codec unchanged.
#[test]
fn every_wire_witness_decodes_and_round_trips_the_binary_codec() {
    let witnesses = [
        include_str!("../../../../🧫️fixtures/🧬️mutations/🎚️change-slider-value/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../../../🧫️fixtures/🧬️mutations/🔃️rotate-transforms/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../../../🧫️fixtures/🧬️mutations/📏️scale-transforms/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🧾️wire-witness/🦠️mutation/🔣️.json"),
        include_str!("../../../../🧫️fixtures/🧬️mutations/🎛️change-widget-input/🧾️wire-witness/🦠️mutation/🔣️.json"),
    ];
    for (witness, kind) in witnesses.iter().zip(["change-slider-value", "drag-transforms", "rotate-transforms", "scale-transforms", "move-nodes", "change-widget-input"]) {
        let mutation: Generation3dMutation = dsl::json::from_json_str(witness).unwrap_or_else(|error| panic!("{kind} witness decodes: {error}"));
        assert_eq!(<Generation3dMutation as protocol::SemanticMutation<Generation3dSnapshot>>::semantics(&mutation).kind, kind);
        let bytes = protocol::OpBinary::encode_op(&mutation).expect("witness encodes");
        assert_eq!(<Generation3dMutation as protocol::OpBinary>::decode_op(&bytes).expect("witness decodes back"), mutation, "{kind} round-trips the binary codec");
        let text = protocol::OpText::print_op(&mutation);
        assert_eq!(<Generation3dMutation as protocol::OpText>::parse_op(&text).expect("witness parses back"), mutation, "{kind} round-trips the text codec: {text}");
    }
}

//#region 🎛️WidgetInput
/// 🎛️ A widget input is absolute: every literal type sets the addressed operator param (a text source's `text` sets its
/// text) whatever it held, and the inverse restores the base widget whole.
#[test]
fn a_widget_input_sets_its_typed_literal_and_inverts_to_the_whole_base_widget() {
    let base = base([0.0; 3]);
    for (id, channel, input) in [
        (ROTATE, "angle", WidgetInputValue::Number(0.25)),
        (TRANSLATE, "offset", WidgetInputValue::Vector([1.5, -2.25, 0.5])),
        (SCALE, "center", WidgetInputValue::Point([0.5, 1.5, -0.25])),
        (SCALE, "uniform", WidgetInputValue::Boolean(true)),
        (TRANSLATE, "label", WidgetInputValue::Text("moved".into())),
    ] {
        let applied = applied_and_restored(&change_widget_input(id, channel, input.clone()), &base);
        assert_eq!(params_of(&applied, id).get(channel).and_then(WidgetInputValue::of_literal), Some(input), "{id}.{channel}");
        applied.retire_cold();
    }
    let applied = applied_and_restored(&change_widget_input("note", "text", WidgetInputValue::Text("Edited".into())), &base);
    assert!(applied.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::InputNote { id, text } if id == "note" && text == "Edited")));
    applied.retire_cold();
    base.retire_cold();
}

/// 🎯️ The vocabulary of a widget input: the value it holds is `no-op`; a missing widget `target-missing`; a literal of
/// another type, a wired input, a slider or a text source off its `text` channel `target-mismatch`; an empty or overlong
/// address and a non-finite number Fatal `mutation.invariant`.
#[test]
fn a_widget_input_reports_its_vocabulary_codes() {
    use protocol::Severity::{Error, Fatal, Warning};
    let mut base = base([2.0, 0.0, 0.0]);
    base.host_snapshot.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "wire".into(), from: "height".into(), to: SCALE.into(), from_port: "number".into(), to_port: "factor".into() });
    let mismatch = vec![(Error, "mutation.target-mismatch".to_string())];
    let invariant = vec![(Fatal, "mutation.invariant".to_string())];
    assert_eq!(outcome_codes(&change_widget_input(TRANSLATE, "offset", WidgetInputValue::Vector([2.0, 0.0, 0.0])), &base), vec![(Warning, "mutation.no-op".into())]);
    assert_eq!(outcome_codes(&change_widget_input("ghost", "offset", WidgetInputValue::Number(1.0)), &base), vec![(Error, "mutation.target-missing".into())]);
    assert_eq!(outcome_codes(&change_widget_input(TRANSLATE, "offset", WidgetInputValue::Number(1.0)), &base), mismatch, "a vector input holds no number");
    assert_eq!(outcome_codes(&change_widget_input(SCALE, "factor", WidgetInputValue::Vector([1.0; 3])), &base), mismatch, "a wired input");
    assert_eq!(outcome_codes(&change_widget_input("height", "value", WidgetInputValue::Number(1.0)), &base), mismatch, "a slider has no operator input");
    assert_eq!(outcome_codes(&change_widget_input("note", "value", WidgetInputValue::Text("x".into())), &base), mismatch, "a text source has only its text");
    assert_eq!(outcome_codes(&change_widget_input("note", "text", WidgetInputValue::Number(1.0)), &base), mismatch, "a text source holds text");
    assert_eq!(outcome_codes(&change_widget_input(TRANSLATE, "", WidgetInputValue::Number(1.0)), &base), invariant);
    assert_eq!(outcome_codes(&change_widget_input(TRANSLATE, "x".repeat(257), WidgetInputValue::Number(1.0)), &base), invariant);
    assert_eq!(outcome_codes(&change_widget_input(ROTATE, "angle", WidgetInputValue::Number(f64::NAN)), &base), invariant);
    assert_eq!(outcome_codes(&change_widget_input(SCALE, "center", WidgetInputValue::Point([0.0, f64::INFINITY, 0.0])), &base), invariant);
    base.retire_cold();
}

/// 🧮️ The typed literal round trip the inspector and the diff share: every type's literal reads back as itself, and a
/// literal of no known type reads as nothing.
#[test]
fn every_widget_input_literal_round_trips() {
    for input in [WidgetInputValue::Number(-3.5), WidgetInputValue::Text("t".into()), WidgetInputValue::Boolean(false), WidgetInputValue::Point([1.0, 2.0, 3.0]), WidgetInputValue::Vector([0.0, 0.0, 1.0])] {
        assert_eq!(WidgetInputValue::of_literal(&input.literal()), Some(input.clone()), "{input:?}");
    }
    assert_eq!(WidgetInputValue::of_literal(&generation3d_vector_literal("mesh", [0.0; 3])), None);
}
//#endregion 🎛️WidgetInput

//#region ⏪️TimeTravel
/// ⏪️ Folds `log` onto `base` the way a fresh history would — the oracle an edited history's Report replay must equal.
fn fresh_fold(base: &Generation3dSnapshot, log: &[Generation3dMutation]) -> Generation3dSnapshot {
    let mut state = base.clone();
    for mutation in log {
        apply_generation3d_mutation(&mut state, mutation).expect("the edited log folds");
    }
    state
}

/// ⏪️ Time travel edits ONE gesture leaf of a committed history: the preview base is the state right before it, the
/// Report replay re-applies every downstream gesture onto the edited one and equals a fresh fold of the edited log, and
/// overwrite commits exactly that head — for the relative drag (downstream drag composes onto the edited offset), the
/// absolute slider value, the relative node drag and the absolute widget input (a downstream input overrides it).
#[semio_framework_async_macros::async_test]
async fn an_edited_gesture_leaf_replays_its_downstream_like_a_fresh_fold() {
    use protocol::OpBinary;
    let log = vec![
        drag_transforms(vec![TRANSLATE.into()], [1.0, 0.0, 0.0]),
        change_slider_value("height", 7.5),
        drag_transforms(vec![TRANSLATE.into()], [0.0, 2.0, 0.0]),
        move_nodes(vec!["height".into(), "note".into()], 5.0, 5.0),
        move_nodes(vec!["height".into()], -1.0, 0.0),
        change_widget_input(ROTATE, "angle", WidgetInputValue::Number(0.25)),
        change_widget_input(ROTATE, "angle", WidgetInputValue::Number(0.75)),
    ];
    for (edited_at, edited) in [(0, drag_transforms(vec![TRANSLATE.into()], [3.0, 0.0, -1.0])), (1, change_slider_value("height", 2.5)), (3, move_nodes(vec!["note".into()], 40.0, -12.5)), (5, change_widget_input(ROTATE, "angle", WidgetInputValue::Number(1.5)))] {
        {
            let base = base([1.0, 1.0, 1.0]);
            let mut store = crate::store_fixture::document_store(base.clone()).await;
            for mutation in &log {
                store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("the gesture applies");
            }
            let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
            let target = ids[edited_at].clone();
            let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(target.clone(), protocol::InputReplacement::Input { schema: crate::GENERATION_3D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
            let preview_base = store.state_before(&target, &drafts).expect("the preview base folds");
            let preview = preview_base.as_ref().clone();
            if let Ok(state) = std::sync::Arc::try_unwrap(preview_base) {
                state.retire_cold();
            }
            let before = fresh_fold(&base, &log[..edited_at]);
            assert_eq!(preview, before, "the preview base is the state right before the edited leaf");
            let mut edited_log = log.clone();
            edited_log[edited_at] = edited.clone();
            let fresh = fresh_fold(&base, &edited_log);
            let mut replay = store.begin_report_replay(&drafts, Some(&target)).expect("the replay begins at the edited leaf");
            assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
            let result = replay.finish().expect("a finished replay yields its result");
            assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "an edited gesture never blocks finalizing");
            assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
            store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
            assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
            for state in [preview, before, fresh, base] {
                state.retire_cold();
            }
            crate::store_fixture::close(store);
        }
    }
}

/// 🧯️ A gesture whose target an upstream edit removed reports its vocabulary code downstream instead of aborting the
/// replay: editing the slider leaf to a stranger id makes the replayed slider value Error `mutation.target-missing`,
/// which blocks finalizing until it is edited again or withdrawn.
#[semio_framework_async_macros::async_test]
async fn a_gesture_edited_onto_a_missing_target_blocks_finalizing() {
    use protocol::OpBinary;
    {
        let base = base([0.0; 3]);
        let mut store = crate::store_fixture::document_store(base.clone()).await;
        for mutation in [change_slider_value("height", 7.5), drag_transforms(vec![TRANSLATE.into()], [1.0, 0.0, 0.0])] {
            store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation], description: None, transaction: None }).await.expect("the gesture applies");
        }
        let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
        let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: crate::GENERATION_3D_SCHEMA.into(), payload: change_slider_value("ghost", 7.5).encode_op().expect("encodes") })].into_iter().collect();
        let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins");
        assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
        let result = replay.finish().expect("finished");
        let report = store.replay_report(&result).expect("report");
        assert!(report.blocks_finalize(), "a missing slider blocks finalizing: {report:?}");
        let edited = report.outcomes.iter().find(|outcome| outcome.mutation_id == ids[0]).expect("the edited leaf reports");
        assert!(edited.messages.iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == protocol::Severity::Error), "{edited:?}");
        drop(result);
        base.retire_cold();
        crate::store_fixture::close(store);
    }
}
//#endregion ⏪️TimeTravel
