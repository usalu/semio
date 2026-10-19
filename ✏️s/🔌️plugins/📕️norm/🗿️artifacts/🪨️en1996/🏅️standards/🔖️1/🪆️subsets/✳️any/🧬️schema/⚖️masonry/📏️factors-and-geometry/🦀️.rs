//! 📏️ Factors and geometry of the EN 1996 masonry checks: the annex parameters, the characteristic strengths and the slenderness, eccentricity and reduction factors. See [`super`].

use super::*;

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
    let ratio = (h / 100.0).min(2.5).max(0.4);
    let width_factor = 1.0 - (-w / 250.0).exp();
    let length_factor = 1.0 - (-l / 400.0).exp();
    ((1.15 - 0.15 * ratio) * (0.90 + 0.20 * width_factor) * (0.92 + 0.16 * length_factor)).clamp(0.70, 1.40)
}

pub fn f_k_pa(annex: AnnexChoice, wall: &MasonryWall) -> f64 {
    let f = fk_factors(annex, wall.unit_material, wall.unit_group, wall.mortar_type);
    let delta = shape_factor_delta(wall.unit_height_m, wall.unit_width_m, wall.unit_length_m);
    let f_b = (wall.f_b_pa / 1e6 * delta).max(0.0);
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
/// ✂️ DE-NA upper-limit ratio f_vlt / f_b for bed-joint shear (DIN EN 1996-1-1/NA).
pub const F_VLT_OVER_FB_DE: f64 = 0.045;

pub fn f_vk_pa(annex: AnnexChoice, mortar: MortarClass, material: UnitMaterial, sigma_d_pa: f64, f_b_pa: f64) -> f64 {
    let f_vk0 = f_vk0_pa(mortar, material);
    let raw = f_vk0 + 0.4 * sigma_d_pa.max(0.0);
    match annex {
        AnnexChoice::En => raw,
        AnnexChoice::De => {
            let f_vlt = F_VLT_OVER_FB_DE * f_b_pa.max(0.0);
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
    let h = wall.height_m.max(1e-6);
    for o in &wall.openings {
        let pier = o.sill_height_m <= 0.05 * h && o.height_m >= 0.85 * h;
        if pier {
            l -= o.width_m;
        } else {
            let open_frac = (o.height_m / h).clamp(0.0, 1.0);
            let sill_frac = (o.sill_height_m / h).clamp(0.0, 1.0);
            l -= o.width_m * open_frac * (0.35 + 0.65 * (1.0 - sill_frac));
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
    (e_full * share).min(0.05 * wall.thickness_m.max(1e-6))
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
