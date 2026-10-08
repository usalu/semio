use super::*;
use crate::mutations::En1995Mutation;
use protocol::{apply_diff, DiffAlgebra, Mutation as _, MutationDiff};

/// 🔺️ A member-scoped change patches only that member's field; annex and connections stay untouched.
#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1995Snapshot::default();
    let mutation = En1995Mutation::ChangeMemberH(crate::mutations::change_member_h::ChangeMemberH { member_id: base.members[0].id.clone(), new_value: 0.5 });
    let outcome = mutation.diff(&base);
    let diff = outcome.diff();
    assert!(diff.annex.is_none() && diff.connections.is_empty());
    assert_eq!((diff.members.removed.len(), diff.members.inserted.len(), diff.members.modified.len()), (0, 0, 1));
    assert_eq!(diff.members.modified[0].patch.h_m, Some(0.5));
    let mut expected = base.clone();
    expected.members[0].h_m = 0.5;
    assert_eq!(apply_diff(diff, &base).expect("valid mutation diff"), expected);
}

/// 🧲 `absorb` coalesces patches per key: the latest value of a field wins and unrelated fields accumulate.
#[semio_framework_async_macros::async_test]
async fn absorb_coalesces_per_key_and_keeps_the_sequential_result() {
    let base = En1995Snapshot::default();
    let member_id = base.members[0].id.clone();
    let mut first = En1995Mutation::ChangeMemberH(crate::mutations::change_member_h::ChangeMemberH { member_id: member_id.clone(), new_value: 0.5 }).diff(&base).diff().clone();
    let mid = apply_diff(&first, &base).expect("first applies");
    let second = En1995Mutation::ChangeMemberH(crate::mutations::change_member_h::ChangeMemberH { member_id: member_id.clone(), new_value: 0.6 }).diff(&mid).diff().clone();
    let third = En1995Mutation::ChangeAnnex(crate::mutations::change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }).diff(&mid).diff().clone();
    first.absorb(second);
    first.absorb(third);
    assert_eq!(first.members.modified.len(), 1, "patch∘patch is one patch");
    let applied = apply_diff(&first, &base).expect("absorbed diff applies");
    assert_eq!(applied.annex, crate::document::AnnexChoice::En);
    assert!((applied.members[0].h_m - 0.6).abs() < 1e-12);
}

