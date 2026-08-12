//! ✂️ Forms mutation payload — `delete-block`, a step's `blocks` id-keyed nested collection's
//! `delete` verb. Physical dir name (`➖remove-block`, wired by `📦️glue.rs`) predates the semantic
//! rename; the Rust module is still `remove_block`, the type/variant/kind are `delete-block`.

use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ✂️DeleteBlock
/// ✂️ Removes a block by id from `step_id`'s `blocks`. Inverse recreates it (with its captured base
/// position) via `create-block`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteBlock {
    pub step_id: String,
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DeleteBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "block", kind: "delete-block", record: "DeletedBlock" };

    fn diff(&self, base: &FormsSnapshot) -> FormsDiff {
        super::diff::diff_delete_block(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_delete_block(self, base)
    }
    fn label(&self) -> String {
        format!("Delete block \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.id.clone()]
    }
}
//#endregion ✂️DeleteBlock
