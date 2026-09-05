//! 📍 Generation2d mutation — `MoveWidget`: absolute reposition of a widget's canvas layout entry
//! (creates the entry if the widget had none). Wired module name (`set_layout`) is a leftover of
//! the pre-semantic generic slot this triad was repurposed from — see `sharedFileRequests` in this
//! ticket's wave2 report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use flow::WidgetLayout;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️MoveWidget
/// 📍 `move-widget` payload — the widget's new absolute canvas position.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_widget(id: String, layout: WidgetLayout) -> Generation2dMutation {
    Generation2dMutation::MoveWidget(MoveWidget { id, layout })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for MoveWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move widget \"{}\" to ({}, {})", self.id, self.layout.x, self.layout.y)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️MoveWidget
