//! `remove-seismic` mutation for EN 1990.
use crate::{En1990Mutation, En1990Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSeismic { pub index: usize, }
impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for RemoveSeismic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "seismic", kind: "remove-seismic", record: "RemovedSeismic" };
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Remove seismic", "Entfernen: seismic") }
}
