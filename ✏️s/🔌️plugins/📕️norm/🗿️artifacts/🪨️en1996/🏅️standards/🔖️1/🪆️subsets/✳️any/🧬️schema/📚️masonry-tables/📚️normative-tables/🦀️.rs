//! 📚️ Normative tables of the EN 1996 masonry checks: unit shape factors, mortar and strength factors, partial and combination factors and fire rows, each with its NDP basis. See [`super`].

use super::*;
use semio_s_plugin_norm_compliance::{ndp_cell, ndp_cell_common, ndp_number};

const DELTA_H_MM: [f64; 6] = [50.0, 65.0, 100.0, 150.0, 200.0, 250.0];
const DELTA_W_MM: [f64; 5] = [50.0, 100.0, 150.0, 200.0, 250.0];

/// 📐️ Cell of the shape-factor table read from the registry (`ndp.en1996.delta.h<mm>.w<mm>`).
fn delta_cell(h: usize, w: usize) -> f64 {
    semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1996.delta.h{}.w{}", DELTA_H_MM[h], DELTA_W_MM[w]))
}

fn interval(grid: &[f64], x: f64) -> (usize, usize, f64) {
    let x = x.clamp(grid[0], grid[grid.len() - 1]);
    let upper = grid.iter().position(|v| x <= *v).unwrap_or(grid.len() - 1).max(1);
    let lower = upper - 1;
    (lower, upper, (x - grid[lower]) / (grid[upper] - grid[lower]))
}

/// 📐️ Shape factor δ of EN 772-1 for the unit height and the least horizontal dimension, bilinear in the table.
pub fn shape_factor_delta(unit_height_m: f64, least_horizontal_m: f64) -> f64 {
    let (h0, h1, th) = interval(&DELTA_H_MM, unit_height_m * 1000.0);
    let (w0, w1, tw) = interval(&DELTA_W_MM, least_horizontal_m * 1000.0);
    let low = delta_cell(h0, w0) + tw * (delta_cell(h0, w1) - delta_cell(h0, w0));
    let high = delta_cell(h1, w0) + tw * (delta_cell(h1, w1) - delta_cell(h1, w0));
    low + th * (high - low)
}

/// 🧈 Mortar kind used by the strength formulas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MortarKind {
    GeneralPurpose,
    ThinLayer,
    Lightweight,
}

impl From<MortarType> for MortarKind {
    fn from(m: MortarType) -> Self {
        match m {
            MortarType::GeneralPurpose => Self::GeneralPurpose,
            MortarType::ThinLayer => Self::ThinLayer,
            MortarType::Lightweight => Self::Lightweight,
        }
    }
}

/// 📐️ Factors of f_k = K f_b^α f_m^β.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FkFactors {
    pub k: f64,
    pub alpha: f64,
    pub beta: f64,
}

fn material_key(material: UnitMaterial) -> &'static str {
    match material {
        UnitMaterial::Clay => "clay",
        UnitMaterial::CalciumSilicate => "casi",
        UnitMaterial::Concrete => "concrete",
        UnitMaterial::Aerated => "aac",
    }
}

fn group_key(group: UnitGroup) -> &'static str {
    match group {
        UnitGroup::Group1 => "g1",
        UnitGroup::Group2 => "g2",
        UnitGroup::Group3 => "g3",
        UnitGroup::Group4 => "g4",
    }
}

fn mortar_key(mortar: MortarKind) -> &'static str {
    match mortar {
        MortarKind::GeneralPurpose => "gp",
        MortarKind::ThinLayer => "tl",
        MortarKind::Lightweight => "lw",
    }
}

/// 📐️ K, α and β of the registry tables `ndp.en1996.k.<mortar>.<material>.<group>`, `alpha.<mortar>`, `beta.<mortar>`.
pub fn fk_factors(annex: AnnexChoice, material: UnitMaterial, group: UnitGroup, mortar: MortarType) -> FkFactors {
    let mortar = mortar_key(mortar.into());
    let k = ndp_cell("ndp.en1996.k", &[mortar, material_key(material), group_key(group)], annex);
    FkFactors { k, alpha: ndp_cell("ndp.en1996.alpha", &[mortar], annex), beta: ndp_cell("ndp.en1996.beta", &[mortar], annex) }
}

/// 🧾 Registry ids (`ndp.en1996.*`) behind [`fk_factors`] for one input set.
pub fn fk_basis(annex: AnnexChoice, material: UnitMaterial, group: UnitGroup, mortar: MortarType) -> Vec<&'static str> {
    let mut out = vec!["ndp.en1996.m02.caps"];
    match annex {
        AnnexChoice::De => out.push("ndp.en1996.m02.k-de"),
        AnnexChoice::En => {
            if !matches!((material, group, mortar), (UnitMaterial::Clay | UnitMaterial::CalciumSilicate | UnitMaterial::Concrete | UnitMaterial::Aerated, UnitGroup::Group1 | UnitGroup::Group2, MortarType::GeneralPurpose)) {
                out.push("ndp.en1996.m02.k-en");
            }
        }
    }
    out
}

/// 🧱️ f_k [MPa] from the normalised unit strength and the mortar strength [MPa], with the EN caps.
pub fn f_k_mpa_from(annex: AnnexChoice, material: UnitMaterial, group: UnitGroup, mortar: MortarType, f_b_mpa: f64, f_m_mpa: f64) -> f64 {
    let f = fk_factors(annex, material, group, mortar);
    let f_b = f_b_mpa.max(0.0);
    let mut f_m = f_m_mpa.max(0.0).min(20.0);
    if !matches!(group, UnitGroup::Group1) {
        f_m = f_m.min(2.0 * f_b);
    }
    (f.k * f_b.powf(f.alpha) * f_m.powf(f.beta)).min(0.8 * f_b)
}

/// 🧱️ Characteristic f_vk0 [MPa] of Table 3.4 by annex, unit material, mortar type and class.
pub fn f_vk0_mpa(annex: AnnexChoice, material: UnitMaterial, mortar: MortarType, mortar_class: MortarClass) -> f64 {
    let band = match mortar {
        MortarType::GeneralPurpose => FVK0_BANDS.iter().find(|(_, lower)| mortar_class.f_m_mpa() >= *lower).map_or("lt2-5", |(key, _)| *key),
        _ => "all",
    };
    ndp_cell("ndp.en1996.fvk0", &[mortar_key(mortar.into()), material_key(material), band], annex)
}

const FVK0_BANDS: [(&str, f64); 4] = [("ge15", 15.0), ("ge10", 10.0), ("ge5", 5.0), ("ge2-5", 2.5)];

/// ✂️ Upper limit f_vlt [MPa] of f_vk: EN `ndp.en1996.fvlt.f-b-factor` f_b, NA-DE `ndp.en1996.fvlt.de-k` f_bt,cal sqrt(1 + σ/f_bt,cal) with f_bt,cal = `ndp.en1996.fvlt.de-share.<g1|g2|g34>` f_b for solid / perforated / hollow units (group 1 / 2 / 3-4).
pub fn f_vlt_mpa(annex: AnnexChoice, group: UnitGroup, f_b_mpa: f64, sigma_mpa: f64) -> f64 {
    match annex {
        AnnexChoice::En => ndp_number("ndp.en1996.fvlt.f-b-factor", annex) * f_b_mpa,
        AnnexChoice::De => {
            let share = ndp_cell("ndp.en1996.fvlt.de-share", &[match group {
                UnitGroup::Group1 => "g1",
                UnitGroup::Group2 => "g2",
                UnitGroup::Group3 | UnitGroup::Group4 => "g34",
            }], annex);
            let f_bt_cal = share * f_b_mpa;
            ndp_number("ndp.en1996.fvlt.de-k", annex) * f_bt_cal * (1.0 + sigma_mpa.max(0.0) / f_bt_cal.max(1e-12)).sqrt()
        }
    }
}

/// 🧱️ f_xk1 or f_xk2 [MPa] of Table 3.6 by unit material and mortar strength, read from `ndp.en1996.fxk.<x1|x2>.<material>.<strong|weak>`.
pub fn f_xk_mpa(direction: u8, f_m_mpa: f64, material: UnitMaterial) -> f64 {
    let strength = if f_m_mpa >= 5.0 { "strong" } else { "weak" };
    ndp_cell_common("ndp.en1996.fxk", &[if direction == 1 { "x1" } else { "x2" }, material_key(material), strength])
}

/// 🧱️ Modulus factor K_E: E = K_E f_k.
pub fn k_e(annex: AnnexChoice) -> f64 {
    ndp_number("ndp.en1996.k-e", annex)
}

/// ♾️ Final creep coefficient φ∞ (upper value of Table 3.2) by unit material.
pub fn phi_infinity_table(material: UnitMaterial) -> f64 {
    match material {
        UnitMaterial::Clay | UnitMaterial::Aerated => 1.5,
        UnitMaterial::CalciumSilicate | UnitMaterial::Concrete => 2.0,
    }
}

/// 🌡️ Shrinkage strain [mm/m] and thermal expansion [1/K] of Table 3.2 by unit material.
pub fn shrinkage_thermal(material: UnitMaterial) -> (f64, f64) {
    match material {
        UnitMaterial::Clay => (0.2, 6e-6),
        UnitMaterial::CalciumSilicate => (0.4, 10e-6),
        UnitMaterial::Aerated => (0.4, 8e-6),
        UnitMaterial::Concrete => (0.6, 10e-6),
    }
}

/// 🌍️ Behaviour factor range `(q_min, q_max, [min row, max row])` of a bracing system (`unreinforced`, `confined`, `reinforced`) from the unverified rows `ndp.en1996.m95.q.<system>.<min|max>` (EN 1998-1 Table 9.1), or `None` for an unknown system.
pub fn behaviour_factor_range(annex: AnnexChoice, system: &str) -> Option<(f64, f64, [&'static str; 2])> {
    let ids = match system {
        "unreinforced" => ["ndp.en1996.m95.q.unreinforced.min", "ndp.en1996.m95.q.unreinforced.max"],
        "confined" => ["ndp.en1996.m95.q.confined.min", "ndp.en1996.m95.q.confined.max"],
        "reinforced" => ["ndp.en1996.m95.q.reinforced.min", "ndp.en1996.m95.q.reinforced.max"],
        _ => return None,
    };
    Some((ndp_number(ids[0], annex), ndp_number(ids[1], annex), ids))
}

/// ⚖️ Partial factor γ_S of reinforcing and prestressing steel, the registry row `ndp.en1992.gamma-s` EN 1996-1-1 2.4.3 refers to.
pub fn gamma_s(annex: AnnexChoice) -> f64 {
    ndp_number("ndp.en1992.gamma-s", annex)
}

/// 🔩️ Design yield strength [Pa] of B500 reinforcement, f_yk/γ_S.
pub fn f_yd_b500_pa(annex: AnnexChoice) -> f64 {
    500e6 / gamma_s(annex)
}

/// ⚖️ Combination factors of the imposed category A to H: ψ0, ψ1, ψ2 of the registry rows `ndp.en1990.psi.*` (EN 1990 Table A1.1), `None` for an unknown category.
pub fn psi_imposed(annex: AnnexChoice, category: &str) -> Option<(f64, f64, f64)> {
    let key = category.trim().to_ascii_uppercase();
    matches!(key.as_str(), "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H").then(|| semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(annex, &key)).flatten().map(semio_s_plugin_norm_compliance::action_factors::PsiRow::as_tuple)
}

/// ⚖️ Combination factor ψ0 of the imposed category, 1.0 for an unknown category (integrity rejects it).
pub fn psi0_imposed(annex: AnnexChoice, category: &str) -> f64 {
    psi_imposed(annex, category).map_or(1.0, |p| p.0)
}

/// ⚖️ Combination factor ψ1 of the imposed category.
pub fn psi1_imposed(annex: AnnexChoice, category: &str) -> f64 {
    psi_imposed(annex, category).map_or(1.0, |p| p.1)
}

/// ⚖️ Combination factor ψ2 of the imposed category.
pub fn psi2_imposed(annex: AnnexChoice, category: &str) -> f64 {
    psi_imposed(annex, category).map_or(1.0, |p| p.2)
}

const FIRE_CLASSES: [u32; 6] = [30, 60, 90, 120, 180, 240];

/// 🔥 Minimum thickness [mm] of fire class `rei` for the unit material, read from the registry (`ndp.en1996.fire.<material>.rei<class>`).
fn fire_mm(material: UnitMaterial, rei: u32) -> f64 {
    let key = match material {
        UnitMaterial::Clay => "clay",
        UnitMaterial::CalciumSilicate => "casi",
        UnitMaterial::Aerated => "aac",
        UnitMaterial::Concrete => "concrete",
    };
    semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1996.fire.{key}.rei{rei}"))
}

/// 🔥 Tabulated minimum wall thickness [m] of the fire class for the unit material (EN 1996-1-2 Annex B base row; the load-ratio columns are not keyed).
pub fn fire_min_thickness_m(rei: u32, material: UnitMaterial) -> f64 {
    let idx = FIRE_CLASSES.iter().position(|c| rei <= *c).unwrap_or(FIRE_CLASSES.len() - 1);
    fire_mm(material, FIRE_CLASSES[idx]) / 1000.0
}

/// 🔥 Largest tabulated fire class whose minimum thickness the thickness meets, 0 when none.
pub fn fire_achievable_rei(t_m: f64, material: UnitMaterial) -> u32 {
    FIRE_CLASSES.iter().rev().copied().find(|c| fire_min_thickness_m(*c, material) <= t_m + 1e-12).unwrap_or(0)
}

/// 💧 Required minimum mortar strength [MPa] of the exposure class, infinite when the combination is inadmissible.
pub fn required_mortar_mpa(exposure: ExposureClass, material: UnitMaterial) -> f64 {
    match (exposure, material) {
        (ExposureClass::Mx1, _) => 1.0,
        (ExposureClass::Mx2, _) => 2.5,
        (ExposureClass::Mx3, _) => 5.0,
        (ExposureClass::Mx4, UnitMaterial::Aerated) => f64::INFINITY,
        (ExposureClass::Mx4, _) => 10.0,
        (ExposureClass::Mx5, _) => f64::INFINITY,
    }
}
