//! 🧪️ `🔎introduce-product-index` fixture — `🔎appends-a-pr600-index`.

use crate::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎introduce-product-index/🔎appends-a-pr600-index/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎introduce-product-index/🔎appends-a-pr600-index/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎introduce-product-index/🔎appends-a-pr600-index/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎introduce-product-index/🔎appends-a-pr600-index/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎introduce-product-index/🔎appends-a-pr600-index/🎯️outcome/🔣️.json");

fn before() -> Iso16757Snapshot {
    serde_json::from_str(BEFORE).expect("before")
}
fn expected_after() -> Iso16757Snapshot {
    serde_json::from_str(AFTER).expect("after")
}
fn mutation() -> Iso16757Mutation {
    serde_json::from_str(MUTATION).expect("mutation")
}
fn built_outcome() -> protocol::MutationOutcome<Iso16757Diff> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation(), &before())
}

#[semio_framework_async_macros::async_test]
async fn appends_a_pr600_index() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("apply");
    assert_eq!(applied, expected_after());
    assert_eq!(applied.catalogue.product_indexes.len(), before().catalogue.product_indexes.len() + 1);
    assert!(applied.catalogue.product_indexes.iter().any(|x| x.id == "index-pr600"));
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Iso16757Snapshot = serde_json::from_str(text).unwrap();
        let reencoded = serde_json::to_value(&decoded).unwrap();
        let original: serde_json::Value = serde_json::from_str(text).unwrap();
        assert_eq!(reencoded, original, "{side} not canonical");
    }
    assert_eq!(serde_json::to_value(mutation()).unwrap(), serde_json::from_str::<serde_json::Value>(MUTATION).unwrap());
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).unwrap();
    assert_eq!(declared["status"], "applied");
    assert_eq!(built_outcome().worst_level(), None);
}

#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).unwrap();
    let committed: serde_json::Value = serde_json::from_str(DIFF).unwrap();
    assert_eq!(produced, committed);
}

#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Iso16757Diff = serde_json::from_str(DIFF).unwrap();
    let produced = protocol::MutationDiff::apply(&decoded, &before()).unwrap();
    assert_eq!(produced, expected_after());
}
