//! ✅ `change-bb2-details-conform` — sets the DIN 4108 `bb2_details_conform` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeBb2DetailsConform {
    pub new_bb2_details_conform: bool,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeBb2DetailsConform {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bb2-details-conform", kind: "change-bb2-details-conform", record: "ChangedBb2DetailsConform" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change bb2 details conform to {}", self.new_bb2_details_conform)
    }
}
//#endregion 🔖️Payload
