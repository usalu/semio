//! ⚙️ EN 1998 app — headless compute (constitutional: engine).

use en1998::Document;
use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

// #region 🔖️NaDe
pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;

    /// 🌋️ German seismic zone per DIN EN 1998-1/NA.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeismicZone {
        Zone0,
        Zone1,
        Zone2,
        Zone3,
    }

    impl SeismicZone {
        pub fn as_u8(self) -> u8 {
            match self {
                Self::Zone0 => 0,
                Self::Zone1 => 1,
                Self::Zone2 => 2,
                Self::Zone3 => 3,
            }
        }

        pub fn a_g(self) -> f64 {
            match self {
                Self::Zone0 => 0.0,
                Self::Zone1 => 0.08,
                Self::Zone2 => 0.15,
                Self::Zone3 => 0.24,
            }
        }
    }

    /// 🪨️ Ground type per EN 1998-1 Table 3.1 (DE NA).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GroundType {
        A,
        B,
        C,
        D,
        E,
    }

    impl GroundType {
        pub fn spectrum_params(self) -> (f64, f64, f64, f64) {
            match self {
                Self::A => (0.05, 0.25, 0.8, 1.0),
                Self::B => (0.15, 0.4, 2.0, 1.0),
                Self::C => (0.20, 0.6, 2.0, 1.15),
                Self::D => (0.25, 0.8, 2.0, 1.35),
                Self::E => (0.35, 1.2, 2.0, 1.4),
            }
        }
    }

    pub fn peak_ground_acceleration(zone: SeismicZone) -> f64 {
        zone.a_g()
    }
}
// #endregion 🔖️NaDe

// #region 🔖️Part1
pub mod part_1 {
    use super::*;

    /// 🏗️ Structural system behaviour factor q per EN 1998-1 Table 6.1.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum StructuralSystem {
        MomentFrameDch,
        MomentFrameDcm,
        MomentFrameDcl,
        ShearWall,
        BracedFrame,
        InvertedPendulum,
        DualSystem,
    }

    impl StructuralSystem {
        pub fn q(self) -> f64 {
            match self {
                Self::MomentFrameDch => 4.0,
                Self::MomentFrameDcm => 3.3,
                Self::MomentFrameDcl => 2.0,
                Self::ShearWall => 3.0,
                Self::BracedFrame => 2.5,
                Self::InvertedPendulum => 1.5,
                Self::DualSystem => 4.0,
            }
        }
    }

    /// 📊️ Importance factor γ_I per EN 1998-1 Table 4.3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImportanceClass {
        Cc1,
        Cc2,
        Cc3,
        Cc4,
    }

    impl ImportanceClass {
        pub fn gamma_i(self) -> f64 {
            match self {
                Self::Cc1 => 0.8,
                Self::Cc2 => 1.0,
                Self::Cc3 => 1.2,
                Self::Cc4 => 1.4,
            }
        }
    }

    /// 🌍️ EN 1998-1 elastic response spectrum shape per §3.2.2.2: Type 1 (M_s ≥ 5.5) vs Type 2 (M_s < 5.5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SpectrumType {
        Type1,
        Type2,
    }

    /// 🪨️ Generic EN 1998-1 ground type per Table 3.1, independent of any national annex table.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EnGroundType {
        A,
        B,
        C,
        D,
        E,
    }

    impl EnGroundType {
        /// 📊️ (S, T_B, T_C, T_D) per EN 1998-1 Table 3.2 (Type 1) / Table 3.3 (Type 2).
        pub fn spectrum_params(self, spectrum: SpectrumType) -> (f64, f64, f64, f64) {
            match spectrum {
                SpectrumType::Type1 => match self {
                    Self::A => (1.0, 0.15, 0.4, 2.0),
                    Self::B => (1.2, 0.15, 0.5, 2.0),
                    Self::C => (1.15, 0.20, 0.6, 2.0),
                    Self::D => (1.35, 0.20, 0.8, 2.0),
                    Self::E => (1.4, 0.15, 0.5, 2.0),
                },
                SpectrumType::Type2 => match self {
                    Self::A => (1.0, 0.05, 0.25, 1.2),
                    Self::B => (1.35, 0.05, 0.25, 1.2),
                    Self::C => (1.5, 0.10, 0.25, 1.2),
                    Self::D => (1.8, 0.10, 0.30, 1.2),
                    Self::E => (1.6, 0.05, 0.25, 1.2),
                },
            }
        }
    }

    /// 📈️ Elastic response spectrum Type 1/2 shape horizontal [g] per EN 1998-1 §3.2.2.2, given resolved (a_g, S, T_B, T_C, T_D).
    pub fn elastic_response_spectrum_type1(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64) -> f64 {
        let eta = 1.0;
        if t <= tb {
            a_g * s * (1.0 + t / tb * (2.5 * eta - 1.0))
        } else if t <= tc {
            a_g * s * 2.5 * eta
        } else if t <= td {
            a_g * s * 2.5 * eta * tc / t
        } else {
            a_g * s * 2.5 * eta * tc * td / (t * t)
        }
    }

    /// 📉️ Design spectrum Sd(T) = S_e(T) · γ_I / q [g].
    pub fn design_spectrum_sd(s_e: f64, gamma_i: f64, q: f64) -> f64 {
        s_e * gamma_i / q
    }

    /// 🌊️ Base shear V_b = S_e(T1) · m · γ_I / q [kN] with mass in tonnes.
    pub fn base_shear_kn(s_e: f64, mass_t: f64, gamma_i: f64, q: f64) -> f64 {
        s_e * mass_t * 9.81 * gamma_i / q
    }

    /// 🌊️ Base shear from design spectrum S_d(T1) [kN].
    pub fn base_shear_from_design_kn(s_d: f64, mass_t: f64) -> f64 {
        s_d * mass_t * 9.81
    }

    /// 🔁️ Redundancy factor ρ per EN 1998-1 §4.2.5.
    pub fn redundancy_factor(multiple_resisting_systems: bool) -> f64 {
        if multiple_resisting_systems {
            1.0
        } else {
            1.3
        }
    }

    /// 📐️ Interstorey drift limit with ρ per EN 1998-1 §4.3.3.4 [mm].
    pub fn drift_limit_mm(height_m: f64, rho: f64, ductility: DuctilityClass, nu: f64) -> f64 {
        let theta = match ductility {
            DuctilityClass::Dch => 0.01,
            DuctilityClass::Dcm => 0.007,
            DuctilityClass::Dcl => 0.005,
        };
        nu * rho * theta * height_m * 1000.0
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DuctilityClass {
        Dch,
        Dcm,
        Dcl,
    }

    pub fn check_drift(drift_mm: f64, limit_mm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-1", "§4.3", "4.3.3"), Quantity::length_m(drift_mm / 1000.0), Quantity::length_m(limit_mm / 1000.0), "interstorey drift SLS", annex)
    }

    pub fn check_base_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-1", "§4.3", "4.3.4"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "seismic base shear ULS", annex)
    }
}
// #endregion 🔖️Part1

// #region 🔖️AnnexParams
/// 🇪️🇺️ Resolved seismic-action model per NDP: DE zone table vs EN Type-1/2 spectrum (EN 1998-1 §3.2 NDP).
#[derive(Clone, Debug, PartialEq)]
pub enum AnnexParams {
    De { zone: na_de::SeismicZone, ground: na_de::GroundType },
    En { a_gr: f64, ground: part_1::EnGroundType, spectrum: part_1::SpectrumType },
}

impl AnnexParams {
    pub fn choice(&self) -> AnnexChoice {
        match self {
            Self::De { .. } => AnnexChoice::De,
            Self::En { .. } => AnnexChoice::En,
        }
    }

    /// 📐️ Resolved (a_g, S, T_B, T_C, T_D) feeding `part_1::elastic_response_spectrum_type1`.
    pub fn ground_params(&self) -> (f64, f64, f64, f64, f64) {
        match self {
            Self::De { zone, ground } => {
                let (tb, tc, td, s) = ground.spectrum_params();
                (zone.a_g(), s, tb, tc, td)
            }
            Self::En { a_gr, ground, spectrum } => {
                let (s, tb, tc, td) = ground.spectrum_params(*spectrum);
                (*a_gr, s, tb, tc, td)
            }
        }
    }

    /// 📈️ Elastic response spectrum S_e(T) [g] resolved for this annex selection.
    pub fn elastic_response_spectrum(&self, t: f64) -> f64 {
        let (a_g, s, tb, tc, td) = self.ground_params();
        part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t)
    }
}
// #endregion 🔖️AnnexParams

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🌉️ Isolated bridge design spectrum reduction factor q_isol.
    pub fn isolation_reduction_factor(period_ratio: f64) -> f64 {
        (period_ratio * period_ratio).max(1.0)
    }

    /// 🌉️ Design spectrum for isolated bridge deck [g].
    pub fn isolated_spectrum_sd(s_e: f64, gamma_i: f64, q_isol: f64) -> f64 {
        s_e * gamma_i / q_isol
    }

    /// 🌉️ Bearing displacement check limit [mm].
    pub fn bearing_displacement_limit_mm(d_max_mm: f64) -> f64 {
        d_max_mm
    }

    pub fn check_bridge_seismic(v_ed: f64, v_rd: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-2", "§5", "5.3"), Quantity::force_kn(v_ed), Quantity::force_kn(v_rd), "bridge seismic shear", AnnexChoice::En)
    }

    pub fn check_isolation_bearing(d_ed_mm: f64, d_rd_mm: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-2", "§7", "7.5"), Quantity::length_m(d_ed_mm / 1000.0), Quantity::length_m(d_rd_mm / 1000.0), "isolation bearing displacement", AnnexChoice::En)
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 🔍️ Knowledge level per EN 1998-3 §3.4, driving the confidence factor CF.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum KnowledgeLevel {
        Kl1,
        Kl2,
        Kl3,
    }

    impl KnowledgeLevel {
        /// 🎯️ Confidence factor CF per EN 1998-3 Table 3.1.
        pub fn confidence_factor(self) -> f64 {
            match self {
                Self::Kl1 => 1.35,
                Self::Kl2 => 1.20,
                Self::Kl3 => 1.00,
            }
        }
    }

    /// ⚖️ Limit state for existing-building assessment per EN 1998-3 §2.1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RetrofitLimitState {
        DamageLimitation,
        SignificantDamage,
        NearCollapse,
    }

    impl RetrofitLimitState {
        /// 📑️ EN 1998-3 §2.3.4 clause subsection per limit state.
        pub fn clause_section(self) -> &'static str {
            match self {
                Self::DamageLimitation => "2.3.4.1",
                Self::SignificantDamage => "2.3.4.2",
                Self::NearCollapse => "2.3.4.3",
            }
        }
    }

    /// 🏚️ Design capacity R_d = R_k / (CF · γ_el) per EN 1998-3 §2.3.3.
    pub fn design_capacity_kn(r_k_kn: f64, cf: f64, gamma_el: f64) -> f64 {
        r_k_kn / (cf * gamma_el)
    }

    /// 🏚️ Existing-element seismic capacity check E_d ≤ R_k / (CF · γ_el) per EN 1998-3 §2.3.3.
    pub fn check_element_capacity(e_d_kn: f64, r_k_kn: f64, cf: f64, gamma_el: f64, limit_state: RetrofitLimitState, annex: AnnexChoice) -> CheckResult {
        let r_d = design_capacity_kn(r_k_kn, cf, gamma_el);
        CheckResult::from_utilization(ClauseId::new("EN 1998-3", "§2.3", limit_state.clause_section()), Quantity::force_kn(e_d_kn), Quantity::force_kn(r_d), "existing element seismic capacity", annex)
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 🏺️ Impulsive period T_i [s] for circular silo/tank per EN 1998-4 Annex.
    pub fn impulsive_period_s(height_m: f64, radius_m: f64) -> f64 {
        0.1 * (height_m / radius_m).sqrt()
    }

    /// 🏺️ Convective (sloshing) period T_c [s] for circular silo/tank.
    pub fn convective_period_s(radius_m: f64) -> f64 {
        2.0 * (radius_m / 9.81).sqrt()
    }

    /// 🏺️ Impulsive mass ratio μ_i.
    pub fn impulsive_mass_ratio(h_over_r: f64) -> f64 {
        (0.45 * h_over_r / (1.0 + 0.75 * h_over_r)).clamp(0.1, 0.85)
    }

    /// 🏺️ Convective mass ratio μ_c.
    pub fn convective_mass_ratio(h_over_r: f64) -> f64 {
        (0.55 / (1.0 + 0.75 * h_over_r)).clamp(0.05, 0.75)
    }

    /// 🏺️ Combined silo base shear via SRSS of impulsive and convective components [kN].
    pub fn silo_base_shear_kn(v_i_kn: f64, v_c_kn: f64) -> f64 {
        (v_i_kn * v_i_kn + v_c_kn * v_c_kn).sqrt()
    }

    /// 🛢️ Tank base shear V = m_i·S_e(T_i) + m_c·S_e(T_c) [kN] per EN 1998-4 §4 simplified model.
    pub fn tank_base_shear_kn(m_i_t: f64, s_e_i: f64, m_c_t: f64, s_e_c: f64) -> f64 {
        (m_i_t * s_e_i + m_c_t * s_e_c) * 9.81
    }

    /// 🏺️ Behaviour factor q capped at 1.5 for silos per EN 1998-4 Table 2.1.
    pub fn silo_behaviour_factor(q_nominal: f64) -> f64 {
        q_nominal.min(1.5)
    }

    pub fn check_silo_wall(n_ed_kn: f64, n_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§3", "3.4"), Quantity::force_kn(n_ed_kn), Quantity::force_kn(n_rd_kn), "silo wall seismic", AnnexChoice::En)
    }

    pub fn check_silo_anchor(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§3", "3.5"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "silo anchorage", AnnexChoice::En)
    }

    pub fn check_tank_base_shear(v_ed_kn: f64, v_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-4", "§4", "4.3"), Quantity::force_kn(v_ed_kn), Quantity::force_kn(v_rd_kn), "tank hydrodynamic base shear", AnnexChoice::En)
    }
}
// #endregion 🔖️Part4

// #region 🔖️Part5
pub mod part_5 {
    use super::*;

    /// 🧱️ Foundation stiffness ratio r = K_f / K_s per EN 1998-5 §7.
    pub fn stiffness_ratio(k_foundation: f64, k_soil: f64) -> f64 {
        k_foundation / k_soil
    }

    /// 🧱️ Radiation damping ratio ξ for shallow foundation.
    pub fn radiation_damping(ratio: f64) -> f64 {
        (0.05 + 0.1 * ratio / (1.0 + ratio)).clamp(0.05, 0.20)
    }

    /// 🧱️ Bearing capacity reduction factor under seismic loading per EN 1998-5 §7.
    pub fn bearing_reduction_factor(a_g: f64) -> f64 {
        (1.0 - 1.5 * a_g).max(0.5)
    }

    /// 🧱️ Seismic bearing pressure [kPa].
    pub fn seismic_bearing_pressure_kpa(v_seismic_kn: f64, area_m2: f64) -> f64 {
        v_seismic_kn / area_m2
    }

    pub fn check_foundation_bearing(p_ed_kpa: f64, p_rd_kpa: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-5", "§7", "7.3"),
            Quantity::new(norm_core::QuantityKind::Pressure, p_ed_kpa * 1000.0),
            Quantity::new(norm_core::QuantityKind::Pressure, p_rd_kpa * 1000.0),
            "foundation seismic bearing",
            AnnexChoice::De,
        )
    }

    pub fn check_foundation_sliding(h_ed_kn: f64, h_rd_kn: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-5", "§7", "7.4"), Quantity::force_kn(h_ed_kn), Quantity::force_kn(h_rd_kn), "foundation seismic sliding", AnnexChoice::De)
    }

    /// 🌍️ Horizontal seismic coefficient k_h = α·S/r per EN 1998-5 §7.3.2.2 (r: wall-displacement class).
    pub fn horizontal_seismic_coefficient(alpha: f64, s: f64, r: f64) -> f64 {
        alpha * s / r
    }

    /// 🧱️ Mononobe-Okabe dynamic active earth-pressure coefficient K_AE per EN 1998-5 Annex E (vertical wall, horizontal backfill, no wall friction). Reduces to the classic Rankine K_a at k_h = 0.
    pub fn mononobe_okabe_k_ae(phi_deg: f64, k_h: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let theta = k_h.atan();
        let bracket = 1.0 + ((phi.sin() * (phi - theta).sin()) / theta.cos()).sqrt();
        (phi - theta).cos().powi(2) / (theta.cos() * bracket * bracket)
    }

    /// 🧱️ Dynamic active thrust increment on a retaining wall [kN/m] from K_AE.
    pub fn retaining_wall_thrust_kn_m(gamma_soil_kn_m3: f64, height_m: f64, k_ae: f64) -> f64 {
        0.5 * gamma_soil_kn_m3 * height_m * height_m * k_ae
    }

    pub fn check_retaining_wall_sliding(h_ed_kn_m: f64, h_rd_kn_m: f64) -> CheckResult {
        CheckResult::from_utilization(ClauseId::new("EN 1998-5", "§6", "E.2"), Quantity::force_kn(h_ed_kn_m), Quantity::force_kn(h_rd_kn_m), "retaining wall seismic thrust", AnnexChoice::En)
    }
}
// #endregion 🔖️Part5

// #region 🔖️Part6
pub mod part_6 {
    use super::*;

    /// 🗼️ Along-wind base overturning moment [kNm] per EN 1998-6 §4 (wind-induced dynamic response of slender towers).
    pub fn along_wind_overturning_knm(rho_air: f64, v_crit_m_s: f64, height_m: f64, diameter_m: f64, c_d: f64) -> f64 {
        let q_z = 0.5 * rho_air * v_crit_m_s * v_crit_m_s / 1000.0;
        q_z * c_d * diameter_m * height_m * height_m / 2.0
    }

    /// 🗼️ Critical wind speed for vortex shedding [m/s].
    pub fn critical_wind_speed_m_s(strouhal: f64, frequency_hz: f64, diameter_m: f64) -> f64 {
        strouhal * frequency_hz * diameter_m
    }

    /// 🗼️ First-mode natural frequency [Hz] for a cantilever tower.
    pub fn tower_frequency_hz(e_i_pa: f64, i_m4: f64, mass_kg_m: f64, height_m: f64) -> f64 {
        let lambda = 1.875;
        let omega = lambda * lambda * (e_i_pa * i_m4 / (mass_kg_m * height_m.powi(4))).sqrt();
        omega / (2.0 * std::f64::consts::PI)
    }

    /// 🗼️ Behaviour factor q capped per EN 1998-6 Table 4.1: 1.5 for chimneys, 2.0 for other towers/masts.
    pub fn tower_behaviour_factor(q_nominal: f64, is_chimney: bool) -> f64 {
        let cap = if is_chimney { 1.5 } else { 2.0 };
        q_nominal.min(cap)
    }

    /// 🗼️ First-mode participation factor Γ for a uniform cantilever with mode shape φ(x) = 1 − cos(πx/2H) per EN 1998-6 Annex B (simplified modal analysis).
    pub fn cantilever_modal_participation_factor() -> f64 {
        let numerator = 1.0 - 2.0 / std::f64::consts::PI;
        let denominator = 1.5 - 4.0 / std::f64::consts::PI;
        numerator / denominator
    }

    /// 🗼️ Modal base shear V_b1 = Γ · S_d(T1) · m · g [kN] for the cantilever first mode.
    pub fn tower_base_shear_kn(gamma: f64, s_d: f64, mass_t: f64) -> f64 {
        gamma * s_d * mass_t * 9.81
    }

    pub fn check_tower_overturning(m_ed_knm: f64, m_rd_knm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1998-6", "§4", "4.3.2"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "tower overturning",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖️Part6

/// 📋️ Building seismic check generalized over DE zone-based or EN Type-1/2-spectrum annex selection.
pub fn check_building_seismic_with_annex(
    annex: AnnexParams,
    importance: part_1::ImportanceClass,
    system: part_1::StructuralSystem,
    t1_s: f64,
    mass_t: f64,
    v_rd_kn: f64,
    drift_mm: f64,
    height_m: f64,
    multiple_resisting_systems: bool,
) -> CheckReport {
    let choice = annex.choice();
    let gamma_i = importance.gamma_i();
    let q = system.q();
    let s_e = annex.elastic_response_spectrum(t1_s);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let v_b = part_1::base_shear_from_design_kn(s_d, mass_t);
    let rho = part_1::redundancy_factor(multiple_resisting_systems);
    let drift_limit = part_1::drift_limit_mm(height_m, rho, part_1::DuctilityClass::Dcm, 1.0);
    let mut report = CheckReport::default();
    report.push(part_1::check_base_shear(v_b, v_rd_kn, choice));
    report.push(part_1::check_drift(drift_mm, drift_limit, choice));
    report
}

/// 📋️ Building seismic check (DE NA zone parameters).
pub fn check_building_seismic(
    zone: na_de::SeismicZone,
    ground: na_de::GroundType,
    importance: part_1::ImportanceClass,
    system: part_1::StructuralSystem,
    t1_s: f64,
    mass_t: f64,
    v_rd_kn: f64,
    drift_mm: f64,
    height_m: f64,
    multiple_resisting_systems: bool,
) -> CheckReport {
    check_building_seismic_with_annex(AnnexParams::De { zone, ground }, importance, system, t1_s, mass_t, v_rd_kn, drift_mm, height_m, multiple_resisting_systems)
}

/// 📋️ Full seismic check across EN 1998 parts 1 through 6.
pub fn check_full_seismic(document: &Document) -> CheckReport {
    let zone = parse_seismic_zone(document.seismic_zone);
    let ground = parse_ground_type(&document.ground_type);
    let importance = parse_importance(&document.importance_class);
    let system = parse_structural_system(&document.structural_system);
    let annex_choice = parse_annex(&document.annex);

    let annex = match annex_choice {
        AnnexChoice::En => AnnexParams::En { a_gr: document.en_a_gr, ground: parse_en_ground_type(&document.en_ground_type), spectrum: parse_spectrum_type(&document.en_spectrum_type) },
        AnnexChoice::De => AnnexParams::De { zone, ground },
    };
    let (a_g, s, tb, tc, td) = annex.ground_params();
    let gamma_i = importance.gamma_i();
    let q = system.q();
    let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, document.t1_s);
    let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
    let v_b = part_1::base_shear_from_design_kn(s_d, document.mass_t);

    let mut report = check_building_seismic_with_annex(annex, importance, system, document.t1_s, document.mass_t, document.v_rd_kn, document.drift_mm, document.height_m, document.multiple_resisting_systems);

    let q_isol = part_2::isolation_reduction_factor(document.period_ratio);
    let s_d_isol = part_2::isolated_spectrum_sd(s_e, gamma_i, q_isol);
    let v_bridge = part_1::base_shear_from_design_kn(s_d_isol, document.mass_t);
    report.push(part_2::check_bridge_seismic(v_bridge, document.bridge_v_rd_kn));
    report.push(part_2::check_isolation_bearing(document.bearing_d_ed_mm, document.bearing_d_rd_mm));

    let kl = parse_knowledge_level(&document.retrofit_knowledge_level);
    let limit_state = parse_retrofit_limit_state(&document.retrofit_limit_state);
    report.push(part_3::check_element_capacity(document.retrofit_e_d_kn, document.retrofit_r_k_kn, kl.confidence_factor(), document.retrofit_gamma_el, limit_state, annex_choice));

    let h_over_r_silo = document.silo_height_m / document.silo_radius_m;
    let mu_i = part_4::impulsive_mass_ratio(h_over_r_silo);
    let mu_c = part_4::convective_mass_ratio(h_over_r_silo);
    let v_i = part_1::base_shear_from_design_kn(s_d, document.mass_t * mu_i);
    let t_c_silo = part_4::convective_period_s(document.silo_radius_m);
    let s_e_c_silo = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c_silo);
    let s_d_c_silo = part_1::design_spectrum_sd(s_e_c_silo, gamma_i, q);
    let v_c = part_1::base_shear_from_design_kn(s_d_c_silo, document.mass_t * mu_c);
    let v_silo = part_4::silo_base_shear_kn(v_i, v_c);
    report.push(part_4::check_silo_wall(v_silo, document.silo_n_rd_kn));
    report.push(part_4::check_silo_anchor(document.silo_v_ed_kn, document.silo_v_rd_kn));
    let _ = part_4::silo_behaviour_factor(document.silo_q_nominal);

    let h_over_r_tank = document.tank_height_m / document.tank_radius_m;
    let mu_i_tank = part_4::impulsive_mass_ratio(h_over_r_tank);
    let mu_c_tank = part_4::convective_mass_ratio(h_over_r_tank);
    let t_i_tank = part_4::impulsive_period_s(document.tank_height_m, document.tank_radius_m);
    let t_c_tank = part_4::convective_period_s(document.tank_radius_m);
    let s_e_i_tank = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_i_tank);
    let s_e_c_tank = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c_tank);
    let v_tank = part_4::tank_base_shear_kn(document.tank_mass_t * mu_i_tank, s_e_i_tank, document.tank_mass_t * mu_c_tank, s_e_c_tank);
    report.push(part_4::check_tank_base_shear(v_tank, document.tank_v_rd_kn));

    let bearing_red = part_5::bearing_reduction_factor(a_g);
    let p_rd = document.foundation_p_rd_kpa * bearing_red;
    let p_ed = part_5::seismic_bearing_pressure_kpa(v_b, document.foundation_area_m2);
    report.push(part_5::check_foundation_bearing(p_ed, p_rd));
    report.push(part_5::check_foundation_sliding(document.foundation_h_ed_kn, document.foundation_h_rd_kn));
    let _ = part_5::radiation_damping(part_5::stiffness_ratio(document.k_foundation, document.k_soil));

    let k_h = part_5::horizontal_seismic_coefficient(a_g, s, document.wall_r);
    let k_ae = part_5::mononobe_okabe_k_ae(document.wall_phi_deg, k_h);
    let h_ed_wall = part_5::retaining_wall_thrust_kn_m(document.wall_soil_gamma_kn_m3, document.wall_height_m, k_ae);
    report.push(part_5::check_retaining_wall_sliding(h_ed_wall, document.wall_h_rd_kn));

    let q_tower = part_6::tower_behaviour_factor(document.tower_q_nominal, document.tower_is_chimney);
    let s_d_tower = part_1::design_spectrum_sd(s_e, gamma_i, q_tower);
    let gamma_modal = part_6::cantilever_modal_participation_factor();
    let _v_tower = part_6::tower_base_shear_kn(gamma_modal, s_d_tower, document.tower_mass_t);
    report.push(part_6::check_tower_overturning(document.tower_m_ed_knm, document.tower_m_rd_knm));

    report
}

// #region 🔖️Session
fn parse_seismic_zone(value: u8) -> na_de::SeismicZone {
    match value {
        0 => na_de::SeismicZone::Zone0,
        1 => na_de::SeismicZone::Zone1,
        3 => na_de::SeismicZone::Zone3,
        _ => na_de::SeismicZone::Zone2,
    }
}

fn parse_ground_type(value: &str) -> na_de::GroundType {
    match value.to_ascii_lowercase().as_str() {
        "a" => na_de::GroundType::A,
        "c" => na_de::GroundType::C,
        "d" => na_de::GroundType::D,
        "e" => na_de::GroundType::E,
        _ => na_de::GroundType::B,
    }
}

fn parse_importance(value: &str) -> part_1::ImportanceClass {
    match value.to_ascii_lowercase().as_str() {
        "cc1" => part_1::ImportanceClass::Cc1,
        "cc3" => part_1::ImportanceClass::Cc3,
        "cc4" => part_1::ImportanceClass::Cc4,
        _ => part_1::ImportanceClass::Cc2,
    }
}

fn parse_structural_system(value: &str) -> part_1::StructuralSystem {
    match value.to_ascii_lowercase().as_str() {
        "moment_frame_dcm" => part_1::StructuralSystem::MomentFrameDcm,
        "moment_frame_dcl" => part_1::StructuralSystem::MomentFrameDcl,
        "shear_wall" => part_1::StructuralSystem::ShearWall,
        "braced_frame" => part_1::StructuralSystem::BracedFrame,
        "inverted_pendulum" => part_1::StructuralSystem::InvertedPendulum,
        "dual_system" => part_1::StructuralSystem::DualSystem,
        _ => part_1::StructuralSystem::MomentFrameDch,
    }
}

fn parse_annex(value: &str) -> AnnexChoice {
    match value.to_ascii_lowercase().as_str() {
        "en" => AnnexChoice::En,
        _ => AnnexChoice::De,
    }
}

fn parse_en_ground_type(value: &str) -> part_1::EnGroundType {
    match value.to_ascii_lowercase().as_str() {
        "a" => part_1::EnGroundType::A,
        "c" => part_1::EnGroundType::C,
        "d" => part_1::EnGroundType::D,
        "e" => part_1::EnGroundType::E,
        _ => part_1::EnGroundType::B,
    }
}

fn parse_spectrum_type(value: &str) -> part_1::SpectrumType {
    match value.to_ascii_lowercase().as_str() {
        "type2" => part_1::SpectrumType::Type2,
        _ => part_1::SpectrumType::Type1,
    }
}

fn parse_knowledge_level(value: &str) -> part_3::KnowledgeLevel {
    match value.to_ascii_lowercase().as_str() {
        "kl1" => part_3::KnowledgeLevel::Kl1,
        "kl3" => part_3::KnowledgeLevel::Kl3,
        _ => part_3::KnowledgeLevel::Kl2,
    }
}

fn parse_retrofit_limit_state(value: &str) -> part_3::RetrofitLimitState {
    match value.to_ascii_lowercase().as_str() {
        "damage_limitation" => part_3::RetrofitLimitState::DamageLimitation,
        "near_collapse" => part_3::RetrofitLimitState::NearCollapse,
        _ => part_3::RetrofitLimitState::SignificantDamage,
    }
}

/// 🧮️ Headless per-document evaluation — the `NormFamily::evaluate` body for `En1998Family` (defined
/// in the sibling `op` crate, which depends on this `engine` crate to call it).
pub fn evaluate(document: &Document) -> CheckReport {
    check_full_seismic(document)
}
// #endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zone2_spectrum_sd_at_t1() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        assert!((s_e - 0.375).abs() < 1e-9);
        let sd = part_1::design_spectrum_sd(s_e, 1.0, 1.0);
        assert!((sd - 0.375).abs() < 1e-9);
    }

    #[test]
    fn zone2_spectrum_sd_at_half_second() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.5);
        assert!((s_e - 0.3).abs() < 1e-9);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        assert!((s_d - 0.075).abs() < 1e-9);
    }

    #[test]
    fn base_shear_uses_design_spectrum() {
        let a_g = 0.15;
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        let v_b = part_1::base_shear_kn(s_e, 100.0, gamma_i, q);
        assert!(v_b > 0.0);
        let expected = s_e * 100.0 * 9.81 * gamma_i / q;
        assert!((v_b - expected).abs() < 1e-6);
        assert!((part_1::base_shear_from_design_kn(s_d, 100.0) - v_b).abs() < 1e-6);
    }

    #[test]
    fn drift_rho_limit() {
        let rho = part_1::redundancy_factor(false);
        assert!((rho - 1.3).abs() < 1e-9);
        let limit = part_1::drift_limit_mm(12.0, rho, part_1::DuctilityClass::Dcm, 1.0);
        assert!((limit - 109.2).abs() < 0.1);
    }

    #[test]
    fn building_seismic_base_shear_uses_sd() {
        let a_g = na_de::SeismicZone::Zone2.a_g();
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let s_e = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, 0.3);
        let gamma_i = part_1::ImportanceClass::Cc2.gamma_i();
        let q = part_1::StructuralSystem::MomentFrameDch.q();
        let s_d = part_1::design_spectrum_sd(s_e, gamma_i, q);
        let expected_v_b = part_1::base_shear_from_design_kn(s_d, 500.0);

        let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
        assert_eq!(report.checks.len(), 2);
        assert!((report.checks[0].computed.value - expected_v_b * 1000.0).abs() < 1e-3);
    }

    #[test]
    fn en_type1_vs_de_zone_divergence_same_nominal_ag() {
        let a_g = 0.15;
        let annex_de = AnnexParams::De { zone: na_de::SeismicZone::Zone2, ground: na_de::GroundType::B };
        let annex_en = AnnexParams::En { a_gr: a_g, ground: part_1::EnGroundType::B, spectrum: part_1::SpectrumType::Type1 };
        let s_e_de = annex_de.elastic_response_spectrum(0.3);
        let s_e_en = annex_en.elastic_response_spectrum(0.3);
        assert!((s_e_de - 0.375).abs() < 1e-9);
        assert!((s_e_en - 0.45).abs() < 1e-9);
        assert!((s_e_en - s_e_de).abs() > 0.05);
    }

    #[test]
    fn full_seismic_e2e() {
        let report = check_full_seismic(&Document::default());
        assert_eq!(report.checks.len(), 12);
    }

    #[test]
    fn full_seismic_en_annex_e2e() {
        let mut document = Document::default();
        document.annex = "en".into();
        let report = check_full_seismic(&document);
        assert_eq!(report.checks.len(), 12);
    }

    #[test]
    fn building_seismic_e2e() {
        let report = check_building_seismic(na_de::SeismicZone::Zone2, na_de::GroundType::B, part_1::ImportanceClass::Cc2, part_1::StructuralSystem::MomentFrameDch, 0.3, 500.0, 800.0, 20.0, 12.0, true);
        assert_eq!(report.checks.len(), 2);
    }

    #[test]
    fn silo_impulsive_convective() {
        let h = 10.0;
        let r = 5.0;
        let t_i = part_4::impulsive_period_s(h, r);
        let t_c = part_4::convective_period_s(r);
        assert!(t_i < t_c);
        let v = part_4::silo_base_shear_kn(200.0, 150.0);
        assert!((v - 250.0).abs() < 1e-6);
    }

    #[test]
    fn silo_behaviour_factor_capped() {
        assert!((part_4::silo_behaviour_factor(2.0) - 1.5).abs() < 1e-9);
        assert!((part_4::silo_behaviour_factor(1.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn tank_base_shear_combines_impulsive_and_convective() {
        let a_g = 0.15;
        let (tb, tc, td, s) = na_de::GroundType::B.spectrum_params();
        let t_i = part_4::impulsive_period_s(8.0, 4.0);
        let t_c = part_4::convective_period_s(4.0);
        let s_e_i = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_i);
        let s_e_c = part_1::elastic_response_spectrum_type1(a_g, s, tb, tc, td, t_c);
        let v_tank = part_4::tank_base_shear_kn(100.0, s_e_i, 50.0, s_e_c);
        assert!((v_tank - 412.8624420576831).abs() < 1e-6);
    }

    #[test]
    fn bridge_isolation_distinct() {
        let q_isol = part_2::isolation_reduction_factor(2.0);
        assert!((q_isol - 4.0).abs() < 1e-9);
    }

    #[test]
    fn retrofit_confidence_factor_scales_capacity_exactly() {
        let r_k = 400.0;
        let r_d_kl3 = part_3::design_capacity_kn(r_k, part_3::KnowledgeLevel::Kl3.confidence_factor(), 1.0);
        let r_d_kl1 = part_3::design_capacity_kn(r_k, part_3::KnowledgeLevel::Kl1.confidence_factor(), 1.0);
        assert!((r_d_kl3 / r_d_kl1 - 1.35).abs() < 1e-9);
    }

    #[test]
    fn mononobe_okabe_k_ae_matches_hand_calc_and_reduces_to_rankine() {
        let k_ae = part_5::mononobe_okabe_k_ae(30.0, 0.2);
        assert!((k_ae - 0.46407409106465564).abs() < 1e-9);
        let k_a_static = part_5::mononobe_okabe_k_ae(30.0, 0.0);
        let rankine_ka = (1.0 - 30.0_f64.to_radians().sin()) / (1.0 + 30.0_f64.to_radians().sin());
        assert!((k_a_static - rankine_ka).abs() < 1e-9);
    }

    #[test]
    fn retaining_wall_thrust_from_k_ae() {
        let k_h = part_5::horizontal_seismic_coefficient(0.15, 1.0, 1.5);
        assert!((k_h - 0.1).abs() < 1e-9);
        let k_ae = part_5::mononobe_okabe_k_ae(30.0, 0.2);
        let thrust = part_5::retaining_wall_thrust_kn_m(18.0, 4.0, k_ae);
        assert!((thrust - 66.82666911331042).abs() < 1e-6);
    }

    #[test]
    fn cantilever_modal_participation_factor_matches_closed_form() {
        let gamma = part_6::cantilever_modal_participation_factor();
        assert!((gamma - 1.602484997695127).abs() < 1e-9);
    }

    #[test]
    fn tower_behaviour_factor_capped_by_type() {
        assert!((part_6::tower_behaviour_factor(3.0, true) - 1.5).abs() < 1e-9);
        assert!((part_6::tower_behaviour_factor(3.0, false) - 2.0).abs() < 1e-9);
    }
}
//#endregion 🧪️Tests
