//! 🧪️ `add-load` fixture — `🚨️no-such-member-3fe6e9`.
//!
//! Source of truth is the committed JSON bundle beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏭️ `⬅️before` is the GLULAM WORKSHOP HALL, the second real fem3d model this artifact carries
//! (ticket `26/09/06/FEM-PLUGIN-END-TO-END`): a 12 m span × 12 m long two-bay GL24h portal hall,
//! eaves at 4.2 m and ridge at 6.5 m, braced by M24 steel rods and standing on a 350 mm C25/30
//! raft slab with a machine pit. Every quantity is SI — Pa, m, m², m⁴, kg/m³, N, N/m.
//!
//! This is finding 1 of W11's report turned into a refusal. `create-load-case` always resolved a carried load's
//! target; `add-load` accepted the same payload against a member that does not exist. Both verbs now resolve it
//! the same way.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::Fem3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️add-load/🚨️no-such-member-3fe6e9/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️add-load/🚨️no-such-member-3fe6e9/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️add-load/🚨️no-such-member-3fe6e9/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️add-load/🚨️no-such-member-3fe6e9/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A refused mutation leaves the document byte-identical to the committed `after`, which is the
/// committed `before` again. `vcs::apply_mutation` is deliberately policy-agnostic — it applies the
/// (empty) diff and returns `Ok`, so REJECTION IS NOT VISIBLE IN THE RESULT, only in the messages.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "add-load/no-such-member-3fe6e9: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "add-load/no-such-member-3fe6e9: a refused mutation must leave every one of the nine members untouched");
}

/// 🚨️ The branch this vector pins, level and address together.
#[test]
fn the_refusal_is_the_declared_diagnostic() {
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "add-load/no-such-member-3fe6e9: a refused mutation must carry the empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "add-load/no-such-member-3fe6e9: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "add-load/no-such-member-3fe6e9: the refusal is reported as mutation.target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "add-load/no-such-member-3fe6e9: a missed target is an Error, not the Fatal a duplicate identity raises");
    assert_eq!(messages[0].target, vec!["raf_l_9".to_string()], "add-load/no-such-member-3fe6e9: the diagnostic addresses exactly \"raf_l_9\"");
}

/// ↩️ The inverse is computed from `before` and the mutation payload alone, never from the verdict,
/// so a refused request still has the one well-formed undo step its kind always emits.
#[test]
fn inverse_has_the_declared_shape() {
    let inverse = inverse_fem3d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "add-load/no-such-member-3fe6e9: this kind always emits exactly one undo step, got {inverse:?}");
    assert!(matches!(inverse[0], Fem3dMutation::RemoveLoad(_)), "add-load/no-such-member-3fe6e9: the inverse of add-load is a RemoveLoad, got {:?}", inverse[0]);
}

/// 🎯️ The declared outcome — status, code, level and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "add-load/no-such-member-3fe6e9: this vector declares a rejected outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "add-load/no-such-member-3fe6e9: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "add-load/no-such-member-3fe6e9: the declared path must match the emitted target");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-load/no-such-member-3fe6e9: committed {label} JSON is not canonical");
    }
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-load/no-such-member-3fe6e9: committed mutation JSON is not canonical");
}

/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("add", "load", "add-load", "AddedLoad"), "add-load/no-such-member-3fe6e9: the fixture must be bound to add-load's own descriptor");
}
