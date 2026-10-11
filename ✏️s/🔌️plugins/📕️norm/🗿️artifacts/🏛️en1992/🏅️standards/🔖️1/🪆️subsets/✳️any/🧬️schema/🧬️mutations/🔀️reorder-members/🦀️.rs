//! 🔀️ `reorder-members`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReorderMembers {
    pub from_index: usize,
    pub to_index: usize,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ReorderMembers {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "member", kind: "reorder-members", record: "ReorderedMembers" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move member #{} to #{}", self.from_index, self.to_index), &format!("Bauteil von #{} nach #{} verschieben", self.from_index, self.to_index))
    }
}
