//! 🪚️ `insert-chase` mutation leaf: inserts one chase row.

use semio_s_artifact_norm_en1996_model::En1996Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertChase {
    pub index: usize,
    pub item: semio_s_artifact_norm_en1996_model::WallChase,
}

impl<Op> protocol::MutationKind<En1996Snapshot, Op> for InsertChase where Op: protocol::Mutation<En1996Snapshot, Diff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::remove_chase::RemoveChase> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "chase",
        kind: "insert-chase",
        record: "InsertedChase",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert chase {}", self.item.id), &format!("Schlitz {} einfügen", self.item.id))
    }
}

impl<Op> protocol::ColdMutationKind<En1996Snapshot, Op> for InsertChase where Op: protocol::ColdMutation<En1996Snapshot, ReferenceDiff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::remove_chase::RemoveChase> {
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1996_diff::En1996Diff> {
        crate::insert_chase::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_chase::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
