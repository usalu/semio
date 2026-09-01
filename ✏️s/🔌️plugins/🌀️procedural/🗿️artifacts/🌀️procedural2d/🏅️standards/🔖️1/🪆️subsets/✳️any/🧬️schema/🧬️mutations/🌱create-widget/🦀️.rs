//! 🌱 Procedural2d mutation — `CreateWidget`: brings a new id-keyed widget into existence at a
//! FINAL-state insertion index. Wired module name (`set_widget`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateWidget
/// 🌱 `create-widget` payload — full initial widget payload plus a FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_widget(index: usize, widget: Widget) -> Procedural2dMutation {
    Procedural2dMutation::CreateWidget(CreateWidget { index, widget })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for CreateWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
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
