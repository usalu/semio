//! ➖️ `remove-member`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveMember {
    pub member_id: String,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for RemoveMember {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "member", kind: "remove-member", record: "RemovedMember" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove member {}", self.member_id), &format!("Bauteil {} entfernen", self.member_id))
    }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}
