//! 🔁️ `change-edition-profile` — upserts one sheet's edition-profile override, addressed by sheet
//! number (the format's native key — `crate::artifacts::vdi3805::edition_profile` is name/code-keyed,
//! not id-keyed).

use crate::artifacts::vdi3805::{EditionProfileChoice, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeEditionProfile {
    pub sheet: String,
    pub new_choice: EditionProfileChoice,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for ChangeEditionProfile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edition-profile", kind: "change-edition-profile", record: "ChangedEditionProfile" };

    fn diff(&self, base: &Vdi3805Snapshot) -> <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change edition profile for sheet {} to {:?}", self.sheet, self.new_choice)
    }
    fn target(&self) -> Vec<String> {
        vec![self.sheet.clone()]
    }
}
//#endregion 🔖️Payload
