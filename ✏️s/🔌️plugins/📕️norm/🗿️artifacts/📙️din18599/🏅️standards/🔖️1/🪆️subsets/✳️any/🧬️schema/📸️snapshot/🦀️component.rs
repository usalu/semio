//! 🧬️ Din18599 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::artifacts::din18599::{MonthlyClimate, UseClass};
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.din18599", layout = "lines")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Snapshot {
    #[state(persistent)]
    pub use_class: UseClass,
    #[dsl(unit = "m2")]
    #[state(persistent)]
    pub heated_area_m2: f64,
    #[state(persistent)]
    pub occupants: u32,
    #[state(persistent)]
    pub h_t: f64,
    #[state(persistent)]
    pub h_v: f64,
    #[dsl(block)]
    #[state(persistent)]
    pub climate: MonthlyClimate,
    #[state(persistent)]
    pub internal_gains_w_m2: f64,
    #[state(persistent)]
    pub solar_gains_kwh: f64,
    #[state(persistent)]
    pub system_losses_kwh: f64,
    #[state(persistent)]
    pub renewable_kwh: f64,
    #[state(persistent)]
    pub annual_limit_kwh: f64,
    #[state(persistent)]
    pub energy_carrier: String,
    #[state(persistent)]
    pub reference_q_p_kwh: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Din18599Snapshot, extension = "din18599", envelope_id = "norm.din18599");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din18599Snapshot {
    fn default() -> Self {
        Self {
            use_class: UseClass::Residential,
            heated_area_m2: 100.0,
            occupants: 4,
            h_t: 92.12124613902822,
            h_v: 40.800000000000004,
            climate: MonthlyClimate {
                theta_e_c: [-14.0, -11.186533479473212, -3.4999999999999964, 7.000000000000001, 17.5, 25.186533479473212, 28.0, 25.186533479473212, 17.5, 7.000000000000001, -3.4999999999999964, -11.186533479473212],
                g_h_w_m2: [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0],
            },
            internal_gains_w_m2: 3.5,
            solar_gains_kwh: 84.0,
            system_losses_kwh: 800.0,
            renewable_kwh: 1500.0,
            annual_limit_kwh: 7500.0,
            energy_carrier: "natural_gas".into(),
            reference_q_p_kwh: 10000.0,
        }
    }
}
//#endregion 🔖️Snapshot
