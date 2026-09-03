//! 🏷️ VCS mutation — `AddTag`: attaches a set-like tag member to the document.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// 🏷️ `add-tag` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-tag")]
pub struct AddTag {
    pub tag: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_tag(tag: String) -> VcsDemoMutation {
    VcsDemoMutation::AddTag(AddTag { tag })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for AddTag {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "tag", kind: "add-tag", record: "AddedTagToVcs" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add tag \"{}\"", self.tag)
    }
    fn target(&self) -> Vec<String> {
        vec![self.tag.clone()]
    }
}
//#endregion 🔖️Mutation
