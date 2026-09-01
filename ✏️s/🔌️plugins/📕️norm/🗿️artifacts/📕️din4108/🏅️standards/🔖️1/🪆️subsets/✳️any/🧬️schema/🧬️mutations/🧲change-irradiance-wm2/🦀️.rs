//! 🔆 `change-irradiance-wm2` — sets the DIN 4108 `irradiance_w_m2` scalar. Kebab form is
//! `#[derive(dsl::Mutations)]`-mandated (its `to_kebab` groups a trailing acronym+digit run like
//! `WM2` as one word, `wm2` — never `w-m2`), not a style choice.


use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeIrradianceWM2 {
    pub new_irradiance_w_m2: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeIrradianceWM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "irradiance-wm2", kind: "change-irradiance-wm2", record: "ChangedIrradianceWM2" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change irradiance w m2 to {}", self.new_irradiance_w_m2)
    }
}
//#endregion 🔖️Payload
