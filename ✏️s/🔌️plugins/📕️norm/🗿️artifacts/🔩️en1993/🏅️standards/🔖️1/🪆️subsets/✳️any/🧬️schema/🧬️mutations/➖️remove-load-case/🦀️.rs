//! ➖️ remove-load-case
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveLoadCase { pub index: usize, }
impl<Op> protocol::MutationKind<En1993Snapshot, Op> for RemoveLoadCase where Op: protocol::Mutation<En1993Snapshot, Diff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::insert_load_case::InsertLoadCase> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "load-case", kind: "remove-load-case", record: "RemovedLoadCase" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove load case #{}", self.index), &format!("Lastfall #{} entfernen", self.index)) }
    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }
}

impl<Op> protocol::ColdMutationKind<En1993Snapshot, Op> for RemoveLoadCase where Op: protocol::ColdMutation<En1993Snapshot, ReferenceDiff = semio_s_artifact_norm_en1993_diff::En1993Diff> + From<crate::insert_load_case::InsertLoadCase> {
    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1993Snapshot>>::ReferenceDiff> { crate::remove_load_case::diff::diff(self, base) }
    fn inverse(&self, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::remove_load_case::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
