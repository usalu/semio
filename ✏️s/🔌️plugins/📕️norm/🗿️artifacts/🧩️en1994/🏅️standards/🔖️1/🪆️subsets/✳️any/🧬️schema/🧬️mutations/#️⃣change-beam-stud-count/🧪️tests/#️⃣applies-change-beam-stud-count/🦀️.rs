use crate::artifact_schema::mutations::change_beam_stud_count::ChangeBeamStudCount;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_stud_count() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamStudCount(ChangeBeamStudCount { index: 0, new_total_count: 55 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
