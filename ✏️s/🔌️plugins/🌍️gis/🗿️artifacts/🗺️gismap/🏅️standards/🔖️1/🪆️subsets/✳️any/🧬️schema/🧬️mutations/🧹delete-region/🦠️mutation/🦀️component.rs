//! 🗑️ `delete-region` mutation payload — removes a region feature from `regions` by id.
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🗑️ Removes the `regions` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-region")]
pub struct DeleteRegion {
    pub id: String,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for DeleteRegion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "region", kind: "delete-region", record: "DeletedRegion" };

    async fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Delete region \"{}\"", self.id)
    }
}
//#endregion 🔹Payload
