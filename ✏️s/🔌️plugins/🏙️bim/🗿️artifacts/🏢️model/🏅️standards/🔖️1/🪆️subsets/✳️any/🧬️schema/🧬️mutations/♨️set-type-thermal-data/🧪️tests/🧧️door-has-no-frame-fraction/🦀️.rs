//! 🧧️ `set-type-thermal-data` / `door-has-no-frame-fraction`: rejected. Source of truth: the committed fixture quintet.

use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};

const CASE: Case = Case {
    dir: "♨️set-type-thermal-data/🧧️door-has-no-frame-fraction",
    before: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️set-type-thermal-data/🧧️door-has-no-frame-fraction/📸️snapshot/⬅️before/🔣️.json"),
    after: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️set-type-thermal-data/🧧️door-has-no-frame-fraction/📸️snapshot/➡️after/🔣️.json"),
    mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️set-type-thermal-data/🧧️door-has-no-frame-fraction/🦠️mutation/🔣️.json"),
    diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️set-type-thermal-data/🧧️door-has-no-frame-fraction/🔺️diff/🔣️.json"),
    outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️set-type-thermal-data/🧧️door-has-no-frame-fraction/🎯️outcome/🔣️.json"),
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
