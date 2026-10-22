//! ➖ `remove-roofs`.

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveRoofs {
    pub index: usize,
}

impl<Op> protocol::MutationKind<En1991Snapshot, Op> for RemoveRoofs where Op: protocol::Mutation<En1991Snapshot, Diff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::insert_roofs::InsertRoofs> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "remove",
        entity: "roofs",
        kind: "remove-roofs",
        record: "RemovedRoofs",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove roof", "Dach entfernen")
    }
}

impl<Op> protocol::ColdMutationKind<En1991Snapshot, Op> for RemoveRoofs where Op: protocol::ColdMutation<En1991Snapshot, ReferenceDiff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::insert_roofs::InsertRoofs> {
    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1991Snapshot>>::ReferenceDiff> {
        crate::remove_roofs::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_roofs::inverse::inverse(self, base)?
    
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
