//! ⚙️ DIN 4108 app — headless compute (constitutional: engine).

use crate::artifacts::din4108::Document;
use crate::artifacts::din4108::mutations::Din4108Mutation;
use crate::document::{table_lookup_linear, AnnexChoice, CheckReport, CheckResult, ClauseId, ClimateZoneDe, NormError, NormFamily, NormFamilyId, NormHost, Quantity, TableEntry1D};

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
    use crate::artifacts::din4108::engine::part_2::{total_resistance, u_value_from_resistance, Layer};

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
    use crate::artifacts::din4108::engine::part_2::{total_resistance, u_value_from_resistance, Layer};

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

/// 📋️ Run all applicable DIN 4108 checks for a typical opaque wall.
pub fn check_opaque_wall(category: part_2::BuildingCategory, layers: &[part_2::Layer], climate: ClimateZoneDe, airtightness_n50: f64) -> Result<CheckReport, NormError> {
    check_opaque_wall_with_bridges(category, layers, climate, airtightness_n50, 0.02)
}

fn moisture_layers_from_wall(layers: &[part_2::Layer], mu_exterior: f64, mu_interior: f64) -> Vec<part_3::MoistureLayer> {
    layers
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let mu = if i == 0 { mu_exterior } else { mu_interior };
            part_3::MoistureLayer { thickness_m: l.thickness_m, lambda_w_mk: l.lambda_w_mk, mu }
        })
        .collect()
}

fn parse_airtightness_class(class: &str) -> part_7::AirtightnessClass {
    match class.to_ascii_lowercase().as_str() {
        "class1" | "1" => part_7::AirtightnessClass::Class1,
        "class3" | "3" => part_7::AirtightnessClass::Class3,
        _ => part_7::AirtightnessClass::Class2,
    }
}

fn parse_application_type(value: &str) -> part_10::ApplicationType {
    match value.to_ascii_uppercase().as_str() {
        "DAA" => part_10::ApplicationType::Daa,
        "DUK" => part_10::ApplicationType::Duk,
        "DZ" => part_10::ApplicationType::Dz,
        "DI" => part_10::ApplicationType::Di,
        "DEO" => part_10::ApplicationType::Deo,
        _ => part_10::ApplicationType::Dad,
    }
}

fn parse_application_class(value: &str) -> part_10::ApplicationClass {
    match value.to_ascii_lowercase().as_str() {
        "dk" => part_10::ApplicationClass::Dk,
        "dg" => part_10::ApplicationClass::Dg,
        _ => part_10::ApplicationClass::Dm,
    }
}

/// 📋️ Opaque wall checks including thermal bridge correction ψ·l [W/(m²K)].
pub fn check_opaque_wall_with_bridges(category: part_2::BuildingCategory, layers: &[part_2::Layer], climate: ClimateZoneDe, airtightness_n50: f64, psi_times_l_sum: f64) -> Result<CheckReport, NormError> {
    check_full_envelope(category, layers, climate, airtightness_n50, psi_times_l_sum, 0.5, 20.0, 0.6, 600.0, 15.0, 1.3, "mineral_wool", "AW-01", "class2", 100.0, true, "DEO", "dk")
}

/// 📋️ Full DIN 4108 parts 1, 2–8, 10, and Beiblatt 2 envelope compliance check.
#[allow(clippy::too_many_arguments, reason = "one argument per parameter the published clause formula itself names; bundling them into a struct would break the 1:1 reading against the standard")]
pub fn check_full_envelope(
    category: part_2::BuildingCategory,
    layers: &[part_2::Layer],
    climate: ClimateZoneDe,
    airtightness_n50: f64,
    psi_times_l_sum: f64,
    rh_int: f64,
    t_int_c: f64,
    solar_absorptance: f64,
    irradiance_w_m2: f64,
    moisture_mu_exterior: f64,
    moisture_mu_interior: f64,
    material_id: &str,
    catalog_id: &str,
    airtightness_class: &str,
    envelope_area_m2: f64,
    bb2_details_conform: bool,
    application_type: &str,
    declared_application_class: &str,
) -> Result<CheckReport, NormError> {
    let mut report = CheckReport::default();
    for part in [part_1::NormPart::Part1, part_1::NormPart::Part2, part_1::NormPart::Part3, part_1::NormPart::Part4, part_1::NormPart::Part5, part_1::NormPart::Part6, part_1::NormPart::Part7, part_1::NormPart::Part8] {
        report.push(part_1::scope_check(part, part_1::BuildingElement::OpaqueWall));
    }
    let u = part_2::u_value_from_resistance(part_2::total_resistance(layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
    report.push(part_1::check_input_plausibility(layers, u));
    let limit = part_2::climate_adjusted_u_limit(category, climate);
    report.push(part_2::check_minimum_thermal_protection(category, layers, climate)?);
    let moisture_layers = moisture_layers_from_wall(layers, moisture_mu_exterior, moisture_mu_interior);
    let t_ext = climate.design_external_temperature_c();
    report.push(part_3::check_surface_temperature(&moisture_layers, t_int_c, t_ext, rh_int)?);
    report.push(part_3::check_glaser_moisture(&moisture_layers, t_int_c, t_ext, rh_int)?);
    let lambda_design = part_4::design_lambda_for_material(material_id)?;
    if let Some(insulation) = layers.last() {
        report.push(part_4::check_design_lambda(material_id, insulation.lambda_w_mk)?);
    } else {
        report.push(part_4::check_design_lambda(material_id, lambda_design)?);
    }
    report.push(part_5::check_summer_heat_protection(layers, climate, t_int_c, solar_absorptance, irradiance_w_m2)?);
    report.push(part_6::check_u_value_with_bridges(layers, psi_times_l_sum, limit)?);
    report.push(part_7::check_airtightness(airtightness_n50, parse_airtightness_class(airtightness_class)));
    report.push(part_8::check_against_catalog(catalog_id, u)?);
    report.push(part_10::check_application_class(parse_application_type(application_type), parse_application_class(declared_application_class)));
    report.push(bb_2::check_beiblatt_2_equivalence(psi_times_l_sum, envelope_area_m2, bb2_details_conform)?);
    Ok(report)
}

// #region 🔖️Session
fn parse_category(category: &str) -> part_2::BuildingCategory {
    match category.to_ascii_lowercase().as_str() {
        "office" => part_2::BuildingCategory::Office,
        "school" => part_2::BuildingCategory::School,
        "industrial" => part_2::BuildingCategory::Industrial,
        _ => part_2::BuildingCategory::Residential,
    }
}

pub fn evaluate(document: &Document) -> CheckReport {
    let layers: Vec<part_2::Layer> = document.layers.iter().map(|layer| part_2::Layer { thickness_m: layer.thickness_m, lambda_w_mk: layer.lambda_w_mk }).collect();
    check_full_envelope(
        parse_category(&document.category),
        &layers,
        document.climate,
        document.airtightness_n50,
        document.psi_times_l_sum,
        document.rh_int,
        document.t_int_c,
        document.solar_absorptance,
        document.irradiance_w_m2,
        document.moisture_mu_exterior,
        document.moisture_mu_interior,
        &document.material_id,
        &document.catalog_id,
        &document.airtightness_class,
        document.envelope_area_m2,
        document.bb2_details_conform,
        &document.application_type,
        &document.declared_application_class,
    )
    .unwrap_or_else(|err| {
        let mut report = CheckReport::default();
        report.push(CheckResult::from_utilization(ClauseId::new("DIN 4108", "input", "1"), Quantity::new(crate::document::QuantityKind::Dimensionless, 2.0), Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0), err.to_string(), AnnexChoice::De));
        report
    })
}

pub struct Din4108Family;

impl NormFamily for Din4108Family {
    type Document = Document;
    type Mutation = Din4108Mutation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::Din4108
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}



//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent Din4108 artifact engine — owns the projection; every transition is a mutation.
pub struct Din4108Engine {
    projection: Document,
}

impl Din4108Engine {
    pub fn new(projection: Document) -> Self {
        Self { projection }
    }

    pub fn into_projection(self) -> Document {
        self.projection
    }
}

impl protocol::ArtifactEngine for Din4108Engine {
    type Projection = Document;
    type Mutation = Din4108Mutation;
    type Diff = crate::artifacts::din4108::diff::Diff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = protocol::Mutation::diff(mutation, &self.projection);
        self.projection = vcs::apply_mutation(&self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        protocol::Mutation::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine

pub type Host = NormHost<Din4108Family>;
// #endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wall() -> Vec<part_2::Layer> {
        vec![part_2::Layer { thickness_m: 0.24, lambda_w_mk: 0.81 }, part_2::Layer { thickness_m: 0.14, lambda_w_mk: 0.035 }]
    }

    fn sample_moisture_wall() -> Vec<part_3::MoistureLayer> {
        vec![part_3::MoistureLayer { thickness_m: 0.24, lambda_w_mk: 0.81, mu: 15.0 }, part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 }]
    }

    #[test]
    fn opaque_wall_passes_din_4108_suite() {
        let report = check_opaque_wall(part_2::BuildingCategory::Residential, &sample_wall(), ClimateZoneDe::Zone2, 2.5).expect("inputs complete");
        assert!(report.all_pass(), "checks: {:?}", report.checks);
    }

    #[test]
    fn worked_example_u_value_known_wall() {
        let layers = sample_wall();
        let r = part_2::total_resistance(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W);
        let u = part_2::u_value_from_resistance(r);
        assert!((u - 0.224).abs() < 0.01, "U = {u}, expected ~0.224");
        assert!((r - 4.466).abs() < 0.02, "R = {r}, expected ~4.466");
    }

    #[test]
    fn worked_example_f_rsi_above_minimum() {
        let layers = sample_moisture_wall();
        let f = part_3::interior_surface_temperature_factor(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.5);
        assert!(f > F_RSI_MINIMUM, "f_Rsi = {f}, must exceed {F_RSI_MINIMUM}");
        let check = part_3::check_surface_temperature(&layers, 20.0, -14.0, 0.5).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[test]
    fn worked_example_glaser_no_condensation_insulated_wall() {
        let layers = sample_moisture_wall();
        assert!(!part_3::condensation_at_interfaces(&layers, 20.0, -14.0, 0.5));
        let check = part_3::check_glaser_moisture(&layers, 20.0, -14.0, 0.5).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[test]
    fn worked_example_magnus_saturation_at_zero_c() {
        let e = part_3::saturation_vapor_pressure_pa(0.0);
        assert!((e - 611.2).abs() < 1.0, "e_sat(0°C) = {e}");
    }

    #[test]
    fn worked_example_vapor_resistance_formula() {
        let layer = part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 };
        let r_mu = part_3::vapor_resistance(&layer);
        let expected = 0.14 / (1.3 * 0.035);
        assert!((r_mu - expected).abs() < 1e-9);
    }

    #[test]
    fn part_2_colder_zone_allows_higher_u_limit() {
        let limit_warm = part_2::climate_adjusted_u_limit(part_2::BuildingCategory::Residential, ClimateZoneDe::Zone4);
        let limit_cold = part_2::climate_adjusted_u_limit(part_2::BuildingCategory::Residential, ClimateZoneDe::Zone1);
        assert!(limit_cold > limit_warm, "zone1={limit_cold}, zone4={limit_warm}");
        assert!((limit_cold - 0.308).abs() < 0.01);
    }

    #[test]
    fn part_4_mineral_wool_lambda() {
        let r = part_4::check_design_lambda("mineral_wool", 0.038).unwrap();
        assert_eq!(r.status, crate::document::CheckStatus::Pass);
        let design = part_4::design_lambda_for_material("mineral_wool").unwrap();
        assert!((design - 0.0385).abs() < 0.001);
    }

    #[test]
    fn part_4_has_fifteen_plus_materials() {
        let materials = ["mineral_wool", "glass_wool", "eps", "xps", "pur", "pir", "wood_fibre", "cellulose", "concrete", "aerated_concrete", "brick", "sand_lime_brick", "timber", "plywood", "gypsum_plaster", "lime_plaster", "clay_plaster"];
        assert!(materials.len() >= 15);
        for m in materials {
            assert!(part_4::material_design(m).is_ok(), "missing {m}");
        }
    }

    #[test]
    fn part_5_summer_heat_zone_dependent() {
        let layers = sample_wall();
        let flux_z2 = part_5::peak_summer_heat_flux_w_m2(&layers, ClimateZoneDe::Zone2, 26.0, 0.6, 600.0);
        let flux_z4 = part_5::peak_summer_heat_flux_w_m2(&layers, ClimateZoneDe::Zone4, 26.0, 0.6, 600.0);
        assert!(flux_z4 > flux_z2);
        let check = part_5::check_summer_heat_protection(&layers, ClimateZoneDe::Zone2, 26.0, 0.6, 600.0).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[test]
    fn part_6_thermal_bridge_increases_u() {
        let layers = sample_wall();
        let u_element = part_2::u_value_from_resistance(part_2::total_resistance(&layers, R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        let u_bridged = part_6::u_value_with_thermal_bridges(u_element, 0.05);
        assert!((u_bridged - (u_element + 0.05)).abs() < 1e-9);
        let check = part_6::check_u_value_with_bridges(&layers, 0.05, 0.35).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[test]
    fn part_8_catalog_lookup() {
        let entry = part_8::catalog_entry("AW-01").unwrap();
        assert!((entry.u_typical_w_m2k - 0.24).abs() < 0.01);
        let u = part_2::u_value_from_resistance(part_2::total_resistance(&sample_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W));
        let check = part_8::check_against_catalog("AW-01", u).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[test]
    fn host_updates_report_after_document_replace() {
        let mut host = Host::default();
        assert!(host.report().all_pass());
        let mut document = Document::default();
        document.layers.clear();
        host.replace_document(document);
        assert!(!host.report().all_pass());
    }

    #[test]
    fn full_envelope_evaluate_covers_all_eight_parts() {
        let document = Document::default();
        let report = evaluate(&document);
        assert!(report.checks.len() >= 15, "expected parts 1–8 checks, got {}", report.checks.len());
        assert!(report.all_pass(), "checks: {:?}", report.checks);
        let f_dry = part_3::interior_surface_temperature_factor(&sample_moisture_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.5);
        let f_humid = part_3::interior_surface_temperature_factor(&sample_moisture_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.8);
        assert!(f_humid < f_dry, "humidity correction must reduce f_Rsi");
    }

    #[test]
    fn part_1_plausibility_flags_implausible_u_value() {
        let layers = sample_wall();
        let ok = part_1::check_input_plausibility(&layers, 0.224);
        assert_eq!(ok.status, crate::document::CheckStatus::Pass);
        let bad = part_1::check_input_plausibility(&layers, 12.0);
        assert_eq!(bad.status, crate::document::CheckStatus::Fail);
        let na = part_1::check_input_plausibility(&[], 0.3);
        assert_eq!(na.status, crate::document::CheckStatus::NotApplicable);
    }

    #[test]
    fn part_10_application_class_admissibility() {
        let admissible = part_10::check_application_class(part_10::ApplicationType::Deo, part_10::ApplicationClass::Dk);
        assert_eq!(admissible.status, crate::document::CheckStatus::Pass);
        let inadmissible = part_10::check_application_class(part_10::ApplicationType::Duk, part_10::ApplicationClass::Dm);
        assert_eq!(inadmissible.status, crate::document::CheckStatus::Fail);
        assert_eq!(part_10::minimum_class(part_10::ApplicationType::Duk), part_10::ApplicationClass::Dg);
    }

    #[test]
    fn bb2_worked_example_conform_details_pass() {
        let psi_l_sum = 18.0;
        let area = 400.0;
        let delta = bb_2::delta_u_wb_actual_w_m2k(psi_l_sum, area);
        assert!((delta - 0.045).abs() < 1e-9, "delta = {delta}");
        let check = bb_2::check_beiblatt_2_equivalence(psi_l_sum, area, true).unwrap();
        assert_eq!(check.status, crate::document::CheckStatus::Pass);
    }

    #[test]
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
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "din4108.document",
        extension: Some("din4108"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::din4108::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::din4108::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::din4108::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::din4108::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("din4108.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "din4108.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::din4108::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::din4108::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("din4108.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "din4108.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::din4108::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::din4108::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("din4108.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "din4108.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::din4108::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::din4108::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("din4108.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "din4108.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("din4108.spr"),
    });
}
