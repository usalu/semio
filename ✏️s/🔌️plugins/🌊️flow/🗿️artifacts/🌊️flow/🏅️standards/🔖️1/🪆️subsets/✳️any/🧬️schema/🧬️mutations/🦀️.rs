//! 🌊️ Flow parent mutation vocabulary — empty by design (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §12,
//! §20.15): a flow's widgets, synapses and layout live in its composed `content` child (`s.stdio.semio@v1/flow`), so every
//! content edit is a child-lane leaf in that child's store (`insert-node`, `remove-edge`, `drag-nodes`, `set-node-param`, …)
//! and the parent owns no leaf that could read the child. Editors publish those child leaves
//! (`crate::editor::flow::edit_rules::ContentEdit`, `crate::editor::flow::flow_removal_leaves`).

use crate::standards::v1::subsets::any::schema::diff::FlowDiff;
use crate::FlowSnapshot;

//#region 🔖️Aggregate
/// 🕳️ The uninhabited parent vocabulary of a document whose whole content is its composed child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum FlowMutation {}

impl protocol::Mutation<FlowSnapshot> for FlowMutation {
    type Diff = FlowDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match *self {}
    }
    fn diff(&self, _base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        match *self {}
    }
    fn inverse(&self, _base: &FlowSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        match *self {}
    }
}

/// 🏷️ No parent kind exists, so no parent operation is ever labelled; child leaves label their own rows.
impl protocol::SemanticMutation<FlowSnapshot> for FlowMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        &[]
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        match *self {}
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match *self {}
    }
    fn target(&self) -> Vec<String> {
        match *self {}
    }
}





/// 🧊️ Nothing to retire: the vocabulary is uninhabited.
impl flow::neural::ColdRetire for FlowMutation {
    fn retire_cold(self) {
        match self {}
    }
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
