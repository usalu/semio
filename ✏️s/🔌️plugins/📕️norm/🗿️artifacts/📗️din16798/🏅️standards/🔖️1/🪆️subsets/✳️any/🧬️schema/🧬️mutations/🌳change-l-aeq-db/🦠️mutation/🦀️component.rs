//! 🔧 `change-l-aeq-db` payload — changes the Din16798 document's `l_aeq_db` (equivalent sound pressure level).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLAeqDb
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLAeqDb {
    pub new_l_aeq_db: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeLAeqDb {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "l-aeq-db", kind: "change-l-aeq-db", record: "ChangedLAeqDb" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_l_aeq_db::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_l_aeq_db::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change equivalent sound pressure level to {}", self.new_l_aeq_db)
    }
}
//#endregion 🔖️ChangeLAeqDb
