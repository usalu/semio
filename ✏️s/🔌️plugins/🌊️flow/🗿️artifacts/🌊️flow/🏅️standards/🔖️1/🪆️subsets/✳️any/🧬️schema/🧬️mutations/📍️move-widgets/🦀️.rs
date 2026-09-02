//! 📍️ Absolute repositions (or clears, when an entry's `layout` is `None`) one or more widgets at
//! once. Plural by taxonomy design (`## Bulk / plural mutations`): the framework host bridge's own
//! diffing (`flow::flow_fixture_operations`) already batches every changed layout key into one
//! `SetLayout` op per real drag gesture, so this mirrors that batch 1:1 rather than splitting into
//! per-widget mutations.

use crate::artifacts::flow::FlowSnapshot;
use crate::artifacts::flow::schema::diff::text::FlowDiff;
use crate::artifacts::flow::schema::mutations::FlowMutation;
use flow::FlowLayoutEntry;
use protocol::{MutationKind, SemanticDescriptor};

//#region 📍️MoveWidgets
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MoveWidgets {
    pub entries: Vec<FlowLayoutEntry>,
}

impl MutationKind<FlowSnapshot, FlowMutation> for MoveWidgets {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "widgets", kind: "move-widgets", record: "MovedWidgets" };

    fn diff(&self, base: &FlowSnapshot) -> protocol::MutationOutcome<FlowDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move {} widget(s)", self.entries.len())
    }
    fn target(&self) -> Vec<String> {
        self.entries.iter().map(|entry| entry.id.clone()).collect()
    }
}
//#endregion 📍️MoveWidgets
