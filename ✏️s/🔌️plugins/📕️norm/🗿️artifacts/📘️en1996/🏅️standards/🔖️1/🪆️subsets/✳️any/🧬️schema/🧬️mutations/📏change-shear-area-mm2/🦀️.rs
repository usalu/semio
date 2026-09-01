//! 📏 `change-shear-area-mm2` payload — changes the En1996 document's `shear_area_mm2` (shear area [mm2]).


use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::mutations::change_shear_area_mm2::ChangeShearAreaMm2;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeShearAreaMm2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeShearAreaMm2 {
    pub new_shear_area_mm2: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeShearAreaMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shear-area-mm2", kind: "change-shear-area-mm2", record: "ChangedShearAreaMm2" };

    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shear area [mm2] to {}", self.new_shear_area_mm2)
    }
}
//#endregion 🔖️ChangeShearAreaMm2
