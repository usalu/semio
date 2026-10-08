//! 🧮️ En1993 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> En1993Snapshot {
    En1993Snapshot::default()
}

fn changed() -> En1993Snapshot {
    let mut other = base();
    other.annex = crate::document::AnnexChoice::En;
    other.materials[0].fy += 1.0;
    let mut extra = other.members[0].clone();
    extra.id = "member-extra".into();
    other.members.insert(0, extra);
    other.load_cases.pop();
    other.member_actions.reverse();
    other.joints.clear();
    other
}

fn midway() -> En1993Snapshot {
    let mut other = base();
    other.materials[0].fy += 1.0;
    other.load_cases.pop();
    other
}

#[semio_framework_async_macros::async_test]
async fn between_carries_the_base_to_the_changed_document() {
    assert_diff_algebra_between_law::<En1993Snapshot, En1993Diff>(&base(), &changed()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_between_diff_restores_the_base() {
    assert_diff_algebra_inverse_law(&base(), &En1993Diff::between(&base(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn absorbing_two_between_diffs_equals_applying_them_in_sequence() {
    assert_mutation_diff_absorb_law(&base(), En1993Diff::between(&base(), &midway()), En1993Diff::between(&midway(), &changed())).await;
}

#[semio_framework_async_macros::async_test]
async fn between_of_equal_documents_is_empty() {
    assert!(En1993Diff::between(&base(), &base()).is_empty());
}
