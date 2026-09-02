//! 🗑️ `delete-position` mutation payload — removes a position feature from `positions` by id.

use crate::artifacts::gismap::GisMapSnapshot;
use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::mutations::GisMapMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Payload
/// 🗑️ Removes the `positions` entry addressed by `id` (BASE-state, per the taxonomy's index/id
/// addressing law). Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-position")]
pub struct DeletePosition {
    pub id: String,
}

impl MutationKind<GisMapSnapshot, GisMapMutation> for DeletePosition {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "position", kind: "delete-position", record: "DeletedPosition" };

    fn diff(&self, base: &GisMapSnapshot) -> protocol::MutationOutcome<GisMapDiff> {
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
