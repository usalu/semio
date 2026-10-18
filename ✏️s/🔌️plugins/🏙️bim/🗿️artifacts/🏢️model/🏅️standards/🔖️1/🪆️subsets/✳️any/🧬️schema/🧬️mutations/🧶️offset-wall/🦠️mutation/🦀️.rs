//! 🧶️ `offset-wall` payload. Creates a wall parallel to a wall at a signed distance to the left of its axis direction (to the right when negative): a shifted line, or a concentric arc of the same sweep, with the type, location line, constraints, phase and name of the original; hosted openings stay on the original.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct OffsetWall {
    pub id: String,
    pub new_id: String,
    pub distance: f64,
}

impl<Op> MutationKind<ModelSnapshot, Op> for OffsetWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<OffsetWall> + From<crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall", kind: "offset-wall", record: "OffsetWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::offset_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::offset_wall::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Offset wall \"{}\" by {} m", self.id, self.distance), &format!("Wand \"{}\" um {} m versetzen", self.id, self.distance))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
