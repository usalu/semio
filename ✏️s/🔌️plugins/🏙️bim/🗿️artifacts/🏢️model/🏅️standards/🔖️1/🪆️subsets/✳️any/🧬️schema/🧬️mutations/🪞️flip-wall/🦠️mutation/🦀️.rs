//! 🪞️ `flip-wall` payload. Reverses the orientation of a wall: its axis swaps start and end and the bulge changes sign, so the same curve runs the other way.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct FlipWall {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for FlipWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<FlipWall>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "toggle", entity: "wall", kind: "flip-wall", record: "FlipWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::flip_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::flip_wall::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Flip wall \"{}\"", self.id), &format!("Wand \"{}\" wenden", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
