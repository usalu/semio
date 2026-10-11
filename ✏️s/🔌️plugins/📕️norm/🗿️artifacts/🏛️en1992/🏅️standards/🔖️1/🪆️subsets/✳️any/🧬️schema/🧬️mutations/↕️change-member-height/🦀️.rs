//! 🔧 `change-member-height`.

use crate::diff::En1992Diff;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMemberHeight {
    pub member_id: String,
    pub new_value: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeMemberHeight {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "member-height", kind: "change-member-height", record: "ChangedChangeMemberHeight" };
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change height of member {}", self.member_id), &format!("Höhe von Bauteil {} ändern", self.member_id))
    }
    fn target(&self) -> Vec<String> { vec![self.member_id.clone()] }
}
