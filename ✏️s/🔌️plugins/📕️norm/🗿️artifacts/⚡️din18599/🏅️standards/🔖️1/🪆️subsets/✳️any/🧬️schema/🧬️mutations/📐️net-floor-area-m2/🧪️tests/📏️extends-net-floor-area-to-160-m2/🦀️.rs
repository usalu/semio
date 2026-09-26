//! 🧪️ Fixture `📐️net-floor-area-m2/📏️extends-net-floor-area-to-160-m2`.
use crate::{Din18599Diff, Din18599Mutation, Din18599Snapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️net-floor-area-m2/📏️extends-net-floor-area-to-160-m2/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️net-floor-area-m2/📏️extends-net-floor-area-to-160-m2/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️net-floor-area-m2/📏️extends-net-floor-area-to-160-m2/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️net-floor-area-m2/📏️extends-net-floor-area-to-160-m2/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐️net-floor-area-m2/📏️extends-net-floor-area-to-160-m2/🎯️outcome/🔣️.json");

fn before() -> Din18599Snapshot { serde_json::from_str(BEFORE).unwrap() }
fn expected_after() -> Din18599Snapshot { serde_json::from_str(AFTER).unwrap() }
fn mutation() -> Din18599Mutation { serde_json::from_str(MUTATION).unwrap() }
fn expected_diff() -> Din18599Diff { serde_json::from_str(DIFF).unwrap() }

#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    let after = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &before()).unwrap();
    assert_eq!(after, expected_after());
}

#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    assert_eq!(raised.diff(), &expected_diff());
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation, &base);
    let mut snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &base).unwrap();
    for step in &<Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(&mutation, &base) {
        let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(step, &snapshot);
        snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &snapshot).unwrap();
    }
    assert_eq!(snapshot, base);
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).unwrap();
    assert_eq!(outcome["status"], "applied");
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din18599Snapshot = serde_json::from_str(text).unwrap();
        assert_eq!(serde_json::to_value(&decoded).unwrap(), serde_json::from_str::<serde_json::Value>(text).unwrap(), "{side}");
    }
    assert_eq!(serde_json::to_value(mutation()).unwrap(), serde_json::from_str::<serde_json::Value>(MUTATION).unwrap());
}
