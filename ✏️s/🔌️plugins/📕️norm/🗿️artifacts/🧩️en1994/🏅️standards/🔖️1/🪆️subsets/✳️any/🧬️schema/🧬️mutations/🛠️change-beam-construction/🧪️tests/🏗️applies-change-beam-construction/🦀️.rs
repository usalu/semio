use crate::artifact_schema::mutations::change_beam_construction::ChangeBeamConstruction;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_beam_construction() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeBeamConstruction(ChangeBeamConstruction { index: 0, new_construction: "unpropped".into() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
