//! ✏️ VCS mutation — `RenameVcs`: changes the document's identity `title` field.
use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Mutation
/// ✏️ `rename-vcs` payload — `new_title` per the taxonomy's naming convention for identity fields.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "rename-vcs")]
pub struct RenameVcs {
    pub new_title: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_vcs(new_title: String) -> VcsDemoMutation {
    VcsDemoMutation::RenameVcs(RenameVcs { new_title })
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for RenameVcs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "vcs", kind: "rename-vcs", record: "RenamedVcs" };

    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename vcs to \"{}\"", self.new_title)
    }
}
//#endregion 🔖️Mutation
