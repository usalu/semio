//! 🔄️ CAD mutation — `RotateObjects` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔄️ Relative multi-select axis-angle rotation, composed onto each object's own current
/// orientation quaternion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rotate-objects")]
pub struct RotateObjects {
    pub object_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}

impl MutationKind<CadSnapshot, CadMutation> for RotateObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "objects", kind: "rotate-objects", record: "RotatedObjects" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rotate {} object(s)", self.object_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.object_ids.clone()
    }
}
//#endregion 🔖️Mutation
