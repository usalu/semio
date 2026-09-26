//! ⚡️ DIN V 18599 app — document entities (constitutional: general).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_norm_contract::{app_surface, document, impl_norm_artifact_record, norm_owned_tool_job_factory, norm_results_window_config_owner, results_window_config};

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

use crate::document::ClimateZoneDe;

// #region 🔖️Types
/// 🏢️ Building use class for DIN V 18599-10 usage profiles / GEG reference area.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 🏷️ DIN V 18599-10 Nutzungsprofil for a thermal zone (typed; drives hours / outdoor-air defaults).
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UsageProfile {
    WFH,
    Office,
    School,
}

/// 🏠️ Residential vs non-residential GEG path (H′T Anlage 2 vs mean-U Anlage 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum BuildingCategory {
    Residential,
    NonResidential,
}

/// 🧱 Attachment type for GEG Anlage 2 H′T limit table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Attachment {
    Detached,
    SemiDetached,
    EndTerrace,
    MidTerrace,
}

/// 🧮 Calculation method — detailed monthly balance (-2) or tabular (-12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum CalculationMethod {
    DetailedMonthly,
    Tabular,
}

/// 🎛️ Building automation / BACS class (DIN V 18599-11).
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum AutomationClass {
    A,
    B,
    C,
    D,
}

/// 🧱 Opaque / transparent envelope element kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum ElementKind {
    Wall,
    Roof,
    Floor,
    Door,
    Window,
}

/// 🌡️ Thermal adjacency for transmission weighting factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Adjacency {
    Outdoor,
    Ground,
    Unheated,
    Heated,
}

/// 🗺️ Thermal zone with part-10 usage profile and setpoints.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct VentilationSystem {
    pub airflow_m3_h: f64,
    pub heat_recovery_eta: f64,
    pub fan_power_w: f64,
}

/// ❄️ Cooling plant parameters when installed (DIN V 18599-7).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CoolingPlant {
    pub eer: f64,
    pub energy_carrier: String,
}

/// ❄️ Cooling / AC — discriminated via optional plant (absent = no cooling system).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LightingSystem {
    pub control_factor: f64,
}

/// ☀️ On-site renewables / PV (DIN V 18599-9).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Renewables {
    pub pv_area_m2: f64,
    pub pv_efficiency: f64,
    pub solar_thermal_kwh_a: f64,
}

/// 📐️ Monthly climate data for balancing. Keeps its `dsl::DslRecord` derive — unlike the snapshot's
/// own storage (now a composed `s.stdio.semio`/`table` child, see `🔖️Composition` below),
/// `update-climate`'s mutation PAYLOAD still carries a literal `MonthlyClimate` on the wire (the
/// payload is real data, never a handle — `📓️migration-recipe.md`'s pattern), so this type still
/// needs its own `DslField` impl for `Din18599MutationDsl`'s `#[dsl(block)]`-nested encoding.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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

    pub fn german_reference(zone: ClimateZoneDe) -> Self {
        let _ = zone;
        Self::potsdam_reference()
    }
}


//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2 (orchestrator-dispatched
/// correction, `norm→C:table` on `din18599.climate`): the inline `MonthlyClimate` (two twelve-month
/// arrays) is replaced by a fixed composed `s.stdio.semio`/`table` CHILD slot — twelve rows (one per
/// calendar month), two columns (`thetaEC`/`gHWM2`). The single `update-climate` mutation triad
/// keeps its exact public payload/wire shape (`MonthlyClimate` travels on the wire as a literal
/// value, same as before — only the SNAPSHOT's own storage becomes a composed child) — only the
/// internal diff/inverse implementation is rewired to mint a fresh content-addressed child handle,
/// mirroring `➗️mathematical`'s/en1990's equivalent pattern.
//#region 🔖️ChildTypes
pub type Din18599ClimateChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: `MonthlyClimate`'s two parallel twelve-month arrays <-> `table`
/// rows — one row per calendar month (index-addressed, month = row index + 1), two columns
/// (`thetaEC: Float`, `gHWM2: Float`).
pub fn din18599_climate_table_from_data(climate: &MonthlyClimate) -> semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "thetaEC".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "gHWM2".into(), kind: SemioTableCellKind::Float }],
        rows: climate.theta_e_c.iter().zip(climate.g_h_w_m2.iter()).map(|(theta, g)| SemioTableRow { cells: vec![SemioValue::Float { lexeme: format!("{theta}") }, SemioValue::Float { lexeme: format!("{g}") }] }).collect(),
    }
}

/// 🌉 Inverse of the converter above — real reconstruction, not a stub. A short/missing row
/// degrades honestly (`0.0` for the missing month(s)) rather than panicking, since an
/// externally-composed mismatch is possible in principle.
pub fn din18599_climate_data_from_table(table: &semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot) -> MonthlyClimate {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableRow;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
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
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral representation of one exact DIN 18599 climate child. It is not serialized
/// and retires with the child owner; equal wire identities never share climate data.
#[derive(Clone, Debug)]
pub struct Din18599ClimateWorkingData {
    pub climate: MonthlyClimate,
}

fn din18599_climate_scene_id(climate: &MonthlyClimate) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = pack::json::to_json_string(climate);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("din18599-climate-{:016x}", hasher.finish())
}

fn din18599_climate_target(child_id: &str) -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: child_id.into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

/// 🏗️ Mints the composed-child handle and transfers the climate into that exact owner.
pub fn din18599_climate_child_from_data(climate: &MonthlyClimate) -> Din18599ClimateChild {
    let scene_id = din18599_climate_scene_id(climate);
    let target = din18599_climate_target(&scene_id);
    store::ArtifactChild::new(scene_id, target).with_local_owner(std::sync::Arc::new(Din18599ClimateWorkingData { climate: climate.clone() }))
}

/// 🔎 The live `MonthlyClimate` behind a snapshot's composed child — the single read call site
/// every energy-balance/compliance/inference/mutation-diff call path in this artifact now uses. A
/// wire-only child fails soft until its child document is materialized by the host.
pub fn din18599_climate(snapshot: &Din18599Snapshot) -> MonthlyClimate {
    snapshot
        .climate
        .local_owner::<Din18599ClimateWorkingData>()
        .map(|data| data.climate.clone())
        .filter(|climate| climate.g_h_w_m2.iter().any(|g| *g > 0.0))
        .unwrap_or_else(MonthlyClimate::potsdam_reference)
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

/// 📋️ Annual energy balancing inputs stored in the persisted snapshot.
pub type BalancingInputs = Din18599Snapshot;
//#endregion 🔖️Types

//#region 🔖️Subjects
/// 🏗 Realistic DIN V 18599 example subjects (compliant + non-compliant).
pub mod subjects {
    use super::*;

    fn potsdam_climate() -> Din18599ClimateChild {
        din18599_climate_child_from_data(&MonthlyClimate::potsdam_reference())
    }

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
            climate: potsdam_climate(),
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
//#endregion 🔖️Subjects

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("din18599", "DIN V 18599")
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const DIN18599_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.din18599", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const DIN18599_DOCUMENT_SCHEMA: &str = "semio.norm.din18599/v1";
//#endregion 🔖️ArtifactKind

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din18599" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din18599.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.din18599@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.din18599/v1" }, ClaimSpec { namespace: "codec-extension", value: "22:semio.norm.din18599/v1:din18599" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "DIN V 18599 energy performance of buildings" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "DIN V 18599 Energetische Bewertung von Gebäuden" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.din18599.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.schema.artifact", kind: "schema", descriptor: "s.norm.din18599", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.inference.outline", kind: "inference", descriptor: "s.norm.din18599.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.composer.any", kind: "composer", descriptor: "s.norm.din18599@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.document", kind: "grammar", descriptor: "din18599.document", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.op", kind: "grammar", descriptor: "din18599.op", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.diff", kind: "grammar", descriptor: "din18599.diff", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.pack", kind: "grammar", descriptor: "din18599.pack", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.spr", kind: "grammar", descriptor: "din18599.spr", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.codec.document.v1", kind: "codec", descriptor: "semio.norm.din18599/v1:din18599", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.localization.en", kind: "localization", descriptor: "DIN V 18599 energy performance of buildings", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.din18599.localization.de", kind: "localization", descriptor: "DIN V 18599 Energetische Bewertung von Gebäuden", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.din18599", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(artifact_schema::din18599_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::din18599_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<editor::din18599::Din18599PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "din18599.document",
                    extension: Some("din18599"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din18599.document"),
                },
                dsl::LanguageSpec {
                    id: "din18599.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din18599.op"),
                },
                dsl::LanguageSpec {
                    id: "din18599.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("din18599.diff"),
                },
                dsl::LanguageSpec {
                    id: "din18599.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din18599.pack"),
                },
                dsl::LanguageSpec {
                    id: "din18599.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("din18599.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod change_building_category {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️building-category/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️building-category/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️building-category/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_attachment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱attachment/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱attachment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱attachment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_use_class {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️use-class/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️use-class/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️use-class/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_method {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮method/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮method/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮method/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_net_floor_area_m2 {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️net-floor-area-m2/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️net-floor-area-m2/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️net-floor-area-m2/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_heated_volume_m3 {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦heated-volume-m3/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦heated-volume-m3/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦heated-volume-m3/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_geg_qp_factor {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️geg-qp-factor/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️geg-qp-factor/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️geg-qp-factor/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_delta_u_wb {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉delta-u-wb/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉delta-u-wb/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉delta-u-wb/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_automation_class {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️automation-class/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️automation-class/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️automation-class/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod specify_heating_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥specify-heating-system/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥specify-heating-system/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥specify-heating-system/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod specify_dhw_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿specify-dhw-system/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿specify-dhw-system/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿specify-dhw-system/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod update_ventilation {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️update-ventilation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️update-ventilation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️update-ventilation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod update_cooling {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️update-cooling/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️update-cooling/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️update-cooling/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod update_lighting {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡update-lighting/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡update-lighting/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡update-lighting/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod update_renewables {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️update-renewables/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️update-renewables/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️update-renewables/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_zones {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-zones/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-zones/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️replace-zones/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod replace_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩replace-elements/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩replace-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩replace-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod change_element_u {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-element-u/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-element-u/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-element-u/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod update_climate {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️update-climate/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️update-climate/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌦️update-climate/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod artifact_schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::diff::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::diff::binary::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
    }
}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
    pub mod binary {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
}
pub use crate::standards::v1::subsets::any::schema::diff::Din18599Diff;
pub use crate::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod compliant_detached {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/✅️compliant-detached/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod noncompliant_detached {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/❌️noncompliant-detached/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod compliant_two_zone {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️compliant-two-zone/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod cooled_office {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/❄️cooled-office/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod din18599 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs"]
            pub mod evaluate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☑️selected-check/🦀️.rs"]
            pub mod selected_check;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️set-snapshot/🦀️.rs"]
            pub mod set_snapshot;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️set-field/🦀️.rs"]
            pub mod set_field;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕insert-item/🦀️.rs"]
            pub mod insert_item;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖remove-item/🦀️.rs"]
            pub mod remove_item;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹apply-remedy/🦀️.rs"]
            pub mod apply_remedy;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🏷️field-meta/🦀️.rs"]
                pub mod field_meta;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️.rs"]
                    pub mod inputs;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod din18599 {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs"]
                    pub mod report;
                }
            }
        }
    }
}
