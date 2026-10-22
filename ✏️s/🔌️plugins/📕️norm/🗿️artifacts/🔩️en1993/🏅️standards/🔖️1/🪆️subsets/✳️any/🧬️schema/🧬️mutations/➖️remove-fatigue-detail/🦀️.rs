//! ➖️ remove-fatigue-detail
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveFatigueDetail { pub index: usize }
impl<Op> protocol::MutationKind<En1993Snapshot, Op> for RemoveFatigueDetail where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::insert_fatigue_detail::InsertFatigueDetail> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "fatigue-detail", kind: "remove-fatigue-detail", record: "RemovedFatigueDetail" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove fatigue detail #{}", self.index), &format!("Kerbdetail #{} entfernen", self.index)) }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for RemoveFatigueDetail where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::insert_fatigue_detail::InsertFatigueDetail> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::remove_fatigue_detail::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::remove_fatigue_detail::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
