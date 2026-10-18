//! 🧾️ `set-element-property` payload. Sets one typed property of an element's property set; an element without properties gets its first entry, an existing value is replaced.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, PropertyValue}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementProperty {
    pub id: String,
    pub pset: String,
    pub property: String,
    pub value: PropertyValue,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetElementProperty
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetElementProperty> + From<crate::standards::v1::subsets::any::schema::mutations::remove_element_property::RemoveElementProperty>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-property", kind: "set-element-property", record: "SetElementProperty" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_element_property::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_element_property::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set property {}.{} of \"{}\"", self.pset, self.property, self.id), &format!("Eigenschaft {}.{} von \"{}\" setzen", self.pset, self.property, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
