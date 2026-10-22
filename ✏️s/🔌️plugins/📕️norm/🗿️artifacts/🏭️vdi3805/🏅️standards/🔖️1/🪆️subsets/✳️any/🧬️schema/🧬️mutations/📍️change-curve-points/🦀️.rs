//! 📉️ `change-curve-points` — whole-value swap of a curve's interpolation point list, addressed
//! by id.

use semio_s_artifact_norm_vdi3805_model::{CurvePoint, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCurvePoints {
    pub id: String,
    pub new_points: Vec<CurvePoint>,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for ChangeCurvePoints where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_curve_points::ChangeCurvePoints> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "curve-points", kind: "change-curve-points", record: "ChangedCurvePoints" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace points for curve \"{}\"", self.id), &format!("Punkte für Kurve \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Vdi3805Snapshot, Op> for ChangeCurvePoints where Op: protocol::ColdMutation<Vdi3805Snapshot, ReferenceDiff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_curve_points::ChangeCurvePoints> {
    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Vdi3805Snapshot>>::ReferenceDiff> {
        crate::change_curve_points::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_curve_points::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
