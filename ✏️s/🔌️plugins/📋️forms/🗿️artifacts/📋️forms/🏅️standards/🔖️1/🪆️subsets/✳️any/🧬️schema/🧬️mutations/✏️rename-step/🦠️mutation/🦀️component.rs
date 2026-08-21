//! ✏️ Forms mutation payloads — the `steps` collection's `rename` and `change` verbs. Physical dir
//! name (`🩹update-step`, wired by `📦️glue.rs`) predates the split of the old generic `UpdateStep`
//! (whole-struct patch) into two granular semantic mutations — title is meaningfully set alone
//! (`rename`), and so is description (`change`), so neither is an inseparable `update` facet.

use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🏷️RenameStep
/// 🏷️ Changes a step's identity `title` field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameStep {
    pub id: String,
    pub new_title: String,
}

impl MutationKind<FormsSnapshot, FormMutation> for RenameStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "step", kind: "rename-step", record: "RenamedStep" };
    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename step to \"{}\"", self.new_title)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏷️RenameStep

//#region 📝️ChangeStepDescription

//#endregion 📝️ChangeStepDescription
