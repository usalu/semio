//! 🧬️ EN 1998 artifact schema — mirrors the snapshot subject.

use crate::{
    En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998RetainingWall, En1998Site, En1998Silo,
    En1998Tank, En1998Tower,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Artifact {
    #[state(artifact)]
    pub annex: String,
    #[state(artifact)]
    pub site: En1998Site,
    #[state(artifact)]
    pub buildings: Vec<En1998Building>,
    #[state(artifact)]
    pub bridges: Vec<En1998Bridge>,
    #[state(artifact)]
    pub assessments: Vec<En1998Assessment>,
    #[state(artifact)]
    pub silos: Vec<En1998Silo>,
    #[state(artifact)]
    pub tanks: Vec<En1998Tank>,
    #[state(artifact)]
    pub foundations: Vec<En1998Foundation>,
    #[state(artifact)]
    pub retaining_walls: Vec<En1998RetainingWall>,
    #[state(artifact)]
    pub towers: Vec<En1998Tower>,
}
//#endregion 🔖️Artifact

impl En1998Artifact {
    pub fn to_snapshot(&self) -> crate::En1998Snapshot {
        crate::En1998Snapshot {
            annex: self.annex.clone(),
            site: self.site.clone(),
            buildings: self.buildings.clone(),
            bridges: self.bridges.clone(),
            assessments: self.assessments.clone(),
            silos: self.silos.clone(),
            tanks: self.tanks.clone(),
            foundations: self.foundations.clone(),
            retaining_walls: self.retaining_walls.clone(),
            towers: self.towers.clone(),
        }
    }

    pub fn from_snapshot(snapshot: crate::En1998Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            site: snapshot.site,
            buildings: snapshot.buildings,
            bridges: snapshot.bridges,
            assessments: snapshot.assessments,
            silos: snapshot.silos,
            tanks: snapshot.tanks,
            foundations: snapshot.foundations,
            retaining_walls: snapshot.retaining_walls,
            towers: snapshot.towers,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: crate::En1998Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

impl Default for En1998Artifact {
    fn default() -> Self {
        Self::from_snapshot(crate::En1998Snapshot::default())
    }
}

// Re-export formula modules (kept below for evaluate).

use crate::document::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

pub mod na_de {
    /// 🌋️ German seismic zone per DIN EN 1998-1/NA (classic zones 0–3).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeismicZone { Zone0, Zone1, Zone2, Zone3 }

    impl SeismicZone {
        /// 📐️ Reference PGA a_gR [m/s²] per DIN EN 1998-1/NA Table NA.1 (2011).
        pub fn a_gr(self) -> f64 {
            match self {
                Self::Zone0 => 0.0,
                Self::Zone1 => 0.4,
                Self::Zone2 => 0.6,
                Self::Zone3 => 0.8,
            }
        }

        pub fn as_u8(self) -> u8 {
            match self {
                Self::Zone0 => 0,
                Self::Zone1 => 1,
                Self::Zone2 => 2,
                Self::Zone3 => 3,
            }
        }
    }

    impl From<crate::DeSeismicZone> for SeismicZone {
        fn from(value: crate::DeSeismicZone) -> Self {
            match value {
                crate::DeSeismicZone::Zone0 => Self::Zone0,
                crate::DeSeismicZone::Zone1 => Self::Zone1,
                crate::DeSeismicZone::Zone2 => Self::Zone2,
                crate::DeSeismicZone::Zone3 => Self::Zone3,
            }
        }
    }

    /// 🪨 DE-NA ground/subsoil combination (A-R … C-S).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GroundCombo { AR, BR, CR, BT, CT, CS }

    impl From<crate::DeGroundCombo> for GroundCombo {
        fn from(value: crate::DeGroundCombo) -> Self {
            match value {
                crate::DeGroundCombo::AR => Self::AR,
                crate::DeGroundCombo::BR => Self::BR,
                crate::DeGroundCombo::CR => Self::CR,
                crate::DeGroundCombo::BT => Self::BT,
                crate::DeGroundCombo::CT => Self::CT,
                crate::DeGroundCombo::CS => Self::CS,
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

        /// 📊️ (S, T_B, T_C, T_D) per DIN EN 1998-1/NA Table NA.4.
        pub fn spectrum_params(self) -> (f64, f64, f64, f64) {
            match self {
                Self::AR => (1.00, 0.05, 0.20, 2.0),
                Self::BR => (1.25, 0.05, 0.25, 2.0),
                Self::CR => (1.50, 0.05, 0.30, 2.0),
                Self::BT => (1.25, 0.10, 0.30, 2.0),
                Self::CT => (1.50, 0.10, 0.40, 2.0),
                Self::CS => (0.75, 0.10, 0.50, 2.0),
            }
        }
    }
}

pub mod part_1 {
    /// 📊️ ψ₂ from EN 1990 Table A1.1 / DIN EN 1990/NA Table NA.A.1.1.
    pub fn psi2_de(category: &str) -> f64 {
        match category.trim().to_ascii_lowercase().as_str() {
            "residential" | "a" => 0.3,
            "office" | "b" => 0.3,
            "congregation" | "c" => 0.6,
            "retail" | "d" => 0.6,
            "storage" | "e" => 0.8,
            "traffic" | "f" => 0.6,
            "vehicles" | "g" => 0.3,
            "roofs" | "h" => 0.0,
            "snow" => 0.2,
            "wind" => 0.0,
            _ => 0.3,
        }
    }

    /// 🏗 φ from EN 1998-1 Table 4.2 (storeys / roofs × correlated / independent).
    pub fn phi_table_4_2(is_roof: bool, correlated_occupancy: bool) -> f64 {
        if is_roof {
            1.0
        } else if correlated_occupancy {
            0.8
        } else {
            0.5
        }
    }

    /// ⚖️ ψ_E,i = φ · ψ₂,i (EN 1998-1 §3.2.4).
    pub fn psi_e(category: &str, is_roof: bool, correlated_occupancy: bool) -> f64 {
        phi_table_4_2(is_roof, correlated_occupancy) * psi2_de(category)
    }

    /// 🏋️ Seismic combination gravity action on a storey [N] — ΣG_k + Σψ_E,i·Q_k,i (6.12b gravity part).
    pub fn storey_seismic_action_n(storey: &crate::En1998Storey, is_roof: bool) -> f64 {
        let mut sum = storey.permanent_gk_n;
        for v in &storey.variables {
            sum += psi_e(&v.category, is_roof, storey.correlated_occupancy) * v.qk_n;
        }
        sum
    }

    /// ⚖️ Seismic mass m = (ΣG_k + Σψ_E·Q_k)/g [kg] (EN 1998-1 §3.2.4).
    pub fn storey_seismic_mass_kg(storey: &crate::En1998Storey, is_roof: bool) -> f64 {
        storey_seismic_action_n(storey, is_roof) / 9.81
    }

    /// ⚖️ Seismic mass from ΣG_k + Σψ_E·Q_k for ancillary entities (bridges/towers).
    pub fn entity_seismic_mass_kg(permanent_gk_n: f64, variables: &[crate::En1998VariableAction], is_roof: bool, correlated: bool) -> f64 {
        let mut sum = permanent_gk_n;
        for v in variables {
            sum += psi_e(&v.category, is_roof, correlated) * v.qk_n;
        }
        sum / 9.81
    }

    /// 🫙 Silo/tank seismic mass with EN 1998-4 filling ratio on content Q_k.
    pub fn filled_content_seismic_mass_kg(permanent_gk_n: f64, content_qk_n: f64, content_category: &str, filling_ratio: f64) -> f64 {
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


    use super::*;

    pub fn gamma_i(importance: &str) -> f64 {
        match importance.trim().to_ascii_uppercase().as_str() {
            "I" | "CC1" => 0.8,
            "III" | "CC3" => 1.2,
            "IV" | "CC4" => 1.4,
            _ => 1.0,
        }
    }

    pub fn behaviour_factor(q0: f64, alpha_u_over_alpha_1: f64, k_w: f64) -> f64 {
        (q0 * alpha_u_over_alpha_1 * k_w).max(1.0)
    }

    /// 📐️ Upper bound on q for systemType×material×ductility (EN 1998-1 Tables 5.1/6.2/7.2/8.1/9.1 + DE-NA).
    pub fn q_max_for_system(system_type: &str, material: &str, ductility: &str) -> f64 {
        let st = system_type.trim().to_ascii_lowercase();
        let mat = material.trim().to_ascii_lowercase();
        let dc = ductility.trim().to_ascii_lowercase();
        match mat.as_str() {
            "rc" | "concrete" => match (st.as_str(), dc.as_str()) {
                ("frame", "dch") => 5.85, // 4.5 × 1.3
                ("frame", "dcm") => 3.9,
                ("frame", _) => 1.5,
                ("wall", "dch") => 4.4,
                ("wall", "dcm") => 3.0,
                ("wall", _) => 1.5,
                ("dual", "dch") => 5.2,
                ("dual", "dcm") => 3.6,
                ("dual", _) => 1.5,
                _ => 1.5,
            },
            "steel" => match (st.as_str(), dc.as_str()) {
                ("frame", "dch") => 6.5,
                ("frame", "dcm") => 4.0,
                ("frame", _) => 1.5,
                ("wall" | "dual", "dch") => 5.0,
                ("wall" | "dual", "dcm") => 4.0,
                _ => 1.5,
            },
            "composite" => match dc.as_str() {
                "dch" => 6.0,
                "dcm" => 4.0,
                _ => 1.5,
            },
            "timber" => match dc.as_str() {
                "dch" => 5.0,
                "dcm" => 2.5,
                _ => 1.5,
            },
            "masonry" => match dc.as_str() {
                "dcm" | "dch" => 2.5,
                _ => 1.5,
            },
            _ => 1.5,
        }
    }

    /// 🧱 RC column min b [m] for DCM/DCH (§5.4.3.2.1 / §5.5.3.2.1 simplified).
    pub fn rc_column_min_b_m(ductility: &str) -> f64 {
        match ductility.trim().to_ascii_lowercase().as_str() {
            "dch" => 0.25,
            "dcm" => 0.25,
            _ => 0.20,
        }
    }

    /// 🧱 RC ρ bounds (min, max) simplified from §5.4.3 / §5.5.3.
    pub fn rc_rho_limits(role: &str, ductility: &str) -> (f64, f64) {
        let dc = ductility.trim().to_ascii_lowercase();
        let role = role.trim().to_ascii_lowercase();
        match (role.as_str(), dc.as_str()) {
            ("column", "dch") => (0.01, 0.04),
            ("column", _) => (0.01, 0.04),
            ("beam", "dch") => (0.0035, 0.04),
            ("beam", _) => (0.0025, 0.04),
            _ => (0.002, 0.04),
        }
    }

    pub fn rc_omega_wd_min(ductility: &str) -> f64 {
        match ductility.trim().to_ascii_lowercase().as_str() {
            "dch" => 0.12,
            "dcm" => 0.08,
            _ => 0.0,
        }
    }

    /// 🏗️ Steel section class max allowed for given q (EN 1998-1 §6.5 / §6.6).
    pub fn steel_max_section_class(q: f64) -> u8 {
        if q > 4.0 { 1 } else if q > 2.0 { 2 } else { 3 }
    }

    pub fn drift_theta(limit_class: &str) -> f64 {
        match limit_class.trim().to_ascii_lowercase().as_str() {
            "brittle" => 0.005,
            "isolated" => 0.010,
            _ => 0.0075,
        }
    }

    /// 📈️ Elastic response spectrum S_e(T) [m/s²] per EN 1998-1 §3.2.2.2.
    pub fn elastic_response_spectrum(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, t: f64) -> f64 {
        let eta = 1.0;
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

    /// 📉️ Design spectrum S_d(T) [m/s²] with lower bound β·a_g (β = 0.2).
    pub fn design_spectrum_sd(s_e: f64, gamma_i: f64, q: f64, a_g: f64) -> f64 {
        let beta = 0.2;
        (s_e * gamma_i / q.max(1.0)).max(beta * a_g)
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

    pub fn lambda_factor(t1: f64, tc: f64, n_storeys: usize) -> f64 {
        if t1 <= 2.0 * tc && n_storeys > 2 { 0.85 } else { 1.0 }
    }

    pub fn base_shear_n(s_d: f64, mass_kg: f64, lambda: f64) -> f64 {
        s_d * mass_kg * lambda
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

    pub fn accidental_torsion_delta(e_ratio: f64, eccentricity_m: f64, plan_width_m: f64) -> f64 {
        let e_acc = e_ratio * plan_width_m;
        let e = eccentricity_m.abs() + e_acc;
        1.0 + e / plan_width_m.max(1e-6)
    }

    pub fn interstorey_drift_limit_m(height_m: f64, nu: f64, theta: f64) -> f64 {
        nu * theta * height_m
    }

    /// 📐️ Interstorey drift sensitivity θ = (P_tot · d_r) / (V_tot · h) per EN 1998-1 §4.4.2.2.
    pub fn drift_sensitivity(p_tot_n: f64, d_r_m: f64, v_tot_n: f64, h_m: f64) -> f64 {
        if v_tot_n.abs() < 1e-12 || h_m.abs() < 1e-12 { return 0.0; }
        (p_tot_n * d_r_m) / (v_tot_n * h_m)
    }

    pub fn lateral_force_method_applicable(t1: f64, tc: f64, elevation_regular: bool) -> bool {
        elevation_regular && t1 <= 4.0 * tc && t1 <= 2.0
    }

    /// 🧱 DE simple masonry Table NA.12-style wall-area ratio limit (simplified: ≥ 2% for zone 2 / IC II).
    pub fn masonry_wall_ratio_limit(zone: u8, importance: &str) -> f64 {
        let base = match zone { 0 => 0.0, 1 => 0.02, 2 => 0.03, _ => 0.04 };
        base * match importance.trim().to_ascii_uppercase().as_str() {
            "III" | "IV" => 1.25,
            _ => 1.0,
        }
    }
}

pub mod part_2 {
    use super::*;
    pub fn isolation_reduction_factor(period_ratio: f64) -> f64 {
        let r = period_ratio.max(1.0);
        (r * r).max(1.0)
    }
    /// 🌉 Design bearing displacement from design spectrum (EN 1998-2 §4.2 / §7): d = S_d · (T/2π)².
    pub fn bearing_displacement_m(s_d: f64, fundamental_period_s: f64) -> f64 {
        let t = fundamental_period_s.max(1e-6);
        s_d * (t / (2.0 * std::f64::consts::PI)).powi(2)
    }
}

pub mod part_3 {
    pub fn confidence_factor(kl: &str) -> f64 {
        match kl.trim().to_ascii_lowercase().as_str() {
            "kl1" => 1.35,
            "kl3" => 1.00,
            _ => 1.20,
        }
    }
    /// 📅 a_g multiplier vs SD from EN 1998-3 Table 2.1 return periods (+ DE NA practice).
    pub fn limit_state_ag_factor(limit_state: &str) -> f64 {
        match limit_state.trim().to_ascii_lowercase().as_str() {
            "nc" | "near_collapse" | "near-collapse" => 1.72,
            "dl" | "damage_limitation" | "damage-limitation" => 0.5,
            _ => 1.0, // sd / significant_damage
        }
    }
    /// 🔧 Whether CF applies for the limit state (DL uses CF = 1.0).
    pub fn limit_state_applies_cf(limit_state: &str) -> bool {
        !matches!(
            limit_state.trim().to_ascii_lowercase().as_str(),
            "dl" | "damage_limitation" | "damage-limitation"
        )
    }
    pub fn design_capacity_n(r_k_n: f64, cf: f64, gamma_el: f64) -> f64 {
        r_k_n / (cf * gamma_el.max(1e-9))
    }
}

pub mod part_4 {
    pub fn impulsive_mass_ratio(h_over_r: f64) -> f64 {
        (1.0 - 0.436 * h_over_r.tanh()).clamp(0.1, 0.9)
    }
    pub fn convective_mass_ratio(h_over_r: f64) -> f64 {
        (1.0 - impulsive_mass_ratio(h_over_r)).clamp(0.1, 0.9)
    }
    pub fn impulsive_period_s(height_m: f64, radius_m: f64) -> f64 {
        2.0 * std::f64::consts::PI * (0.574 * radius_m / (9.81 * height_m / radius_m.max(1e-9)).tanh().sqrt()).sqrt().max(1e-6)
    }
    pub fn convective_period_s(radius_m: f64) -> f64 {
        2.0 * std::f64::consts::PI * (radius_m / (1.84 * 9.81)).sqrt()
    }
    pub fn silo_behaviour_factor(q_nominal: f64) -> f64 { q_nominal.min(1.5).max(1.0) }
    pub fn srss(a: f64, b: f64) -> f64 { (a * a + b * b).sqrt() }
}

pub mod part_5 {
    pub fn bearing_reduction_factor(a_g_m_s2: f64) -> f64 {
        (1.0 - 1.5 * (a_g_m_s2 / 9.81)).max(0.5)
    }
    pub fn seismic_bearing_pressure_pa(v_b_n: f64, area_m2: f64) -> f64 {
        if area_m2 <= 0.0 { return 0.0; }
        v_b_n / area_m2
    }
    pub fn horizontal_seismic_coefficient(a_g_m_s2: f64, s: f64, r: f64) -> f64 {
        (a_g_m_s2 / 9.81) * s / r.max(1.0)
    }
    pub fn mononobe_okabe_k_ae(phi_deg: f64, k_h: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let theta = k_h.atan();
        let bracket = 1.0 + ((phi.sin() * (phi - theta).sin()) / theta.cos().max(1e-9)).sqrt();
        (phi - theta).cos().powi(2) / (theta.cos().max(1e-9) * bracket * bracket)
    }
    pub fn retaining_wall_thrust_n_per_m(gamma: f64, height_m: f64, k_ae: f64) -> f64 {
        0.5 * gamma * height_m * height_m * k_ae
    }
    pub fn stiffness_ratio(k_f: f64, k_s: f64) -> f64 {
        if k_s <= 0.0 { return 0.0; }
        k_f / k_s
    }
    pub fn radiation_damping(ratio: f64) -> f64 { (ratio / (1.0 + ratio)).clamp(0.0, 1.0) }
}

pub mod part_6 {
    pub fn tower_behaviour_factor(q_nominal: f64, is_chimney: bool) -> f64 {
        if is_chimney { q_nominal.min(2.0).max(1.0) } else { q_nominal.min(3.5).max(1.0) }
    }
    pub fn cantilever_modal_participation_factor() -> f64 { 1.5 }
    pub fn tower_base_shear_n(gamma: f64, s_d: f64, mass_kg: f64) -> f64 { gamma * s_d * mass_kg }
    /// 🗼 Cantilever overturning M_Ed ≈ V · (2/3) H (EN 1998-6 §4.3).
    pub fn tower_overturning_moment_nm(v_ed_n: f64, height_m: f64) -> f64 { v_ed_n * (2.0 / 3.0) * height_m }
}

/// 🇪️🇺️ Resolved seismic-action model.
#[derive(Clone, Debug, PartialEq)]
pub enum AnnexParams {
    De { zone: na_de::SeismicZone, combo: na_de::GroundCombo },
    En { a_gr: f64, ground: char, spectrum_type1: bool },
}

impl AnnexParams {
    pub fn choice(&self) -> AnnexChoice {
        match self { Self::De { .. } => AnnexChoice::De, Self::En { .. } => AnnexChoice::En }
    }

    pub fn ground_params(&self) -> (f64, f64, f64, f64, f64) {
        match self {
            Self::De { zone, combo } => {
                let (s, tb, tc, td) = combo.spectrum_params();
                (zone.a_gr(), s, tb, tc, td)
            }
            Self::En { a_gr, ground, spectrum_type1 } => {
                let (s, tb, tc, td) = en_ground_params(*ground, *spectrum_type1);
                (*a_gr, s, tb, tc, td)
            }
        }
    }

    pub fn elastic_response_spectrum(&self, t: f64) -> f64 {
        let (a_g, s, tb, tc, td) = self.ground_params();
        part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t)
    }
}

fn en_ground_params(ground: char, type1: bool) -> (f64, f64, f64, f64) {
    if type1 {
        match ground {
            'A' => (1.0, 0.15, 0.4, 2.0),
            'C' => (1.15, 0.20, 0.6, 2.0),
            'D' => (1.35, 0.20, 0.8, 2.0),
            'E' => (1.4, 0.15, 0.5, 2.0),
            _ => (1.2, 0.15, 0.5, 2.0),
        }
    } else {
        match ground {
            'A' => (1.0, 0.05, 0.25, 1.2),
            'C' => (1.5, 0.10, 0.25, 1.2),
            'D' => (1.8, 0.10, 0.30, 1.2),
            'E' => (1.6, 0.05, 0.25, 1.2),
            _ => (1.35, 0.05, 0.25, 1.2),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;


//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1998` — handcrafted schema facet leaves.
pub fn en1998_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1998",
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
    use crate::{En1998Diff, En1998Mutation, En1998Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1998BuilderConstruction {
        snapshot: En1998Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for En1998BuilderConstruction {
        type Snapshot = En1998Snapshot;
        type Mutation = En1998Mutation;
        type Diff = En1998Diff;
        fn empty() -> Self {
            Self { snapshot: En1998Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<En1998Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1998Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1998Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1998Parts {
        pub snapshot: Option<En1998Snapshot>,
    }

    pub struct En1998AnalyzerAnalysis;

    impl ArtifactAnalysis for En1998AnalyzerAnalysis {
        type Parts = En1998Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1998", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1998Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1998Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1998Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1998BuilderFacets {
        construction: En1998BuilderConstruction,
        analysis: En1998AnalyzerAnalysis,
        composition: super::super::io::derived_composition::En1998ComposerComposition,
    }
    builder: En1998Builder,
    analyzer: En1998Analyzer,
    composer: En1998Composer,
);
//#endregion 🧬️DerivedArtifactFacets
