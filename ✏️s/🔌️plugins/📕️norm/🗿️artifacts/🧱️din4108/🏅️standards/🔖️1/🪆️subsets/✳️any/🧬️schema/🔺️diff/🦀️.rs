//! 🧬️ Din4108 diff schema — sparse field delta over the artifact.

use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Diff {
    #[state(artifact)]
    pub climate_zone: Option<crate::document::ClimateZoneDe>,
    #[state(artifact)]
    pub usage: Option<String>,
    #[state(artifact)]
    pub t_int_c: Option<f64>,
    #[state(artifact)]
    pub rh_int: Option<f64>,
    #[state(artifact)]
    pub has_mechanical_ventilation: Option<bool>,
    #[state(artifact)]
    pub airtightness_n50: Option<f64>,
    #[state(artifact)]
    pub bb2_details_conform: Option<bool>,
    #[state(artifact)]
    pub zones: Option<Din4108ZoneList>,
    #[state(artifact)]
    pub elements: Option<Din4108ElementList>,
    #[state(artifact)]
    pub thermal_bridges: Option<Din4108ThermalBridgeList>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108ZoneList {
    pub values: Vec<crate::ThermalZone>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108ElementList {
    pub values: Vec<crate::EnvelopeElement>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108ThermalBridgeList {
    pub values: Vec<crate::ThermalBridge>,
}

impl protocol::MutationDiff<crate::Din4108Snapshot> for Din4108Diff {
    fn apply(&self, base: &crate::Din4108Snapshot) -> Result<crate::Din4108Snapshot, protocol::MutationApplyError> {
        let mut next = base.clone();
        if let Some(v) = self.climate_zone { next.climate_zone = v; }
        if let Some(v) = &self.usage { next.usage = v.clone(); }
        if let Some(v) = self.t_int_c { next.t_int_c = v; }
        if let Some(v) = self.rh_int { next.rh_int = v; }
        if let Some(v) = self.has_mechanical_ventilation { next.has_mechanical_ventilation = v; }
        if let Some(v) = self.airtightness_n50 { next.airtightness_n50 = v; }
        if let Some(v) = self.bb2_details_conform { next.bb2_details_conform = v; }
        if let Some(v) = &self.zones { next.zones = v.values.clone(); }
        if let Some(v) = &self.elements { next.elements = v.values.clone(); }
        if let Some(v) = &self.thermal_bridges { next.thermal_bridges = v.values.clone(); }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(climate_zone);
        take!(usage);
        take!(t_int_c);
        take!(rh_int);
        take!(has_mechanical_ventilation);
        take!(airtightness_n50);
        take!(bb2_details_conform);
        take!(zones);
        take!(elements);
        take!(thermal_bridges);
    }
}

