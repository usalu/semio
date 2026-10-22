//! 🖨️ `change-print-target` — sets the document's `print_target` scalar (`None` clears it).


use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖨️ChangePrintTarget
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePrintTarget {
    pub new_print_target: Option<String>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ChangePrintTarget where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_print_target::ChangePrintTarget> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "print-target", kind: "change-print-target", record: "ChangedPrintTarget" };

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

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for ChangePrintTarget where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_print_target::ChangePrintTarget> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_print_target(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_print_target(self, base)?
    
    })
}
}
//#endregion 🖨️ChangePrintTarget

//#region 🖨️ChangePrintTarget
pub fn diff_change_print_target(payload: &ChangePrintTarget, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.print_target == payload.new_print_target {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Print target is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { print_target: Some(semio_s_artifact_layout_layout_diff::PrintTargetChange { target: payload.new_print_target.clone() }), ..Default::default() })
}
//#endregion 🖨️ChangePrintTarget

//#region 🖨️ChangePrintTarget
pub fn inverse_change_print_target<Op: From<crate::change_print_target::ChangePrintTarget>>(_payload: &ChangePrintTarget, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(ChangePrintTarget { new_print_target: base.print_target.clone() })]

    })())
}
//#endregion 🖨️ChangePrintTarget

