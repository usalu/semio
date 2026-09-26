use crate::artifact_schema::mutations::insert_column::InsertColumn;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_insert_column() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::InsertColumn(InsertColumn { index: 0, column: CompositeColumn::default_placeholder() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
