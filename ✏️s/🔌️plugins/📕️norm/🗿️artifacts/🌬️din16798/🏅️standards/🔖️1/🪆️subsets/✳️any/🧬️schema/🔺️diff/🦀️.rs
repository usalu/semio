//! 🧬️ Din16798 diff — sparse field delta; list fields rewritten as wholes.

use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::Din16798Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub theta_rm_c: Option<f64>,
    #[state(artifact)]
    pub outdoor_co2_ppm: Option<f64>,
    #[state(artifact)]
    pub zones: Option<Din16798ZoneList>,
    #[state(artifact)]
    pub vent_systems: Option<Din16798VentList>,
    #[state(artifact)]
    pub envelope_n50_h_inv: Option<f64>,
    #[state(artifact)]
    pub envelope_volume_m3: Option<f64>,
    #[state(artifact)]
    pub cellar_area_m2: Option<f64>,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: Option<f64>,
    #[state(artifact)]
    pub night_setback_k: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din16798ZoneList {
    pub values: Vec<crate::ZoneDocument>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din16798VentList {
    pub values: Vec<crate::VentSystemDocument>,
}

impl protocol::MutationDiff<crate::Din16798Snapshot> for Din16798Diff {
    fn apply(&self, base: &crate::Din16798Snapshot) -> Result<crate::Din16798Snapshot, protocol::MutationApplyError> {
        let mut next = base.clone();
        if let Some(v) = self.annex { next.annex = v; }
        if let Some(v) = self.theta_rm_c { next.theta_rm_c = v; }
        if let Some(v) = self.outdoor_co2_ppm { next.outdoor_co2_ppm = v; }
        if let Some(v) = &self.zones { next.zones = v.values.clone(); }
        if let Some(v) = &self.vent_systems { next.vent_systems = v.values.clone(); }
        if let Some(v) = self.envelope_n50_h_inv { next.envelope_n50_h_inv = v; }
        if let Some(v) = self.envelope_volume_m3 { next.envelope_volume_m3 = v; }
        if let Some(v) = self.cellar_area_m2 { next.cellar_area_m2 = v; }
        if let Some(v) = self.cellar_ventilation_m3_h { next.cellar_ventilation_m3_h = v; }
        if let Some(v) = self.night_setback_k { next.night_setback_k = v; }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(theta_rm_c);
        take!(outdoor_co2_ppm);
        take!(zones);
        take!(vent_systems);
        take!(envelope_n50_h_inv);
        take!(envelope_volume_m3);
        take!(cellar_area_m2);
        take!(cellar_ventilation_m3_h);
        take!(night_setback_k);
    }
}
