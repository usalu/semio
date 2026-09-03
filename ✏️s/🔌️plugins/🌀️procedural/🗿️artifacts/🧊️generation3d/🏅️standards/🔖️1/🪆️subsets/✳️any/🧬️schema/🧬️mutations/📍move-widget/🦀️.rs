//! 📍 `move-widget` payload — absolute spatial reposition of a widget's canvas position
//! (`📓️taxonomy.md`'s `move` row: "Absolute spatial reposition", addr + position).
//!
//! Directory kept at its pre-migration `🎛set-layout` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use flow::WidgetLayout;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️MoveWidget
/// 📍 Places `id`'s position at `layout`, upserting the per-widget override entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for MoveWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::move_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::move_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Move widget \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️MoveWidget
