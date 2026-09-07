//! 🧪️ `change-load-case-self-weight` fixture — `🔁️keeps-self-weight-ff696b`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units.
//!
//! 🔁️ The dead case already carries self-weight, so this is `change-load-case-self-weight`'s no-op branch: APPLIED with a Warning, an empty diff, and a document that does not move. `change-load-case-self-weight` has no Fatal branch at all.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A no-op `change-load-case-self-weight` is APPLIED, not rejected — it simply changes nothing, so the document comes
/// out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn no_op_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("change-load-case-self-weight's no-op diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-load-case-self-weight/keeps-self-weight-ff696b: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "change-load-case-self-weight/keeps-self-weight-ff696b: an APPLIED no-op still leaves the document exactly where it was");
}

/// ⚠️ A value that is already what the payload asks for is a Warning-level `mutation.no-op`, never
/// an Error and never a Fatal — the mutation applies, it just carries no change.
#[test]
fn the_no_op_is_a_warning_not_a_rejection() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem2d::diff::Fem2dDiff::default(), "change-load-case-self-weight/keeps-self-weight-ff696b: a no-op change-load-case-self-weight must carry the empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "change-load-case-self-weight/keeps-self-weight-ff696b: an unchanged value is reported as no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning — the mutation still APPLIES, it simply changes nothing");
    assert!(messages[0].target.is_empty(), "change-load-case-self-weight raises its no-op through the 2-arg `warn` builder, which attaches no target address");
}

/// ↩️ `change-load-case-self-weight` inverts from BASE: the case exists, so the inverse is a step restoring the flag it already had.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-load-case-self-weight/keeps-self-weight-ff696b: change-load-case-self-weight always emits exactly one inverse step, even for a no-op, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-load-case-self-weight/keeps-self-weight-ff696b: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — applied, with exactly one `warn`-level `mutation.no-op` — is what this
/// kind really emits here.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "change-load-case-self-weight/keeps-self-weight-ff696b declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    let declared = outcome.get("messages").and_then(dsl::DslValue::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), produced.messages().len(), "the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(dsl::DslValue::as_str), Some("warn"), "change-load-case-self-weight's no-op is declared at warn level");
    assert_eq!(declared[0].get("code").and_then(dsl::DslValue::as_str), Some(produced.messages()[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-load-case-self-weight/keeps-self-weight-ff696b: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "change-load-case-self-weight/keeps-self-weight-ff696b changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-load-case-self-weight/keeps-self-weight-ff696b: committed mutation JSON is not canonical");
}

/// 🔺️ A no-op produces the artifact's `Default` diff — all seventeen sparse slots left `None`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-load-case-self-weight/keeps-self-weight-ff696b: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes into Fem2dDiff");
    assert_eq!(typed, crate::artifacts::fem2d::diff::Fem2dDiff::default(), "change-load-case-self-weight/keeps-self-weight-ff696b: a no-op delta is the artifact's Default diff");
}

/// 🔣️ The committed diff is itself canonical. `Fem2dDiff` carries a container-level `default` and no
/// per-field skip, so all seventeen sparse slots must be present as `null`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-load-case-self-weight/keeps-self-weight-ff696b: committed diff JSON is not canonical");
    let slots = original.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 17, "Fem2dDiff emits all seventeen sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. For a no-op that
/// is the identity — and still a real assertion: `apply` must leave every other member alone too.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-load-case-self-weight/keeps-self-weight-ff696b: committed diff did not carry before to after");
}
