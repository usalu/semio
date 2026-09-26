use crate::artifact_schema::mutations::remove_column::RemoveColumn;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_remove_column() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::RemoveColumn(RemoveColumn { index: 0 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
