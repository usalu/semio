//! 🍰️ `split-slab` payload. Cuts a slab along the infinite line through two points into two slabs: the piece to the left of the line (seen from its first point to its second) stays, the piece to the right is created with the type, offset, slope and name of the original; holes go with the piece they lie in, split arcs keep their circle. The line must cross the outline exactly twice and no hole.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SplitSlab {
    pub id: String,
    pub new_id: String,
    pub line_start: Point2,
    pub line_end: Point2,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SplitSlab {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "split", entity: "slab", kind: "split-slab", record: "SplitSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Split slab \"{}\" along the line through ({}, {}) and ({}, {})", self.id, self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y), &format!("Decke \"{}\" entlang der Linie durch ({}, {}) und ({}, {}) teilen", self.id, self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
