//! 🗑️ `delete-position` mutation payload — removes a position feature from `positions` by id.
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🗑️ Removes the `positions` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-position")]
pub struct DeletePosition {
    pub id: String,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for DeletePosition {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "position", kind: "delete-position", record: "DeletedPosition" };

    fn diff(&self, base: &GisMapSnapshot) -> GisMapDiff {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete position \"{}\"", self.id)
    }
}
//#endregion 🔹Payload
