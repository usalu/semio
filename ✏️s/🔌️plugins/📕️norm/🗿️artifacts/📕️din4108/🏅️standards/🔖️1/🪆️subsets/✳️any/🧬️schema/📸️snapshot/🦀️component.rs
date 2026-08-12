//! 🧬️ Din4108 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::artifacts::din4108::LayerDocument;
use crate::document::ClimateZoneDe;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.din4108", layout = "lines")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Snapshot {
    #[state(persistent)]
    pub category: String,
    #[dsl(table)]
    #[state(persistent)]
    pub layers: Vec<crate::artifacts::din4108::LayerDocument>,
    #[state(persistent)]
    pub climate: ClimateZoneDe,
    #[state(persistent)]
    pub airtightness_n50: f64,
    #[state(persistent)]
    pub psi_times_l_sum: f64,
    #[state(persistent)]
    pub rh_int: f64,
    #[state(persistent)]
    pub catalog_id: String,
    #[state(persistent)]
    pub material_id: String,
    #[state(persistent)]
    pub airtightness_class: String,
    #[state(persistent)]
    pub t_int_c: f64,
    #[state(persistent)]
    pub solar_absorptance: f64,
    #[state(persistent)]
    pub irradiance_w_m2: f64,
    #[state(persistent)]
    pub moisture_mu_exterior: f64,
    #[state(persistent)]
    pub moisture_mu_interior: f64,
    #[dsl(unit = "m2")]
    #[state(persistent)]
    pub envelope_area_m2: f64,
    #[state(persistent)]
    pub bb2_details_conform: bool,
    #[state(persistent)]
    pub application_type: String,
    #[state(persistent)]
    pub declared_application_class: String,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️component.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Din4108Snapshot, extension = "din4108", envelope_id = "norm.din4108");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din4108Snapshot {
    fn default() -> Self {
        Self {
            category: "residential".into(),
            layers: vec![LayerDocument { thickness_m: 0.24, lambda_w_mk: 0.81 }, LayerDocument { thickness_m: 0.14, lambda_w_mk: 0.035 }],
            climate: ClimateZoneDe::Zone2,
            airtightness_n50: 2.5,
            psi_times_l_sum: 0.02,
            rh_int: 0.5,
            catalog_id: "AW-01".into(),
            material_id: "mineral_wool".into(),
            airtightness_class: "class2".into(),
            t_int_c: 20.0,
            solar_absorptance: 0.6,
            irradiance_w_m2: 600.0,
            moisture_mu_exterior: 15.0,
            moisture_mu_interior: 1.3,
            envelope_area_m2: 100.0,
            bb2_details_conform: true,
            application_type: "DEO".into(),
            declared_application_class: "dk".into(),
        }
    }
}
//#endregion 🔖️Snapshot
