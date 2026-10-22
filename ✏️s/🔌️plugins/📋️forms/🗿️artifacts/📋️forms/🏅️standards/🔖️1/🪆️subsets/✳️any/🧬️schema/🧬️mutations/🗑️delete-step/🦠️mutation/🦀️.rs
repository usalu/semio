//! 🗑️ Forms mutation payload — `delete-step`, the `steps` id-keyed collection's `delete` verb.
//! Physical dir name (`➖remove-step`, wired by `🦀️.rs`) predates the semantic rename; the Rust
//! module is still `remove_step`, the type/variant/kind are `delete-step`.

use crate::{FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🗑️DeleteStep
/// 🗑️ Removes a step by id, cascading to every block it carried. Inverse recreates it (with its
/// captured base position and blocks) via `create-step`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteStep {
    pub id: String,
}

impl<Op> MutationKind<FormsSnapshot, Op> for DeleteStep
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<DeleteStep> + From<crate::standards::v1::subsets::any::schema::mutations::create_step::mutation::CreateStep>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete step \"{}\"", self.id), &format!("Schritt \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<FormsSnapshot, Op> for DeleteStep
where
    Op: protocol::ColdMutation<FormsSnapshot, ReferenceDiff = FormsDiff> + From<DeleteStep> + From<crate::standards::v1::subsets::any::schema::mutations::create_step::mutation::CreateStep>,
{
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_step::diff::diff_delete_step(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::delete_step::inverse::inverse_delete_step(self, base)?
    
    })
}
}
//#endregion 🗑️DeleteStep
