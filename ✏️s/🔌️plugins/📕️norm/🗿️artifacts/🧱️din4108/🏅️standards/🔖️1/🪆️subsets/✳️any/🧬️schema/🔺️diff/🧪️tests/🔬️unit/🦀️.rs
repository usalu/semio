//! 🧮️ Din4108 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> Din4108Snapshot {
    Din4108Snapshot::default()
}

fn changed() -> Din4108Snapshot {
    let mut other = base();
    other.t_int_c += 1.0;
    other.zones[0].windows.remove(0);
    other.elements[0].layers[1].thickness_m += 0.01;
    other.elements.remove(2);
    other.thermal_bridges.reverse();
    other
}

fn midway() -> Din4108Snapshot {
    let mut other = base();
    other.t_int_c += 1.0;
    other.elements.remove(2);
    other
}

#[semio_framework_async_macros::async_test]
async fn between_carries_the_base_to_the_changed_document() {
    assert_diff_algebra_between_law::<Din4108Snapshot, Din4108Diff>(&base(), &changed()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_between_diff_restores_the_base() {
    assert_diff_algebra_inverse_law(&base(), &Din4108Diff::between(&base(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn absorbing_two_between_diffs_equals_applying_them_in_sequence() {
    assert_mutation_diff_absorb_law(&base(), Din4108Diff::between(&base(), &midway()), Din4108Diff::between(&midway(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn between_of_equal_documents_is_empty() {
    assert!(Din4108Diff::between(&base(), &base()).is_empty());
}
