//! 🪝️ `set-beam-axis` payload. Replaces the axis of a beam, a line or an arc by bulge, exactly like the axis of a wall; this is how a beam is moved, stretched, curved or straightened. The inferred sweep, joins and quantities follow.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Axis;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBeamAxis {
    pub id: String,
    pub axis: Axis,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetBeamAxis
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetBeamAxis>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "beam", kind: "set-beam-axis", record: "SetBeamAxis" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reshape the axis of beam \"{}\"", self.id), &format!("Achse von Träger \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetBeamAxis
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetBeamAxis>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_beam_axis::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_beam_axis::inverse::inverse(self, base))
    }
}
