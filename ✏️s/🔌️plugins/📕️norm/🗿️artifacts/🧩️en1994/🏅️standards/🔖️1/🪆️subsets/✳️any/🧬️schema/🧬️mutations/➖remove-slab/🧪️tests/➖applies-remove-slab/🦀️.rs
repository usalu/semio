use crate::artifact_schema::mutations::remove_slab::RemoveSlab;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_remove_slab() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::RemoveSlab(RemoveSlab { index: 0 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
