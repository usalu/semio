use crate::artifact_schema::mutations::change_beam_stud_spacing_m::ChangeBeamStudSpacingM;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_stud_spacing_m() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamStudSpacingM(ChangeBeamStudSpacingM { index: 0, new_spacing_m: 0.20 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    assert!((next.beams[0].studs.spacing_m - 0.20).abs() < 1e-9);
}
