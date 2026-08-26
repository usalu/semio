//! ☃️ `change-en-sk-kn-m2` — sets the En1991 characteristic snow load scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeEnSKKnM2 {
    pub new_en_s_k_kn_m2: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeEnSKKnM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "en-sk-kn-m2", kind: "change-en-sk-kn-m2", record: "ChangedEnSkKnM2" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change characteristic snow load to {:?}", self.new_en_s_k_kn_m2)
    }
}
//#endregion 🔖️Payload
