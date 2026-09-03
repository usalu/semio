//! 🌱 Generation2d mutation — `CreateWidget`: brings a new id-keyed widget into existence at a
//! FINAL-state insertion index. Wired module name (`set_widget`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateWidget
/// 🌱 `create-widget` payload — full initial widget payload plus a FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_widget(index: usize, widget: Widget) -> Generation2dMutation {
    Generation2dMutation::CreateWidget(CreateWidget { index, widget })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for CreateWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create widget \"{}\"", widget_id(&self.widget))
    }
    fn target(&self) -> Vec<String> {
        vec![widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️CreateWidget
