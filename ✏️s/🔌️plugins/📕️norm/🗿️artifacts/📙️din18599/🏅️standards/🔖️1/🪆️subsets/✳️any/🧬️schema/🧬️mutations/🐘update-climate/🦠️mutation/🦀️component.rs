//! 🐘 `update-climate` payload — replaces the Din18599 document's `climate` facet
//! (monthly external temperature + solar irradiance profile). Per `📓️derivation-rules.md` rule 1's
//! `update-<facet>` exception: `MonthlyClimate`'s two twelve-month arrays (`theta_e_c`, `g_h_w_m2`)
//! are entered together as one climate dataset (e.g. loaded from a reference climate zone via
//! `MonthlyClimate::german_reference`), never meaningfully edited one month/array at a time from this
//! app's own input surface — an inseparable ≥2-field facet, not independently-set scalars.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::{Din18599Snapshot, MonthlyClimate};
use serde::{Deserialize, Serialize};

//#region 🔖️UpdateClimate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClimate {
    pub new_climate: MonthlyClimate,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for UpdateClimate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "climate", kind: "update-climate", record: "UpdatedClimate" };

    fn diff(&self, base: &Din18599Snapshot) -> Din18599Diff {
        crate::artifacts::din18599::mutations::update_climate::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        crate::artifacts::din18599::mutations::update_climate::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Update monthly climate profile".to_string()
    }
}
//#endregion 🔖️UpdateClimate
