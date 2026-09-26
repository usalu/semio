use crate::artifact_schema::mutations::change_structure_kind::ChangeStructureKind;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_structure_kind() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeStructureKind(ChangeStructureKind { new_structure_kind: "bridge".into() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
