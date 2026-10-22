//! 🔡️ `set-family-parameter` payload. Sets exactly the provided kind and formula of the parameter `name` of a family, or adds it (a new parameter needs both). Formulas are canonical text of the expression language, use only existing parameters and never close a circle.

use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::FamilyParameterPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::ParameterKind;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFamilyParameter {
    pub family: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ParameterKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SetFamilyParameter {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> FamilyParameterPatch {
        FamilyParameterPatch { kind: self.kind, value: self.value.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(family: String, name: String, patch: FamilyParameterPatch) -> Self {
        Self { family, name, kind: patch.kind, value: patch.value }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetFamilyParameter
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetFamilyParameter> + From<crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "family-parameter", kind: "set-family-parameter", record: "SetFamilyParameter" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set parameter \"{}\" of family \"{}\"", self.name, self.family), &format!("Parameter \"{}\" der Familie \"{}\" setzen", self.name, self.family))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{}.{}", self.family, self.name)]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetFamilyParameter
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetFamilyParameter> + From<crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::inverse::inverse(self, base))
    }
}
