//! 🔄️ EN 1999-1-3 fatigue: the S–N math of `fatigue_math` (re-exported) and the grouping of fatigue details into spectra. See [`super`].

use crate::*;
pub use semio_s_artifact_norm_en1999_fatigue_math::*;

pub fn gamma_mf(annex: AnnexChoice) -> f64 { na_de::AnnexParams::for_choice(annex).gamma_mf }

/// 🗂️ The spectra of a document: fatigue details of one member and one detail category form one spectrum (each row is one block `(Δσ_Ed, n)`); rows without a member stay alone. Index groups in first-seen order, the first row names the spectrum.
pub fn spectrum_groups(details: &[FatigueDetail]) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (i, d) in details.iter().enumerate() {
        let same = |j: usize| !d.member_id.is_empty() && details[j].member_id == d.member_id && details[j].detail_category == d.detail_category;
        match groups.iter_mut().find(|g| same(g[0])) {
            Some(g) => g.push(i),
            None => groups.push(vec![i]),
        }
    }
    groups
}

/// 🧮 `(Δσ_C, m1, m2)` of a spectrum from its first row (the S–N curve is the one of the first block; `m2` defaults to `m1 + 2`).
pub fn spectrum_curve(first: &FatigueDetail) -> Option<(f64, f64, f64)> {
    let dc = detail_category_delta_sigma_c(&first.detail_category)?;
    (first.m1 > 0.0).then(|| (dc, first.m1, if first.m2 > 0.0 { first.m2 } else { first.m1 + 2.0 }))
}

/// 🎯 The stress blocks of a fatigue category: the `fatigueDetails` rows of the same member and detail category.
pub fn category_blocks<'a>(category: &FatigueCategory, details: &'a [FatigueDetail]) -> Vec<&'a FatigueDetail> {
    details.iter().filter(|d| d.member_id == category.member_id && d.detail_category == category.detail_category).collect()
}

/// 🧮 `(Δσ_C, m1, m2)` of a category: Δσ_C from Annex J by the detail category, the slopes from the category row when positive, else from its first block (`m2` defaults to `m1 + 2`).
pub fn category_curve(category: &FatigueCategory, blocks: &[&FatigueDetail]) -> Option<(f64, f64, f64)> {
    let dc = detail_category_delta_sigma_c(&category.detail_category)?;
    let first = blocks.first();
    let m1 = if category.m1 > 0.0 { category.m1 } else { first.map_or(0.0, |b| b.m1) };
    let m2 = if category.m2 > 0.0 { category.m2 } else if category.m1 <= 0.0 && first.is_some_and(|b| b.m2 > 0.0) { first.map_or(0.0, |b| b.m2) } else { m1 + 2.0 };
    (m1 > 0.0).then_some((dc, m1, m2))
}

/// 📊 Palmgren–Miner damage of one category over its blocks.
#[derive(Clone, Debug, PartialEq)]
pub struct CategoryDamage {
    pub delta_sigma_c: f64,
    pub m1: f64,
    pub m2: f64,
    pub total: f64,
    pub shares: Vec<f64>,
    pub governing: usize,
}

/// 📊 D = Σ n/N of a category (`None` while the category is unknown or no slope is positive); γ_Ff and γ_Mf are the annex values.
pub fn category_damage(category: &FatigueCategory, details: &[FatigueDetail], gamma_ff: f64, gamma_mf: f64) -> Option<CategoryDamage> {
    let blocks = category_blocks(category, details);
    let (dc, m1, m2) = category_curve(category, &blocks)?;
    let shares: Vec<f64> = blocks.iter().map(|d| damage(d.delta_sigma_ed, d.n_cycles, dc, m1, m2, gamma_ff, gamma_mf)).collect();
    let governing = shares.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).map_or(0, |(i, _)| i);
    Some(CategoryDamage { delta_sigma_c: dc, m1, m2, total: shares.iter().sum(), shares, governing })
}

/// 🗂️ Whether a spectrum group (index list into `details`) belongs to a category row, so the category check replaces the plain spectrum check.
pub fn group_has_category(group: &[usize], details: &[FatigueDetail], categories: &[FatigueCategory]) -> bool {
    group.first().is_some_and(|i| categories.iter().any(|c| c.member_id == details[*i].member_id && c.detail_category == details[*i].detail_category))
}
