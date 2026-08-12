//! 🫳️ CAD mutation — `DragObjects` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🫳️ Relative multi-select spatial offset (a real gumball-drag gesture) applied to every object in
/// `object_ids`, across whichever pane each one lives in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "drag-objects")]
pub struct DragObjects {
    pub object_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl MutationKind<CadSnapshot, CadMutation> for DragObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "objects", kind: "drag-objects", record: "DraggedObjects" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Drag {} object(s)", self.object_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.object_ids.clone()
    }
}
//#endregion 🔖️Mutation
