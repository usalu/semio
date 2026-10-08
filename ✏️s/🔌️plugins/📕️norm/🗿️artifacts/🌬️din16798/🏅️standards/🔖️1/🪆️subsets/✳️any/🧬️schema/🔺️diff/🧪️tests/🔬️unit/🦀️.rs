//! 🧮️ Din16798 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> Din16798Snapshot {
    Din16798Snapshot::default()
}

fn changed() -> Din16798Snapshot {
    let mut other = base();
    other.theta_rm_c += 1.0;
    let mut extra = other.zones[0].clone();
    extra.id = "zone-extra".into();
    other.zones.insert(0, extra);
    other.vent_systems[0].heat_recovery_eta += 0.01;
    other
}

fn midway() -> Din16798Snapshot {
    let mut other = base();
    other.theta_rm_c += 1.0;
    other.vent_systems[0].heat_recovery_eta += 0.01;
    other
}

#[semio_framework_async_macros::async_test]
async fn between_carries_the_base_to_the_changed_document() {
    assert_diff_algebra_between_law::<Din16798Snapshot, Din16798Diff>(&base(), &changed()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_between_diff_restores_the_base() {
    assert_diff_algebra_inverse_law(&base(), &Din16798Diff::between(&base(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn absorbing_two_between_diffs_equals_applying_them_in_sequence() {
    assert_mutation_diff_absorb_law(&base(), Din16798Diff::between(&base(), &midway()), Din16798Diff::between(&midway(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn between_of_equal_documents_is_empty() {
    assert!(Din16798Diff::between(&base(), &base()).is_empty());
}
