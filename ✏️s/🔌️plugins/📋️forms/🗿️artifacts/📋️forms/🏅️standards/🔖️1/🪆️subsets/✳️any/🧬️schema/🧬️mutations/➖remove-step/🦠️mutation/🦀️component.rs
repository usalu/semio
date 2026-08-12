//! 🗑️ Forms mutation payload — `delete-step`, the `steps` id-keyed collection's `delete` verb.
//! Physical dir name (`➖remove-step`, wired by `📦️glue.rs`) predates the semantic rename; the Rust
//! module is still `remove_step`, the type/variant/kind are `delete-step`.

use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🗑️DeleteStep
/// 🗑️ Removes a step by id, cascading to every block it carried. Inverse recreates it (with its
/// captured base position and blocks) via `create-step`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteStep {
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DeleteStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    fn diff(&self, base: &FormsSnapshot) -> FormsDiff {
        super::diff::diff_delete_step(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_delete_step(self, base)
    }
    fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStep
