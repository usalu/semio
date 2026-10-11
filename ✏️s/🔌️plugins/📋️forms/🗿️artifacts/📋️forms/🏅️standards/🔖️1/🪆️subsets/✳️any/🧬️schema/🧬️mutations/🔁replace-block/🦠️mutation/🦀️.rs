//! 🔁️ Forms mutation payload — `replace-block`, a whole-value swap of one block's large structured
//! payload (`FormQuestion` has 15+ optional fields plus a boxed recursive condition expression tree
//! — derivation-rules rule 2's `replace-<singular>-<payload>` case, not a per-field `change-block-*`
//! fan-out). Physical dir name (`🩹update-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `update_block`, the type/variant/kind are `replace-block`.

use crate::{FormMutation, FormQuestion, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔁️ReplaceBlock
/// 🔁️ Replaces the block matching `block.id` inside `step_id`'s `blocks` wholesale.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReplaceBlock {
    pub step_id: String,
    pub block: FormQuestion,
}

impl MutationKind<FormsSnapshot, FormMutation> for ReplaceBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "block", kind: "replace-block", record: "ReplacedBlock" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_replace_block(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse_replace_block(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace block \"{}\"", self.block.id), &format!("Block \"{}\" ersetzen", self.block.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block.id.clone()]
    }
}
//#endregion 🔁️ReplaceBlock
