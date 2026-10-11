//! 🧶️ `offset-wall` payload. Creates a wall parallel to a wall at a signed distance to the left of its axis direction (to the right when negative): a shifted line, or a concentric arc of the same sweep, with the type, location line, constraints, phase and name of the original; hosted openings stay on the original.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct OffsetWall {
    pub id: String,
    pub new_id: String,
    pub distance: f64,
}

impl MutationKind<ModelSnapshot, ModelMutation> for OffsetWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall", kind: "offset-wall", record: "OffsetWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Offset wall \"{}\" by {} m", self.id, self.distance), &format!("Wand \"{}\" um {} m versetzen", self.id, self.distance))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
