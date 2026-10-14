//! 🔁️ `change-edition-profile` — upserts one sheet's edition-profile override, addressed by sheet
//! number (the format's native key — `crate::edition_profile` is name/code-keyed,
//! not id-keyed).

use semio_s_artifact_norm_vdi3805_model::{EditionProfileChoice, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeEditionProfile {
    pub sheet: String,
    pub new_choice: EditionProfileChoice,
}

impl<Op> protocol::MutationKind<Vdi3805Snapshot, Op> for ChangeEditionProfile where Op: protocol::Mutation<Vdi3805Snapshot, Diff = semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff> + From<crate::change_edition_profile::ChangeEditionProfile> + From<crate::remove_edition_profile::RemoveEditionProfile> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edition-profile", kind: "change-edition-profile", record: "ChangedEditionProfile" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        crate::change_edition_profile::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_edition_profile::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change edition profile for sheet {} to {:?}", self.sheet, self.new_choice), &format!("Ausgabeprofil für Blatt {} auf {:?} ändern", self.sheet, self.new_choice))
    }
    fn target(&self) -> Vec<String> {
        vec![self.sheet.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
