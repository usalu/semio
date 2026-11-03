//! 📏️ Numeric limits of the EN 1996 checks (slenderness, applicability of the simplified method, reinforcement ratios, overlap, eccentricity, concentrated-load area ratio), each a registry row `ndp.en1996.limit.*` (unverified). See [`super`].

use semio_s_plugin_norm_compliance::ndp_cell_common;

/// 📏️ Value of the registry row `ndp.en1996.limit.<keys…>`, whose EN and DE values agree.
pub fn normative_limit(keys: &[&str]) -> f64 {
    ndp_cell_common("ndp.en1996.limit", keys)
}

/// 📏️ Value of the registry row `ndp.en1996.limit.<keys…>` for the annex, for limits whose EN and DE values differ.
pub fn normative_limit_for(keys: &[&str], annex: semio_s_plugin_norm_compliance::AnnexChoice) -> f64 {
    semio_s_plugin_norm_compliance::ndp_cell("ndp.en1996.limit", keys, annex)
}
