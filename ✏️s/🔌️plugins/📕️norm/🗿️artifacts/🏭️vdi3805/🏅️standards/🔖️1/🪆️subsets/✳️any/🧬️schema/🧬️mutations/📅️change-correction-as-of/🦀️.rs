//! 📅️ `change-correction-as-of` — sets the document root's correction cut-off edition.

use semio_s_artifact_norm_vdi3805_model::{EditionId, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCorrectionAsOf {
    pub new_correction_as_of: EditionId,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for ChangeCorrectionAsOf where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_correction_as_of::ChangeCorrectionAsOf> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "correction-as-of", kind: "change-correction-as-of", record: "ChangedCorrectionAsOf" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change correction date to {}-{:02}", self.new_correction_as_of.year, self.new_correction_as_of.month), &format!("Korrekturstand auf {}-{:02} ändern", self.new_correction_as_of.year, self.new_correction_as_of.month))
    }
}

impl<Op> protocol::ColdMutationKind<Vdi3805Snapshot, Op> for ChangeCorrectionAsOf where Op: protocol::ColdMutation<Vdi3805Snapshot, ReferenceDiff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_correction_as_of::ChangeCorrectionAsOf> {
    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Vdi3805Snapshot>>::ReferenceDiff> {
        crate::change_correction_as_of::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_correction_as_of::inverse::inverse(self, base)?
    
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
