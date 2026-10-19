//! 🧪️ `create-material` fixture — `🚫️rejects`.
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
//! 🚫️ An id already in the table is FATAL, whatever the payload's other fields say — the guard compares identity only.

use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::inverse_fem2d_mutation_cold;
use semio_s_artifact_fem_2d_mutations::central_apply::apply_fem2d_mutation;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

static BEFORE: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-material/🚫️rejects/📸️snapshot/⬅️before/🔣️.json");
static AFTER: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-material/🚫️rejects/📸️snapshot/➡️after/🔣️.json");
static MUTATION: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-material/🚫️rejects/🦠️mutation/🔣️.json");
static OUTCOME: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-material/🚫️rejects/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshotValue {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshotValue {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ A refused `create-material` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-material/rejects-a-duplicate-f3220b: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "create-material/rejects-a-duplicate-f3220b: a refused mutation must leave the snapshot exactly where it was");
}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshotValue>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff::default(), "create-material/rejects-a-duplicate-f3220b: a rejecting create-material must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "create-material/rejects-a-duplicate-f3220b: the refusal is reported as mutation.duplicate-id");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Fatal, "a duplicate identity is an invariant breach — Fatal, and no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["steel_s355".to_string()], "the diagnostic addresses the offending id");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshotValue>>::semantics(&mutation());
    assert_eq!(semantics.kind, "create-material", "the fixture must be bound to create-material's own descriptor");
}

/// ↩️ A refused aggregate mutation has no executable inverse and preserves its fixture snapshot.
#[test]
fn inverse_of_the_refused_mutation() {
    let base = before();
    let operation = mutation();
    let inverse = inverse_fem2d_mutation_cold(&base, &operation).expect("aggregate inverse admission");
    assert!(inverse.is_empty(), "refused mutations cannot publish inverse candidates: {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &operation).expect("unchanged forward outcome");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("admitted inverse step");
    }
    assert_eq!(snapshot, expected_after());
    assert_eq!(snapshot, base);
    println!("[DEBUG] FEM2d refused fixture retains its snapshot with an empty aggregate inverse");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(OUTCOME.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_value::DslValue::as_str), Some("rejected"), "create-material/rejects-a-duplicate-f3220b declares a rejected outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshotValue>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_value::DslValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared: Vec<String> = outcome.get("path").and_then(semio_framework_value::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared, message.target, "the declared path must match the emitted target");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshotValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-material/rejects-a-duplicate-f3220b: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE.as_str(), AFTER.as_str(), "create-material/rejects-a-duplicate-f3220b changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-material/rejects-a-duplicate-f3220b: committed mutation JSON is not canonical");
}
