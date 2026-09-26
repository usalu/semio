use crate::mutations::{remove_member_action::RemoveMemberAction, insert_member_action::InsertMemberAction};
use crate::{MemberAction, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = MemberAction { id: "act-new".into(), member_id: "member-b1".into(), load_case_id: "uls-1".into(), action: crate::DesignAction { n: 0.0, vy: 0.0, vz: 10_000.0, my: 10_000.0, mz: 0.0, t: 0.0 } };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertMemberAction { index: 0, member_action: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.member_actions.len();
    let out = protocol::MutationKind::diff(&RemoveMemberAction { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.member_actions.len(), before_len - 1);
}
