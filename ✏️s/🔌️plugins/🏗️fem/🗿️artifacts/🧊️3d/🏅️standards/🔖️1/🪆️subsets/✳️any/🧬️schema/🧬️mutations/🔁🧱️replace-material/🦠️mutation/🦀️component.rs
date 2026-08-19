//! 🔁️ Fem3d mutation — `ReplaceMaterial` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemMaterial};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing material's payload (structured, no field-by-field editor
/// gesture exists for this entity — every real caller sets the whole record at once).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-material")]
pub struct ReplaceMaterial {
    pub id: String,
    pub new_material: FemMaterial,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "material", kind: "replace-material", record: "ReplacedMaterial" };

    async fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace material \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
