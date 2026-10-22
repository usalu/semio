//! 🧹 `change-wall-base-width` payload.

use semio_s_artifact_norm_en1997_diff::En1997Diff;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeWallBaseWidth {
    pub id: String,
    pub new_base_width: f64,
}

impl<Op> protocol::MutationKind<En1997Snapshot, Op> for ChangeWallBaseWidth where Op: protocol::Mutation<En1997Snapshot, Diff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_wall_base_width::ChangeWallBaseWidth> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change", entity: "wall-base-width", kind: "change-wall-base-width", record: "ChangedWallBaseWidth",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change retaining wall base width", "Sohlbreite der Stützwand ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1997Snapshot, Op> for ChangeWallBaseWidth where Op: protocol::ColdMutation<En1997Snapshot, ReferenceDiff = semio_s_artifact_norm_en1997_diff::En1997Diff> + From<crate::change_wall_base_width::ChangeWallBaseWidth> {
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::change_wall_base_width::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_wall_base_width::inverse::inverse(self, base)?
    
    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
