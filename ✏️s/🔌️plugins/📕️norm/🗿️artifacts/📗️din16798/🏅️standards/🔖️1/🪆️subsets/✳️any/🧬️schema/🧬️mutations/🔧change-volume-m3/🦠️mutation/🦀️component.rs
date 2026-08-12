//! 🔧 `change-volume-m3` payload — changes the Din16798 document's `volume_m3` (building volume).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeVolumeM3
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeVolumeM3 {
    pub new_volume_m3: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeVolumeM3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "volume-m3", kind: "change-volume-m3", record: "ChangedVolumeM3" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_volume_m3::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_volume_m3::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change building volume to {}", self.new_volume_m3)
    }
}
//#endregion 🔖️ChangeVolumeM3
