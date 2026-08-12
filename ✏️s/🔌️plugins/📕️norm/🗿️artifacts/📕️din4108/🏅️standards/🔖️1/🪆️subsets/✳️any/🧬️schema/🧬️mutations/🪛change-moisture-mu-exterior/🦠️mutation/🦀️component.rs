//! 🌫️ `change-moisture-mu-exterior` — sets the DIN 4108 `moisture_mu_exterior` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMoistureMuExterior {
    pub new_moisture_mu_exterior: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeMoistureMuExterior {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "moisture-mu-exterior", kind: "change-moisture-mu-exterior", record: "ChangedMoistureMuExterior" };

    fn diff(&self, base: &Din4108Snapshot) -> <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change moisture mu exterior to {}", self.new_moisture_mu_exterior)
    }
}
//#endregion 🔖️Payload
