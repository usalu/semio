//! 🫧️ `remove-element-property` payload. Removes one property of an element; removing its last property deletes the element's property entry.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveElementProperty {
    pub id: String,
    pub pset: String,
    pub property: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for RemoveElementProperty
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RemoveElementProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_element_property::SetElementProperty>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "element-property", kind: "remove-element-property", record: "RemoveElementProperty" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::remove_element_property::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::remove_element_property::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove property {}.{} of \"{}\"", self.pset, self.property, self.id), &format!("Eigenschaft {}.{} von \"{}\" entfernen", self.pset, self.property, self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
