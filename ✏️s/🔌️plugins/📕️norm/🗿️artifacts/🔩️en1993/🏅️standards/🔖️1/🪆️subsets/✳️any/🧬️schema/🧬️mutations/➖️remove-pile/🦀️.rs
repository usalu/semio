//! ➖️ remove-pile
use crate::{En1993Mutation, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemovePile { pub index: usize }
impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for RemovePile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "pile", kind: "remove-pile", record: "RemovedPile" };
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove pile #{}", self.index), &format!("Pfahl #{} entfernen", self.index)) }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}
