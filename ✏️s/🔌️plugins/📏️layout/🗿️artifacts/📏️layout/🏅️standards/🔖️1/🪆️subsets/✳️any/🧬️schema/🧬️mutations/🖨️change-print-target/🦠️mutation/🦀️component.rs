//! 🖨️ `change-print-target` — sets the document's `print_target` scalar (`None` clears it).

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🖨️ChangePrintTarget
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePrintTarget {
    pub new_print_target: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePrintTarget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "print-target", kind: "change-print-target", record: "ChangedPrintTarget" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_print_target(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_print_target(self, base)
    }
    async fn label(&self) -> String {
        match &self.new_print_target {
            Some(target) => format!("Set print target to \"{target}\""),
            None => "Clear print target".into(),
        }
    }
}
//#endregion 🖨️ChangePrintTarget
