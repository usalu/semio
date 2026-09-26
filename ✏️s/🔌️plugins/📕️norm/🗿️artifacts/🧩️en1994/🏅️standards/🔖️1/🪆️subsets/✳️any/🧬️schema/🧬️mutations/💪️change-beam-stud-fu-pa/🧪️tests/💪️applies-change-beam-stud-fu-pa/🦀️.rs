use crate::artifact_schema::mutations::change_beam_stud_fu_pa::ChangeBeamStudFUPa;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_stud_fu_pa() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamStudFUPa(ChangeBeamStudFUPa { index: 0, new_f_u_pa: 500e6 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
