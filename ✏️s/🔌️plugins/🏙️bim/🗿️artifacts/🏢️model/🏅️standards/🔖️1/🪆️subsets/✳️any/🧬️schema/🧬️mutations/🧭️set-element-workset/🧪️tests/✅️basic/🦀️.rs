//! 🧪️ Language-neutral WP15 fixture and concrete inverse law.
use crate::standards::v1::subsets::any::schema::mutations::kit::{self, Case};
const CASE: Case = Case {
    dir: "🧭️set-element-workset/✅️basic",
    before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️set-element-workset/✅️basic/📸️snapshot/⬅️before/🔣️.json"),
    after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️set-element-workset/✅️basic/📸️snapshot/➡️after/🔣️.json"),
    mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️set-element-workset/✅️basic/🦠️mutation/🔣️.json"),
    diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️set-element-workset/✅️basic/🔺️diff/🔣️.json"),
    outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️set-element-workset/✅️basic/🎯️outcome/🔣️.json"),
};
#[semio_framework_async_macros::async_test]
async fn declared_fixture_and_inverse_laws() { kit::outcome(&CASE); kit::produces_diff(&CASE); kit::applies(&CASE); kit::inverse_restores(&CASE); protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&kit::mutation(&CASE), &kit::before(&CASE)).await; }
