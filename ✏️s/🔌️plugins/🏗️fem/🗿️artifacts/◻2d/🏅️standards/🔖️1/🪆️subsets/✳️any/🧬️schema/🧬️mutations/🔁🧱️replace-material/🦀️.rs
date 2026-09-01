//! 🔁️ Fem2d mutation — `ReplaceMaterial` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemMaterial};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dMaterialsDelta, Fem2dMaterialsPatchEntry};
use crate::artifacts::fem2d::mutations::Fem2dMutation;
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

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ReplaceMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "material", kind: "replace-material", record: "ReplacedMaterial" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
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
