//! 🧬️ Din18599 diff schema — sparse field delta over the artifact.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din18599 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
    pub climate: Option<crate::MonthlyClimate>,
    #[state(artifact)]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle::optional"))]
    pub climate_table: Option<crate::Din18599ClimateChild>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ZoneList {
    pub values: Vec<crate::ThermalZone>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din18599ElementList {
    pub values: Vec<crate::EnvelopeElement>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::Din18599Artifact;
use crate::Din18599Snapshot;
use protocol::MutationDiff;

impl Din18599Diff {
    pub fn apply_to_artifact(&self, artifact: &Din18599Artifact) -> protocol::MutationApplyResult<Din18599Artifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(value) = self.building_category {
                next.building_category = value;
            }
            if let Some(value) = self.attachment {
                next.attachment = value;
            }
            if let Some(value) = self.use_class {
                next.use_class = value;
            }
            if let Some(value) = self.method {
                next.method = value;
            }
            if let Some(value) = self.net_floor_area_m2 {
                next.net_floor_area_m2 = value;
            }
            if let Some(value) = self.heated_volume_m3 {
                next.heated_volume_m3 = value;
            }
            if let Some(value) = self.geg_qp_factor {
                next.geg_qp_factor = value;
            }
            if let Some(value) = self.delta_u_wb_w_m2k {
                next.delta_u_wb_w_m2k = value;
            }
            if let Some(value) = self.automation_class {
                next.automation_class = value;
            }
            if let Some(list) = &self.zones {
                next.zones = list.values.clone();
            }
            if let Some(list) = &self.elements {
                next.elements = list.values.clone();
            }
            if let Some(value) = &self.heating {
                next.heating = value.clone();
            }
            if let Some(value) = &self.dhw {
                next.dhw = value.clone();
            }
            if let Some(value) = &self.ventilation {
                next.ventilation = value.clone();
            }
            if let Some(value) = &self.cooling {
                next.cooling = value.clone();
            }
            if let Some(value) = &self.lighting {
                next.lighting = value.clone();
            }
            if let Some(value) = &self.renewables {
                next.renewables = value.clone();
            }
            if let Some(value) = &self.climate {
                next.climate = value.clone();
            }
            if let Some(value) = &self.climate_table {
                next.climate_table = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<Din18599Snapshot> for Din18599Diff {
    fn apply(&self, snapshot: &Din18599Snapshot) -> protocol::MutationApplyResult<Din18599Snapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(value) = self.building_category {
                next.building_category = value;
            }
            if let Some(value) = self.attachment {
                next.attachment = value;
            }
            if let Some(value) = self.use_class {
                next.use_class = value;
            }
            if let Some(value) = self.method {
                next.method = value;
            }
            if let Some(value) = self.net_floor_area_m2 {
                next.net_floor_area_m2 = value;
            }
            if let Some(value) = self.heated_volume_m3 {
                next.heated_volume_m3 = value;
            }
            if let Some(value) = self.geg_qp_factor {
                next.geg_qp_factor = value;
            }
            if let Some(value) = self.delta_u_wb_w_m2k {
                next.delta_u_wb_w_m2k = value;
            }
            if let Some(value) = self.automation_class {
                next.automation_class = value;
            }
            if let Some(list) = &self.zones {
                next.zones = list.values.clone();
            }
            if let Some(list) = &self.elements {
                next.elements = list.values.clone();
            }
            if let Some(value) = &self.heating {
                next.heating = value.clone();
            }
            if let Some(value) = &self.dhw {
                next.dhw = value.clone();
            }
            if let Some(value) = &self.ventilation {
                next.ventilation = value.clone();
            }
            if let Some(value) = &self.cooling {
                next.cooling = value.clone();
            }
            if let Some(value) = &self.lighting {
                next.lighting = value.clone();
            }
            if let Some(value) = &self.renewables {
                next.renewables = value.clone();
            }
            if let Some(value) = &self.climate {
                next.climate = value.clone();
            }
            if let Some(value) = &self.climate_table {
                next.climate_table = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(building_category);
        take!(attachment);
        take!(use_class);
        take!(method);
        take!(net_floor_area_m2);
        take!(heated_volume_m3);
        take!(geg_qp_factor);
        take!(delta_u_wb_w_m2k);
        take!(automation_class);
        take!(zones);
        take!(elements);
        take!(heating);
        take!(dhw);
        take!(ventilation);
        take!(cooling);
        take!(lighting);
        take!(renewables);
        take!(climate);
        take!(climate_table);
    }
}
