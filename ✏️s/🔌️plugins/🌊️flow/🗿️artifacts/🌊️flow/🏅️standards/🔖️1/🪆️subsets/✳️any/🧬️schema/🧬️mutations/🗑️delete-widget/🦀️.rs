//! 🗑️ Removes a widget by id, capturing the cascade (severed synapses + layout entry) for undo.

use crate::artifacts::flow::FlowSnapshot;
use crate::artifacts::flow::schema::diff::text::FlowDiff;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🗑️DeleteWidget
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DeleteWidget {
    pub id: String,
}

impl MutationKind<FlowSnapshot, FlowMutation> for DeleteWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete widget \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteWidget
