use crate::artifact_schema::mutations::change_beam_transverse_as::ChangeBeamTransverseAs;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_transverse_as() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamTransverseAs(ChangeBeamTransverseAs { index: 0, new_transverse_as_m2_per_m: 4e-4 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
