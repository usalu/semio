use crate::artifact_schema::mutations::change_insulation_thickness_m::ChangeInsulationThicknessM;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_insulation_thickness_m() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeInsulationThicknessM(ChangeInsulationThicknessM { new_insulation_thickness_m: 0.028 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
