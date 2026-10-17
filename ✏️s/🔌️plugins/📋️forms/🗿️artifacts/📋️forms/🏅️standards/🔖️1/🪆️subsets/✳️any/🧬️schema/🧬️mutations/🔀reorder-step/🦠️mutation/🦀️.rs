//! 🔀️ Forms mutation payload — `reorder-step`, repositioning within the `steps` ordered list (never
//! spatial — see `📓️taxonomy.md`'s `reorder` vs `move` distinction). Physical dir name (`↔️move-step`,
//! wired by `🦀️.rs`) predates the semantic rename; the Rust module is still `move_step`, the
//! type/variant/kind are `reorder-step`.

use crate::{FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔀️ReorderStep
/// 🔀️ Repositions a step to a FINAL-state `to_index` within `steps`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReorderStep {
    pub id: String,
    pub to_index: usize,
}

impl<Op> MutationKind<FormsSnapshot, Op> for ReorderStep
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<ReorderStep>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "step", kind: "reorder-step", record: "ReorderedStep" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        crate::standards::v1::subsets::any::schema::mutations::reorder_step::diff::diff_reorder_step(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::reorder_step::inverse::inverse_reorder_step(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder step \"{}\"", self.id), &format!("Reihenfolge von Schritt \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀️ReorderStep
