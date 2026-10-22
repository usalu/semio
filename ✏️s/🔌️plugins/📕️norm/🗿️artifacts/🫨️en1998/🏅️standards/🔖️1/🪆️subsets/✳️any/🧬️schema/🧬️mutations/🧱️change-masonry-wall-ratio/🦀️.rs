//! 🧱 `change-masonry-wall-ratio` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMasonryWallRatio {
    pub building_index: usize,
    pub new_masonry_wall_area_ratio: f64,
}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for ChangeMasonryWallRatio where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_masonry_wall_ratio::ChangeMasonryWallRatio> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "masonry-wall-ratio",
        kind: "change-masonry-wall-ratio",
        record: "ChangeMasonryWallRatio",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change masonry wall area ratio", "Mauerwerkswand-Flächenanteil ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-masonry-wall-ratio".into()]
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for ChangeMasonryWallRatio where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + From<crate::change_masonry_wall_ratio::ChangeMasonryWallRatio> {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::change_masonry_wall_ratio::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_masonry_wall_ratio::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
