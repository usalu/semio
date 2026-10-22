//! 🪛️ `create-wall-sweep` payload. Brings a new sweep onto one face of a wall: a profile run along the face (a baseboard, a cornice, a drip rail) at a height above the wall base, optionally set into the wall; its solid, length and areas are inferred.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, WallSweep};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWallSweep {
    pub id: String,
    pub wall_sweep: WallSweep,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateWallSweep
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateWallSweep> + From<semio_s_artifact_bim_model_mut_any_111::standards::v1::subsets::any::schema::mutations::delete_wall_sweep::DeleteWallSweep>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall-sweep", kind: "create-wall-sweep", record: "CreateWallSweep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wall sweep \"{}\"", self.wall_sweep.name), &format!("Wandprofil \"{}\" anlegen", self.wall_sweep.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateWallSweep
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateWallSweep> + From<semio_s_artifact_bim_model_mut_any_111::standards::v1::subsets::any::schema::mutations::delete_wall_sweep::DeleteWallSweep>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_wall_sweep::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_wall_sweep::inverse::inverse(self, base))
    }
}
