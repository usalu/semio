//! 🔠️ `remove-family-parameter` payload. Removes the parameter `name` of a family; refused while the formula of another parameter or a solid of the family refers to it.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveFamilyParameter {
    pub family: String,
    pub name: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for RemoveFamilyParameter
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RemoveFamilyParameter> + From<crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "family-parameter", kind: "remove-family-parameter", record: "RemoveFamilyParameter" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove parameter \"{}\" of family \"{}\"", self.name, self.family), &format!("Parameter \"{}\" der Familie \"{}\" entfernen", self.name, self.family))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}.{}", self.family, self.name)]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for RemoveFamilyParameter
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<RemoveFamilyParameter> + From<crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::inverse::inverse(self, base))
    }
}
