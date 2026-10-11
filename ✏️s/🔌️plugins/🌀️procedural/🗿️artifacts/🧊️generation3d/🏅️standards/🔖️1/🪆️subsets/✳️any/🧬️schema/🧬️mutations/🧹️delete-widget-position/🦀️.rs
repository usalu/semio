//! 🗑️ `delete-widget-position` payload — removes a widget's per-widget canvas-position override.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
//#region 🔖️DeleteWidgetPosition
/// 🗑️ Removes the position override for `id`; diff/inverse leaves capture the removed position
/// from `base` so undo is a real `move-widget`.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWidgetPosition {
    pub id: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for DeleteWidgetPosition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget-position", kind: "delete-widget-position", record: "DeletedWidgetPosition" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_widget_position::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::delete_widget_position::inverse::inverse(self, base)?
    
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
