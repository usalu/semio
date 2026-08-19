//! 🧹 Procedural2d mutation — `ClearWidgetLayout`: empties a widget's canvas layout entry wholesale
//! (e.g. resetting to auto-layout). Wired module name (`remove_layout`) is a leftover of the
//! pre-semantic generic slot this triad was repurposed from — see `sharedFileRequests` in this
//! ticket's wave2 report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ClearWidgetLayout
/// 🧹 `clear-widget-layout` payload — removes the layout entry for widget `id`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearWidgetLayout {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn clear_widget_layout(id: String) -> Procedural2dMutation {
    Procedural2dMutation::ClearWidgetLayout(ClearWidgetLayout { id })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for ClearWidgetLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "widget-layout", kind: "clear-widget-layout", record: "ClearedWidgetLayout" };

    async fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Clear layout for widget \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ClearWidgetLayout
