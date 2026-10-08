//! 🧪️ Keyed-diff algebra of En1992: same-key coalescing and the negative diff, over real mutation diffs.

use super::En1992Diff;
use crate::mutations::change_action_vk::ChangeActionVk;
use crate::mutations::change_member_height::ChangeMemberHeight;
use crate::mutations::change_member_width::ChangeMemberWidth;
use crate::mutations::insert_member::InsertMember;
use crate::mutations::remove_member::RemoveMember;
use crate::mutations::reorder_members::ReorderMembers;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn diff_of(mutation: &En1992Mutation, base: &En1992Snapshot) -> En1992Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

fn width(base: &En1992Snapshot, value: f64) -> En1992Mutation {
    En1992Mutation::ChangeMemberWidth(ChangeMemberWidth { member_id: base.members[0].id.clone(), new_value: value })
}

#[semio_framework_async_macros::async_test]
async fn patches_on_one_row_coalesce_into_one_patch() {
    let base = En1992Snapshot::default();
    let first = diff_of(&width(&base, 0.41), &base);
    let mid = protocol::apply_diff(&first, &base).expect("first");
    let second = diff_of(&En1992Mutation::ChangeMemberHeight(ChangeMemberHeight { member_id: base.members[0].id.clone(), new_value: 0.77 }), &mid);
    let mut sum = first.clone();
    sum.absorb(second.clone());
    assert_eq!(sum.members.as_ref().expect("members").modified.len(), 1);
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first, second).await;
}

#[semio_framework_async_macros::async_test]
async fn nested_action_patches_coalesce_by_action_id() {
    let base = En1992Snapshot::default();
    let member = base.members.iter().find(|member| !member.actions.is_empty()).expect("member with actions").clone();
    let action = member.actions[0].id.clone();
    let set = |value: f64| En1992Mutation::ChangeActionVk(ChangeActionVk { member_id: member.id.clone(), action_id: action.clone(), new_value: value });
    let first = diff_of(&set(11.0), &base);
    let mid = protocol::apply_diff(&first, &base).expect("first");
    let second = diff_of(&set(22.0), &mid);
    let mut sum = first.clone();
    sum.absorb(second.clone());
    let patches = &sum.members.as_ref().expect("members").modified;
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].patch.actions.as_ref().expect("actions").modified.len(), 1);
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first, second).await;
}

#[semio_framework_async_macros::async_test]
async fn insert_then_remove_cancels_and_remove_then_insert_replaces() {
    let base = En1992Snapshot::default();
    let mut member = base.members[0].clone();
    member.id = "member-created".into();
    let insert = En1992Mutation::InsertMember(InsertMember { index: 1, member: member.clone() });
    let created = diff_of(&insert, &base);
    let mid = protocol::apply_diff(&created, &base).expect("insert");
    let deleted = diff_of(&En1992Mutation::RemoveMember(RemoveMember { member_id: member.id.clone() }), &mid);
    let mut sum = created.clone();
    sum.absorb(deleted.clone());
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, created, deleted).await;

    let index = 0;
    let existing = base.members[index].clone();
    let removed = diff_of(&En1992Mutation::RemoveMember(RemoveMember { member_id: existing.id.clone() }), &base);
    let mid = protocol::apply_diff(&removed, &base).expect("remove");
    let restored = diff_of(&En1992Mutation::InsertMember(InsertMember { index, member: existing.clone() }), &mid);
    let mut sum = removed.clone();
    sum.absorb(restored.clone());
    assert_eq!(protocol::apply_diff(&sum, &base).expect("replace"), base);
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, removed, restored).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_and_inverse_follow_the_negative_diff() {
    let base = En1992Snapshot::default();
    let moved = diff_of(&En1992Mutation::ReorderMembers(ReorderMembers { from_index: 0, to_index: base.members.len() - 1 }), &base);
    let after = protocol::apply_diff(&moved, &base).expect("reorder");
    assert_ne!(after, base);
    assert_eq!(protocol::apply_diff(&moved.inverse(&base), &after).expect("inverse"), base);
}
