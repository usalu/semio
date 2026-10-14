//! 🧬️ Din16798 artifact schema + DIN EN 16798 compliance helpers.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, OccupancyType, Quantity, QuantityKind, Remedy, RemedyBound, SubjectRef};
use semio_s_artifact_norm_din16798_model::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub theta_rm_c: f64,
    #[state(artifact)]
    pub outdoor_co2_ppm: f64,
    #[state(artifact)]
    pub zones: Vec<ZoneDocument>,
    #[state(artifact)]
    pub vent_systems: Vec<VentSystemDocument>,
    #[state(artifact)]
    pub envelope_n50_h_inv: f64,
    #[state(artifact)]
    pub envelope_volume_m3: f64,
    #[state(artifact)]
    pub cellar_area_m2: f64,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: f64,
    #[state(artifact)]
    pub night_setback_k: f64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din16798Artifact {
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_din16798_model::Din16798Snapshot {
        semio_s_artifact_norm_din16798_model::Din16798Snapshot {
            annex: self.annex,
            theta_rm_c: self.theta_rm_c,
            outdoor_co2_ppm: self.outdoor_co2_ppm,
            zones: self.zones.clone(),
            vent_systems: self.vent_systems.clone(),
            envelope_n50_h_inv: self.envelope_n50_h_inv,
            envelope_volume_m3: self.envelope_volume_m3,
            cellar_area_m2: self.cellar_area_m2,
            cellar_ventilation_m3_h: self.cellar_ventilation_m3_h,
            night_setback_k: self.night_setback_k,
        }
    }
    pub fn from_snapshot(snapshot: semio_s_artifact_norm_din16798_model::Din16798Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            theta_rm_c: snapshot.theta_rm_c,
            outdoor_co2_ppm: snapshot.outdoor_co2_ppm,
            zones: snapshot.zones,
            vent_systems: snapshot.vent_systems,
            envelope_n50_h_inv: snapshot.envelope_n50_h_inv,
            envelope_volume_m3: snapshot.envelope_volume_m3,
            cellar_area_m2: snapshot.cellar_area_m2,
            cellar_ventilation_m3_h: snapshot.cellar_ventilation_m3_h,
            night_setback_k: snapshot.night_setback_k,
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din16798` — twenty handcrafted schema leaves.
pub fn din16798_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.din16798",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_din16798_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_din16798_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_din16798_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

