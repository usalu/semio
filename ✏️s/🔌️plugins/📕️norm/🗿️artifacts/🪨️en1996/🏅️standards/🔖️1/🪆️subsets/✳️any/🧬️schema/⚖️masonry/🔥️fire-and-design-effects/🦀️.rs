//! 🔥️ Fire resistance and design effects of the EN 1996 masonry checks: the minimum fire thickness, the mortar strength, the combination factors and the governing design effects. See [`super`].

use super::*;

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
pub(super) fn situation_from_str(s: &str) -> DesignSituation {
    match s.trim().to_ascii_lowercase().as_str() {
        "accidental" | "bsa" => DesignSituation::Accidental,
        "seismic" => DesignSituation::Seismic,
        "transient" | "bst" => DesignSituation::Transient,
        _ => DesignSituation::Persistent,
    }
}

/// ⚖️ Combination factor ψ₀ for imposed floor categories (EN 1990 Table A1.1 / DE NA).
pub fn psi0_imposed(category: &str) -> f64 {
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
    pub(super) n_top: f64,
    pub(super) n_mid: f64,
    pub(super) n_bot: f64,
    pub(super) v: f64,
    pub(super) w_pa: f64,
    pub(super) combo_en: String,
    pub(super) combo_de: String,
}

fn gamma_g_sup(annex: AnnexChoice) -> f64 {
    match annex {
        AnnexChoice::De | AnnexChoice::En => 1.35,
    }
}

fn gamma_g_inf() -> f64 { 1.0 }

pub(super) fn gamma_q(annex: AnnexChoice, accidental: bool) -> f64 {
    if accidental {
        1.0
    } else {
        match annex {
            AnnexChoice::De | AnnexChoice::En => 1.5,
        }
    }
}

/// ⚖️ Form EN 1990 (+ DE NA) ULS design effects from characteristic actions + self-weight.
pub(super) fn design_effects(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, default_sit: DesignSituation) -> Vec<DesignEffects> {
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

pub(super) fn governing_compression(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| a.n_bot.partial_cmp(&b.n_bot).unwrap()).unwrap_or(&effects[0])
}

pub(super) fn governing_shear(effects: &[DesignEffects]) -> &DesignEffects {
    // Critical: high V with low N (friction) — maximize V/N ratio roughly max V then min N
    effects.iter().max_by(|a, b| {
        let ka = a.v / a.n_mid.max(1.0);
        let kb = b.v / b.n_mid.max(1.0);
        ka.partial_cmp(&kb).unwrap()
    }).unwrap_or(&effects[0])
}

pub(super) fn governing_lateral(effects: &[DesignEffects]) -> &DesignEffects {
    effects.iter().max_by(|a, b| a.w_pa.partial_cmp(&b.w_pa).unwrap()).unwrap_or(&effects[0])
}
//#endregion
