//! 🧬️ Din18599 artifact schema — every field of the artifact with its state class.

use crate::artifacts::din18599::{Din18599ClimateChild, MonthlyClimate, UseClass};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Din18599 artifact state across the artifact and presence lanes. `climate` mirrors
/// `Din18599Snapshot`'s composed `s.stdio.semio.table` child slot (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — `to_snapshot`/`from_snapshot` copy the
/// handle across verbatim, same as `➗️mathematical`'s `MathematicalArtifact`/en1990's `En1990Artifact`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Artifact {
    #[state(artifact)]
    pub use_class: UseClass,
    #[state(artifact)]
    pub heated_area_m2: f64,
    #[state(artifact)]
    pub occupants: u32,
    #[state(artifact)]
    pub h_t: f64,
    #[state(artifact)]
    pub h_v: f64,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub climate: Din18599ClimateChild,
    #[state(artifact)]
    pub internal_gains_w_m2: f64,
    #[state(artifact)]
    pub solar_gains_kwh: f64,
    #[state(artifact)]
    pub system_losses_kwh: f64,
    #[state(artifact)]
    pub renewable_kwh: f64,
    #[state(artifact)]
    pub annual_limit_kwh: f64,
    #[state(artifact)]
    pub energy_carrier: String,
    #[state(artifact)]
    pub reference_q_p_kwh: f64,
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din18599Artifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> crate::artifacts::din18599::Din18599Snapshot {
        crate::artifacts::din18599::Din18599Snapshot {
            use_class: self.use_class,
            heated_area_m2: self.heated_area_m2,
            occupants: self.occupants,
            h_t: self.h_t,
            h_v: self.h_v,
            climate: self.climate.clone(),
            internal_gains_w_m2: self.internal_gains_w_m2,
            solar_gains_kwh: self.solar_gains_kwh,
            system_losses_kwh: self.system_losses_kwh,
            renewable_kwh: self.renewable_kwh,
            annual_limit_kwh: self.annual_limit_kwh,
            energy_carrier: self.energy_carrier.clone(),
            reference_q_p_kwh: self.reference_q_p_kwh,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: crate::artifacts::din18599::Din18599Snapshot) -> Self {
        Self {
            use_class: snapshot.use_class,
            heated_area_m2: snapshot.heated_area_m2,
            occupants: snapshot.occupants,
            h_t: snapshot.h_t,
            h_v: snapshot.h_v,
            climate: snapshot.climate,
            internal_gains_w_m2: snapshot.internal_gains_w_m2,
            solar_gains_kwh: snapshot.solar_gains_kwh,
            system_losses_kwh: snapshot.system_losses_kwh,
            renewable_kwh: snapshot.renewable_kwh,
            annual_limit_kwh: snapshot.annual_limit_kwh,
            energy_carrier: snapshot.energy_carrier.clone(),
            reference_q_p_kwh: snapshot.reference_q_p_kwh,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub async fn set_snapshot(&mut self, snapshot: crate::artifacts::din18599::Din18599Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din18599` — twenty handcrafted schema leaves.
pub async fn din18599_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.din18599",
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
    use crate::artifacts::din18599::{Din18599Diff, Din18599Mutation, Din18599Snapshot};
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
        async fn empty() -> Self {
            Self { snapshot: Din18599Snapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    use crate::artifacts::din18599::Din18599Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Din18599Parts {
        pub snapshot: Option<Din18599Snapshot>,
    }

    pub struct Din18599AnalyzerAnalysis;

    impl ArtifactAnalysis for Din18599AnalyzerAnalysis {
        type Parts = Din18599Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.din18599", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
use crate::artifacts::din16798::standards::v1::subsets::any::schema::part_3::residential_ventilation_rate;
/// 📐️ Pure DIN V 18599 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`. `part_1` through `part_12` operate on
/// `BalancingInputs` (a type alias for `Din18599Snapshot` defined at the artifact root — see
/// `crate::artifacts::din18599::BalancingInputs`), but remain per-metric pure helpers, not the
/// whole-document composition; `balance_annual`/`evaluate` (the actual snapshot-level conformance
/// law) live in `💡️inferences`. Depends on `din4108` and `din16798`'s relocated schema helpers for
/// the reference-building envelope/ventilation calculations.
use crate::artifacts::din18599::BalancingInputs;
use crate::artifacts::din4108::standards::v1::subsets::any::schema::part_2::{total_resistance, u_value_from_resistance, Layer};
use crate::artifacts::din4108::standards::v1::subsets::any::schema::{R_SE_WALL_M2K_W, R_SI_WALL_M2K_W};
use crate::document::{AnnexChoice, CheckResult, ClauseId, ClimateZoneDe, NormError, Quantity};

// #region 🔖️Shared
/// 🧱️ Transmission loss coefficient H_T [W/K].
pub async fn transmission_loss_coefficient(envelope_u_a: f64, thermal_bridge_psi_l: f64, ground_floor_u_a: f64) -> f64 {
    envelope_u_a + thermal_bridge_psi_l + ground_floor_u_a
}

/// 🌬️ Ventilation loss coefficient H_V [W/K] per DIN V 18599-2.
pub async fn ventilation_loss_coefficient(airflow_m3_h: f64, heat_recovery_eta: f64) -> f64 {
    let rho_cp = 0.34;
    let factor = 1.0 - heat_recovery_eta.clamp(0.0, 0.95);
    rho_cp * airflow_m3_h * factor
}

/// 📐️ Envelope area estimate for single-storey building [m²].
pub async fn envelope_area_m2(floor_area_m2: f64) -> f64 {
    3.0 * floor_area_m2
}

async fn heating_degree_hours(climate: &MonthlyClimate, theta_int: f64) -> f64 {
    let hours_per_month = [744.0, 672.0, 744.0, 720.0, 744.0, 720.0, 744.0, 744.0, 720.0, 744.0, 720.0, 744.0];
    climate.theta_e_c.iter().zip(hours_per_month).map(|(&theta_e, hours)| (theta_int - theta_e).max(0.0) * hours).sum()
}

async fn cooling_degree_hours(climate: &MonthlyClimate, theta_int_cool: f64) -> f64 {
    let hours_per_month = [744.0, 672.0, 744.0, 720.0, 744.0, 720.0, 744.0, 744.0, 720.0, 744.0, 720.0, 744.0];
    climate.theta_e_c.iter().zip(hours_per_month).map(|(&theta_e, hours)| (theta_e - theta_int_cool).max(0.0) * hours).sum()
}
// #endregion 🔖️Shared

// 📌️ Deviation from the original monolith: this was `impl BalancingInputs { pub fn
// reference_residential(..) }` — an inherent method. It needs `din4108_engine`/`din16798_engine`,
// which `din18599` (rs) must not depend on (rs sits below engine in every constitutional crate),
// so it moved here as a free function. See `crate::artifacts::din18599::Din18599Snapshot`'s `Default` impl for the
// precomputed numeric result of `reference_residential(ClimateZoneDe::Zone2, 100.0)`.
pub async fn reference_residential(zone: ClimateZoneDe, area_m2: f64) -> BalancingInputs {
    let occupants = ((area_m2 / 30.0).ceil() as u32).max(1);
    let wall_layers = reference_wall_layers();
    from_building(&wall_layers, area_m2, occupants, zone, 0.0).expect("reference building")
}

/// 🧱️ Build balancing inputs from envelope layers and occupancy (DIN V 18599-2/-7).
pub async fn from_building(wall_layers: &[Layer], floor_area_m2: f64, occupants: u32, climate: ClimateZoneDe, heat_recovery_eta: f64) -> Result<BalancingInputs, NormError> {
    if wall_layers.is_empty() {
        return Err(NormError::IncompleteInput { field: "wall_layers".into() });
    }
    if floor_area_m2 <= 0.0 {
        return Err(NormError::InvalidValue { field: "floor_area_m2".into(), reason: "must be positive".into() });
    }
    let r = total_resistance(wall_layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
    let u = u_value_from_resistance(r);
    let a_env = envelope_area_m2(floor_area_m2);
    let side = floor_area_m2.sqrt();
    let psi_l = 0.10 * side * 4.0;
    let h_t = transmission_loss_coefficient(u * a_env, 0.10 * psi_l, 0.15 * floor_area_m2);
    let airflow = residential_ventilation_rate(floor_area_m2, occupants);
    let h_v = ventilation_loss_coefficient(airflow, heat_recovery_eta);
    let solar = floor_area_m2 * MonthlyClimate::german_reference(climate).g_h_w_m2.iter().sum::<f64>() / 1000.0 * 0.6;
    Ok(BalancingInputs {
        use_class: UseClass::Residential,
        heated_area_m2: floor_area_m2,
        occupants,
        h_t,
        h_v,
        climate: crate::artifacts::din18599::din18599_climate_child_from_data(&MonthlyClimate::german_reference(climate)),
        internal_gains_w_m2: 3.5,
        solar_gains_kwh: solar,
        system_losses_kwh: 8.0 * floor_area_m2,
        renewable_kwh: 15.0 * floor_area_m2,
        annual_limit_kwh: 75.0 * floor_area_m2,
        energy_carrier: "natural_gas".into(),
        reference_q_p_kwh: tabular_specific_primary_energy_kwh_m2(UseClass::Residential) * floor_area_m2,
    })
}

pub async fn reference_wall_layers() -> Vec<Layer> {
    vec![Layer { thickness_m: 0.12, lambda_w_mk: 0.035 }, Layer { thickness_m: 0.24, lambda_w_mk: 0.77 }]
}

async fn transmission_losses_kwh(inputs: &BalancingInputs) -> f64 {
    let dh = heating_degree_hours(&crate::artifacts::din18599::din18599_climate(inputs), 19.0);
    inputs.h_t * dh / 1000.0
}

async fn ventilation_losses_kwh(inputs: &BalancingInputs) -> f64 {
    let dh = heating_degree_hours(&crate::artifacts::din18599::din18599_climate(inputs), 19.0);
    inputs.h_v * dh / 1000.0
}

async fn internal_gains_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.internal_gains_w_m2 * inputs.heated_area_m2 * 8760.0 / 1000.0 * 0.35
}

async fn solar_gains_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.solar_gains_kwh
}

async fn system_losses_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.system_losses_kwh
}

async fn net_heating_demand_kwh(inputs: &BalancingInputs) -> f64 {
    let q_t = transmission_losses_kwh(inputs);
    let q_v = ventilation_losses_kwh(inputs);
    let q_i = internal_gains_kwh(inputs);
    let q_s = solar_gains_kwh(inputs);
    (q_t + q_v - q_i - q_s).max(0.0)
}

async fn cooling_demand_kwh(inputs: &BalancingInputs) -> f64 {
    let cdh = cooling_degree_hours(&crate::artifacts::din18599::din18599_climate(inputs), 26.0);
    let gain_factor = 0.35;
    (inputs.h_t + inputs.h_v) * cdh * gain_factor / 1000.0
}

async fn dhw_demand_kwh(inputs: &BalancingInputs) -> f64 {
    inputs.occupants as f64 * 900.0
}

async fn primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
    let q_h = net_heating_demand_kwh(inputs);
    let q_c = cooling_demand_kwh(inputs);
    let q_w = dhw_demand_kwh(inputs);
    let f_p = primary_energy_factor(&inputs.energy_carrier);
    (q_h + q_c + q_w) * f_p + inputs.system_losses_kwh - inputs.renewable_kwh
}

async fn automation_factor(inputs: &BalancingInputs) -> f64 {
    match inputs.use_class {
        UseClass::Residential => 0.95,
        UseClass::Office => 0.90,
        UseClass::School => 0.92,
    }
}

/// ⚡️ Primary energy factor f_p per energy carrier (DIN V 18599-10 Table 12).
pub async fn primary_energy_factor(carrier: &str) -> f64 {
    match carrier {
        "natural_gas" => 1.1,
        "heating_oil" => 1.1,
        "lpg" => 1.1,
        "coal" => 1.1,
        "district_heat" => 0.7,
        "district_heat_chp" => 0.6,
        "electricity_grid" => 1.8,
        "electricity_renewable" => 0.0,
        "biomass" => 0.5,
        "heat_pump_electricity" => 1.8,
        _ => 1.1,
    }
}

/// 📐️ Reference area factor f_ref by use class (DIN V 18599-12 Table 2).
pub async fn reference_area_factor(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential => 1.00,
        UseClass::Office => 1.20,
        UseClass::School => 1.10,
    }
}

/// 📊️ Tabular specific primary energy q_P,tab [kWh/(m²a)] (DIN V 18599-12 Table 3).
pub async fn tabular_specific_primary_energy_kwh_m2(use_class: UseClass) -> f64 {
    match use_class {
        UseClass::Residential => 100.0,
        UseClass::Office => 95.0,
        UseClass::School => 85.0,
    }
}

// #region 🔖️Part1
pub mod part_1 {
    use super::*;

    /// ⚡️ One final-energy delivery by carrier, feeding the primary-energy aggregation (DIN V 18599-1 §6).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct FinalEnergyDelivery {
        pub carrier: String,
        pub q_f_kwh: f64,
    }

    /// ⚡️ Primary energy aggregation Q_p = Σ Q_f,i · f_p,i across final-energy carriers (DIN V 18599-1 §6, factors from -10 Table 12).
    pub async fn aggregate_primary_energy_kwh(deliveries: &[FinalEnergyDelivery]) -> f64 {
        deliveries.iter().map(|d| d.q_f_kwh * primary_energy_factor(&d.carrier)).sum()
    }

    async fn final_energy_breakdown(inputs: &BalancingInputs) -> Vec<FinalEnergyDelivery> {
        let q_f = net_heating_demand_kwh(inputs) + cooling_demand_kwh(inputs) + dhw_demand_kwh(inputs);
        vec![FinalEnergyDelivery { carrier: inputs.energy_carrier.clone(), q_f_kwh: q_f }]
    }

    /// ✅️ Primary-energy aggregation Q_p against the reference-building target (DIN V 18599-1 §6).
    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let q_p = aggregate_primary_energy_kwh(&final_energy_breakdown(inputs));
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-1", "§6", "6.1"),
            Quantity::new(crate::document::QuantityKind::Energy, q_p),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.reference_q_p_kwh),
            "primary energy aggregation Q_p vs. reference building target",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part1

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🔥️ Annual transmission losses Q_T [kWh/a] (DIN V 18599-2).
    pub async fn transmission_losses_kwh(inputs: &BalancingInputs) -> f64 {
        super::transmission_losses_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = transmission_losses_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-2", "§8", "8.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "transmission losses Q_T",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 🌬️ Annual ventilation losses Q_V [kWh/a] (DIN V 18599-3).
    pub async fn ventilation_losses_kwh(inputs: &BalancingInputs) -> f64 {
        super::ventilation_losses_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = ventilation_losses_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-3", "§7", "7.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "ventilation losses Q_V",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 💡️ Annual internal gains Q_I [kWh/a] (DIN V 18599-4).
    pub async fn internal_gains_kwh(inputs: &BalancingInputs) -> f64 {
        super::internal_gains_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = internal_gains_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-4", "§6", "6.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh * 2.0),
            "internal gains Q_I",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part4

// #region 🔖️Part5
pub mod part_5 {
    use super::*;

    /// ☀️ Annual solar gains Q_S [kWh/a] (DIN V 18599-5).
    pub async fn solar_gains_kwh(inputs: &BalancingInputs) -> f64 {
        super::solar_gains_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = solar_gains_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-5", "§6", "6.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh * 2.0),
            "solar gains Q_S",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part5

// #region 🔖️Part6
pub mod part_6 {
    use super::*;

    /// ⚙️ Annual system losses [kWh/a] (DIN V 18599-6).
    pub async fn system_losses_kwh(inputs: &BalancingInputs) -> f64 {
        super::system_losses_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = system_losses_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-6", "§8", "8.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "system losses",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part6

// #region 🔖️Part7
pub mod part_7 {
    use super::*;

    /// 🔥️ Net heating demand Q_H [kWh/a] (DIN V 18599-7).
    pub async fn net_heating_demand_kwh(inputs: &BalancingInputs) -> f64 {
        super::net_heating_demand_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = net_heating_demand_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-7", "§9", "9.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "net heating demand Q_H",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part7

// #region 🔖️Part8
pub mod part_8 {
    use super::*;

    /// ❄️ Annual cooling demand Q_C [kWh/a] from cooling degree hours (DIN V 18599-8).
    pub async fn cooling_demand_kwh(inputs: &BalancingInputs) -> f64 {
        super::cooling_demand_kwh(inputs)
    }

    pub async fn cooling_degree_hours(climate: &MonthlyClimate) -> f64 {
        super::cooling_degree_hours(climate, 26.0)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = cooling_demand_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-8", "§10", "10.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "cooling demand Q_C",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part8

// #region 🔖️Part9
pub mod part_9 {
    use super::*;

    /// 🚿️ Annual DHW demand Q_W [kWh/a] from occupants (DIN V 18599-9).
    pub async fn dhw_demand_kwh(inputs: &BalancingInputs) -> f64 {
        super::dhw_demand_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = dhw_demand_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-9", "§11", "11.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "DHW demand Q_W",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part9

// #region 🔖️Part10
pub mod part_10 {
    use super::*;

    /// ⚡️ Annual primary energy Q_P [kWh/a] (DIN V 18599-10).
    pub async fn primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
        super::primary_energy_kwh(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = primary_energy_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-10", "§12", "12.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "primary energy Q_P",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part10

// #region 🔖️Part11
pub mod part_11 {
    use super::*;

    /// 🎛️ Building automation factor (DIN V 18599-11).
    pub async fn automation_factor(inputs: &BalancingInputs) -> f64 {
        super::automation_factor(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = automation_factor(inputs);
        Ok(CheckResult::from_minimum(
            ClauseId::new("DIN V 18599-11", "§7", "7.1"),
            Quantity::new(crate::document::QuantityKind::Dimensionless, value),
            Quantity::new(crate::document::QuantityKind::Dimensionless, 0.85),
            "building automation factor",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part11

// #region 🔖️Part12
pub mod part_12 {
    use super::*;

    /// 📊️ Tabular method primary energy Q_P,tab [kWh/a] (DIN V 18599-12).
    pub async fn tabular_primary_energy_kwh(inputs: &BalancingInputs) -> f64 {
        let a_ref = inputs.heated_area_m2 * reference_area_factor(inputs.use_class);
        tabular_specific_primary_energy_kwh_m2(inputs.use_class) * a_ref * automation_factor(inputs)
    }

    pub async fn check(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
        let value = tabular_primary_energy_kwh(inputs);
        Ok(CheckResult::from_utilization(
            ClauseId::new("DIN V 18599-12", "§4", "4.1"),
            Quantity::new(crate::document::QuantityKind::Energy, value),
            Quantity::new(crate::document::QuantityKind::Energy, inputs.annual_limit_kwh),
            "tabular method reference",
            AnnexChoice::De,
        ))
    }
}
// #endregion 🔖️Part12

/// ✅️ Primary energy compliance gate.
pub async fn check_primary_energy(inputs: &BalancingInputs) -> Result<CheckResult, NormError> {
    part_10::check(inputs)
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    async fn reference_100m2_inputs() -> BalancingInputs {
        from_building(&reference_wall_layers(), 100.0, 4, ClimateZoneDe::Zone2, 0.0).unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn from_building_computes_h_t_from_u_value() {
        let inputs = reference_100m2_inputs();
        let r = total_resistance(&reference_wall_layers(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
        let u = u_value_from_resistance(r);
        let a_env = envelope_area_m2(100.0);
        let side = 10.0;
        let psi_l = 0.10 * side * 4.0;
        let expected_h_t = transmission_loss_coefficient(u * a_env, 0.10 * psi_l, 15.0);
        assert!((inputs.h_t - expected_h_t).abs() < 1e-6);
        assert!(inputs.h_t > 40.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_building_computes_h_v_from_ventilation() {
        let inputs = reference_100m2_inputs();
        let airflow = residential_ventilation_rate(100.0, 4);
        assert!((airflow - 120.0).abs() < 1e-9);
        let expected_h_v = ventilation_loss_coefficient(airflow, 0.0);
        assert!((inputs.h_v - expected_h_v).abs() < 1e-6);
        assert!((inputs.h_v - 40.8).abs() < 0.1);
    }

    #[semio_framework_async_macros::async_test]
    async fn reference_100m2_q_t_numeric() {
        let inputs = reference_100m2_inputs();
        let q_t = part_2::transmission_losses_kwh(&inputs);
        assert!((q_t - 11_054.56).abs() < 1.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn reference_100m2_q_v_numeric() {
        let inputs = reference_100m2_inputs();
        let q_v = part_3::ventilation_losses_kwh(&inputs);
        assert!((q_v - 4_896.01).abs() < 1.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn reference_100m2_q_p_numeric() {
        let inputs = reference_100m2_inputs();
        let q_p = part_10::primary_energy_kwh(&inputs);
        assert!((q_p - 19_608.96).abs() < 5.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn dhw_from_occupants_4_persons() {
        let inputs = reference_100m2_inputs();
        let q_w = part_9::dhw_demand_kwh(&inputs);
        assert!((q_w - 3600.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn cooling_degree_hours_zone2_positive() {
        let climate = MonthlyClimate::german_reference(ClimateZoneDe::Zone2);
        let cdh = part_8::cooling_degree_hours(&climate);
        assert!(cdh > 1000.0);
        let inputs = reference_100m2_inputs();
        let q_c = part_8::cooling_demand_kwh(&inputs);
        assert!(q_c > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn part_4_internal_gains_positive() {
        let inputs = reference_100m2_inputs();
        let q_i = part_4::internal_gains_kwh(&inputs);
        assert!(q_i > 1000.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn part_5_solar_gains_positive() {
        let inputs = reference_100m2_inputs();
        let q_s = part_5::solar_gains_kwh(&inputs);
        assert!(q_s > 0.0, "q_s={q_s}");
    }

    #[semio_framework_async_macros::async_test]
    async fn part_6_system_losses_scale_with_area() {
        let inputs = reference_100m2_inputs();
        let q_sys = part_6::system_losses_kwh(&inputs);
        assert!((q_sys - 800.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn part_11_automation_factor_residential() {
        let inputs = reference_100m2_inputs();
        assert!((part_11::automation_factor(&inputs) - 0.95).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn part_12_tabular_distinct_from_part_10() {
        let inputs = reference_100m2_inputs();
        let q_p = part_10::primary_energy_kwh(&inputs);
        let q_tab = part_12::tabular_primary_energy_kwh(&inputs);
        assert!((q_tab - 9500.0).abs() < 1.0, "q_tab={q_tab}");
        assert!((q_p - q_tab).abs() > 100.0, "tabular must differ from detailed: q_p={q_p}, q_tab={q_tab}");
    }

    #[semio_framework_async_macros::async_test]
    async fn primary_energy_factor_table_cited() {
        assert!((primary_energy_factor("natural_gas") - 1.1).abs() < 1e-9);
        assert!((primary_energy_factor("district_heat") - 0.7).abs() < 1e-9);
        assert!((primary_energy_factor("electricity_grid") - 1.8).abs() < 1e-9);
        assert!((reference_area_factor(UseClass::Office) - 1.20).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn part_1_primary_energy_aggregation_worked_example() {
        let deliveries = vec![part_1::FinalEnergyDelivery { carrier: "natural_gas".into(), q_f_kwh: 10_000.0 }, part_1::FinalEnergyDelivery { carrier: "electricity_grid".into(), q_f_kwh: 2_000.0 }];
        let q_p = part_1::aggregate_primary_energy_kwh(&deliveries);
        assert!((q_p - 14_600.0).abs() < 1e-9, "q_p = {q_p}, expected 10000*1.1 + 2000*1.8 = 14600");
    }

    #[semio_framework_async_macros::async_test]
    async fn part_8_cooling_demand_numeric_worked_example() {
        let inputs = reference_100m2_inputs();
        let cdh = part_8::cooling_degree_hours(&crate::artifacts::din18599::din18599_climate(&inputs));
        let expected_q_c = (inputs.h_t + inputs.h_v) * cdh * 0.35 / 1000.0;
        let q_c = part_8::cooling_demand_kwh(&inputs);
        assert!((q_c - expected_q_c).abs() < 1e-6, "q_c = {q_c}, expected {expected_q_c}");
        assert!((q_c - 69.23).abs() < 1.0, "q_c = {q_c}, expected ~69.23 kWh");
    }

    #[semio_framework_async_macros::async_test]
    async fn reference_residential_matches_from_building() {
        let via_helper = reference_residential(ClimateZoneDe::Zone2, 100.0);
        let via_from_building = reference_100m2_inputs();
        assert_eq!(via_helper, via_from_building);
    }
}
//#endregion 🧪️ComplianceHelpersTests
