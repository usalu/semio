//! 🔀️ `reorder-routes` mutation payload — repositions a route feature within `routes`
//! by id (id-keyed collection, so addressing is `id`+`to_index`, not a bare index pair).
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🔀️ Moves the `routes` entry addressed by `id` to `to_index`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "reorder-routes")]
pub struct ReorderRoutes {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for ReorderRoutes {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "routes", kind: "reorder-routes", record: "ReorderedRoutes" };

    async fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Reorder route \"{}\" to {}", self.id, self.to_index)
    }
}
//#endregion 🔹Payload
