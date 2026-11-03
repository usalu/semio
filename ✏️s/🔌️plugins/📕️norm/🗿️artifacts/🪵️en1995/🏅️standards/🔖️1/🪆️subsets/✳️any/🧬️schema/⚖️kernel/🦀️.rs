//! ⚖️ EN 1995 verification kernel: strength tables, factor tables and the catalogue T-nn of pure clause formulas with their registry. Units: N, mm, N/mm². See [`semio_s_artifact_norm_en1995_schema`].

use semio_s_artifact_norm_en1995_model::{TimberProduct, TimberProperties};
pub use semio_s_plugin_norm_mechanics::BeamSupports;
use semio_s_plugin_norm_compliance::{ndp_common, ndp_number, ndp_number_opt, AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

/// ⏱ Load-duration class — Instantaneous is shortest (EN 1995-1-1 3.1.3 (2)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadDuration {
    Permanent,
    Long,
    Medium,
    Short,
    Instantaneous,
}

/// 🔁 The kernel duration class of the persisted load-duration class.
pub fn load_duration_of(class: semio_s_artifact_norm_en1995_model::LoadDurationClass) -> LoadDuration {
    match class {
        semio_s_artifact_norm_en1995_model::LoadDurationClass::Permanent => LoadDuration::Permanent,
        semio_s_artifact_norm_en1995_model::LoadDurationClass::Long => LoadDuration::Long,
        semio_s_artifact_norm_en1995_model::LoadDurationClass::Medium => LoadDuration::Medium,
        semio_s_artifact_norm_en1995_model::LoadDurationClass::Short => LoadDuration::Short,
        semio_s_artifact_norm_en1995_model::LoadDurationClass::Instantaneous => LoadDuration::Instantaneous,
    }
}

/// 🔤 Duration class from its spelling (catalogue inputs); unknown spellings read as medium.
pub fn parse_load_duration(s: &str) -> LoadDuration {
    match s.trim().to_ascii_lowercase().as_str() {
        "permanent" | "perm" => LoadDuration::Permanent,
        "long" => LoadDuration::Long,
        "short" => LoadDuration::Short,
        "instantaneous" | "inst" => LoadDuration::Instantaneous,
        _ => LoadDuration::Medium,
    }
}

/// ✂️ k_cr of EN 1995-1-1 6.1.7 (2): 1.0 for LVL/CLT, 0.67 (EN) or min(1, 2.5/f_v,k) (NA-DE) for solid timber and glulam; `f_v_k_pa` in Pa.
pub fn k_cr_value(annex: AnnexChoice, product: TimberProduct, f_v_k_pa: f64) -> f64 {
    match (annex, product) {
        (_, TimberProduct::Lvl | TimberProduct::Clt) => 1.0,
        (AnnexChoice::En, _) => ndp_number("ndp.en1995.k-cr.en-constant", annex),
        (AnnexChoice::De, _) => {
            let f = f_v_k_pa / 1e6;
            if f <= 0.0 { 1.0 } else { (ndp_number("ndp.en1995.k-cr.de-numerator", annex) / f).min(1.0) }
        }
    }
}

#[path = "📚️strength-tables/🦀️.rs"]
mod strength_tables;
pub use strength_tables::*;

#[path = "📇️catalogue/🦀️.rs"]
mod catalogue;
pub use catalogue::*;

#[path = "🧱️cross-sections/🦀️.rs"]
mod cross_sections;

#[path = "📐️stability/🦀️.rs"]
mod stability;

#[path = "🪚️varying-sections/🦀️.rs"]
mod varying_sections;

#[path = "🛋️serviceability/🦀️.rs"]
mod serviceability;

#[path = "🔩️connections/🦀️.rs"]
mod connections;

#[path = "🧩️components/🦀️.rs"]
mod components;

#[path = "🔥️fire/🦀️.rs"]
mod fire;

#[path = "🌉️bridges/🦀️.rs"]
mod bridges;

pub use components::{bracing_qd, clt_gamma, gamma_beam, is_mineral_part, joint_profile, ultimate_slip_modulus, FibreStress, JointProfile, racking_factor, wall_panel_racking, GammaOut, Part};
pub use connections::{case_modes, embedment_fh, embedment_probe, EmbedmentProbe, kser_value, Modes, modes_steel_timber, modes_tt_double, modes_tt_single, n_ef_value, rope_share, rope_share_of, screw_withdrawal, screw_withdrawal_n, spacing_table, splitting_f90_rk, yield_moment};
pub use serviceability::{creep_deflection, deflection_case, deflection_probe, DeflectionProbe, creep_factor, deflection_tapered_udl, floor_strip_deflection, floor_f1, floor_plate_f1, floor_plate_mode, floor_plate_peak_acceleration, unit_point_deflection, velocity_response};
pub use stability::{column_probe, kc_factor, kcrit_factor, lef_ltb, ltb_probe, BucklingAxis, ColumnProbe, LtbProbe, sigma_m_crit_exact, sigma_m_crit_simplified};
pub use bridges::{bridge_f1, crowd_acceleration, crowd_psi, miner_damage, single_pedestrian_acceleration};
pub use cross_sections::{bearing_length_ef, k_c90_factor, k_cr_factor, HOLE_LIMIT_RATIO};
pub use varying_sections::{apex_kl, apex_kp, apex_kr, km_alpha, notch_ft90, notch_kv, moment_shape, moment_signed, notch_probe, taper_probe, MomentLine, NotchProbe, TaperProbe, TaperSection, NOTCH_NA_MIN_RATIO};
pub use fire::{protected_probe, ProtectedProbe, charring_rates, cladding_times, clt_char_depth, clt_residual_parts, clt_residual_stress, d_ef_mm, reduced_properties, Reduced};
