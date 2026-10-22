//! 🧹 `remove-layer` payload.

use semio_s_artifact_norm_en1997_diff::En1997Diff;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveLayer {
    pub index: usize,
}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for RemoveLayer where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::insert_layer::InsertLayer> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "remove", entity: "layer", kind: "remove-layer", record: "RemovedLayer",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove soil layer", "Bodenschicht entfernen")
    }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for RemoveLayer where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::insert_layer::InsertLayer> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::remove_layer::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_layer::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
