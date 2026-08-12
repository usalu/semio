//! 🛁 `change-n-pile-ed-kn` payload — changes the En1997 document's `n_pile_ed_kn` (design pile axial load N_Ed [kN]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNPileEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNPileEdKn {
    pub new_n_pile_ed_kn: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeNPileEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-pile-ed-kn", kind: "change-n-pile-ed-kn", record: "ChangedNPileEdKn" };

    fn diff(&self, base: &En1997Snapshot) -> En1997Diff {
        crate::artifacts::en1997::mutations::change_n_pile_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_n_pile_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design pile axial load N_Ed [kN] to {}", self.new_n_pile_ed_kn)
    }
}
//#endregion 🔖️ChangeNPileEdKn
