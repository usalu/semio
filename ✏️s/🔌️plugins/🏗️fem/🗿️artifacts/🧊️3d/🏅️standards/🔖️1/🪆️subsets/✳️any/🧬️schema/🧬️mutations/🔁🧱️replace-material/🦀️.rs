//! 🔁️ Fem3d mutation — `ReplaceMaterial` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemMaterial};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta, Fem3dMaterialsPatchEntry};
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing material's payload (structured, no field-by-field editor
/// gesture exists for this entity — every real caller sets the whole record at once).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-material")]
pub struct ReplaceMaterial {
    pub id: String,
    pub new_material: FemMaterial,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "material", kind: "replace-material", record: "ReplacedMaterial" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace material \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
