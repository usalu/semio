//! 🦠️ `🔁replace-widget` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceWidget {
    pub widget: Widget,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_widget(widget: Widget) -> Generation2dMutation {
    Generation2dMutation::ReplaceWidget(ReplaceWidget { widget })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for ReplaceWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "widget", kind: "replace-widget", record: "ReplacedWidget" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace widget \"{}\"", widget_id(&self.widget))
    }
    fn target(&self) -> Vec<String> {
        vec![widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️Mutation
