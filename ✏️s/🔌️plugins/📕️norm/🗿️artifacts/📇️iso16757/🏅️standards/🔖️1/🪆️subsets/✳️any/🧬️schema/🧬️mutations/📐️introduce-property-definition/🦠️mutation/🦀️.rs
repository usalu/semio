//! 🆕️ `introduce-property-definition` — brings a new id-keyed catalogue property definition into
//! existence.

use crate::{part_1::PropertyDefinition, Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct IntroducePropertyDefinition {
    pub property_definition: PropertyDefinition,
    pub index: Option<usize>,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for IntroducePropertyDefinition {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "insert", entity: "propertyDefinition", kind: "introduce-property-definition", record: "IntroducedPropertyDefinition" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Iso16757Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create property definition \"{}\"", self.property_definition.names.preferred.text), &format!("Eigenschaftsdefinition \"{}\" erstellen", self.property_definition.names.preferred.text))
    }
    fn target(&self) -> Vec<String> {
        vec![self.property_definition.id.clone()]
    }
}
//#endregion 🔖️Payload
