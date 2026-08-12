//! 📏️ CAD mutation — `ScaleObject` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadPaneId, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📏️ Scale one object's `scale` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-object")]
pub struct ScaleObject {
    pub pane: CadPaneId,
    pub object_id: String,
    pub new_scale: [f64; 3],
}

impl MutationKind<CadSnapshot, CadMutation> for ScaleObject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "object", kind: "scale-object", record: "ScaledObject" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale object \"{}\"", self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Mutation
