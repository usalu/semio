//! 🧪️ Committed vector `🛢insert-tank` / `🛢️adds-a-water-tank`: the mutation reaches the committed after-snapshot and its own inverse restores the before-snapshot.
use crate::{En1998Diff, En1998Mutation, En1998Snapshot};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🛢insert-tank/🛢️adds-a-water-tank/🎯️outcome/🔣️.json");
fn before() -> En1998Snapshot { pack::json::from_json_str(BEFORE).expect("before") }
fn mutation() -> En1998Mutation { pack::json::from_json_str(MUTATION).expect("mutation") }
fn apply(mutation: &En1998Mutation, base: &En1998Snapshot) -> En1998Snapshot {
    let raised = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "insert-tank raised {:?}", raised.messages());
    <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(raised.diff(), base).expect("apply")
}
#[test]
fn applies_to_committed_after_and_diff() {
    let base = before();
    let raised = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<En1998Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "insert-tank must move the document");
    assert_eq!(after, pack::json::from_json_str::<En1998Snapshot>(AFTER).expect("after"));
}
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "insert-tank changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}
#[test]
fn declared_outcome_holds() {
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}
