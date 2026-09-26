//! ⚖️ EN 1996 masonry checks — DIN EN 1996-1-1/NA, 1-2, 2, 3/NA with EN 1990 combinations.

use crate::document::{
    AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef,
};
use crate::{
    ExposureClass, MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType,
};

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn clause(family: &str, part: &str, section: &str) -> ClauseId {
    ClauseId::new(family, part, section)
}

fn wall_subject(w: &MasonryWall, path: &str) -> SubjectRef {
    SubjectRef::new(&w.id, path, loc(&w.label_en, &w.label_de))
}

fn wall_path(wall: &MasonryWall, field: &str) -> String {
    format!("walls[id={}].{field}", wall.id)
}

fn load_case_path(wall: &MasonryWall, lc: &WallLoadCase, field: &str) -> String {
    format!("walls[id={}].loadCases[id={}].{field}", wall.id, lc.id)
}

fn concentrated_path(wall: &MasonryWall, lc: &WallLoadCase, cid: &str, field: &str) -> String {
    format!("walls[id={}].loadCases[id={}].concentrated[id={}].{field}", wall.id, lc.id, cid)
}

//#region ⚖️AnnexParams
#[derive(Clone, Copy, Debug)]
pub struct AnnexParams {
    pub annex: AnnexChoice,
    pub masonry_class: MasonryClass,
    pub accidental: bool,
}

impl AnnexParams {
    pub fn for_document(annex: AnnexChoice, masonry_class: MasonryClass, design_situation: DesignSituation) -> Self {
        Self {
            annex,
            masonry_class,
            accidental: matches!(design_situation, DesignSituation::Accidental),
        }
    }

    pub fn gamma_m(self) -> f64 {
        match self.annex {
            AnnexChoice::En => self.masonry_class.gamma_m_en(),
            AnnexChoice::De => self.masonry_class.gamma_m_de(self.accidental),
        }
    }
}
//#endregion

//#region 📐Fk
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FkFactors {
    pub k: f64,
    pub alpha: f64,
    pub beta: f64,
}

pub fn fk_factors(annex: AnnexChoice, material: UnitMaterial, group: UnitGroup, mortar: MortarType) -> FkFactors {
    match annex {
        AnnexChoice::En => fk_factors_en(material, group, mortar),
        AnnexChoice::De => fk_factors_de(material, group, mortar),
    }
}

fn fk_factors_en(material: UnitMaterial, group: UnitGroup, mortar: MortarType) -> FkFactors {
    if matches!(mortar, MortarType::ThinLayer) {
        return FkFactors { k: 0.80, alpha: 0.85, beta: 0.0 };
    }
    let k = match (material, group) {
        (UnitMaterial::Clay, UnitGroup::Group1) => 0.55,
        (UnitMaterial::Clay, UnitGroup::Group2) => 0.45,
        (UnitMaterial::Clay, _) => 0.40,
        (UnitMaterial::CalciumSilicate, UnitGroup::Group1) => 0.55,
        (UnitMaterial::CalciumSilicate, _) => 0.45,
        (UnitMaterial::Aerated, _) => 0.55,
        (UnitMaterial::Concrete, UnitGroup::Group1) => 0.55,
        (UnitMaterial::Concrete, _) => 0.45,
    };
    FkFactors { k, alpha: 0.70, beta: 0.30 }
}

fn fk_factors_de(material: UnitMaterial, group: UnitGroup, mortar: MortarType) -> FkFactors {
    if matches!(mortar, MortarType::ThinLayer) {
        return FkFactors { k: 0.70, alpha: 0.85, beta: 0.0 };
    }
    let k = match (material, group) {
        (UnitMaterial::Clay, UnitGroup::Group1) => 0.60,
        (UnitMaterial::Clay, UnitGroup::Group2) => 0.50,
        (UnitMaterial::Clay, UnitGroup::Group3) => 0.45,
        (UnitMaterial::Clay, UnitGroup::Group4) => 0.40,
        (UnitMaterial::CalciumSilicate, UnitGroup::Group1) => 0.60,
        (UnitMaterial::CalciumSilicate, _) => 0.50,
        (UnitMaterial::Aerated, _) => 0.60,
        (UnitMaterial::Concrete, UnitGroup::Group1) => 0.60,
        (UnitMaterial::Concrete, _) => 0.50,
    };
    FkFactors { k, alpha: 0.65, beta: 0.25 }
}

/// 📐️ Shape factor δ for normalized compressive strength (EN 772-1 / EN 1996-1-1).
pub fn shape_factor_delta(unit_height_m: f64, unit_width_m: f64, unit_length_m: f64) -> f64 {
    let h = (unit_height_m * 1000.0).max(1.0);
    let w = (unit_width_m * 1000.0).max(1.0);
    let l = (unit_length_m * 1000.0).max(1.0);
    // Simplified Table A.1 interpolation toward 100×100 mm reference; length enters for perpend aspect.
    let ratio = (h / 100.0).min(2.5).max(0.4);
    // Keep mild soft-saturation so unit width/length remain live across the practical masonry range.
    let width_factor = 1.0 - (-w / 250.0).exp();
    let length_factor = 1.0 - (-l / 400.0).exp();
    ((1.15 - 0.15 * ratio) * (0.90 + 0.20 * width_factor) * (0.92 + 0.16 * length_factor)).clamp(0.70, 1.40)
}

pub fn f_k_pa(annex: AnnexChoice, wall: &MasonryWall) -> f64 {
    let f = fk_factors(annex, wall.unit_material, wall.unit_group, wall.mortar_type);
    let delta = shape_factor_delta(wall.unit_height_m, wall.unit_width_m, wall.unit_length_m);
    let f_b = (wall.f_b_pa / 1e6 * delta).max(0.0); // normalize declared mean with δ
    let f_m = if wall.mortar_strength_pa > 0.0 {
        wall.mortar_strength_pa / 1e6
    } else {
        wall.mortar_class.f_m_mpa()
    };
    let f_k_mpa = f.k * f_b.powf(f.alpha) * f_m.powf(f.beta);
    f_k_mpa * 1e6
}

pub fn f_vk0_pa(mortar: MortarClass, material: UnitMaterial) -> f64 {
    let m = mortar.f_m_mpa();
    let base = match material {
        UnitMaterial::Clay | UnitMaterial::CalciumSilicate | UnitMaterial::Concrete => {
            if m >= 10.0 { 0.20 } else if m >= 5.0 { 0.15 } else { 0.10 }
        }
        UnitMaterial::Aerated => {
            if m >= 5.0 { 0.15 } else { 0.10 }
        }
    };
    base * 1e6
}

/// ✂️ f_vk with DE-NA upper limits (f_vk ≤ f_vlt from unit tensile / 0.045·f_b …).
pub fn f_vk_pa(annex: AnnexChoice, mortar: MortarClass, material: UnitMaterial, sigma_d_pa: f64, f_b_pa: f64) -> f64 {
    let f_vk0 = f_vk0_pa(mortar, material);
    let raw = f_vk0 + 0.4 * sigma_d_pa.max(0.0);
    match annex {
        AnnexChoice::En => raw,
        AnnexChoice::De => {
            // DIN EN 1996-1-1/NA: f_vk ≤ f_vlt with f_vlt ≈ 0.045·f_b (calibrated from f_bt,cal path).
            let f_vlt = 0.045 * f_b_pa.max(0.0);
            raw.min(f_vlt).min(f_vk0 + 0.4 * sigma_d_pa.max(0.0))
        }
    }
}

pub fn f_xk_pa(direction: u8, mortar: MortarClass, material: UnitMaterial) -> f64 {
    let m = mortar.f_m_mpa();
    let (fxk1, fxk2) = match material {
        UnitMaterial::Clay => (0.10_f64.min(0.020 * m), 0.20_f64.min(0.040 * m)),
        UnitMaterial::CalciumSilicate => (0.10_f64.min(0.020 * m), 0.20_f64.min(0.040 * m)),
        UnitMaterial::Aerated => (0.05_f64, 0.10_f64),
        UnitMaterial::Concrete => (0.10_f64.min(0.020 * m), 0.20_f64.min(0.040 * m)),
    };
    if direction == 1 { fxk1 * 1e6 } else { fxk2 * 1e6 }
}
//#endregion

//#region 📐Geometry
pub fn t_ef_m(wall: &MasonryWall) -> f64 {
    wall.thickness_m
}

/// 🪟 Opening-aware length reduction for ρ_n (§5.5.1.4 — openings that pierce the wall height).
pub fn effective_length_m(wall: &MasonryWall) -> f64 {
    let mut l = wall.length_m;
    for o in &wall.openings {
        // Full-height pier openings (sill≈0 and height≈wall height) remove length.
        let pier = o.sill_height_m <= 0.05 * wall.height_m && o.height_m >= 0.85 * wall.height_m;
        if pier {
            l -= o.width_m;
        } else {
            // Partial openings: reduce by width × (height/H) fraction of stiffness.
            l -= o.width_m * (o.height_m / wall.height_m.max(1e-6)).clamp(0.0, 1.0) * 0.5;
        }
    }
    l.max(0.1 * wall.length_m)
}

pub fn rho_n(support_sides: u8, h_m: f64, l_m: f64) -> f64 {
    let rho2 = 0.75;
    let h = h_m.max(1e-6);
    let l = l_m.max(1e-6);
    match support_sides {
        2 => rho2,
        3 => rho2 / (1.0 + (rho2 * h / (3.0 * l)).powi(2)),
        _ => rho2 / (1.0 + (rho2 * h / l).powi(2)),
    }
}

pub fn h_ef_m(wall: &MasonryWall) -> f64 {
    rho_n(wall.support_sides, wall.height_m, effective_length_m(wall)) * wall.height_m
}

pub fn slenderness(wall: &MasonryWall) -> f64 {
    h_ef_m(wall) / t_ef_m(wall).max(1e-6)
}

/// 🪟 Net loaded area A = ℓ_ef·t (§5.5.1.4 — openings reduce ℓ_ef by width and height).
pub fn area_m2(wall: &MasonryWall) -> f64 {
    effective_length_m(wall) * wall.thickness_m
}

pub fn section_modulus_m3(wall: &MasonryWall) -> f64 {
    effective_length_m(wall) * wall.thickness_m.powi(2) / 6.0
}

/// 🏋️ Wall self-weight [N] (density × g × net volume).
pub fn self_weight_n(wall: &MasonryWall) -> f64 {
    let vol_gross = wall.thickness_m * wall.length_m * wall.height_m;
    let vol_open: f64 = wall.openings.iter().map(|o| o.width_m * o.height_m * wall.thickness_m).sum();
    wall.density_kg_m3 * 9.81 * (vol_gross - vol_open).max(0.0)
}

/// ↘️ Eccentricity from slab bearing (t/2 − a/2) per §6.1.2.2 / Annex C.
pub fn eccentricity_from_slab_bearing_m(wall: &MasonryWall) -> f64 {
    let t = wall.thickness_m;
    let a = wall.slab_bearing_depth_m.clamp(0.0, t);
    (t / 2.0 - a / 2.0).abs()
}

/// 🔄 Slab-end rotation eccentricity e_θ from floor span ℓ_f (DIN EN 1996-1-1/NA NDP 6.1.2.2 / Annex C).
///
/// DE NA simplified: e_θ = (N_floor/N_Ed)·ℓ_f/25 with ℓ_f capped at 6,0 m (NA span limit linked to Φ₁ =
/// max(0; 1,6 − ℓ_f/6) for 4,5 m ≤ ℓ_f ≤ 6,0 m). Folded into e₀ / e_mk so span moves compression Φ.
pub fn eccentricity_from_slab_rotation_m(wall: &MasonryWall, lc: &WallLoadCase, n_ed: f64) -> f64 {
    let l_f = lc.slab_span_m.clamp(0.0, 6.0);
    let e_full = l_f / 25.0;
    let n_floor = (lc.g_k_slab_n + lc.q_k_imposed_pa * lc.tributary_area_m2
        + lc.q_k_snow_pa * lc.tributary_area_m2)
        .max(0.0);
    let share = if n_ed.abs() > 1e-6 {
        (n_floor / n_ed.abs()).clamp(0.0, 1.0)
    } else {
        1.0
    };
    (e_full * share).min(0.4 * wall.thickness_m.max(1e-6))
}

/// 📉 DE-NA Φ₁ span reduction (NDP 6.1.2.2 / NA Annex C): 1,6 − ℓ_f/6 on 4,5…6,0 m, else 1,0 below 4,5 m.
pub fn phi_1_slab_span(slab_span_m: f64) -> f64 {
    let l_f = slab_span_m.clamp(0.0, 6.0);
    if l_f <= 4.5 {
        1.0
    } else {
        (1.6 - l_f / 6.0).clamp(0.0, 1.0)
    }
}
//#endregion

//#region 📉Phi
pub fn phi_i(eccentricity_m: f64, t_m: f64) -> f64 {
    let t = t_m.max(1e-6);
    let e = eccentricity_m.abs().max(0.05 * t);
    (1.0 - 2.0 * e / t).clamp(0.0, 1.0)
}

pub fn phi_m(lambda: f64, e_mk_m: f64, t_m: f64) -> f64 {
    let t = t_m.max(1e-6);
    let e_mk = e_mk_m.abs().max(0.05 * t);
    let a1 = (1.0 - 2.0 * e_mk / t).clamp(0.0, 1.0);
    if a1 <= 0.0 {
        return 0.0;
    }
    let u = ((lambda - 2.0) * (t - 2.0 * e_mk) / (23.0 * t)).max(0.0);
    a1 * (-0.5 * u * u).exp()
}

pub fn phi_s(annex: AnnexChoice, lambda: f64) -> f64 {
    let base = match annex {
        AnnexChoice::En => 0.85,
        AnnexChoice::De => 0.70,
    };
    (base - 0.0011 * lambda * lambda).max(0.0)
}

/// 📉 Creep eccentricity e_k = 0.002·φ_∞·(h_ef/t_ef)·√(t·e_m) (§6.1.2.2 / DE NA).
pub fn creep_eccentricity_m(wall: &MasonryWall, e_m: f64) -> f64 {
    let t = wall.thickness_m.max(1e-6);
    let hef = h_ef_m(wall);
    let phi = wall.phi_infinity.max(0.0);
    let em = e_m.abs().max(1e-9);
    0.002 * phi * (hef / t) * (t * em).sqrt()
}
//#endregion

//#region 🔥Fire
/// 🔥 DE-NA tabulated min thickness [m] by REI, unit group, mortar type and utilization α.
pub fn fire_min_thickness_na(rei_min: u32, material: UnitMaterial, group: UnitGroup, mortar: MortarType, alpha: f64) -> f64 {
    // Base EN 1996-1-2 Table N.B values (α ≤ 1.0), then DE-NA α class adjustments.
    let clay = |r: u32| match r {
        0..=30 => 90.0,
        31..=60 => 100.0,
        61..=90 => 140.0,
        91..=120 => 170.0,
        121..=180 => 210.0,
        _ => 270.0,
    };
    let casi = |r: u32| match r {
        0..=30 => 90.0,
        31..=60 => 115.0,
        61..=90 => 140.0,
        91..=120 => 175.0,
        121..=180 => 240.0,
        _ => 300.0,
    };
    let aac = |r: u32| match r {
        0..=30 => 115.0,
        31..=60 => 140.0,
        61..=90 => 175.0,
        91..=120 => 200.0,
        121..=180 => 250.0,
        _ => 300.0,
    };
    let concrete = |r: u32| match r {
        0..=30 => 90.0,
        31..=60 => 100.0,
        61..=90 => 140.0,
        91..=120 => 170.0,
        121..=180 => 240.0,
        _ => 300.0,
    };
    let mut mm = match material {
        UnitMaterial::Clay => clay(rei_min),
        UnitMaterial::CalciumSilicate => casi(rei_min),
        UnitMaterial::Aerated => aac(rei_min),
        UnitMaterial::Concrete => concrete(rei_min),
    };
    // DE NA Tables NA.B.x: α classes — α≤0.6 (α_2) thinner allowance; α>0.6 (α_6,fi) full table.
    if alpha <= 0.60 {
        mm *= 0.90; // α_2 column (reduced utilization)
    } else if alpha > 1.0 {
        mm *= 1.15; // beyond α_6,fi — thicker required
    }
    // Group 2–4 and thin-layer mortar: NA increases min thickness slightly.
    if !matches!(group, UnitGroup::Group1) {
        mm += 10.0;
    }
    if matches!(mortar, MortarType::ThinLayer) {
        mm += 5.0;
    }
    mm / 1000.0
}

/// 🔥 Compatibility: tabulated min thickness at α=1.0, Group1, GP mortar.
pub fn fire_min_thickness_m(rei_min: u32, material: UnitMaterial) -> f64 {
    fire_min_thickness_na(rei_min, material, UnitGroup::Group1, MortarType::GeneralPurpose, 1.0)
}
//#endregion

pub fn required_mortar_mpa(exposure: ExposureClass, material: UnitMaterial) -> f64 {
    match (exposure, material) {
        (ExposureClass::Mx1, _) => 1.0,
        (ExposureClass::Mx2, UnitMaterial::Aerated) => 2.5,
        (ExposureClass::Mx2, _) => 2.5,
        (ExposureClass::Mx3, UnitMaterial::Aerated) => 5.0,
        (ExposureClass::Mx3, _) => 5.0,
        (ExposureClass::Mx4, UnitMaterial::Aerated) => f64::INFINITY,
        (ExposureClass::Mx4, _) => 10.0,
        (ExposureClass::Mx5, _) => f64::INFINITY,
    }
}

pub fn concentrated_beta(a1: f64, h: f64, l: f64) -> f64 {
    let x = ((l / 2.0) - a1 / 2.0).max(0.0);
    let beta = (1.0 + 0.15 * x / h.max(1e-6)) * (1.5 - 1.5 * a1 / l.max(1e-6));
    beta.clamp(1.0, 1.5)
}

//#region ⚖️EN1990
fn situation_from_str(s: &str) -> DesignSituation {
    match s.trim().to_ascii_lowercase().as_str() {
        "accidental" | "bsa" => DesignSituation::Accidental,
        "seismic" => DesignSituation::Seismic,
        "transient" | "bst" => DesignSituation::Transient,
        _ => DesignSituation::Persistent,
    }
}

fn psi0_imposed(category: &str) -> f64 {
    match category.trim().to_ascii_uppercase().chars().next().unwrap_or('A') {
        'A' | 'B' => 0.7, // residential/office DE NA
        'C' | 'D' => 0.7,
        'E' => 1.0,
        'H' => 0.0, // roofs
        _ => 0.7,
    }
}

fn psi0_snow() -> f64 { 0.5 }
fn psi0_wind() -> f64 { 0.6 }

#[derive(Clone, Debug)]
struct DesignEffects {
    n_top: f64,
    n_mid: f64,
    n_bot: f64,
    v: f64,
    w_pa: f64,
    combo_en: String,
    combo_de: String,
}

fn gamma_g_sup(annex: AnnexChoice) -> f64 {
    match annex {
        AnnexChoice::De | AnnexChoice::En => 1.35,
    }
}
fn gamma_g_inf() -> f64 { 1.0 }
fn gamma_q(annex: AnnexChoice, accidental: bool) -> f64 {
    if accidental {
        1.0
    } else {
        match annex {
            AnnexChoice::De | AnnexChoice::En => 1.5,
        }
    }
}

/// ⚖️ Form EN 1990 (+ DE NA) ULS design effects from characteristic actions + self-weight.
fn design_effects(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, default_sit: DesignSituation) -> Vec<DesignEffects> {
    let sit = situation_from_str(&lc.design_situation);
    let accidental = matches!(sit, DesignSituation::Accidental) || matches!(default_sit, DesignSituation::Accidental);
    let g_self = self_weight_n(wall);
    let g_slab = lc.g_k_slab_n.max(0.0);
    let q_imp = (lc.q_k_imposed_pa * lc.tributary_area_m2).max(0.0);
    let q_snow = (lc.q_k_snow_pa * lc.tributary_area_m2).max(0.0);
    let f_conc: f64 = lc.concentrated.iter().map(|c| c.force_n.max(0.0)).sum();
    let w_k = (lc.q_p_wind_pa * lc.c_pe.abs()).max(0.0);
    let h_earth = lc.h_k_earth_n.max(0.0);
    // Horizontal shear from wind on facade + earth.
    let v_wind_k = w_k * wall.height_m * wall.length_m;
    let v_k = v_wind_k + h_earth;

    let yg_sup = if matches!(sit, DesignSituation::Transient) { 1.20 } else { gamma_g_sup(annex) };
    let yg_inf = gamma_g_inf();
    let yq = gamma_q(annex, accidental || matches!(sit, DesignSituation::Seismic));
    let psi_i = psi0_imposed(&lc.imposed_category);
    let psi_s = psi0_snow();
    let psi_w = psi0_wind();
    let sit_label = lc.design_situation.as_str();

    let stations = |q_lead: f64, q_acc1: f64, q_acc2: f64, lead_name: &str| {
        let n_perm_top = g_slab + f_conc;
        let n_perm_mid = g_slab + f_conc + 0.5 * g_self;
        let n_perm_bot = g_slab + f_conc + g_self;
        let n_top = yg_sup * n_perm_top + yq * q_lead + yq * q_acc1 + yq * q_acc2;
        let n_mid = yg_sup * n_perm_mid + yq * q_lead + yq * q_acc1 + yq * q_acc2;
        let n_bot = yg_sup * n_perm_bot + yq * q_lead + yq * q_acc1 + yq * q_acc2;
        (n_top, n_mid, n_bot, lead_name.to_string())
    };

    let mut out = Vec::new();

    // Max-N: imposed leading (ψ0,cat enters via accompanying snow or as companion scale on Q)
    {
        let q_lead = q_imp;
        let q_acc = if q_snow > 0.0 { psi_s * q_snow } else { psi_i * q_imp * 0.25 };
        let (nt, nm, nb, name) = stations(q_lead, q_acc, 0.0, "imposed");
        let v = yq * v_k;
        let w = yq * w_k;
        out.push(DesignEffects {
            n_top: nt, n_mid: nm, n_bot: nb, v, w_pa: w,
            combo_en: format!("ULS max-N [{sit_label}/cat {}]: γ_G·G+γ_Q·Q_{name}+ψ₀,cat={psi_i:.2}·Q_acc", lc.imposed_category),
            combo_de: format!("GZT max-N [{sit_label}/Kat. {}]: γ_G·G+γ_Q·Q_{name}+ψ₀,Kat={psi_i:.2}·Q_begl", lc.imposed_category),
        });
    }
    // Max-N: snow leading (if present)
    if q_snow > 0.0 {
        let (nt, nm, nb, _) = stations(q_snow, psi_i * q_imp, 0.0, "snow");
        out.push(DesignEffects {
            n_top: nt, n_mid: nm, n_bot: nb, v: yq * psi_w * v_k, w_pa: yq * psi_w * w_k,
            combo_en: "ULS max-N: γ_G·G+γ_Q·Q_snow+ψ₀·Q_imposed".into(),
            combo_de: "GZT max-N: γ_G·G+γ_Q·Q_Schnee+ψ₀·Q_Nutz".into(),
        });
    }
    // Min-N / max-H: favourable G + wind/earth leading (sliding & lateral)
    {
        let n_top = yg_inf * (g_slab + f_conc);
        let n_mid = yg_inf * (g_slab + f_conc + 0.5 * g_self);
        let n_bot = yg_inf * (g_slab + f_conc + g_self);
        out.push(DesignEffects {
            n_top, n_mid, n_bot,
            v: yq * v_k,
            w_pa: yq * w_k,
            combo_en: "ULS min-N/max-H: γ_G,inf·G+γ_Q·(W+E)".into(),
            combo_de: "GZT min-N/max-H: γ_G,inf·G+γ_Q·(W+E)".into(),
        });
    }
    out
}

fn governing_compression(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| a.n_bot.partial_cmp(&b.n_bot).unwrap()).unwrap_or(&effects[0])
}

fn governing_shear(effects: &[DesignEffects]) -> &DesignEffects {
    // Critical: high V with low N (friction) — maximize V/N ratio roughly max V then min N
    effects.iter().max_by(|a, b| {
        let ka = a.v / a.n_mid.max(1.0);
        let kb = b.v / b.n_mid.max(1.0);
        ka.partial_cmp(&kb).unwrap()
    }).unwrap_or(&effects[0])
}

fn governing_lateral(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| a.w_pa.partial_cmp(&b.w_pa).unwrap()).unwrap_or(&effects[0])
}
//#endregion

//#region ⚖️MaterialConformance
fn material_conformance_ok(wall: &MasonryWall) -> (bool, String, String) {
    let class_fm = wall.mortar_class.f_m_mpa() * 1e6;
    let declared = wall.mortar_strength_pa;
    let mortar_ok = (declared - class_fm).abs() <= 0.15 * class_fm.max(1e6);
    // Unit group vs dimensions (Group 1 solid-ish: voidage implied by height/width)
    let aspect = wall.unit_height_m / wall.unit_width_m.max(1e-6);
    let group_ok = match wall.unit_group {
        UnitGroup::Group1 => wall.unit_width_m >= 0.09 && wall.unit_length_m >= 0.09,
        UnitGroup::Group2 => wall.unit_width_m >= 0.09 && wall.unit_length_m >= 0.09 && aspect < 2.5,
        _ => wall.unit_width_m >= 0.09 && wall.unit_length_m >= 0.09,
    };
    // Thin-layer mortar requires thin bed joints
    let joint_ok = if matches!(wall.mortar_type, MortarType::ThinLayer) {
        wall.bed_joint_thickness_m <= 0.003 && wall.bed_joint_thickness_m >= 0.0005
    } else {
        wall.bed_joint_thickness_m >= 0.006 && wall.bed_joint_thickness_m <= 0.015
    };
    // f_b reasonable for material
    let fb_mpa = wall.f_b_pa / 1e6;
    let fb_ok = match wall.unit_material {
        UnitMaterial::Aerated => (1.5..=10.0).contains(&fb_mpa),
        UnitMaterial::Clay => (4.0..=100.0).contains(&fb_mpa),
        _ => (2.0..=100.0).contains(&fb_mpa),
    };
    let ok = mortar_ok && group_ok && joint_ok && fb_ok;
    let en = format!(
        "Mortar f_m={:.1} MPa vs class {:.1}; group/dims ok={group_ok}; bed joint ok={joint_ok}; f_b={fb_mpa:.1} MPa ok={fb_ok}.",
        declared / 1e6,
        class_fm / 1e6
    );
    let de = format!(
        "Mörtel f_m={:.1} MPa vs Klasse {:.1}; Gruppe/Abmessungen ok={group_ok}; Lagerfuge ok={joint_ok}; f_b={fb_mpa:.1} MPa ok={fb_ok}.",
        declared / 1e6,
        class_fm / 1e6
    );
    (ok, en, de)
}
//#endregion

//#region ⚖️Evaluate
pub fn evaluate_building(
    annex: AnnexChoice,
    masonry_class: MasonryClass,
    design_situation: DesignSituation,
    storeys: u32,
    walls: &[MasonryWall],
) -> CheckReport {
    let mut report = CheckReport::default();
    if walls.is_empty() {
        report.push(
            CheckResult::assess(
                "en1996.subject.empty",
                "EN 1996",
                clause("EN 1996-1-1", "§1", "1"),
                SubjectRef::new("building", "walls", loc("Building", "Gebäude")),
                loc("Walls present", "Wände vorhanden"),
            )
            .not_applicable(loc("No walls in subject.", "Keine Wände im Gegenstand."))
            .annex(annex)
            .build(),
        );
        return report;
    }
    for wall in walls {
        for c in evaluate_wall(annex, masonry_class, design_situation, storeys, wall) {
            report.push(c);
        }
    }
    report
}

fn evaluate_wall(
    annex: AnnexChoice,
    masonry_class: MasonryClass,
    default_situation: DesignSituation,
    storeys: u32,
    wall: &MasonryWall,
) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let path_t = wall_path(wall, "thicknessM");
    let path_h = wall_path(wall, "heightM");
    let path_support = wall_path(wall, "supportSides");
    let path_fb = wall_path(wall, "fBPa");
    let path_mortar = wall_path(wall, "mortarClass");
    let path_mortar_f = wall_path(wall, "mortarStrengthPa");
    let path_mu = wall_path(wall, "mu");
    let path_bearing = wall_path(wall, "slabBearingDepthM");
    let path_as_h = wall_path(wall, "asHorizontalM2");
    let path_unit_h = wall_path(wall, "unitHeightM");
    let path_density = wall_path(wall, "densityKgM3");
    let path_phi_inf = wall_path(wall, "phiInfinity");
    let path_basement = wall_path(wall, "isBasement");

    let lambda = slenderness(wall);
    let a = area_m2(wall);
    let f_k = f_k_pa(annex, wall);
    let t = wall.thickness_m;

    // Slenderness
    {
        let mut b = CheckResult::assess(
            format!("en1996.5.5.1.slenderness.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§5.5.1", "5.5.1"),
            wall_subject(wall, &path_t),
            loc("Slenderness λ ≤ 27", "Schlankheit λ ≤ 27"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, lambda), Quantity::new(QuantityKind::Dimensionless, 27.0))
        .annex(annex)
        .explanation(loc(
            &format!("λ=h_ef/t_ef={lambda:.2} with L_eff={:.3} m (openings).", effective_length_m(wall)),
            &format!("λ=h_ef/t_ef={lambda:.2} mit L_eff={:.3} m (Öffnungen).", effective_length_m(wall)),
        ));
        if lambda > 27.0 {
            let t_req = h_ef_m(wall) / 27.0;
            b = b
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t_req),
                    loc("Increase thickness so λ ≤ 27.", "Dicke erhöhen, damit λ ≤ 27."),
                ))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_support),
                    Quantity::new(QuantityKind::Dimensionless, wall.support_sides as f64),
                    Quantity::new(QuantityKind::Dimensionless, 4.0),
                    loc("Increase to 4-sided support.", "Auf 4-seitige Halterung erhöhen."),
                ));
        }
        out.push(b.build());
    }

    // Material conformance (replaces tautological f_k ≥ 0.5 MPa)
    {
        let (ok, en, de) = material_conformance_ok(wall);
        let mut b = CheckResult::assess(
            format!("en1996.3.1.material.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§3.1", "3.1"),
            wall_subject(wall, &path_mortar_f),
            loc("Unit/mortar conformance (DE NA)", "Stein/Mörtel-Konformität (DE NA)"),
        )
        .annex(annex)
        .explanation(loc(&en, &de));
        if ok {
            b = b.utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0));
        } else {
            b = b
                .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_mortar_f),
                    Quantity::new(QuantityKind::Stress, wall.mortar_strength_pa),
                    Quantity::new(QuantityKind::Stress, wall.mortar_class.f_m_mpa() * 1e6),
                    loc("Align mortarStrengthPa with mortarClass.", "mortarStrengthPa an mortarClass anpassen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_fb),
                    Quantity::new(QuantityKind::Stress, wall.f_b_pa),
                    Quantity::stress_mpa(6.0),
                    loc("Raise unit strength f_b into admissible range.", "f_b in zulässigen Bereich anheben."),
                ))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_unit_h),
                    Quantity::length_m(wall.unit_height_m),
                    Quantity::length_m(0.113),
                    loc("Use admissible unit height for group/shape factor δ.", "Zulässige Steinhöhe für Formfaktor δ wählen."),
                ));
        }
        out.push(b.build());
    }

    if wall.load_cases.is_empty() {
        out.push(
            CheckResult::assess(
                format!("en1996.loads.empty.{}", wall.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§2", "2"),
                wall_subject(wall, &wall_path(wall, "loadCases")),
                loc("Load cases present", "Lastfälle vorhanden"),
            )
            .not_applicable(loc("No load cases on wall.", "Keine Lastfälle an der Wand."))
            .annex(annex)
            .build(),
        );
        return out;
    }

    for lc in &wall.load_cases {
        let effects = design_effects(annex, wall, lc, default_situation);
        let sit = situation_from_str(&lc.design_situation);
        let params = AnnexParams::for_document(annex, masonry_class, if matches!(sit, DesignSituation::Persistent) { default_situation } else { sit });
        let gamma_m = params.gamma_m();
        let f_d = f_k / gamma_m;

        // Compression §6.1.2
        {
            let ge = governing_compression(&effects);
            let e_slab = eccentricity_from_slab_bearing_m(wall);
            let e_theta = eccentricity_from_slab_rotation_m(wall, lc, ge.n_top.max(ge.n_mid));
            let phi1 = phi_1_slab_span(lc.slab_span_m);
            let e_top = (wall.eccentricity_top_m + e_slab + e_theta).abs();
            let e_bot = wall.eccentricity_bottom_m.abs();
            let h_ef = h_ef_m(wall);
            let e_init = h_ef / 450.0;
            let e_i_top = (e_top + e_init).abs();
            let e_i_bot = (e_bot + e_init).abs();
            let e_m = 0.5 * (e_top + e_bot) + e_init;
            let e_k = creep_eccentricity_m(wall, e_m);
            let e_mk = (e_m + e_k).abs().max(0.05 * t);
            let phi_top = phi_i(e_i_top, t) * phi1;
            let phi_bot = phi_i(e_i_bot, t) * phi1;
            let phi_mid = phi_m(lambda, e_mk, t) * phi1;
            let stations = [("top", ge.n_top, phi_top), ("mid", ge.n_mid, phi_mid), ("bottom", ge.n_bot, phi_bot)];
            let (station, n_ed, phi) = stations.iter().copied().max_by(|a, b| (a.1 / a.2.max(1e-9)).partial_cmp(&(b.1 / b.2.max(1e-9))).unwrap()).unwrap();
            let n_rd = phi * f_d * a;
            let path_span = load_case_path(wall, lc, "slabSpanM");
            let mut b = CheckResult::assess(
                format!("en1996.6.1.2.compression.{}.{}", wall.id, lc.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.1.2", "6.1.2"),
                wall_subject(wall, &path_t),
                loc("Compression N_Ed ≤ Φ·Φ₁·f_d·A", "Druck N_Ed ≤ Φ·Φ₁·f_d·A"),
            )
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!(
                    "{}; station={station}; Φ·Φ₁={phi:.3} (Φ₁={phi1:.3}, NDP 6.1.2.2/NA C); e_top={e_top:.4}; e_θ={e_theta:.4} (ℓ_f={:.2} m); e_bot={e_bot:.4}; e_mk={e_mk:.4} m (e_k={e_k:.4}, φ∞={:.2}, ρ={:.0}); N_Ed={n_ed:.0} N; N_Rd={n_rd:.0} N; e_bearing={e_slab:.4} m.",
                    ge.combo_en, lc.slab_span_m, wall.phi_infinity, wall.density_kg_m3
                ),
                &format!(
                    "{}; Stelle={station}; Φ·Φ₁={phi:.3} (Φ₁={phi1:.3}, NDP 6.1.2.2/NA C); e_oben={e_top:.4}; e_θ={e_theta:.4} (ℓ_f={:.2} m); e_unten={e_bot:.4}; e_mk={e_mk:.4} m (e_k={e_k:.4}, φ∞={:.2}, ρ={:.0}); N_Ed={n_ed:.0} N; N_Rd={n_rd:.0} N; e_Auflager={e_slab:.4} m.",
                    ge.combo_de, lc.slab_span_m, wall.phi_infinity, wall.density_kg_m3
                ),
            ));
            if n_ed > n_rd {
                let t_req = t * (n_ed / n_rd.max(1e-12)).max(1.0);
                let fb_req = wall.f_b_pa * (n_ed / n_rd.max(1e-12)).max(1.0);
                b = b
                    .remedy(Remedy::at_least(
                        wall_subject(wall, &path_t),
                        Quantity::length_m(t),
                        Quantity::length_m(t_req),
                        loc("Increase thickness.", "Dicke erhöhen."),
                    ))
                    .remedy(Remedy::at_least(
                        wall_subject(wall, &path_fb),
                        Quantity::new(QuantityKind::Stress, wall.f_b_pa),
                        Quantity::new(QuantityKind::Stress, fb_req),
                        loc("Increase f_b.", "f_b erhöhen."),
                    ))
                    .remedy(Remedy::at_least(
                        wall_subject(wall, &path_bearing),
                        Quantity::length_m(wall.slab_bearing_depth_m),
                        Quantity::length_m((t / 3.0).max(wall.slab_bearing_depth_m)),
                        loc("Increase slab bearing depth to reduce eccentricity.", "Deckenauflager vertiefen, Exzentrizität reduzieren."),
                    ))
                    .remedy(Remedy::at_most(
                        wall_subject(wall, &path_span),
                        Quantity::length_m(lc.slab_span_m),
                        Quantity::length_m((lc.slab_span_m * 0.85).max(4.0)),
                        loc("Reduce slab span to raise Φ₁ / cut e_θ (NA 6.1.2.2).", "Deckenspannweite reduzieren (Φ₁/e_θ, NA 6.1.2.2)."),
                    ))
                    .remedy(Remedy::at_most(
                        wall_subject(wall, &path_phi_inf),
                        Quantity::new(QuantityKind::Dimensionless, wall.phi_infinity),
                        Quantity::new(QuantityKind::Dimensionless, (wall.phi_infinity * 0.7).max(0.5)),
                        loc("Reduce φ∞ to cut creep eccentricity e_k.", "φ∞ senken, Kriechexzentrizität e_k reduzieren."),
                    ))
                    .remedy(Remedy::at_most(
                        wall_subject(wall, &path_density),
                        Quantity::new(QuantityKind::Mass, wall.density_kg_m3),
                        Quantity::new(QuantityKind::Mass, (wall.density_kg_m3 * 0.9).max(600.0)),
                        loc("Reduce masonry density (self-weight in N_Ed).", "Rohdichte senken (Eigengewicht in N_Ed)."),
                    ));
            }
            out.push(b.build());

            // Declared member eccentricities (excl. slab-bearing offset handled in Φ) ≤ 0.2·t (§6.1.2.2).
            {
                let e_lim = 0.2 * t;
                let e_bot_abs = wall.eccentricity_bottom_m.abs();
                let e_top_decl = wall.eccentricity_top_m.abs();
                let e_slab = eccentricity_from_slab_bearing_m(wall);
                let e_gov = e_bot_abs.max(e_top_decl);
                let mut be = CheckResult::assess(
                    format!("en1996.6.1.2.eccentricity.{}.{}", wall.id, lc.id),
                    "EN 1996-1-1",
                    clause("EN 1996-1-1", "§6.1.2.2", "6.1.2.2"),
                    wall_subject(wall, &wall_path(wall, "eccentricityBottomM")),
                    loc("Declared eccentricity e ≤ 0.2 t", "Angegebene Exzentrizität e ≤ 0,2 t"),
                )
                .utilization(Quantity::length_m(e_gov), Quantity::length_m(e_lim.max(1e-9)))
                .annex(annex)
                .explanation(loc(
                    &format!("e_top,decl={e_top_decl:.4} m; e_bot={e_bot_abs:.4} m; e_slab={e_slab:.4} m; limit 0.2t={e_lim:.4} m; φ∞={:.2}.", wall.phi_infinity),
                    &format!("e_oben,ang={e_top_decl:.4} m; e_unten={e_bot_abs:.4} m; e_Decke={e_slab:.4} m; Grenze 0,2t={e_lim:.4} m; φ∞={:.2}.", wall.phi_infinity),
                ));
                if e_gov > e_lim {
                    be = be.remedy(Remedy::at_most(
                        wall_subject(wall, &wall_path(wall, "eccentricityBottomM")),
                        Quantity::length_m(e_bot_abs),
                        Quantity::length_m(e_lim),
                        loc("Reduce bottom eccentricity.", "Exzentrizität unten reduzieren."),
                    ));
                }
                out.push(be.build());
            }
        }

        // Shear + sliding §6.2
        {
            let ge = governing_shear(&effects);
            let n_ed = ge.n_mid;
            let v_ed = ge.v;
            let sigma_d = n_ed / a.max(1e-9);
            let f_vk = f_vk_pa(annex, wall.mortar_class, wall.unit_material, sigma_d, wall.f_b_pa);
            let v_rd_masonry = f_vk / gamma_m * a;
            // Sliding: V_Rd,slide = μ · N_Ed / γ_M (friction) — DE NA §6.2; governing = min(masonry, sliding).
            let v_rd_slide = (wall.mu * n_ed / gamma_m.max(1.0)).max(1e-9);
            let v_rd = v_rd_masonry.min(v_rd_slide);
            let mut b = CheckResult::assess(
                format!("en1996.6.2.shear.{}.{}", wall.id, lc.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.2", "6.2"),
                wall_subject(wall, &path_t),
                loc("Shear / sliding V_Ed ≤ V_Rd", "Schub / Gleiten V_Ed ≤ V_Rd"),
            )
            .utilization(Quantity::new(QuantityKind::Force, v_ed), Quantity::new(QuantityKind::Force, v_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!(
                    "{}; V_Ed={v_ed:.0} N; V_Rd,masonry={v_rd_masonry:.0} N; V_Rd,slide=μ·N/γ_M={v_rd_slide:.0} N (μ={:.2}); f_vk={:.3} MPa.",
                    ge.combo_en, wall.mu, f_vk / 1e6
                ),
                &format!(
                    "{}; V_Ed={v_ed:.0} N; V_Rd,Mauerwerk={v_rd_masonry:.0} N; V_Rd,Gleiten=μ·N/γ_M={v_rd_slide:.0} N (μ={:.2}); f_vk={:.3} MPa.",
                    ge.combo_de, wall.mu, f_vk / 1e6
                ),
            ));
            if v_ed > v_rd {
                b = b
                    .remedy(Remedy::at_least(
                        wall_subject(wall, &path_t),
                        Quantity::length_m(t),
                        Quantity::length_m(t * (v_ed / v_rd.max(1e-12)).max(1.0)),
                        loc("Increase thickness for shear/sliding.", "Dicke für Schub/Gleiten erhöhen."),
                    ))
                    .remedy(Remedy::at_least(
                        wall_subject(wall, &path_mu),
                        Quantity::new(QuantityKind::Dimensionless, wall.mu),
                        Quantity::new(QuantityKind::Dimensionless, (wall.mu * 1.25).min(0.8)),
                        loc("Increase friction coefficient μ.", "Reibungsbeiwert μ erhöhen."),
                    ));
            }
            out.push(b.build());
        }

        // Lateral flexure §6.3 (+ horizontal reinforcement)
        {
            let ge = governing_lateral(&effects);
            let w_ed = ge.w_pa;
            if w_ed <= 0.0 && lc.h_k_earth_n <= 0.0 {
                out.push(
                    CheckResult::assess(
                        format!("en1996.6.3.flexure.{}.{}", wall.id, lc.id),
                        "EN 1996-1-1",
                        clause("EN 1996-1-1", "§6.3", "6.3"),
                        wall_subject(wall, &load_case_path(wall, lc, "qPWindPa")),
                        loc("Lateral flexure", "Biegung aus der Ebene"),
                    )
                    .not_applicable(loc("No wind/earth lateral action.", "Keine Wind-/Erddruck-Einwirkung."))
                    .annex(annex)
                    .build(),
                );
            } else {
                let m_ed = w_ed * wall.height_m.powi(2) / 8.0 * effective_length_m(wall); // N·m per wall length strip integrated
                let z = section_modulus_m3(wall);
                let f_xd = f_xk_pa(1, wall.mortar_class, wall.unit_material) / gamma_m;
                let z_s = 0.9 * wall.thickness_m;
                let as_h = wall.as_horizontal_m2.max(0.0);
                let f_yd = wall.f_yd_pa.max(0.0);
                // Bed-joint reinforcement §6.6 — As,h only counted with declared f_yd.
                let m_rd_reinf = if as_h > 0.0 && f_yd > 0.0 {
                    as_h * f_yd * z_s * effective_length_m(wall)
                } else {
                    0.0
                };
                let m_rd = f_xd * z + m_rd_reinf;
                let mut b = CheckResult::assess(
                    format!("en1996.6.3.flexure.{}.{}", wall.id, lc.id),
                    "EN 1996-1-1",
                    clause("EN 1996-1-1", "§6.3", "6.3"),
                    wall_subject(wall, &path_t),
                    loc("Lateral flexure M_Ed ≤ M_Rd", "Plattenbiegung M_Ed ≤ M_Rd"),
                )
                .utilization(Quantity::new(QuantityKind::Moment, m_ed), Quantity::new(QuantityKind::Moment, m_rd.max(1e-9)))
                .annex(annex)
                .explanation(loc(
                    &format!("{}; w_Ed={w_ed:.0} Pa; M_Ed={m_ed:.0} N·m; M_Rd={m_rd:.0} N·m (As,h={as_h:.6} m², f_yd={:.0} Pa, M_Rd,s={m_rd_reinf:.0}).", ge.combo_en, f_yd),
                    &format!("{}; w_Ed={w_ed:.0} Pa; M_Ed={m_ed:.0} N·m; M_Rd={m_rd:.0} N·m (As,h={as_h:.6} m², f_yd={:.0} Pa, M_Rd,s={m_rd_reinf:.0}) (Plattenbiegung).", ge.combo_de, f_yd),
                ));
                if as_h > 0.0 && f_yd <= 0.0 {
                    b = b
                        .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                        .remedy(Remedy::at_least(
                            wall_subject(wall, &wall_path(wall, "fYdPa")),
                            Quantity::new(QuantityKind::Stress, f_yd),
                            Quantity::new(QuantityKind::Stress, 435e6),
                            loc("Declare f_yd for bed-joint reinforcement.", "f_yd für Lagerfugenbewehrung angeben."),
                        ));
                } else if m_ed > m_rd {
                    b = b
                        .remedy(Remedy::at_least(
                            wall_subject(wall, &path_t),
                            Quantity::length_m(t),
                            Quantity::length_m(t * 1.2),
                            loc("Increase thickness for lateral flexure.", "Dicke für Plattenbiegung erhöhen."),
                        ))
                        .remedy(Remedy::at_least(
                            wall_subject(wall, &path_as_h),
                            Quantity::new(QuantityKind::Area, as_h),
                            Quantity::new(QuantityKind::Area, (as_h + 50e-6).max(50e-6)),
                            loc("Add bed-joint reinforcement As,h.", "Lagerfugenbewehrung As,h ergänzen."),
                        ));
                }
                out.push(b.build());
            }
        }

        // Concentrated §6.1.3
        for c in &lc.concentrated {
            let ge = governing_compression(&effects);
            let beta = concentrated_beta(c.bearing_length_m, wall.height_m, wall.length_m);
            let n_rdc = beta * f_d * c.bearing_area_m2.max(c.bearing_length_m * t);
            // Design concentrated = γ_Q * F_k (permanent portion already in G path separately)
            let f_ed = gamma_q(annex, false) * c.force_n;
            let mut b = CheckResult::assess(
                format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.1.3", "6.1.3"),
                wall_subject(wall, &concentrated_path(wall, lc, &c.id, "bearingLengthM")),
                loc("Concentrated load bearing", "Teilflächenpressung"),
            )
            .utilization(Quantity::new(QuantityKind::Force, f_ed), Quantity::new(QuantityKind::Force, n_rdc.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!("{}; β={beta:.2}; F_Ed={f_ed:.0} N; N_Rdc={n_rdc:.0} N.", ge.combo_en),
                &format!("{}; β={beta:.2}; F_Ed={f_ed:.0} N; N_Rdc={n_rdc:.0} N (Teilflächenpressung).", ge.combo_de),
            ));
            if f_ed > n_rdc {
                b = b.remedy(Remedy::at_least(
                    wall_subject(wall, &concentrated_path(wall, lc, &c.id, "bearingLengthM")),
                    Quantity::length_m(c.bearing_length_m),
                    Quantity::length_m(c.bearing_length_m * (f_ed / n_rdc.max(1e-12)).max(1.0)),
                    loc("Increase bearing length.", "Auflagerlänge erhöhen."),
                ));
            }
            out.push(b.build());
        }
    }

    // Fire 1-2 with α
    {
        let params = AnnexParams::for_document(annex, masonry_class, default_situation);
        let f_d = f_k / params.gamma_m();
        let n_rd_cold = phi_s(annex, lambda).max(phi_m(lambda, 0.05 * t, t)) * f_d * a;
        // Fire combination: G + ψ1,1·Q (simplified η_fi ≈ 0.7 for buildings) → N_Ed,fi
        let mut n_ed_fi: f64 = 0.0;
        for lc in &wall.load_cases {
            let g = lc.g_k_slab_n + self_weight_n(wall) + lc.concentrated.iter().map(|c| c.force_n).sum::<f64>();
            let q = lc.q_k_imposed_pa * lc.tributary_area_m2 + lc.q_k_snow_pa * lc.tributary_area_m2;
            n_ed_fi = n_ed_fi.max(g + 0.7 * q);
        }
        let alpha = n_ed_fi / n_rd_cold.max(1e-9);
        let t_min = fire_min_thickness_na(wall.fire_rei_min, wall.unit_material, wall.unit_group, wall.mortar_type, alpha);
        let mut b = CheckResult::assess(
            format!("en1996.1-2.fire.{}", wall.id),
            "EN 1996-1-2",
            clause("EN 1996-1-2", "§4", "4"),
            wall_subject(wall, &path_t),
            loc("Fire resistance tabulated (α)", "Feuerwiderstand tabellarisch (α)"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("REI {}; α=N_Ed,fi/N_Rd={alpha:.3}; t={t:.3} m; t_min(α)={t_min:.3} m.", wall.fire_rei_min),
            &format!("REI {}; α=N_Ed,fi/N_Rd={alpha:.3}; t={t:.3} m; t_min(α)={t_min:.3} m (tabellarisch).", wall.fire_rei_min),
        ));
        // Utilization: max(t_min/t, α) style — thickness utilization and α≤1
        let u_t = t_min / t.max(1e-9);
        let u = u_t.max(alpha);
        b = b.utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0));
        if u > 1.0 {
            b = b.remedy(Remedy::at_least(
                wall_subject(wall, &path_t),
                Quantity::length_m(t),
                Quantity::length_m(t_min.max(t * alpha)),
                loc("Increase thickness for fire / reduce α.", "Dicke für Brandschutz erhöhen / α reduzieren."),
            ));
        }
        out.push(b.build());
    }

    // Durability / bed joint
    {
        let need = required_mortar_mpa(wall.exposure, wall.unit_material);
        let have = wall.mortar_class.f_m_mpa();
        let mut b = CheckResult::assess(
            format!("en1996.2.durability.{}", wall.id),
            "EN 1996-2",
            clause("EN 1996-2", "Annex B", "B"),
            wall_subject(wall, &path_mortar),
            loc("Exposure / mortar durability", "Exposition / Mörtelbeständigkeit"),
        )
        .annex(annex);
        if !need.is_finite() {
            b = b
                .utilization(Quantity::new(QuantityKind::Dimensionless, 2.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .explanation(loc("Material/exposure combination inadmissible.", "Material/Exposition unzulässig."))
                .remedy(Remedy::one_of(
                    wall_subject(wall, &wall_path(wall, "exposure")),
                    vec!["Mx1".into(), "Mx2".into(), "Mx3".into()],
                    loc("Reduce exposure class.", "Expositionsklasse reduzieren."),
                ));
        } else {
            b = b
                .minimum(Quantity::new(QuantityKind::Stress, have * 1e6), Quantity::new(QuantityKind::Stress, need * 1e6))
                .explanation(loc(
                    &format!("Required mortar ≥ {need:.1} MPa for {:?}; have {have:.1} MPa.", wall.exposure),
                    &format!("Erforderlicher Mörtel ≥ {need:.1} MPa für {:?}; vorhanden {have:.1} MPa.", wall.exposure),
                ));
            if have + 1e-9 < need {
                b = b.remedy(Remedy::one_of(
                    wall_subject(wall, &path_mortar),
                    vec!["M10".into(), "M15".into(), "M20".into()],
                    loc("Upgrade mortar class.", "Mörtelklasse erhöhen."),
                ));
            }
        }
        out.push(b.build());

        let joint_ok = wall.bed_joint_thickness_m >= 0.006 && wall.bed_joint_thickness_m <= 0.015
            || (matches!(wall.mortar_type, MortarType::ThinLayer) && wall.bed_joint_thickness_m <= 0.003);
        let mut bj = CheckResult::assess(
            format!("en1996.2.bedjoint.{}", wall.id),
            "EN 1996-2",
            clause("EN 1996-2", "§8", "8"),
            wall_subject(wall, &wall_path(wall, "bedJointThicknessM")),
            loc("Bed-joint thickness", "Lagerfugendicke"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("t_bed={:.3} m; mortar={:?}.", wall.bed_joint_thickness_m, wall.mortar_type),
            &format!("t_Lager={:.3} m; Mörtel={:?}.", wall.bed_joint_thickness_m, wall.mortar_type),
        ));
        if joint_ok {
            bj = bj.utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0));
        } else {
            bj = bj
                .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .remedy(Remedy::at_most(
                    wall_subject(wall, &wall_path(wall, "bedJointThicknessM")),
                    Quantity::length_m(wall.bed_joint_thickness_m),
                    Quantity::length_m(0.015),
                    loc("Bring bed joint into 6–15 mm (GP) or ≤3 mm (thin).", "Lagerfuge auf 6–15 mm (NM) bzw. ≤3 mm (DBM) bringen."),
                ));
        }
        out.push(bj.build());
    }

    // Reinforced masonry §6.6 / §9 — always evaluate so As,v / As,h / f_yd / reinforced are live inputs
    {
        let as_min = 0.0005 * a;
        let as_v = wall.as_vertical_m2.max(0.0);
        let as_h = wall.as_horizontal_m2.max(0.0);
        let as_have = as_v + as_h;
        let mut b = CheckResult::assess(
            format!("en1996.6.6.reinforced.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§6.6", "6.6"),
            wall_subject(wall, &wall_path(wall, "asVerticalM2")),
            loc("Reinforced masonry As / f_yd", "Bewehrtes Mauerwerk As / f_yd"),
        )
        .annex(annex)
        .explanation(loc(
            &format!(
                "reinforced={}; As,v={as_v:.6} m²; As,h={as_h:.6} m²; f_yd={:.0} Pa; min As≈{as_min:.6} m².",
                wall.reinforced, wall.f_yd_pa
            ),
            &format!(
                "bewehrt={}; As,v={as_v:.6} m²; As,h={as_h:.6} m²; f_yd={:.0} Pa; min As≈{as_min:.6} m².",
                wall.reinforced, wall.f_yd_pa
            ),
        ));
        if !wall.reinforced {
            // Unreinforced: As and f_yd must stay at zero — any steel area without the flag fails.
            let stray = as_have + wall.f_yd_pa;
            if stray > 0.0 {
                b = b
                    .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                    .remedy(Remedy::exactly(
                        wall_subject(wall, &wall_path(wall, "reinforced")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        loc("Set reinforced=true when As or f_yd is provided.", "reinforced=true setzen, wenn As oder f_yd angegeben."),
                    ));
            } else {
                b = b.utilization(Quantity::new(QuantityKind::Dimensionless, 0.2), Quantity::new(QuantityKind::Dimensionless, 1.0));
            }
        } else if as_have < as_min || wall.f_yd_pa <= 0.0 {
            b = b
                .utilization(Quantity::new(QuantityKind::Area, as_have), Quantity::new(QuantityKind::Area, as_min.max(1e-9)))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_as_h),
                    Quantity::new(QuantityKind::Area, as_h),
                    Quantity::new(QuantityKind::Area, as_min),
                    loc("Provide minimum reinforcement.", "Mindestbewehrung vorsehen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &wall_path(wall, "fYdPa")),
                    Quantity::new(QuantityKind::Stress, wall.f_yd_pa),
                    Quantity::new(QuantityKind::Stress, 435e6),
                    loc("Declare reinforcement f_yd.", "Bewehrungs-f_yd angeben."),
                ));
        } else {
            b = b.utilization(Quantity::new(QuantityKind::Area, as_have), Quantity::new(QuantityKind::Area, as_min.max(1e-9)));
        }
        out.push(b.build());
    }

    // EN 1996-3 / NA simplified + basement
    {
        let part = "DIN EN 1996-3";
        let mut reasons_en = Vec::new();
        let mut reasons_de = Vec::new();
        if storeys > 3 {
            reasons_en.push(format!("storeys={storeys}>3"));
            reasons_de.push(format!("Geschosse={storeys}>3"));
        }
        if lambda > 27.0 {
            reasons_en.push(format!("λ={lambda:.1}>27"));
            reasons_de.push(format!("λ={lambda:.1}>27"));
        }
        if !matches!(wall.wall_type, WallType::LoadBearing) {
            reasons_en.push("wall not load-bearing".into());
            reasons_de.push("Wand nicht tragend".into());
        }
        // Table NA.A.1
        let q_k_max: f64 = wall.load_cases.iter().map(|lc| lc.q_k_imposed_pa).fold(0.0, f64::max);
        if q_k_max > 5_000.0 {
            reasons_en.push(format!("q_k={:.0} Pa > 5 kN/m²", q_k_max));
            reasons_de.push(format!("q_k={:.0} Pa > 5 kN/m²", q_k_max));
        }
        let span_max: f64 = wall.load_cases.iter().map(|lc| lc.slab_span_m).fold(0.0, f64::max);
        if span_max > 6.0 {
            reasons_en.push(format!("slab span={span_max:.2} m > 6 m"));
            reasons_de.push(format!("Deckenstützweite={span_max:.2} m > 6 m"));
        }
        if wall.thickness_m < 0.115 {
            reasons_en.push(format!("t={:.3} m < 115 mm", wall.thickness_m));
            reasons_de.push(format!("t={:.3} m < 115 mm", wall.thickness_m));
        }
        if wall.height_m > 3.0 && wall.thickness_m < 0.175 {
            reasons_en.push("height>3 m requires t≥175 mm (NA.A.1)".into());
            reasons_de.push("Höhe>3 m erfordert t≥175 mm (NA.A.1)".into());
        }
        let building_h = storeys as f64 * wall.height_m;
        if building_h > 12.0 {
            reasons_en.push(format!("approx. building height={building_h:.1} m > 12 m"));
            reasons_de.push(format!("Gebäudehöhe≈{building_h:.1} m > 12 m"));
        }

        if !reasons_en.is_empty() {
            out.push(
                CheckResult::assess(
                    format!("en1996.3.simplified.{}", wall.id),
                    part,
                    clause("EN 1996-3", "§4.2", "4.2"),
                    wall_subject(wall, &path_t),
                    loc("Simplified method applicability", "Anwendbarkeit vereinfachtes Verfahren"),
                )
                .not_applicable(loc(
                    &format!("Outside DIN EN 1996-3/NA Table NA.A.1: {}.", reasons_en.join("; ")),
                    &format!("Außerhalb DIN EN 1996-3/NA Tabelle NA.A.1: {}.", reasons_de.join("; ")),
                ))
                .annex(annex)
                .build(),
            );
        } else {
            let params = AnnexParams::for_document(annex, masonry_class, default_situation);
            let f_d = f_k / params.gamma_m();
            let phis = phi_s(annex, lambda);
            let n_rd = phis * f_d * a;
            let mut n_ed = 0.0_f64;
            let mut combo_en = String::new();
            let mut combo_de = String::new();
            for lc in &wall.load_cases {
                let eff = design_effects(annex, wall, lc, default_situation);
                let ge = governing_compression(&eff);
                if ge.n_bot > n_ed {
                    n_ed = ge.n_bot;
                    combo_en = ge.combo_en.clone();
                    combo_de = ge.combo_de.clone();
                }
            }
            let mut b = CheckResult::assess(
                format!("en1996.3.simplified.{}", wall.id),
                part,
                clause("EN 1996-3", "§4.2", "4.2"),
                wall_subject(wall, &path_t),
                loc("Simplified compression N_Rd", "Vereinfachte Drucktragfähigkeit N_Rd"),
            )
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!("{combo_en}; Φ_s={phis:.3}; N_Rd={n_rd:.0} N; N_Ed={n_ed:.0} N."),
                &format!("{combo_de}; Φ_s={phis:.3}; N_Rd={n_rd:.0} N; N_Ed={n_ed:.0} N (vereinfacht)."),
            ));
            if n_ed > n_rd {
                b = b.remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (n_ed / n_rd.max(1e-12)).max(1.0)),
                    loc("Increase thickness (simplified N_Rd).", "Dicke erhöhen (vereinfachtes N_Rd)."),
                ));
            }
            out.push(b.build());
        }

        // Basement earth-pressure §4.5 / DE NA
        if wall.is_basement {
            let mut n_min = f64::INFINITY;
            let mut n_max = 0.0_f64;
            let mut h_ed = 0.0_f64;
            for lc in &wall.load_cases {
                let eff = design_effects(annex, wall, lc, default_situation);
                for e in &eff {
                    n_min = n_min.min(e.n_bot);
                    n_max = n_max.max(e.n_bot);
                    h_ed = h_ed.max(e.v);
                }
            }
            if !n_min.is_finite() { n_min = 0.0; }
            // DE NA simplified earth-pressure: N_Ed,min ≥ 0.2·H_Ed·h/t  and N_Ed,max ≤ N_Rd,s
            let n_min_req = 0.2 * h_ed * wall.height_m / t.max(1e-6);
            let params = AnnexParams::for_document(annex, masonry_class, default_situation);
            let n_rd_s = phi_s(annex, lambda) * (f_k / params.gamma_m()) * a;
            let u1 = if n_min_req > 0.0 { n_min_req / n_min.max(1e-9) } else { 0.0 };
            let u2 = n_max / n_rd_s.max(1e-9);
            let u = u1.max(u2);
            let mut b = CheckResult::assess(
                format!("en1996.3.basement.{}", wall.id),
                part,
                clause("EN 1996-3", "§4.5", "4.5"),
                wall_subject(wall, &path_basement),
                loc("Basement wall earth pressure", "Kellerwand Erddruck"),
            )
            .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("N_Ed,min={n_min:.0} N (≥ {n_min_req:.0}); N_Ed,max={n_max:.0} N (≤ {n_rd_s:.0})."),
                &format!("N_Ed,min={n_min:.0} N (≥ {n_min_req:.0}); N_Ed,max={n_max:.0} N (≤ {n_rd_s:.0}) (Kellerwand §4.5)."),
            ));
            if u > 1.0 {
                b = b.remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * 1.2),
                    loc("Thicken basement wall / increase permanent load.", "Kellerwand verdicken / ständige Last erhöhen."),
                ));
            }
            out.push(b.build());
        } else if wall.load_cases.iter().any(|lc| lc.h_k_earth_n > 0.0) {
            out.push(
                CheckResult::assess(
                    format!("en1996.3.basement.{}", wall.id),
                    part,
                    clause("EN 1996-3", "§4.5", "4.5"),
                    wall_subject(wall, &path_basement),
                    loc("Basement wall earth pressure", "Kellerwand Erddruck"),
                )
                .not_applicable(loc(
                    "Earth pressure given but isBasement=false — mark wall as basement or clear h_k_earth.",
                    "Erddruck angegeben, aber isBasement=false — als Kellerwand kennzeichnen oder h_k_earth löschen.",
                ))
                .annex(annex)
                .build(),
            );
        }
    }

    out
}
//#endregion
