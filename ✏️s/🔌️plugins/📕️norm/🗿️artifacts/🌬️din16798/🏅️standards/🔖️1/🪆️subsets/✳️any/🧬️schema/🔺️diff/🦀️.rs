//! 🧬️ Din16798 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_norm_contract::{norm_list_delta, norm_row_patch};

use crate::Din16798Snapshot;

//#region 🔖️Rows
norm_row_patch! {
    /// 🩹 Sparse field patch of one `ZoneDocument`.
    pub Din16798ZonePatch of crate::ZoneDocument { set { name: String, usage_type: String, floor_area_m2: f64, occupants: u32, comfort_category: String, pollution_class: String, comfort_model: String, t_op_winter_c: f64, t_op_summer_c: f64, air_speed_m_s: f64, clothing_clo: f64, metabolic_rate_met: f64, rh_percent: f64, outdoor_air_supplied_m3_h: f64, co2_ppm: f64, illuminance_lx: f64, noise_db: f64, turbulence_intensity_percent: f64, vent_method: String, vent_system_id: String } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `ZoneDocument` list.
    pub Din16798ZoneDelta { addition: Din16798ZoneAddition, modification: Din16798ZoneModification, row: crate::ZoneDocument, patch: Din16798ZonePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `VentSystemDocument`.
    pub Din16798VentSystemPatch of crate::VentSystemDocument { set { name: String, system_type: String, sfp_w_m3_s: f64, sfp_required_class: u8, heat_recovery_eta: f64, oda_class: String, filter_sup_class: String, years_since_inspection: u32, humidification_required_kg_h: f64, humidification_provided_kg_h: f64, fan_q_v_m3_s: f64, fan_t_run_h: f64, duct_class: String, duct_test_pressure_pa: f64, duct_leakage_m3_s_m2: f64, design_airflow_m3_h: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `VentSystemDocument` list.
    pub Din16798VentSystemDelta { addition: Din16798VentSystemAddition, modification: Din16798VentSystemModification, row: crate::VentSystemDocument, patch: Din16798VentSystemPatch, key: id }
}
//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse delta for the Din16798 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub theta_rm_c: Option<f64>,
    #[state(artifact)]
    pub outdoor_co2_ppm: Option<f64>,
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
    #[state(artifact)]
    pub zones: Din16798ZoneDelta,
    #[state(artifact)]
    pub vent_systems: Din16798VentSystemDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<Din16798Snapshot> for Din16798Diff {
    fn apply(&self, base: &Din16798Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Din16798Snapshot> {
        let mut next = base.clone();
        if let Some(value) = self.annex {
            next.annex = value;
        }
        if let Some(value) = self.theta_rm_c {
            next.theta_rm_c = value;
        }
        if let Some(value) = self.outdoor_co2_ppm {
            next.outdoor_co2_ppm = value;
        }
        if let Some(value) = self.envelope_n50_h_inv {
            next.envelope_n50_h_inv = value;
        }
        if let Some(value) = self.envelope_volume_m3 {
            next.envelope_volume_m3 = value;
        }
        if let Some(value) = self.cellar_area_m2 {
            next.cellar_area_m2 = value;
        }
        if let Some(value) = self.cellar_ventilation_m3_h {
            next.cellar_ventilation_m3_h = value;
        }
        if let Some(value) = self.night_setback_k {
            next.night_setback_k = value;
        }
        next.zones = self.zones.commit_onto(&base.zones).map_err(|error| error.under(["zones"]))?;
        next.vent_systems = self.vent_systems.commit_onto(&base.vent_systems).map_err(|error| error.under(["ventSystems"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.theta_rm_c.is_some() {
            self.theta_rm_c = other.theta_rm_c;
        }
        if other.outdoor_co2_ppm.is_some() {
            self.outdoor_co2_ppm = other.outdoor_co2_ppm;
        }
        if other.envelope_n50_h_inv.is_some() {
            self.envelope_n50_h_inv = other.envelope_n50_h_inv;
        }
        if other.envelope_volume_m3.is_some() {
            self.envelope_volume_m3 = other.envelope_volume_m3;
        }
        if other.cellar_area_m2.is_some() {
            self.cellar_area_m2 = other.cellar_area_m2;
        }
        if other.cellar_ventilation_m3_h.is_some() {
            self.cellar_ventilation_m3_h = other.cellar_ventilation_m3_h;
        }
        if other.night_setback_k.is_some() {
            self.night_setback_k = other.night_setback_k;
        }
        self.zones.absorb(other.zones);
        self.vent_systems.absorb(other.vent_systems);
    }
}

impl DiffAlgebra<Din16798Snapshot> for Din16798Diff {
    fn inverse(&self, base: &Din16798Snapshot) -> Self {
        Self {
            annex: self.annex.map(|_| base.annex),
            theta_rm_c: self.theta_rm_c.map(|_| base.theta_rm_c),
            outdoor_co2_ppm: self.outdoor_co2_ppm.map(|_| base.outdoor_co2_ppm),
            envelope_n50_h_inv: self.envelope_n50_h_inv.map(|_| base.envelope_n50_h_inv),
            envelope_volume_m3: self.envelope_volume_m3.map(|_| base.envelope_volume_m3),
            cellar_area_m2: self.cellar_area_m2.map(|_| base.cellar_area_m2),
            cellar_ventilation_m3_h: self.cellar_ventilation_m3_h.map(|_| base.cellar_ventilation_m3_h),
            night_setback_k: self.night_setback_k.map(|_| base.night_setback_k),
            zones: self.zones.inverse(&base.zones),
            vent_systems: self.vent_systems.inverse(&base.vent_systems),
        }
    }

    fn between(base: &Din16798Snapshot, other: &Din16798Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then_some(other.annex),
            theta_rm_c: (base.theta_rm_c != other.theta_rm_c).then_some(other.theta_rm_c),
            outdoor_co2_ppm: (base.outdoor_co2_ppm != other.outdoor_co2_ppm).then_some(other.outdoor_co2_ppm),
            envelope_n50_h_inv: (base.envelope_n50_h_inv != other.envelope_n50_h_inv).then_some(other.envelope_n50_h_inv),
            envelope_volume_m3: (base.envelope_volume_m3 != other.envelope_volume_m3).then_some(other.envelope_volume_m3),
            cellar_area_m2: (base.cellar_area_m2 != other.cellar_area_m2).then_some(other.cellar_area_m2),
            cellar_ventilation_m3_h: (base.cellar_ventilation_m3_h != other.cellar_ventilation_m3_h).then_some(other.cellar_ventilation_m3_h),
            night_setback_k: (base.night_setback_k != other.night_setback_k).then_some(other.night_setback_k),
            zones: Din16798ZoneDelta::between(&base.zones, &other.zones),
            vent_systems: Din16798VentSystemDelta::between(&base.vent_systems, &other.vent_systems),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.theta_rm_c.is_none()
            && self.outdoor_co2_ppm.is_none()
            && self.envelope_n50_h_inv.is_none()
            && self.envelope_volume_m3.is_none()
            && self.cellar_area_m2.is_none()
            && self.cellar_ventilation_m3_h.is_none()
            && self.night_setback_k.is_none()
            && self.zones.is_empty()
            && self.vent_systems.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
