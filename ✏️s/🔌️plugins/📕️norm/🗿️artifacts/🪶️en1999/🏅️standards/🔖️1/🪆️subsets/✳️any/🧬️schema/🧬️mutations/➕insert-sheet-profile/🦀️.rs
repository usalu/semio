//! ➕ `insert-sheet-profile` mutation leaf.

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_model::SheetProfile;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertSheetProfile {
    pub index: u32,
    pub sheet_profile: SheetProfile,
}

impl<Op> protocol::MutationKind<En1999Snapshot, Op> for InsertSheetProfile where Op: protocol::Mutation<En1999Snapshot, Diff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::remove_sheet_profile::RemoveSheetProfile> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "add",
        entity: "sheet-profile",
        kind: "insert-sheet-profile",
        record: "AddedSheetProfile",
    };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Insert sheet profile", "Blechprofil einfügen")
    }
}

impl<Op> protocol::ColdMutationKind<En1999Snapshot, Op> for InsertSheetProfile where Op: protocol::ColdMutation<En1999Snapshot, ReferenceDiff = semio_s_artifact_norm_en1999_diff::En1999Diff> + From<crate::remove_sheet_profile::RemoveSheetProfile> {
    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        crate::insert_sheet_profile::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::insert_sheet_profile::inverse::inverse(self, base)?

    })
}
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
