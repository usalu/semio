//! 🌡️ `change-t-int-c` — sets the DIN 4108 `t_int_c` scalar.


use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeTIntC {
    pub new_t_int_c: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeTIntC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "t-int-c", kind: "change-t-int-c", record: "ChangedTIntC" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change t int c to {}", self.new_t_int_c)
    }
}
//#endregion 🔖️Payload
