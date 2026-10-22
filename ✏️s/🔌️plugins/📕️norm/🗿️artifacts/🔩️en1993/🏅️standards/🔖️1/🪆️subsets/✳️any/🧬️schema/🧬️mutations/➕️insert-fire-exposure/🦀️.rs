//! ➕️ insert-fire-exposure
use semio_s_artifact_norm_en1993_model::{FireExposure, En1993Snapshot};
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertFireExposure { pub index: Option<usize>, pub fire_exposure: FireExposure }
impl<Op> protocol::MutationKind<En1993Snapshot, Op> for InsertFireExposure where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_fire_exposure::RemoveFireExposure> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "fire-exposure", kind: "insert-fire-exposure", record: "InsertedFireExposure" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert fire exposure at position #{}", self.index.map_or_else(|| "Ende".to_string(), |index| index.to_string())), &format!("Brandbeanspruchung an Position #{} einfügen", self.index.map_or_else(|| "end".to_string(), |index| index.to_string()))) }
    fn target(&self) -> Vec<String> { vec![self.index.map_or_else(|| "end".to_string(), |index| index.to_string())] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for InsertFireExposure where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::remove_fire_exposure::RemoveFireExposure> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::insert_fire_exposure::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_fire_exposure::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
