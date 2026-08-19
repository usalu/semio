//! 🔧 `change-sfp-wm3-s` payload — changes the Din16798 document's `sfp_w_m3_s` (specific fan power).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSfpWM3S
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSfpWM3S {
    pub new_sfp_w_m3_s: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeSfpWM3S {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sfp-wm3-s", kind: "change-sfp-wm3-s", record: "ChangedSfpWM3S" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_sfp_w_m3_s::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_sfp_w_m3_s::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change specific fan power to {}", self.new_sfp_w_m3_s)
    }
}
//#endregion 🔖️ChangeSfpWM3S
