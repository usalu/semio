//! ➕️ `insert-fatigue-band` mutation leaf: inserts one Fatigue band element into the addressed row.

use semio_s_artifact_norm_en1993_model::{FatigueBand, En1993Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertFatigueBand {
    pub id: String,
    pub index: Option<usize>,
    pub element: FatigueBand,
}

impl<Op> protocol::MutationKind<En1993Snapshot, Op> for InsertFatigueBand where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_fatigue_band::RemoveFatigueBand> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "fatigue-band", kind: "insert-fatigue-band", record: "InsertedFatigueBand" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert fatigue band into {}", self.id), &format!("Ermüdungsband in {} einfügen", self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.id.clone(), self.element.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for InsertFatigueBand where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_fatigue_band::RemoveFatigueBand> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::insert_fatigue_band::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { crate::insert_fatigue_band::inverse::inverse(self, base) }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
