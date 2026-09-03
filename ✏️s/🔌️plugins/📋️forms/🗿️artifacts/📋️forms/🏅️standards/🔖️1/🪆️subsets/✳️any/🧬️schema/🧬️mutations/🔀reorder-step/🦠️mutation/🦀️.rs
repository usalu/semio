//! 🔀️ Forms mutation payload — `reorder-step`, repositioning within the `steps` ordered list (never
//! spatial — see `📓️taxonomy.md`'s `reorder` vs `move` distinction). Physical dir name (`↔️move-step`,
//! wired by `🦀️.rs`) predates the semantic rename; the Rust module is still `move_step`, the
//! type/variant/kind are `reorder-step`.

use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔀️ReorderStep
/// 🔀️ Repositions a step to a FINAL-state `to_index` within `steps`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
pub struct ReorderStep {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<FormsSnapshot, FormMutation> for ReorderStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "step", kind: "reorder-step", record: "ReorderedStep" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_reorder_step(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_reorder_step(self, base)
    }
    async fn label(&self) -> String {
        format!("Reorder step \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderStep
