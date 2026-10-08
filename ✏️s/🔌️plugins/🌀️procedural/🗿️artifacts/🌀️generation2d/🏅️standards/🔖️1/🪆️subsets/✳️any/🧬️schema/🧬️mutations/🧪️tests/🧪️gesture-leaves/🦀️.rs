//! 🧪️ Laws of the generation2d gesture leaves (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, design §5, §13): the
//! ABSOLUTE `change-slider-value` a slider press commits and the RELATIVE `move-nodes` a node drag commits — outcome codes
//! from the frozen vocabulary, exact inverses, labels in English and German, wire witnesses, and time travel: an edited
//! gesture leaf replays its downstream exactly like a fresh fold of the edited log.

use super::*;
use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;
use semio_framework_artifact_flow_flow::{Widget, WidgetLayout};

/// 🧱️ A base with one slider and one note, both placed, and one unplaced slider.
fn base() -> Generation2dSnapshot {
    let mut snapshot = empty_generation2d_snapshot();
    snapshot.host_snapshot.widgets.push(Widget::InputSlider { id: "height".into(), label: "Height".into(), value: 6.0, min: 0.0, max: 10.0, step: 0.5 });
    snapshot.host_snapshot.widgets.push(Widget::InputNote { id: "note".into(), text: "Note".into() });
    snapshot.host_snapshot.widgets.push(Widget::InputSlider { id: "loose".into(), label: "Loose".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.1 });
    for (id, x, y) in [("height", 10.0, 20.0), ("note", -5.0, 0.0)] {
        let _ = snapshot.host_snapshot.layout.insert(id.to_string(), WidgetLayout { x, y });
    }
    snapshot
}

fn outcome_codes(mutation: &Generation2dMutation, base: &Generation2dSnapshot) -> Vec<(semio_framework_diagnostic::Severity, String)> {
    let (delta, messages) = Mutation::diff(mutation, base).into_parts();
    delta.retire_cold();
    messages.into_iter().map(|message| (message.level, message.code.0)).collect()
}

/// ♻️ Applies `mutation` to a copy of `base`, then its inverse, and answers the applied copy; the inverse must restore
/// `base` exactly.
fn applied_and_restored(mutation: &Generation2dMutation, base: &Generation2dSnapshot) -> Generation2dSnapshot {
    let inverse = inverse_generation2d_mutation(base, mutation).expect("valid retained mutation inverse fixture");
    let mut applied = base.clone();
    apply_generation2d_mutation(&mut applied, mutation).expect("the gesture leaf applies to its base");
    let mut restored = applied.clone();
    for step in &inverse {
        apply_generation2d_mutation(&mut restored, step).expect("every inverse row applies");
    }
    assert_eq!(&restored, base, "the gesture leaf inverts exactly to its base");
    restored.retire_cold();
    applied
}

fn slider(snapshot: &Generation2dSnapshot, id: &str) -> Option<(f64, (f64, f64))> {
    snapshot.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id: widget_id, value, min, max, .. } if widget_id == id => Some((*value, (*min, *max))),
        _ => None,
    })
}

/// 🎚️ A slider value is absolute: in range it sets the value, out of range it widens the range like the canvas knob; the
/// value it holds is `no-op`, a non-slider `target-mismatch`, a missing widget `target-missing`, a non-finite value Fatal
/// `mutation.invariant`; the inverse restores the base slider whole.
#[test]
fn a_slider_value_is_absolute_and_inverts_to_the_whole_base_slider() {
    use semio_framework_diagnostic::Severity::Error;
use semio_framework_diagnostic::Severity::Fatal;
use semio_framework_diagnostic::Severity::Warning;
    let base = base();
    for (value, range) in [(7.5, (0.0, 10.0)), (42.0, (0.0, 50.0))] {
        let applied = applied_and_restored(&change_slider_value("height", value), &base);
        assert_eq!(slider(&applied, "height"), Some((value, range)));
        applied.retire_cold();
    }
    assert_eq!(outcome_codes(&change_slider_value("height", 6.0), &base), vec![(Warning, "mutation.no-op".into())]);
    assert_eq!(outcome_codes(&change_slider_value("note", 1.0), &base), vec![(Error, "mutation.target-mismatch".into())]);
    assert_eq!(outcome_codes(&change_slider_value("ghost", 1.0), &base), vec![(Error, "mutation.target-missing".into())]);
    assert_eq!(outcome_codes(&change_slider_value("height", f64::NAN), &base), vec![(Fatal, "mutation.invariant".into())]);
    base.retire_cold();
}

/// 🚚️ A node drag moves every placed node by its offset from the BASE position and inverts to the absolute base
/// positions; a missing or unplaced node is skipped with `mutation.partial`, none left is `target-missing` /
/// `target-mismatch`, a zero offset `mutation.no-op`, a repeated or empty target list Fatal `mutation.invariant`.
#[test]
fn a_node_drag_moves_placed_nodes_relative_to_their_base_position() {
    use semio_framework_diagnostic::Severity::Error;
use semio_framework_diagnostic::Severity::Fatal;
use semio_framework_diagnostic::Severity::Warning;
use crate::central_apply::{apply_generation2d_mutation};
    let base = base();
    let applied = applied_and_restored(&move_nodes(vec!["height".into(), "note".into()], 40.0, -12.5), &base);
    assert_eq!(applied.host_snapshot.layout.get("height").map(|layout| (layout.x, layout.y)), Some((50.0, 7.5)));
    assert_eq!(applied.host_snapshot.layout.get("note").map(|layout| (layout.x, layout.y)), Some((35.0, -12.5)));
    applied.retire_cold();
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into(), "ghost".into()], 1.0, 1.0), &base), vec![(Warning, "mutation.partial".into())]);
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into(), "loose".into()], 1.0, 1.0), &base), vec![(Warning, "mutation.partial".into())]);
    assert_eq!(outcome_codes(&move_nodes(vec!["ghost".into()], 1.0, 1.0), &base), vec![(Error, "mutation.target-missing".into())]);
    assert_eq!(outcome_codes(&move_nodes(vec!["loose".into()], 1.0, 1.0), &base), vec![(Error, "mutation.target-mismatch".into())]);
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into()], 0.0, 0.0), &base), vec![(Warning, "mutation.no-op".into())]);
    assert_eq!(outcome_codes(&move_nodes(vec!["height".into(), "height".into()], 1.0, 1.0), &base), vec![(Fatal, "mutation.invariant".into())]);
    assert_eq!(outcome_codes(&move_nodes(Vec::new(), 1.0, 1.0), &base), vec![(Fatal, "mutation.invariant".into())]);
    assert!(inverse_generation2d_mutation(&base, &move_nodes(vec!["height".into()], 0.0, 0.0)).expect("valid retained mutation inverse fixture").is_empty(), "an identity drag owes no inverse");
    base.retire_cold();
}

/// 🏷️ Both gesture leaves label their history rows in English and German (decimal comma).
#[test]
fn gesture_leaves_label_their_rows_in_english_and_german() {
    for (mutation, english, german) in [
        (change_slider_value("height", 7.5), "Set slider \"height\" to 7.5", "Schieberegler \"height\" auf 7,5 setzen"),
        (move_nodes(vec!["a".into(), "b".into()], 40.0, -12.5), "Move 2 node(s) by (40, -12.5)", "2 Knoten um (40; -12,5) verschieben"),
    ] {
        let label = <Generation2dMutation as protocol::SemanticMutation<Generation2dSnapshot>>::label(&mutation);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), english);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), german);
    }
}

/// 🧾️ Every committed wire witness decodes to its leaf and round-trips the binary and text op codecs unchanged.
#[test]
fn every_wire_witness_decodes_and_round_trips_both_codecs() {
    let witnesses = [
        (include_str!("../../../../🧫️fixtures/🧬️mutations/🎚️change-slider-value/🧾️wire-witness/🦠️mutation/🔣️.json"), "change-slider-value"),
        (include_str!("../../../../🧫️fixtures/🧬️mutations/🚚️move-nodes/🧾️wire-witness/🦠️mutation/🔣️.json"), "move-nodes"),
    ];
    for (witness, kind) in witnesses {
        let mutation: Generation2dMutation = semio_framework_pack_json::from_json_str(witness, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{kind} witness decodes: {error}"));
        assert_eq!(<Generation2dMutation as protocol::SemanticMutation<Generation2dSnapshot>>::semantics(&mutation).kind, kind);
        let bytes = protocol::OpBinary::encode_op(&mutation).expect("witness encodes");
        assert_eq!(<Generation2dMutation as protocol::OpBinary>::decode_op(&bytes).expect("witness decodes back"), mutation, "{kind} round-trips the binary codec");
        let text = protocol::OpText::print_op(&mutation);
        assert_eq!(<Generation2dMutation as protocol::OpText>::parse_op(&text).expect("witness parses back"), mutation, "{kind} round-trips the text codec: {text}");
    }
}

//#region ⏪️TimeTravel
/// ⏪️ Folds `log` onto `base` the way a fresh history would — the oracle an edited history's Report replay must equal.
fn fresh_fold(base: &Generation2dSnapshot, log: &[Generation2dMutation]) -> Generation2dSnapshot {
    let mut state = base.clone();
    for mutation in log {
        apply_generation2d_mutation(&mut state, mutation).expect("the edited log folds");
    }
    state
}

/// ⏪️ Time travel edits ONE gesture leaf of a committed history: the preview base is the state right before it, the
/// Report replay re-applies every downstream gesture onto the edited one and equals a fresh fold of the edited log, and
/// overwrite commits exactly that head.
#[semio_framework_async_macros::async_test]
async fn an_edited_gesture_leaf_replays_its_downstream_like_a_fresh_fold() {
    use protocol::OpBinary;
    let log = vec![move_nodes(vec!["height".into()], 5.0, 5.0), change_slider_value("height", 7.5), move_nodes(vec!["height".into(), "note".into()], -1.0, 2.0)];
    for (edited_at, edited) in [(0, move_nodes(vec!["height".into(), "note".into()], 40.0, -12.5)), (1, change_slider_value("height", 2.5))] {
        let base = base();
        let mut store = crate::store_fixture::document_store(base.clone()).await;
        for mutation in &log {
            store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }).await.expect("the gesture applies");
        }
        let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
        let target = ids[edited_at].clone();
        let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(target.clone(), protocol::InputReplacement::Input { schema: crate::GENERATION_2D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
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
//#endregion ⏪️TimeTravel
