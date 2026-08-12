//! 🦂 `change-tank-v-rd-kn` payload — changes the En1998 document's `tank_v_rd_kn` (tank shear resistance V_Rd [kN]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeTankVRdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTankVRdKn {
    pub new_tank_v_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeTankVRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "tank-v-rd-kn", kind: "change-tank-v-rd-kn", record: "ChangedTankVRdKn" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_tank_v_rd_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_tank_v_rd_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change tank shear resistance V_Rd [kN] to {}", self.new_tank_v_rd_kn)
    }
}
//#endregion 🔖️ChangeTankVRdKn
