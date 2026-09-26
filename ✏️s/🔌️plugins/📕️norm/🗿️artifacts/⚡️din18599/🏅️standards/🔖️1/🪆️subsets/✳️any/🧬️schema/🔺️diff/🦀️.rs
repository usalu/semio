//! 🧬️ Din18599 diff schema — sparse field delta over the artifact.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din18599 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Diff {
    #[state(artifact)]
    pub building_category: Option<crate::BuildingCategory>,
    #[state(artifact)]
    pub attachment: Option<crate::Attachment>,
    #[state(artifact)]
    pub use_class: Option<crate::UseClass>,
    #[state(artifact)]
    pub method: Option<crate::CalculationMethod>,
    #[state(artifact)]
    pub net_floor_area_m2: Option<f64>,
    #[state(artifact)]
    pub heated_volume_m3: Option<f64>,
    #[state(artifact)]
    pub geg_qp_factor: Option<f64>,
    #[state(artifact)]
    pub delta_u_wb_w_m2k: Option<f64>,
    #[state(artifact)]
    pub automation_class: Option<crate::AutomationClass>,
    #[state(artifact)]
    pub zones: Option<Din18599ZoneList>,
    #[state(artifact)]
    pub elements: Option<Din18599ElementList>,
    #[state(artifact)]
    pub heating: Option<crate::HeatingSystem>,
    #[state(artifact)]
    pub dhw: Option<crate::DhwSystem>,
    #[state(artifact)]
    pub ventilation: Option<crate::VentilationSystem>,
    #[state(artifact)]
    pub cooling: Option<crate::CoolingSystem>,
    #[state(artifact)]
    pub lighting: Option<crate::LightingSystem>,
    #[state(artifact)]
    pub renewables: Option<crate::Renewables>,
    #[state(artifact)]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle::optional"))]
    pub climate: Option<crate::Din18599ClimateChild>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ZoneList {
    pub values: Vec<crate::ThermalZone>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ElementList {
    pub values: Vec<crate::EnvelopeElement>,
}
//#endregion 🔖️DeltaHelpers
