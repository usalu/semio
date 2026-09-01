//! 🔧 `change-l-aeq-db` payload — changes the Din16798 document's `l_aeq_db` (equivalent sound pressure level).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_l_aeq_db::ChangeLAeqDb;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLAeqDb
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLAeqDb {
    pub new_l_aeq_db: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeLAeqDb {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "l-aeq-db", kind: "change-l-aeq-db", record: "ChangedLAeqDb" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change equivalent sound pressure level to {}", self.new_l_aeq_db)
    }
}
//#endregion 🔖️ChangeLAeqDb
