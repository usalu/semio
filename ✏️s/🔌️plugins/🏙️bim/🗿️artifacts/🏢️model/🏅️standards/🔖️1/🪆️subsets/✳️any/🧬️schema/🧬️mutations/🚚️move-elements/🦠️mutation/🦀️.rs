//! 🚚️ `move-elements` payload. Translates placed elements by a vector: walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces and grid lines each get one sparse patch of exactly their placement fields; hosted openings follow their host by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveElements {
    pub ids: Vec<String>,
    pub vector: Point2,
}

impl MutationKind<ModelSnapshot, ModelMutation> for MoveElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "elements", kind: "move-elements", record: "MoveElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {} element(s) by ({}, {}) m", self.ids.len(), self.vector.x, self.vector.y), &format!("{} Element(e) um ({}, {}) m verschieben", self.ids.len(), self.vector.x, self.vector.y))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
