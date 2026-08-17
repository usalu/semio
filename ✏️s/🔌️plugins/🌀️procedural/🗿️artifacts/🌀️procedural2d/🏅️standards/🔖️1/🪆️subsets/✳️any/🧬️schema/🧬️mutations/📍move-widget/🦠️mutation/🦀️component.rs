//! 📍 Procedural2d mutation — `MoveWidget`: absolute reposition of a widget's canvas layout entry
//! (creates the entry if the widget had none). Wired module name (`set_layout`) is a leftover of
//! the pre-semantic generic slot this triad was repurposed from — see `sharedFileRequests` in this
//! ticket's wave2 report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::WidgetLayout;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️MoveWidget
/// 📍 `move-widget` payload — the widget's new absolute canvas position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_widget(id: String, layout: WidgetLayout) -> Procedural2dMutation {
    Procedural2dMutation::MoveWidget(MoveWidget { id, layout })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for MoveWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
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
