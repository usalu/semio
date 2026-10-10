//! 🦈️ `split-wall` payload. Splits a wall at the fraction `t` of its axis into the original wall and a new wall that continues it with the same type, location, base, top and phase; openings beyond the split point move onto the new wall.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SplitWall {
    pub id: String,
    pub t: f64,
    pub new_id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SplitWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "split", entity: "wall", kind: "split-wall", record: "SplitWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Split wall \"{}\" at {}", self.id, self.t), &format!("Wand \"{}\" bei {} teilen", self.id, self.t))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
