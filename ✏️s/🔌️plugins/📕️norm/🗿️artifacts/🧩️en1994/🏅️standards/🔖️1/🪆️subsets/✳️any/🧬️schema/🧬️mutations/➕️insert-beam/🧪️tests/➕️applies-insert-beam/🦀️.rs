use crate::artifact_schema::mutations::insert_beam::InsertBeam;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_insert_beam() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::InsertBeam(InsertBeam { index: 0, beam: CompositeBeam::default_placeholder() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
