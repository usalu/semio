//! 🧬️ Din18599 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din18599_model::{Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UsageProfile, UseClass, VentilationSystem};
use framework_schema::ArtifactSchema;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

//#region 🔖️Artifact
/// 🧬️ Full Din18599 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Artifact {
    #[state(artifact)]
    pub building_category: BuildingCategory,
    #[state(artifact)]
    pub attachment: Attachment,
    #[state(artifact)]
    pub use_class: UseClass,
    #[state(artifact)]
    pub method: CalculationMethod,
    #[state(artifact)]
    pub net_floor_area_m2: f64,
    #[state(artifact)]
    pub heated_volume_m3: f64,
    #[state(artifact)]
    pub geg_qp_factor: f64,
    #[state(artifact)]
    pub delta_u_wb_w_m2k: f64,
    #[state(artifact)]
    pub automation_class: AutomationClass,
    #[state(artifact)]
    pub zones: Vec<ThermalZone>,
    #[state(artifact)]
    pub elements: Vec<EnvelopeElement>,
    #[state(artifact)]
    pub heating: HeatingSystem,
    #[state(artifact)]
    pub dhw: DhwSystem,
    #[state(artifact)]
    pub ventilation: VentilationSystem,
    #[state(artifact)]
    pub cooling: CoolingSystem,
    #[state(artifact)]
    pub lighting: LightingSystem,
    #[state(artifact)]
    pub renewables: Renewables,
    #[state(artifact)]
    pub climate: MonthlyClimate,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle"))]
    pub climate_table: Din18599ClimateChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din18599Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Din18599Snapshot {
        Din18599Snapshot {
            building_category: self.building_category,
            attachment: self.attachment,
            use_class: self.use_class,
            method: self.method,
            net_floor_area_m2: self.net_floor_area_m2,
            heated_volume_m3: self.heated_volume_m3,
            geg_qp_factor: self.geg_qp_factor,
            delta_u_wb_w_m2k: self.delta_u_wb_w_m2k,
            automation_class: self.automation_class,
            zones: self.zones.clone(),
            elements: self.elements.clone(),
            heating: self.heating.clone(),
            dhw: self.dhw.clone(),
            ventilation: self.ventilation.clone(),
            cooling: self.cooling.clone(),
            lighting: self.lighting.clone(),
            renewables: self.renewables.clone(),
            climate: self.climate.clone(),
            climate_table: self.climate_table.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Din18599Snapshot) -> Self {
        Self {
            building_category: snapshot.building_category,
            attachment: snapshot.attachment,
            use_class: snapshot.use_class,
            method: snapshot.method,
            net_floor_area_m2: snapshot.net_floor_area_m2,
            heated_volume_m3: snapshot.heated_volume_m3,
            geg_qp_factor: snapshot.geg_qp_factor,
            delta_u_wb_w_m2k: snapshot.delta_u_wb_w_m2k,
            automation_class: snapshot.automation_class,
            zones: snapshot.zones,
            elements: snapshot.elements,
            heating: snapshot.heating,
            dhw: snapshot.dhw,
            ventilation: snapshot.ventilation,
            cooling: snapshot.cooling,
            lighting: snapshot.lighting,
            renewables: snapshot.renewables,
            climate: snapshot.climate,
            climate_table: snapshot.climate_table,
        }
    }

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din18599` — twenty handcrafted schema leaves.
pub fn din18599_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.din18599",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_din18599_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_din18599_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_din18599_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

//#region 🪢️Mounts
#[path = "."]
pub mod checks {
    #[cfg(test)]
    #[path = "🧪️tests/🔬️oracle/🦀️.rs"]
    mod oracle_tests;
}
//#endregion 🪢️Mounts
