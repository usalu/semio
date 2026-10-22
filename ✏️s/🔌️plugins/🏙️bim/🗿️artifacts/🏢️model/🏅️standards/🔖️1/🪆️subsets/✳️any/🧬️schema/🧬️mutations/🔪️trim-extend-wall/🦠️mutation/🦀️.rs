//! 🔪️ `trim-extend-wall` payload. Trims or extends one end of a wall to the axis of another wall: the intersection of the two axes (lines and circles carried on infinitely) nearest to the old end becomes the new end, as a concrete axis. Openings keep their place in the world: when the start moves their offsets move with it, and an opening that would no longer fit refuses the call.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct TrimExtendWall {
    pub id: String,
    pub end: semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::WallEnd,
    pub target: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for TrimExtendWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<TrimExtendWall> + From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "wall", kind: "trim-extend-wall", record: "TrimExtendWall" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Trim or extend the {:?} of wall \"{}\" to wall \"{}\"", self.end, self.id, self.target), &format!("{:?} der Wand \"{}\" auf Wand \"{}\" kürzen oder verlängern", self.end, self.id, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for TrimExtendWall
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<TrimExtendWall> + From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::trim_extend_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::trim_extend_wall::inverse::inverse(self, base))
    }
}
