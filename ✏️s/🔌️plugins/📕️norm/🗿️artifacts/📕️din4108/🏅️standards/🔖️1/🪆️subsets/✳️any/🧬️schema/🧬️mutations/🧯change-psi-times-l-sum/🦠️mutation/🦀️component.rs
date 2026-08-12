//! 🌉 `change-psi-times-l-sum` — sets the DIN 4108 `psi_times_l_sum` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePsiTimesLSum {
    pub new_psi_times_l_sum: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangePsiTimesLSum {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "psi-times-l-sum", kind: "change-psi-times-l-sum", record: "ChangedPsiTimesLSum" };

    fn diff(&self, base: &Din4108Snapshot) -> <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change psi times l sum to {}", self.new_psi_times_l_sum)
    }
}
//#endregion 🔖️Payload
