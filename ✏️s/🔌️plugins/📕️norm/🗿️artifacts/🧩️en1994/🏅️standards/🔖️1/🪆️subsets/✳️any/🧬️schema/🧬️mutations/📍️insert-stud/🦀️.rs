//! 📍️ `insert-stud` mutation leaf: inserts one stud row.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertStud {
    pub index: usize,
    pub item: semio_s_artifact_norm_en1994_model::Stud,
}

impl<Op> protocol::MutationKind<En1994Snapshot, Op> for InsertStud where Op: protocol::Mutation<En1994Snapshot, Diff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::remove_stud::RemoveStud> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "stud",
        kind: "insert-stud",
        record: "InsertedStud",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert stud {}", self.item.id), &format!("Kopfbolzendübel {} einfügen", self.item.id))
    }
}

impl<Op> protocol::ColdMutationKind<En1994Snapshot, Op> for InsertStud where Op: protocol::ColdMutation<En1994Snapshot, ReferenceDiff = semio_s_artifact_norm_en1994_diff::En1994Diff> + From<crate::remove_stud::RemoveStud> {
    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1994_diff::En1994Diff> {
        crate::insert_stud::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_stud::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
