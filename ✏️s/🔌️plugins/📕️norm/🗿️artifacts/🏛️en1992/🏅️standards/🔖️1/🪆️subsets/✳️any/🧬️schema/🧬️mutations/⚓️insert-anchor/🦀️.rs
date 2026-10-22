//! ⚓️ `insert-anchor`.

use semio_s_artifact_norm_en1992_diff::En1992Diff;

use semio_s_artifact_norm_en1992_model::{Anchor, En1992Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertAnchor {
    pub index: usize,
    pub anchor: Anchor,
}

impl<Op> protocol::MutationKind<En1992Snapshot, Op> for InsertAnchor where Op: protocol::Mutation<En1992Snapshot, Diff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::remove_anchor::RemoveAnchor> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "anchor", kind: "insert-anchor", record: "InsertedAnchor" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert anchor {}", self.anchor.id), &format!("Dübel {} einfügen", self.anchor.id))
    }
}

impl<Op> protocol::ColdMutationKind<En1992Snapshot, Op> for InsertAnchor where Op: protocol::ColdMutation<En1992Snapshot, ReferenceDiff = semio_s_artifact_norm_en1992_diff::En1992Diff> + From<crate::remove_anchor::RemoveAnchor> {
    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> { crate::insert_anchor::diff::diff(self, base) }
    fn inverse(&self, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ crate::insert_anchor::inverse::inverse(self, base)? 
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
