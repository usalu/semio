//! 📏 `change-shear-area-mm2` payload — changes the En1996 document's `shear_area_mm2` (shear area [mm2]).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeShearAreaMm2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeShearAreaMm2 {
    pub new_shear_area_mm2: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeShearAreaMm2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shear-area-mm2", kind: "change-shear-area-mm2", record: "ChangedShearAreaMm2" };

    async fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
        crate::artifacts::en1996::mutations::change_shear_area_mm2::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_shear_area_mm2::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change shear area [mm2] to {}", self.new_shear_area_mm2)
    }
}
//#endregion 🔖️ChangeShearAreaMm2
