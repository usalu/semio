//! 🗑️ `delete-widget-position` payload — removes a widget's per-widget canvas-position override.
//!
//! Directory kept at its pre-migration `➖remove-layout` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteWidgetPosition
/// 🗑️ Removes the position override for `id`; diff/inverse leaves capture the removed position
/// from `base` so undo is a real `move-widget`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidgetPosition {
    pub id: String,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for DeleteWidgetPosition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget-position", kind: "delete-widget-position", record: "DeletedWidgetPosition" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::delete_widget_position::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::delete_widget_position::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete widget position \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidgetPosition
