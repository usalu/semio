//! 🔁️ `change-edition-profile` — upserts one sheet's edition-profile override, addressed by sheet
//! number (the format's native key — `crate::edition_profile` is name/code-keyed,
//! not id-keyed).

use crate::{EditionProfileChoice, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeEditionProfile {
    pub sheet: String,
    pub new_choice: EditionProfileChoice,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeEditionProfile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edition-profile", kind: "change-edition-profile", record: "ChangedEditionProfile" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
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
