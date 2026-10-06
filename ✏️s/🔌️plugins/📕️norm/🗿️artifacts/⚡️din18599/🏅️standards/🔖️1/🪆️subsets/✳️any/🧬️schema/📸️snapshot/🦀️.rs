//! 🧬️ Din18599 snapshot schema — complete building subject (zones, envelope, systems).

use crate::{
    Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted Din18599 building energy subject. The monthly climate is parent-owned state and the composed
/// `s.stdio.semio`/`table` child `climateTable` is derived from it; envelope and zones are id-keyed lists; plant systems
/// are nested records. Derived H_T / H_V / Q_P are never stored as free inputs.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(extension = "din18599")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Snapshot {
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
    #[dsl(table)]
    #[state(artifact)]
    pub zones: Vec<ThermalZone>,
    #[dsl(table)]
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

//#region 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din18599Snapshot {
    fn default() -> Self {
        crate::subjects::compliant_detached_house()
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge











//#endregion 🌉️ExternalCodecBridge
