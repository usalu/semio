//! 🔝️ `set-wall-top` payload. Sets how a wall's top is resolved: free height, the own storey's top, or another storey of the same building.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::TopConstraint;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallTop {
    pub id: String,
    pub top: TopConstraint,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetWallTop
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWallTop>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-top", record: "SetWallTop" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Constrain the top of wall \"{}\"", self.id), &format!("Oberkante von Wand \"{}\" festlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetWallTop
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetWallTop>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_wall_top::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_wall_top::inverse::inverse(self, base))
    }
}
