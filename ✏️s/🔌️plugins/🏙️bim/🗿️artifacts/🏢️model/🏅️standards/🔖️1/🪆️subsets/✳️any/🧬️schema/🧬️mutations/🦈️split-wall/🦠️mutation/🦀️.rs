//! 🦈️ `split-wall` payload. Splits a wall at the fraction `t` of its axis into the original wall and a new wall that continues it with the same type, location, base, top and phase; openings beyond the split point move onto the new wall.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SplitWall {
    pub id: String,
    pub t: f64,
    pub new_id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SplitWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SplitWall> + From<semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall> + From<semio_s_artifact_bim_model_mut_any_79::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening> + From<semio_s_artifact_bim_model_mut_any_161::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "split", entity: "wall", kind: "split-wall", record: "SplitWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::split_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::split_wall::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Split wall \"{}\" at {}", self.id, self.t), &format!("Wand \"{}\" bei {} teilen", self.id, self.t))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
