//! 🗑️ `delete-widget` payload — removes an id-keyed [`Widget`] from the fixture. It does NOT cascade: the wires naming
//! the widget and its layout entry stay standing, and a command that deletes a widget spells that cascade as leaves of
//! its own (`generation3d_widget_removal`: `disconnect-synapse`, `delete-widget-position`, then this leaf).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
//#region 🔖️DeleteWidget
/// 🗑️ Removes the widget with `id`; the diff/inverse leaves capture the full removed payload from
/// `base` so undo is a real `create-widget`, never a sentinel.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWidget {
    pub id: String,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for DeleteWidget
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::create_widget::CreateWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::delete_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::delete_widget::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete widget \"{}\"", self.id), &format!("Widget \"{}\" löschen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidget
