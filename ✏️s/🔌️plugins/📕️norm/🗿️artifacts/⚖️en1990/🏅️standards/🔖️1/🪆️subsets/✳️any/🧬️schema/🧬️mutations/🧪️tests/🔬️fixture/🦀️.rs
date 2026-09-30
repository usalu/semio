//! 🧫️ Every committed `En1990Mutation` specification vector — one canonical case per mutation leaf — held to one law.
//!
//! @see ../../../../🧫️fixtures/🧬️mutations — the committed `(before, mutation, after, diff, outcome)` bundles.
//! @see ../../../../🔮️oracles/🔣️.json — the `en1990-1-any` catalog that registers each bundle.
//! @see ../../../../🧪️tests/⚖️mutate-en1990-1/🥒️.feature — the independent Python reference reading the same bundles.

use crate::diff::En1990Diff;
use crate::{En1990Mutation, En1990Snapshot};
use dsl::ToValue;
use protocol::{Mutation, MutationDiff};

//#region 🧾️Vector
/// 🧾️ One committed vector: the semantic kind it witnesses and its five committed files.
pub(crate) struct Vector {
    pub(crate) kind: &'static str,
    pub(crate) before: &'static str,
    pub(crate) mutation: &'static str,
    pub(crate) after: &'static str,
    pub(crate) diff: &'static str,
    pub(crate) outcome: &'static str,
}

/// 🔣️ A committed file as the independent `serde_json` oracle reads it.
fn committed(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("a committed vector file is JSON")
}

/// 🪞️ A production value as the independent `serde_json` oracle reads its Rust wire.
fn wire<T: ToValue>(value: &T) -> serde_json::Value {
    serde_json::Value::from(value.to_value())
}

/// 🎯️ The same op addressing a position no collection holds, for an op whose payload carries an `index`.
fn out_of_range(op: &En1990Mutation) -> Option<En1990Mutation> {
    let mut payload = serde_json::Value::from(op.payload_value());
    *payload.get_mut("index")? = serde_json::Value::from(u64::from(u32::MAX));
    op.with_payload_value(dsl::DslValue::from(&payload)).ok()
}

/// ⚖️ The law every committed vector obeys: the mutation file is the canonical Rust wire of one `kind` op whose binary frame
/// round-trips; both snapshots and the diff are canonical; production dispatch turns BEFORE into exactly the committed diff
/// under the committed outcome and lands on AFTER; the committed diff alone carries BEFORE to AFTER; the op's own inverse
/// restores BEFORE; and the leaf descriptor declares exactly the outcome classes dispatch reaches from the vector — its own
/// status, `no-op` when re-applying the op to AFTER changes nothing, `rejected` when the op addressing an index no collection
/// holds is refused.
pub(crate) fn assert_vector(vector: Vector) {
    let kind = vector.kind;
    let op: En1990Mutation = store::os_store::test_support::assert_wire_witness(vector.mutation);
    assert_eq!(op.descriptor().semantic_kind, kind, "{kind}: the committed mutation is another kind's op");
    let framed = protocol::OpBinary::encode_op(&op).expect("the op encodes to its binary frame");
    assert_eq!(<En1990Mutation as protocol::OpBinary>::decode_op(&framed).expect("its binary frame decodes"), op, "{kind}: the binary frame does not round-trip");
    let before: En1990Snapshot = pack::json::from_json_str(vector.before).expect("the committed before-snapshot decodes");
    let after: En1990Snapshot = pack::json::from_json_str(vector.after).expect("the committed after-snapshot decodes");
    let delta: En1990Diff = pack::json::from_json_str(vector.diff).expect("the committed diff decodes");
    assert_eq!(wire(&before), committed(vector.before), "{kind}: the committed before-snapshot is not the canonical wire");
    assert_eq!(wire(&after), committed(vector.after), "{kind}: the committed after-snapshot is not the canonical wire");
    assert_eq!(wire(&delta), committed(vector.diff), "{kind}: the committed diff is not the canonical wire");
    let outcome = op.diff(&before);
    assert_eq!(wire(outcome.diff()), committed(vector.diff), "{kind}: production dispatch produces another diff than the committed one");
    let status = committed(vector.outcome)["status"].as_str().expect("the committed outcome names its status").to_string();
    match status.as_str() {
        "applied" => assert!(outcome.messages().is_empty(), "{kind}: an applied vector raises {:?}", outcome.messages()),
        "no-op" => assert_ne!(outcome.worst_level(), Some(protocol::Severity::Fatal), "{kind}: a no-op vector is refused"),
        "rejected" => assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal), "{kind}: a rejected vector is accepted"),
        other => panic!("{kind}: unknown committed outcome status {other:?}"),
    }
    let applied = MutationDiff::apply(outcome.diff(), &before).expect("the produced diff applies to the committed before-snapshot");
    assert_eq!(applied, after, "{kind}: production dispatch does not land on the committed after-snapshot");
    assert_eq!(MutationDiff::apply(&delta, &before).expect("the committed diff applies to the committed before-snapshot"), after, "{kind}: the committed diff does not carry before to after");
    assert_eq!(status == "applied", applied != before, "{kind}: an applied vector must move the document and only an applied one may");
    let inverse = op.inverse(&before);
    assert_eq!(status == "applied", !inverse.is_empty(), "{kind}: an applied vector computes a non-empty inverse and only an applied one does");
    let restored = inverse.iter().fold(applied, |current, step| MutationDiff::apply(step.diff(&current).diff(), &current).expect("an inverse step applies"));
    assert_eq!(restored, before, "{kind}: replaying the inverse does not restore the committed before-snapshot");
    let no_op = op.diff(&after).messages().iter().any(|message| message.code.0 == "mutation.no-op");
    let rejected = out_of_range(&op).is_some_and(|stray| stray.diff(&before).worst_level() >= Some(protocol::Severity::Error));
    let reached: Vec<&str> = [(true, status.as_str()), (no_op, "no-op"), (rejected, "rejected")].into_iter().filter_map(|(reached, class)| reached.then_some(class)).collect();
    let declared: Vec<&str> = op.descriptor().outcome_classes.iter().map(|class| class.as_str()).collect();
    assert_eq!(declared, reached, "{kind}: the descriptor's outcome classes are not the ones production dispatch reaches");
}
//#endregion 🧾️Vector

//#region 🧪️Cases
#[path = "../../🌍️change-annex/🧪️tests/🌍en/🦀️.rs"]
mod change_annex;
#[path = "../../🏷️change-project-id/🧪️tests/📛office-tower-b/🦀️.rs"]
mod change_project_id;
#[path = "../../⛰️change-altitude-m/🧪️tests/🗻950-m/🦀️.rs"]
mod change_altitude_m;
#[path = "../../⚠️change-consequence-class/🧪️tests/🚨cc3/🦀️.rs"]
mod change_consequence_class;
#[path = "../../🎯change-reliability-class/🧪️tests/🎯rc3/🦀️.rs"]
mod change_reliability_class;
#[path = "../../📅change-design-working-life-category/🧪️tests/⏳cat-5/🦀️.rs"]
mod change_design_working_life_category;
#[path = "../../📆change-design-working-life-years/🧪️tests/📆100-y/🦀️.rs"]
mod change_design_working_life_years;
#[path = "../../⏱️change-reference-period-years/🧪️tests/⌛1-y/🦀️.rs"]
mod change_reference_period_years;
#[path = "../../👁️change-supervision-level/🧪️tests/👀dsl3/🦀️.rs"]
mod change_supervision_level;
#[path = "../../🔍change-inspection-level/🧪️tests/🔍il3/🦀️.rs"]
mod change_inspection_level;
#[path = "../../📐change-beta-computed/🧪️tests/📐4-3/🦀️.rs"]
mod change_beta_computed;
#[path = "../../⚓️change-permanents/🧪️tests/⚓95-kn/🦀️.rs"]
mod change_permanents;
#[path = "../../🏋️change-variables/🧪️tests/⛄adds-snow/🦀️.rs"]
mod change_variables;
#[path = "../../💥change-accidentals/🧪️tests/💥75-kn/🦀️.rs"]
mod change_accidentals;
#[path = "../../🌋️change-seismics/🧪️tests/🌋class-iii/🦀️.rs"]
mod change_seismics;
#[path = "../../🏗️change-members/🧪️tests/💪300-kn/🦀️.rs"]
mod change_members;
#[path = "../../🌉change-bridge-sls/🧪️tests/🌉deck-check/🦀️.rs"]
mod change_bridge_sls;
#[path = "../../🔗change-effects/🧪️tests/🔗half-wind/🦀️.rs"]
mod change_effects;
#[path = "../../✂️remove-effect/🧪️tests/🔌e-1/🦀️.rs"]
mod remove_effect;
#[path = "../../🪚remove-member/🧪️tests/🪚beam-b1/🦀️.rs"]
mod remove_member;
#[path = "../../🕳️remove-seismic/🧪️tests/❌e-1/🦀️.rs"]
mod remove_seismic;
#[path = "../../🧯remove-accidental/🧪️tests/🧯impact/🦀️.rs"]
mod remove_accidental;
#[path = "../../📤remove-variable/🧪️tests/📤wind/🦀️.rs"]
mod remove_variable;
#[path = "../../➖remove-permanent/🧪️tests/➖g-inf/🦀️.rs"]
mod remove_permanent;
#[path = "../../📎insert-effect/🧪️tests/📎office-half/🦀️.rs"]
mod insert_effect;
#[path = "../../🔩insert-member/🧪️tests/🔩beam-b2/🦀️.rs"]
mod insert_member;
#[path = "../../🌋insert-seismic/🧪️tests/🌋class-iv/🦀️.rs"]
mod insert_seismic;
#[path = "../../💣insert-accidental/🧪️tests/💣explosion/🦀️.rs"]
mod insert_accidental;
#[path = "../../📥insert-variable/🧪️tests/📥snow/🦀️.rs"]
mod insert_variable;
#[path = "../../➕insert-permanent/🧪️tests/➕finishes/🦀️.rs"]
mod insert_permanent;
//#endregion 🧪️Cases
