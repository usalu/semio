//! 🗑️ `delete-route` mutation payload — removes a route feature from `routes` by id.

use crate::artifacts::gismap::GisMapSnapshot;
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 🗑️ Removes the `routes` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-route")]
pub struct DeleteRoute {
    pub id: String,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for DeleteRoute {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "route", kind: "delete-route", record: "DeletedRoute" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete route \"{}\"", self.id)
    }
}
//#endregion 🔹Payload
