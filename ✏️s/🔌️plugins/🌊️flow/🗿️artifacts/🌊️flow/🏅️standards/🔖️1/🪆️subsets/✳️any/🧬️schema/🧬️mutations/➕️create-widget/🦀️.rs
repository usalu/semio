//! ➕️ Brings a new [`Widget`] into existence at `index`.

use crate::artifacts::flow::FlowSnapshot;
use crate::artifacts::flow::schema::diff::text::FlowDiff;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use flow::Widget;
use protocol::{Identified, MutationKind, SemanticDescriptor};

//#region ➕️CreateWidget
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

impl MutationKind<FlowSnapshot, FlowMutation> for CreateWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create widget \"{}\"", self.widget.id())
    }
    fn target(&self) -> Vec<String> {
        vec![self.widget.id().clone()]
    }
}
//#endregion ➕️CreateWidget
