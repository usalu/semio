//! 🔁️ Forms mutation payload — `replace-block`, a whole-value swap of one block's large structured
//! payload (`FormQuestion` has 15+ optional fields plus a boxed recursive condition expression tree
//! — derivation-rules rule 2's `replace-<singular>-<payload>` case, not a per-field `change-block-*`
//! fan-out). Physical dir name (`🩹update-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `update_block`, the type/variant/kind are `replace-block`.

use serde::{Deserialize, Serialize};
use crate::artifacts::forms::{FormMutation, FormQuestion, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔁️ReplaceBlock
/// 🔁️ Replaces the block matching `block.id` inside `step_id`'s `blocks` wholesale.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
pub struct ReplaceBlock {
    pub step_id: String,
    pub block: FormQuestion,
}

impl MutationKind<FormsSnapshot, FormMutation> for ReplaceBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "block", kind: "replace-block", record: "ReplacedBlock" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_replace_block(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_replace_block(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace block \"{}\"", self.block.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block.id.clone()]
    }
}
//#endregion 🔁️ReplaceBlock
