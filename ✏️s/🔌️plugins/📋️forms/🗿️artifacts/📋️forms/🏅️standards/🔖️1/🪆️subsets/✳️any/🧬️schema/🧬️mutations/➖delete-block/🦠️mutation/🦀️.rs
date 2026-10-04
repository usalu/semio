//! ✂️ Forms mutation payload — `delete-block`, a step's `blocks` id-keyed nested collection's
//! `delete` verb. Physical dir name (`➖remove-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `remove_block`, the type/variant/kind are `delete-block`.

use crate::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region ✂️DeleteBlock
/// ✂️ Removes a block by id from `step_id`'s `blocks`. Inverse recreates it (with its captured base
/// position) via `create-block`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteBlock {
    pub step_id: String,
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DeleteBlock {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "block", kind: "delete-block", record: "DeletedBlock" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_delete_block(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse_delete_block(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete block \"{}\"", self.id), &format!("Block \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.id.clone()]
    }
}
//#endregion ✂️DeleteBlock
