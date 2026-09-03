//! 🚚️ Forms mutation payload — `move-block-to-step`, repositioning a block across (or within) its
//! owning step's `blocks` list (derivation-rules rule 5's hierarchy `move-to-<container>` pattern —
//! this crosses a container boundary, unlike a plain in-list `reorder`). Physical dir name
//! (`↔️move-block`, wired by `🦀️.rs`) predates the semantic rename; the Rust module is still
//! `move_block`, the type/variant/kind are `move-block-to-step`.

use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🚚️MoveBlockToStep
/// 🚚️ Moves the block `block_id` (currently inside `step_id`) into `to_step_id`'s `blocks`, at a
/// FINAL-state `index` within the destination. `step_id == to_step_id` is a plain reorder within one
/// step.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
pub struct MoveBlockToStep {
    pub step_id: String,
    pub block_id: String,
    pub to_step_id: String,
    pub index: usize,
}

impl MutationKind<FormsSnapshot, FormMutation> for MoveBlockToStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "block", kind: "move-block-to-step", record: "MovedBlockToStep" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_move_block_to_step(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_move_block_to_step(self, base)
    }
    async fn label(&self) -> String {
        format!("Move block \"{}\" to step \"{}\"", self.block_id, self.to_step_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block_id.clone()]
    }
}
//#endregion 🚚️MoveBlockToStep
