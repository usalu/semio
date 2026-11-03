//! 🧮️ Pure per-entity EN 1993 clause functions in mm, N, MPa (schema-neutral, one entity in, numbers out). See [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

pub use semio_s_artifact_norm_en1993_clauses_core::{classification, effective_width, resistance, stability, mcr_fe, warping_torsion, global, joints, fatigue_fire, misc, toughness};
pub use semio_s_artifact_norm_en1993_clauses_special::{plates_shells, cold_stainless, parts_2_6, cold_formed, stainless, plated_ext, shell_stability, tension_components, joint_components, bridge_fatigue, towers_masts, containment, sheet_piling, crane_runway, weld_group, parametric_sections, hollow_joints, bolt_group_ic, stress_range, effective_length, fasteners};

/// 🌀 End conditions of a `warpingSupport` value: fork, both ends restrained, restrained at the start with a fork end, or cantilever (start restrained, free end).
pub fn ends_of(support: semio_s_artifact_norm_en1993_model::WarpingSupport) -> (warping_torsion::End, warping_torsion::End) {
    use warping_torsion::End;
    match support {
        semio_s_artifact_norm_en1993_model::WarpingSupport::Fixed => (End::Fixed, End::Fixed),
        semio_s_artifact_norm_en1993_model::WarpingSupport::FixedFork => (End::Fixed, End::Fork),
        semio_s_artifact_norm_en1993_model::WarpingSupport::Cantilever => (End::Fixed, End::Free),
        semio_s_artifact_norm_en1993_model::WarpingSupport::Fork => (End::Fork, End::Fork),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧮️w3-norm/🦀️.rs"]
mod w3_norm;
