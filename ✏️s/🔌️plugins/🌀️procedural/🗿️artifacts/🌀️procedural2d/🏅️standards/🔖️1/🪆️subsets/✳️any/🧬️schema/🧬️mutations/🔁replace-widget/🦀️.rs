//! 🦠️ `🔁replace-widget` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
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
pub fn replace_widget(widget: Widget) -> Procedural2dMutation {
    Procedural2dMutation::ReplaceWidget(ReplaceWidget { widget })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for ReplaceWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "widget", kind: "replace-widget", record: "ReplacedWidget" };

    fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
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
