//! 🌄️ `set-ceiling` payload. Sets exactly the provided fields of a ceiling: type, drop below the storey top, slope (set or cleared) and name; absent fields stay untouched.

use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::ceilings_roofs_openings::CeilingPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Slope;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCeiling {
    pub id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub ceiling_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slope: Option<Assigned<Option<Slope>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetCeiling {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> CeilingPatch {
        CeilingPatch { ceiling_type: self.ceiling_type.clone(), offset: self.offset, slope: self.slope.clone(), name: self.name.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: CeilingPatch) -> Self {
        Self { id, ceiling_type: patch.ceiling_type, offset: patch.offset, slope: patch.slope, name: patch.name }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCeiling
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCeiling>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "ceiling", kind: "set-ceiling", record: "SetCeiling" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_ceiling::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_ceiling::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ceiling \"{}\"", self.id), &format!("Unterdecke \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
