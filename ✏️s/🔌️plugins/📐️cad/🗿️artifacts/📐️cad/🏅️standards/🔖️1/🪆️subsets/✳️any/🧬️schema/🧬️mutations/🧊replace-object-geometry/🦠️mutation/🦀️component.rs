//! 🧊️ CAD mutation — `ReplaceObjectGeometry` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧊️ Whole-value swap of an object's geometry-identity trio (`extent`/`mesh_url`/`solid_handle`) —
/// the three fields the brep kernel/mesh importer always set together, never independently.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-object-geometry")]
pub struct ReplaceObjectGeometry {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_extent: Option<[f64; 3]>,
    pub new_mesh_url: Option<String>,
    pub new_solid_handle: Option<String>,
}

impl MutationKind<CadSnapshot, CadMutation> for ReplaceObjectGeometry {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "object", kind: "replace-object-geometry", record: "ReplacedObjectGeometry" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace geometry of object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
