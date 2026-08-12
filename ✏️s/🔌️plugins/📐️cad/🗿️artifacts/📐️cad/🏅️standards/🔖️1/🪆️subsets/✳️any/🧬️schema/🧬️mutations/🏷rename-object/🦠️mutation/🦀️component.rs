//! 🏷️ CAD mutation — `RenameObject` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ Rename one object's `label` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-object")]
pub struct RenameObject {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_label: String,
}

impl MutationKind<CadSnapshot, CadMutation> for RenameObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "object", kind: "rename-object", record: "RenamedObject" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
