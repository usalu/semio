//! `remove-seismic` mutation for EN 1990.
use crate::{En1990Mutation, En1990Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveSeismic { pub index: usize, }
impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for RemoveSeismic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "seismic", kind: "remove-seismic", record: "RemovedSeismic" };
    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove seismic action", "Erdbebeneinwirkung entfernen")
    }
}
