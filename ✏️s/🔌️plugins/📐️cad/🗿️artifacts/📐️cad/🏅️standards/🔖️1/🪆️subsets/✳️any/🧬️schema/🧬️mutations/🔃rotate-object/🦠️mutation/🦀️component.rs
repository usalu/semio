//! 🔃️ CAD mutation — `RotateObject` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔃️ Rotate one object's `orientation` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rotate-object")]
pub struct RotateObject {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_orientation: [f64; 4],
}

impl MutationKind<CadSnapshot, CadMutation> for RotateObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "object", kind: "rotate-object", record: "RotatedObject" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rotate object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
