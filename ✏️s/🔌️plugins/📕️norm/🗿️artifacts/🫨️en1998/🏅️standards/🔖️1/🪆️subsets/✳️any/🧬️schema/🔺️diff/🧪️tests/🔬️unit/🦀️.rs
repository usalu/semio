//! 🧮️ En1998 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> En1998Snapshot {
    En1998Snapshot::default()
}

fn changed() -> En1998Snapshot {
    let mut other = base();
    other.annex = "en".into();
    other.site.a_gr += 0.1;
    other.buildings[0].plan_regular = !other.buildings[0].plan_regular;
    other.buildings[0].storeys.remove(1);
    let mut extra = other.buildings[0].clone();
    extra.id = "bldg-extra".into();
    other.buildings.push(extra);
    other
}

fn midway() -> En1998Snapshot {
    let mut other = base();
    other.site.a_gr += 0.1;
    other.buildings[0].plan_regular = !other.buildings[0].plan_regular;
    other
}

#[semio_framework_async_macros::async_test]
async fn between_carries_the_base_to_the_changed_document() {
    assert_diff_algebra_between_law::<En1998Snapshot, En1998Diff>(&base(), &changed()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_between_diff_restores_the_base() {
    assert_diff_algebra_inverse_law(&base(), &En1998Diff::between(&base(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn absorbing_two_between_diffs_equals_applying_them_in_sequence() {
    assert_mutation_diff_absorb_law(&base(), En1998Diff::between(&base(), &midway()), En1998Diff::between(&midway(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn between_of_equal_documents_is_empty() {
    assert!(En1998Diff::between(&base(), &base()).is_empty());
}
