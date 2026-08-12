//! 🔧 `change-years-since-inspection` payload — changes the Din16798 document's `years_since_inspection` (years since last inspection).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeYearsSinceInspection
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeYearsSinceInspection {
    pub new_years_since_inspection: u32,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeYearsSinceInspection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "years-since-inspection", kind: "change-years-since-inspection", record: "ChangedYearsSinceInspection" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_years_since_inspection::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_years_since_inspection::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change years since last inspection to {}", self.new_years_since_inspection)
    }
}
//#endregion 🔖️ChangeYearsSinceInspection
