//! 🗑️ Forms mutation payload — `delete-step`, the `steps` id-keyed collection's `delete` verb.
//! Physical dir name (`➖remove-step`, wired by `🦀️.rs`) predates the semantic rename; the Rust
//! module is still `remove_step`, the type/variant/kind are `delete-step`.

use crate::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🗑️DeleteStep
/// 🗑️ Removes a step by id, cascading to every block it carried. Inverse recreates it (with its
/// captured base position and blocks) via `create-step`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteStep {
    pub id: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for DeleteStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_delete_step(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse_delete_step(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete step \"{}\"", self.id), &format!("Schritt \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteStep
