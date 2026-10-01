//! 🧫️ Every committed `En1990Mutation` specification vector — one canonical `✅apply` case per mutation leaf, plus each
//! insert's `⛔dupe` refusal and `📏clamp` warning — held to one law.
//!
//! @see ../../../../🧫️fixtures/🧬️mutations — the committed `(before, mutation, after, diff, outcome)` bundles.
//! @see ../../../../🔮️oracles/🔣️.json — the `en1990-1-any` catalog that registers each bundle.
//! @see ../../../../🧪️tests/⚖️mutate-en1990-1/🥒️.feature — the independent Python reference reading the same bundles.

use crate::diff::En1990Diff;
use crate::{En1990Mutation, En1990Snapshot};
use dsl::ToValue;
use protocol::{Mutation, MutationDiff};

//#region 🧾️Vector
/// 🧾️ One committed vector: the semantic kind it witnesses and its committed files — a refused vector commits no diff.
pub(crate) struct Vector {
    pub(crate) kind: &'static str,
    pub(crate) before: &'static str,
    pub(crate) mutation: &'static str,
    pub(crate) after: &'static str,
    pub(crate) diff: Option<&'static str>,
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

/// 🏷️ A committed outcome's messages as `(level, code)` pairs.
fn committed_messages(outcome: &serde_json::Value) -> Vec<(String, String)> {
    let pair = |message: &serde_json::Value| (message["level"].as_str().unwrap_or_default().to_string(), message["code"].as_str().unwrap_or_default().to_string());
    outcome["messages"].as_array().map(|messages| messages.iter().map(pair).collect()).unwrap_or_default()
}

/// ⚖️ The law every committed vector obeys: the mutation file is the canonical Rust wire of one `kind` op whose binary frame
/// round-trips; both snapshots and any committed diff are canonical; production dispatch turns BEFORE into exactly the
/// committed diff under exactly the committed messages — refused at error or fatal level exactly when the vector is
/// `rejected`, which commits no diff — and lands on AFTER, which an `applied` vector moves and no other does; the committed
/// diff alone carries BEFORE to AFTER; an applied op's own inverse restores BEFORE; and the leaf descriptor declares the
/// vector's status and, for an applied vector, exactly the outcome classes dispatch reaches from it — `no-op` when
/// re-applying the op to AFTER raises `mutation.no-op`, `rejected` when that re-application or the op addressing an index
/// no collection holds is refused.
pub(crate) fn assert_vector(vector: Vector) {
    let kind = vector.kind;
    let op: En1990Mutation = store::os_store::test_support::assert_wire_witness(vector.mutation);
    assert_eq!(op.descriptor().semantic_kind, kind, "{kind}: the committed mutation is another kind's op");
    let framed = protocol::OpBinary::encode_op(&op).expect("the op encodes to its binary frame");
    assert_eq!(<En1990Mutation as protocol::OpBinary>::decode_op(&framed).expect("its binary frame decodes"), op, "{kind}: the binary frame does not round-trip");
    let before: En1990Snapshot = pack::json::from_json_str(vector.before).expect("the committed before-snapshot decodes");
    let after: En1990Snapshot = pack::json::from_json_str(vector.after).expect("the committed after-snapshot decodes");
    assert_eq!(wire(&before), committed(vector.before), "{kind}: the committed before-snapshot is not the canonical wire");
    assert_eq!(wire(&after), committed(vector.after), "{kind}: the committed after-snapshot is not the canonical wire");
    let outcome = op.diff(&before);
    let committed_outcome = committed(vector.outcome);
    let status = committed_outcome["status"].as_str().expect("the committed outcome names its status").to_string();
    assert!(["applied", "no-op", "rejected"].contains(&status.as_str()), "{kind}: unknown committed outcome status {status:?}");
    let raised: Vec<(String, String)> = outcome.messages().iter().map(|message| (format!("{:?}", message.level).to_lowercase(), message.code.0.clone())).collect();
    assert_eq!(raised, committed_messages(&committed_outcome), "{kind}: production dispatch raises other messages than the committed outcome");
    let refused = outcome.worst_level() >= Some(protocol::Severity::Error);
    assert_eq!(refused, status == "rejected", "{kind}: a vector is refused exactly when its committed outcome is rejected");
    assert_eq!(vector.diff.is_none(), refused, "{kind}: a vector commits a diff exactly when dispatch does not refuse it");
    if let Some(diff) = vector.diff {
        let delta: En1990Diff = pack::json::from_json_str(diff).expect("the committed diff decodes");
        assert_eq!(wire(&delta), committed(diff), "{kind}: the committed diff is not the canonical wire");
        assert_eq!(wire(outcome.diff()), committed(diff), "{kind}: production dispatch produces another diff than the committed one");
        assert_eq!(MutationDiff::apply(&delta, &before).expect("the committed diff applies to the committed before-snapshot"), after, "{kind}: the committed diff does not carry before to after");
    }
    let applied = MutationDiff::apply(outcome.diff(), &before).expect("the produced diff applies to the committed before-snapshot");
    assert_eq!(applied, after, "{kind}: production dispatch does not land on the committed after-snapshot");
    assert_eq!(status == "applied", applied != before, "{kind}: an applied vector must move the document and only an applied one may");
    if status == "applied" {
        let inverse = op.inverse(&before);
        assert!(!inverse.is_empty(), "{kind}: an applied vector computes a non-empty inverse");
        let restored = inverse.iter().fold(applied, |current, step| MutationDiff::apply(step.diff(&current).diff(), &current).expect("an inverse step applies"));
        assert_eq!(restored, before, "{kind}: replaying the inverse does not restore the committed before-snapshot");
    }
    let again = op.diff(&after);
    let no_op = again.messages().iter().any(|message| message.code.0 == "mutation.no-op");
    let rejected = again.worst_level() >= Some(protocol::Severity::Error) || out_of_range(&op).is_some_and(|stray| stray.diff(&before).worst_level() >= Some(protocol::Severity::Error));
    let reached: Vec<&str> = [(true, status.as_str()), (no_op, "no-op"), (rejected, "rejected")].into_iter().filter_map(|(reached, class)| reached.then_some(class)).collect();
    let declared: Vec<&str> = op.descriptor().outcome_classes.iter().map(|class| class.as_str()).collect();
    if status == "applied" {
        assert_eq!(declared, reached, "{kind}: the descriptor's outcome classes are not the ones production dispatch reaches");
    } else {
        assert!(declared.contains(&status.as_str()), "{kind}: the descriptor does not declare the committed {status:?} outcome");
    }
}
//#endregion 🧾️Vector

//#region 🧪️Cases
#[path = "../../🌍️change-annex/🧪️tests/✅apply/🦀️.rs"]
mod change_annex;
#[path = "../../🏷️change-project-id/🧪️tests/✅apply/🦀️.rs"]
mod change_project_id;
#[path = "../../⛰️change-altitude-m/🧪️tests/✅apply/🦀️.rs"]
mod change_altitude_m;
#[path = "../../⚠️change-consequence-class/🧪️tests/✅apply/🦀️.rs"]
mod change_consequence_class;
#[path = "../../🎯change-reliability-class/🧪️tests/✅apply/🦀️.rs"]
mod change_reliability_class;
#[path = "../../📅change-design-working-life-category/🧪️tests/✅apply/🦀️.rs"]
mod change_design_working_life_category;
#[path = "../../📆change-design-working-life-years/🧪️tests/✅apply/🦀️.rs"]
mod change_design_working_life_years;
#[path = "../../⏱️change-reference-period-years/🧪️tests/✅apply/🦀️.rs"]
mod change_reference_period_years;
#[path = "../../👁️change-supervision-level/🧪️tests/✅apply/🦀️.rs"]
mod change_supervision_level;
#[path = "../../🔍change-inspection-level/🧪️tests/✅apply/🦀️.rs"]
mod change_inspection_level;
#[path = "../../📐change-beta-computed/🧪️tests/✅apply/🦀️.rs"]
mod change_beta_computed;
#[path = "../../⚓️change-permanents/🧪️tests/✅apply/🦀️.rs"]
mod change_permanents;
#[path = "../../🏋️change-variables/🧪️tests/✅apply/🦀️.rs"]
mod change_variables;
#[path = "../../💥change-accidentals/🧪️tests/✅apply/🦀️.rs"]
mod change_accidentals;
#[path = "../../🌋️change-seismics/🧪️tests/✅apply/🦀️.rs"]
mod change_seismics;
#[path = "../../🏗️change-members/🧪️tests/✅apply/🦀️.rs"]
mod change_members;
#[path = "../../🌉change-bridge-sls/🧪️tests/✅apply/🦀️.rs"]
mod change_bridge_sls;
#[path = "../../🔗change-effects/🧪️tests/✅apply/🦀️.rs"]
mod change_effects;
#[path = "../../✂️remove-effect/🧪️tests/✅apply/🦀️.rs"]
mod remove_effect;
#[path = "../../🪚remove-member/🧪️tests/✅apply/🦀️.rs"]
mod remove_member;
#[path = "../../🕳️remove-seismic/🧪️tests/✅apply/🦀️.rs"]
mod remove_seismic;
#[path = "../../🧯remove-accidental/🧪️tests/✅apply/🦀️.rs"]
mod remove_accidental;
#[path = "../../📤remove-variable/🧪️tests/✅apply/🦀️.rs"]
mod remove_variable;
#[path = "../../➖remove-permanent/🧪️tests/✅apply/🦀️.rs"]
mod remove_permanent;
#[path = "../../📎insert-effect/🧪️tests/✅apply/🦀️.rs"]
mod insert_effect;
#[path = "../../🔩insert-member/🧪️tests/✅apply/🦀️.rs"]
mod insert_member;
#[path = "../../🌋insert-seismic/🧪️tests/✅apply/🦀️.rs"]
mod insert_seismic;
#[path = "../../💣insert-accidental/🧪️tests/✅apply/🦀️.rs"]
mod insert_accidental;
#[path = "../../📥insert-variable/🧪️tests/✅apply/🦀️.rs"]
mod insert_variable;
#[path = "../../➕insert-permanent/🧪️tests/✅apply/🦀️.rs"]
mod insert_permanent;
#[path = "../../➕insert-permanent/🧪️tests/⛔dupe/🦀️.rs"]
mod insert_permanent_dupe;
#[path = "../../➕insert-permanent/🧪️tests/📏clamp/🦀️.rs"]
mod insert_permanent_clamp;
#[path = "../../🌋insert-seismic/🧪️tests/⛔dupe/🦀️.rs"]
mod insert_seismic_dupe;
#[path = "../../🌋insert-seismic/🧪️tests/📏clamp/🦀️.rs"]
mod insert_seismic_clamp;
#[path = "../../💣insert-accidental/🧪️tests/⛔dupe/🦀️.rs"]
mod insert_accidental_dupe;
#[path = "../../💣insert-accidental/🧪️tests/📏clamp/🦀️.rs"]
mod insert_accidental_clamp;
#[path = "../../📎insert-effect/🧪️tests/📏clamp/🦀️.rs"]
mod insert_effect_clamp;
#[path = "../../📥insert-variable/🧪️tests/⛔dupe/🦀️.rs"]
mod insert_variable_dupe;
#[path = "../../📥insert-variable/🧪️tests/📏clamp/🦀️.rs"]
mod insert_variable_clamp;
#[path = "../../🔩insert-member/🧪️tests/⛔dupe/🦀️.rs"]
mod insert_member_dupe;
#[path = "../../🔩insert-member/🧪️tests/📏clamp/🦀️.rs"]
mod insert_member_clamp;
//#endregion 🧪️Cases
