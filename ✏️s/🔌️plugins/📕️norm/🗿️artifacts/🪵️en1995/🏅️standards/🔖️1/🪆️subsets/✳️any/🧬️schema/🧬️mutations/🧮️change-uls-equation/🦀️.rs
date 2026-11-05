//! 🧮️ `change-uls-equation` — changes `ulsEquation` (ULS combination equation: 6.10 or 6.10a/b with xi = 0.85) of the document.
use semio_s_artifact_norm_en1995_model::UlsEquation;
use semio_s_artifact_norm_en1995_model::{En1995Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeUlsEquation { pub new_value: UlsEquation }
impl<Op> protocol::MutationKind<En1995Snapshot, Op> for ChangeUlsEquation where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::change_uls_equation::ChangeUlsEquation> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "uls-equation", kind: "change-uls-equation", record: "ChangedUlsEquation" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel { semio_ui_locale::LocalizedLabel::native(&format!("Change ULS combination equation to {}", equation_designation(self.new_value)), &format!("GZT-Kombinationsgleichung auf {} ändern", equation_designation(self.new_value))) }
    fn target(&self) -> Vec<String> { vec![equation_designation(self.new_value).to_owned()] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for ChangeUlsEquation where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<crate::change_uls_equation::ChangeUlsEquation> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::change_uls_equation::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::change_uls_equation::inverse::inverse(self, base)? 
    })
}
}

/// 🔢️ Clause designation of the ULS combination equation (EN 1990 Eq. 6.10 or 6.10a/b).
pub fn equation_designation(equation: UlsEquation) -> &'static str {
    match equation {
        UlsEquation::Eq610 => "6.10",
        UlsEquation::Eq610ab => "6.10a/b",
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
