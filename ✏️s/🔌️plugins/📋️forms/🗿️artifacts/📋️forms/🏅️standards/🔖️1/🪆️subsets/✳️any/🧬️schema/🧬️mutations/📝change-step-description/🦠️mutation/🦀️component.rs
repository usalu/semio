//! ✏️ Forms mutation payloads — the `steps` collection's `rename` and `change` verbs. Physical dir
//! name (`🩹update-step`, wired by `📦️glue.rs`) predates the split of the old generic `UpdateStep`
//! (whole-struct patch) into two granular semantic mutations — title is meaningfully set alone
//! (`rename`), and so is description (`change`), so neither is an inseparable `update` facet.

use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📝️ Sets a step's `description` scalar (a `None` clears it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeStepDescription {
    pub id: String,
    pub new_description: Option<String>,
}

impl MutationKind<FormsSnapshot, FormMutation> for ChangeStepDescription {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "step-description", kind: "change-step-description", record: "ChangedStepDescription" };
    fn diff(&self, base: &FormsSnapshot) -> FormsDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change step \"{}\" description", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
