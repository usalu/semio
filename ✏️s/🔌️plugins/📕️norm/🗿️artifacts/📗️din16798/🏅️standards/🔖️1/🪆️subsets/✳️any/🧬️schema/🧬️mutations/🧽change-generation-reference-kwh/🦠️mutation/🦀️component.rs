//! 🔧 `change-generation-reference-kwh` payload — changes the Din16798 document's `generation_reference_kwh` (generation energy reference).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeGenerationReferenceKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeGenerationReferenceKwh {
    pub new_generation_reference_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeGenerationReferenceKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "generation-reference-kwh", kind: "change-generation-reference-kwh", record: "ChangedGenerationReferenceKwh" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_generation_reference_kwh::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_generation_reference_kwh::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change generation energy reference to {}", self.new_generation_reference_kwh)
    }
}
//#endregion 🔖️ChangeGenerationReferenceKwh
