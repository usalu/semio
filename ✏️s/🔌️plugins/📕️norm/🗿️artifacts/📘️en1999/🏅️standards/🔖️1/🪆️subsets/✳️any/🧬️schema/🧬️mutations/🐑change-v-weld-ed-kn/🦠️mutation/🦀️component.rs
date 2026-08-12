//! 🐑 `change-v-weld-ed-kn` payload — changes the En1999 document's `v_weld_ed_kn` (design weld shear force V_Ed [kN]).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeVWeldEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeVWeldEdKn {
    pub new_v_weld_ed_kn: f64,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeVWeldEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-weld-ed-kn", kind: "change-v-weld-ed-kn", record: "ChangedVWeldEdKn" };

    fn diff(&self, base: &En1999Snapshot) -> En1999Diff {
        crate::artifacts::en1999::mutations::change_v_weld_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_v_weld_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design weld shear force V_Ed [kN] to {}", self.new_v_weld_ed_kn)
    }
}
//#endregion 🔖️ChangeVWeldEdKn
