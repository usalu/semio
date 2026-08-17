//! 🌍 `change-seismic-action` — sets the EN 1990 document's seismic accidental action `A_Ed`
//! [kN], combined per Eq. 6.12b; `0.0` disables the seismic design situation.

use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSeismicAction {
    pub new_seismic_a_ed_kn: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeSeismicAction {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "seismic-action", kind: "change-seismic-action", record: "ChangedSeismicAction" };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change seismic action A_Ed to {} kN", self.new_seismic_a_ed_kn)
    }
}
//#endregion 🔖️Payload
