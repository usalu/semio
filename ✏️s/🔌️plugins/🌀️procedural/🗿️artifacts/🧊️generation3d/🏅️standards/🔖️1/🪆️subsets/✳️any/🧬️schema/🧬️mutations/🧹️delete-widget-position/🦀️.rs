//! 🗑️ `delete-widget-position` payload — removes a widget's per-widget canvas-position override.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
//#region 🔖️DeleteWidgetPosition
/// 🗑️ Removes the position override for `id`; diff/inverse leaves capture the removed position
/// from `base` so undo is a real `move-widget`.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWidgetPosition {
    pub id: String,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for DeleteWidgetPosition
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::move_widget::MoveWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget-position", kind: "delete-widget-position", record: "DeletedWidgetPosition" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::delete_widget_position::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::delete_widget_position::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete widget position \"{}\"", self.id), &format!("Widget-Position \"{}\" löschen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidgetPosition
