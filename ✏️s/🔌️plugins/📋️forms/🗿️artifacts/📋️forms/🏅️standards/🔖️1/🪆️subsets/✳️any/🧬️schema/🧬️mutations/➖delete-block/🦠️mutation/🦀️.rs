//! ✂️ Forms mutation payload — `delete-block`, a step's `blocks` id-keyed nested collection's
//! `delete` verb. Physical dir name (`➖remove-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `remove_block`, the type/variant/kind are `delete-block`.

use serde::{Deserialize, Serialize};
use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region ✂️DeleteBlock
/// ✂️ Removes a block by id from `step_id`'s `blocks`. Inverse recreates it (with its captured base
/// position) via `create-block`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
pub struct DeleteBlock {
    pub step_id: String,
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DeleteBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "block", kind: "delete-block", record: "DeletedBlock" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_delete_block(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_delete_block(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete block \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.id.clone()]
    }
}
//#endregion ✂️DeleteBlock
