use crate::{Din16798Mutation, Din16798Snapshot, VentSystemDocument};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertVentSystem { pub index: usize, pub vent: VentSystemDocument }
impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for InsertVentSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vent-system", kind: "insert-vent-system", record: "InsertVentSystem" };
    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<<Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Insert vent system", "Lüftungssystem einfügen") }
}
