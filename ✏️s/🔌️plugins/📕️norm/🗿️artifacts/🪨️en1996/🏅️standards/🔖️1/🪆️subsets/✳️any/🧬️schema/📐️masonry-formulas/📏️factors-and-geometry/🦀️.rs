//! 📏️ Factors and geometry of the EN 1996 masonry checks: the annex parameters, the characteristic strengths and the slenderness, eccentricity and reduction factors. See [`super`].

use super::*;

//#region ⚖️AnnexParams
#[derive(Clone, Copy, Debug)]
pub struct AnnexParams {
    pub annex: AnnexChoice,
    pub masonry_class: MasonryClass,
    pub reduced: bool,
}

impl AnnexParams {
    pub fn for_document(annex: AnnexChoice, masonry_class: MasonryClass, design_situation: DesignSituation) -> Self {
        Self {
            annex,
            masonry_class,
            reduced: match design_situation {
                DesignSituation::Persistent => false,
                DesignSituation::Transient | DesignSituation::Accidental | DesignSituation::Seismic => true,
            },
        }
    }

    pub fn gamma_m(self) -> f64 {
        match self.annex {
            AnnexChoice::En => self.masonry_class.gamma_m_en(),
            AnnexChoice::De => self.masonry_class.gamma_m_de(self.reduced),
        }
    }

    /// 🔖️ Registry id of the γ_M row [`Self::gamma_m`] reads, so a check cites exactly the factor it used.
    pub fn gamma_m_ndp(self) -> &'static str {
        match self.annex {
            AnnexChoice::En => self.masonry_class.gamma_m_id(),
            AnnexChoice::De if self.reduced => "ndp.en1996.gamma-m.accidental",
            AnnexChoice::De => self.masonry_class.gamma_m_id(),
        }
    }
}
//#endregion

//#region 📐Strength
/// 📐️ Normalised unit strength f_b [Pa] of the wall: declared strength times the shape factor of EN 772-1.
pub fn f_b_normalised_pa(wall: &MasonryWall) -> f64 {
    wall.f_b_pa.max(0.0) * shape_factor_delta(wall.unit_height_m, wall.unit_width_m.min(wall.unit_length_m))
}

/// 🧈 Mortar strength f_m [MPa] of the wall: the declared value, else the class value.
pub fn f_m_mpa(wall: &MasonryWall) -> f64 {
    if wall.mortar_strength_pa > 0.0 { wall.mortar_strength_pa / 1e6 } else { wall.mortar_class.f_m_mpa() }
}

pub fn f_k_pa(annex: AnnexChoice, wall: &MasonryWall) -> f64 {
    f_k_mpa_from(annex, wall.unit_material, wall.unit_group, wall.mortar_type, f_b_normalised_pa(wall) / 1e6, f_m_mpa(wall)) * 1e6
}

/// ✂️ Characteristic shear strength f_vk [Pa] = min(f_vk0 + 0.4 σ_d, f_vlt).
pub fn f_vk_pa(annex: AnnexChoice, wall: &MasonryWall, sigma_d_pa: f64) -> f64 {
    let sigma = sigma_d_pa.max(0.0);
    let raw = f_vk0_mpa(annex, wall.unit_material, wall.mortar_type, wall.mortar_class) * 1e6 + 0.4 * sigma;
    raw.min(f_vlt_mpa(annex, wall.unit_group, f_b_normalised_pa(wall) / 1e6, sigma / 1e6) * 1e6)
}

/// 🧱️ Characteristic flexural strength f_xk1 (direction 1) or f_xk2 [Pa].
pub fn f_xk_pa(direction: u8, wall: &MasonryWall) -> f64 {
    f_xk_mpa(direction, f_m_mpa(wall), wall.unit_material) * 1e6
}
//#endregion

//#region 📐Geometry
pub fn t_ef_m(wall: &MasonryWall) -> f64 {
    wall.thickness_m
}

/// 🪟 Net loaded length ℓ_ef: the wall length less the width of every opening, since the piers carry the vertical load.
pub fn effective_length_m(wall: &MasonryWall) -> f64 {
    (wall.length_m - wall.openings.iter().map(|o| o.width_m).sum::<f64>()).max(1e-3)
}

/// 🧱️ Restraint factor ρ₂ of two-side held walls: 0.75 when the slab bears at least 2t/3, else 1.0.
pub fn rho_2(wall: &MasonryWall) -> f64 {
    if wall.slab_bearing_depth_m >= 2.0 * wall.thickness_m / 3.0 { 0.75 } else { 1.0 }
}

/// 📐️ Restraint factor ρ_n for two, three or four held sides (EN 1996-1-1 5.5.1.2).
pub fn rho_n(support_sides: u8, rho2: f64, h_m: f64, l_m: f64) -> f64 {
    let h = h_m.max(1e-6);
    let l = l_m.max(1e-6);
    match support_sides {
        0 | 1 => 2.0,
        2 => rho2,
        3 if h > 3.5 * l => (1.5 * l / h).max(0.3),
        3 => (rho2 / (1.0 + (rho2 * h / (3.0 * l)).powi(2))).max(0.3),
        _ if h > 1.15 * l => 0.5 * l / h,
        _ => rho2 / (1.0 + (rho2 * h / l).powi(2)),
    }
}

pub fn h_ef_m(wall: &MasonryWall) -> f64 {
    rho_n(wall.support_sides, rho_2(wall), wall.height_m, wall.length_m) * wall.height_m
}

pub fn slenderness(wall: &MasonryWall) -> f64 {
    h_ef_m(wall) / t_ef_m(wall).max(1e-6)
}

/// 🪟 Net loaded area A = ℓ_ef·t.
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

/// 🔄 NA-DE slab-end rotation eccentricity e_θ = (N_floor/N_Ed)·ℓ_f/25 with ℓ_f capped at 6 m and e_θ at 0.05 t (NA-DE Annex C).
pub fn eccentricity_from_slab_rotation_m(annex: AnnexChoice, wall: &MasonryWall, lc: &WallLoadCase, n_ed: f64) -> f64 {
    if annex != AnnexChoice::De {
        return 0.0;
    }
    let l_f = lc.slab_span_m.clamp(0.0, 6.0);
    let e_full = l_f / 25.0;
    let n_floor = (lc.g_k_slab_n + lc.q_k_imposed_pa * lc.tributary_area_m2 + lc.q_k_snow_pa * lc.tributary_area_m2).max(0.0);
    let share = if n_ed.abs() > 1e-6 { (n_floor / n_ed.abs()).clamp(0.0, 1.0) } else { 1.0 };
    (e_full * share).min(0.05 * wall.thickness_m.max(1e-6))
}

/// 📉 NA-DE slab-span reduction Φ₂ = min(0.9, 1.6 − ℓ_f/6) for ℓ_f > 4.2 m, else 1.0 (NA-DE).
pub fn phi_2_slab_span(annex: AnnexChoice, slab_span_m: f64) -> f64 {
    if annex != AnnexChoice::De {
        return 1.0;
    }
    let l_f = slab_span_m.clamp(0.0, 6.0);
    if l_f <= 4.2 { 1.0 } else { (1.6 - l_f / 6.0).clamp(0.0, 0.9) }
}
//#endregion

//#region 📉Phi
/// 📉 Reduction factor Φ_i = 1 − 2e/t at the top or bottom with e ≥ 0.05 t.
pub fn phi_i(eccentricity_m: f64, t_m: f64) -> f64 {
    let t = t_m.max(1e-6);
    let e = eccentricity_m.abs().max(0.05 * t);
    (1.0 - 2.0 * e / t).clamp(0.0, 1.0)
}

/// 📉 The terms of the mid-height factor of Annex G as `(λ, A1, u, Φ_m)` with λ = (h_ef/t_ef)/√K_E, A1 = 1 − 2 e_mk/t and u = (λ − 0.063)/(0.73 − 1.17 e_mk/t); Φ_m = A1·exp(−u²/2), zero when A1 ≤ 0 (λ and u are then still reported).
pub fn phi_m_terms(slenderness: f64, e_mk_m: f64, t_m: f64, k_e: f64) -> (f64, f64, f64, f64) {
    let t = t_m.max(1e-6);
    let e_mk = e_mk_m.abs().max(0.05 * t);
    let a1 = 1.0 - 2.0 * e_mk / t;
    let lambda = slenderness / k_e.max(1e-9).sqrt();
    let u = (lambda - 0.063) / (0.73 - 1.17 * e_mk / t);
    (lambda, a1, u, if a1 <= 0.0 { 0.0 } else { a1 * (-0.5 * u * u).exp() })
}

/// 📉 Mid-height factor Φ_m = A1·exp(−u²/2) of Annex G with λ = (h_ef/t_ef)/√K_E.
pub fn phi_m(slenderness: f64, e_mk_m: f64, t_m: f64, k_e: f64) -> f64 {
    phi_m_terms(slenderness, e_mk_m, t_m, k_e).3
}

/// 📉 Simplified Φ_s = 0.85 − 0.0011 (h_ef/t)² of EN 1996-3.
pub fn phi_s(lambda: f64) -> f64 {
    (0.85 - 0.0011 * lambda * lambda).max(0.0)
}

/// 📉 Creep eccentricity e_k = 0.002·φ∞·(h_ef/t_ef)·√(t·e_m) (§6.1.2.2).
pub fn creep_eccentricity_m(wall: &MasonryWall, e_m: f64) -> f64 {
    let t = wall.thickness_m.max(1e-6);
    0.002 * wall.phi_infinity.max(0.0) * (h_ef_m(wall) / t) * (t * e_m.abs().max(1e-9)).sqrt()
}

/// 🏋️ Concentrated-load factor β = (1 + 0.3 a1/hc)(1.5 − 1.1 Ab/Aef), at most min(1.25 + a1/(2 hc), 1.5) and at least 1.
pub fn concentrated_beta(a1_m: f64, hc_m: f64, ab_m2: f64, a_ef_m2: f64) -> f64 {
    let hc = hc_m.max(1e-6);
    if ab_m2 > 0.45 * a_ef_m2 {
        return 1.0;
    }
    let beta = (1.0 + 0.3 * a1_m / hc) * (1.5 - 1.1 * ab_m2 / a_ef_m2.max(1e-9));
    beta.min((1.25 + a1_m / (2.0 * hc)).min(1.5)).max(1.0)
}

/// 🔧 Smallest multiple of `step` that is at least `x`, so a remedy lands on the passing side of its limit.
pub fn ceil_to(x: f64, step: f64) -> f64 {
    (x / step - 1e-9).ceil() * step
}
/// 🔧 Largest multiple of `step` that is at most `x`, so a remedy lands on the passing side of its limit.
pub fn floor_to(x: f64, step: f64) -> f64 {
    (x / step + 1e-9).floor() * step
}

/// 🔧 Smallest (or largest, when `bound < x0`) value between `x0` and `bound` for which `passes` holds, rounded to `step` on the passing side; `None` when even `bound` fails.
pub fn solve_threshold(x0: f64, bound: f64, step: f64, passes: impl Fn(f64) -> bool) -> Option<f64> {
    if passes(x0) {
        return Some(x0);
    }
    if !passes(bound) {
        return None;
    }
    let (mut bad, mut good) = (x0, bound);
    for _ in 0..60 {
        let mid = 0.5 * (bad + good);
        if passes(mid) { good = mid } else { bad = mid }
    }
    let rounded = if good >= x0 { ceil_to(good, step) } else { -ceil_to(-good, step) };
    Some(if passes(rounded) { rounded } else { good })
}
//#endregion
