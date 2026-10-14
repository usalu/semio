//! 🧬️ Din16798 snapshot — building with zones and ventilation systems.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface;

/// 🚪 Assessed room/zone for DIN EN 16798-1 indoor environment.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ZoneDocument {
    pub id: String,
    pub name: String,
    pub usage_type: String,
    #[dsl(unit = "m2")]
    pub floor_area_m2: f64,
    pub occupants: u32,
    pub comfort_category: String,
    pub pollution_class: String,
    pub comfort_model: String,
    pub t_op_winter_c: f64,
    pub t_op_summer_c: f64,
    #[dsl(unit = "m/s")]
    pub air_speed_m_s: f64,
    pub clothing_clo: f64,
    pub metabolic_rate_met: f64,
    #[dsl(unit = "pct")]
    pub rh_percent: f64,
    pub outdoor_air_supplied_m3_h: f64,
    pub co2_ppm: f64,
    pub illuminance_lx: f64,
    pub noise_db: f64,
    pub turbulence_intensity_percent: f64,
    pub vent_method: String,
    pub vent_system_id: String,
}

impl Default for ZoneDocument {
    fn default() -> Self {
        Self {
            id: "zone-office".into(),
            name: "Open office".into(),
            usage_type: "office".into(),
            floor_area_m2: 200.0,
            occupants: 20,
            comfort_category: "II".into(),
            pollution_class: "low".into(),
            comfort_model: "fixed_hvac".into(),
            t_op_winter_c: 22.0,
            t_op_summer_c: 24.5,
            air_speed_m_s: 0.1,
            clothing_clo: 0.5,
            metabolic_rate_met: 1.2,
            rh_percent: 45.0,
            outdoor_air_supplied_m3_h: 1008.0,
            co2_ppm: 900.0,
            illuminance_lx: 500.0,
            noise_db: 34.0,
            turbulence_intensity_percent: 40.0,
            vent_method: "method_1_perceived_air_quality".into(),
            vent_system_id: "vent-central".into(),
        }
    }
}

/// 🌬️ Mechanical ventilation system serving zones (DIN EN 16798-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct VentSystemDocument {
    pub id: String,
    pub name: String,
    pub system_type: String,
    pub sfp_w_m3_s: f64,
    pub sfp_required_class: u8,
    pub heat_recovery_eta: f64,
    pub oda_class: String,
    pub filter_sup_class: String,
    pub years_since_inspection: u32,
    pub humidification_required_kg_h: f64,
    pub humidification_provided_kg_h: f64,
    pub fan_q_v_m3_s: f64,
    #[dsl(unit = "h")]
    pub fan_t_run_h: f64,
    pub duct_class: String,
    #[dsl(unit = "Pa")]
    pub duct_test_pressure_pa: f64,
    pub duct_leakage_m3_s_m2: f64,
    pub design_airflow_m3_h: f64,
}

impl Default for VentSystemDocument {
    fn default() -> Self {
        Self {
            id: "vent-central".into(),
            name: "Central AHU".into(),
            system_type: "central_mech".into(),
            sfp_w_m3_s: 950.0,
            sfp_required_class: 3,
            heat_recovery_eta: 0.75,
            oda_class: "ODA2".into(),
            filter_sup_class: "ePM1_55".into(),
            years_since_inspection: 1,
            humidification_required_kg_h: 0.0,
            humidification_provided_kg_h: 0.0,
            fan_q_v_m3_s: 1008.0 / 3600.0,
            fan_t_run_h: 2500.0,
            duct_class: "C".into(),
            duct_test_pressure_pa: 400.0,
            duct_leakage_m3_s_m2: 0.08,
            design_airflow_m3_h: 1008.0,
        }
    }
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("din16798", "DIN EN 16798", DIN16798_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const DIN16798_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.din16798", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const DIN16798_DOCUMENT_SCHEMA: &str = "semio.norm.din16798/v1";
