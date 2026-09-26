//! 🧬️ Din16798 snapshot — building with zones and ventilation systems.

use crate::document::AnnexChoice;
use crate::{VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.din16798", layout = "lines")]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub theta_rm_c: f64,
    #[state(artifact)]
    pub outdoor_co2_ppm: f64,
    #[dsl(table)]
    #[state(artifact)]
    pub zones: Vec<ZoneDocument>,
    #[dsl(table)]
    #[state(artifact)]
    pub vent_systems: Vec<VentSystemDocument>,
    #[state(artifact)]
    pub envelope_n50_h_inv: f64,
    #[dsl(unit = "m3")]
    #[state(artifact)]
    pub envelope_volume_m3: f64,
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub cellar_area_m2: f64,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: f64,
    #[dsl(unit = "K")]
    #[state(artifact)]
    pub night_setback_k: f64,
}

crate::impl_norm_artifact_record!(Din16798Snapshot, extension = "din16798", envelope_id = "norm.din16798");

impl Default for Din16798Snapshot {
    fn default() -> Self {
        Self::compliant_office()
    }
}

impl Din16798Snapshot {
    /// 🏢 Realistic compliant Cat II open office with central mechanical ventilation.
    pub fn compliant_office() -> Self {
        Self {
            annex: AnnexChoice::De,
            theta_rm_c: 15.0,
            outdoor_co2_ppm: 400.0,
            zones: vec![ZoneDocument::default()],
            vent_systems: vec![VentSystemDocument::default()],
            envelope_n50_h_inv: 1.5,
            envelope_volume_m3: 600.0,
            cellar_area_m2: 0.0,
            cellar_ventilation_m3_h: 0.0,
            night_setback_k: 4.0,
        }
    }

    /// 🏠 Residential Cat II with EN 16798-1 §6.3 method 3 predefined rates.
    pub fn residential_method3() -> Self {
        let mut z = ZoneDocument::default();
        z.id = "zone-flat".into();
        z.name = "Living room".into();
        z.usage_type = "residential".into();
        z.floor_area_m2 = 80.0;
        z.occupants = 3;
        z.vent_method = "method_3_predefined_rates".into();
        z.outdoor_air_supplied_m3_h = 363.6; // 80 m² × 1.0 + 3 × 7.0 L/s → m³/h
        z.co2_ppm = 850.0;
        z.noise_db = 24.0;
        let mut v = VentSystemDocument::default();
        v.id = "vent-res".into();
        v.name = "Residential unit".into();
        v.system_type = "decentral_mech".into();
        v.design_airflow_m3_h = 400.0;
        v.sfp_w_m3_s = 900.0;
        v.fan_q_v_m3_s = 400.0 / 3600.0;
        z.vent_system_id = v.id.clone();
        Self {
            annex: AnnexChoice::De,
            theta_rm_c: 12.0,
            outdoor_co2_ppm: 400.0,
            zones: vec![z],
            vent_systems: vec![v],
            envelope_n50_h_inv: 1.2,
            envelope_volume_m3: 200.0,
            cellar_area_m2: 0.0,
            cellar_ventilation_m3_h: 0.0,
            night_setback_k: 3.0,
        }
    }

    /// ⚠️ Non-compliant multi-failure office across Parts 1 and 3.
    pub fn noncompliant_office() -> Self {
        let mut z = ZoneDocument::default();
        z.id = "zone-office-fail".into();
        z.name = "Overcrowded office".into();
        z.t_op_summer_c = 28.5;
        z.outdoor_air_supplied_m3_h = 400.0;
        z.co2_ppm = 1400.0;
        z.noise_db = 42.0;
        z.illuminance_lx = 200.0;
        z.rh_percent = 75.0;
        let mut v = VentSystemDocument::default();
        v.id = "vent-weak".into();
        v.sfp_w_m3_s = 2200.0;
        v.sfp_required_class = 3;
        v.heat_recovery_eta = 0.55;
        v.oda_class = "ODA3".into();
        v.filter_sup_class = "ePM10_50".into();
        v.years_since_inspection = 6;
        v.duct_leakage_m3_s_m2 = 0.30;
        v.design_airflow_m3_h = 400.0;
        z.vent_system_id = v.id.clone();
        Self {
            annex: AnnexChoice::De,
            theta_rm_c: 18.0,
            outdoor_co2_ppm: 400.0,
            zones: vec![z],
            vent_systems: vec![v],
            envelope_n50_h_inv: 4.5,
            envelope_volume_m3: 600.0,
            cellar_area_m2: 40.0,
            cellar_ventilation_m3_h: 5.0,
            night_setback_k: 1.0,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
pub fn encode_din16798_snapshot_json(snapshot: &Din16798Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

pub fn decode_din16798_snapshot_json(text: &str) -> Result<Din16798Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

pub fn decode_din16798_dsl(text: &str) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_din16798_dsl(snapshot: &Din16798Snapshot) -> String {
    <Din16798Snapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

pub fn encode_din16798_pack(snapshot: &Din16798Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

pub fn decode_din16798_pack(bytes: &[u8]) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
//#endregion 🌉️ExternalCodecBridge
