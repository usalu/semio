use crate::artifact_schema::mutations::change_fatigue_detail::ChangeFatigueDetail;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_fatigue_detail() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeFatigueDetail(ChangeFatigueDetail { new_fatigue_detail: "flange_butt_weld".into() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
