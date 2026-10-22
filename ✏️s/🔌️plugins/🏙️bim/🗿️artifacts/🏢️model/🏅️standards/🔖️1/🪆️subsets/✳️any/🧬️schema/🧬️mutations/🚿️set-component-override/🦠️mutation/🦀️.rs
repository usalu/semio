//! 🚿️ `set-component-override` payload. Overrides the formula of the family parameter `name` for one component, or replaces its override. The formula must parse, use only parameters of the family of the component and never close a circle under the overrides of the component.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetComponentOverride {
    pub component: String,
    pub name: String,
    pub value: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetComponentOverride
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetComponentOverride> + From<crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "component-override", kind: "set-component-override", record: "SetComponentOverride" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Override parameter \"{}\" of component \"{}\"", self.name, self.component), &format!("Parameter \"{}\" der Komponente \"{}\" überschreiben", self.name, self.component))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}.{}", self.component, self.name)]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetComponentOverride
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetComponentOverride> + From<crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_component_override::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_component_override::inverse::inverse(self, base))
    }
}
