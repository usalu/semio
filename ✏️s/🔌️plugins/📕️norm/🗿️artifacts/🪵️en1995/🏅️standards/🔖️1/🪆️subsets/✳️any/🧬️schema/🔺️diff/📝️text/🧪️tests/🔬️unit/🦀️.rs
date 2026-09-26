use super::*;
use crate::mutations::En1995Mutation;
use protocol::{Mutation as _, MutationDiff};

/// 🔺️ A member-scoped change writes only the `members` list; annex and connections stay `None`.
#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1995Snapshot::default();
    let mutation = En1995Mutation::ChangeMemberH(crate::mutations::change_member_h::ChangeMemberH { member_id: base.members[0].id.clone(), new_value: 0.5 });
    let outcome = mutation.diff(&base);
    let diff = outcome.diff();
    assert!(diff.annex.is_none() && diff.connections.is_none() && diff.artifact.is_none());
    let mut expected = base.clone();
    expected.members[0].h_m = 0.5;
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), expected);
}

/// 🧲 `absorb` keeps the latest list per field and a whole-artifact replacement wins outright.
#[semio_framework_async_macros::async_test]
async fn absorb_takes_latest_field_and_replacement_wins() {
    let base = En1995Snapshot::default();
    let mut first = En1995Mutation::ChangeMemberH(crate::mutations::change_member_h::ChangeMemberH { member_id: base.members[0].id.clone(), new_value: 0.5 }).diff(&base).diff().clone();
    let second = En1995Mutation::ChangeAnnex(crate::mutations::set_snapshot::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }).diff(&base).diff().clone();
    first.absorb(second);
    assert_eq!(first.annex, Some(crate::document::AnnexChoice::En));
    assert!(first.members.is_some());
    let applied = first.apply(&base).expect("absorbed diff applies");
    assert_eq!(applied.annex, crate::document::AnnexChoice::En);
    assert!((applied.members[0].h_m - 0.5).abs() < 1e-12);
}
