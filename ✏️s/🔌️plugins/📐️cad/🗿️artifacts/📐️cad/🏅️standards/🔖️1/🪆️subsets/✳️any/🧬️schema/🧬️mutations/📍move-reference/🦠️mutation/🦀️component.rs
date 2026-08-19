//! 📍️ CAD mutation — `MoveReference` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📍️ Move one reference overlay's `origin` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-reference")]
pub struct MoveReference {
    pub model_definition_id: String,
    pub reference_id: String,
    pub new_origin: [f64; 3],
}

impl MutationKind<CadSnapshot, CadMutation> for MoveReference {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "reference", kind: "move-reference", record: "MovedReference" };

    async fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move reference \"{}\"", self.reference_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone(), self.reference_id.clone()]
    }
}
//#endregion 🔖️Mutation
