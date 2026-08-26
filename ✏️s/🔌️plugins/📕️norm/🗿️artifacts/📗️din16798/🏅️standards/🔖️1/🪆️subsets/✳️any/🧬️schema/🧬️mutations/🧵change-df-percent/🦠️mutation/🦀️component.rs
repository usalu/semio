//! 🔧 `change-df-percent` payload — changes the Din16798 document's `df_percent` (daylight factor).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDfPercent
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDfPercent {
    pub new_df_percent: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDfPercent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "df-percent", kind: "change-df-percent", record: "ChangedDfPercent" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_df_percent::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_df_percent::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change daylight factor to {}", self.new_df_percent)
    }
}
//#endregion 🔖️ChangeDfPercent
