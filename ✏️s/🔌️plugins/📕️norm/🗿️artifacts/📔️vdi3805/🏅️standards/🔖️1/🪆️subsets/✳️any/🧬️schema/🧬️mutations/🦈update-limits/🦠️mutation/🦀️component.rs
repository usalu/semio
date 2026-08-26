//! 🛡️ `update-limits` — atomically updates the untrusted-input security limits facet
//! (`max_file_bytes`/`max_records`/`max_field_length`/`max_nesting_depth` are one security policy,
//! never set one-field-at-a-time).

use crate::artifacts::vdi3805::{SecurityLimits, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLimits {
    pub new_limits: SecurityLimits,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for UpdateLimits {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "limits", kind: "update-limits", record: "UpdatedLimits" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update security limits (max-file-bytes={})", self.new_limits.max_file_bytes)
    }
}
//#endregion 🔖️Payload
