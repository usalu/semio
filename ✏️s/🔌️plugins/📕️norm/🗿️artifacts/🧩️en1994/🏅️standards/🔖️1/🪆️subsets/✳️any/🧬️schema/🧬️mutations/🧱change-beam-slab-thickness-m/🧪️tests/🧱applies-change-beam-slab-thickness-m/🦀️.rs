use crate::artifact_schema::mutations::change_beam_slab_thickness_m::ChangeBeamSlabThicknessM;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_slab_thickness_m() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamSlabThicknessM(ChangeBeamSlabThicknessM { index: 0, new_slab_thickness_m: 0.16 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
