//! 🖌️ `set-material` payload. Patches exactly the provided fields of a material; layouts, solids and quantities that use it follow by inference.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{MaterialCategory, Rgb};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::MaterialPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMaterial {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<MaterialCategory>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub color: Option<Rgb>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub conductivity: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub specific_heat: Option<f64>,
}

impl SetMaterial {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> MaterialPatch {
        MaterialPatch { name: self.name.clone(), category: self.category.clone(), color: self.color.clone(), density: self.density.clone(), conductivity: self.conductivity.clone(), specific_heat: self.specific_heat.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: MaterialPatch) -> Self {
        Self { id, name: patch.name, category: patch.category, color: patch.color, density: patch.density, conductivity: patch.conductivity, specific_heat: patch.specific_heat }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetMaterial
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetMaterial>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "material", kind: "set-material", record: "SetMaterial" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit material \"{}\"", self.id), &format!("Material \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetMaterial
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetMaterial>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_material::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_material::inverse::inverse(self, base))
    }
}
