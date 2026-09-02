//! 🔧️ `update-script-limits` — atomically updates the part-number script execution limits facet
//! (`max_steps`/`max_recursion`/`timeout_ms` are validated together, never one-field-at-a-time).

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateScriptLimits {
    pub new_max_steps: u32,
    pub new_max_recursion: u32,
    pub new_timeout_ms: u64,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for UpdateScriptLimits {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "script-limits", kind: "update-script-limits", record: "UpdatedScriptLimits" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Update script limits (max-steps={}, max-recursion={}, timeout-ms={})", self.new_max_steps, self.new_max_recursion, self.new_timeout_ms)
    }
}
//#endregion 🔖️Payload
