//! 🧹 Generation2d mutation — `ClearWidgetLayout`: empties a widget's canvas layout entry wholesale
//! (e.g. resetting to auto-layout). Wired module name (`remove_layout`) is a leftover of the
//! pre-semantic generic slot this triad was repurposed from — see `sharedFileRequests` in this
//! ticket's wave2 report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ClearWidgetLayout
/// 🧹 `clear-widget-layout` payload — removes the layout entry for widget `id`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ClearWidgetLayout {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn clear_widget_layout(id: String) -> Generation2dMutation {
    Generation2dMutation::ClearWidgetLayout(ClearWidgetLayout { id })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for ClearWidgetLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "widget-layout", kind: "clear-widget-layout", record: "ClearedWidgetLayout" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Clear layout for widget \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ClearWidgetLayout
