//! 🦒 `change-sigma-ed-shell-mpa` payload — changes the En1999 document's `sigma_ed_shell_mpa` (shell design stress [MPa]).


use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::mutations::change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSigmaEdShellMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSigmaEdShellMpa {
    pub new_sigma_ed_shell_mpa: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeSigmaEdShellMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sigma-ed-shell-mpa", kind: "change-sigma-ed-shell-mpa", record: "ChangedSigmaEdShellMpa" };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shell design stress [MPa] to {}", self.new_sigma_ed_shell_mpa)
    }
}
//#endregion 🔖️ChangeSigmaEdShellMpa
