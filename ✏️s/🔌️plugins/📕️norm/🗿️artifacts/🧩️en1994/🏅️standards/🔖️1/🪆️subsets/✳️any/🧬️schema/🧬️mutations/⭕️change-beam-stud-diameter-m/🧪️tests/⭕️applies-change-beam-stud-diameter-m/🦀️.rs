use crate::artifact_schema::mutations::change_beam_stud_diameter_m::ChangeBeamStudDiameterM;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_stud_diameter_m() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamStudDiameterM(ChangeBeamStudDiameterM { index: 0, new_diameter_m: 0.022 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
