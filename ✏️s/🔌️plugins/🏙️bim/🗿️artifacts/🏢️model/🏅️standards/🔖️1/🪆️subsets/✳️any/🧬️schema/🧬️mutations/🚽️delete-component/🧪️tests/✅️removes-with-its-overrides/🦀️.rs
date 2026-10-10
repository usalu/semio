//! ✅️ `delete-component` / `removes-with-its-overrides`: applied. Source of truth: the committed fixture quintet.

use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};

const CASE: Case = Case {
    dir: "🚽️delete-component/✅️removes-with-its-overrides",
    before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚽️delete-component/✅️removes-with-its-overrides/📸️snapshot/⬅️before/🔣️.json"),
    after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚽️delete-component/✅️removes-with-its-overrides/📸️snapshot/➡️after/🔣️.json"),
    mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚽️delete-component/✅️removes-with-its-overrides/🦠️mutation/🔣️.json"),
    diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚽️delete-component/✅️removes-with-its-overrides/🔺️diff/🔣️.json"),
    outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚽️delete-component/✅️removes-with-its-overrides/🎯️outcome/🔣️.json"),
};

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    kit::outcome(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_after_snapshot() {
    kit::applies(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() {
    kit::produces_diff(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_before_snapshot() {
    kit::inverse_restores(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    kit::canonical(&CASE);
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&kit::mutation(&CASE), &kit::before(&CASE)).await;
}
