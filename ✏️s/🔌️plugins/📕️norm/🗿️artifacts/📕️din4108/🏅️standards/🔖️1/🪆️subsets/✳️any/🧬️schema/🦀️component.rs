//! 🧬️ Din4108 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Din4108 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Artifact {
    #[state(artifact)]
    pub category: String,
    #[state(artifact)]
    pub layers: Vec<crate::artifacts::din4108::LayerDocument>,
    #[state(artifact)]
    pub climate: ClimateZoneDe,
    #[state(artifact)]
    pub airtightness_n50: f64,
    #[state(artifact)]
    pub psi_times_l_sum: f64,
    #[state(artifact)]
    pub rh_int: f64,
    #[state(artifact)]
    pub catalog_id: String,
    #[state(artifact)]
    pub material_id: String,
    #[state(artifact)]
    pub airtightness_class: String,
    #[state(artifact)]
    pub t_int_c: f64,
    #[state(artifact)]
    pub solar_absorptance: f64,
    #[state(artifact)]
    pub irradiance_w_m2: f64,
    #[state(artifact)]
    pub moisture_mu_exterior: f64,
    #[state(artifact)]
    pub moisture_mu_interior: f64,
    #[state(artifact)]
    pub envelope_area_m2: f64,
    #[state(artifact)]
    pub bb2_details_conform: bool,
    #[state(artifact)]
    pub application_type: String,
    #[state(artifact)]
    pub declared_application_class: String,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din4108Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::din4108::Din4108Snapshot {
        crate::artifacts::din4108::Din4108Snapshot {
            category: self.category.clone(),
            layers: self.layers.clone(),
            climate: self.climate,
            airtightness_n50: self.airtightness_n50,
            psi_times_l_sum: self.psi_times_l_sum,
            rh_int: self.rh_int,
            catalog_id: self.catalog_id.clone(),
            material_id: self.material_id.clone(),
            airtightness_class: self.airtightness_class.clone(),
            t_int_c: self.t_int_c,
            solar_absorptance: self.solar_absorptance,
            irradiance_w_m2: self.irradiance_w_m2,
            moisture_mu_exterior: self.moisture_mu_exterior,
            moisture_mu_interior: self.moisture_mu_interior,
            envelope_area_m2: self.envelope_area_m2,
            bb2_details_conform: self.bb2_details_conform,
            application_type: self.application_type.clone(),
            declared_application_class: self.declared_application_class.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::din4108::Din4108Snapshot) -> Self {
        Self {
            category: snapshot.category.clone(),
            layers: snapshot.layers.clone(),
            climate: snapshot.climate,
            airtightness_n50: snapshot.airtightness_n50,
            psi_times_l_sum: snapshot.psi_times_l_sum,
            rh_int: snapshot.rh_int,
            catalog_id: snapshot.catalog_id.clone(),
            material_id: snapshot.material_id.clone(),
            airtightness_class: snapshot.airtightness_class.clone(),
            t_int_c: snapshot.t_int_c,
            solar_absorptance: snapshot.solar_absorptance,
            irradiance_w_m2: snapshot.irradiance_w_m2,
            moisture_mu_exterior: snapshot.moisture_mu_exterior,
            moisture_mu_interior: snapshot.moisture_mu_interior,
            envelope_area_m2: snapshot.envelope_area_m2,
            bb2_details_conform: snapshot.bb2_details_conform,
            application_type: snapshot.application_type.clone(),
            declared_application_class: snapshot.declared_application_class.clone(),
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::din4108::Din4108Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din4108` — twenty handcrafted schema leaves.
pub fn din4108_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.din4108",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Din4108BuilderConstruction {
        snapshot: Din4108Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Din4108BuilderConstruction {
        type Snapshot = Din4108Snapshot;
        type Mutation = Din4108Mutation;
        type Diff = Din4108Diff;
        fn empty() -> Self {
            Self { snapshot: Din4108Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::din4108::Din4108Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Din4108Parts {
        pub snapshot: Option<Din4108Snapshot>,
    }

    pub struct Din4108AnalyzerAnalysis;

    impl ArtifactAnalysis for Din4108AnalyzerAnalysis {
        type Parts = Din4108Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.din4108", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Din4108Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Din4108BuilderFacets {
        construction: Din4108BuilderConstruction,
        analysis: Din4108AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Din4108ComposerComposition,
    }
    builder: Din4108Builder,
    analyzer: Din4108Analyzer,
    composer: Din4108Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure DIN 4108 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`. Every `part_N`/`bb_2` module is a pure function
/// library over document types (layers, categories, climate); the snapshot-level composition
/// (`evaluate`, `check_full_envelope`, …) lives in `💡️inferences` since it is a conformance law over
/// the whole `Din4108Snapshot`.
use crate::document::{table_lookup_linear, AnnexChoice, CheckResult, ClauseId, ClimateZoneDe, NormError, Quantity, TableEntry1D};

pub const R_SI_WALL_M2K_W: f64 = 0.13;
pub const R_SE_WALL_M2K_W: f64 = 0.04;
pub const F_RSI_MINIMUM: f64 = 0.25;

// #region 🔖️Part1
/// ⚠️ DIN 4108-1 (terms, symbols, applicability) is withdrawn from current normative practice; `scope_check` is kept only for clause-trace continuity across parts, while `check_input_plausibility` performs the substantive input validation part 1 no longer defines.
pub mod part_1 {
    use super::*;

    /// 📜️ DIN 4108 part identifiers covered by this crate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NormPart {
        Part1,
        Part2,
        Part3,
        Part4,
        Part5,
        Part6,
        Part7,
        Part8,
    }

    /// 🧱️ Building envelope element kinds referenced across DIN 4108.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BuildingElement {
        OpaqueWall,
        Roof,
        Floor,
        Window,
        Door,
    }

    /// 📋️ Human-readable scope statement per part (DIN 4108-1 definitions).
    pub fn part_scope(part: NormPart) -> &'static str {
        match part {
            NormPart::Part1 => "definitions, symbols, and applicability of the DIN 4108 series",
            NormPart::Part2 => "minimum thermal protection requirements for heated buildings",
            NormPart::Part3 => "moisture protection and interior surface temperature limits",
            NormPart::Part4 => "design thermal conductivity values for building materials",
            NormPart::Part5 => "summer heat protection against overheating",
            NormPart::Part6 => "U-value calculation and proof methods including thermal bridges",
            NormPart::Part7 => "airtightness of the building envelope",
            NormPart::Part8 => "component catalog references for standard constructions",
        }
    }

    /// ✅️ Whether a DIN 4108 part applies to the given building element.
    pub fn applies_to_element(part: NormPart, element: BuildingElement) -> bool {
        match part {
            NormPart::Part1 | NormPart::Part8 => true,
            NormPart::Part7 => !matches!(element, BuildingElement::Window),
            NormPart::Part2 | NormPart::Part3 | NormPart::Part4 | NormPart::Part5 | NormPart::Part6 => match element {
                BuildingElement::OpaqueWall | BuildingElement::Roof | BuildingElement::Floor => true,
                BuildingElement::Window | BuildingElement::Door => {
                    matches!(part, NormPart::Part4 | NormPart::Part6)
                }
            },
        }
    }

    /// 📑️ Clause trace for scope verification.
    pub fn scope_check(part: NormPart, element: BuildingElement) -> CheckResult {
        let applies = applies_to_element(part, element);
        CheckResult {
            clause: ClauseId::new("DIN 4108-1", "§3", "3.1"),
            status: if applies { crate::document::CheckStatus::Pass } else { crate::document::CheckStatus::NotApplicable },
            computed: Quantity::new(crate::document::QuantityKind::Dimensionless, if applies { 1.0 } else { 0.0 }),
            limit: Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            utilization: if applies { 1.0 } else { 0.0 },
            message: format!("scope: {} for {:?}", part_scope(part), element),
            annex: AnnexChoice::De,
        }
    }

    pub const U_VALUE_PLAUSIBLE_MIN_W_M2K: f64 = 0.1;
    pub const U_VALUE_PLAUSIBLE_MAX_W_M2K: f64 = 5.0;

    /// ✅️ Basic input-plausibility validation for an envelope Document (λ>0, R_T>0, U within a sane range), replacing DIN 4108-1's withdrawn definitions role.
    pub fn check_input_plausibility(layers: &[part_2::Layer], u_value: f64) -> CheckResult {
        if layers.is_empty() {
            return CheckResult {
                clause: ClauseId::new("DIN 4108-1", "§3", "3.1"),
                status: crate::document::CheckStatus::NotApplicable,
                computed: Quantity::new(crate::document::QuantityKind::Dimensionless, 0.0),
                limit: Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
                utilization: 0.0,
                message: "no layers supplied; input plausibility validation not applicable".into(),
                annex: AnnexChoice::De,
            };
        }
        let lambda_ok = layers.iter().all(|l| l.lambda_w_mk > 0.0);
        let r_total = part_2::total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
        let r_ok = r_total > 0.0;
        let u_ok = (U_VALUE_PLAUSIBLE_MIN_W_M2K..=U_VALUE_PLAUSIBLE_MAX_W_M2K).contains(&u_value);
        let plausible = lambda_ok && r_ok && u_ok;
        CheckResult {
            clause: ClauseId::new("DIN 4108-1", "§3", "3.1"),
            status: if plausible { crate::document::CheckStatus::Pass } else { crate::document::CheckStatus::Fail },
            computed: Quantity::u_value_w_m2k(u_value),
            limit: Quantity::u_value_w_m2k(U_VALUE_PLAUSIBLE_MAX_W_M2K),
            utilization: if plausible { u_value / U_VALUE_PLAUSIBLE_MAX_W_M2K } else { 2.0 },
            message: format!("input plausibility: λ>0={lambda_ok}, R_T>0={r_ok} ({r_total:.3} m²K/W), U∈[{U_VALUE_PLAUSIBLE_MIN_W_M2K},{U_VALUE_PLAUSIBLE_MAX_W_M2K}]={u_ok} ({u_value:.3} W/m²K)"),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖️Part1

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🏠️ Building category for minimum thermal protection (DIN 4108-2).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BuildingCategory {
        Residential,
        Office,
        School,
        Industrial,
    }

    /// 📋️ Layer in a building component for R-value accumulation.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Layer {
        pub thickness_m: f64,
        pub lambda_w_mk: f64,
    }

    /// 📐️ Total thermal resistance R_T = R_si + Σ(d/λ) + R_se [m²K/W].
    pub fn total_resistance(layers: &[Layer], r_si: f64, r_se: f64) -> f64 {
        let mut r = r_si + r_se;
        for layer in layers {
            if layer.lambda_w_mk > 0.0 {
                r += layer.thickness_m / layer.lambda_w_mk;
            }
        }
        r
    }

    /// 📉️ U-value from resistance [W/(m²K)].
    pub fn u_value_from_resistance(r_total: f64) -> f64 {
        if r_total <= 0.0 {
            return f64::INFINITY;
        }
        1.0 / r_total
    }

    fn base_limit_u_w_m2k(category: BuildingCategory) -> f64 {
        match category {
            BuildingCategory::Residential => 0.28,
            BuildingCategory::Office => 0.28,
            BuildingCategory::School => 0.28,
            BuildingCategory::Industrial => 0.50,
        }
    }

    /// 🌡️ Climate-adjusted U-limit: colder zones (higher HDD) allow slightly higher U [W/(m²K)].
    pub fn climate_adjusted_u_limit(category: BuildingCategory, climate: ClimateZoneDe) -> f64 {
        let base = base_limit_u_w_m2k(category);
        let hdd = climate.heating_degree_days();
        let table = [TableEntry1D { x: 2000.0, y: 1.00 }, TableEntry1D { x: 2600.0, y: 1.03 }, TableEntry1D { x: 3200.0, y: 1.06 }, TableEntry1D { x: 3800.0, y: 1.10 }];
        base * table_lookup_linear(&table, hdd)
    }

    /// ✅️ Check minimum thermal protection per DIN 4108-2 §4.
    pub fn check_minimum_thermal_protection(category: BuildingCategory, layers: &[Layer], climate: ClimateZoneDe) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput { field: "layers".into() });
        }
        let r = total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
        let u = u_value_from_resistance(r);
        let limit = climate_adjusted_u_limit(category, climate);
        Ok(CheckResult::from_utilization(ClauseId::new("DIN 4108-2", "§4", "4.1"), Quantity::u_value_w_m2k(u), Quantity::u_value_w_m2k(limit), "minimum thermal protection U-value", AnnexChoice::De))
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 💧️ Layer for Glaser moisture analysis (DIN 4108-3).
    #[derive(Clone, Debug, PartialEq)]
    pub struct MoistureLayer {
        pub thickness_m: f64,
        pub lambda_w_mk: f64,
        pub mu: f64,
    }

    /// 🌫️ Saturation vapor pressure via Magnus formula [Pa], T in °C.
    pub fn saturation_vapor_pressure_pa(t_c: f64) -> f64 {
        611.2 * (17.67 * t_c / (t_c + 243.5)).exp()
    }

    /// 💧️ Vapor diffusion resistance R_μ = d / (μ · λ) [m²·h·Pa/kg].
    pub fn vapor_resistance(layer: &MoistureLayer) -> f64 {
        if layer.mu <= 0.0 || layer.lambda_w_mk <= 0.0 {
            return f64::INFINITY;
        }
        layer.thickness_m / (layer.mu * layer.lambda_w_mk)
    }

    /// 🌡️ Temperature at each layer interface [°C], including interior and exterior surfaces.
    pub fn interface_temperatures_c(layers: &[MoistureLayer], r_si: f64, r_se: f64, t_int_c: f64, t_ext_c: f64) -> Vec<f64> {
        let r_layers: Vec<f64> = layers.iter().map(|l| if l.lambda_w_mk > 0.0 { l.thickness_m / l.lambda_w_mk } else { 0.0 }).collect();
        let r_total: f64 = r_si + r_se + r_layers.iter().sum::<f64>();
        let mut temps = Vec::with_capacity(layers.len() + 1);
        let mut r_cum = r_si;
        temps.push(t_int_c - (r_cum / r_total) * (t_int_c - t_ext_c));
        for r in &r_layers {
            r_cum += r;
            temps.push(t_int_c - (r_cum / r_total) * (t_int_c - t_ext_c));
        }
        temps
    }

    /// 💧️ Vapor pressure at each interface [Pa] assuming linear drop through R_μ.
    pub fn interface_vapor_pressures_pa(layers: &[MoistureLayer], t_int_c: f64, rh_int: f64) -> Vec<f64> {
        let p_int = rh_int * saturation_vapor_pressure_pa(t_int_c);
        let r_mu: Vec<f64> = layers.iter().map(vapor_resistance).collect();
        let r_mu_total: f64 = r_mu.iter().sum();
        if r_mu_total <= 0.0 {
            return vec![p_int; layers.len() + 1];
        }
        let mut pressures = Vec::with_capacity(layers.len() + 1);
        let mut r_cum = 0.0;
        pressures.push(p_int);
        for r in &r_mu {
            r_cum += r;
            let fraction = r_cum / r_mu_total;
            pressures.push(p_int * (1.0 - fraction));
        }
        pressures
    }

    /// 🧊️ Dew-point temperature from vapor pressure via inverse Magnus [°C].
    pub fn dew_point_temperature_c(vapor_pressure_pa: f64) -> f64 {
        if vapor_pressure_pa <= 0.0 {
            return -273.15;
        }
        let ln_ratio = (vapor_pressure_pa / 611.2).ln();
        243.5 * ln_ratio / (17.67 - ln_ratio)
    }

    /// ❄️ True when condensation risk exists at any interface (Glaser steady-state).
    pub fn condensation_at_interfaces(layers: &[MoistureLayer], t_int_c: f64, t_ext_c: f64, rh_int: f64) -> bool {
        let temps = interface_temperatures_c(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, t_int_c, t_ext_c);
        let pressures = interface_vapor_pressures_pa(layers, t_int_c, rh_int);
        temps.iter().zip(pressures.iter()).any(|(&t, &p)| p > saturation_vapor_pressure_pa(t) + 1.0)
    }

    /// 🌡️ Interior surface temperature factor f_Rsi with DIN 4108-3 humidity correction.
    pub fn interior_surface_temperature_factor(layers: &[MoistureLayer], r_si: f64, r_se: f64, t_int_c: f64, t_ext_c: f64, rh_int: f64) -> f64 {
        let mut r_total = r_si + r_se;
        for layer in layers {
            if layer.lambda_w_mk > 0.0 {
                r_total += layer.thickness_m / layer.lambda_w_mk;
            }
        }
        let delta_t = t_int_c - t_ext_c;
        if delta_t.abs() < f64::EPSILON {
            return 1.0;
        }
        let t_si = t_int_c - (r_si / r_total) * delta_t;
        let f_thermal = (t_si - t_ext_c) / delta_t;
        let rh = rh_int.clamp(0.0, 1.0);
        f_thermal - 0.10 * (rh - 0.5).max(0.0)
    }

    /// ✅️ Check interior surface temperature factor against limit 0.25 (DIN 4108-3).
    pub fn check_surface_temperature(layers: &[MoistureLayer], t_int_c: f64, t_ext_c: f64, rh_int: f64) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput { field: "layers".into() });
        }
        let f = interior_surface_temperature_factor(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, t_int_c, t_ext_c, rh_int);
        Ok(CheckResult::from_minimum(
            ClauseId::new("DIN 4108-3", "§6", "6.1"),
            Quantity::new(crate::document::QuantityKind::Dimensionless, f),
            Quantity::new(crate::document::QuantityKind::Dimensionless, F_RSI_MINIMUM),
            "interior surface temperature factor f_Rsi",
            AnnexChoice::De,
        ))
    }

    /// ✅️ Glaser dew-point check at every layer interface (DIN 4108-3).
    pub fn check_glaser_moisture(layers: &[MoistureLayer], t_int_c: f64, t_ext_c: f64, rh_int: f64) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput { field: "layers".into() });
        }
        let condenses = condensation_at_interfaces(layers, t_int_c, t_ext_c, rh_int);
        let margin = if condenses { 0.0 } else { 1.0 };
        Ok(CheckResult::from_minimum(
            ClauseId::new("DIN 4108-3", "§7", "7.1"),
            Quantity::new(crate::document::QuantityKind::Dimensionless, margin),
            Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            "Glaser interface dew-point margin",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 📊️ Tabulated design material properties (DIN 4108-4 Table 1).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MaterialDesign {
        pub lambda_dry_w_mk: f64,
        pub moisture_factor: f64,
    }

    /// 📋️ Lookup material design values by identifier.
    pub fn material_design(material: &str) -> Result<MaterialDesign, NormError> {
        let props = match material {
            "mineral_wool" => MaterialDesign { lambda_dry_w_mk: 0.035, moisture_factor: 1.10 },
            "glass_wool" => MaterialDesign { lambda_dry_w_mk: 0.037, moisture_factor: 1.10 },
            "eps" => MaterialDesign { lambda_dry_w_mk: 0.035, moisture_factor: 1.05 },
            "xps" => MaterialDesign { lambda_dry_w_mk: 0.034, moisture_factor: 1.05 },
            "pur" => MaterialDesign { lambda_dry_w_mk: 0.025, moisture_factor: 1.05 },
            "pir" => MaterialDesign { lambda_dry_w_mk: 0.023, moisture_factor: 1.05 },
            "wood_fibre" => MaterialDesign { lambda_dry_w_mk: 0.040, moisture_factor: 1.15 },
            "cellulose" => MaterialDesign { lambda_dry_w_mk: 0.040, moisture_factor: 1.15 },
            "concrete" => MaterialDesign { lambda_dry_w_mk: 2.10, moisture_factor: 1.20 },
            "aerated_concrete" => MaterialDesign { lambda_dry_w_mk: 0.16, moisture_factor: 1.25 },
            "brick" => MaterialDesign { lambda_dry_w_mk: 0.81, moisture_factor: 1.15 },
            "sand_lime_brick" => MaterialDesign { lambda_dry_w_mk: 0.99, moisture_factor: 1.15 },
            "timber" => MaterialDesign { lambda_dry_w_mk: 0.13, moisture_factor: 1.20 },
            "plywood" => MaterialDesign { lambda_dry_w_mk: 0.15, moisture_factor: 1.20 },
            "gypsum_plaster" => MaterialDesign { lambda_dry_w_mk: 0.51, moisture_factor: 1.10 },
            "lime_plaster" => MaterialDesign { lambda_dry_w_mk: 0.70, moisture_factor: 1.10 },
            "clay_plaster" => MaterialDesign { lambda_dry_w_mk: 0.91, moisture_factor: 1.10 },
            _ => {
                return Err(NormError::InvalidValue { field: "material".into(), reason: format!("unknown material: {material}") });
            }
        };
        Ok(props)
    }

    /// 📊️ Design thermal conductivity λ = λ_dry · moisture_factor (DIN 4108-4).
    pub fn design_lambda(lambda_dry: f64, moisture_factor: f64) -> f64 {
        lambda_dry * moisture_factor
    }

    /// 📊️ Design λ from catalog material name.
    pub fn design_lambda_for_material(material: &str) -> Result<f64, NormError> {
        let m = material_design(material)?;
        Ok(design_lambda(m.lambda_dry_w_mk, m.moisture_factor))
    }

    /// ✅️ Verify design value within tabulated bounds.
    pub fn check_design_lambda(material: &str, lambda_design: f64) -> Result<CheckResult, NormError> {
        let limit = design_lambda_for_material(material)?;
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN 4108-4", "Table 1", "λ"),
            Quantity::new(crate::document::QuantityKind::ThermalConductivity, lambda_design),
            Quantity::new(crate::document::QuantityKind::ThermalConductivity, limit),
            "design thermal conductivity",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part4

// #region 🔖️Part5
pub mod part_5 {
    use super::*;
    use crate::artifacts::din4108::standards::v1::subsets::any::schema::part_2::{total_resistance, u_value_from_resistance, Layer};

    /// ☀️ Peak summer heat flux through opaque element [W/m²].
    pub fn peak_summer_heat_flux_w_m2(layers: &[Layer], climate: ClimateZoneDe, t_int_c: f64, solar_absorptance: f64, irradiance_w_m2: f64) -> f64 {
        let u = u_value_from_resistance(total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        let t_summer = climate.summer_design_temperature_c();
        let conductive = u * (t_summer - t_int_c).max(0.0);
        conductive + solar_absorptance * irradiance_w_m2 * 0.04
    }

    /// 🌡️ Climate-dependent peak heat flux limit [W/m²] (DIN 4108-5 simplified).
    pub fn summer_heat_flux_limit_w_m2(climate: ClimateZoneDe) -> f64 {
        let table = [TableEntry1D { x: 26.0, y: 45.0 }, TableEntry1D { x: 28.0, y: 50.0 }, TableEntry1D { x: 30.0, y: 55.0 }, TableEntry1D { x: 32.0, y: 60.0 }];
        table_lookup_linear(&table, climate.summer_design_temperature_c())
    }

    /// ✅️ Check summer heat protection per DIN 4108-5.
    pub fn check_summer_heat_protection(layers: &[Layer], climate: ClimateZoneDe, t_int_c: f64, solar_absorptance: f64, irradiance_w_m2: f64) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput { field: "layers".into() });
        }
        let flux = peak_summer_heat_flux_w_m2(layers, climate, t_int_c, solar_absorptance, irradiance_w_m2);
        let limit = summer_heat_flux_limit_w_m2(climate);
        Ok(CheckResult::from_utilization(ClauseId::new("DIN 4108-5", "§4", "4.1"), Quantity::new(crate::document::QuantityKind::Power, flux), Quantity::new(crate::document::QuantityKind::Power, limit), "peak summer heat flux", AnnexChoice::De))
    }
}
// #endregion 🔖️Part5

// #region 🔖️Part6
pub mod part_6 {
    use super::*;
    use crate::artifacts::din4108::standards::v1::subsets::any::schema::part_2::{total_resistance, u_value_from_resistance, Layer};

    /// 🔗️ U-value including linear thermal bridge correction U' = U + Σ(ψ·l) [W/(m²K)].
    pub fn u_value_with_thermal_bridges(u_element: f64, psi_times_l_sum: f64) -> f64 {
        u_element + psi_times_l_sum
    }

    /// ✅️ U-value proof per DIN 4108-6.
    pub fn check_u_value(layers: &[Layer], limit_u: f64) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput { field: "layers".into() });
        }
        let u = u_value_from_resistance(total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        Ok(CheckResult::from_utilization(ClauseId::new("DIN 4108-6", "§5", "5.1"), Quantity::u_value_w_m2k(u), Quantity::u_value_w_m2k(limit_u), "component U-value", AnnexChoice::De))
    }

    /// ✅️ U-value proof with thermal bridge correction ψ·l per DIN 4108-6 §5.
    pub fn check_u_value_with_bridges(layers: &[Layer], psi_times_l_sum: f64, limit_u: f64) -> Result<CheckResult, NormError> {
        if layers.is_empty() {
            return Err(NormError::IncompleteInput { field: "layers".into() });
        }
        let u_element = u_value_from_resistance(total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        let u = u_value_with_thermal_bridges(u_element, psi_times_l_sum);
        Ok(CheckResult::from_utilization(ClauseId::new("DIN 4108-6", "§5", "5.2"), Quantity::u_value_w_m2k(u), Quantity::u_value_w_m2k(limit_u), "component U-value with thermal bridges", AnnexChoice::De))
    }
}
// #endregion 🔖️Part6

// #region 🔖️Part7
pub mod part_7 {
    use super::*;

    /// 🌬️ Airtightness class per DIN 4108-7.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AirtightnessClass {
        Class1,
        Class2,
        Class3,
    }

    impl AirtightnessClass {
        pub fn n50_limit_h(self) -> f64 {
            match self {
                Self::Class1 => 1.0,
                Self::Class2 => 3.0,
                Self::Class3 => 6.0,
            }
        }
    }

    /// ✅️ Check blower-door n50 against class limit.
    pub fn check_airtightness(n50_measured: f64, class: AirtightnessClass) -> CheckResult {
        let limit = class.n50_limit_h();
        CheckResult::from_utilization(
            ClauseId::new("DIN 4108-7", "§4", "4.2"),
            Quantity::new(crate::document::QuantityKind::AirPermeability, n50_measured),
            Quantity::new(crate::document::QuantityKind::AirPermeability, limit),
            "n50 airtightness",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖️Part7

// #region 🔖️Part8
pub mod part_8 {
    use super::*;

    /// 📦️ Standard construction catalog entry (DIN 4108-8).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CatalogEntry {
        pub id: &'static str,
        pub description: &'static str,
        pub u_typical_w_m2k: f64,
        pub r_typical_m2k_w: f64,
    }

    const CATALOG: &[CatalogEntry] = &[
        CatalogEntry { id: "AW-01", description: "solid brick wall with ETICS", u_typical_w_m2k: 0.24, r_typical_m2k_w: 4.2 },
        CatalogEntry { id: "AW-02", description: "reinforced concrete wall with external insulation", u_typical_w_m2k: 0.22, r_typical_m2k_w: 4.5 },
        CatalogEntry { id: "AW-03", description: "timber frame wall with mineral wool", u_typical_w_m2k: 0.18, r_typical_m2k_w: 5.6 },
        CatalogEntry { id: "AD-01", description: "pitched roof with mineral wool", u_typical_w_m2k: 0.20, r_typical_m2k_w: 5.0 },
        CatalogEntry { id: "AD-02", description: "flat roof with PUR insulation", u_typical_w_m2k: 0.18, r_typical_m2k_w: 5.6 },
        CatalogEntry { id: "BF-01", description: "basement ceiling to unheated cellar", u_typical_w_m2k: 0.30, r_typical_m2k_w: 3.3 },
        CatalogEntry { id: "FF-01", description: "ground floor slab on grade", u_typical_w_m2k: 0.35, r_typical_m2k_w: 2.9 },
        CatalogEntry { id: "FE-01", description: "wood-aluminium window triple glazing", u_typical_w_m2k: 0.95, r_typical_m2k_w: 1.05 },
    ];

    /// 🔍️ Lookup catalog entry by component reference id.
    pub fn catalog_entry(id: &str) -> Result<&'static CatalogEntry, NormError> {
        CATALOG.iter().find(|e| e.id == id).ok_or_else(|| NormError::InvalidValue { field: "catalog_id".into(), reason: format!("unknown catalog reference: {id}") })
    }

    /// ✅️ Verify computed U-value against catalog reference (DIN 4108-8).
    pub fn check_against_catalog(id: &str, u_computed: f64) -> Result<CheckResult, NormError> {
        let entry = catalog_entry(id)?;
        Ok(CheckResult::from_utilization(ClauseId::new("DIN 4108-8", "Table 1", id), Quantity::u_value_w_m2k(u_computed), Quantity::u_value_w_m2k(entry.u_typical_w_m2k), format!("catalog reference {}", entry.description), AnnexChoice::De))
    }
}
// #endregion 🔖️Part8

// #region 🔖️Part10
pub mod part_10 {
    use super::*;

    /// 🏷️ Factory-made thermal insulation application type codes (DIN 4108-10 Table 1).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ApplicationType {
        Dad,
        Daa,
        Duk,
        Dz,
        Di,
        Deo,
    }

    /// 📋️ Human-readable usage per application type (DIN 4108-10 Table 1).
    pub fn application_description(application: ApplicationType) -> &'static str {
        match application {
            ApplicationType::Dad => "roof insulation above rafters",
            ApplicationType::Daa => "roof insulation between rafters",
            ApplicationType::Duk => "insulation under screed",
            ApplicationType::Dz => "perimeter insulation",
            ApplicationType::Di => "interior insulation",
            ApplicationType::Deo => "external wall insulation with render (ETICS)",
        }
    }

    /// 🔩️ Compressive-strength / dimensional-stability application classes (DIN 4108-10), ordered dm < dk < dg by increasing load-bearing demand.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ApplicationClass {
        Dm,
        Dk,
        Dg,
    }

    /// 📋️ Minimum required application class per usage (DIN 4108-10 Table 1).
    pub fn minimum_class(application: ApplicationType) -> ApplicationClass {
        match application {
            ApplicationType::Dad | ApplicationType::Daa | ApplicationType::Di => ApplicationClass::Dm,
            ApplicationType::Deo => ApplicationClass::Dk,
            ApplicationType::Duk | ApplicationType::Dz => ApplicationClass::Dg,
        }
    }

    /// ✅️ Check that a declared product application class is admissible for its declared usage (DIN 4108-10).
    pub fn check_application_class(application: ApplicationType, declared_class: ApplicationClass) -> CheckResult {
        let required = minimum_class(application);
        let admissible = declared_class >= required;
        CheckResult {
            clause: ClauseId::new("DIN 4108-10", "Table 1", "1.1"),
            status: if admissible { crate::document::CheckStatus::Pass } else { crate::document::CheckStatus::Fail },
            computed: Quantity::new(crate::document::QuantityKind::Dimensionless, declared_class as i32 as f64),
            limit: Quantity::new(crate::document::QuantityKind::Dimensionless, required as i32 as f64),
            utilization: if admissible { 1.0 } else { 2.0 },
            message: format!("application class {declared_class:?} for {} (requires >= {required:?})", application_description(application)),
            annex: AnnexChoice::De,
        }
    }
}
// #endregion 🔖️Part10

// #region 🔖️BB2
pub mod bb_2 {
    use super::*;

    /// 🌉️ Beiblatt-2-conform detail allowance ΔU_WB [W/(m²K)] when thermal bridges follow standard-conforming details.
    pub const DELTA_U_WB_CONFORM_W_M2K: f64 = 0.05;

    /// 🌉️ Blanket flat-rate surcharge ΔU_WB [W/(m²K)] applied when details are not Beiblatt-2-conform.
    pub const DELTA_U_WB_FLAT_RATE_W_M2K: f64 = 0.10;

    /// 📐️ Area-normalised thermal-bridge surcharge ΔU_WB,actual = Σ(ψ·l) / A [W/(m²K)] (DIN 4108 Beiblatt 2).
    pub fn delta_u_wb_actual_w_m2k(psi_l_sum_w_k: f64, envelope_area_m2: f64) -> f64 {
        if envelope_area_m2 <= 0.0 {
            return f64::INFINITY;
        }
        psi_l_sum_w_k / envelope_area_m2
    }

    /// ✅️ Beiblatt 2 equivalence check: ΔU_WB,actual against the conform detail allowance, or the flat-rate surcharge if details are not Beiblatt-2-conform.
    pub fn check_beiblatt_2_equivalence(psi_l_sum_w_k: f64, envelope_area_m2: f64, details_conform: bool) -> Result<CheckResult, NormError> {
        if envelope_area_m2 <= 0.0 {
            return Err(NormError::InvalidValue { field: "envelope_area_m2".into(), reason: "must be positive".into() });
        }
        let actual = delta_u_wb_actual_w_m2k(psi_l_sum_w_k, envelope_area_m2);
        let limit = if details_conform { DELTA_U_WB_CONFORM_W_M2K } else { DELTA_U_WB_FLAT_RATE_W_M2K };
        Ok(CheckResult::from_utilization(ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"), Quantity::u_value_w_m2k(actual), Quantity::u_value_w_m2k(limit), "thermal bridge surcharge ΔU_WB (Beiblatt 2 equivalence)", AnnexChoice::De))
    }
}
// #endregion 🔖️BB2
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    fn sample_wall() -> Vec<part_2::Layer> {
        vec![part_2::Layer { thickness_m: 0.24, lambda_w_mk: 0.81 }, part_2::Layer { thickness_m: 0.14, lambda_w_mk: 0.035 }]
    }

    fn sample_moisture_wall() -> Vec<part_3::MoistureLayer> {
        vec![part_3::MoistureLayer { thickness_m: 0.24, lambda_w_mk: 0.81, mu: 15.0 }, part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 }]
    }

    #[semio_framework_async_macros::async_test]
    fn worked_example_u_value_known_wall() {
        let layers = sample_wall();
        let r = part_2::total_resistance(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
        let u = part_2::u_value_from_resistance(r);
        assert!((u - 0.224).abs() < 0.01, "U = {u}, expected ~0.224");
        assert!((r - 4.466).abs() < 0.02, "R = {r}, expected ~4.466");
    }

    #[semio_framework_async_macros::async_test]
    fn worked_example_f_rsi_above_minimum() {
        let layers = sample_moisture_wall();
        let f = part_3::interior_surface_temperature_factor(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.5);
        assert!(f > F_RSI_MINIMUM, "f_Rsi = {f}, must exceed {F_RSI_MINIMUM}");
        let check = part_3::check_surface_temperature(&layers, 20.0, -14.0, 0.5).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    fn worked_example_glaser_no_condensation_insulated_wall() {
        let layers = sample_moisture_wall();
        assert!(!part_3::condensation_at_interfaces(&layers, 20.0, -14.0, 0.5));
        let check = part_3::check_glaser_moisture(&layers, 20.0, -14.0, 0.5).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    fn worked_example_magnus_saturation_at_zero_c() {
        let e = part_3::saturation_vapor_pressure_pa(0.0);
        assert!((e - 611.2).abs() < 1.0, "e_sat(0°C) = {e}");
    }

    #[semio_framework_async_macros::async_test]
    fn worked_example_vapor_resistance_formula() {
        let layer = part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 };
        let r_mu = part_3::vapor_resistance(&layer);
        let expected = 0.14 / (1.3 * 0.035);
        assert!((r_mu - expected).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn part_2_colder_zone_allows_higher_u_limit() {
        let limit_warm = part_2::climate_adjusted_u_limit(part_2::BuildingCategory::Residential, ClimateZoneDe::Zone4);
        let limit_cold = part_2::climate_adjusted_u_limit(part_2::BuildingCategory::Residential, ClimateZoneDe::Zone1);
        assert!(limit_cold > limit_warm, "zone1={limit_cold}, zone4={limit_warm}");
        assert!((limit_cold - 0.308).abs() < 0.01);
    }

    #[semio_framework_async_macros::async_test]
    fn part_4_mineral_wool_lambda() {
        let r = part_4::check_design_lambda("mineral_wool", 0.038).unwrap();
        assert_eq!(r.status, crate::document::CheckStatus::Pass);
        let design = part_4::design_lambda_for_material("mineral_wool").unwrap();
        assert!((design - 0.0385).abs() < 0.001);
    }

    #[semio_framework_async_macros::async_test]
    fn part_4_has_fifteen_plus_materials() {
        let materials = ["mineral_wool", "glass_wool", "eps", "xps", "pur", "pir", "wood_fibre", "cellulose", "concrete", "aerated_concrete", "brick", "sand_lime_brick", "timber", "plywood", "gypsum_plaster", "lime_plaster", "clay_plaster"];
        assert!(materials.len() >= 15);
        for m in materials {
            assert!(part_4::material_design(m).is_ok(), "missing {m}");
        }
    }

    #[semio_framework_async_macros::async_test]
    fn part_5_summer_heat_zone_dependent() {
        let layers = sample_wall();
        let flux_z2 = part_5::peak_summer_heat_flux_w_m2(&layers, ClimateZoneDe::Zone2, 26.0, 0.6, 600.0);
        let flux_z4 = part_5::peak_summer_heat_flux_w_m2(&layers, ClimateZoneDe::Zone4, 26.0, 0.6, 600.0);
        assert!(flux_z4 > flux_z2);
        let check = part_5::check_summer_heat_protection(&layers, ClimateZoneDe::Zone2, 26.0, 0.6, 600.0).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    fn part_6_thermal_bridge_increases_u() {
        let layers = sample_wall();
        let u_element = part_2::u_value_from_resistance(part_2::total_resistance(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        let u_bridged = part_6::u_value_with_thermal_bridges(u_element, 0.05);
        assert!((u_bridged - (u_element + 0.05)).abs() < 1e-9);
        let check = part_6::check_u_value_with_bridges(&layers, 0.05, 0.35).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    fn part_8_catalog_lookup() {
        let entry = part_8::catalog_entry("AW-01").unwrap();
        assert!((entry.u_typical_w_m2k - 0.24).abs() < 0.01);
        let u = part_2::u_value_from_resistance(part_2::total_resistance(&sample_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        let check = part_8::check_against_catalog("AW-01", u).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    fn part_1_plausibility_flags_implausible_u_value() {
        let layers = sample_wall();
        let ok = part_1::check_input_plausibility(&layers, 0.224);
        assert_eq!(ok.status, crate::document::CheckStatus::Pass);
        let bad = part_1::check_input_plausibility(&layers, 12.0);
        assert_eq!(bad.status, crate::document::CheckStatus::Fail);
        let na = part_1::check_input_plausibility(&[], 0.3);
        assert_eq!(na.status, crate::document::CheckStatus::NotApplicable);
    }

    #[semio_framework_async_macros::async_test]
    fn part_10_application_class_admissibility() {
        let admissible = part_10::check_application_class(part_10::ApplicationType::Deo, part_10::ApplicationClass::Dk);
        assert_eq!(admissible.status, crate::document::CheckStatus::Pass);
        let inadmissible = part_10::check_application_class(part_10::ApplicationType::Duk, part_10::ApplicationClass::Dm);
        assert_eq!(inadmissible.status, crate::document::CheckStatus::Fail);
        assert_eq!(part_10::minimum_class(part_10::ApplicationType::Duk), part_10::ApplicationClass::Dg);
    }

    #[semio_framework_async_macros::async_test]
    fn bb2_worked_example_conform_details_pass() {
        let psi_l_sum = 18.0;
        let area = 400.0;
        let delta = bb_2::delta_u_wb_actual_w_m2k(psi_l_sum, area);
        assert!((delta - 0.045).abs() < 1e-9, "delta = {delta}");
        let check = bb_2::check_beiblatt_2_equivalence(psi_l_sum, area, true).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[semio_framework_async_macros::async_test]
    fn bb2_non_conform_details_fall_back_to_flat_rate_surcharge() {
        let psi_l_sum = 32.0;
        let area = 400.0;
        let check = bb_2::check_beiblatt_2_equivalence(psi_l_sum, area, false).unwrap();
        assert!((check.limit.value - bb_2::DELTA_U_WB_FLAT_RATE_W_M2K).abs() < 1e-9);
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
        let over_flat_rate = bb_2::check_beiblatt_2_equivalence(45.0, area, false).unwrap();
        assert_eq!(over_flat_rate.status, crate::document::CheckStatus::Fail);
    }
}
//#endregion 🧪️ComplianceHelpersTests
