//! 🖨️ `change-print-target` — sets the document's `print_target` scalar (`None` clears it).


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::LayoutMutation;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖨️ChangePrintTarget
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangePrintTarget {
    pub new_print_target: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePrintTarget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "print-target", kind: "change-print-target", record: "ChangedPrintTarget" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_print_target(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_print_target(self, base)
    }
    async fn label(&self) -> String {
        match &self.new_print_target {
            Some(target) => format!("Set print target to \"{target}\""),
            None => "Clear print target".into(),
        }
    }
}
//#endregion 🖨️ChangePrintTarget


//#region 🖨️ChangePrintTarget
pub async fn diff_change_print_target(payload: &ChangePrintTarget, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.print_target == payload.new_print_target {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Print target is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { print_target: Some(payload.new_print_target.clone()), ..Default::default() })
}
//#endregion 🖨️ChangePrintTarget


//#region 🖨️ChangePrintTarget
pub async fn inverse_change_print_target(_payload: &ChangePrintTarget, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::ChangePrintTarget(ChangePrintTarget { new_print_target: base.print_target.clone() })]
}
//#endregion 🖨️ChangePrintTarget
