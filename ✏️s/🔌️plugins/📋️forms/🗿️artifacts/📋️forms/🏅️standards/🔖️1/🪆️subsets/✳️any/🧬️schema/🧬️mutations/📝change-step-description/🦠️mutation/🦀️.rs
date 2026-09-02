//! ✏️ Forms mutation payloads — the `steps` collection's `rename` and `change` verbs. Physical dir
//! name (`🩹update-step`, wired by `🦀️.rs`) predates the split of the old generic `UpdateStep`
//! (whole-struct patch) into two granular semantic mutations — title is meaningfully set alone
//! (`rename`), and so is description (`change`), so neither is an inseparable `update` facet.

use serde::{Deserialize, Serialize};
use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔖️Mutation
/// 📝️ Sets a step's `description` scalar (a `None` clears it).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
pub struct ChangeStepDescription {
    pub id: String,
    pub new_description: Option<String>,
}

impl MutationKind<FormsSnapshot, FormMutation> for ChangeStepDescription {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "step-description", kind: "change-step-description", record: "ChangedStepDescription" };
    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change step \"{}\" description", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
