//! 🔧 `change-air-speed-ms` payload — changes the Din16798 document's `air_speed_m_s` (air speed).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAirSpeedMS
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAirSpeedMS {
    pub new_air_speed_m_s: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeAirSpeedMS {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "air-speed-ms", kind: "change-air-speed-ms", record: "ChangedAirSpeedMS" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_air_speed_m_s::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_air_speed_m_s::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change air speed to {}", self.new_air_speed_m_s)
    }
}
//#endregion 🔖️ChangeAirSpeedMS
