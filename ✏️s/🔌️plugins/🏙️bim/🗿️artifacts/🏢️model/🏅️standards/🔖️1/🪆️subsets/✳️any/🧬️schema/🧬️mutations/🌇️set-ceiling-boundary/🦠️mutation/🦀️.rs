//! 🌇️ `set-ceiling-boundary` payload. Replaces the outline of a ceiling as one semantic unit: its boundary loop and its hole loops, validated together.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Vertex;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCeilingBoundary {
    pub id: String,
    pub boundary: Vec<Vertex>,
    pub holes: Vec<Vec<Vertex>>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCeilingBoundary
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCeilingBoundary>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "ceiling", kind: "set-ceiling-boundary", record: "SetCeilingBoundary" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reshape the outline of ceiling \"{}\"", self.id), &format!("Umriss der Unterdecke \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
