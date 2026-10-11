//! 🔧 `change-cement-type` payload.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeCementType {
    pub new_cement_type: String,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeCementType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cement-type", kind: "change-cement-type", record: "ChangedCementType" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change cement type to {}", self.new_cement_type), &format!("Zementart auf {} ändern", self.new_cement_type))
    }
}
