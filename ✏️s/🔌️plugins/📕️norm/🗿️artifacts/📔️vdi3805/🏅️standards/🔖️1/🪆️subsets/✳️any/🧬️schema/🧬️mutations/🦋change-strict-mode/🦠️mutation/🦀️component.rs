//! 🔐️ `change-strict-mode` — toggles the document root's strict-mode flag.

use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeStrictMode {
    pub new_strict_mode: bool,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeStrictMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "strict-mode", kind: "change-strict-mode", record: "ChangedStrictMode" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change strict mode to {}", self.new_strict_mode)
    }
}
//#endregion 🔖️Payload
