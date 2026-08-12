//! 🔁️ Whole-value swap of a widget's payload — flow widgets are heterogeneous enum variants, so a
//! granular per-field patch buys nothing (matches `flow::Widget`'s own `Patchable` impl).
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{FlowDiff, FlowSnapshot};
use flow::Widget;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔁️ReplaceWidget
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceWidget {
    pub id: String,
    pub widget: Widget,
}

impl MutationKind<FlowSnapshot, FlowMutation> for ReplaceWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "widget", kind: "replace-widget", record: "ReplacedWidget" };

    fn diff(&self, base: &FlowSnapshot) -> FlowDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FlowSnapshot) -> Vec<FlowMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace widget \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔁️ReplaceWidget
