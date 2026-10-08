//! 🧮️ En1990 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> En1990Snapshot {
    En1990Snapshot::default()
}

fn changed() -> En1990Snapshot {
    let mut other = base();
    other.annex = crate::document::AnnexChoice::En;
    other.consequence_class = 3;
    other.permanents.remove(0);
    other.permanents[0].gk += 1.0;
    other.variables.reverse();
    other.effects.truncate(2);
    other.members[0].span = 7.0;
    other
}

fn midway() -> En1990Snapshot {
    let mut other = base();
    other.consequence_class = 3;
    other.variables.reverse();
    other
}

#[semio_framework_async_macros::async_test]
async fn between_carries_the_base_to_the_changed_document() {
    assert_diff_algebra_between_law::<En1990Snapshot, En1990Diff>(&base(), &changed()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_between_diff_restores_the_base() {
    assert_diff_algebra_inverse_law(&base(), &En1990Diff::between(&base(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn absorbing_two_between_diffs_equals_applying_them_in_sequence() {
    assert_mutation_diff_absorb_law(&base(), En1990Diff::between(&base(), &midway()), En1990Diff::between(&midway(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn between_of_equal_documents_is_empty() {
    assert!(En1990Diff::between(&base(), &base()).is_empty());
}
