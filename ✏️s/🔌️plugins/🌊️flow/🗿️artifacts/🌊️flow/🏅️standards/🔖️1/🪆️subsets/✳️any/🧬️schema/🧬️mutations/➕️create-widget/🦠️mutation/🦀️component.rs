//! ➕️ Brings a new [`Widget`] into existence at `index`.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use flow::Widget;
use protocol::{Identified, MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ➕️CreateWidget
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

impl MutationKind<FlowSnapshot, FlowMutation> for CreateWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    async fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create widget \"{}\"", self.widget.id())
    }
    async fn target(&self) -> Vec<String> {
        vec![self.widget.id().clone()]
    }
}
//#endregion ➕️CreateWidget
