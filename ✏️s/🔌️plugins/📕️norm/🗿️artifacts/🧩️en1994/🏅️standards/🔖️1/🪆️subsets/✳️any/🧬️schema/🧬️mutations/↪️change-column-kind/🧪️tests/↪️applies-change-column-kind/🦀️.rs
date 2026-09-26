use crate::artifact_schema::mutations::change_column_kind::ChangeColumnKind;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_column_kind() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeColumnKind(ChangeColumnKind { index: 0, new_kind: "partially_encased".into() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    assert_eq!(next.columns[0].kind, "partially_encased");
}
