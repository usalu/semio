//! 🔢️ `change-part-number-input` — upserts one scripted part-number input value, addressed by key.

use semio_s_artifact_norm_iso16757_model::{CatalogueValue, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangePartNumberInput {
    pub key: String,
    pub new_value: CatalogueValue,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for ChangePartNumberInput where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_part_number_input::mutation::ChangePartNumberInput> + From<crate::remove_part_number_input::mutation::RemovePartNumberInput> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-number-input", kind: "change-part-number-input", record: "ChangedPartNumberInput" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change part-number input \"{}\"", self.key), &format!("Artikelnummer-Eingabe \"{}\" ändern", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for ChangePartNumberInput where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_part_number_input::mutation::ChangePartNumberInput> + From<crate::remove_part_number_input::mutation::RemovePartNumberInput> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::change_part_number_input::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_part_number_input::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
