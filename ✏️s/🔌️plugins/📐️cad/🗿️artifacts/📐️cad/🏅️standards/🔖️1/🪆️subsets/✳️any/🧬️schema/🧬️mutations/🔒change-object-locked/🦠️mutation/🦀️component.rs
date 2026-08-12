//! 🔒️ CAD mutation — `ChangeObjectLocked` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔒️ Change lock state of one object's `locked` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-locked")]
pub struct ChangeObjectLocked {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_locked: bool,
}

impl MutationKind<CadSnapshot, CadMutation> for ChangeObjectLocked {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-locked", record: "ChangedObjectLocked" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change lock state of object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
