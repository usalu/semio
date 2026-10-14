//! 📍 `move-widget` payload — absolute spatial reposition of a widget's canvas position
//! (`📓️taxonomy.md`'s `move` row: "Absolute spatial reposition", addr + position).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::WidgetLayout;
//#region 🔖️MoveWidget
/// 📍 Places `id`'s position at `layout`, upserting the per-widget override entry.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for MoveWidget
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<MoveWidget> + From<crate::delete_widget_position::DeleteWidgetPosition>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::move_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_widget::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move widget \"{}\"", self.id), &format!("Widget \"{}\" verschieben", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️MoveWidget
