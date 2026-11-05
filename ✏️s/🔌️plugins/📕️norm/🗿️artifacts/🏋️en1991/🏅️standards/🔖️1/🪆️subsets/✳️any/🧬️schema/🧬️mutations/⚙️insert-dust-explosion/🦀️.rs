//! ⚙ `insert-dust-explosion` mutation leaf: inserts one dust explosion row.

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertDustExplosion {
    pub index: usize,
    pub item: semio_s_artifact_norm_en1991_model::DustExplosion,
}

impl<Op> protocol::MutationKind<En1991Snapshot, Op> for InsertDustExplosion where Op: protocol::Mutation<En1991Snapshot, Diff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::remove_dust_explosion::RemoveDustExplosion> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "dust-explosion",
        kind: "insert-dust-explosion",
        record: "InsertedDustExplosion",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native(&format!("Insert dust explosion {}", self.item.id), &format!("Staubexplosion {} einfügen", self.item.id))
    }
}

impl<Op> protocol::ColdMutationKind<En1991Snapshot, Op> for InsertDustExplosion where Op: protocol::ColdMutation<En1991Snapshot, ReferenceDiff = semio_s_artifact_norm_en1991_diff::En1991Diff> + From<crate::remove_dust_explosion::RemoveDustExplosion> {
    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1991_diff::En1991Diff> {
        crate::insert_dust_explosion::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::insert_dust_explosion::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
