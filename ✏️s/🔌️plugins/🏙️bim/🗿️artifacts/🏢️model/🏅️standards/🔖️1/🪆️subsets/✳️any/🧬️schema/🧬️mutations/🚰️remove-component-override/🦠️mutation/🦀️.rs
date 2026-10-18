//! 🚰️ `remove-component-override` payload. Removes the override of the family parameter `name` of one component; the component evaluates the formula of its family again.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveComponentOverride {
    pub component: String,
    pub name: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for RemoveComponentOverride
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RemoveComponentOverride> + From<crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "component-override", kind: "remove-component-override", record: "RemoveComponentOverride" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::remove_component_override::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::remove_component_override::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reset parameter \"{}\" of component \"{}\"", self.name, self.component), &format!("Parameter \"{}\" der Komponente \"{}\" zurücksetzen", self.name, self.component))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}.{}", self.component, self.name)]
    }
}
