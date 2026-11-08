//! 🦴️ `change-sheet-profile-bottom-flange` mutation leaf: sets `bottom_flange` of one connection.

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSheetProfileBottomFlange {
    pub sheet_profile_id: String,
    pub new_bottom_flange: f64,
}

impl<Op> protocol::MutationKind<En1999Snapshot, Op> for ChangeSheetProfileBottomFlange where Op: protocol::Mutation<En1999Snapshot, Diff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_sheet_profile_bottom_flange::ChangeSheetProfileBottomFlange> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "sheet-profile-bottom-flange",
        kind: "change-sheet-profile-bottom-flange",
        record: "ChangedSheetProfileBottomFlange",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Change sheet profile: Bottom flange width b_b", "Blechprofil: Untergurtbreite b_b ändern")
    }
}

impl<Op> protocol::ColdMutationKind<En1999Snapshot, Op> for ChangeSheetProfileBottomFlange where Op: protocol::ColdMutation<En1999Snapshot, ReferenceDiff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::change_sheet_profile_bottom_flange::ChangeSheetProfileBottomFlange> {
    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::change_sheet_profile_bottom_flange::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::change_sheet_profile_bottom_flange::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
