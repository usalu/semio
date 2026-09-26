use crate::mutations::insert_member_action::InsertMemberAction;
use crate::{MemberAction, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = MemberAction { id: "act-new".into(), member_id: "member-b1".into(), load_case_id: "uls-1".into(), action: crate::DesignAction { n: 0.0, vy: 0.0, vz: 10_000.0, my: 10_000.0, mz: 0.0, t: 0.0 } };
    let payload = InsertMemberAction { index: base.member_actions.len(), member_action: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.member_actions.len(), base.member_actions.len() + 1);
}
