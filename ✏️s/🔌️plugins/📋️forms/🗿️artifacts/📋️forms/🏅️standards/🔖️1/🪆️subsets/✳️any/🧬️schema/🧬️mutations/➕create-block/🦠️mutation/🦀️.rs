//! 🧩️ Forms mutation payload — `create-block`, a step's `blocks` id-keyed nested collection's
//! `create` verb. Physical dir name (`➕add-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `add_block`, the type/variant/kind are `create-block`.

use crate::artifacts::forms::{FormMutation, FormQuestion, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🧩️CreateBlock
/// 🧩️ Brings a new [`FormQuestion`] into existence inside `step_id`'s `blocks`, at an optional
/// FINAL-state `index` (`None` appends). An unknown `step_id` is Fatal `mutation.invariant`; a
/// duplicate `block.id` within that step is Fatal `mutation.duplicate-id`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBlock {
    pub step_id: String,
    pub block: FormQuestion,
    pub index: Option<usize>,
}

impl MutationKind<FormsSnapshot, FormMutation> for CreateBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "block", kind: "create-block", record: "CreatedBlock" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_create_block(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_create_block(self, base)
    }
    async fn label(&self) -> String {
        format!("Create block \"{}\"", self.block.label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block.id.clone()]
    }
}
//#endregion 🧩️CreateBlock
