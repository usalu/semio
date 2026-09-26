//! 🧪️ Fixture `🌦️update-climate/🌧️refuses-a-negative-january-irradiance`.
use crate::{Din18599Diff, Din18599Mutation, Din18599Snapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌦️update-climate/🌧️refuses-a-negative-january-irradiance/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌦️update-climate/🌧️refuses-a-negative-january-irradiance/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌦️update-climate/🌧️refuses-a-negative-january-irradiance/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌦️update-climate/🌧️refuses-a-negative-january-irradiance/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌦️update-climate/🌧️refuses-a-negative-january-irradiance/🎯️outcome/🔣️.json");

fn before() -> Din18599Snapshot { serde_json::from_str(BEFORE).unwrap() }
fn expected_after() -> Din18599Snapshot { serde_json::from_str(AFTER).unwrap() }
fn mutation() -> Din18599Mutation { serde_json::from_str(MUTATION).unwrap() }

#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    let after = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &before()).unwrap();
    assert_eq!(after, expected_after());
    assert_eq!(expected_after(), before());
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).unwrap();
    assert_eq!(outcome["status"], "rejected");
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    assert_eq!(raised.diff(), &Din18599Diff::default());
    assert_eq!(DIFF_ABSENT.len(), 0);
}
