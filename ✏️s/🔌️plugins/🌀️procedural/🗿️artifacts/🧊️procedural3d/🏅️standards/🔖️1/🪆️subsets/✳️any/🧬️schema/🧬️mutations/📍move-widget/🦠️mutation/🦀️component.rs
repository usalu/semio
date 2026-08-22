//! 📍 `move-widget` payload — absolute spatial reposition of a widget's canvas position
//! (`📓️taxonomy.md`'s `move` row: "Absolute spatial reposition", addr + position).
//!
//! Directory kept at its pre-migration `🎛set-layout` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::WidgetLayout;
use serde::{Deserialize, Serialize};

//#region 🔖️MoveWidget
/// 📍 Places `id`'s position at `layout`, upserting the per-widget override entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for MoveWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };

    fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::move_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::move_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Move widget \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️MoveWidget
