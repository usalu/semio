//! 🔪️ `trim-extend-wall` payload. Trims or extends one end of a wall to the axis of another wall: the intersection of the two axes (lines and circles carried on infinitely) nearest to the old end becomes the new end, as a concrete axis. Openings keep their place in the world: when the start moves their offsets move with it, and an opening that would no longer fit refuses the call.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct TrimExtendWall {
    pub id: String,
    pub end: super::super::modify::WallEnd,
    pub target: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for TrimExtendWall {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "wall", kind: "trim-extend-wall", record: "TrimExtendWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Trim or extend the {:?} of wall \"{}\" to wall \"{}\"", self.end, self.id, self.target), &format!("{:?} der Wand \"{}\" auf Wand \"{}\" kürzen oder verlängern", self.end, self.id, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
