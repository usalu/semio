//! 📅️ `change-correction-as-of` — sets the document root's correction cut-off edition.

use crate::{EditionId, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeCorrectionAsOf {
    pub new_correction_as_of: EditionId,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeCorrectionAsOf {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "correction-as-of", kind: "change-correction-as-of", record: "ChangedCorrectionAsOf" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change correction date to {}-{:02}", self.new_correction_as_of.year, self.new_correction_as_of.month), &format!("Korrekturstand auf {}-{:02} ändern", self.new_correction_as_of.year, self.new_correction_as_of.month))
    }
}
//#endregion 🔖️Payload
