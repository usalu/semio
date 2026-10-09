//! 🧷️ `set-wall-end-join` payload. Sets the authored join preference of one end of a wall: mitered, butted or not joined; an absent join returns the end to the geometry (automatic). Only the wall layout reads it, nothing is derived here.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallEndJoin {
    pub id: String,
    pub end: super::super::modify::WallEnd,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub join: Option<crate::EndJoin>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetWallEndJoin {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-end-join", record: "SetWallEndJoin" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the {:?} join of wall \"{}\" to {}", self.end, self.id, self.join.map_or("automatic".to_string(), |join| format!("{join:?}"))), &format!("Verbindung am Wandende {:?} von \"{}\" auf {} setzen", self.end, self.id, self.join.map_or("automatisch".to_string(), |join| format!("{join:?}"))))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
