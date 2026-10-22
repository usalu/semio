//! ➖️ `remove-silo` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveSilo {
    pub index: usize,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for RemoveSilo where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::insert_silo::InsertSilo> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "remove",
        entity: "silo",
        kind: "remove-silo",
        record: "RemoveSilo",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove silo", "Silo entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for RemoveSilo where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::insert_silo::InsertSilo> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::remove_silo::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_silo::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
