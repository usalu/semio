//! ➕️ `add-selection-constraint` — inserts one property constraint into the active selection at `index` (absent appends).

use crate::{part_1::SelectionConstraint, Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AddSelectionConstraint {
    pub constraint: SelectionConstraint,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for AddSelectionConstraint { const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "selection-constraint", kind: "add-selection-constraint", record: "AddedSelectionConstraint" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add selection constraint on \"{}\"", self.constraint.property_id), &format!("Auswahlbedingung für \"{}\" hinzufügen", self.constraint.property_id))
    }
}
//#endregion 🔖️Payload
