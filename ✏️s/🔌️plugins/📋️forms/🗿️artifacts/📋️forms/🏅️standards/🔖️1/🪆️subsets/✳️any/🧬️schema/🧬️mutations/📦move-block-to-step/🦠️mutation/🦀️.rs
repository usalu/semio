//! 🚚️ Forms mutation payload — `move-block-to-step`, repositioning a block across (or within) its
//! owning step's `blocks` list (derivation-rules rule 5's hierarchy `move-to-<container>` pattern —
//! this crosses a container boundary, unlike a plain in-list `reorder`). Physical dir name
//! (`↔️move-block`, wired by `🦀️.rs`) predates the semantic rename; the Rust module is still
//! `move_block`, the type/variant/kind are `move-block-to-step`.

use crate::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🚚️MoveBlockToStep
/// 🚚️ Moves the block `block_id` (currently inside `step_id`) into `to_step_id`'s `blocks`, at a
/// FINAL-state `index` within the destination. `step_id == to_step_id` is a plain reorder within one
/// step.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveBlockToStep {
    pub step_id: String,
    pub block_id: String,
    pub to_step_id: String,
    pub index: usize,
}

impl MutationKind<FormsSnapshot, FormMutation> for MoveBlockToStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "block", kind: "move-block-to-step", record: "MovedBlockToStep" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_move_block_to_step(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse_move_block_to_step(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move block \"{}\" to step \"{}\"", self.block_id, self.to_step_id), &format!("Block \"{}\" nach Schritt \"{}\" verschieben", self.block_id, self.to_step_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block_id.clone()]
    }
}
//#endregion 🚚️MoveBlockToStep
