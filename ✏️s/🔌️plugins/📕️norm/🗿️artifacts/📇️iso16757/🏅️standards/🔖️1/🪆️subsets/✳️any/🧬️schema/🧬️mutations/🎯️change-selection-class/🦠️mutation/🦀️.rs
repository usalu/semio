//! 🎯️ `change-selection-class` — sets the product-class scalar of the active selection request.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSelectionClass {
    pub new_class_id: String,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for ChangeSelectionClass where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_selection_class::mutation::ChangeSelectionClass> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "selection-class", kind: "change-selection-class", record: "ChangedSelectionClass" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change selection class to \"{}\"", self.new_class_id), &format!("Auswahlklasse auf \"{}\" ändern", self.new_class_id))
    }
}

impl<Op> protocol::ColdMutationKind<Iso16757Snapshot, Op> for ChangeSelectionClass where Op: protocol::ColdMutation<Iso16757Snapshot, ReferenceDiff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::change_selection_class::mutation::ChangeSelectionClass> {
    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<Iso16757Snapshot>>::ReferenceDiff> {
        crate::change_selection_class::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_selection_class::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
