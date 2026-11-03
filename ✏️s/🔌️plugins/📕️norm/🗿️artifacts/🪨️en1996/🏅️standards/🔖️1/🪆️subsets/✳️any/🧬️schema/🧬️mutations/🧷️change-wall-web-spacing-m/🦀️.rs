//! 🧷 `change-wall-web-spacing-m` mutation leaf: sets `web_spacing_m` of one wall.

use semio_s_artifact_norm_en1996_model::En1996Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeWallWebSpacingM {
    pub id: String,
    pub new_web_spacing_m: f64,
}

impl<Op> protocol::MutationKind<En1996Snapshot, Op> for ChangeWallWebSpacingM where Op: protocol::Mutation<En1996Snapshot, Diff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::change_wall_web_spacing_m::ChangeWallWebSpacingM> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "wall-web-spacing-m",
        kind: "change-wall-web-spacing-m",
        record: "ChangedWallWebSpacingM",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Wall: Web spacing of the shear wall (0 = not given)", "Wand: Stegabstand der Schubwand (0 = nicht angegeben) ändern")
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<En1996Snapshot, Op> for ChangeWallWebSpacingM where Op: protocol::ColdMutation<En1996Snapshot, ReferenceDiff = semio_s_artifact_norm_en1996_diff::En1996Diff> + From<crate::change_wall_web_spacing_m::ChangeWallWebSpacingM> {
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<semio_s_artifact_norm_en1996_diff::En1996Diff> {
        crate::change_wall_web_spacing_m::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_wall_web_spacing_m::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
