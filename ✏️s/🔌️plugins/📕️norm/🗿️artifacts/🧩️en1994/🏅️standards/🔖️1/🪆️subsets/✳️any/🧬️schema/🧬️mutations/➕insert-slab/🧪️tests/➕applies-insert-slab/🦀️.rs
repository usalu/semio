use crate::artifact_schema::mutations::insert_slab::InsertSlab;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_insert_slab() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::InsertSlab(InsertSlab { index: 0, slab: CompositeSlab::default_placeholder() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
