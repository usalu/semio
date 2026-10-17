//! 🗂️ Fixture-driven mutation laws: every committed case folder proves the same laws, discovered at run time, so a leaf is covered by adding its case folders.

use crate::standards::v1::subsets::any::schema::mutations::kit;

#[semio_framework_async_macros::async_test]
async fn the_fixture_root_holds_cases() {
    assert!(!kit::cases().is_empty(), "no committed mutation case was discovered");
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    for case in kit::cases() {
        kit::outcome(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_after_snapshot() {
    for case in kit::cases() {
        kit::applies(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() {
    for case in kit::cases() {
        kit::produces_diff(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_before_snapshot() {
    for case in kit::cases() {
        kit::inverse_restores(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for case in kit::cases() {
        kit::canonical(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for case in kit::cases().iter().filter(|case| kit::applied(case)) {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&kit::mutation(case), &kit::before(case)).await;
    }
}
