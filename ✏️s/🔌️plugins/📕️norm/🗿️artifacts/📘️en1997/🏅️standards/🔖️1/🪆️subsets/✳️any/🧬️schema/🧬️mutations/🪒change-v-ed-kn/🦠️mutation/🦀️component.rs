//! 🪒 `change-v-ed-kn` payload — changes the En1997 document's `v_ed_kn` (design vertical load V_Ed [kN]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeVEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeVEdKn {
    pub new_v_ed_kn: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeVEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-ed-kn", kind: "change-v-ed-kn", record: "ChangedVEdKn" };

    async fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_v_ed_kn::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_v_ed_kn::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change design vertical load V_Ed [kN] to {}", self.new_v_ed_kn)
    }
}
//#endregion 🔖️ChangeVEdKn
