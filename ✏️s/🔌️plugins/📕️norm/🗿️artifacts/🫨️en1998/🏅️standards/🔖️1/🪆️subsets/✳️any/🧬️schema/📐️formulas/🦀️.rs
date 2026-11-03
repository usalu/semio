//! 📐️ EN 1998 formula modules of the schema: DE-NA ground combinations and the parts 1 to 6 closed-form checks. See [`super`].

#![allow(async_fn_in_trait)]

use semio_s_artifact_norm_en1998_model::{En1998Assessment, En1998Member, En1998Site, En1998Snapshot, MemberKind, T1Method};

#[path = "🏚️assessment-state/🦀️.rs"]
pub mod part_3_state;

#[path = "🔩️critical-region/🦀️.rs"]
pub mod critical_region;

#[path = "🛞️isolation/🦀️.rs"]
pub mod isolation;

#[path = "🪨️soil-structure/🦀️.rs"]
pub mod soil_structure;

#[path = "🫧️liquefaction-hazard/🦀️.rs"]
pub mod liquefaction_hazard;

pub mod na_de {
    /// 🌋️ Elastic plateau ordinate of subsoil A-R is 2.5 a_gR S with S = 1, so the reference acceleration equivalent to the plateau acceleration S_aP,R of DIN EN 1998-1/NA:2023-11 is S_aP,R / 2.5.
    pub fn a_gr_from_s_ap_r(s_ap_r: f64) -> f64 {
        s_ap_r / 2.5
    }

    /// 🪨 DE-NA ground/subsoil combination (A-R … C-S).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GroundCombo { AR, BR, CR, BT, CT, CS }

    impl From<semio_s_artifact_norm_en1998_model::DeGroundCombo> for GroundCombo {
        fn from(value: semio_s_artifact_norm_en1998_model::DeGroundCombo) -> Self {
            match value {
                semio_s_artifact_norm_en1998_model::DeGroundCombo::AR => Self::AR,
                semio_s_artifact_norm_en1998_model::DeGroundCombo::BR => Self::BR,
                semio_s_artifact_norm_en1998_model::DeGroundCombo::CR => Self::CR,
                semio_s_artifact_norm_en1998_model::DeGroundCombo::BT => Self::BT,
                semio_s_artifact_norm_en1998_model::DeGroundCombo::CT => Self::CT,
                semio_s_artifact_norm_en1998_model::DeGroundCombo::CS => Self::CS,
            }
        }
    }

    impl GroundCombo {
        pub fn parse(ground_class: &str, subsoil: &str) -> Option<Self> {
            let key = format!("{}-{}", ground_class.trim().to_ascii_uppercase(), subsoil.trim().to_ascii_uppercase());
            match key.as_str() {
                "A-R" => Some(Self::AR),
                "B-R" => Some(Self::BR),
                "C-R" => Some(Self::CR),
                "B-T" => Some(Self::BT),
                "C-T" => Some(Self::CT),
                "C-S" => Some(Self::CS),
                _ => None,
            }
        }

        /// 📊️ (S, T_B, T_C, T_D) per DIN EN 1998-1/NA Table NA.4, read from the shared registry (`ndp.en1998.na4.*`).
        pub fn spectrum_params(self) -> (f64, f64, f64, f64) {
            let key = self.key();
            let n = |symbol: &str| semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1998.na4.{key}.{symbol}"), semio_s_plugin_norm_compliance::AnnexChoice::De);
            (n("s"), n("tb"), n("tc"), n("td"))
        }

        /// 🔖️ Registry key of the combination (`ndp.en1998.na4.<key>.*`).
        pub fn key(self) -> &'static str {
            match self { Self::AR => "ar", Self::BR => "br", Self::CR => "cr", Self::BT => "bt", Self::CT => "ct", Self::CS => "cs" }
        }
    }
}

pub mod part_1 {
    /// 📊️ ψ₂ from EN 1990 Table A1.1 / DIN EN 1990/NA Table NA.A.1.1.
    pub fn psi2_de(category: semio_s_artifact_norm_en1998_model::ImposedCategory) -> f64 {
        use semio_s_artifact_norm_en1998_model::ImposedCategory as C;
        let key = match category {
            C::A => "residential",
            C::B => "office",
            C::C => "congregation",
            C::D => "retail",
            C::E => "storage",
            C::F => "traffic_light",
            C::G => "traffic_heavy",
            C::H => "roof",
            C::Snow => "snow_high",
            C::Wind => "wind",
        };
        semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1990.psi.{key}.2"), semio_s_plugin_norm_compliance::AnnexChoice::De)
    }

    /// 🏗 φ from EN 1998-1 Table 4.2: categories A–C roof 1.0, correlated storeys 0.8, independent storeys 0.5; categories D–F and archives 1.0.
    pub fn phi_table_4_2(category: semio_s_artifact_norm_en1998_model::ImposedCategory, is_roof: bool, correlated_occupancy: bool) -> f64 {
        use semio_s_artifact_norm_en1998_model::ImposedCategory as C;
        match category {
            C::D | C::E | C::F => 1.0,
            _ if is_roof => 1.0,
            _ if correlated_occupancy => 0.8,
            _ => 0.5,
        }
    }

    /// ⚖️ ψ_E,i = φ · ψ₂,i (EN 1998-1 §3.2.4).
    pub fn psi_e(category: semio_s_artifact_norm_en1998_model::ImposedCategory, is_roof: bool, correlated_occupancy: bool) -> f64 {
        phi_table_4_2(category, is_roof, correlated_occupancy) * psi2_de(category)
    }

    /// 🏋️ Seismic combination gravity action on a storey [N] — ΣG_k + Σψ_E,i·Q_k,i (6.12b gravity part).
    pub fn storey_seismic_action_n(storey: &semio_s_artifact_norm_en1998_model::En1998Storey, is_roof: bool) -> f64 {
        let mut sum = storey.permanent_gk_n;
        for v in &storey.variables {
            sum += psi_e(v.category, is_roof, storey.correlated_occupancy) * v.qk_n;
        }
        sum
    }

    /// ⚖️ Seismic mass m = (ΣG_k + Σψ_E·Q_k)/g [kg] (EN 1998-1 §3.2.4).
    pub fn storey_seismic_mass_kg(storey: &semio_s_artifact_norm_en1998_model::En1998Storey, is_roof: bool) -> f64 {
        storey_seismic_action_n(storey, is_roof) / 9.81
    }

    /// ⚖️ Seismic mass from ΣG_k + Σψ_E·Q_k for ancillary entities (bridges/towers).
    pub fn entity_seismic_mass_kg(permanent_gk_n: f64, variables: &[semio_s_artifact_norm_en1998_model::En1998VariableAction], is_roof: bool, correlated: bool) -> f64 {
        let mut sum = permanent_gk_n;
        for v in variables {
            sum += psi_e(v.category, is_roof, correlated) * v.qk_n;
        }
        sum / 9.81
    }

    /// 🛢️ Fluid content mass: the full contents act (EN 1998-4), scaled by the filling ratio.
    pub fn fluid_content_mass_kg(permanent_gk_n: f64, content_qk_n: f64, filling_ratio: f64) -> f64 {
        (permanent_gk_n + filling_ratio.clamp(0.0, 1.0) * content_qk_n) / 9.81
    }

    /// 🫙 Silo/tank seismic mass with EN 1998-4 filling ratio on content Q_k.
    pub fn filled_content_seismic_mass_kg(permanent_gk_n: f64, content_qk_n: f64, content_category: semio_s_artifact_norm_en1998_model::ImposedCategory, filling_ratio: f64) -> f64 {
        let psi = psi_e(content_category, true, true);
        (permanent_gk_n + filling_ratio.clamp(0.0, 1.0) * psi * content_qk_n) / 9.81
    }

    /// 📐 Plan regularity metrics EN 1998-1 §4.2.3.2 — (e0x, e0y, rx, ry, ℓₛ).
    pub fn plan_regularity_metrics(
        cm_x: f64,
        cm_y: f64,
        cs_x: f64,
        cs_y: f64,
        k_x: f64,
        k_y: f64,
        plan_width_m: f64,
        plan_length_m: f64,
    ) -> (f64, f64, f64, f64, f64) {
        let e0x = (cm_x - cs_x).abs();
        let e0y = (cm_y - cs_y).abs();
        let lx = plan_width_m.max(0.0);
        let ly = plan_length_m.max(0.0);
        let ls = ((lx * lx + ly * ly) / 12.0).sqrt();
        let k_theta = k_x * (ly * 0.5).powi(2) + k_y * (lx * 0.5).powi(2);
        let rx = (k_theta / k_y.max(1e-9)).sqrt();
        let ry = (k_theta / k_x.max(1e-9)).sqrt();
        (e0x, e0y, rx, ry, ls)
    }

    /// 📐 Plan eccentricity utilisation per direction: max(e0x / (0.30 rx), e0y / (0.30 ry)) after EN 1998-1 4.2.3.2(5)b; a zero radius gives 1e9.
    pub fn plan_eccentricity_utilization(e0x: f64, e0y: f64, rx: f64, ry: f64) -> f64 {
        let u = |e: f64, r: f64| if r <= 0.0 { 1e9 } else { e / (0.30 * r) };
        u(e0x, rx).max(u(e0y, ry)).min(1e9)
    }

    #[cfg(test)] use crate::*;

    /// 🌋️ Importance factor γ_I read from `ndp.en1998.en1.gamma-i.<class>`.
    pub fn gamma_i(importance: semio_s_artifact_norm_en1998_model::ImportanceClass) -> f64 {
        use semio_s_artifact_norm_en1998_model::ImportanceClass as I;
        let class = match importance {
            I::I => "i",
            I::Iii => "iii",
            I::Iv => "iv",
            I::Ii => "ii",
        };
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.gamma-i.{class}"))
    }

    fn class_key(ductility: semio_s_artifact_norm_en1998_model::DuctilityClass) -> &'static str {
        match ductility {
            semio_s_artifact_norm_en1998_model::DuctilityClass::Dch => "dch",
            semio_s_artifact_norm_en1998_model::DuctilityClass::Dcm => "dcm",
            semio_s_artifact_norm_en1998_model::DuctilityClass::Dcl => "low",
        }
    }

    /// 🎛️ q = q₀·αu/α1·k_w with 0.5 ≤ k_w ≤ 1.0 and the lower bound q ≥ 1.5 (EN 1998-1 §5.2.2.2).
    pub fn behaviour_factor(q0: f64, alpha_u_over_alpha_1: f64, k_w: f64) -> f64 {
        (q0 * alpha_u_over_alpha_1 * k_w.clamp(0.5, 1.0)).max(1.5)
    }

    /// 🏚️ Reduction of q by 20 % for buildings irregular in elevation (EN 1998-1 Table 4.1).
    pub fn elevation_irregularity_q_factor(elevation_regular: bool) -> f64 {
        if elevation_regular { 1.0 } else { 0.8 }
    }

    /// 📐️ Upper bound on q for systemType×material×ductility (EN 1998-1 Tables 5.1/6.2/7.2/8.1/9.1 + DE-NA).
    pub fn q_max_for_system(system_type: semio_s_artifact_norm_en1998_model::SystemType, material: semio_s_artifact_norm_en1998_model::StructuralMaterial, ductility: semio_s_artifact_norm_en1998_model::DuctilityClass) -> f64 {
        use semio_s_artifact_norm_en1998_model::{StructuralMaterial as Mat, SystemType as Sys};
        let class = class_key(ductility);
        let system = match (material, system_type) {
            (Mat::Rc, Sys::Frame) => "rc.frame".to_owned(),
            (Mat::Rc, Sys::Wall) => "rc.wall".to_owned(),
            (Mat::Rc, Sys::Dual) => "rc.dual".to_owned(),
            (Mat::Steel, Sys::Frame) => "steel.frame".to_owned(),
            (Mat::Steel, Sys::Wall | Sys::Dual) => "steel.wall".to_owned(),
            (Mat::Composite, _) => "composite".to_owned(),
            (Mat::Timber, _) => "timber".to_owned(),
            (Mat::Masonry, _) => "masonry".to_owned(),
            _ => return semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.q-max.default"),
        };
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.q-max.{system}.{class}"))
    }

    /// 🧱 RC column min b [m] for DCM/DCH (§5.4.3.2.1 / §5.5.3.2.1 simplified).
    pub fn rc_column_min_b_m(ductility: semio_s_artifact_norm_en1998_model::DuctilityClass) -> f64 {
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.rc.column-min-b.{}", class_key(ductility)))
    }

    /// 🧱 RC beam min b [m] (§5.4.1.2.1 / §5.5.1.2.1 simplified, unverified row `ndp.en1998.en1.rc.beam-min-b.<class>`).
    pub fn rc_beam_min_b_m(ductility: semio_s_artifact_norm_en1998_model::DuctilityClass) -> f64 {
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.rc.beam-min-b.{}", class_key(ductility)))
    }

    /// 🧱 RC ρ bounds (min, max) simplified from §5.4.3 / §5.5.3, read from `ndp.en1998.en1.rc.rho-min|rho-max.<role>.<class>`.
    pub fn rc_rho_limits(role: &str, ductility: semio_s_artifact_norm_en1998_model::DuctilityClass) -> (f64, f64) {
        let role = match role.trim().to_ascii_lowercase().as_str() {
            "column" => "column",
            "beam" => "beam",
            _ => "other",
        };
        let class = class_key(ductility);
        (semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.rc.rho-min.{role}.{class}")), semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.rc.rho-max.{role}.{class}")))
    }

    pub fn rc_omega_wd_min(ductility: semio_s_artifact_norm_en1998_model::DuctilityClass) -> f64 {
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.rc.omega-wd-min.{}", class_key(ductility)))
    }

    /// 🏗️ Steel section class max allowed for given q (EN 1998-1 §6.5 / §6.6).
    pub fn steel_max_section_class(q: f64) -> u8 {
        if q > 4.0 { 1 } else if q > 2.0 { 2 } else { 3 }
    }

    pub fn drift_theta(limit_class: semio_s_artifact_norm_en1998_model::DriftLimitClass) -> f64 {
        let class = match limit_class {
            semio_s_artifact_norm_en1998_model::DriftLimitClass::Brittle => "brittle",
            semio_s_artifact_norm_en1998_model::DriftLimitClass::Isolated => "isolated",
            semio_s_artifact_norm_en1998_model::DriftLimitClass::Ductile => "ductile",
        };
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.drift-theta.{class}"))
    }

    /// 📈️ Elastic response spectrum S_e(T) [m/s²] per EN 1998-1 §3.2.2.2.
    pub fn elastic_response_spectrum(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64) -> f64 {
        elastic_response_spectrum_eta(a_g, s, tb, tc, td, t, 1.0)
    }

    /// 📈️ Elastic response spectrum with damping correction η (EN 1998-1 §3.2.2.2(3)).
    pub fn elastic_response_spectrum_eta(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64, eta: f64) -> f64 {
        if t <= 0.0 {
            return a_g * s;
        }
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

    /// 📉️ Design spectrum S_d(T) [m/s²] of EN 1998-1 Eq. 3.13–3.16 with a_g = γ_I·a_gR and β = 0.2.
    pub fn design_spectrum_at(a_gr: f64, gamma_i: f64, s: f64, tb: f64, tc: f64, td: f64, q: f64, t: f64) -> f64 {
        design_spectrum_horizontal(a_gr * gamma_i, s, tb, tc, td, q, t)
    }

    pub fn t1_from_ct(ct: f64, height_m: f64) -> f64 {
        ct * height_m.powf(0.75)
    }

    /// 🎻 Rayleigh estimate T₁ = 2π √(Σ m_i δ_i² / Σ f_i δ_i) with f_i = m_i g (unit lateral load ∝ weight).
    pub fn t1_rayleigh(masses_kg: &[f64], displacements_m: &[f64]) -> f64 {
        let mut num = 0.0;
        let mut den = 0.0;
        for (m, d) in masses_kg.iter().zip(displacements_m.iter()) {
            let f = *m * 9.81;
            num += m * d * d;
            den += f * d;
        }
        if den <= 0.0 || num <= 0.0 {
            return 0.0;
        }
        2.0 * std::f64::consts::PI * (num / den).sqrt()
    }

    /// 🎻 Rayleigh period of a shear building: deflections d_i = Σ_{j≤i} V_j/k_j under F_i = m_i·g; None when a storey stiffness is missing (no default shape).
    pub fn rayleigh_period_shear_building(masses_kg: &[f64], stiffness_n_per_m: &[f64]) -> Option<f64> {
        if masses_kg.is_empty() || masses_kg.len() != stiffness_n_per_m.len() || stiffness_n_per_m.iter().any(|k| *k <= 0.0) {
            return None;
        }
        let n = masses_kg.len();
        let mut shear = vec![0.0; n];
        let mut above = 0.0;
        for j in (0..n).rev() {
            above += masses_kg[j] * 9.81;
            shear[j] = above;
        }
        let mut disp = Vec::with_capacity(n);
        let mut sum = 0.0;
        for j in 0..n {
            sum += shear[j] / stiffness_n_per_m[j];
            disp.push(sum);
        }
        let t = t1_rayleigh(masses_kg, &disp);
        if t > 0.0 { Some(t) } else { None }
    }

    /// ⏱️ Fundamental period of a building by method (ct, rayleigh, given); the flag is false when the chosen method has no input and the Ct estimate stands in.
    pub fn fundamental_period_s(method: T1Method, ct: f64, t1_given_s: f64, total_height_m: f64, masses_kg: &[f64], stiffness_x: &[f64], stiffness_y: &[f64]) -> (f64, bool) {
        match method {
            T1Method::Given => (t1_given_s, t1_given_s > 0.0),
            T1Method::Rayleigh => {
                let periods: Vec<f64> = [rayleigh_period_shear_building(masses_kg, stiffness_x), rayleigh_period_shear_building(masses_kg, stiffness_y)].into_iter().flatten().collect();
                match periods.into_iter().reduce(f64::min) {
                    Some(t) => (t, true),
                    None => (t1_from_ct(ct, total_height_m), false),
                }
            }
            T1Method::Ct => (t1_from_ct(ct, total_height_m), true),
        }
    }

    pub fn lambda_factor(t1: f64, tc: f64, n_storeys: usize) -> f64 {
        if t1 <= 2.0 * tc && n_storeys > 2 { 0.85 } else { 1.0 }
    }

    pub fn base_shear_n(s_d: f64, mass_kg: f64, lambda: f64) -> f64 {
        s_d * mass_kg * lambda
    }

    /// 🪨 Design base shear of one system: q capped at q_max (Table 5.1) and reduced by 0.8 for irregular elevation (Table 4.1), S_d ≥ β·a_g, λ correction (EN 1998-1 §4.3.3.2.2). Shared by the base shear check and the foundation sliding demand.
    #[derive(Clone, Copy, Debug)]
    pub struct DesignBaseShear {
        pub q: f64,
        pub q_max: f64,
        pub s_d: f64,
        pub lambda: f64,
        pub f_b: f64,
    }

    /// 🪨 Computes [`DesignBaseShear`]; without a system q = 1.5 and no cap applies.
    pub fn design_base_shear(system: Option<&semio_s_artifact_norm_en1998_model::En1998System>, elevation_regular: bool, n_storeys: usize, total_mass_kg: f64, t1: f64, a_g: f64, gamma_i: f64, s: f64, tb: f64, tc: f64, td: f64) -> DesignBaseShear {
        let (q, q_max) = system.map_or((1.5, f64::INFINITY), |sys| (behaviour_factor(sys.q0, sys.alpha_u_over_alpha_1, sys.k_w), q_max_for_system(sys.system_type, sys.material, sys.ductility_class)));
        let s_d = design_spectrum_at(a_g, gamma_i, s, tb, tc, td, q.min(q_max) * elevation_irregularity_q_factor(elevation_regular), t1);
        let lambda = lambda_factor(t1, tc, n_storeys);
        DesignBaseShear { q, q_max, s_d, lambda, f_b: base_shear_n(s_d, total_mass_kg, lambda) }
    }

    /// 📊 Storey forces F_i ∝ m_i z_i (EN 1998-1 §4.3.3.2.3).
    pub fn storey_forces_n(f_b: f64, masses_kg: &[f64], heights_m: &[f64]) -> Vec<f64> {
        let mut z = 0.0;
        let mut zs = Vec::with_capacity(heights_m.len());
        for h in heights_m {
            z += h;
            zs.push(z);
        }
        let denom: f64 = masses_kg.iter().zip(zs.iter()).map(|(m, zi)| m * zi).sum();
        if denom <= 0.0 {
            return masses_kg.iter().map(|_| 0.0).collect();
        }
        masses_kg.iter().zip(zs.iter()).map(|(m, zi)| f_b * (m * zi) / denom).collect()
    }

    /// 🌀 Accidental torsion amplification δ = 1 + 0.6·x/L_e (EN 1998-1 §4.3.3.2.4(2)); x = distance of the element from the centre of mass, L_e = distance of the outermost resisting elements.
    pub fn torsion_amplification(x_m: f64, l_e_m: f64) -> f64 {
        1.0 + 0.6 * x_m.abs() / l_e_m.max(1e-6)
    }

    /// 🌀 Upper bound of δ for the simplified procedure: x ≤ L_e (structural eccentricity e₀ ≤ L_e/2) gives 1.6.
    pub fn torsion_amplification_simplified_limit() -> f64 {
        semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en.torsion.simplified-bound")
    }

    /// 📏 Damage limitation utilisation d_r·ν / (θ·h) (EN 1998-1 §4.4.3.2(1)); a non-positive limit yields infinity.
    pub fn drift_utilization(d_r_m: f64, nu: f64, theta: f64, height_m: f64) -> f64 {
        let limit = theta * height_m;
        if limit <= 0.0 { if d_r_m * nu > 0.0 { f64::INFINITY } else { 0.0 } } else { d_r_m * nu / limit }
    }

    /// 🏚️ Soft storey stiffness utilisation: lower storey stiffer than 0.70 of the upper one is flagged (EN 1998-1 §4.2.3.3), per direction.
    pub fn soft_storey_utilization(k_lower: f64, k_upper: f64) -> f64 {
        if k_upper <= 0.0 { 0.0 } else if k_lower <= 0.0 { f64::INFINITY } else { 0.70 * k_upper / k_lower }
    }

    /// 🔁 Second-order class of θ (EN 1998-1 §4.4.2.2(2)-(3)): amplification factor 1/(1-θ) for 0.1 < θ ≤ 0.2, None above 0.3.
    pub fn p_delta_amplification(theta: f64) -> Option<f64> {
        if theta <= 0.1 { Some(1.0) } else if theta <= 0.2 { Some(1.0 / (1.0 - theta)) } else if theta <= 0.3 { Some(1.0 / (1.0 - theta)) } else { None }
    }

    /// 🧭 Horizontal component combination E_x + 0.30 E_y and 0.30 E_x + E_y (EN 1998-1 §4.3.3.5.1(3)); returns the larger.
    pub fn horizontal_component_envelope(e_x: f64, e_y: f64) -> f64 {
        (e_x.abs() + 0.30 * e_y.abs()).max(0.30 * e_x.abs() + e_y.abs())
    }

    /// 📈 Damping correction η = max(√(10/(5+ξ)), 0.55) with ξ in percent (EN 1998-1 §3.2.2.2(3)).
    pub fn damping_correction(xi_percent: f64) -> f64 {
        (10.0 / (5.0 + xi_percent)).sqrt().max(0.55)
    }

    /// 📉 Design spectrum S_d(T) [m/s²] of EN 1998-1 §3.2.2.5 Eq. 3.13-3.16 with β = 0.2.
    pub fn design_spectrum_horizontal(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, q: f64, t: f64) -> f64 {
        let q = q.max(1.0);
        let beta = 0.2;
        let ags = a_g * s;
        if t <= tb {
            ags * (2.0 / 3.0 + t / tb * (2.5 / q - 2.0 / 3.0))
        } else if t <= tc {
            ags * 2.5 / q
        } else if t <= td {
            (ags * 2.5 / q * tc / t).max(beta * a_g)
        } else {
            (ags * 2.5 / q * tc * td / (t * t)).max(beta * a_g)
        }
    }

    /// ↕️ Vertical elastic spectrum S_ve(T) [m/s²] of EN 1998-1 §3.2.2.3 with a_vg = ratio·a_g, T_B = 0.05, T_C = 0.15, T_D = 1.0 (recommended values).
    pub fn vertical_elastic_spectrum(a_g: f64, avg_ratio: f64, t: f64) -> f64 {
        let (tb, tc, td, eta) = (0.05, 0.15, 1.0, 1.0);
        let avg = a_g * avg_ratio;
        if t <= tb {
            avg * (1.0 + t / tb * (3.0 * eta - 1.0))
        } else if t <= tc {
            avg * 3.0 * eta
        } else if t <= td {
            avg * 3.0 * eta * tc / t
        } else {
            avg * 3.0 * eta * tc * td / (t * t)
        }
    }

    /// 🔗 CQC modal correlation ρ_ij = 8ξ²(1+r)r^1.5 / ((1-r²)² + 4ξ²r(1+r)²) with r = ω_j/ω_i (EN 1998-1 Eq. 4.18 form).
    pub fn cqc_correlation(xi: f64, omega_i: f64, omega_j: f64) -> f64 {
        let r = omega_j / omega_i.max(1e-12);
        8.0 * xi * xi * (1.0 + r) * r.powf(1.5) / ((1.0 - r * r).powi(2) + 4.0 * xi * xi * r * (1.0 + r).powi(2))
    }

    /// 🔗 SRSS of modal responses (valid when T_j ≤ 0.9·T_i for all i < j).
    pub fn srss_combine(modal: &[f64]) -> f64 {
        modal.iter().map(|e| e * e).sum::<f64>().sqrt()
    }

    /// 🔗 CQC of modal responses for periods `t` and equal damping ξ.
    pub fn cqc_combine(modal: &[f64], periods_s: &[f64], xi: f64) -> f64 {
        let omega: Vec<f64> = periods_s.iter().map(|t| 2.0 * std::f64::consts::PI / t.max(1e-9)).collect();
        let mut sum = 0.0;
        for i in 0..modal.len() {
            for j in 0..modal.len() {
                sum += cqc_correlation(xi, omega[i], omega[j]) * modal[i] * modal[j];
            }
        }
        sum.max(0.0).sqrt()
    }

    /// 🧮 Shear building eigenproblem (K - ω²M)φ = 0 solved by cyclic Jacobi on M^-½ K M^-½; returns (periods [s] ascending, mode shapes φ[mode][storey]).
    pub fn shear_building_modes(masses_kg: &[f64], storey_stiffness_n_per_m: &[f64]) -> (Vec<f64>, Vec<Vec<f64>>) {
        let n = masses_kg.len();
        let mut a = vec![vec![0.0; n]; n];
        for i in 0..n {
            let k_i = storey_stiffness_n_per_m[i];
            let k_next = if i + 1 < n { storey_stiffness_n_per_m[i + 1] } else { 0.0 };
            a[i][i] = k_i + k_next;
            if i + 1 < n {
                a[i][i + 1] = -k_next;
                a[i + 1][i] = -k_next;
            }
        }
        for i in 0..n {
            for j in 0..n {
                a[i][j] /= (masses_kg[i] * masses_kg[j]).sqrt().max(1e-12);
            }
        }
        let mut v = vec![vec![0.0; n]; n];
        for i in 0..n { v[i][i] = 1.0; }
        for _ in 0..100 {
            let mut off = 0.0;
            for i in 0..n { for j in i + 1..n { off += a[i][j] * a[i][j]; } }
            if off < 1e-18 { break; }
            for p in 0..n {
                for q in p + 1..n {
                    if a[p][q].abs() < 1e-30 { continue; }
                    let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                    let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                    let t = if theta == 0.0 { 1.0 } else { t };
                    let c = 1.0 / (t * t + 1.0).sqrt();
                    let sn = t * c;
                    for k in 0..n {
                        let (akp, akq) = (a[k][p], a[k][q]);
                        a[k][p] = c * akp - sn * akq;
                        a[k][q] = sn * akp + c * akq;
                    }
                    for k in 0..n {
                        let (apk, aqk) = (a[p][k], a[q][k]);
                        a[p][k] = c * apk - sn * aqk;
                        a[q][k] = sn * apk + c * aqk;
                    }
                    for k in 0..n {
                        let (vkp, vkq) = (v[k][p], v[k][q]);
                        v[k][p] = c * vkp - sn * vkq;
                        v[k][q] = sn * vkp + c * vkq;
                    }
                }
            }
        }
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|x, y| a[*x][*x].partial_cmp(&a[*y][*y]).unwrap_or(std::cmp::Ordering::Equal));
        let mut periods = Vec::with_capacity(n);
        let mut shapes = Vec::with_capacity(n);
        for idx in order {
            let w2 = a[idx][idx].max(1e-12);
            periods.push(2.0 * std::f64::consts::PI / w2.sqrt());
            shapes.push((0..n).map(|i| v[i][idx] / masses_kg[i].max(1e-12).sqrt()).collect());
        }
        (periods, shapes)
    }

    /// ⚖️ Effective modal masses M_eff,n = (φᵀMr)²/(φᵀMφ) with influence vector r = 1.
    pub fn effective_modal_masses(masses_kg: &[f64], shapes: &[Vec<f64>]) -> Vec<f64> {
        shapes.iter().map(|phi| {
            let l: f64 = phi.iter().zip(masses_kg).map(|(p, m)| p * m).sum();
            let m_n: f64 = phi.iter().zip(masses_kg).map(|(p, m)| p * p * m).sum();
            if m_n <= 0.0 { 0.0 } else { l * l / m_n }
        }).collect()
    }

    /// 🏗️ Non-structural element force F_a = S_a·W_a·γ_a/q_a with S_a = α·S·[3(1+z/H)/(1+(1-T_a/T_1)²) - 0.5] ≥ α·S (EN 1998-1 §4.3.5.2).
    pub fn non_structural_force_n(alpha: f64, s: f64, z_m: f64, h_m: f64, t_a: f64, t_1: f64, w_a_n: f64, gamma_a: f64, q_a: f64) -> f64 {
        let ratio = if t_1 > 0.0 { t_a / t_1 } else { 0.0 };
        let s_a = (alpha * s * (3.0 * (1.0 + z_m / h_m.max(1e-9)) / (1.0 + (1.0 - ratio).powi(2)) - 0.5)).max(alpha * s);
        s_a * w_a_n * gamma_a / q_a.max(1.0)
    }

    /// 🧾️ Registry ids of the Table 3.1 thresholds that the ground classification reads; a check built on them reports the unverified basis.
    pub const GROUND_NDP_IDS: [&str; 12] = [
        "ndp.en1998.en1.ground.vs-a-min", "ndp.en1998.en1.ground.vs-b-min", "ndp.en1998.en1.ground.vs-c-min",
        "ndp.en1998.en1.ground.spt-b-min", "ndp.en1998.en1.ground.spt-c-min", "ndp.en1998.en1.ground.cu-b-min", "ndp.en1998.en1.ground.cu-c-min",
        "ndp.en1998.en1.ground.e.thickness-min-m", "ndp.en1998.en1.ground.e.thickness-max-m", "ndp.en1998.en1.ground.e.substratum-vs-min",
        "ndp.en1998.en1.ground.s1.thickness-min-m", "ndp.en1998.en1.ground.s1.plasticity-index-min",
    ];

    fn ground_threshold(key: &str) -> f64 { semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.ground.{key}")) }

    /// 🌍️ EN 1998-1 Table 3.1 ground class from v_s,30 [m/s] against the registry thresholds `ground.vs-{a,b,c}-min` (a value on a class boundary takes the lower class); 0 or less is not given.
    pub fn ground_class_from_vs30(v_s30: f64) -> Option<char> {
        if v_s30 <= 0.0 { None } else if v_s30 > ground_threshold("vs-a-min") { Some('A') } else if v_s30 > ground_threshold("vs-b-min") { Some('B') } else if v_s30 > ground_threshold("vs-c-min") { Some('C') } else { Some('D') }
    }

    /// 📐️ v_s,30 = 30 / Σ(h_i / v_i) over layers summing to 30 m (EN 1998-1 Eq. 3.1); empty or non-positive layers give 0.
    pub fn vs30_from_layers(layers: &[(f64, f64)]) -> f64 {
        let total: f64 = layers.iter().map(|(h, _)| h).sum();
        if layers.is_empty() || layers.iter().any(|(h, v)| *h <= 0.0 || *v <= 0.0) || total <= 0.0 { return 0.0; }
        total / layers.iter().map(|(h, v)| h / v).sum::<f64>()
    }

    /// 🌍️ Ground types of EN 1998-1 Table 3.1: A to D by the parameters, E the shallow alluvium, S1 and S2 the soils that need a special study.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GroundType { A, B, C, D, E, S1, S2 }

    impl GroundType {
        /// 🔤 Name as written in the table.
        pub fn name(self) -> &'static str {
            match self { Self::A => "A", Self::B => "B", Self::C => "C", Self::D => "D", Self::E => "E", Self::S1 => "S1", Self::S2 => "S2" }
        }

        /// 🅰️ Letter of the spectrum parameter table (Table 3.2 / 3.3); S1 and S2 have none and need a special study (EN 1998-1 3.1.2(4)).
        pub fn spectrum_letter(self) -> Option<char> {
            match self { Self::A => Some('A'), Self::B => Some('B'), Self::C => Some('C'), Self::D => Some('D'), Self::E => Some('E'), Self::S1 | Self::S2 => None }
        }
    }

    /// 🧱 Site entity of the ground classification: v_s,30 (Eq. 3.1), N_SPT (Eq. 3.2), c_u (Eq. 3.3) as time-averaged values (0 = not given), the surface alluvium and the soft clay or liquefiable layers of the profile.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct GroundInputs {
        pub v_s30_m_s: f64,
        pub n_spt: f64,
        pub c_u_kpa: f64,
        pub alluvium_thickness_m: f64,
        pub alluvium_v_s_m_s: f64,
        pub below_alluvium_v_s_m_s: f64,
        pub soft_clay_thickness_m: f64,
        pub soft_clay_pi: f64,
        pub liquefiable_or_sensitive: bool,
    }

    /// 🌍️ Ground type by EN 1998-1 Table 3.1 with the thresholds of the registry rows `ground.*` (a value on a class boundary takes the lower class): S2 for liquefiable or sensitive soils, S1 for a soft clay layer thick enough and with a high plasticity index, E for type C or D alluvium of the registered thickness range over a stiff substratum, otherwise A to D by v_s,30, then N_SPT, then c_u; `None` while none is given.
    pub fn ground_type_table_3_1(inputs: &GroundInputs) -> Option<GroundType> {
        if inputs.liquefiable_or_sensitive {
            return Some(GroundType::S2);
        }
        if inputs.soft_clay_thickness_m >= ground_threshold("s1.thickness-min-m") && inputs.soft_clay_pi > ground_threshold("s1.plasticity-index-min") {
            return Some(GroundType::S1);
        }
        let alluvium_class = ground_class_from_vs30(inputs.alluvium_v_s_m_s);
        if (ground_threshold("e.thickness-min-m")..=ground_threshold("e.thickness-max-m")).contains(&inputs.alluvium_thickness_m) && matches!(alluvium_class, Some('C' | 'D')) && inputs.below_alluvium_v_s_m_s > ground_threshold("e.substratum-vs-min") {
            return Some(GroundType::E);
        }
        let by_threshold = |value: f64, upper: f64, lower: f64| if value > upper { GroundType::B } else if value > lower { GroundType::C } else { GroundType::D };
        if inputs.v_s30_m_s > 0.0 {
            return Some(if inputs.v_s30_m_s > ground_threshold("vs-a-min") { GroundType::A } else { by_threshold(inputs.v_s30_m_s, ground_threshold("vs-b-min"), ground_threshold("vs-c-min")) });
        }
        if inputs.n_spt > 0.0 {
            return Some(by_threshold(inputs.n_spt, ground_threshold("spt-b-min"), ground_threshold("spt-c-min")));
        }
        if inputs.c_u_kpa > 0.0 {
            return Some(by_threshold(inputs.c_u_kpa, ground_threshold("cu-b-min"), ground_threshold("cu-c-min")));
        }
        None
    }

    /// 🧱 Table 3.1 inputs of the site entity (N_SPT, c_u, soft clay, liquefiable and alluvium fields) with the v_s,30 chosen by the caller (the soil profile value, or the site's own).
    pub fn ground_inputs_of_site(site: &super::En1998Site, v_s30_m_s: f64) -> GroundInputs {
        GroundInputs {
            v_s30_m_s,
            n_spt: site.n_spt,
            c_u_kpa: site.c_u_kpa,
            alluvium_thickness_m: site.alluvium_thickness_m,
            alluvium_v_s_m_s: site.alluvium_v_s_m_s,
            below_alluvium_v_s_m_s: site.below_alluvium_v_s_m_s,
            soft_clay_thickness_m: site.soft_clay_thickness_m,
            soft_clay_pi: site.soft_clay_pi,
            liquefiable_or_sensitive: site.liquefiable_layer,
        }
    }

    /// 🌍️ Ground letter of the spectrum parameters for the v_s,30 chosen by the caller: the Table 3.1 type derived from that value and the site's other classification inputs when it has a spectrum letter, else the declared EN ground type, else B.
    pub fn spectrum_ground_letter_for(site: &super::En1998Site, v_s30_m_s: f64) -> char {
        ground_type_table_3_1(&ground_inputs_of_site(site, v_s30_m_s))
            .and_then(GroundType::spectrum_letter)
            .or_else(|| Some(site.en_ground_type.letter()))
            .unwrap_or('B')
    }

    /// 🌍️ Ground letter of the spectrum parameters from the site's own inputs only (see [`spectrum_ground_letter_for`]); the document-level choice with foundation layers is [`spectrum_ground_letter_of`].
    pub fn spectrum_ground_letter(site: &super::En1998Site) -> char {
        spectrum_ground_letter_for(site, site.v_s30_m_s)
    }

    /// 🪨 Where the v_s,30 behind the spectrum comes from.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum GroundSource {
        FoundationLayers { foundation_id: String },
        Site,
    }

    impl GroundSource {
        /// 🗣️ Words for the soil column label: whether the spectra follow the layers of the foundation `own_foundation_id`, those of another foundation or the site input.
        pub fn describe(&self, own_foundation_id: &str, de: bool) -> String {
            match (self, de) {
                (Self::FoundationLayers { foundation_id }, true) if foundation_id == own_foundation_id => "Spektren folgen diesen Schichten".to_owned(),
                (Self::FoundationLayers { foundation_id }, false) if foundation_id == own_foundation_id => "spectra follow these layers".to_owned(),
                (Self::FoundationLayers { foundation_id }, true) => format!("Spektren folgen den Schichten von {foundation_id}"),
                (Self::FoundationLayers { foundation_id }, false) => format!("spectra follow the layers of {foundation_id}"),
                (Self::Site, true) => "Spektren folgen der Standorteingabe".to_owned(),
                (Self::Site, false) => "spectra follow the site input".to_owned(),
            }
        }
    }

    /// 🪨 v_s,30 of the soil layers of one foundation (Eq. 3.1 over the top 30 m, layers ordered top to bottom); 0 without usable layers.
    pub fn foundation_v_s30_m_s(document: &super::En1998Snapshot, foundation_id: &str) -> f64 {
        let layers: Vec<(f64, f64)> = document.soil_layers.iter().filter(|l| l.foundation_id == foundation_id).map(|l| (l.thickness_m, l.shear_wave_velocity_m_s)).collect();
        time_averaged_over_depth(&layers, 30.0)
    }

    /// 🪨 The v_s,30 the spectrum follows, by this precedence: (1) the soil layers of the foundations, when at least one foundation has layers with a positive v_s,30, the lowest value governs because the spectrum is one document-level quantity and softer ground is the conservative choice; (2) otherwise the site's own `v_s30_m_s`. N_SPT, c_u, the soft clay, alluvium and liquefiable inputs always come from the site.
    pub fn effective_v_s30(document: &super::En1998Snapshot) -> (f64, GroundSource) {
        document.foundations.iter()
            .map(|f| (foundation_v_s30_m_s(document, &f.id), f))
            .filter(|(v, _)| *v > 0.0)
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map_or((document.site.v_s30_m_s, GroundSource::Site), |(v, f)| (v, GroundSource::FoundationLayers { foundation_id: f.id.clone() }))
    }

    /// 🌍️ Ground letter of the spectra of the document: Table 3.1 on the [`effective_v_s30`] (foundation layers before the site's own value).
    pub fn spectrum_ground_letter_of(document: &super::En1998Snapshot) -> char {
        spectrum_ground_letter_for(&document.site, effective_v_s30(document).0)
    }

    /// 📐️ Time-averaged value over the layers `(thickness, value)` for N_SPT (Eq. 3.2) and c_u (Eq. 3.3), the same harmonic average over the thickness as v_s,30; empty or non-positive layers give 0.
    pub fn harmonic_average_over_layers(layers: &[(f64, f64)]) -> f64 {
        vs30_from_layers(layers)
    }

    /// 🌍️ Time-averaged value over the top `depth_m` of the layers `(thickness, value)` (Eq. 3.1 with H = `depth_m`): layers below the depth are cut off and a profile shorter than the depth continues with its last layer; 0 while a value or thickness is not positive.
    pub fn time_averaged_over_depth(layers: &[(f64, f64)], depth_m: f64) -> f64 {
        let Some(last) = layers.last() else { return 0.0 };
        if depth_m <= 0.0 || layers.iter().any(|(h, v)| *h <= 0.0 || *v <= 0.0) {
            return 0.0;
        }
        let mut top = 0.0;
        let mut cut: Vec<(f64, f64)> = Vec::new();
        for (h, v) in layers {
            let take = h.min((depth_m - top).max(0.0));
            if take > 0.0 { cut.push((take, *v)); }
            top += h;
        }
        if top < depth_m { cut.push((depth_m - top, last.1)); }
        harmonic_average_over_layers(&cut)
    }

    /// 📏️ Plan slenderness λ = L_max / L_min with the limit 4 (EN 1998-1 §4.2.3.2(4)); utilisation λ / 4.
    pub fn plan_slenderness_utilization(lx_m: f64, ly_m: f64) -> f64 {
        if lx_m <= 0.0 || ly_m <= 0.0 { return f64::INFINITY; }
        lx_m.max(ly_m) / lx_m.min(ly_m) / 4.0
    }

    /// 📐️ Plan recess utilisation: area between outline and convex envelope per floor area against 5 % (EN 1998-1 §4.2.3.2(3)).
    pub fn plan_envelope_utilization(gap_ratio: f64) -> f64 { gap_ratio.max(0.0) / semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.plan-gap-ratio") }

    /// 📐️ Set-back utilisation: gradual set-back per floor against 20 % of the plan dimension below (EN 1998-1 §4.2.3.3(5)a, unverified for the other cases b to d).
    pub fn setback_utilization(setback_ratio: f64) -> f64 { setback_ratio.max(0.0) / 0.20 }

    /// 🧱 Seismic joint gap to a neighbouring unit d ≥ √(d₁² + d₂²) (EN 1998-1 §4.4.2.7(2)).
    pub fn seismic_joint_gap_m(d1_m: f64, d2_m: f64) -> f64 {
        (d1_m * d1_m + d2_m * d2_m).sqrt()
    }

    /// 🪵️ Timber connection ductility utilisation: fastener diameter against 12 mm and thickness against 8 d (unverified NDPs `en1.timber.max-diameter-mm` and `en1.timber.min-thickness-factor`).
    pub fn timber_connection_utilization(d_mm: f64, t_mm: f64) -> f64 {
        if d_mm <= 0.0 { return 0.0; }
        let (d_max, factor) = (semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.timber.max-diameter-mm"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.timber.min-thickness-factor"));
        (d_mm / d_max).max(if t_mm <= 0.0 { f64::INFINITY } else { factor * d_mm / t_mm })
    }

    /// 🪨️ Masonry wall slenderness utilisation: h_ef / t_ef against 12 and t_ef against the 0.24 m minimum (unverified NDP `en1.masonry.slenderness`, unreinforced masonry of units other than natural stone).
    pub fn masonry_wall_utilization(h_ef_m: f64, t_ef_m: f64) -> f64 {
        if t_ef_m <= 0.0 { return 0.0; }
        let (slenderness, t_min) = (semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.masonry.slenderness-max"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.masonry.min-thickness-m"));
        (h_ef_m / t_ef_m / slenderness).max(t_min / t_ef_m)
    }

    /// 🏛️ Column/beam capacity-design ratio ΣM_Rc / (γ_Rd·ΣM_Rb) with γ_Rd = 1.3 (EN 1998-1 §4.4.2.3(4)); needs ≥ 1.0.
    pub fn strong_column_ratio(sum_m_rc: f64, sum_m_rb: f64) -> f64 {
        if sum_m_rb <= 0.0 { f64::INFINITY } else { sum_m_rc / (STRONG_COLUMN_FACTOR * sum_m_rb) }
    }

    /// 🏛️ Over-strength factor γ_Rd = 1.3 of the column/beam capacity design (EN 1998-1 §4.4.2.3(4), recommended value for DCM and DCH).
    pub const STRONG_COLUMN_FACTOR: f64 = 1.3;

    /// 📐️ Interstorey drift sensitivity θ = (P_tot · d_r) / (V_tot · h) per EN 1998-1 §4.4.2.2.
    pub fn drift_sensitivity(p_tot_n: f64, d_r_m: f64, v_tot_n: f64, h_m: f64) -> f64 {
        if v_tot_n.abs() < 1e-12 || h_m.abs() < 1e-12 { return if p_tot_n * d_r_m > 0.0 { f64::INFINITY } else { 0.0 }; }
        (p_tot_n * d_r_m) / (v_tot_n * h_m)
    }

    pub fn lateral_force_method_applicable(t1: f64, tc: f64, elevation_regular: bool) -> bool {
        elevation_regular && t1 <= 4.0 * tc && t1 <= 2.0
    }

    /// 🧱 DE simple masonry Table NA.12-style wall-area ratio limit, stepped by the reference acceleration a_gR [m/s²] (below 0.5: 2 %, below 0.7: 3 %, else 4 %; none without seismic action; unverified).
    pub fn masonry_wall_ratio_limit(a_gr: f64, importance: semio_s_artifact_norm_en1998_model::ImportanceClass) -> f64 {
        let band = if a_gr <= 0.0 { "none" } else if a_gr < 0.5 { "lt0-5" } else if a_gr < 0.7 { "lt0-7" } else { "ge0-7" };
        let base = semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en1.masonry-wall-ratio.{band}"));
        base * match importance {
            semio_s_artifact_norm_en1998_model::ImportanceClass::Iii | semio_s_artifact_norm_en1998_model::ImportanceClass::Iv => semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.masonry-wall-ratio.importance-iii-iv"),
            _ => 1.0,
        }
    }

    /// 🧾️ Registry ids that the simple masonry wall-area limit reads; a check built on them reports the unverified basis.
    pub const MASONRY_NDP_IDS: [&str; 5] = [
        "ndp.en1998.en1.masonry-wall-ratio.none", "ndp.en1998.en1.masonry-wall-ratio.lt0-5", "ndp.en1998.en1.masonry-wall-ratio.lt0-7",
        "ndp.en1998.en1.masonry-wall-ratio.ge0-7", "ndp.en1998.en1.masonry-wall-ratio.importance-iii-iv",
    ];

    /// 🧱 One shear wall row of a simple masonry building (EN 1998-1 §9.7): plan length, thickness, axis direction from the plan x axis in radians and the id of the storey row it belongs to (empty = every storey).
    #[derive(Clone, Debug, PartialEq)]
    pub struct MasonryWall {
        pub length_m: f64,
        pub thickness_m: f64,
        pub direction_rad: f64,
        pub storey_id: String,
    }

    impl MasonryWall {
        /// 🏬 Whether the wall counts in the storey row `storey_id`: a wall without a storey reference counts in every storey.
        pub fn in_storey(&self, storey_id: &str) -> bool {
            self.storey_id.is_empty() || self.storey_id == storey_id
        }

        /// 🧭 Whether the wall axis is closer to the plan y axis than to the x axis.
        pub fn along_y(&self) -> bool {
            self.direction_rad.sin().abs() > self.direction_rad.cos().abs()
        }
    }

    /// 🧱 Whether a member row is a masonry wall row: kind wall, material masonry, positive length and thickness; the other kinds never carry wall fields into a check.
    pub fn is_masonry_wall(member: &super::En1998Member) -> bool {
        member.role == super::MemberKind::Wall && member.material == semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry && member.wall_length_m > 0.0 && member.wall_thickness_m > 0.0
    }

    /// 🧱 The masonry wall rows among the member rows (see [`is_masonry_wall`]).
    pub fn masonry_walls_of(members: &[super::En1998Member]) -> Vec<MasonryWall> {
        members.iter()
            .filter(|m| is_masonry_wall(m))
            .map(|m| MasonryWall { length_m: m.wall_length_m, thickness_m: m.wall_thickness_m, direction_rad: m.wall_direction_rad, storey_id: m.wall_storey_id.clone() })
            .collect()
    }

    /// 🧱 Wall-area density Σ l·t / A of the walls running along the plan y axis (else x) in the storey row `storey_id`, with A the floor area; a wall belongs to the direction its axis is closer to and 0 for no floor area.
    pub fn masonry_wall_density(walls: &[MasonryWall], floor_area_m2: f64, along_y: bool, storey_id: &str) -> f64 {
        if floor_area_m2 <= 0.0 {
            return 0.0;
        }
        walls.iter()
            .filter(|w| w.in_storey(storey_id) && w.along_y() == along_y)
            .map(|w| w.length_m * w.thickness_m)
            .sum::<f64>() / floor_area_m2
    }

    /// 🧱 Governing wall-area density over the storey rows `storey_ids` (bottom to top; no rows means one anonymous storey) in both plan directions: the smallest one with its direction (`along_y`) and storey id; `None` without walls. A wall whose storey id names no row counts in no storey (the integrity check reports it).
    pub fn masonry_governing_density(walls: &[MasonryWall], floor_area_m2: f64, storey_ids: &[&str]) -> Option<(f64, bool, String)> {
        if walls.is_empty() {
            return None;
        }
        let anonymous = [""];
        let ids: &[&str] = if storey_ids.is_empty() { &anonymous } else { storey_ids };
        ids.iter()
            .flat_map(|id| [false, true].map(|along_y| (masonry_wall_density(walls, floor_area_m2, along_y, id), along_y, (*id).to_owned())))
            .min_by(|a, b| a.0.total_cmp(&b.0))
    }

    /// 🌋️ Participation Γ = Σ m φ / Σ m φ² and equivalent mass m* = Σ m φ of the N2 single-degree-of-freedom system (EN 1998-1 Annex B.2).
    pub fn n2_participation(masses_kg: &[f64], shape: &[f64]) -> Option<(f64, f64)> {
        let m_star: f64 = masses_kg.iter().zip(shape).map(|(m, p)| m * p).sum();
        let denominator: f64 = masses_kg.iter().zip(shape).map(|(m, p)| m * p * p).sum();
        if m_star <= 0.0 || denominator <= 0.0 { None } else { Some((m_star / denominator, m_star)) }
    }

    /// 🌋️ Storey shear coefficients S_i = Σ_{j≥i} m_j φ_j of the load pattern F_j = λ m_j φ_j.
    pub fn n2_storey_shear_coefficients(masses_kg: &[f64], shape: &[f64]) -> Vec<f64> {
        let mut out: Vec<f64> = masses_kg.iter().zip(shape).map(|(m, p)| m * p).collect();
        for i in (0..out.len().saturating_sub(1)).rev() {
            out[i] += out[i + 1];
        }
        out
    }

    /// 🌋️ Elastic-plastic capacity curve of the shear building under the pattern λ m φ: the weakest storey yields at λ_y = min V_R,i/S_i, the base shear is λ_y S_1 and the control node (roof) displacement Σ λ_y S_i/k_i; returns (d_y, V_y, governing storey index), `None` for missing storey data.
    pub fn n2_pushover_yield(masses_kg: &[f64], stiffness: &[f64], resistance_n: &[f64], shape: &[f64]) -> Option<(f64, f64, usize)> {
        let s = n2_storey_shear_coefficients(masses_kg, shape);
        if s.is_empty() || stiffness.len() != s.len() || resistance_n.len() != s.len() || stiffness.iter().chain(resistance_n).any(|v| *v <= 0.0) || s.iter().any(|v| *v <= 0.0) { return None; }
        let (index, lambda) = resistance_n.iter().zip(&s).map(|(r, c)| r / c).enumerate().fold((0, f64::INFINITY), |best, (i, v)| if v < best.1 { (i, v) } else { best });
        let displacement: f64 = s.iter().zip(stiffness).map(|(c, k)| lambda * c / k).sum();
        Some((displacement, lambda * s[0], index))
    }

    /// 🌋️ Force of a piecewise linear capacity curve (points sorted by displacement) at displacement d, held constant beyond the last point.
    pub fn curve_force_at(points: &[(f64, f64)], d: f64) -> f64 {
        let mut previous = (0.0, 0.0);
        for p in points {
            if d <= p.0 {
                return if p.0 > previous.0 { previous.1 + (p.1 - previous.1) * (d - previous.0) / (p.0 - previous.0) } else { p.1 };
            }
            previous = *p;
        }
        previous.1
    }

    /// 🌋️ Area under a piecewise linear capacity curve from the origin up to displacement d.
    pub fn curve_energy(points: &[(f64, f64)], d: f64) -> f64 {
        let mut area = 0.0;
        let mut previous = (0.0, 0.0);
        for p in points {
            if previous.0 >= d { break; }
            let end = p.0.min(d);
            area += 0.5 * (previous.1 + curve_force_at(points, end)) * (end - previous.0);
            previous = *p;
        }
        if previous.0 < d { area += previous.1 * (d - previous.0); }
        area
    }

    /// 🌋️ Equal-energy bilinear idealisation of the SDOF capacity curve up to d_m* (EN 1998-1 Annex B.3): F_y* is the maximum force up to d_m*, d_y* = 2 (d_m* − E_m*/F_y*).
    pub fn n2_bilinear(points: &[(f64, f64)], d_m_star: f64) -> Option<(f64, f64)> {
        let f_y = points.iter().filter(|p| p.0 <= d_m_star).map(|p| p.1).fold(curve_force_at(points, d_m_star), f64::max);
        if f_y <= 0.0 || d_m_star <= 0.0 { return None; }
        let d_y = 2.0 * (d_m_star - curve_energy(points, d_m_star) / f_y);
        if d_y <= 0.0 { None } else { Some((f_y, d_y)) }
    }

    /// 🌋️ N2 target displacement of the SDOF system (EN 1998-1 Annex B.5): T* = 2π √(m* d_y*/F_y*), d_et* = S_e(T*) (T*/2π)², d_t* = d_et* for T* ≥ T_C or F_y*/m* ≥ S_e(T*), else d_et*/q_u (1 + (q_u − 1) T_C/T*).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct N2Target {
        pub t_star: f64,
        pub q_u: f64,
        pub d_et_star: f64,
        pub d_t_star: f64,
    }

    pub fn n2_target(s_e: &dyn Fn(f64) -> f64, t_c: f64, m_star: f64, f_y_star: f64, d_y_star: f64) -> N2Target {
        let t_star = 2.0 * std::f64::consts::PI * (m_star * d_y_star / f_y_star).sqrt();
        let s_e_star = s_e(t_star);
        let q_u = s_e_star * m_star / f_y_star;
        let d_et_star = s_e_star * (t_star / (2.0 * std::f64::consts::PI)).powi(2);
        let d_t_star = if t_star >= t_c || q_u <= 1.0 { d_et_star } else { d_et_star / q_u * (1.0 + (q_u - 1.0) * t_c / t_star) };
        N2Target { t_star, q_u, d_et_star, d_t_star }
    }

    /// 🌋️ Iterated N2 solution on the SDOF curve: the bilinear idealisation is redone for d_m* = d_t* until the target displacement settles (EN 1998-1 Annex B.3 and B.5); returns the target and the idealisation (F_y*, d_y*).
    pub fn n2_solve(points: &[(f64, f64)], s_e: &dyn Fn(f64) -> f64, t_c: f64, m_star: f64) -> Option<(N2Target, (f64, f64))> {
        let last = points.last()?.0;
        let mut d_m = last;
        let mut result = None;
        for _ in 0..24 {
            let (f_y, d_y) = n2_bilinear(points, d_m)?;
            let target = n2_target(s_e, t_c, m_star, f_y, d_y);
            let settled = (target.d_t_star - d_m).abs() <= 1e-9 * d_m.max(1e-12);
            result = Some((target, (f_y, d_y)));
            if settled { break; }
            d_m = target.d_t_star.min(last).max(1e-12);
        }
        result
    }

    /// 🌋️ N2 assessment of a shear building in one direction: SDOF transformation, elastic-plastic pushover (the weakest storey yields first) and the target displacement of the roof; `weakest_drift_m` is the drift of that storey at the plateau.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct N2Assessment {
        pub gamma: f64,
        pub m_star: f64,
        pub yield_displacement: f64,
        pub yield_shear: f64,
        pub weakest: usize,
        pub weakest_drift_m: f64,
        pub target: N2Target,
        pub target_roof_m: f64,
    }

    pub fn n2_assess(masses_kg: &[f64], stiffness: &[f64], resistance_n: &[f64], shape: &[f64], s_e: &dyn Fn(f64) -> f64, t_c: f64) -> Option<N2Assessment> {
        let (gamma, m_star) = n2_participation(masses_kg, shape)?;
        let (yield_displacement, yield_shear, weakest) = n2_pushover_yield(masses_kg, stiffness, resistance_n, shape)?;
        let s = n2_storey_shear_coefficients(masses_kg, shape);
        let weakest_drift_m = yield_shear / s[0] * s[weakest] / stiffness[weakest];
        let sdof = [(0.0, 0.0), (yield_displacement / gamma, yield_shear / gamma), (1.0e3 * yield_displacement / gamma, yield_shear / gamma)];
        let (target, _) = n2_solve(&sdof, s_e, t_c, m_star)?;
        Some(N2Assessment { gamma, m_star, yield_displacement, yield_shear, weakest, weakest_drift_m, target, target_roof_m: gamma * target.d_t_star })
    }

    /// 📉 Point of the acceleration-displacement response spectrum (ADRS) of an elastic spectrum value `s_e` at the period `t`: S_d = S_e (T/2π)² and S_a = S_e (EN 1998-1 Annex B.5).
    pub fn n2_adrs_point(s_e: f64, t: f64) -> (f64, f64) {
        (s_e * (t / (2.0 * std::f64::consts::PI)).powi(2), s_e)
    }

    /// 📉 Acceleration S_a = (2π/T)² S_d of the radial line of the period `t` in the ADRS format; the line through the yield point of the capacity spectrum has the period T*.
    pub fn n2_adrs_radial_sa(t: f64, s_d: f64) -> f64 {
        (2.0 * std::f64::consts::PI / t).powi(2) * s_d
    }

    /// 📉 Plateau acceleration S_a = F_y*/m* of the capacity spectrum of the SDOF system in the ADRS format.
    pub fn n2_adrs_capacity_sa(f_y_star: f64, m_star: f64) -> f64 {
        if m_star > 0.0 { f_y_star / m_star } else { 0.0 }
    }

    /// 🌋️ Roof displacement capacity of the pushover mechanism: the other storeys stay at their plateau drift, the weakest storey of height h deforms up to its drift capacity (taken as the chord rotation capacity of the limit state).
    pub fn n2_roof_capacity_m(assessment: &N2Assessment, weakest_storey_height_m: f64, drift_capacity_rad: f64) -> f64 {
        (assessment.yield_displacement - assessment.weakest_drift_m) + drift_capacity_rad * weakest_storey_height_m
    }

    /// 🌋️ Drift capacity of the weakest storey at which the roof capacity equals the target displacement.
    pub fn n2_required_drift_capacity_rad(assessment: &N2Assessment, weakest_storey_height_m: f64) -> f64 {
        if weakest_storey_height_m <= 0.0 { return f64::INFINITY; }
        (assessment.target_roof_m - (assessment.yield_displacement - assessment.weakest_drift_m)) / weakest_storey_height_m
    }

    /// 📈 Outcome of the acceptance test of an accelerogram set: record count, lowest ratio of the mean record spectrum to the elastic spectrum over the period range, and the two verdicts.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AccelerogramAcceptance {
        pub records: usize,
        pub lowest_mean_ratio: f64,
        pub enough_records: bool,
        pub spectrum_matched: bool,
    }

    /// 📈 Acceptance of a set of accelerograms (EN 1998-1 3.2.3.1.2): `record_spectra[r][p]` is the response spectrum of record r at the period p of the range 0.2 T1 to 2 T1 and `elastic[p]` the elastic spectrum there; the set needs at least `min_records` records and a mean spectrum of at least `min_ratio` times the elastic spectrum at every period.
    pub fn accelerogram_set_acceptance(record_spectra: &[Vec<f64>], elastic: &[f64], min_records: usize, min_ratio: f64) -> AccelerogramAcceptance {
        let records = record_spectra.len();
        let lowest_mean_ratio = elastic
            .iter()
            .enumerate()
            .map(|(p, e)| if records == 0 || *e <= 0.0 { f64::NAN } else { record_spectra.iter().map(|r| r.get(p).copied().unwrap_or(f64::NAN)).sum::<f64>() / records as f64 / e })
            .fold(f64::INFINITY, |low, ratio| if low.is_nan() || ratio.is_nan() { f64::NAN } else { low.min(ratio) });
        AccelerogramAcceptance { records, lowest_mean_ratio, enough_records: records >= min_records, spectrum_matched: lowest_mean_ratio >= min_ratio }
    }
}

pub mod part_2 {
    #[cfg(test)] use crate::*;
    /// 🌉 Capacity-design shear of a ductile pier V_CD = γ_0·M_Rd / L_s (EN 1998-2 §5.6.3.3 principle; γ_0 is the unverified NDP `en2.pier.gamma-0`).
    pub fn pier_capacity_design_shear_n(gamma_0: f64, m_rd_nm: f64, pier_height_m: f64) -> f64 {
        if pier_height_m <= 0.0 { return f64::INFINITY; }
        gamma_0 * m_rd_nm / pier_height_m
    }
    /// 🌉 Behaviour factor of a bridge: isolated (T_eff ≥ 3·T_fixed, EN 1998-2 §7) is designed elastically with q = 1.0, otherwise limited ductile q = 1.5.
    pub fn design_behaviour_factor(period_ratio: f64) -> f64 {
        if period_ratio >= 3.0 { 1.0 } else { 1.5 }
    }
    /// 🌉 Design bearing displacement d = S_e(T)·(T/2π)² (EN 1998-2 §7, elastic displacement spectrum).
    pub fn bearing_displacement_m(s_e: f64, fundamental_period_s: f64) -> f64 {
        let t = fundamental_period_s.max(1e-6);
        s_e * (t / (2.0 * std::f64::consts::PI)).powi(2)
    }
    /// 🌉 Importance factor γ_I of a bridge class (1 = I, 2 = II, 3 = III; other values are rounded into the range) from the unverified rows `ndp.en1998.en2.gamma-i.*` (EN 1998-2 2.1), `a_g = γ_I a_gR`.
    pub fn bridge_gamma_i(importance_class: f64) -> f64 {
        let class = match importance_class.round().clamp(1.0, 3.0) as u8 { 1 => "i", 3 => "iii", _ => "ii" };
        semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1998.en2.gamma-i.{class}"))
    }
    /// 🌉 Vertical elastic spectrum of a bridge `S_ve(T)` with `a_vg = ratio·a_g` (ratio from the unverified row `en2.spectrum.vertical-ratio`, EN 1998-2 3.2) on the corner periods of [`part_1::vertical_elastic_spectrum`].
    pub fn bridge_vertical_spectrum(a_g_design: f64, avg_ratio: f64, t: f64) -> f64 {
        super::part_1::vertical_elastic_spectrum(a_g_design, avg_ratio, t)
    }
    /// 🌉 Shear-span ratio `α_s = L_v / h` of a pier with the shear span `L_v = M/V` (the pier height for a cantilever) and the section depth `h` in the bending direction; infinity without depth.
    pub fn shear_span_ratio(shear_span_m: f64, section_depth_m: f64) -> f64 {
        if section_depth_m <= 0.0 { f64::INFINITY } else { shear_span_m / section_depth_m }
    }
    /// 🌉 Behaviour factor of a vertical RC pier in bending `q = q_0·λ(α_s)` with `λ = 1` from `full` and `λ = √(α_s/full)` between `min` and `full` (EN 1998-2 4.1.6, constants from the unverified rows `en2.q.*`); `None` below `min` (squat pier outside the rule).
    pub fn rc_pier_behaviour_factor(q0: f64, alpha_s: f64, full: f64, min: f64) -> Option<f64> {
        if !(alpha_s >= min) { None } else if alpha_s >= full { Some(q0) } else { Some(q0 * (alpha_s / full).sqrt()) }
    }
    /// 🌉 Combination of the seismic components `E_x + f E_y + f E_z` with the permutation of the three components that gives the largest sum of absolute values (EN 1998-2 4.1.8, `f` from the unverified row `en2.combination.minor-factor`).
    pub fn component_combination(ex: f64, ey: f64, ez: f64, minor_factor: f64) -> f64 {
        let (x, y, z) = (ex.abs(), ey.abs(), ez.abs());
        (x + minor_factor * (y + z)).max(y + minor_factor * (x + z)).max(z + minor_factor * (x + y))
    }
}

pub mod part_3 {
    use semio_s_plugin_norm_mechanics::section::fibre::{elastic_plastic, parabola_rectangle, rectangle_fibres, Fibre, FibreSection, Limits, Reinforcement};

    /// 🔧 Chord rotation capacity of a member for the limit state: DL uses θ_y, NC uses θ_um, SD uses the unverified fraction `en3.chord-rotation.sd-factor` of θ_um (EN 1998-3 §2.3 and Annex A principles); `None` for an unknown limit state.
    pub fn chord_rotation_capacity_rad(limit_state: &str, theta_y: f64, theta_um: f64, sd_factor: f64) -> Option<f64> {
        match limit_state.trim().to_ascii_lowercase().as_str() {
            "dl" | "damage_limitation" | "damage-limitation" => Some(theta_y),
            "sd" | "significant_damage" | "significant-damage" => Some(sd_factor * theta_um),
            "nc" | "near_collapse" | "near-collapse" => Some(theta_um),
            _ => None,
        }
    }
    /// 🔧 Confidence factor of EN 1998-3 Table 3.1; an unknown knowledge level is treated like KL1 (the most conservative).
    pub fn confidence_factor(kl: semio_s_artifact_norm_en1998_model::KnowledgeLevel) -> f64 {
        semio_s_plugin_norm_compliance::ndp_common(match kl {
            semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl1 => "ndp.en1998.en3.cf.kl1",
            semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl2 => "ndp.en1998.en3.cf.kl2",
            semio_s_artifact_norm_en1998_model::KnowledgeLevel::Kl3 => "ndp.en1998.en3.cf.kl3",
        })
    }
    /// 📅 Return period [yr] of the limit state: the registry row `ndp.en1998.en3.na.return-period` where the annex fixes it (NA-DE, all limit states), else the EN 1998-3 §2.1 recommendation (NC 2475, SD 475, DL 225).
    pub fn limit_state_return_period_yr(limit_state: &str, annex: semio_s_plugin_norm_compliance::AnnexChoice) -> f64 {
        if let Some(fixed) = semio_s_plugin_norm_compliance::ndp_number_opt("ndp.en1998.en3.na.return-period", annex) {
            return fixed;
        }
        semio_s_plugin_norm_compliance::ndp_common(match limit_state.trim().to_ascii_lowercase().as_str() {
            "nc" | "near_collapse" | "near-collapse" => "ndp.en1998.en3.return-period.nc",
            "dl" | "damage_limitation" | "damage-limitation" => "ndp.en1998.en3.return-period.dl",
            _ => "ndp.en1998.en3.return-period.sd",
        })
    }
    /// 📅 a_g multiplier relative to the 475 year design value: (T_R/475)^(1/k) with k = 3 (EN 1998-1 §2.1(4) note), T_R of the limit state under the annex.
    pub fn limit_state_ag_factor(limit_state: &str, annex: semio_s_plugin_norm_compliance::AnnexChoice) -> f64 {
        (limit_state_return_period_yr(limit_state, annex) / semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en3.return-period.sd")).powf(1.0 / semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en3.return-period.exponent"))
    }
    /// 🔧 Whether CF applies for the limit state (DL uses CF = 1.0).
    pub fn limit_state_applies_cf(limit_state: &str) -> bool {
        !matches!(
            limit_state.trim().to_ascii_lowercase().as_str(),
            "dl" | "damage_limitation" | "damage-limitation"
        )
    }
    pub fn design_capacity_n(r_k_n: f64, cf: f64, gamma_el: f64) -> f64 {
        if cf <= 0.0 || gamma_el <= 0.0 { 0.0 } else { r_k_n / (cf * gamma_el) }
    }
    /// 🔧 Coefficients of the chord rotation expressions of EN 1998-3 Annex A (A.1 ultimate, A.10b yield), the steel modulus and the material strains of the fibre section (parabola-rectangle concrete up to `max_concrete_mpa`, elastic perfectly plastic steel) read from the registry; all rows are unverified.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexA {
        pub y_base: f64,
        pub y_slenderness: f64,
        pub y_bar_slip: f64,
        pub u_base: f64,
        pub u_axial: f64,
        pub u_reinforcement: f64,
        pub u_shear_span: f64,
        pub u_confinement: f64,
        pub u_diagonal: f64,
        pub steel_modulus_mpa: f64,
        pub eps_c2_permille: f64,
        pub eps_cu_permille: f64,
        pub parabola_exponent: f64,
        pub eps_su_permille: f64,
        pub max_concrete_mpa: f64,
    }

    impl AnnexA {
        pub fn from_registry(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
            let n = |key: &str| semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1998.en3.annex-a.{key}"), annex);
            Self { y_base: n("theta-y.base"), y_slenderness: n("theta-y.slenderness"), y_bar_slip: n("theta-y.bar-slip"), u_base: n("theta-um.base"), u_axial: n("theta-um.axial"), u_reinforcement: n("theta-um.reinforcement"), u_shear_span: n("theta-um.shear-span"), u_confinement: n("theta-um.confinement"), u_diagonal: n("theta-um.diagonal"), steel_modulus_mpa: n("steel-modulus-mpa"), eps_c2_permille: n("section.eps-c2-permille"), eps_cu_permille: n("section.eps-cu-permille"), parabola_exponent: n("section.parabola-exponent"), eps_su_permille: n("section.eps-su-permille"), max_concrete_mpa: n("section.max-concrete-mpa") }
        }
    }

    /// 🏛️ Rectangular reinforced concrete member end with the inputs of EN 1998-3 Annex A in metres, newtons and MPa (bar diameter in metres).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct RcMember {
        pub h: f64,
        pub b: f64,
        pub d: f64,
        pub l_v: f64,
        pub n: f64,
        pub a_s1: f64,
        pub a_s2: f64,
        pub a_si: f64,
        pub d_b: f64,
        pub f_y: f64,
        pub f_c: f64,
        pub a_sx: f64,
        pub s_h: f64,
        pub f_yw: f64,
        pub b_0: f64,
        pub h_0: f64,
        pub restrained: f64,
        pub b_i: f64,
        pub rho_d: f64,
        pub shear_cracking: bool,
    }

    impl RcMember {
        /// 🏛️ The member described by an assessment, `None` while the section is not given (then θ_y and θ_um are the direct inputs).
        pub fn of(a: &super::En1998Assessment) -> Option<Self> {
            let given = [a.section_depth_m, a.section_width_m, a.effective_depth_m, a.shear_span_m, a.tension_steel_area_m2, a.steel_yield_mpa, a.concrete_strength_mpa];
            if given.iter().any(|v| *v <= 0.0) || a.effective_depth_m >= a.section_depth_m {
                return None;
            }
            Some(Self { h: a.section_depth_m, b: a.section_width_m, d: a.effective_depth_m, l_v: a.shear_span_m, n: a.axial_force_n, a_s1: a.tension_steel_area_m2, a_s2: a.compression_steel_area_m2, a_si: a.intermediate_steel_area_m2, d_b: a.bar_diameter_mm / 1000.0, f_y: a.steel_yield_mpa, f_c: a.concrete_strength_mpa, a_sx: a.stirrup_leg_area_m2, s_h: a.stirrup_spacing_m, f_yw: a.stirrup_yield_mpa, b_0: a.core_width_m, h_0: a.core_depth_m, restrained: a.restrained_bar_count, b_i: a.restrained_bar_spacing_m, rho_d: a.diagonal_reinforcement_ratio, shear_cracking: a.shear_cracking_before_yield >= 0.5 })
        }

        /// 🏛️ The member with the mean strengths divided by the confidence factor (EN 1998-3 3.5: mean values over CF give the capacities of ductile mechanisms).
        pub fn over_cf(self, cf: f64) -> Self {
            Self { f_y: self.f_y / cf, f_c: self.f_c / cf, f_yw: self.f_yw / cf, ..self }
        }
    }

    /// 🏛️ Secant modulus E_cm = 22 (f_cm/10)^0.3 GPa in MPa (EN 1992-1-1 Table 3.1) from the mean strength.
    pub fn concrete_modulus_mpa(f_cm_mpa: f64) -> f64 {
        22_000.0 * (f_cm_mpa / 10.0).powf(0.3)
    }

    /// 🏛️ Neutral axis depth ratio ξ_y = x/d of the cracked elastic section when the tension steel yields: ξ = √(α²A² + 2αB) − αA with A = ρ + ρ' + ν_d, B = ρ + ρ'δ' + ν_d, α = E_s/E_c, ν_d = N/(b d f_y) and δ' = d'/d (d' = h − d); steel in the web is not counted.
    pub fn yield_neutral_axis_ratio(m: &RcMember, steel_modulus_mpa: f64) -> f64 {
        let alpha = steel_modulus_mpa / concrete_modulus_mpa(m.f_c);
        let (rho, rho_c, delta) = (m.a_s1 / (m.b * m.d), m.a_s2 / (m.b * m.d), (m.h - m.d) / m.d);
        let nu = m.n / (m.b * m.d * m.f_y * 1.0e6);
        let (a, b) = (rho + rho_c + nu, rho + rho_c * delta + nu);
        ((alpha * alpha * a * a + 2.0 * alpha * b).max(0.0)).sqrt() - alpha * a
    }

    /// 🏛️ Yield curvature φ_y = (f_y/E_s) / ((1 − ξ_y) d) in 1/m.
    pub fn yield_curvature_per_m(m: &RcMember, steel_modulus_mpa: f64) -> f64 {
        let xi = yield_neutral_axis_ratio(m, steel_modulus_mpa);
        if !(0.0..1.0).contains(&xi) { return f64::NAN; }
        m.f_y / steel_modulus_mpa / ((1.0 - xi) * m.d)
    }

    /// 🏛️ Extreme fibre concrete stress at yield over f_c, E_c·ε_y·ξ/(1 − ξ)/f_c; above 1 the linear cracked section is invalid.
    pub fn concrete_stress_ratio_at_yield(m: &RcMember, steel_modulus_mpa: f64) -> f64 {
        let xi = yield_neutral_axis_ratio(m, steel_modulus_mpa);
        if !(0.0..1.0).contains(&xi) { return f64::NAN; }
        concrete_modulus_mpa(m.f_c) * (m.f_y / steel_modulus_mpa) * xi / (1.0 - xi) / m.f_c
    }

    /// 🧵 Moment-curvature points of the fibre section: yield (first of the tension steel at f_y/E_s or the extreme concrete at ε_c2) and ultimate (first of ε_cu at the extreme concrete or ε_su in the tension steel) at the axial force N; κ in 1/m, M in N·m.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct SectionResponse {
        pub kappa_y: f64,
        pub m_y: f64,
        pub kappa_u: f64,
        pub m_u: f64,
    }

    /// 🧶 Number of concrete strips of the fibre section.
    pub const SECTION_SLICES: usize = 120;

    impl RcMember {
        fn fibre_section(&self, k: &AnnexA) -> FibreSection<'_> {
            let (strength, peak, exponent, modulus, yield_stress) = (self.f_c, k.eps_c2_permille, k.parabola_exponent, k.steel_modulus_mpa, self.f_y);
            let bar = |area_m2: f64, z_m: f64| Reinforcement { fibre: Fibre::lumped(z_m * 1.0e3, area_m2 * 1.0e6), law: Box::new(move |e: f64| elastic_plastic(yield_stress, modulus, e)), displaces: true };
            let reinforcement = [(self.a_s1, 0.5 * self.h - self.d), (self.a_s2, self.d - 0.5 * self.h)].into_iter().filter(|(area, _)| *area > 0.0).map(|(area, z)| bar(area, z)).collect();
            FibreSection { concrete: rectangle_fibres(self.b * 1.0e3, self.h * 1.0e3, SECTION_SLICES), concrete_law: Box::new(move |e: f64| parabola_rectangle(strength, peak, exponent, e)), reinforcement, top: 0.5 * self.h * 1.0e3, cache: Default::default() }
        }

        fn limits(&self, k: &AnnexA) -> Limits {
            Limits { concrete_yield: k.eps_c2_permille, concrete_ultimate: k.eps_cu_permille, steel_yield: self.f_y / k.steel_modulus_mpa * 1.0e3, steel_ultimate: k.eps_su_permille }
        }

        /// 🎯 Yield and ultimate point of the rectangular section with the tension steel A_s1 at depth d and the compression steel A_s2 at h − d from the compressed face, under the axial force N; `None` above the strength range of the registry rows or when N is outside what the section balances. Remembered for the last members (inputs compared in place): the check, the stiffness, the neutral axis depth, the curve, the capacity scene and the bridge views of one pier all ask for it, and each ask integrates the fibre section about a hundred times.
        pub fn section_response(&self, k: &AnnexA) -> Option<SectionResponse> {
            use std::cell::RefCell;
            thread_local! {
                static MEMO: RefCell<Vec<(RcMember, AnnexA, Option<SectionResponse>)>> = const { RefCell::new(Vec::new()) };
            }
            if let Some(known) = MEMO.with(|memo| memo.borrow().iter().find(|(member, coefficients, _)| member == self && coefficients == k).map(|(_, _, response)| *response)) {
                return known;
            }
            let response = self.fibre_response(k);
            MEMO.with(|memo| {
                let mut memo = memo.borrow_mut();
                if memo.len() >= 32 {
                    memo.remove(0);
                }
                memo.push((*self, *k, response));
            });
            response
        }

        /// 🧵 The uncached fibre analysis behind [`Self::section_response`].
        fn fibre_response(&self, k: &AnnexA) -> Option<SectionResponse> {
            if self.f_c <= 0.0 || self.f_c > k.max_concrete_mpa || self.f_y <= 0.0 {
                return None;
            }
            let c = self.fibre_section(k).characteristics(self.n, &self.limits(k))?;
            Some(SectionResponse { kappa_y: c.kappa_y, m_y: c.m_y * 1.0e-3, kappa_u: c.kappa_u, m_u: c.m_u * 1.0e-3 })
        }

        /// 📏 Depth x [m] of the compression zone measured from the compressed face at the yield curvature of the fibre section: x = h/2 + ε_m/κ_y with the mean strain ε_m of the force balance at N; `None` where `section_response` is.
        pub fn yield_neutral_axis_depth_m(&self, k: &AnnexA) -> Option<f64> {
            let response = self.section_response(k)?;
            let eps_m = self.fibre_section(k).balance(response.kappa_y, self.n, 10.0 * k.eps_cu_permille)?;
            (response.kappa_y > 0.0).then(|| (0.5 * self.h * 1.0e3 + eps_m / response.kappa_y) * 1.0e-3)
        }

        /// 🧱 Rectangular pier section b × h with the longitudinal ratio `rho` = A_s/(b h) split evenly into the two outer layers at the cover `cover` (centre of bar) from each face, the axial force `n` (N, compression positive) and the strengths `f_c`, `f_y` (MPa); stirrup and confinement data stay zero.
        pub fn pier(b: f64, h: f64, cover: f64, rho: f64, n: f64, f_c: f64, f_y: f64) -> Self {
            let half = 0.5 * rho * b * h;
            Self { h, b, d: h - cover, l_v: 0.0, n, a_s1: half, a_s2: half, a_si: 0.0, d_b: 0.0, f_y, f_c, a_sx: 0.0, s_h: 0.0, f_yw: 0.0, b_0: 0.0, h_0: 0.0, restrained: 0.0, b_i: 0.0, rho_d: 0.0, shear_cracking: false }
        }

        /// 🧱 Effective flexural stiffness EI_eff = M_y/φ_y in N·m² of the fibre section at its axial force (EN 1998-2 2.3.6.1, Annex C); `None` without a section response.
        pub fn effective_stiffness_nm2(&self, k: &AnnexA) -> Option<f64> {
            let r = self.section_response(k)?;
            (r.kappa_y > 0.0 && r.m_y > 0.0).then(|| r.m_y / r.kappa_y)
        }

        /// 📉 Moment-curvature curve (κ in 1/m, M in N·m) of the section at its axial force up to the ultimate curvature, `points` curvatures dense towards zero; empty without a response.
        pub fn moment_curvature(&self, k: &AnnexA, points: usize) -> Vec<(f64, f64)> {
            let Some(response) = self.section_response(k) else { return Vec::new() };
            self.fibre_section(k).curve(self.n, response.kappa_u, points, 10.0 * k.eps_cu_permille).into_iter().map(|(kappa, m)| (kappa, m * 1.0e-3)).collect()
        }
    }

    /// 🏛️ Yield chord rotation θ_y = φ_y (L_V + a_V z)/3 + 0.0013 (1 + 1.5 h/L_V) + 0.13 φ_y d_b f_y/√f_c (EN 1998-3 A.10b), z = d − d', for the yield curvature `phi` in 1/m.
    pub fn chord_rotation_yield_of_curvature_rad(m: &RcMember, k: &AnnexA, phi: f64) -> f64 {
        let z = (m.d - (m.h - m.d)).max(0.0);
        let a_v = if m.shear_cracking { 1.0 } else { 0.0 };
        phi * (m.l_v + a_v * z) / 3.0 + k.y_base * (1.0 + k.y_slenderness * m.h / m.l_v) + k.y_bar_slip * phi * m.d_b * m.f_y / m.f_c.sqrt()
    }

    /// 🏛️ Yield chord rotation θ_y (EN 1998-3 A.10b) with φ_y from the moment-curvature of the fibre section, or from the cracked elastic section (`yield_curvature_per_m`) where the section response is not available.
    pub fn chord_rotation_yield_rad(m: &RcMember, k: &AnnexA) -> f64 {
        let phi = m.section_response(k).map_or_else(|| yield_curvature_per_m(m, k.steel_modulus_mpa), |r| r.kappa_y);
        chord_rotation_yield_of_curvature_rad(m, k, phi)
    }

    /// 🔧 Design chord rotation capacities of an assessment for the confidence factor `cf`: the direct inputs divided by CF and γ_el, or, while a direct input is 0 and the section is given, the Annex A expressions of the member with the strengths over CF; `member` is the section the derived values come from.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct DesignCapacities {
        pub yield_rad: f64,
        pub ultimate_rad: f64,
        pub member: Option<RcMember>,
    }

    pub fn design_capacities(a: &super::En1998Assessment, k: &AnnexA, cf: f64) -> DesignCapacities {
        let member = RcMember::of(a).map(|m| m.over_cf(cf));
        let yield_rad = if a.chord_rotation_yield_rad > 0.0 { design_capacity_n(a.chord_rotation_yield_rad, cf, a.gamma_el) } else { member.map_or(0.0, |m| chord_rotation_yield_rad(&m, k)) };
        let ultimate_rad = if a.chord_rotation_ultimate_rad > 0.0 { design_capacity_n(a.chord_rotation_ultimate_rad, cf, a.gamma_el) } else { member.map_or(0.0, |m| chord_rotation_ultimate_rad(&m, a.gamma_el, k)) };
        DesignCapacities { yield_rad, ultimate_rad, member }
    }

    /// 🏛️ Confinement effectiveness α = (1 − s_h/2b_0)(1 − s_h/2h_0)(1 − Σb_i²/6b_0h_0) (EN 1998-3 A.3.2.2) with Σb_i² = n·b_i².
    pub fn confinement_effectiveness(m: &RcMember) -> f64 {
        if m.b_0 <= 0.0 || m.h_0 <= 0.0 { return 0.0; }
        ((1.0 - m.s_h / (2.0 * m.b_0)) * (1.0 - m.s_h / (2.0 * m.h_0)) * (1.0 - m.restrained * m.b_i * m.b_i / (6.0 * m.b_0 * m.h_0))).max(0.0)
    }

    /// 🏛️ Transverse reinforcement ratio ρ_sx = A_sx/(b_w s_h) in the loading direction.
    pub fn transverse_ratio(m: &RcMember) -> f64 {
        if m.s_h <= 0.0 { 0.0 } else { m.a_sx / (m.b * m.s_h) }
    }

    /// 🏛️ Ultimate chord rotation θ_um = (1/γ_el) 0.016 (0.3^ν) [max(0.01, ω')/max(0.01, ω) f_c]^0.225 (L_V/h)^0.35 25^(α ρ_sx f_yw/f_c) 1.25^(100 ρ_d) (EN 1998-3 A.1) with ν = N/(b h f_c), ω = ρ f_y/f_c and ω' = ρ' f_y/f_c.
    pub fn chord_rotation_ultimate_rad(m: &RcMember, gamma_el: f64, k: &AnnexA) -> f64 {
        if gamma_el <= 0.0 { return 0.0; }
        let nu = (m.n / (m.b * m.h * m.f_c * 1.0e6)).max(0.0);
        let omega = m.a_s1 / (m.b * m.d) * m.f_y / m.f_c;
        let omega_c = m.a_s2 / (m.b * m.d) * m.f_y / m.f_c;
        let confinement = confinement_effectiveness(m) * transverse_ratio(m) * m.f_yw / m.f_c;
        k.u_base / gamma_el * k.u_axial.powf(nu) * (omega_c.max(0.01) / omega.max(0.01) * m.f_c).powf(k.u_reinforcement) * (m.l_v / m.h).powf(k.u_shear_span) * k.u_confinement.powf(confinement) * k.u_diagonal.powf(100.0 * m.rho_d)
    }
}

pub mod part_4 {
    /// 🫙 Housner impulsive mass ratio m_i/m = tanh(1.7·R/H)/(1.7·R/H) (EN 1998-4 Annex A.3).
    pub fn impulsive_mass_ratio(h_over_r: f64) -> f64 {
        let x = 1.7 / h_over_r.max(1e-9);
        x.tanh() / x
    }
    /// 🫙 Housner convective mass ratio m_c/m = 0.460·(R/H)·tanh(1.84·H/R) = 0.230·(D/H)·tanh(3.68·H/D) (EN 1998-4 Annex A.3, coefficient unverified; the first sloshing mode of the rigid tank gives 2/(ε(ε²−1)) = 0.4548).
    pub fn convective_mass_ratio(h_over_r: f64) -> f64 {
        0.460 / h_over_r.max(1e-9) * (1.84 * h_over_r).tanh()
    }
    /// 🌊 Sloshing period T_c = 2π/ω_c with ω_c² = (1.84·g/R)·tanh(1.84·H/R) (EN 1998-4 Annex A.3).
    pub fn convective_period_s(height_m: f64, radius_m: f64) -> f64 {
        let r = radius_m.max(1e-9);
        let omega2 = 1.84 * 9.81 / r * (1.84 * height_m / r).tanh();
        2.0 * std::f64::consts::PI / omega2.max(1e-12).sqrt()
    }
    /// 🌊 Damping correction of the convective mode, ξ_c = 0.5 % (EN 1998-4 Annex A.3).
    pub fn convective_damping_eta() -> f64 { (10.0_f64 / 5.5).sqrt() }
    pub fn silo_behaviour_factor(q_nominal: f64) -> f64 { q_nominal.min(1.5).max(1.0) }
    pub fn srss(a: f64, b: f64) -> f64 { (a * a + b * b).sqrt() }
    /// ⬇️ Axial demand of a silo shell N = m·g·(1 + a_vg/g) with a_vg = 0.9·a_g (EN 1998-1 §3.2.2.3 type 1 ratio, unverified NDP).
    pub fn silo_axial_demand_n(mass_kg: f64, a_g: f64) -> f64 {
        mass_kg * (9.81 + semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.vertical-spectrum.avg-ratio") * a_g)
    }
}

pub mod part_5 {
    pub use semio_s_plugin_norm_mechanics::geotechnics::slope::{Circle, Pseudostatic};

    /// 🌊️ Depth reduction r_d of the simplified liquefaction procedure (Youd et al. 2001; the EN 1998-5 Annex B chart is the unverified basis `ref.liquefaction.youd-2001`).
    pub fn liquefaction_rd(z_m: f64) -> f64 {
        if z_m <= 9.15 { 1.0 - 0.00765 * z_m } else if z_m <= 23.0 { 1.174 - 0.0267 * z_m } else if z_m <= 30.0 { 0.744 - 0.008 * z_m } else { 0.5 }
    }
    /// 🌊️ Cyclic stress ratio CSR = 0.65 (a_max/g) (σ_v/σ'_v) r_d.
    pub fn liquefaction_csr(a_max_ms2: f64, sigma_v: f64, sigma_v_eff: f64, z_m: f64) -> f64 {
        if sigma_v_eff <= 0.0 { return f64::INFINITY; }
        0.65 * (a_max_ms2 / 9.81) * (sigma_v / sigma_v_eff) * liquefaction_rd(z_m)
    }
    /// 🌊️ Magnitude scaling factor MSF = 10^2.24 / M^2.56.
    pub fn liquefaction_msf(magnitude: f64) -> f64 { 10f64.powf(2.24) / magnitude.max(1e-9).powf(2.56) }
    /// 🌊️ Clean-sand equivalent blow count (N1)60cs = α + β (N1)60 from the fines content FC [%].
    pub fn liquefaction_n1_60cs(n1_60: f64, fines_percent: f64) -> f64 {
        let (alpha, beta) = if fines_percent <= 5.0 { (0.0, 1.0) } else if fines_percent < 35.0 { ((1.76 - 190.0 / (fines_percent * fines_percent)).exp(), 0.99 + fines_percent.powf(1.5) / 1000.0) } else { (5.0, 1.2) };
        alpha + beta * n1_60
    }
    /// 🌊️ Cyclic resistance ratio of a magnitude 7.5 event from (N1)60cs; infinity from 30 blows on (not liquefiable).
    pub fn liquefaction_crr75(n_cs: f64) -> f64 {
        if n_cs >= 30.0 { f64::INFINITY } else { 1.0 / (34.0 - n_cs) + n_cs / 135.0 + 50.0 / (10.0 * n_cs + 45.0).powi(2) - 1.0 / 200.0 }
    }
    /// 🌊️ Factor of safety against liquefaction F_L = CRR_7.5 · MSF / CSR.
    pub fn liquefaction_safety_factor(n1_60: f64, fines_percent: f64, a_max_ms2: f64, sigma_v: f64, sigma_v_eff: f64, z_m: f64, magnitude: f64) -> f64 {
        let csr = liquefaction_csr(a_max_ms2, sigma_v, sigma_v_eff, z_m);
        if csr <= 0.0 { return f64::INFINITY; }
        liquefaction_crr75(liquefaction_n1_60cs(n1_60, fines_percent)) * liquefaction_msf(magnitude) / csr
    }
    /// 🏗️ Partial resistance factor γ_Rd of the seismic bearing check (EN 1998-5 Annex F, sand: 1.0).
    pub fn bearing_gamma_rd() -> f64 { 1.0 }
    /// 🏗️ Bearing pressure p = N/(B·(B − 2e)) of a square footing of area A under N and M with e = M/N; infinity when the resultant leaves the core (EN 1998-5 Annex F, simplified).
    pub fn eccentric_bearing_pressure_pa(n_ed_n: f64, m_ed_nm: f64, area_m2: f64) -> f64 {
        if area_m2 <= 0.0 || n_ed_n <= 0.0 { return if n_ed_n > 0.0 { f64::INFINITY } else { 0.0 }; }
        let b = area_m2.sqrt();
        let e = m_ed_nm.abs() / n_ed_n;
        let effective = b * (b - 2.0 * e);
        if effective <= 0.0 { f64::INFINITY } else { n_ed_n / effective }
    }
    pub fn horizontal_seismic_coefficient(a_g_m_s2: f64, s: f64, r: f64) -> f64 {
        (a_g_m_s2 / 9.81) * s / r.max(1.0)
    }
    /// 🌋 Share F_V/F_H of the vertical pseudo-static force (EN 1998-5 §4.1.3.3, §7.3.2.2): the `high` share where the vertical to horizontal ground acceleration ratio exceeds the `threshold`, else the `low` share.
    pub fn vertical_force_ratio(avg_over_ag: f64, threshold: f64, high: f64, low: f64) -> f64 {
        if avg_over_ag > threshold { high } else { low }
    }
    /// 🧱 Friction part of the seismic sliding resistance (V − u) tan δ / γ_Rd [N] (EN 1998-5 §5.3); a negative effective vertical force carries no friction.
    pub fn sliding_friction_resistance_n(v_n: f64, u_n: f64, delta_deg: f64, gamma_rd: f64) -> f64 {
        (v_n - u_n).max(0.0) * delta_deg.to_radians().tan() / gamma_rd
    }
    /// 🧱 Interface friction angle δ [°] that carries the sliding demand `h_ed` together with the passive resistance; `None` without an effective vertical force.
    pub fn required_friction_angle_deg(h_ed_n: f64, passive_n: f64, v_n: f64, u_n: f64, gamma_rd: f64) -> Option<f64> {
        let effective = v_n - u_n;
        (effective > 0.0).then(|| ((h_ed_n - passive_n).max(0.0) * gamma_rd / effective).atan().to_degrees())
    }
    pub fn stiffness_ratio(k_f: f64, k_s: f64) -> f64 {
        if k_s <= 0.0 { return 0.0; }
        k_f / k_s
    }
    pub fn radiation_damping(ratio: f64) -> f64 { (ratio / (1.0 + ratio)).clamp(0.0, 1.0) }
    /// 🌊️ Annex B cyclic stress ratio τ_e/σ'_v0 = c (a_g S/g)(σ_v0/σ'_v0) with the registry factor c = 0.65 and no depth reduction r_d (unverified, see `ndp.en1998.en5.liquefaction.annex-b.*`).
    pub fn liquefaction_csr_annex_b(csr_factor: f64, a_g_s_ms2: f64, sigma_v: f64, sigma_v_eff: f64) -> f64 {
        if sigma_v_eff <= 0.0 { return f64::INFINITY; }
        csr_factor * (a_g_s_ms2 / 9.81) * (sigma_v / sigma_v_eff)
    }

    /// 🌊️ One liquefiable layer with the ground water and the design acceleration a_g S, evaluated at any depth of the same uniform profile.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct LiquefactionLayer {
        pub n1_60: f64,
        pub fines_percent: f64,
        pub unit_weight_n_m3: f64,
        pub water_depth_m: f64,
        pub magnitude: f64,
        pub a_g_s_ms2: f64,
        pub csr_factor: f64,
    }

    impl LiquefactionLayer {
        /// 🌊️ Total and effective vertical stress at depth z for a uniform unit weight and the hydrostatic pore pressure below the water table.
        pub fn stresses(&self, z_m: f64) -> (f64, f64) {
            let total = self.unit_weight_n_m3 * z_m;
            (total, total - 9810.0 * (z_m - self.water_depth_m).max(0.0))
        }
        pub fn crr(&self) -> f64 {
            liquefaction_crr75(liquefaction_n1_60cs(self.n1_60, self.fines_percent)) * liquefaction_msf(self.magnitude)
        }
        /// 🌊️ F_L = CRR·MSF/CSR with the Annex B stress ratio at depth z (no r_d).
        pub fn factor_annex_b(&self, z_m: f64) -> f64 {
            let (total, effective) = self.stresses(z_m);
            let csr = liquefaction_csr_annex_b(self.csr_factor, self.a_g_s_ms2, total, effective);
            if csr <= 0.0 { f64::INFINITY } else { self.crr() / csr }
        }
        /// 🌊️ F_L with the Youd et al. (2001) stress ratio at depth z (r_d applied).
        pub fn factor_youd(&self, z_m: f64) -> f64 {
            let (total, effective) = self.stresses(z_m);
            let csr = liquefaction_csr(self.a_g_s_ms2, total, effective, z_m);
            if csr <= 0.0 { f64::INFINITY } else { self.crr() / csr }
        }
    }

    /// 🪨️ One layer of a soil profile, listed top to bottom; a blow count (N1)60 of 0 marks a layer that is not assessed for liquefaction (clay, rock).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ProfileLayer {
        pub thickness_m: f64,
        pub unit_weight_n_m3: f64,
        pub shear_wave_velocity_m_s: f64,
        pub n1_60: f64,
        pub fines_percent: f64,
    }

    /// 🪨️ A layered soil profile with the ground water, the design magnitude and the design acceleration a_g S: stresses integrate the unit weights of the layers, the Annex B factor of safety uses the resistance of the layer at the depth, the site response uses the layer velocities.
    #[derive(Clone, Debug, PartialEq)]
    pub struct SoilProfile {
        pub layers: Vec<ProfileLayer>,
        pub water_depth_m: f64,
        pub magnitude: f64,
        pub a_g_s_ms2: f64,
        pub csr_factor: f64,
    }

    impl SoilProfile {
        /// 🪨️ Depth of the bottom of every layer, i.e. the interface depths with the base of the profile last.
        pub fn bottoms(&self) -> Vec<f64> {
            self.layers.iter().scan(0.0, |depth, layer| { *depth += layer.thickness_m.max(0.0); Some(*depth) }).collect()
        }
        /// 🪨️ Depth of the base of the profile.
        pub fn depth_m(&self) -> f64 {
            self.layers.iter().map(|l| l.thickness_m.max(0.0)).sum()
        }
        /// 🪨️ Index of the layer containing depth z (an interface belongs to the layer above, depths below the base to the last layer).
        pub fn layer_at(&self, z_m: f64) -> usize {
            self.bottoms().iter().position(|bottom| z_m <= *bottom + 1e-12).unwrap_or(self.layers.len().saturating_sub(1))
        }
        /// 🪨️ Total and effective vertical stress at depth z: the layer unit weights integrated from the surface (the last layer continues below the base) and the hydrostatic pore pressure below the water table.
        pub fn stresses(&self, z_m: f64) -> (f64, f64) {
            let mut top = 0.0;
            let mut total = 0.0;
            for layer in &self.layers {
                total += layer.unit_weight_n_m3 * layer.thickness_m.max(0.0).min((z_m - top).max(0.0));
                top += layer.thickness_m.max(0.0);
            }
            if let Some(last) = self.layers.last() {
                total += last.unit_weight_n_m3 * (z_m - top).max(0.0);
            }
            (total, total - 9810.0 * (z_m - self.water_depth_m).max(0.0))
        }
        /// 🌊️ Cyclic resistance CRR·MSF of layer i; infinity for a layer that is not assessed.
        pub fn resistance(&self, index: usize) -> f64 {
            match self.layers.get(index) {
                Some(layer) if layer.n1_60 > 0.0 => liquefaction_crr75(liquefaction_n1_60cs(layer.n1_60, layer.fines_percent)) * liquefaction_msf(self.magnitude),
                _ => f64::INFINITY,
            }
        }
        /// 🌊️ Annex B factor of safety F_L at depth z of the profile.
        pub fn factor(&self, z_m: f64) -> f64 {
            let (total, effective) = self.stresses(z_m);
            let csr = liquefaction_csr_annex_b(self.csr_factor, self.a_g_s_ms2, total, effective);
            if csr <= 0.0 { f64::INFINITY } else { self.resistance(self.layer_at(z_m)) / csr }
        }
        /// 🌊️ Depth and value of the smallest F_L inside layer i (nine samples from just below the top, at least 0.1 m, to the bottom).
        pub fn layer_governing(&self, index: usize) -> (f64, f64) {
            let bottoms = self.bottoms();
            let Some(bottom) = bottoms.get(index).copied() else { return (0.0, f64::INFINITY) };
            let top = if index == 0 { 0.0 } else { bottoms[index - 1] };
            let from = (top + 1e-6).max(0.1).min(bottom);
            (0..=8).map(|i| from + (bottom - from) * f64::from(i) / 8.0).map(|z| (z, self.factor(z))).fold((from, f64::INFINITY), |best, point| if point.1 < best.1 { point } else { best })
        }
        /// 🌊️ Layer, depth and value of the smallest F_L of the profile.
        pub fn governing(&self) -> Option<(usize, f64, f64)> {
            (0..self.layers.len()).map(|i| { let (z, f) = self.layer_governing(i); (i, z, f) }).fold(None, |best: Option<(usize, f64, f64)>, point| match best { Some(b) if b.2 <= point.2 => Some(b), _ => Some(point) })
        }
        /// 🌍️ Time-averaged shear wave velocity v_s,H = H/Σ(h_i/v_i) over the depth H (EN 1998-1 3.1.2 expression 3.1 with H = 30 m); a profile shorter than H continues with its last layer. Zero while a velocity is missing.
        pub fn equivalent_velocity_m_s(&self, depth_m: f64) -> f64 {
            let mut top = 0.0;
            let mut travel = 0.0;
            for layer in &self.layers {
                let h = layer.thickness_m.max(0.0).min((depth_m - top).max(0.0));
                if h > 0.0 {
                    if layer.shear_wave_velocity_m_s <= 0.0 { return 0.0; }
                    travel += h / layer.shear_wave_velocity_m_s;
                }
                top += layer.thickness_m.max(0.0);
            }
            if top < depth_m {
                let Some(last) = self.layers.last() else { return 0.0 };
                if last.shear_wave_velocity_m_s <= 0.0 { return 0.0; }
                travel += (depth_m - top) / last.shear_wave_velocity_m_s;
            }
            if travel > 0.0 { depth_m / travel } else { 0.0 }
        }
        /// 🌍️ Fundamental period of the profile over rigid base T_0 = 4 Σ h_i/v_i (quarter wave of the vertically travelling shear wave).
        pub fn site_period_s(&self) -> f64 {
            if self.layers.iter().any(|l| l.thickness_m > 0.0 && l.shear_wave_velocity_m_s <= 0.0) { return 0.0; }
            4.0 * self.layers.iter().map(|l| l.thickness_m.max(0.0) / l.shear_wave_velocity_m_s.max(1e-12)).sum::<f64>()
        }
    }

    /// 🏗️ Constants of the generalised failure surface of EN 1998-5 Annex F for cohesionless soil, read from the registry (unverified).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct AnnexF {
        pub a: f64,
        pub b: f64,
        pub c: f64,
        pub d: f64,
        pub e: f64,
        pub f: f64,
        pub m: f64,
        pub k: f64,
        pub k_prime: f64,
        pub c_t: f64,
        pub c_t_prime: f64,
        pub c_m: f64,
        pub c_m_prime: f64,
        pub beta: f64,
        pub gamma: f64,
        pub gamma_rd: f64,
    }

    impl AnnexF {
        pub fn from_registry(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
            let n = |key: &str| semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1998.en5.annex-f.sand.{key}"), annex);
            Self { a: n("a"), b: n("b"), c: n("c"), d: n("d"), e: n("e"), f: n("f"), m: n("m"), k: n("k"), k_prime: n("k-prime"), c_t: n("c-t"), c_t_prime: n("c-t-prime"), c_m: n("c-m"), c_m_prime: n("c-m-prime"), beta: n("beta"), gamma: n("gamma"), gamma_rd: semio_s_plugin_norm_compliance::ndp_number("ndp.en1998.en5.annex-f.gamma-rd", annex) }
        }

        /// 🏗️ Dimensionless soil inertia F̄ = a_g S/(g tan φ') of the cohesionless soil.
        pub fn soil_inertia(a_g_s_ms2: f64, phi_deg: f64) -> f64 {
            let tan = phi_deg.to_radians().tan();
            if tan <= 0.0 { f64::INFINITY } else { a_g_s_ms2 / 9.81 / tan }
        }

        /// 🏗️ Normalised loads (N̄, V̄, M̄) = γ_Rd (N, V, M/B)/N_max; `None` without footing width or N_max.
        pub fn normalise(&self, n_ed: f64, v_ed: f64, m_ed: f64, width_m: f64, n_max: f64) -> Option<(f64, f64, f64)> {
            if width_m <= 0.0 || n_max <= 0.0 { return None; }
            Some((self.gamma_rd * n_ed / n_max, self.gamma_rd * v_ed.abs() / n_max, self.gamma_rd * m_ed.abs() / (width_m * n_max)))
        }

        /// 🏗️ Upper bound (1 − m F̄^k)^k' of N̄ at the given soil inertia.
        pub fn vertical_bound(&self, f_bar: f64) -> f64 {
            (1.0 - self.m * f_bar.powf(self.k)).max(0.0).powf(self.k_prime)
        }

        /// 🏗️ Failure surface value F(N̄, V̄, M̄, F̄) of Annex F; negative inside the surface, zero on it, `+∞` outside the admissible N̄ range.
        pub fn surface(&self, n: f64, v: f64, m: f64, f_bar: f64) -> f64 {
            let room = self.vertical_bound(f_bar) - n;
            if n <= 0.0 || room <= 0.0 { return f64::INFINITY; }
            let horizontal = (1.0 - self.e * f_bar).max(0.0).powf(self.c_t) * (self.beta * v).powf(self.c_t_prime) / (n.powf(self.a) * room.powf(self.b));
            let moment = (1.0 - self.f * f_bar).max(0.0).powf(self.c_m) * (self.gamma * m).powf(self.c_m_prime) / (n.powf(self.c) * room.powf(self.d));
            horizontal + moment - 1.0
        }

        /// 🏗️ Largest V̄ on the surface at M̄ = 0 for a given N̄ (closed form of the first term).
        pub fn horizontal_limit(&self, n: f64, f_bar: f64) -> f64 {
            let room = self.vertical_bound(f_bar) - n;
            if n <= 0.0 || room <= 0.0 { return 0.0; }
            (n.powf(self.a) * room.powf(self.b) / (1.0 - self.e * f_bar).max(1e-12).powf(self.c_t)).powf(1.0 / self.c_t_prime) / self.beta
        }

        /// 🏗️ Largest M̄ on the surface at V̄ = 0 for a given N̄ (closed form of the second term).
        pub fn moment_limit(&self, n: f64, f_bar: f64) -> f64 {
            let room = self.vertical_bound(f_bar) - n;
            if n <= 0.0 || room <= 0.0 { return 0.0; }
            (n.powf(self.c) * room.powf(self.d) / (1.0 - self.f * f_bar).max(1e-12).powf(self.c_m)).powf(1.0 / self.c_m_prime) / self.gamma
        }

        /// 🏗️ Utilisation 1/λ of the proportional load path: λ is the factor on (N̄, V̄, M̄) at which the surface is reached; infinity when no vertical load acts with a horizontal one.
        pub fn utilisation(&self, n: f64, v: f64, m: f64, f_bar: f64) -> f64 {
            if n <= 0.0 { return if v <= 0.0 && m <= 0.0 { 0.0 } else { f64::INFINITY }; }
            let upper = self.vertical_bound(f_bar) / n * (1.0 - 1e-12);
            if upper <= 0.0 { return f64::INFINITY; }
            let g = |lambda: f64| self.surface(lambda * n, lambda * v, lambda * m, f_bar);
            if g(upper) <= 0.0 { return 1.0 / upper; }
            let (mut low, mut high) = (0.0, upper);
            for _ in 0..80 {
                let mid = 0.5 * (low + high);
                if g(mid) <= 0.0 { low = mid } else { high = mid }
            }
            1.0 / low.max(1e-12)
        }
    }

    /// 🏗️ EI = E_p π d⁴/64 of a circular pile.
    pub fn pile_bending_stiffness(e_p_pa: f64, d_m: f64) -> f64 {
        e_p_pa * std::f64::consts::PI * d_m.powi(4) / 64.0
    }

    /// 🏗️ Winkler characteristic number β = (k_h d/(4 E I))^¼ in 1/m.
    pub fn pile_beta_per_m(e_p_pa: f64, d_m: f64, k_h_n_m3: f64) -> f64 {
        let ei = pile_bending_stiffness(e_p_pa, d_m);
        if ei <= 0.0 { return f64::NAN; }
        (k_h_n_m3 * d_m / (4.0 * ei)).powf(0.25)
    }

    /// 🏗️ Inertial response of a long pile to the head shear V: head fixed against rotation (M(z) = −V/(2β) e^{−βz}(cos βz − sin βz), y₀ = Vβ/k) or free (M(z) = V/β e^{−βz} sin βz, y₀ = 2Vβ/k) with k = k_h d.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PileInertial {
        pub beta: f64,
        pub max_moment_nm: f64,
        pub head_moment_nm: f64,
        pub head_deflection_m: f64,
        pub long: bool,
    }

    pub fn pile_inertial(e_p_pa: f64, d_m: f64, length_m: f64, k_h_n_m3: f64, head_shear_n: f64, fixed_head: bool) -> PileInertial {
        let beta = pile_beta_per_m(e_p_pa, d_m, k_h_n_m3);
        let k = k_h_n_m3 * d_m;
        let free_max = (-std::f64::consts::FRAC_PI_4).exp() * std::f64::consts::FRAC_PI_4.sin();
        let (max_moment_nm, head_moment_nm, head_deflection_m) = if fixed_head {
            (head_shear_n / (2.0 * beta), head_shear_n / (2.0 * beta), head_shear_n * beta / k)
        } else {
            (free_max * head_shear_n / beta, 0.0, 2.0 * head_shear_n * beta / k)
        };
        PileInertial { beta, max_moment_nm, head_moment_nm, head_deflection_m, long: beta * length_m >= std::f64::consts::PI }
    }

    /// 🏗️ Bending moment of the inertial response at depth z below the pile head.
    pub fn pile_inertial_moment_at(i: &PileInertial, head_shear_n: f64, fixed_head: bool, z_m: f64) -> f64 {
        let x = i.beta * z_m;
        if fixed_head { -head_shear_n / (2.0 * i.beta) * (-x).exp() * (x.cos() - x.sin()) } else { head_shear_n / i.beta * (-x).exp() * x.sin() }
    }

    /// 🏗️ Coefficients of the kinematic pile moment at a layer interface of Nikolaou et al. (2001), read from the registry (unverified reference basis, not part of EN 1998-5).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Nikolaou {
        pub coefficient: f64,
        pub length_exponent: f64,
        pub modulus_exponent: f64,
        pub velocity_exponent: f64,
    }

    impl Nikolaou {
        pub fn from_registry(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
            let n = |key: &str| semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1998.ref.pile.nikolaou-2001.{key}"), annex);
            Self { coefficient: n("coefficient"), length_exponent: n("length-exponent"), modulus_exponent: n("modulus-exponent"), velocity_exponent: n("velocity-exponent") }
        }
    }

    /// 🏗️ Soil Young's modulus E = 3 ρ v_s² (ν = 0.5) from the unit weight.
    pub fn soil_young_modulus_pa(unit_weight_n_m3: f64, vs_m_s: f64) -> f64 {
        3.0 * unit_weight_n_m3 / 9.81 * vs_m_s * vs_m_s
    }

    /// 🏗️ Shear stress τ_c = (a_g S/g) σ_v at the layer interface at depth z.
    pub fn interface_shear_stress_pa(a_g_s_ms2: f64, unit_weight_n_m3: f64, interface_depth_m: f64) -> f64 {
        a_g_s_ms2 / 9.81 * unit_weight_n_m3 * interface_depth_m
    }

    /// 🏗️ Shear modulus ratio G₂/G₁ = (v_s2/v_s1)² of the two layers at equal density.
    pub fn stiffness_contrast(vs_upper_m_s: f64, vs_lower_m_s: f64) -> f64 {
        if vs_upper_m_s <= 0.0 { f64::INFINITY } else { (vs_lower_m_s / vs_upper_m_s).powi(2) }
    }

    /// 🏗️ Kinematic pile moment M = c τ_c d³ (L/d)^p (E_p/E_1)^q (v_s2/v_s1)^r at the interface.
    pub fn pile_kinematic_moment_nm(k: &Nikolaou, tau_c_pa: f64, d_m: f64, length_m: f64, e_p_pa: f64, e_upper_pa: f64, vs_upper_m_s: f64, vs_lower_m_s: f64) -> f64 {
        if d_m <= 0.0 || e_upper_pa <= 0.0 || vs_upper_m_s <= 0.0 { return f64::NAN; }
        k.coefficient * tau_c_pa * d_m.powi(3) * (length_m / d_m).powf(k.length_exponent) * (e_p_pa / e_upper_pa).powf(k.modulus_exponent) * (vs_lower_m_s / vs_upper_m_s).powf(k.velocity_exponent)
    }

    /// 🔗 Tie force N = ±coefficient · α · S · N_Ed [N] of a foundation tie beam (EN 1998-5 5.4.1.2), with α = a_g/g.
    pub fn tie_force_n(coefficient: f64, a_g_m_s2: f64, s: f64, n_ed_n: f64) -> f64 {
        coefficient * (a_g_m_s2 / 9.81) * s * n_ed_n.abs()
    }

    /// 🔗 Tension resistance N_Rd = A_s f_yd [N] of the tie steel.
    pub fn tie_resistance_n(steel_area_m2: f64, design_strength_pa: f64) -> f64 {
        steel_area_m2 * design_strength_pa
    }

    /// ⛰️ Slices of the pseudo-static slope stability (the kernel default of the circle search).
    pub const SLOPE_SLICES: usize = 40;

    /// ⛰️ Pseudo-static design forces of the slope (EN 1998-5 4.1.3.3): `F_H = factor · α · S · S_T · W` with α = a_g/g and `F_V = ±share · F_H`.
    pub fn slope_pseudostatic(horizontal_factor: f64, a_g_m_s2: f64, s: f64, topographic: f64, vertical_share: f64) -> semio_s_plugin_norm_mechanics::geotechnics::slope::Pseudostatic {
        semio_s_plugin_norm_mechanics::geotechnics::slope::Pseudostatic { horizontal: horizontal_factor * (a_g_m_s2 / 9.81) * s * topographic, vertical: vertical_share }
    }

    /// ⛰️ Dry homogeneous slope with a circular slip surface.
    pub fn slope_circle(height_m: f64, angle_deg: f64, unit_weight_n_m3: f64, centre_x: f64, centre_z: f64, radius_m: f64) -> semio_s_plugin_norm_mechanics::geotechnics::slope::Circle {
        semio_s_plugin_norm_mechanics::geotechnics::slope::Circle { h: height_m, beta_deg: angle_deg, xc: centre_x, yc: centre_z, r: radius_m, gamma: unit_weight_n_m3, gamma_sat: unit_weight_n_m3, gwl_level: f64::NEG_INFINITY, ru: 0.0, q: 0.0 }
    }

    /// ⛰️ Simplified Bishop factor of safety of the circle with the strength divided by the partial factors (M2): `tan φ_d = tan φ′ / γ_φ`, `c_d = c′ / γ_c`.
    pub fn slope_factor(circle: &semio_s_plugin_norm_mechanics::geotechnics::slope::Circle, friction_angle_deg: f64, cohesion_pa: f64, gamma_phi: f64, gamma_c: f64, p: &semio_s_plugin_norm_mechanics::geotechnics::slope::Pseudostatic) -> Option<f64> {
        semio_s_plugin_norm_mechanics::geotechnics::slope::bishop_with(circle, friction_angle_deg.to_radians().tan() / gamma_phi, cohesion_pa / gamma_c, SLOPE_SLICES, p)
    }

    /// ⛰️ Critical circle: the least factor over a grid of centres above the slope (x from −h/2 to the toe plus h/2, z from 0.8 h to 2.4 h) and radii of 0.85 to 1.3 times the toe distance, `None` when no circle develops a sliding mass.
    pub fn slope_critical_circle(height_m: f64, angle_deg: f64, unit_weight_n_m3: f64, friction_angle_deg: f64, cohesion_pa: f64, gamma_phi: f64, gamma_c: f64, p: &semio_s_plugin_norm_mechanics::geotechnics::slope::Pseudostatic) -> Option<(semio_s_plugin_norm_mechanics::geotechnics::slope::Circle, f64)> {
        let run = height_m / angle_deg.to_radians().tan();
        let mut best: Option<(semio_s_plugin_norm_mechanics::geotechnics::slope::Circle, f64)> = None;
        for i in 0..=8 {
            let x = -0.5 * height_m + (run + height_m) * f64::from(i) / 8.0;
            for j in 0..=6 {
                let z = height_m * (0.8 + 1.6 * f64::from(j) / 6.0);
                let toe = ((x - run).powi(2) + z * z).sqrt();
                for scale in [0.85, 1.0, 1.15, 1.3] {
                    let circle = slope_circle(height_m, angle_deg, unit_weight_n_m3, x, z, toe * scale);
                    if let Some(f) = slope_factor(&circle, friction_angle_deg, cohesion_pa, gamma_phi, gamma_c, p).filter(|f| f.is_finite() && *f > 0.0) {
                        if best.as_ref().is_none_or(|(_, least)| f < *least) {
                            best = Some((circle, f));
                        }
                    }
                }
            }
        }
        best
    }

    /// ⛰️ Design forces and strength factors of the seismic slope check: the pseudo-static forces, the M2 factors and the unit weight of the governing soil layer.
    pub struct SlopeForces {
        pub pseudostatic: semio_s_plugin_norm_mechanics::geotechnics::slope::Pseudostatic,
        pub gamma_phi: f64,
        pub gamma_c: f64,
        pub unit_weight_n_m3: f64,
        pub topographic: f64,
        pub vertical_share: f64,
    }

    /// ⛰️ The given and the critical slip circle with their factors of safety.
    pub struct SlopeResult {
        pub fixed: Option<(semio_s_plugin_norm_mechanics::geotechnics::slope::Circle, f64)>,
        pub critical: Option<(semio_s_plugin_norm_mechanics::geotechnics::slope::Circle, f64)>,
    }

    impl SlopeResult {
        /// ⛰️ The governing factor of safety, the least of the given and the critical circle; `None` when no circle develops a sliding mass.
        pub fn least(&self) -> Option<f64> {
            [self.fixed.as_ref().map(|c| c.1), self.critical.as_ref().map(|c| c.1)].into_iter().flatten().reduce(f64::min)
        }
    }

    /// ⛰️ Forces of the slope `slope` (EN 1998-5 4.1.3.3) for the design acceleration `a_g_m_s2` (γ_I a_g) and the soil factor `s`; `None` when the governing layer is not a soil layer of the document.
    pub fn slope_forces(document: &semio_s_artifact_norm_en1998_model::En1998Snapshot, slope: &semio_s_artifact_norm_en1998_model::En1998Slope, annex: semio_s_plugin_norm_compliance::AnnexChoice, a_g_m_s2: f64, s: f64) -> Option<SlopeForces> {
        use semio_s_plugin_norm_compliance::{ndp_common, ndp_number};
        let layer = document.soil_layers.iter().find(|l| l.id == slope.governing_layer_id)?;
        let avg = ndp_common(if document.site.en_spectrum_type == semio_s_artifact_norm_en1998_model::EnSpectrumType::Type2 { "ndp.en1998.vertical-spectrum.avg-ratio-type2" } else { "ndp.en1998.vertical-spectrum.avg-ratio" });
        let vertical_share = vertical_force_ratio(avg, ndp_number("ndp.en1998.en5.vertical-force.threshold", annex), ndp_number("ndp.en1998.en5.vertical-force.ratio-high", annex), ndp_number("ndp.en1998.en5.vertical-force.ratio-low", annex));
        let topographic = if slope.topographic_amplification > 0.0 { slope.topographic_amplification } else { 1.0 };
        Some(SlopeForces { pseudostatic: slope_pseudostatic(ndp_number("ndp.en1998.en5.slope.horizontal-factor", annex), a_g_m_s2, s, topographic, vertical_share), gamma_phi: ndp_number("ndp.en1997.m2.phi", annex), gamma_c: ndp_number("ndp.en1997.m2.c", annex), unit_weight_n_m3: layer.unit_weight_n_m3, topographic, vertical_share })
    }

    /// ⛰️ Factors of the given circle of the slope and of the critical circle search for a slope of `height_m` and `angle_deg` with the friction angle `friction_angle_deg` (the other inputs from the slope row).
    pub fn slope_result(forces: &SlopeForces, slope: &semio_s_artifact_norm_en1998_model::En1998Slope, height_m: f64, angle_deg: f64, friction_angle_deg: f64) -> SlopeResult {
        let circle = slope_circle(height_m, angle_deg, forces.unit_weight_n_m3, slope.circle_centre_x, slope.circle_centre_z, slope.circle_radius);
        let fixed = slope_factor(&circle, friction_angle_deg, slope.cohesion, forces.gamma_phi, forces.gamma_c, &forces.pseudostatic).filter(|f| f.is_finite() && *f > 0.0).map(|f| (circle, f));
        let critical = slope_critical_circle(height_m, angle_deg, forces.unit_weight_n_m3, friction_angle_deg, slope.cohesion, forces.gamma_phi, forces.gamma_c, &forces.pseudostatic);
        SlopeResult { fixed, critical }
    }

    /// ⛰️ Factor of safety of the given slip circle of the slope for a horizontal seismic coefficient `horizontal` = F_H / W (the vertical share and the strengths of `forces` kept), `None` when the circle develops no sliding mass.
    pub fn slope_factor_at(forces: &SlopeForces, slope: &semio_s_artifact_norm_en1998_model::En1998Slope, horizontal: f64) -> Option<f64> {
        let circle = slope_circle(slope.height, slope.angle, forces.unit_weight_n_m3, slope.circle_centre_x, slope.circle_centre_z, slope.circle_radius);
        let p = semio_s_plugin_norm_mechanics::geotechnics::slope::Pseudostatic { horizontal, vertical: forces.vertical_share };
        slope_factor(&circle, slope.friction_angle, slope.cohesion, forces.gamma_phi, forces.gamma_c, &p).filter(|f| f.is_finite() && *f > 0.0)
    }

    /// ⛰️ The sliding mass of a slip circle: where the circle enters and leaves the ground, its weight per metre of slope width and its centroid (x, z).
    pub struct SlipMass {
        pub entry: [f64; 2],
        pub exit: [f64; 2],
        pub weight_n_per_m: f64,
        pub centroid: [f64; 2],
    }

    /// ⛰️ Entry, exit, weight and centroid of the sliding mass of the circle from the slices of the shared kernel, `None` when no sliding mass develops.
    pub fn slope_slip_mass(circle: &semio_s_plugin_norm_mechanics::geotechnics::slope::Circle) -> Option<SlipMass> {
        let slices = semio_s_plugin_norm_mechanics::geotechnics::slope::slices(circle, SLOPE_SLICES)?;
        let (first, last) = (slices.first()?, slices.last()?);
        let weight: f64 = slices.iter().map(|k| k.w).sum();
        let centroid = [slices.iter().map(|k| k.w * k.x).sum::<f64>() / weight, slices.iter().map(|k| k.w * (k.yb + 0.5 * k.h)).sum::<f64>() / weight];
        Some(SlipMass { entry: [first.x0, first.base[0]], exit: [last.x1, last.base[1]], weight_n_per_m: weight, centroid })
    }
}

pub mod part_6 {
    /// 🗼 Fundamental period of a uniform cantilever T = 2π / (3.516 √(EI / (μ H⁴))) = 1.787 √(M H³ / EI) with M the total mass; infinity without stiffness.
    pub fn uniform_cantilever_period_s(mass_kg: f64, height_m: f64, ei_nm2: f64) -> f64 {
        if ei_nm2 <= 0.0 || mass_kg <= 0.0 || height_m <= 0.0 { return f64::INFINITY; }
        2.0 * std::f64::consts::PI / 3.516015 * (mass_kg * height_m.powi(3) / ei_nm2).sqrt()
    }
    pub fn tower_behaviour_factor(q_nominal: f64, is_chimney: bool) -> f64 {
        if is_chimney { q_nominal.min(2.0).max(1.0) } else { q_nominal.min(3.5).max(1.0) }
    }
    /// 🗼 Equivalent lateral force factor λ = 1.0 of a uniform cantilever evaluated at the spectrum plateau (EN 1998-6 §4.3, the period needs the unavailable stiffness EI).
    pub fn cantilever_force_factor() -> f64 { 1.0 }
    pub fn tower_base_shear_n(gamma: f64, s_d: f64, mass_kg: f64) -> f64 { gamma * s_d * mass_kg }
    /// 🗼 Cantilever overturning M_Ed ≈ V · (2/3) H (EN 1998-6 §4.3).
    pub fn tower_overturning_moment_nm(v_ed_n: f64, height_m: f64) -> f64 { v_ed_n * (2.0 / 3.0) * height_m }
    /// 🗼 Roots β_n L of 1 + cos(βL) cosh(βL) = 0 of the clamped-free uniform beam; beyond the fifth the asymptote (2n − 1) π/2.
    pub fn cantilever_beta_l(n: u32) -> f64 {
        const ROOTS: [f64; 5] = [1.875_104_068_711_961, 4.694_091_132_974_175, 7.854_757_438_237_613, 10.995_540_734_875_467, 14.137_168_391_046_471];
        ROOTS.get(n.saturating_sub(1) as usize).copied().unwrap_or_else(|| f64::from(2 * n - 1) * std::f64::consts::FRAC_PI_2)
    }

    /// 🗼 Mode shape of the n-th cantilever mode at height ratio x/L, normalised to 1 at the tip: cosh − cos − σ (sinh − sin).
    pub fn cantilever_mode_shape(n: u32, ratio: f64) -> f64 {
        let b = cantilever_beta_l(n);
        let sigma = (b.cosh() + b.cos()) / (b.sinh() + b.sin());
        let raw = |x: f64| (b * x).cosh() - (b * x).cos() - sigma * ((b * x).sinh() - (b * x).sin());
        raw(ratio) / raw(1.0)
    }

    /// 🗼 Period ratio T_n/T_1 = (β_1 L/β_n L)² of the cantilever modes.
    pub fn cantilever_period_ratio(n: u32) -> f64 {
        (cantilever_beta_l(1) / cantilever_beta_l(n)).powi(2)
    }

    /// 🗼 Participation Γ_n = ∫φ dx/∫φ² dx of the tip-normalised mode of a uniform cantilever (midpoint rule, 400 cells).
    pub fn cantilever_participation(n: u32) -> f64 {
        let cells = 400;
        let (mut linear, mut square) = (0.0, 0.0);
        for i in 0..cells {
            let phi = cantilever_mode_shape(n, (f64::from(i) + 0.5) / f64::from(cells));
            linear += phi;
            square += phi * phi;
        }
        linear / square
    }
}
