//! 🧪️ `replace-load` fixture — `⛔️rejects-a-missing-7a1e1f`.
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
//! ⛔️ The case resolves, the load does not: a `replace-load` addresses BOTH ids and refuses on the first one this base cannot answer.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-load/⛔️rejects-a-missing-7a1e1f/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-load/⛔️rejects-a-missing-7a1e1f/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-load/⛔️rejects-a-missing-7a1e1f/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-load/⛔️rejects-a-missing-7a1e1f/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused `replace-load` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-load/⛔️rejects-a-missing-7a1e1f: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "replace-load/⛔️rejects-a-missing-7a1e1f: a refused mutation must leave the snapshot exactly where it was");
}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem2dDiff::default(), "replace-load/⛔️rejects-a-missing-7a1e1f: a rejecting replace-load must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "replace-load/⛔️rejects-a-missing-7a1e1f: the refusal is reported as mutation.target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing target is an Error, the level a merge policy may still choose to tolerate");
    assert_eq!(messages[0].target, vec!["lw9".to_string()], "the diagnostic addresses the offending id");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "replace-load", "the fixture must be bound to replace-load's own descriptor");
}

/// ↩️ `replace-load`'s inverse is BASE-derived. With no such load on `before` there is nothing to restore, so the inverse collapses to `Vec::new()`.
#[test]
fn inverse_of_the_refused_mutation() {
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "replace-load/⛔️rejects-a-missing-7a1e1f: a target BASE never held leaves nothing to restore, got {inverse:?}");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "replace-load/⛔️rejects-a-missing-7a1e1f declares a rejected outcome");
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
        assert_eq!(reencoded, original, "replace-load/⛔️rejects-a-missing-7a1e1f: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "replace-load/⛔️rejects-a-missing-7a1e1f changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-load/⛔️rejects-a-missing-7a1e1f: committed mutation JSON is not canonical");
}
