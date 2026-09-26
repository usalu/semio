//! 🧬️ Din18599 artifact schema — every field of the artifact with its state class.

use crate::{
    Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, EnvelopeElement, HeatingSystem, LightingSystem, Renewables, ThermalZone, UsageProfile, UseClass, VentilationSystem,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Din18599 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Artifact {
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
    #[state(artifact)]
    pub zones: Vec<ThermalZone>,
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
    #[child(kind = "s.stdio.semio")]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle"))]
    pub climate: Din18599ClimateChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din18599Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::Din18599Snapshot {
        crate::Din18599Snapshot {
            building_category: self.building_category,
            attachment: self.attachment,
            use_class: self.use_class,
            method: self.method,
            net_floor_area_m2: self.net_floor_area_m2,
            heated_volume_m3: self.heated_volume_m3,
            geg_qp_factor: self.geg_qp_factor,
            delta_u_wb_w_m2k: self.delta_u_wb_w_m2k,
            automation_class: self.automation_class,
            zones: self.zones.clone(),
            elements: self.elements.clone(),
            heating: self.heating.clone(),
            dhw: self.dhw.clone(),
            ventilation: self.ventilation.clone(),
            cooling: self.cooling.clone(),
            lighting: self.lighting.clone(),
            renewables: self.renewables.clone(),
            climate: self.climate.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::Din18599Snapshot) -> Self {
        Self {
            building_category: snapshot.building_category,
            attachment: snapshot.attachment,
            use_class: snapshot.use_class,
            method: snapshot.method,
            net_floor_area_m2: snapshot.net_floor_area_m2,
            heated_volume_m3: snapshot.heated_volume_m3,
            geg_qp_factor: snapshot.geg_qp_factor,
            delta_u_wb_w_m2k: snapshot.delta_u_wb_w_m2k,
            automation_class: snapshot.automation_class,
            zones: snapshot.zones,
            elements: snapshot.elements,
            heating: snapshot.heating,
            dhw: snapshot.dhw,
            ventilation: snapshot.ventilation,
            cooling: snapshot.cooling,
            lighting: snapshot.lighting,
            renewables: snapshot.renewables,
            climate: snapshot.climate,
        }
    }

    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::Din18599Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din18599` — twenty handcrafted schema leaves.
pub fn din18599_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.din18599",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{Din18599Diff, Din18599Mutation, Din18599Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Din18599BuilderConstruction {
        snapshot: Din18599Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Din18599BuilderConstruction {
        type Snapshot = Din18599Snapshot;
        type Mutation = Din18599Mutation;
        type Diff = Din18599Diff;
        fn empty() -> Self {
            Self { snapshot: Din18599Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::Din18599Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Din18599Parts {
        pub snapshot: Option<Din18599Snapshot>,
    }

    pub struct Din18599AnalyzerAnalysis;

    impl ArtifactAnalysis for Din18599AnalyzerAnalysis {
        type Parts = Din18599Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.din18599", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Din18599Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Din18599BuilderFacets {
        construction: Din18599BuilderConstruction,
        analysis: Din18599AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Din18599ComposerComposition,
    }
    builder: Din18599Builder,
    analyzer: Din18599Analyzer,
    composer: Din18599Composer,
);
//#endregion 🧬️DerivedArtifactFacets


//#region 🔖️ComplianceHelpers
use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::{Din18599Snapshot, ElementKind, MonthlyClimate};

const HOURS_PER_MONTH: f64 = DIN_V_18599_1_HOURS_PER_MONTH;
const RHO_CA: f64 = DIN_V_18599_1_RHO_CA;

/// ⚡️ GEG Annex 4 / DIN V 18599-10 primary energy factors f_p (non-renewable).
pub fn primary_energy_factor(carrier: &str) -> f64 {
    use geg_anlage4_primary_energy_factors as f;
    match carrier {
        "natural_gas" | "gas" => f::NATURAL_GAS,
        "heating_oil" | "oil" => f::HEATING_OIL,
        "electricity" | "heat_pump_electric" => f::ELECTRICITY_GRID,
        "district_heating" => f::DISTRICT_HEATING,
        "biomass" | "wood" => f::BIOMASS,
        _ => f::NATURAL_GAS,
    }
}

/// 🎛️ DIN V 18599-11 automation / BACS factor on final energy.
pub fn automation_factor(class: AutomationClass) -> f64 {
    use din_v_18599_11_automation_factor as f;
    match class {
        AutomationClass::A => f::CLASS_A,
        AutomationClass::B => f::CLASS_B,
        AutomationClass::C => f::CLASS_C,
        AutomationClass::D => f::CLASS_D,
    }
}

fn adjacency_factor(a: Adjacency) -> f64 {
    use din_v_18599_2_adjacency_fx as f;
    match a {
        Adjacency::Outdoor => f::OUTDOOR,
        Adjacency::Ground => f::GROUND,
        Adjacency::Unheated => f::UNHEATED,
        Adjacency::Heated => f::HEATED,
    }
}

/// 🧱 Transmission heat transfer coefficient H_T [W/K] from envelope + ΔU_WB surcharge.
pub fn transmission_loss_coefficient(doc: &Din18599Snapshot) -> f64 {
    let sum_ua: f64 = doc.elements.iter().map(|e| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2).sum();
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum();
    sum_ua + doc.delta_u_wb_w_m2k * a_env
}

/// 🌬️ Ventilation heat transfer coefficient H_V [W/K] (DIN V 18599-6).
pub fn ventilation_loss_coefficient(doc: &Din18599Snapshot) -> f64 {
    zone_balances(doc, &doc.elements, doc.delta_u_wb_w_m2k).iter().map(|z| z.h_v).sum()
}

/// 🏠️ Specific transmission heat loss H′T [W/(m²·K)] — H_T / A with A = heat-transferring envelope area (GEG).
pub fn h_t_prime(doc: &Din18599Snapshot) -> f64 {
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    transmission_loss_coefficient(doc) / a_env
}

/// 📏 GEG §16 / Anlage 2 H′T limit for residential by attachment and A_N.
pub fn geg_ht_prime_limit(attachment: Attachment, a_n: f64) -> f64 {
    use geg_anlage2_ht_prime_limits as lim;
    let small = a_n <= lim::AN_THRESHOLD_M2;
    match attachment {
        Attachment::Detached => if small { lim::DETACHED_AN_LE_350 } else { lim::DETACHED_AN_GT_350 },
        Attachment::SemiDetached | Attachment::EndTerrace => if small { lim::SEMI_OR_END_AN_LE_350 } else { lim::SEMI_OR_END_AN_GT_350 },
        Attachment::MidTerrace => lim::MID_TERRACE,
    }
}

/// 🧱 GEG Anlage 2 reference U-values by element kind.
pub fn reference_u(kind: ElementKind) -> f64 {
    use geg_anlage2_reference_u as u;
    match kind {
        ElementKind::Wall => u::WALL,
        ElementKind::Roof => u::ROOF,
        ElementKind::Floor => u::FLOOR,
        ElementKind::Window => u::WINDOW,
        ElementKind::Door => u::DOOR,
    }
}

/// 📊 Mean U for non-residential GEG Anlage 3 (by element kind groups).
pub fn mean_u_for_kind(doc: &Din18599Snapshot, kind: ElementKind) -> Option<(f64, f64)> {
    let (area, ua) = doc
        .elements
        .iter()
        .filter(|e| e.kind == kind)
        .fold((0.0, 0.0), |(a, ua), e| (a + e.area_m2, ua + e.u_value_w_m2k * e.area_m2));
    if area <= 0.0 {
        None
    } else {
        let limit = match kind {
            ElementKind::Wall => geg_anlage3_mean_u::WALL,
            ElementKind::Roof => geg_anlage3_mean_u::ROOF,
            ElementKind::Floor => geg_anlage3_mean_u::FLOOR,
            ElementKind::Window => geg_anlage3_mean_u::WINDOW,
            ElementKind::Door => geg_anlage3_mean_u::DOOR,
        };
        Some((ua / area, limit))
    }
}

/// 🏷️ DIN V 18599-10 profile row for a zone usage profile.
pub struct UsageProfileRow {
    pub fan_hours_a: f64,
    pub lighting_hours_a: f64,
    pub outdoor_air_change_1_h: f64,
    pub lighting_lpd_limit_w_m2: f64,
}

/// 📋 Resolve DIN V 18599-10 defaults for a typed zone usage profile.
pub fn usage_profile_row(profile: UsageProfile) -> UsageProfileRow {
    match profile {
        UsageProfile::WFH => UsageProfileRow {
            fan_hours_a: din_v_18599_10_fan_hours::RESIDENTIAL_WFH,
            lighting_hours_a: din_v_18599_10_lighting_hours::WFH,
            outdoor_air_change_1_h: din_v_18599_10_outdoor_air_change::WFH,
            lighting_lpd_limit_w_m2: din_v_18599_4_lighting_power_density::RESIDENTIAL,
        },
        UsageProfile::Office => UsageProfileRow {
            fan_hours_a: din_v_18599_10_fan_hours::OFFICE,
            lighting_hours_a: din_v_18599_10_lighting_hours::OFFICE,
            outdoor_air_change_1_h: din_v_18599_10_outdoor_air_change::OFFICE,
            lighting_lpd_limit_w_m2: din_v_18599_4_lighting_power_density::OFFICE,
        },
        UsageProfile::School => UsageProfileRow {
            fan_hours_a: din_v_18599_10_fan_hours::SCHOOL,
            lighting_hours_a: din_v_18599_10_lighting_hours::SCHOOL,
            outdoor_air_change_1_h: din_v_18599_10_outdoor_air_change::SCHOOL,
            lighting_lpd_limit_w_m2: din_v_18599_4_lighting_power_density::SCHOOL,
        },
    }
}

fn occupants(doc: &Din18599Snapshot) -> u32 {
    doc.zones.iter().map(|z| z.occupants).sum()
}

fn elements_for_zone<'a>(elements: &'a [EnvelopeElement], zone_id: &str) -> Vec<&'a EnvelopeElement> {
    elements.iter().filter(|e| e.zone_id == zone_id).collect()
}

/// 🧮 Per-zone monthly balance (DIN V 18599-1/-2).
#[derive(Clone, Debug)]
pub struct ZoneBalance {
    pub zone_id: String,
    pub h_t: f64,
    pub h_v: f64,
    pub q_h_nd_kwh: f64,
    pub q_c_nd_kwh: f64,
    pub q_l_kwh: f64,
    pub area_m2: f64,
    pub volume_m3: f64,
}

fn zone_transmission(elements: &[&EnvelopeElement], delta_u: f64, volume_share: f64, a_env_total: f64) -> f64 {
    let sum_ua: f64 = elements.iter().map(|e| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2).sum();
    sum_ua + delta_u * a_env_total * volume_share
}

fn zone_ventilation_hv(zone: &ThermalZone, doc: &Din18599Snapshot, volume_share: f64) -> f64 {
    let row = usage_profile_row(zone.usage_profile);
    let v_mech_zone = doc.ventilation.airflow_m3_h * volume_share;
    let v_profile = row.outdoor_air_change_1_h * zone.volume_m3;
    let v_mech = v_mech_zone + v_profile;
    let v_inf = DIN_V_18599_2_INFILTRATION_N_INF * doc.heated_volume_m3.max(1e-9) * volume_share;
    let eta = doc.ventilation.heat_recovery_eta.clamp(0.0, 1.0);
    RHO_CA * (v_mech * (1.0 - eta) + v_inf)
}

/// 🧮 Zone-level balances; building totals aggregate these.
pub fn zone_balances(doc: &Din18599Snapshot, elements: &[EnvelopeElement], delta_u: f64) -> Vec<ZoneBalance> {
    let climate = crate::din18599_climate(doc);
    let a_env_total: f64 = elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    let v_net: f64 = doc.zones.iter().map(|z| z.volume_m3).sum::<f64>().max(1e-9);
    let mut out = Vec::with_capacity(doc.zones.len());
    for zone in &doc.zones {
        let z_elems = elements_for_zone(elements, &zone.id);
        let share = zone.volume_m3 / v_net;
        let h_t = zone_transmission(&z_elems, delta_u, share, a_env_total);
        let h_v = zone_ventilation_hv(zone, doc, share);
        let theta_i = zone.theta_i_heat_c;
        let theta_c = zone.theta_i_cool_c;
        let qi_w = zone.internal_gains_w_m2;
        let row = usage_profile_row(zone.usage_profile);
        let mut q_h = 0.0;
        let mut q_c = 0.0;
        for m in 0..12 {
            let te = climate.theta_e_c[m];
            let qt = h_t * (theta_i - te).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qv = h_v * (theta_i - te).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qi = qi_w * zone.area_m2 * HOURS_PER_MONTH / 1000.0;
            let mut qs = 0.0;
            let g_h = climate.g_h_w_m2[m];
            for e in z_elems.iter().filter(|e| e.g_value > 0.0) {
                qs += g_h * orientation_solar_factor(e.orientation_deg, e.tilt_deg) * e.area_m2 * e.g_value * e.fc.clamp(0.0, 1.0) * HOURS_PER_MONTH / 1000.0;
            }
            let loss = qt + qv;
            let gain = qi + qs;
            let gamma = if loss <= 1e-9 { 1e9 } else { gain / loss };
            let eta_u = utilization_factor(gamma);
            q_h += (loss - eta_u * gain).max(0.0);
            let loss_c = (h_t + h_v) * (theta_c - te).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qt_c = h_t * (te - theta_c).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let qv_c = h_v * (te - theta_c).max(0.0) * HOURS_PER_MONTH / 1000.0;
            let gamma_c = if loss_c <= 1e-9 { 1e9 } else { (qi + qs) / loss_c };
            let eta_c = utilization_factor(gamma_c);
            q_c += ((qi + qs) - eta_c * loss_c).max(0.0) + qt_c + qv_c;
        }
        let q_l = zone.lighting_power_w_m2 * zone.area_m2 * row.lighting_hours_a * doc.lighting.control_factor / 1000.0;
        out.push(ZoneBalance {
            zone_id: zone.id.clone(),
            h_t,
            h_v,
            q_h_nd_kwh: q_h,
            q_c_nd_kwh: q_c,
            q_l_kwh: q_l,
            area_m2: zone.area_m2,
            volume_m3: zone.volume_m3,
        });
    }
    out
}

fn orientation_solar_factor(orientation_deg: f64, tilt_deg: f64) -> f64 {
    use din_v_18599_2_solar_geometry_factors as s;
    let az = orientation_deg.rem_euclid(360.0);
    let from_south = (az - 180.0).abs().min(360.0 - (az - 180.0).abs());
    let horiz = s::NORTH_HORIZ + (s::SOUTH_HORIZ - s::NORTH_HORIZ) * (1.0 - from_south / 180.0);
    let t = tilt_deg.clamp(0.0, 90.0) / 90.0;
    let tilt = s::LOW_TILT + (s::STEEP_TILT - s::LOW_TILT) * t;
    horiz * tilt
}

/// ☀️ Monthly solar gains Q_S,m [kWh] from fenestration (DIN V 18599-2).
pub fn solar_gains_month_kwh(doc: &Din18599Snapshot, climate: &MonthlyClimate, month: usize) -> f64 {
    let irr = climate.g_h_w_m2[month];
    doc.elements
        .iter()
        .filter(|e| e.g_value > 0.0)
        .map(|e| irr * orientation_solar_factor(e.orientation_deg, e.tilt_deg) * e.area_m2 * e.g_value * e.fc.clamp(0.0, 1.0) * HOURS_PER_MONTH / 1000.0)
        .sum()
}

fn utilization_factor(gamma: f64) -> f64 {
    let a = DIN_V_18599_2_UTILIZATION_A;
    if gamma.abs() < 1e-9 {
        1.0
    } else if (gamma - 1.0).abs() < 1e-9 {
        a / (a + 1.0)
    } else {
        (1.0 - gamma.powf(-a)) / (1.0 - gamma.powf(-(a + 1.0)))
    }
}

/// 🧮 Derived monthly / annual balance quantities.
#[derive(Clone, Debug)]
pub struct BalanceDerived {
    pub h_t: f64,
    pub h_v: f64,
    pub h_t_prime: f64,
    pub a_over_v_e: f64,
    pub q_h_nd_kwh: f64,
    pub q_c_nd_kwh: f64,
    pub q_w_kwh: f64,
    pub q_l_kwh: f64,
    pub q_f_heat_kwh: f64,
    pub q_f_dhw_kwh: f64,
    pub q_f_cool_kwh: f64,
    pub q_f_light_kwh: f64,
    pub q_f_aux_kwh: f64,
    pub q_pv_kwh: f64,
    pub q_p_kwh: f64,
    pub q_p_ref_kwh: f64,
    pub q_p_limit_kwh: f64,
    pub q_h_ref_kwh: f64,
}

fn heating_system_efficiency(doc: &Din18599Snapshot) -> f64 {
    (doc.heating.generation_efficiency * doc.heating.distribution_efficiency * doc.heating.storage_efficiency * doc.heating.transfer_efficiency).max(DIN_V_18599_5_ETA_SYS_FLOOR)
}

//#region 📜️NormTables
/// ⛽ GEG Anlage 4 — Primärenergiefaktoren f_P [-].
pub mod geg_anlage4_primary_energy_factors {
    pub const NATURAL_GAS: f64 = 1.1;
    pub const HEATING_OIL: f64 = 1.1;
    pub const ELECTRICITY_GRID: f64 = 1.8;
    pub const DISTRICT_HEATING: f64 = 0.7;
    pub const BIOMASS: f64 = 0.2;
}

/// 🧱 GEG Anlage 2 — Höchstwerte der Wärmedurchgangskoeffizienten U_ref [W/(m²·K)] (Wohngebäude, Neubau).
pub mod geg_anlage2_reference_u {
    pub const WALL: f64 = 0.28;
    pub const ROOF: f64 = 0.20;
    pub const FLOOR: f64 = 0.35;
    pub const WINDOW: f64 = 1.3;
    pub const DOOR: f64 = 1.8;
}

/// 🧱 GEG Anlage 3 — mittlere U-Werte Nichtwohngebäude [W/(m²·K)].
pub mod geg_anlage3_mean_u {
    pub const WALL: f64 = 0.28;
    pub const ROOF: f64 = 0.20;
    pub const FLOOR: f64 = 0.35;
    pub const WINDOW: f64 = 1.5;
    pub const DOOR: f64 = 1.8;
}

/// 🏗️ GEG Anlage 1 Tabelle 1 — Referenzausführung Lüftung mit WRG η_WRG [-].
pub const GEG_ANLAGE1_TABELLE1_ETA_WRG_REF: f64 = 0.80;

/// 🌡️ GEG Anlage 1 / Anlage 2 — Wärmebrückenzuschlag der Referenzausführung ΔU_WB [W/(m²·K)].
pub const GEG_ANLAGE1_DELTA_U_WB_REF: f64 = 0.05;

/// 🔥 GEG Anlage 1 Tabelle 1 — Teillastwirkungsgrade der Referenz-Wärmeerzeugung/-verteilung/-speicherung/-übergabe [-].
pub mod geg_anlage1_tabelle1_heating_eta {
    pub const GENERATION: f64 = 0.95;
    pub const DISTRIBUTION: f64 = 0.95;
    pub const STORAGE: f64 = 0.98;
    pub const TRANSFER: f64 = 0.95;
    pub const SYSTEM_PRODUCT: f64 = GENERATION * DISTRIBUTION * STORAGE * TRANSFER;
}

/// 🪟 GEG Anlage 1 — Referenz-g-Wert Fenster [-].
pub const GEG_ANLAGE1_WINDOW_G_REF: f64 = 0.60;

/// ⏱️ DIN V 18599-10 Nutzungsprofile — jährliche Betriebsstunden mechanischer Lüftung [h/a].
pub mod din_v_18599_10_fan_hours {
    pub const RESIDENTIAL_WFH: f64 = 5000.0;
    pub const OFFICE: f64 = 2500.0;
    pub const SCHOOL: f64 = 2000.0;
}

/// 💡 DIN V 18599-4 — spezifische elektrische Bewertungsleistung p_LX [W/m²] (Grenzwerte je Nutzungsprofil).
pub mod din_v_18599_4_lighting_power_density {
    pub const RESIDENTIAL: f64 = 10.0;
    pub const OFFICE: f64 = 12.0;
    pub const SCHOOL: f64 = 15.0;
}

/// 🚿 DIN V 18599-10 — Nutzwärmebedarf Trinkwarmwasser q_w [kWh/(Person·a)].
pub mod din_v_18599_10_dhw_specific {
    pub const RESIDENTIAL: f64 = 500.0;
    pub const OFFICE: f64 = 100.0;
    pub const SCHOOL: f64 = 150.0;
}

/// 🚿 DIN V 18599-8 — relative Verlustzuschläge auf die Nutzenergie (Speicher / Verteilung).
pub mod din_v_18599_8_dhw_loss_fractions {
    pub const STORAGE_OF_USEFUL: f64 = 0.25;
    pub const DISTRIBUTION_OF_USEFUL: f64 = 0.20;
    pub const STORAGE_FLOOR_KWH_A: f64 = 50.0;
    pub const DISTRIBUTION_FLOOR_KWH_A: f64 = 30.0;
}

/// 📊 DIN V 18599-12:2018 Tabelle 5 — spezifische Primärenergiekennwerte q_p,tab [kWh/(m²·a)] (Tabellenverfahren, vor f_BAC).
pub mod din_v_18599_12_tabelle5_qp_specific {
    pub const RESIDENTIAL: f64 = 66.0;
    pub const OFFICE: f64 = 100.0;
    pub const SCHOOL: f64 = 85.0;
}

/// ⚙️ DIN V 18599-5 — Mindest-Gesamtaufwandzahl / Erzeugernutzungsgrad-Produkt η_sys,min [-].
pub const DIN_V_18599_5_ETA_SYS_MIN: f64 = 0.85;

/// 🧊 DIN V 18599-7 — Kühlbedarf-Grenze als Anteil am Heizwärmebedarf [-].
pub const DIN_V_18599_7_COOLING_TO_HEATING_RATIO: f64 = 0.5;

/// 🪟 DIN V 18599-7 — Abminderung g-Wert bei Kühlüberschreitung (Multiplikator / Untergrenze).
pub mod din_v_18599_7_g_remedy {
    pub const REDUCE_FACTOR: f64 = 0.7;
    pub const FLOOR: f64 = 0.3;
}

/// 📐 GEG Anlage 2 — Höchstwerte H′T [W/(m²·K)] nach Gebäudegröße und Anbauart.
pub mod geg_anlage2_ht_prime_limits {
    pub const DETACHED_AN_LE_350: f64 = 0.40;
    pub const DETACHED_AN_GT_350: f64 = 0.50;
    pub const SEMI_OR_END_AN_LE_350: f64 = 0.45;
    pub const SEMI_OR_END_AN_GT_350: f64 = 0.50;
    pub const MID_TERRACE: f64 = 0.50;
    pub const AN_THRESHOLD_M2: f64 = 350.0;
}

/// 🎛️ DIN V 18599-11 — Korrekturfaktoren f_BAC auf den Endenergiebedarf nach Automationsklasse.
pub mod din_v_18599_11_automation_factor {
    pub const CLASS_A: f64 = 0.88;
    pub const CLASS_B: f64 = 0.93;
    pub const CLASS_C: f64 = 0.97;
    pub const CLASS_D: f64 = 1.00;
}

/// 🧱 DIN V 18599-2 — Temperaturkorrekturfaktoren F_x für angrenzende Bereiche [-].
pub mod din_v_18599_2_adjacency_fx {
    pub const OUTDOOR: f64 = 1.0;
    pub const GROUND: f64 = 0.6;
    pub const UNHEATED: f64 = 0.5;
    pub const HEATED: f64 = 0.0;
}

/// 🌡️ DIN V 18599-10 — Standard-Raumtemperaturen / innere Gewinne wenn keine Zone vorliegt.
pub mod din_v_18599_10_zone_defaults {
    pub const THETA_I_HEAT_C: f64 = 20.0;
    pub const THETA_I_COOL_C: f64 = 26.0;
    pub const INTERNAL_GAINS_W_M2: f64 = 3.5;
}

/// ☀️ DIN V 18599-2 — Orientierung-/Neigungsfaktoren für solare Einstrahlung (Monatsbilanz).
pub mod din_v_18599_2_solar_geometry_factors {
    pub const SOUTH_HORIZ: f64 = 1.0;
    pub const EAST_WEST_HORIZ: f64 = 0.7;
    pub const NORTH_HORIZ: f64 = 0.4;
    pub const LOW_TILT: f64 = 0.9;
    pub const STEEP_TILT: f64 = 1.0;
    pub const MID_TILT: f64 = 0.95;
}

/// 📊 DIN V 18599-2 — Ausnutzungsgrad der Gewinne (Zeitkonstante-Parameter a).
pub const DIN_V_18599_2_UTILIZATION_A: f64 = 0.95;

/// 🌬️ DIN V 18599-1 — ρ·c_a Volumenstrom-Kennwert [Wh/(m³·K)] ≡ [W/(m³/h·K)].
pub const DIN_V_18599_1_RHO_CA: f64 = 0.34;

/// ⏱️ Mittlere Monatsstunden für die Monatsbilanz [h].
pub const DIN_V_18599_1_HOURS_PER_MONTH: f64 = 730.0;

/// 🔥 Numerische Untergrenze für η_sys (Schutz vor Division durch 0) [-].
pub const DIN_V_18599_5_ETA_SYS_FLOOR: f64 = 0.05;

/// ⚖️ GEG §10 — Primärenergie-Grenzwertfaktor auf Q_P,Ref (typisch 0,55 für Wohngebäude-Neubau).
pub const GEG_SECTION10_QP_FACTOR_DEFAULT: f64 = 0.55;

/// 💡 DIN V 18599-10 — jährliche Beleuchtungsvollbenutzungsstunden t_L [h/a] je Nutzungsprofil.
pub mod din_v_18599_10_lighting_hours {
    pub const WFH: f64 = 1700.0;
    pub const OFFICE: f64 = 2500.0;
    pub const SCHOOL: f64 = 1800.0;
}

/// 🌬️ DIN V 18599-10 — Außenluftwechsel n_Außen [1/h] bezogen auf Nettovolumen.
pub mod din_v_18599_10_outdoor_air_change {
    pub const WFH: f64 = 0.5;
    pub const OFFICE: f64 = 1.0;
    pub const SCHOOL: f64 = 1.5;
}

/// 📐 DIN V 18599-1 §6.1.1 — Nettoluftvolumen / Bruttovolumen V/V_e (Standardfaktor).
pub const DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO: f64 = 0.80;

/// 🌬️ DIN V 18599-2 — Infiltrationsluftwechsel n_inf bezogen auf V_e [1/h].
pub const DIN_V_18599_2_INFILTRATION_N_INF: f64 = 0.2;
//#endregion 📜️NormTables

/// 🚿 DIN V 18599-10 Nutzenergie Trinkwarmwasser [kWh/(Person·a)] by use class.
pub fn dhw_profile_specific_kwh_person_a(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential => din_v_18599_10_dhw_specific::RESIDENTIAL,
        UseClass::Office => din_v_18599_10_dhw_specific::OFFICE,
        UseClass::School => din_v_18599_10_dhw_specific::SCHOOL,
    }
}

/// 🚿 DIN V 18599-8 Q_W limit: profile Nutzenergie + loss allowances scaled to that Nutzenergie.
pub fn dhw_limit_kwh_a(doc: &Din18599Snapshot) -> f64 {
    let n = occupants(doc) as f64;
    let q_nd = n * dhw_profile_specific_kwh_person_a(doc.use_class);
    let storage_cap = (din_v_18599_8_dhw_loss_fractions::STORAGE_OF_USEFUL * q_nd).max(din_v_18599_8_dhw_loss_fractions::STORAGE_FLOOR_KWH_A);
    let dist_cap = (din_v_18599_8_dhw_loss_fractions::DISTRIBUTION_OF_USEFUL * q_nd).max(din_v_18599_8_dhw_loss_fractions::DISTRIBUTION_FLOOR_KWH_A);
    q_nd + storage_cap + dist_cap
}

fn element_u_path(e: &EnvelopeElement) -> String {
    format!("elements[id={}].uValueWM2k", e.id)
}

fn element_g_path(e: &EnvelopeElement) -> String {
    format!("elements[id={}].gValue", e.id)
}


fn fan_operating_hours_weighted(doc: &Din18599Snapshot) -> f64 {
    let v: f64 = doc.zones.iter().map(|z| z.volume_m3).sum::<f64>().max(1e-9);
    doc.zones.iter().map(|z| usage_profile_row(z.usage_profile).fan_hours_a * z.volume_m3 / v).sum()
}

fn balance_for(doc: &Din18599Snapshot, elements: &[EnvelopeElement], delta_u: f64, heat_eff: f64, carrier: &str, pv_area: f64) -> BalanceDerived {
    let climate = crate::din18599_climate(doc);
    let zones = zone_balances(doc, elements, delta_u);
    let h_t: f64 = zones.iter().map(|z| z.h_t).sum();
    let h_v: f64 = zones.iter().map(|z| z.h_v).sum();
    let q_h: f64 = zones.iter().map(|z| z.q_h_nd_kwh).sum();
    let q_c: f64 = zones.iter().map(|z| z.q_c_nd_kwh).sum();
    let q_l: f64 = zones.iter().map(|z| z.q_l_kwh).sum();
    let a_env: f64 = elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    let v_e = doc.heated_volume_m3.max(1e-9);
    let q_w = occupants(doc) as f64 * doc.dhw.specific_demand_kwh_person_a + doc.dhw.storage_loss_kwh_a + doc.dhw.distribution_loss_kwh_a;
    let auto = automation_factor(doc.automation_class);
    let q_f_heat = q_h / heat_eff * auto;
    let q_f_dhw = q_w / heat_eff.max(0.5) * auto;
    let q_f_cool = if doc.cooling.is_installed() {
        q_c / doc.cooling.eer().max(0.5) * auto
    } else {
        0.0
    };
    let q_f_light = q_l * auto;
    let q_f_aux = doc.ventilation.fan_power_w * fan_operating_hours_weighted(doc) / 1000.0;
    let annual_g = climate.g_h_w_m2.iter().sum::<f64>() / 12.0 * 8760.0 / 1000.0;
    let q_pv = pv_area * doc.renewables.pv_efficiency * annual_g + doc.renewables.solar_thermal_kwh_a;
    let q_p = q_f_heat * primary_energy_factor(carrier)
        + q_f_dhw * primary_energy_factor(&doc.dhw.energy_carrier)
        + q_f_cool * primary_energy_factor(doc.cooling.energy_carrier())
        + (q_f_light + q_f_aux) * primary_energy_factor("electricity")
        - q_pv * primary_energy_factor("electricity");
    BalanceDerived {
        h_t,
        h_v,
        h_t_prime: h_t / a_env,
        a_over_v_e: a_env / v_e,
        q_h_nd_kwh: q_h,
        q_c_nd_kwh: q_c,
        q_w_kwh: q_w,
        q_l_kwh: q_l,
        q_f_heat_kwh: q_f_heat,
        q_f_dhw_kwh: q_f_dhw,
        q_f_cool_kwh: q_f_cool,
        q_f_light_kwh: q_f_light,
        q_f_aux_kwh: q_f_aux,
        q_pv_kwh: q_pv,
        q_p_kwh: q_p,
        q_p_ref_kwh: 0.0,
        q_p_limit_kwh: 0.0,
        q_h_ref_kwh: 0.0,
    }
}

/// 🧮 Full derived balance for the actual building including GEG reference run.
pub fn derive_balance(doc: &Din18599Snapshot) -> BalanceDerived {
    let mut actual = balance_for(doc, &doc.elements, doc.delta_u_wb_w_m2k, heating_system_efficiency(doc), &doc.heating.energy_carrier, doc.renewables.pv_area_m2);
    let ref_elements: Vec<EnvelopeElement> = doc
        .elements
        .iter()
        .map(|e| {
            let mut r = e.clone();
            r.u_value_w_m2k = reference_u(e.kind);
            if e.kind == ElementKind::Window {
                r.g_value = GEG_ANLAGE1_WINDOW_G_REF;
                r.fc = 1.0;
            }
            r
        })
        .collect();
    let reference = balance_for(doc, &ref_elements, GEG_ANLAGE1_DELTA_U_WB_REF, geg_anlage1_tabelle1_heating_eta::SYSTEM_PRODUCT, "natural_gas", 0.0);
    actual.q_p_ref_kwh = reference.q_p_kwh;
    actual.q_p_limit_kwh = doc.geg_qp_factor * reference.q_p_kwh;
    actual.q_h_ref_kwh = reference.q_h_nd_kwh;
    actual
}

/// 📊 DIN V 18599-12:2018 Tabelle 5 — specific primary energy [kWh/(m²·a)] before A_N and f_BAC.
pub fn tabular_specific_primary_energy_kwh_m2(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential => din_v_18599_12_tabelle5_qp_specific::RESIDENTIAL,
        UseClass::Office => din_v_18599_12_tabelle5_qp_specific::OFFICE,
        UseClass::School => din_v_18599_12_tabelle5_qp_specific::SCHOOL,
    }
}

fn energy(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Energy, v)
}
fn heat_transfer(v: f64) -> Quantity {
    Quantity::new(QuantityKind::HeatTransferCoefficient, v)
}
fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

fn whole_building() -> SubjectRef {
    SubjectRef::whole(LocalizedCopy::new("Building", "Gebäude"))
}

fn element_ref(e: &EnvelopeElement, path: &str) -> SubjectRef {
    SubjectRef::new(&e.id, path, LocalizedCopy::new(e.label_en.clone(), e.label_de.clone()))
}

fn worst_element<'a>(doc: &'a Din18599Snapshot) -> Option<(usize, &'a EnvelopeElement)> {
    doc.elements
        .iter()
        .enumerate()
        .filter(|(_, e)| matches!(e.kind, ElementKind::Wall | ElementKind::Roof | ElementKind::Floor) && e.adjacency != Adjacency::Heated)
        .max_by(|(_, a), (_, b)| a.u_value_w_m2k.partial_cmp(&b.u_value_w_m2k).unwrap_or(std::cmp::Ordering::Equal))
}

/// 🔎 Required U on worst element so H′T ≤ limit (analytic on that element's contribution).
pub fn required_u_for_ht_prime(doc: &Din18599Snapshot, limit: f64) -> Option<(usize, f64)> {
    let (idx, el) = worst_element(doc)?;
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum::<f64>().max(1e-9);
    let h_t_max = limit * a_env;
    let f = adjacency_factor(el.adjacency);
    let others: f64 = doc
        .elements
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != idx)
        .map(|(_, e)| adjacency_factor(e.adjacency) * e.u_value_w_m2k * e.area_m2)
        .sum();
    let a_env: f64 = doc.elements.iter().map(|e| e.area_m2).sum();
    let bridges = doc.delta_u_wb_w_m2k * a_env;
    let need_ua = (h_t_max - others - bridges).max(0.0);
    let u_req = if f * el.area_m2 <= 1e-9 { 0.0 } else { need_ua / (f * el.area_m2) };
    Some((idx, u_req))
}

fn copy(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

//#endregion 🔖️ComplianceHelpers

//#region 🔖️Checks
/// ⚖️ Evaluate DIN V 18599 / GEG compliance for the building subject.
pub fn evaluate_document(doc: &Din18599Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let derived = derive_balance(doc);
    let annex = AnnexChoice::De;

    // --- DIN V 18599-1 heated gross volume V_e plausibility vs Σ zone net volumes ---
    {
        let v_e = doc.heated_volume_m3;
        let v_net: f64 = doc.zones.iter().map(|z| z.volume_m3).sum();
        let v_net_min = DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO * v_e;
        let mut b = CheckResult::assess(
            "din18599.1.heated-volume",
            "DIN V 18599-1",
            ClauseId::new("DIN V 18599", "1", "6.1.1"),
            SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
            copy("Heated volume V_e vs zone net volumes", "Bruttovolumen V_e gegenüber Nettovolumina"),
        )
        .annex(annex)
        .explanation(copy(
            &format!("V_e = {v_e:.1} m³; ΣV_zone = {v_net:.1} m³; A/V_e = {:.3} m⁻¹; DIN V 18599-1 expects V ≈ 0.80·V_e.", derived.a_over_v_e),
            &format!("V_e = {v_e:.1} m³; ΣV_zone = {v_net:.1} m³; A/V_e = {:.3} m⁻¹; DIN V 18599-1 erwartet V ≈ 0,80·V_e.", derived.a_over_v_e),
        ));
        if v_net > v_e + 1e-6 {
            b = b
                .utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_e))
                .remedy(Remedy::at_least(
                    SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
                    Quantity::new(QuantityKind::Volume, v_e),
                    Quantity::new(QuantityKind::Volume, v_net),
                    copy(
                        &format!("Raise V_e to at least {v_net:.1} m³ so gross volume covers Σ zone net volumes."),
                        &format!("V_e auf mindestens {v_net:.1} m³ anheben, damit das Bruttovolumen Σ Nettovolumina abdeckt."),
                    ),
                ));
        } else if v_net + 1e-6 < v_net_min {
            b = b
                .utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_net_min))
                .remedy(Remedy::at_most(
                    SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
                    Quantity::new(QuantityKind::Volume, v_e),
                    Quantity::new(QuantityKind::Volume, v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                    copy(
                        &format!("Lower V_e toward {:.1} m³ so ΣV_zone / V_e ≥ 0.80 (DIN V 18599-1).", v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                        &format!("V_e Richtung {:.1} m³ senken, damit ΣV_zone / V_e ≥ 0,80 (DIN V 18599-1).", v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                    ),
                ));
        } else {
            b = b.utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_e));
        }
        report.push(b.build());
    }

    // --- Climate composition handle (DIN V 18599-10 TRY / reference climate child) ---
    {
        let child_id = doc.climate.child_id.as_str();
        let target = &doc.climate.target;
        let dialect_ok = target.dialect.artifact_kind == "s.stdio.semio"
            && target.dialect.standard == "v1"
            && target.dialect.subset == "table";
        let id_ok = !child_id.is_empty() && child_id == target.artifact_id.as_str();
        let potsdam = crate::din18599_climate_child_from_data(&crate::MonthlyClimate::potsdam_reference());
        let mut valid_handles = vec![potsdam.child_id.clone()];
        if id_ok && dialect_ok && !valid_handles.iter().any(|h| h == child_id) {
            valid_handles.push(child_id.to_string());
        }
        let ok = id_ok && dialect_ok;
        let mut b = CheckResult::assess(
            "din18599.1.climate-composition",
            "DIN V 18599-10",
            ClauseId::new("DIN V 18599", "10", "climate"),
            SubjectRef::new("", "climate.childId", copy("Climate composition", "Klimakomposition")),
            copy("Climate child handle integrity", "Integrität des Klima-Child-Handles"),
        )
        .annex(annex)
        .explanation(copy(
            &format!("Climate child '{child_id}' → {}@{}/{}.", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            &format!("Klima-Child '{child_id}' → {}@{}/{}.", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
        ));
        if ok {
            b = b.utilization(dimensionless(1.0), dimensionless(1.0));
        } else {
            b = b
                .utilization(dimensionless(0.0), dimensionless(1.0))
                .remedy(Remedy::one_of(
                    SubjectRef::new("", "climate.childId", copy("Climate child id", "Klima-Child-ID")),
                    valid_handles,
                    copy(
                        "Restore a valid s.stdio.semio@v1/table climate handle (reference TRY / Potsdam).",
                        "Gültigen s.stdio.semio@v1/table-Klima-Handle wiederherstellen (Referenz-TRY / Potsdam).",
                    ),
                ));
        }
        report.push(b.build());
    }

    // --- Net floor area A_N vs Σ zone areas ---
    {
        let a_n = doc.net_floor_area_m2;
        let a_zones: f64 = doc.zones.iter().map(|z| z.area_m2).sum();
        let mut b = CheckResult::assess(
            "din18599.1.net-floor-area",
            "DIN V 18599-1",
            ClauseId::new("DIN V 18599", "1", "A_N"),
            SubjectRef::new("", "netFloorAreaM2", copy("Net floor area A_N", "Nettogrundfläche A_N")),
            copy("Net floor area vs zone areas", "Nettogrundfläche gegenüber Zonenflächen"),
        )
        .utilization(Quantity::new(QuantityKind::Area, a_zones), Quantity::new(QuantityKind::Area, a_n.max(1e-9)))
        .annex(annex)
        .explanation(copy(
            &format!("A_N = {a_n:.1} m²; ΣA_zone = {a_zones:.1} m²."),
            &format!("A_N = {a_n:.1} m²; ΣA_zone = {a_zones:.1} m²."),
        ));
        if (a_zones - a_n).abs() > 0.05 * a_n.max(1.0) {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "netFloorAreaM2", copy("Net floor area A_N", "Nettogrundfläche A_N")),
                Quantity::new(QuantityKind::Area, a_n),
                Quantity::new(QuantityKind::Area, a_zones),
                copy(
                    &format!("Set A_N to {a_zones:.1} m² to match Σ zone areas."),
                    &format!("A_N auf {a_zones:.1} m² setzen, damit Σ Zonenflächen übereinstimmt."),
                ),
            ));
        }
        report.push(b.build());
    }

    // --- Element zoneId linkage ---
    {
        let zone_ids: Vec<String> = doc.zones.iter().map(|z| z.id.clone()).collect();
        for el in &doc.elements {
            let path = format!("elements[id={}].zoneId", el.id);
            let ok = zone_ids.iter().any(|z| z == &el.zone_id);
            let mut b = CheckResult::assess(
                format!("din18599.1.element-zone.{}", el.id),
                "DIN V 18599-1",
                ClauseId::new("DIN V 18599", "1", "zone"),
                SubjectRef::new(&el.id, &path, copy("Element zone", "Bauteilzone")),
                copy("Envelope element zone assignment", "Zonenzuordnung Hüllbauteil"),
            )
            .annex(annex)
            .explanation(copy(
                &format!("Element {} assigned to zone '{}'.", el.label_en, el.zone_id),
                &format!("Bauteil {} der Zone '{}' zugeordnet.", el.label_de, el.zone_id),
            ));
            if ok {
                b = b.utilization(dimensionless(1.0), dimensionless(1.0));
            } else {
                b = b
                    .utilization(dimensionless(0.0), dimensionless(1.0))
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&el.id, &path, copy("Element zone", "Bauteilzone")),
                        zone_ids.clone(),
                        copy(
                            &format!("Assign {} to an existing zone id.", el.label_en),
                            &format!("{} einer vorhandenen Zonen-ID zuordnen.", el.label_de),
                        ),
                    ));
            }
            report.push(b.build());
        }
    }

    // --- GEG H′T (residential) / mean U (non-residential) ---
    match doc.building_category {
        BuildingCategory::Residential => {
            let limit = geg_ht_prime_limit(doc.attachment, doc.net_floor_area_m2);
            let computed = derived.h_t_prime;
            let mut b = CheckResult::assess(
                "din18599.geg.ht-prime",
                "GEG §16 / Anlage 2",
                ClauseId::new("GEG", "Anlage 2", "H'T"),
                SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                copy("Specific transmission heat loss H′T", "Spezifischer Transmissionswärmeverlust H′T"),
            )
            .utilization(heat_transfer(computed), heat_transfer(limit))
            .annex(annex)
            .explanation(copy(
                &format!("H′T = H_T/A = {computed:.3} W/(m²·K); limit {limit:.2} W/(m²·K) for {:?} with A_N={:.0} m².", doc.attachment, doc.net_floor_area_m2),
                &format!("H′T = H_T/A = {computed:.3} W/(m²·K); Grenzwert {limit:.2} W/(m²·K) für {:?} mit A_N={:.0} m².", doc.attachment, doc.net_floor_area_m2),
            ));
            if computed > limit {
                for el in &doc.elements {
                    let target = reference_u(el.kind);
                    if el.u_value_w_m2k > target + 1e-9 {
                        let path = element_u_path(el);
                        b = b.remedy(Remedy::at_most(
                            element_ref(el, &path),
                            heat_transfer(el.u_value_w_m2k),
                            heat_transfer(target),
                            copy(
                                &format!("Reduce U of {} from {:.3} to GEG reference {:.3} W/(m²·K).", el.label_en, el.u_value_w_m2k, target),
                                &format!("U-Wert von {} von {:.3} auf GEG-Referenz {:.3} W/(m²·K) senken.", el.label_de, el.u_value_w_m2k, target),
                            ),
                        ));
                    }
                }
                if doc.delta_u_wb_w_m2k > GEG_ANLAGE1_DELTA_U_WB_REF + 1e-9 {
                    b = b.remedy(Remedy::at_most(
                        SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                        heat_transfer(doc.delta_u_wb_w_m2k),
                        heat_transfer(GEG_ANLAGE1_DELTA_U_WB_REF),
                        copy(
                            "Lower ΔU_WB to the GEG reference surcharge 0.05 W/(m²·K).",
                            "ΔU_WB auf den GEG-Referenzzuschlag 0.05 W/(m²·K) senken.",
                        ),
                    ));
                }
            }
            report.push(b.build());
        }
        BuildingCategory::NonResidential => {
            for kind in [ElementKind::Wall, ElementKind::Roof, ElementKind::Floor, ElementKind::Window] {
                match mean_u_for_kind(doc, kind) {
                    None => {
                        report.push(
                            CheckResult::assess(
                                format!("din18599.geg.mean-u.{kind:?}").to_lowercase(),
                                "GEG Anlage 3",
                                ClauseId::new("GEG", "Anlage 3", format!("{kind:?}")),
                                whole_building(),
                                copy(&format!("Mean U ({kind:?})"), &format!("Mittlerer U-Wert ({kind:?})")),
                            )
                            .not_applicable(copy("No elements of this kind.", "Keine Bauteile dieser Art vorhanden."))
                            .annex(annex)
                            .build(),
                        );
                    }
                    Some((mean, limit)) => {
                        let mut b = CheckResult::assess(
                            format!("din18599.geg.mean-u.{kind:?}").to_lowercase(),
                            "GEG Anlage 3",
                            ClauseId::new("GEG", "Anlage 3", format!("{kind:?}")),
                            whole_building(),
                            copy(&format!("Mean U-value ({kind:?})"), &format!("Mittlerer U-Wert ({kind:?})")),
                        )
                        .utilization(heat_transfer(mean), heat_transfer(limit))
                        .annex(annex)
                        .explanation(copy(
                            &format!("Ū = {mean:.3} W/(m²·K) vs limit {limit:.2}."),
                            &format!("Ū = {mean:.3} W/(m²·K) gegenüber Grenzwert {limit:.2}."),
                        ));
                        if mean > limit {
                            if let Some(el) = doc.elements.iter().filter(|e| e.kind == kind).max_by(|a, b| a.u_value_w_m2k.partial_cmp(&b.u_value_w_m2k).unwrap_or(std::cmp::Ordering::Equal)) {
                                let path = element_u_path(el);
                                b = b.remedy(Remedy::at_most(
                                    element_ref(el, &path),
                                    heat_transfer(el.u_value_w_m2k),
                                    heat_transfer(limit),
                                    copy(
                                        &format!("Reduce U of {} to at most {limit:.2} W/(m²·K).", el.label_en),
                                        &format!("U-Wert von {} auf höchstens {limit:.2} W/(m²·K) senken.", el.label_de),
                                    ),
                                ));
                            }
                        }
                        report.push(b.build());
                    }
                }
            }
        }
    }

    // --- Part 2 monthly heating demand vs GEG Anlage 1 reference Q_H,nd ---
    {
        let q_h = derived.q_h_nd_kwh;
        let q_h_ref = derived.q_h_ref_kwh.max(1.0);
        let mut b = CheckResult::assess(
            "din18599.2.heating-demand",
            "DIN V 18599-2",
            ClauseId::new("DIN V 18599", "2", "5"),
            whole_building(),
            copy("Net heating demand Q_H,nd", "Heizwärmebedarf Q_H,nd"),
        )
        .utilization(energy(q_h), energy(q_h_ref))
        .annex(annex)
        .explanation(copy(
            &format!("Q_H,nd = {q_h:.0} kWh/a vs reference building Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
            &format!("Q_H,nd = {q_h:.0} kWh/a gegenüber Referenzgebäude Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
        ));
        if q_h > q_h_ref {
            let mut remedied = false;
            for el in &doc.elements {
                let target = reference_u(el.kind);
                if el.u_value_w_m2k > target + 1e-9 {
                    let path = element_u_path(el);
                    b = b.remedy(Remedy::at_most(
                        element_ref(el, &path),
                        heat_transfer(el.u_value_w_m2k),
                        heat_transfer(target),
                        copy(
                            &format!("Improve envelope: set U of {} ≤ {:.3} W/(m²·K) (GEG reference).", el.label_en, target),
                            &format!("Hülle verbessern: U von {} ≤ {:.3} W/(m²·K) (GEG-Referenz).", el.label_de, target),
                        ),
                    ));
                    remedied = true;
                }
            }
            if !remedied {
                if doc.delta_u_wb_w_m2k > GEG_ANLAGE1_DELTA_U_WB_REF + 1e-9 {
                    b = b.remedy(Remedy::at_most(
                        SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                        heat_transfer(doc.delta_u_wb_w_m2k),
                        heat_transfer(GEG_ANLAGE1_DELTA_U_WB_REF),
                        copy("Lower ΔU_WB to 0.05 W/(m²·K) to approach the reference heat demand.", "ΔU_WB auf 0.05 W/(m²·K) senken, um den Referenz-Heizwärmebedarf zu erreichen."),
                    ));
                } else if doc.ventilation.heat_recovery_eta < GEG_ANLAGE1_TABELLE1_ETA_WRG_REF {
                    b = b.remedy(Remedy::at_least(
                        SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
                        dimensionless(doc.ventilation.heat_recovery_eta),
                        dimensionless(GEG_ANLAGE1_TABELLE1_ETA_WRG_REF),
                        copy(
                        &format!("Raise heat recovery to at least {:.0}% to reduce Q_H,nd toward the reference.", GEG_ANLAGE1_TABELLE1_ETA_WRG_REF * 100.0),
                        &format!("Wärmerückgewinnung auf mindestens {:.0}% anheben, um Q_H,nd Richtung Referenz zu senken.", GEG_ANLAGE1_TABELLE1_ETA_WRG_REF * 100.0),
                    ),
                    ));
                } else {
                    b = b.remedy(Remedy::at_least(
                        SubjectRef::new("", "renewables.pvAreaM2", copy("PV area", "PV-Fläche")),
                        Quantity::new(QuantityKind::Area, doc.renewables.pv_area_m2),
                        Quantity::new(QuantityKind::Area, doc.renewables.pv_area_m2 + 10.0),
                        copy("Increase PV area while refining the envelope to meet the reference heat demand.", "PV-Fläche erhöhen und Hülle nachschärfen, um den Referenz-Heizwärmebedarf zu erreichen."),
                    ));
                }
            }
        }
        report.push(b.build());
    }

    // --- Part 6/7 ventilation heat recovery ---
    {
        let eta = doc.ventilation.heat_recovery_eta;
        let min_eta = GEG_ANLAGE1_TABELLE1_ETA_WRG_REF;
        let mut b = CheckResult::assess(
            "din18599.6.heat-recovery",
            "DIN V 18599-6",
            ClauseId::new("DIN V 18599", "6", "6"),
            SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
            copy("Ventilation heat recovery", "Wärmerückgewinnung Lüftung"),
        )
        .minimum(dimensionless(eta), dimensionless(min_eta))
        .annex(annex)
        .explanation(copy(
            &format!("η_WRG = {eta:.2}; H_V = {:.1} W/K at {:.0} m³/h.", derived.h_v, doc.ventilation.airflow_m3_h),
            &format!("η_WRG = {eta:.2}; H_V = {:.1} W/K bei {:.0} m³/h.", derived.h_v, doc.ventilation.airflow_m3_h),
        ));
        if eta < min_eta {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
                dimensionless(eta),
                dimensionless(min_eta),
                copy(
                    &format!("Increase heat recovery from {:.0}% to at least {:.0}%.", eta * 100.0, min_eta * 100.0),
                    &format!("Wärmerückgewinnung von {:.0}% auf mindestens {:.0}% erhöhen.", eta * 100.0, min_eta * 100.0),
                ),
            ));
        }
        report.push(b.build());
    }

    // --- Part 4 lighting (DIN V 18599-4) — zone installed power vs profile limit ---
    {
        for zone in &doc.zones {
            let row = usage_profile_row(zone.usage_profile);
            let lpd = zone.lighting_power_w_m2;
            let limit = row.lighting_lpd_limit_w_m2;
            let path = format!("zones[id={}].lightingPowerWM2", zone.id);
            let mut b = CheckResult::assess(
                format!("din18599.4.lighting-power.{}", zone.id),
                "DIN V 18599-4",
                ClauseId::new("DIN V 18599", "4", "5"),
                SubjectRef::new(&zone.id, &path, copy("Zone lighting power", "Zonenbeleuchtungsleistung")),
                copy("Lighting installed power density", "Spezifische Beleuchtungsleistung"),
            )
            .utilization(Quantity::new(QuantityKind::Power, lpd), Quantity::new(QuantityKind::Power, limit))
            .annex(annex)
            .explanation(copy(
                &format!("Zone {} LPD = {lpd:.1} W/m² vs profile {:?} limit {limit:.1} W/m²; t_L = {:.0} h/a.", zone.label_en, zone.usage_profile, row.lighting_hours_a),
                &format!("Zone {} LPD = {lpd:.1} W/m² gegenüber Profil {:?} Grenzwert {limit:.1} W/m²; t_L = {:.0} h/a.", zone.label_de, zone.usage_profile, row.lighting_hours_a),
            ));
            if lpd > limit + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new(&zone.id, &path, copy("Zone lighting power", "Zonenbeleuchtungsleistung")),
                    Quantity::new(QuantityKind::Power, lpd),
                    Quantity::new(QuantityKind::Power, limit),
                    copy(
                        &format!("Reduce lighting power in {} to at most {limit:.1} W/m².", zone.label_en),
                        &format!("Beleuchtungsleistung in {} auf höchstens {limit:.1} W/m² senken.", zone.label_de),
                    ),
                ));
            }
            report.push(b.build());
        }
    }

// --- Part 5 heating system efficiency ---
    {
        let eff = heating_system_efficiency(doc);
        let min_eff = DIN_V_18599_5_ETA_SYS_MIN;
        let mut b = CheckResult::assess(
            "din18599.5.heating-efficiency",
            "DIN V 18599-5",
            ClauseId::new("DIN V 18599", "5", "6"),
            SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugeraufwand")),
            copy("Heating overall system efficiency", "Gesamtaufwandzahl Heizung"),
        )
        .minimum(dimensionless(eff), dimensionless(min_eff))
        .annex(annex)
        .explanation(copy(
            &format!("η_sys = {eff:.3}; Q_f,heat = {:.0} kWh/a.", derived.q_f_heat_kwh),
            &format!("η_sys = {eff:.3}; Q_f,Wärme = {:.0} kWh/a.", derived.q_f_heat_kwh),
        ));
        if eff < min_eff {
            let gen_req = (min_eff / (doc.heating.distribution_efficiency * doc.heating.storage_efficiency * doc.heating.transfer_efficiency).max(DIN_V_18599_5_ETA_SYS_FLOOR)).min(1.05);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugungswirkungsgrad")),
                dimensionless(doc.heating.generation_efficiency),
                dimensionless(gen_req),
                copy(
                    &format!("Raise generation efficiency from {:.2} to at least {:.2}.", doc.heating.generation_efficiency, gen_req),
                    &format!("Erzeugungswirkungsgrad von {:.2} auf mindestens {:.2} erhöhen.", doc.heating.generation_efficiency, gen_req),
                ),
            ));
        }
        report.push(b.build());
    }

    // --- Part 8 DHW (DIN V 18599-8 / -10 profile + loss allowances) ---
    {
        let q_w = derived.q_w_kwh;
        let n = occupants(doc) as f64;
        let profile = dhw_profile_specific_kwh_person_a(doc.use_class);
        let limit = dhw_limit_kwh_a(doc);
        let mut b = CheckResult::assess(
            "din18599.8.dhw",
            "DIN V 18599-8",
            ClauseId::new("DIN V 18599", "8", "6"),
            SubjectRef::new("", "dhw.specificDemandKwhPersonA", copy("DHW specific demand", "TWW-spezifischer Bedarf")),
            copy("Domestic hot water demand", "Trinkwarmwasserbedarf"),
        )
        .utilization(energy(q_w), energy(limit))
        .annex(annex)
        .explanation(copy(
            &format!(
                "Q_W = {q_w:.0} kWh/a (= {n:.0}·{:.0} + storage {:.0} + distribution {:.0}); limit {limit:.0} from DIN V 18599-10 profile {profile:.0} kWh/(Person·a) plus -8 loss caps.",
                doc.dhw.specific_demand_kwh_person_a, doc.dhw.storage_loss_kwh_a, doc.dhw.distribution_loss_kwh_a
            ),
            &format!(
                "Q_W = {q_w:.0} kWh/a (= {n:.0}·{:.0} + Speicher {:.0} + Verteilung {:.0}); Grenzwert {limit:.0} aus DIN-V-18599-10-Profil {profile:.0} kWh/(Person·a) zuzüglich -8-Verlustgrenzen.",
                doc.dhw.specific_demand_kwh_person_a, doc.dhw.storage_loss_kwh_a, doc.dhw.distribution_loss_kwh_a
            ),
        ));
        if q_w > limit {
            if doc.dhw.specific_demand_kwh_person_a > profile + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "dhw.specificDemandKwhPersonA", copy("DHW specific demand", "TWW-spezifischer Bedarf")),
                    energy(doc.dhw.specific_demand_kwh_person_a),
                    energy(profile),
                    copy(
                        &format!("Reduce specific DHW demand from {:.0} to at most {profile:.0} kWh/(Person·a) (DIN V 18599-10).", doc.dhw.specific_demand_kwh_person_a),
                        &format!("Spezifischen TWW-Bedarf von {:.0} auf höchstens {profile:.0} kWh/(Person·a) senken (DIN V 18599-10).", doc.dhw.specific_demand_kwh_person_a),
                    ),
                ));
            }
            let room = (limit - n * doc.dhw.specific_demand_kwh_person_a.min(profile) - doc.dhw.distribution_loss_kwh_a).max(0.0);
            if doc.dhw.storage_loss_kwh_a > room + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "dhw.storageLossKwhA", copy("DHW storage loss", "TWW-Speicherverlust")),
                    energy(doc.dhw.storage_loss_kwh_a),
                    energy(room),
                    copy(
                        &format!("Reduce DHW storage losses from {:.0} to at most {room:.0} kWh/a.", doc.dhw.storage_loss_kwh_a),
                        &format!("TWW-Speicherverluste von {:.0} auf höchstens {room:.0} kWh/a senken.", doc.dhw.storage_loss_kwh_a),
                    ),
                ));
            }
            let room_dist = (limit - n * doc.dhw.specific_demand_kwh_person_a.min(profile) - doc.dhw.storage_loss_kwh_a.min(room)).max(0.0);
            if doc.dhw.distribution_loss_kwh_a > room_dist + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "dhw.distributionLossKwhA", copy("DHW distribution loss", "TWW-Verteilverlust")),
                    energy(doc.dhw.distribution_loss_kwh_a),
                    energy(room_dist),
                    copy(
                        &format!("Reduce DHW distribution losses from {:.0} to at most {room_dist:.0} kWh/a.", doc.dhw.distribution_loss_kwh_a),
                        &format!("TWW-Verteilverluste von {:.0} auf höchstens {room_dist:.0} kWh/a senken.", doc.dhw.distribution_loss_kwh_a),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

        // --- Part 2 cooling need Q_C,nd (always assessed so θ_i,c / gains remain normative) ---
    {
        let q_c = derived.q_c_nd_kwh;
        let a_n = doc.net_floor_area_m2.max(1.0);
        let limit = a_n * 200.0;
        let zid = doc.zones.first().map(|z| z.id.clone()).unwrap_or_default();
        let path = format!("zones[id={zid}].thetaICoolC");
        let theta = doc.zones.first().map(|z| z.theta_i_cool_c).unwrap_or(26.0);
        let mut b = CheckResult::assess(
            "din18599.2.cooling-need",
            "DIN V 18599-2",
            ClauseId::new("DIN V 18599", "2", "Q_C,nd"),
            SubjectRef::new(&zid, &path, copy("Indoor cooling setpoint", "Kühlsolltemperatur")),
            copy("Net cooling demand Q_C,nd", "Kühlbedarf Q_C,nd"),
        )
        .annex(annex)
        .utilization(energy(q_c), energy(limit))
        .explanation(copy(
            &format!("Q_C,nd = {q_c:.0} kWh/a; allowance {limit:.0} kWh/a (200 kWh/(m²·a)·A_N)."),
            &format!("Q_C,nd = {q_c:.0} kWh/a; Zulassung {limit:.0} kWh/a (200 kWh/(m²·a)·A_N)."),
        ));
        if q_c > limit {
            b = b.remedy(Remedy::at_most(
                SubjectRef::new(&zid, &path, copy("Indoor cooling setpoint", "Kühlsolltemperatur")),
                Quantity::new(QuantityKind::Temperature, theta),
                Quantity::new(QuantityKind::Temperature, theta + 2.0),
                copy(
                    "Raise cooling setpoints or reduce solar/internal gains so Q_C,nd ≤ 200 kWh/(m²·a)·A_N.",
                    "Kühlsollwerte anheben oder solare/interne Gewinne senken, damit Q_C,nd ≤ 200 kWh/(m²·a)·A_N.",
                ),
            ));
        }
        report.push(b.build());
    }

// --- Part 7 cooling ---
    if doc.cooling.is_installed() {
        let q_c = derived.q_c_nd_kwh;
        let limit = derived.q_h_nd_kwh.max(1.0) * DIN_V_18599_7_COOLING_TO_HEATING_RATIO;
        let mut b = CheckResult::assess(
            "din18599.7.cooling",
            "DIN V 18599-7",
            ClauseId::new("DIN V 18599", "7", "6"),
            whole_building(),
            copy("Net cooling demand Q_C,nd", "Kühlbedarf Q_C,nd"),
        )
        .utilization(energy(q_c), energy(limit))
        .annex(annex)
        .explanation(copy(&format!("Q_C,nd = {q_c:.0} kWh/a (limit 0.5 · Q_H,nd)."), &format!("Q_C,nd = {q_c:.0} kWh/a (Grenzwert 0,5 · Q_H,nd).")));
        if q_c > limit {
            if let Some(el) = doc.elements.iter().filter(|e| e.kind == ElementKind::Window).max_by(|a, b| a.g_value.partial_cmp(&b.g_value).unwrap()) {
                let g_req = (el.g_value * din_v_18599_7_g_remedy::REDUCE_FACTOR).max(din_v_18599_7_g_remedy::FLOOR);
                b = b.remedy(Remedy::at_most(
                    element_ref(el, &element_g_path(el)),
                    dimensionless(el.g_value),
                    dimensionless(g_req),
                    copy(
                        &format!("Reduce g-value of {} from {:.2} to at most {:.2}.", el.label_en, el.g_value, g_req),
                        &format!("g-Wert von {} von {:.2} auf höchstens {:.2} senken.", el.label_de, el.g_value, g_req),
                    ),
                ));
            }
        }
        report.push(b.build());
    } else {
        report.push(
            CheckResult::assess(
                "din18599.7.cooling",
                "DIN V 18599-7",
                ClauseId::new("DIN V 18599", "7", "6"),
                whole_building(),
                copy("Net cooling demand Q_C,nd", "Kühlbedarf Q_C,nd"),
            )
            .not_applicable(copy("No cooling system present.", "Keine Kühlanlage vorhanden."))
            .annex(annex)
            .build(),
        );
    }

    // --- Part 11 automation ---
    {
        let f = automation_factor(doc.automation_class);
        let computed = if matches!(doc.automation_class, AutomationClass::D) { 1.1 } else { f };
        let mut b = CheckResult::assess(
            "din18599.11.automation",
            "DIN V 18599-11",
            ClauseId::new("DIN V 18599", "11", "5"),
            SubjectRef::new("", "automationClass", copy("Automation class", "Automationsklasse")),
            copy("Building automation class", "Gebäudeautomation"),
        )
        .utilization(dimensionless(computed), dimensionless(1.0))
        .annex(annex)
        .explanation(copy(
            &format!("BACS class {:?}, factor f_BAC = {f:.2}.", doc.automation_class),
            &format!("GA-Klasse {:?}, Faktor f_BAC = {f:.2}.", doc.automation_class),
        ));
        if matches!(doc.automation_class, AutomationClass::D) {
            b = b.remedy(Remedy::one_of(
                SubjectRef::new("", "automationClass", copy("Automation class", "Automationsklasse")),
                vec!["A".into(), "B".into(), "C".into()],
                copy("Upgrade building automation to class C or better.", "Gebäudeautomation auf Klasse C oder besser anheben."),
            ));
        }
        report.push(b.build());
    }

    // --- Part 9 / renewables PV credit (minimum PV for failing primary energy) ---
    // --- GEG primary energy vs reference × 0.55 ---
    {
        let q_p = derived.q_p_kwh;
        let limit = derived.q_p_limit_kwh.max(1.0);
        let mut b = CheckResult::assess(
            "din18599.geg.qp",
            "GEG §10 / Anlage 1–2",
            ClauseId::new("GEG", "§10", "Q_P"),
            SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
            copy("Primary energy Q_P ≤ 0.55 · Q_P,Ref", "Primärenergie Q_P ≤ 0,55 · Q_P,Ref"),
        )
        .utilization(energy(q_p), energy(limit))
        .annex(annex)
        .explanation(copy(
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; limit = 0.55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; Grenzwert = 0,55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
        ));
        if q_p > limit {
            let deficit = q_p - limit;
            let fp_el = primary_energy_factor("electricity");
            let annual_g = {
                let climate = crate::din18599_climate(doc);
                climate.g_h_w_m2.iter().sum::<f64>() / 12.0 * 8760.0 / 1000.0
            };
            let pv_extra = if doc.renewables.pv_efficiency * annual_g * fp_el <= 1e-9 {
                50.0
            } else {
                deficit / (doc.renewables.pv_efficiency.max(DIN_V_18599_5_ETA_SYS_FLOOR) * annual_g * fp_el)
            };
            let pv_req = doc.renewables.pv_area_m2 + pv_extra;
            b = b
                .remedy(Remedy::at_least(
                    SubjectRef::new("", "renewables.pvAreaM2", copy("PV area", "PV-Fläche")),
                    Quantity::new(QuantityKind::Area, doc.renewables.pv_area_m2),
                    Quantity::new(QuantityKind::Area, pv_req),
                    copy(
                        &format!("Increase PV area from {:.1} m² to at least {:.1} m².", doc.renewables.pv_area_m2, pv_req),
                        &format!("PV-Fläche von {:.1} m² auf mindestens {:.1} m² erhöhen.", doc.renewables.pv_area_m2, pv_req),
                    ),
                ))
                .remedy(Remedy::one_of(
                    SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
                    vec!["natural_gas".into(), "biomass".into(), "district_heating".into()],
                    copy(
                        &format!("Switch heating carrier from '{}' (f_p={:.2}) to biomass or district heating.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                        &format!("Energieträger Heizung von '{}' (f_p={:.2}) auf Biomasse oder Fernwärme umstellen.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                    ),
                ));
            let gen_req = (doc.heating.generation_efficiency * (q_p / limit).min(1.4)).min(1.05);
            if gen_req > doc.heating.generation_efficiency {
                b = b.remedy(Remedy::at_least(
                    SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugungswirkungsgrad")),
                    dimensionless(doc.heating.generation_efficiency),
                    dimensionless(gen_req),
                    copy(
                        &format!("Raise generation efficiency from {:.2} to at least {:.2}.", doc.heating.generation_efficiency, gen_req),
                        &format!("Erzeugungswirkungsgrad von {:.2} auf mindestens {:.2} erhöhen.", doc.heating.generation_efficiency, gen_req),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

    // --- Part 12 tabular (applicability gated) ---
    match doc.method {
        CalculationMethod::Tabular => {
            let q_tab = tabular_specific_primary_energy_kwh_m2(doc.use_class) * doc.net_floor_area_m2 * automation_factor(doc.automation_class);
            let limit = derived.q_p_limit_kwh.max(1.0);
            let mut b = CheckResult::assess(
                "din18599.12.tabular",
                "DIN V 18599-12",
                ClauseId::new("DIN V 18599", "12", "4"),
                whole_building(),
                copy("Tabular primary energy", "Tabellarische Primärenergie"),
            )
            .utilization(energy(q_tab), energy(limit))
            .annex(annex)
            .explanation(copy(
                &format!("Q_P,tab = {q_tab:.0} kWh/a vs GEG limit {limit:.0}."),
                &format!("Q_P,tab = {q_tab:.0} kWh/a gegenüber GEG-Grenzwert {limit:.0}."),
            ));
            if q_tab > limit {
                b = b.remedy(Remedy::one_of(
                    SubjectRef::new("", "method", copy("Calculation method", "Berechnungsverfahren")),
                    vec!["DetailedMonthly".into()],
                    copy("Switch to detailed monthly method to resolve tabular exceedance.", "Auf detailliertes Monatsbilanzverfahren wechseln."),
                ));
            }
            report.push(b.build());
        }
        CalculationMethod::DetailedMonthly => {
            report.push(
                CheckResult::assess(
                    "din18599.12.tabular",
                    "DIN V 18599-12",
                    ClauseId::new("DIN V 18599", "12", "4"),
                    whole_building(),
                    copy("Tabular primary energy", "Tabellarische Primärenergie"),
                )
                .not_applicable(copy("Detailed monthly method selected; tabular method not applicable.", "Detailliertes Monatsverfahren gewählt; Tabellenverfahren nicht anwendbar."))
                .annex(annex)
                .build(),
            );
        }
    }

    report
}
//#endregion 🔖️Checks

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️oracle/🦀️.rs"]
mod oracle_tests;
//#endregion 🧪️ComplianceTests
