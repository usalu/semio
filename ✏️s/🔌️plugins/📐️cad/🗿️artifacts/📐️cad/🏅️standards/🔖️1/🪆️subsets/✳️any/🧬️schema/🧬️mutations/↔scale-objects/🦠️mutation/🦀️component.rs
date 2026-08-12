//! ↔️ CAD mutation — `ScaleObjects` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ↔️ Relative multi-select per-axis scale factor, composed onto each object's own current scale.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-objects")]
pub struct ScaleObjects {
    pub object_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

impl MutationKind<CadSnapshot, CadMutation> for ScaleObjects {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "objects", kind: "scale-objects", record: "ScaledObjects" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale {} object(s)", self.object_ids.len())
    }
    fn target(&self) -> Vec<String> {
        self.object_ids.clone()
    }
}
//#endregion 🔖️Mutation
