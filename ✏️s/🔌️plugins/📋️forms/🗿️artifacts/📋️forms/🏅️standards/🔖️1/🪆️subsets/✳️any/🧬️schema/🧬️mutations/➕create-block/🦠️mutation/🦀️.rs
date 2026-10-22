//! 🧩️ Forms mutation payload — `create-block`, a step's `blocks` id-keyed nested collection's
//! `create` verb. Physical dir name (`➕add-block`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `add_block`, the type/variant/kind are `create-block`.

use crate::{FormQuestion, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🧩️CreateBlock
/// 🧩️ Brings a new [`FormQuestion`] into existence inside `step_id`'s `blocks`, at an optional
/// FINAL-state `index` (`None` appends). An unknown `step_id` is Error `mutation.target-missing`; a
/// duplicate `block.id` within that step is Fatal `mutation.duplicate-id`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateBlock {
    pub step_id: String,
    pub block: FormQuestion,
    pub index: Option<usize>,
}

impl<Op> MutationKind<FormsSnapshot, Op> for CreateBlock
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<CreateBlock> + From<crate::standards::v1::subsets::any::schema::mutations::delete_block::mutation::DeleteBlock>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "block", kind: "create-block", record: "CreatedBlock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create block \"{}\"", self.block.label), &format!("Block \"{}\" erstellen", self.block.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<FormsSnapshot, Op> for CreateBlock
where
    Op: protocol::ColdMutation<FormsSnapshot, ReferenceDiff = FormsDiff> + From<CreateBlock> + From<crate::standards::v1::subsets::any::schema::mutations::delete_block::mutation::DeleteBlock>,
{
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_block::diff::diff_create_block(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::create_block::inverse::inverse_create_block(self, base)?
    
    })
}
}
//#endregion 🧩️CreateBlock
