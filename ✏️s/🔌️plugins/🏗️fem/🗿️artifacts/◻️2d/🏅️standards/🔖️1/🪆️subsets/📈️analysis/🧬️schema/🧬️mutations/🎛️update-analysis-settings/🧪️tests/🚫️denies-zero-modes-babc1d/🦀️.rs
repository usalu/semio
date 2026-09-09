//! 🧪️ `update-analysis-settings` fixture — `🚫️denies-zero-modes-babc1d`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening).
//! Every value is in SI base units.
//!
//! 🛡️ This vector pins a branch the 26/09/06/FEM-PLUGIN-END-TO-END hardening wave ADDED; before it
//! the payload below was accepted (see `📓️w13-fem2d-semantics.md` for the per-kind rule table).
//!
//! 🚫️ Before this wave `update-analysis-settings` had NO rejection branch at all — its only guard was
//! the equality no-op, so zero modes, four billion modes and a negative deformation scale were all
//! accepted unconditionally (W10 finding F1). It now carries bounds, and this is the first vector in
//! the corpus that pins them. The analysis facet has no id, so the diagnostic's target is empty.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🚫️denies-zero-modes-babc1d/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🚫️denies-zero-modes-babc1d/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🚫️denies-zero-modes-babc1d/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🚫️denies-zero-modes-babc1d/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `update-analysis-settings` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "update-analysis-settings/denies-zero-modes-babc1d: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "update-analysis-settings/denies-zero-modes-babc1d: a refused mutation must leave the snapshot exactly where it was");
}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem2dDiff::default(), "update-analysis-settings/denies-zero-modes-babc1d: a rejecting update-analysis-settings must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "update-analysis-settings/denies-zero-modes-babc1d: the refusal is reported as mutation.invariant");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "an inadmissible payload is wrong against every base, not just this one — Fatal");
    assert_eq!(messages[0].target, Vec::<String>::new(), "the analysis facet has no id, so the diagnostic carries no address");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "update-analysis-settings", "the fixture must be bound to update-analysis-settings's own descriptor");
}

/// ↩️ `update-analysis-settings` ALWAYS emits exactly one inverse step carrying `base.analysis`, refused or not — it has no branch that can collapse to `Vec::new()`.
#[test]
fn inverse_of_the_refused_mutation() {
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "update-analysis-settings/denies-zero-modes-babc1d: update-analysis-settings undoes with exactly one step, got {inverse:?}");
    let Fem2dMutation::UpdateAnalysisSettings(undo) = &inverse[0] else {
        panic!("update-analysis-settings's inverse must be a UpdateAnalysisSettings, got {:?}", inverse[0]);
    };
    assert_eq!(undo.settings, before().analysis, "the inverse carries the settings the document already had");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "update-analysis-settings/denies-zero-modes-babc1d declares a rejected outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared, message.target, "the declared path must match the emitted target");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-analysis-settings/denies-zero-modes-babc1d: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "update-analysis-settings/denies-zero-modes-babc1d changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-analysis-settings/denies-zero-modes-babc1d: committed mutation JSON is not canonical");
}
