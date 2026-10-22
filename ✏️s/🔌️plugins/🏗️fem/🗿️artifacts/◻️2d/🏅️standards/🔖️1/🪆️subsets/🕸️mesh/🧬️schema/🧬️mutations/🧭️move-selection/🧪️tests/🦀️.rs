//! 🤝️ Shared wire fixture for transform undo, collaborator preservation and semantic/diff inverse parity.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::{ColdDiffAlgebra, DiffAlgebra};
use protocol::{ColdMutation, Mutation};
use protocol::MutationDiff;

use protocol::ColdMutationDiff;


fn decode<T: semio_framework_value::FromValue>(value: &serde_json::Value) -> T {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed wire fixture")
}

fn apply(snapshot: &Fem2dSnapshotValue, mutation: &Fem2dMutation) -> Fem2dSnapshotValue {
    let outcome = mutation.diff(snapshot);
    assert!(outcome.messages().iter().all(|message| message.level < semio_framework_diagnostic::Severity::Error), "{outcome:?}");
    protocol::apply_diff_cold(outcome.diff(), snapshot).expect("central diff application")
}

#[test]
fn transform_inverse_preserves_collaborator_fields_and_matches_diff_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("third-party wire parser");
    let before: Fem2dSnapshotValue = decode(&fixture["before"]);
    let mine: Fem2dMutation = decode(&fixture["mine"]);
    let peers: Vec<Fem2dMutation> = decode(&fixture["collaborator"]);
    let inverse = mine.inverse(&before).expect("semantic inverse");
    assert_eq!(inverse, decode::<Vec<Fem2dMutation>>(&fixture["inverse"]));
    let moved = apply(&before, &mine);
    let combined = peers.iter().fold(moved.clone(), |state, step| apply(&state, step));
    let restored = inverse.iter().rev().fold(combined.clone(), |state, step| apply(&state, step));
    let mut oracle = fixture["before"].clone();
    for change in fixture["oracle"].as_array().expect("independent pointer assignments") {
        *oracle.pointer_mut(change["pointer"].as_str().expect("pointer")).expect("existing oracle field") = change["value"].clone();
    }
    assert_eq!(oracle, fixture["expected"], "third-party JSON pointer oracle agrees with the shared outcome");
    assert_eq!(restored, decode::<Fem2dSnapshotValue>(&oracle));
    let negative = DiffAlgebra::inverse(mine.diff(&before).diff(), &before);
    assert_eq!(protocol::apply_diff_cold(&negative, &combined).expect("diff inverse"), restored);
    let mut summed = Fem2dDiff::default();
    let mut cursor = moved;
    for step in inverse.iter().rev() {
        summed.absorb(step.diff(&cursor).diff().clone());
        cursor = apply(&cursor, step);
    }
    assert_eq!(cursor, before);
    assert_eq!(summed, negative);
    println!("[DEBUG] FEM2d transform undo preserves peer node ordinate, region fields and independently authored holes; semantic inverse equals central diff inverse");
}

#[test]
fn transform_non_finite_output_is_refused_before_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("wire fixture");
    let mut before: Fem2dSnapshotValue = decode(&fixture["before"]);
    before.nodes[0].x = fixture["overflow"]["nodeX"].as_f64().expect("finite coordinate");
    let operation: Fem2dMutation = decode(&fixture["overflow"]["mutation"]);
    let outcome = operation.diff(&before);
    assert_eq!(outcome.diff(), &Fem2dDiff::default());
    assert_eq!(outcome.messages()[0].code.0, fixture["overflow"]["expectedCode"].as_str().expect("refusal code"));
    assert!(operation.inverse(&before).expect("refused inverse").is_empty());
    println!("[DEBUG] FEM2d transform overflow refuses before any region or node publication");
}
