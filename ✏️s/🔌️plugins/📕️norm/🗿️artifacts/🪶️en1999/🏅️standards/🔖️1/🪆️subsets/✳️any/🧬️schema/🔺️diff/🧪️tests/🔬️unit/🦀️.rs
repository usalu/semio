//! 🧪️ Keyed-diff algebra of En1999: nested-key coalescing, list-setter rows and the negative diff.

use super::En1999Diff;
use crate::mutations::add_member::AddMember;
use crate::mutations::change_member_buckling_length::ChangeMemberBucklingLength;
use crate::mutations::change_members::ChangeMembers;
use crate::mutations::change_plate_thickness::ChangePlateThickness;
use crate::mutations::remove_member::RemoveMember;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn diff_of(mutation: &En1999Mutation, base: &En1999Snapshot) -> En1999Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

async fn law(base: &En1999Snapshot, first: &En1999Mutation, second: impl Fn(&En1999Snapshot) -> En1999Mutation) -> En1999Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

fn buckling(base: &En1999Snapshot, axis: &str, value: f64) -> En1999Mutation {
    En1999Mutation::ChangeMemberBucklingLength(ChangeMemberBucklingLength { member_id: base.members[0].id.clone(), axis: axis.into(), new_length: value })
}

#[semio_framework_async_macros::async_test]
async fn buckling_patches_on_one_member_coalesce_into_one_patch() {
    let base = En1999Snapshot::default();
    let sum = law(&base, &buckling(&base, "y", 2.5), |mid| buckling(mid, "z", 3.5)).await;
    let rows = sum.members.expect("members");
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].patch.buckling_length_y, Some(2.5));
    assert_eq!(rows.modified[0].patch.buckling_length_z, Some(3.5));
}

#[semio_framework_async_macros::async_test]
async fn nested_element_patches_coalesce_by_element_id() {
    let base = En1999Snapshot::default();
    let section = base.sections.iter().find(|section| !section.elements.is_empty()).expect("section with elements").clone();
    let set = |value: f64| En1999Mutation::ChangePlateThickness(ChangePlateThickness { section_id: section.id.clone(), element_id: section.elements[0].id.clone(), new_thickness: value });
    let sum = law(&base, &set(0.004), |_| set(0.006)).await;
    let patches = sum.sections.expect("sections").modified;
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].patch.elements.as_ref().expect("elements").modified.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn add_then_remove_cancels_and_remove_then_add_replaces() {
    let base = En1999Snapshot::default();
    let mut member = base.members[0].clone();
    member.id = "member-created".into();
    let sum = law(&base, &En1999Mutation::AddMember(AddMember { index: 1, member: member.clone() }), |_| En1999Mutation::RemoveMember(RemoveMember { id: member.id.clone() })).await;
    assert!(sum.is_empty(), "create∘delete must cancel: {sum:?}");
    let existing = base.members[0].clone();
    let sum = law(&base, &En1999Mutation::RemoveMember(RemoveMember { id: existing.id.clone() }), |_| En1999Mutation::AddMember(AddMember { index: 0, member: existing.clone() })).await;
    assert_eq!(protocol::apply_diff(&sum, &base).expect("replace"), base);
}

#[semio_framework_async_macros::async_test]
async fn a_list_setter_names_every_row_and_inverts() {
    let base = En1999Snapshot::default();
    let mut members = base.members.clone();
    members[0].length += 1.0;
    let mut extra = members[1].clone();
    extra.id = "member-extra".into();
    members.insert(0, extra);
    members.pop();
    let mutation = En1999Mutation::ChangeMembers(ChangeMembers { members: members.clone() });
    let forward = diff_of(&mutation, &base);
    let after = protocol::apply_diff(&forward, &base).expect("apply");
    assert_eq!(after.members, members);
    assert_eq!(protocol::apply_diff(&forward.inverse(&base), &after).expect("inverse"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
