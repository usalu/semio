//! 🖨️ `change-print-target` — sets the document's `print_target` scalar (`None` clears it).

use crate::mutations::LayoutMutation;
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖨️ChangePrintTarget
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePrintTarget {
    pub new_print_target: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePrintTarget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "print-target", kind: "change-print-target", record: "ChangedPrintTarget" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_print_target(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_print_target(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&{
        match &self.new_print_target {
            Some(target) => format!("Set print target to \"{target}\""),
            None => "Clear print target".into(),
        }
        }, &{
        match &self.new_print_target {
            Some(target) => format!("Druckziel auf \"{target}\" setzen"),
            None => "Druckziel leeren".into(),
        }
        })
    }
}
//#endregion 🖨️ChangePrintTarget

//#region 🖨️ChangePrintTarget
pub fn diff_change_print_target(payload: &ChangePrintTarget, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.print_target == payload.new_print_target {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Print target is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { print_target: Some(crate::standards::v1::subsets::any::schema::diff::PrintTargetChange { target: payload.new_print_target.clone() }), ..Default::default() })
}
//#endregion 🖨️ChangePrintTarget

//#region 🖨️ChangePrintTarget
pub fn inverse_change_print_target(_payload: &ChangePrintTarget, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![LayoutMutation::ChangePrintTarget(ChangePrintTarget { new_print_target: base.print_target.clone() })]

    })())
}
//#endregion 🖨️ChangePrintTarget
