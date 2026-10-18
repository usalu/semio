//! 🧬️ Din18599 snapshot schema — complete building subject (zones, envelope, systems).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted Din18599 building energy subject. The monthly climate is parent-owned state and the composed
/// `s.stdio.semio`/`table` child `climateTable` is derived from it; envelope and zones are id-keyed lists; plant systems
/// are nested records. Derived H_T / H_V / Q_P are never stored as free inputs.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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

impl Default for Din18599Snapshot {
    fn default() -> Self {
        crate::subjects::compliant_detached_house()
    }
}
//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

/// 🏢️ Building use class for DIN V 18599-10 usage profiles / GEG reference area.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 🏷️ DIN V 18599-10 Nutzungsprofil for a thermal zone (typed; drives hours / outdoor-air defaults).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UsageProfile {
    WFH,
    Office,
    School,
}

/// 🏠️ Residential vs non-residential GEG path (H′T Anlage 2 vs mean-U Anlage 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum BuildingCategory {
    Residential,
    NonResidential,
}

/// 🧱 Attachment type for GEG Anlage 2 H′T limit table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Attachment {
    Detached,
    SemiDetached,
    EndTerrace,
    MidTerrace,
}

/// 🧮 Calculation method — detailed monthly balance (-2) or tabular (-12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum CalculationMethod {
    DetailedMonthly,
    Tabular,
}

/// 🎛️ Building automation / BACS class (DIN V 18599-11).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum AutomationClass {
    A,
    B,
    C,
    D,
}

/// 🧱 Opaque / transparent envelope element kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum ElementKind {
    Wall,
    Roof,
    Floor,
    Door,
    Window,
}

/// 🌡️ Thermal adjacency for transmission weighting factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Adjacency {
    Outdoor,
    Ground,
    Unheated,
    Heated,
}

/// 🗺️ Thermal zone with part-10 usage profile and setpoints.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ThermalZone {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub usage_profile: UsageProfile,
    pub area_m2: f64,
    pub volume_m3: f64,
    pub theta_i_heat_c: f64,
    pub theta_i_cool_c: f64,
    pub occupants: u32,
    pub internal_gains_w_m2: f64,
    pub lighting_power_w_m2: f64,
}

/// 🧱 Envelope element (area, U, orientation, g, Fc, adjacency).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct EnvelopeElement {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub kind: ElementKind,
    pub zone_id: String,
    pub area_m2: f64,
    pub u_value_w_m2k: f64,
    pub orientation_deg: f64,
    pub tilt_deg: f64,
    pub g_value: f64,
    pub fc: f64,
    pub adjacency: Adjacency,
}

/// 🔥 Heating system efficiencies and carrier (DIN V 18599-5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HeatingSystem {
    pub generation_efficiency: f64,
    pub distribution_efficiency: f64,
    pub storage_efficiency: f64,
    pub transfer_efficiency: f64,
    pub energy_carrier: String,
}

/// 🚿 Domestic hot water system (DIN V 18599-8).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DhwSystem {
    pub specific_demand_kwh_person_a: f64,
    pub storage_loss_kwh_a: f64,
    pub distribution_loss_kwh_a: f64,
    pub energy_carrier: String,
}

/// 🌬️ Ventilation with heat recovery (DIN V 18599-6/-7).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct VentilationSystem {
    pub airflow_m3_h: f64,
    pub heat_recovery_eta: f64,
    pub fan_power_w: f64,
}

/// ❄️ Cooling plant parameters when installed (DIN V 18599-7).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CoolingPlant {
    pub eer: f64,
    pub energy_carrier: String,
}

/// ❄️ Cooling / AC — discriminated via optional plant (absent = no cooling system).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CoolingSystem {
    pub plant: Option<CoolingPlant>,
}

impl CoolingSystem {
    /// 🔎 Whether a cooling plant is installed.
    pub fn is_installed(&self) -> bool {
        self.plant.is_some()
    }

    /// ❄️ EER when installed (else 1.0 sentinel unused by evaluate).
    pub fn eer(&self) -> f64 {
        self.plant.as_ref().map(|p| p.eer).unwrap_or(1.0)
    }

    /// ⛽ Carrier when installed.
    pub fn energy_carrier(&self) -> &str {
        self.plant.as_ref().map(|p| p.energy_carrier.as_str()).unwrap_or("electricity")
    }
}

/// 💡 Building-level lighting control (DIN V 18599-4); installed power lives on zones.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LightingSystem {
    pub control_factor: f64,
}

/// ☀️ On-site renewables / PV (DIN V 18599-9).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Renewables {
    pub pv_area_m2: f64,
    pub pv_efficiency: f64,
    pub solar_thermal_kwh_a: f64,
}

/// 📐️ Monthly climate data for balancing (DIN V 18599-10): parent-owned persisted state of the document and the literal
/// payload of `update-climate`; the composed `climateTable` child is derived from it (see `🔖️Composition`).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MonthlyClimate {
    pub theta_e_c: [f64; 12],
    pub g_h_w_m2: [f64; 12],
}

impl MonthlyClimate {
    /// 🌤️ Potsdam reference climate (DIN V 18599-10 / TRY-aligned monthly means).
    pub fn potsdam_reference() -> Self {
        Self {
            theta_e_c: [-0.4, 0.6, 4.1, 8.4, 13.4, 16.6, 18.4, 17.9, 14.0, 9.2, 4.4, 1.0],
            g_h_w_m2: [25.0, 50.0, 95.0, 145.0, 185.0, 200.0, 195.0, 170.0, 120.0, 70.0, 35.0, 20.0],
        }
    }

    /// 🌦️ German reference climate assessed for this family (Potsdam TRY monthly means).
    pub fn german_reference() -> Self {
        Self::potsdam_reference()
    }
}

pub type Din18599ClimateChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot>;

/// 🪆️ Child slot of the derived climate table (the camelCase name of `Din18599Snapshot::climate_table`).
pub const DIN18599_CLIMATE_TABLE_SLOT: &str = "climateTable";

/// 🌉 `MonthlyClimate`'s two parallel twelve-month arrays -> `table` rows: one row per calendar month (month = row index
/// + 1), two columns (`thetaEC: Float`, `gHWM2: Float`).
pub fn din18599_climate_table_from_data(climate: &MonthlyClimate) -> semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableCellKind;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableColumn;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableRow;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA;
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "thetaEC".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "gHWM2".into(), kind: SemioTableCellKind::Float }],
        rows: climate.theta_e_c.iter().zip(climate.g_h_w_m2.iter()).map(|(theta, g)| SemioTableRow { cells: vec![SemioValue::Float { lexeme: format!("{theta}") }, SemioValue::Float { lexeme: format!("{g}") }] }).collect(),
    }
}

/// 🔁️ Inverse of [`din18599_climate_table_from_data`] for a foreign reader of the derived child; a short or non-numeric row
/// reads `0.0` for the missing month.
pub fn din18599_climate_data_from_table(table: &semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot) -> MonthlyClimate {
    use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableRow;
    use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
    fn cell_f64(row: Option<&SemioTableRow>, index: usize) -> f64 {
        match row.and_then(|row| row.cells.get(index)) {
            Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    let mut theta_e_c = [0.0; 12];
    let mut g_h_w_m2 = [0.0; 12];
    for month in 0..12 {
        let row = table.rows.get(month);
        theta_e_c[month] = cell_f64(row, 0);
        g_h_w_m2[month] = cell_f64(row, 1);
    }
    MonthlyClimate { theta_e_c, g_h_w_m2 }
}

/// 🏷️ The derived climate table handle: `content_id("din18599-climate", canonical JSON of the climate)`, targeting
/// `s.stdio.semio@v1/table`. Equal climates name the same child on every replica.
pub fn din18599_climate_table_child(climate: &MonthlyClimate) -> Din18599ClimateChild {
    let child_id = store::content_id("din18599-climate", semio_framework_pack_json::to_json_string(climate).as_bytes());
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🌱️ Initial pack of the derived climate table the snapshot names (`ArtifactApp::genesis_child_pack`); `None` for any
/// other slot and for a handle that is not the derivation of the snapshot's own climate.
pub fn genesis_din18599_child_pack(snapshot: &Din18599Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    (slot == DIN18599_CLIMATE_TABLE_SLOT && child_id == snapshot.climate_table.child_id && child_id == din18599_climate_table_child(&snapshot.climate).child_id).then(|| din18599_climate_table_from_data(&snapshot.climate).encode_pack())
}

/// 🧬️ Bounded projection of the snapshot's composed child fields for a loaded parent.
pub fn din18599_child_restore_projection(snapshot: &Din18599Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::from(format!("din18599 child projection failed: {error}")))
}

/// 🏗 Realistic DIN V 18599 example subjects (compliant + non-compliant).
pub mod subjects {
    use crate::*;

    fn base_house(compliant: bool) -> Din18599Snapshot {
        let wall_u = if compliant { 0.20 } else { 0.48 };
        let roof_u = if compliant { 0.14 } else { 0.40 };
        let floor_u = if compliant { 0.25 } else { 0.45 };
        let window_u = if compliant { 0.95 } else { 2.70 };
        let window_g = if compliant { 0.55 } else { 0.70 };
        let gen_eff = if compliant { 0.98 } else { 0.78 };
        let pv = if compliant { 18.0 } else { 0.0 };
        let delta_u = if compliant { 0.03 } else { 0.10 };
        Din18599Snapshot {
            building_category: BuildingCategory::Residential,
            attachment: Attachment::Detached,
            use_class: UseClass::Residential,
            method: CalculationMethod::DetailedMonthly,
            net_floor_area_m2: 140.0,
            heated_volume_m3: 364.0,
            geg_qp_factor: 0.55,
            delta_u_wb_w_m2k: delta_u,
            automation_class: if compliant { AutomationClass::B } else { AutomationClass::D },
            zones: vec![ThermalZone {
                id: "zone-living".into(),
                label_en: "Living".into(),
                label_de: "Wohnen".into(),
                usage_profile: UsageProfile::WFH,
                area_m2: 140.0,
                volume_m3: 364.0,
                theta_i_heat_c: 20.0,
                theta_i_cool_c: 26.0,
                occupants: 4,
                internal_gains_w_m2: 3.5,
                lighting_power_w_m2: if compliant { 6.0 } else { 12.0 },
            }],
            elements: vec![
                EnvelopeElement { id: "wall-n".into(), label_en: "North wall".into(), label_de: "Nordwand".into(), kind: ElementKind::Wall, zone_id: "zone-living".into(), area_m2: 42.0, u_value_w_m2k: wall_u, orientation_deg: 0.0, tilt_deg: 90.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "wall-e".into(), label_en: "East wall".into(), label_de: "Ostwand".into(), kind: ElementKind::Wall, zone_id: "zone-living".into(), area_m2: 36.0, u_value_w_m2k: wall_u, orientation_deg: 90.0, tilt_deg: 90.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "wall-s".into(), label_en: "South wall".into(), label_de: "Südwand".into(), kind: ElementKind::Wall, zone_id: "zone-living".into(), area_m2: 42.0, u_value_w_m2k: wall_u, orientation_deg: 180.0, tilt_deg: 90.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "wall-w".into(), label_en: "West wall".into(), label_de: "Westwand".into(), kind: ElementKind::Wall, zone_id: "zone-living".into(), area_m2: 36.0, u_value_w_m2k: wall_u, orientation_deg: 270.0, tilt_deg: 90.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "roof".into(), label_en: "Roof".into(), label_de: "Dach".into(), kind: ElementKind::Roof, zone_id: "zone-living".into(), area_m2: 150.0, u_value_w_m2k: roof_u, orientation_deg: 180.0, tilt_deg: 35.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "floor".into(), label_en: "Ground floor".into(), label_de: "Bodenplatte".into(), kind: ElementKind::Floor, zone_id: "zone-living".into(), area_m2: 140.0, u_value_w_m2k: floor_u, orientation_deg: 0.0, tilt_deg: 0.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Ground },
                EnvelopeElement { id: "win-s".into(), label_en: "South windows".into(), label_de: "Südfenster".into(), kind: ElementKind::Window, zone_id: "zone-living".into(), area_m2: 18.0, u_value_w_m2k: window_u, orientation_deg: 180.0, tilt_deg: 90.0, g_value: window_g, fc: if compliant { 0.7 } else { 1.0 }, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "win-n".into(), label_en: "North windows".into(), label_de: "Nordfenster".into(), kind: ElementKind::Window, zone_id: "zone-living".into(), area_m2: 6.0, u_value_w_m2k: window_u, orientation_deg: 0.0, tilt_deg: 90.0, g_value: window_g, fc: 1.0, adjacency: Adjacency::Outdoor },
                EnvelopeElement { id: "door".into(), label_en: "Entrance door".into(), label_de: "Haustür".into(), kind: ElementKind::Door, zone_id: "zone-living".into(), area_m2: 2.1, u_value_w_m2k: if compliant { 1.3 } else { 3.0 }, orientation_deg: 0.0, tilt_deg: 90.0, g_value: 0.6, fc: 1.0, adjacency: Adjacency::Outdoor },
            ],
            heating: HeatingSystem {
                generation_efficiency: gen_eff,
                distribution_efficiency: if compliant { 0.96 } else { 0.90 },
                storage_efficiency: 0.98,
                transfer_efficiency: if compliant { 0.96 } else { 0.92 },
                energy_carrier: if compliant { "natural_gas".into() } else { "heating_oil".into() },
            },
            dhw: DhwSystem {
                specific_demand_kwh_person_a: 500.0,
                storage_loss_kwh_a: if compliant { 200.0 } else { 450.0 },
                distribution_loss_kwh_a: if compliant { 150.0 } else { 300.0 },
                energy_carrier: "natural_gas".into(),
            },
            ventilation: VentilationSystem {
                airflow_m3_h: 140.0,
                heat_recovery_eta: if compliant { 0.80 } else { 0.0 },
                fan_power_w: if compliant { 60.0 } else { 0.0 },
            },
            cooling: CoolingSystem { plant: None },
            lighting: LightingSystem {
                control_factor: if compliant { 0.8 } else { 1.0 },
            },
            renewables: Renewables { pv_area_m2: pv, pv_efficiency: 0.18, solar_thermal_kwh_a: if compliant { 1200.0 } else { 0.0 } },
            climate: MonthlyClimate::potsdam_reference(),
            climate_table: din18599_climate_table_child(&MonthlyClimate::potsdam_reference()),
        }
    }

    /// ✅ GEG-compliant detached house near Potsdam (realistic passing subject).
    pub fn compliant_detached_house() -> Din18599Snapshot {
        base_house(true)
    }

    /// ❌ Same geometry with thin envelope, oil boiler, no HRV/PV (multiple GEG failures).
    pub fn noncompliant_detached_house() -> Din18599Snapshot {
        base_house(false)
    }

    /// 🏢 Two-zone compliant house (living + office) for zone-assignment tests.
    pub fn compliant_two_zone_house() -> Din18599Snapshot {
        let mut doc = base_house(true);
        doc.zones = vec![
            ThermalZone {
                id: "zone-living".into(),
                label_en: "Living".into(),
                label_de: "Wohnen".into(),
                usage_profile: UsageProfile::WFH,
                area_m2: 90.0,
                volume_m3: 234.0,
                theta_i_heat_c: 20.0,
                theta_i_cool_c: 26.0,
                occupants: 3,
                internal_gains_w_m2: 3.5,
                lighting_power_w_m2: 6.0,
            },
            ThermalZone {
                id: "zone-office".into(),
                label_en: "Home office".into(),
                label_de: "Arbeitszimmer".into(),
                usage_profile: UsageProfile::Office,
                area_m2: 50.0,
                volume_m3: 130.0,
                theta_i_heat_c: 21.0,
                theta_i_cool_c: 26.0,
                occupants: 1,
                internal_gains_w_m2: 5.0,
                lighting_power_w_m2: 10.0,
            },
        ];
        for el in &mut doc.elements {
            if el.id == "win-n" || el.id == "wall-n" {
                el.zone_id = "zone-office".into();
            }
        }
        doc
    }

    /// ❄️ Non-residential office with cooling plant (scopes cooling leaves for perturbation).
    pub fn cooled_office_building() -> Din18599Snapshot {
        let mut doc = base_house(true);
        doc.use_class = UseClass::Office;
        doc.zones[0].usage_profile = UsageProfile::Office;
        doc.zones[0].lighting_power_w_m2 = 10.0;
        doc.zones[0].internal_gains_w_m2 = 6.0;
        doc.dhw.specific_demand_kwh_person_a = 100.0;
        doc.dhw.storage_loss_kwh_a = 40.0;
        doc.dhw.distribution_loss_kwh_a = 30.0;
        doc.cooling = CoolingSystem {
            plant: Some(CoolingPlant {
                eer: 3.2,
                energy_carrier: "electricity".into(),
            }),
        };
        doc.renewables.pv_area_m2 = 80.0;
        doc.zones[0].theta_i_cool_c = 26.0;
        doc
    }
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("din18599", "DIN V 18599", DIN18599_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const DIN18599_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.din18599", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const DIN18599_DOCUMENT_SCHEMA: &str = "semio.norm.din18599/v1";
