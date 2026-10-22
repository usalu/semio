//! 🧹 `insert-pile` payload.

use semio_s_artifact_norm_en1997_diff::En1997Diff;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertPile {
    pub index: usize,
    pub pile: semio_s_artifact_norm_en1997_model::Pile,
}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for InsertPile where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::remove_pile::RemovePile> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert", entity: "pile", kind: "insert-pile", record: "InsertedPile",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert pile", "Pfahl einfügen")
    }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for InsertPile where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::remove_pile::RemovePile> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::insert_pile::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::insert_pile::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
