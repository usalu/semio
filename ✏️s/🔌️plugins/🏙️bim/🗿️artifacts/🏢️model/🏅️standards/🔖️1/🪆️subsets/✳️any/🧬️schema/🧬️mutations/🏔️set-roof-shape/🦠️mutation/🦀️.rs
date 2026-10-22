//! 🏔️ `set-roof-shape` payload. Sets exactly the provided fields of a roof's form: shape, overhang and base offset; absent fields stay untouched.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::RoofShape;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRoofShape {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<RoofShape>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overhang: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetRoofShape
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetRoofShape>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "roof", kind: "set-roof-shape", record: "SetRoofShape" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change the form of roof \"{}\"", self.id), &format!("Form des Dachs \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetRoofShape
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetRoofShape>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_roof_shape::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_roof_shape::inverse::inverse(self, base))
    }
}
