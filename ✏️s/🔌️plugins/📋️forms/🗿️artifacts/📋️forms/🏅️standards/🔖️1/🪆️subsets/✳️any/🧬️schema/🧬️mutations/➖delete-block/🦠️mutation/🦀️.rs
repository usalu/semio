//! ✂️ Forms mutation payload — `delete-block`, a step's `blocks` id-keyed nested collection's
//! `delete` verb. Physical dir name (`➖remove-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `remove_block`, the type/variant/kind are `delete-block`.

use crate::{FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region ✂️DeleteBlock
/// ✂️ Removes a block by id from `step_id`'s `blocks`. Inverse recreates it (with its captured base
/// position) via `create-block`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteBlock {
    pub step_id: String,
    pub id: String,
}

impl<Op> MutationKind<FormsSnapshot, Op> for DeleteBlock
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<DeleteBlock> + From<crate::standards::v1::subsets::any::schema::mutations::create_block::mutation::CreateBlock>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "block", kind: "delete-block", record: "DeletedBlock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete block \"{}\"", self.id), &format!("Block \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<FormsSnapshot, Op> for DeleteBlock
where
    Op: protocol::ColdMutation<FormsSnapshot, ReferenceDiff = FormsDiff> + From<DeleteBlock> + From<crate::standards::v1::subsets::any::schema::mutations::create_block::mutation::CreateBlock>,
{
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_block::diff::diff_delete_block(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::delete_block::inverse::inverse_delete_block(self, base)?
    
    })
}
}
//#endregion ✂️DeleteBlock
