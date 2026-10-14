//! 🆕️ `introduce-property-definition` — brings a new id-keyed catalogue property definition into
//! existence.

use semio_s_artifact_norm_iso16757_model::part_1::PropertyDefinition;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct IntroducePropertyDefinition {
    pub property_definition: PropertyDefinition,
    pub index: Option<usize>,
}

impl<Op> protocol::MutationKind<Iso16757Snapshot, Op> for IntroducePropertyDefinition where Op: protocol::Mutation<Iso16757Snapshot, Diff = semio_s_artifact_norm_iso16757_diff::Iso16757Diff> + From<crate::retire_property_definition::mutation::RetirePropertyDefinition> {
    const SEMANTICS: protocol::SemanticDescriptor =

        protocol::SemanticDescriptor { verb: "insert", entity: "propertyDefinition", kind: "introduce-property-definition", record: "IntroducedPropertyDefinition" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        crate::introduce_property_definition::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::introduce_property_definition::inverse::inverse(self, base)?
    
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
