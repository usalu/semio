//! ✂️ `remove-selection-constraint` — removes one property constraint by BASE-state index.

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RemoveSelectionConstraint {
    pub index: usize,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for RemoveSelectionConstraint where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::add_selection_constraint::mutation::AddSelectionConstraint> { const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "selection-constraint", kind: "remove-selection-constraint", record: "RemovedSelectionConstraint" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::remove_selection_constraint::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::remove_selection_constraint::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove selection constraint #{}", self.index), &format!("Auswahlbedingung #{} entfernen", self.index))
    }
}
//#endregion 🔖️Payload
