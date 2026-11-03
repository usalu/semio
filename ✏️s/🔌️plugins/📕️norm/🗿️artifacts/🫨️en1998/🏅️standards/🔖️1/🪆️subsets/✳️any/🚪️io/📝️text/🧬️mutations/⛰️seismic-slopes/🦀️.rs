//! ⛰️ EN 1998 seismic slopes mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_circle_radius::ChangeSeismicSlopeCircleRadius;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_circle_centre_z::ChangeSeismicSlopeCircleCentreZ;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_circle_centre_x::ChangeSeismicSlopeCircleCentreX;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_topographic_factor::ChangeSeismicSlopeTopographicFactor;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_cohesion::ChangeSeismicSlopeCohesion;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_friction_angle::ChangeSeismicSlopeFrictionAngle;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_angle::ChangeSeismicSlopeAngle;
use semio_s_artifact_norm_en1998_mut_seismic_slopes::change_seismic_slope_height::ChangeSeismicSlopeHeight;

/// ✍️ Prints `change-seismic-slope-circle-radius` as an op-text line.
pub fn print_change_seismic_slope_circle_radius(p: &ChangeSeismicSlopeCircleRadius) -> String {
    format!("change-seismic-slope-circle-radius id={} new-circle-radius={}", enc_json(&p.id), enc_json(&p.new_circle_radius))
}

/// 📖 Parses the arguments of `change-seismic-slope-circle-radius`.
pub fn parse_change_seismic_slope_circle_radius(args: &Args) -> Result<ChangeSeismicSlopeCircleRadius, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-circle-radius", k);
    Ok(ChangeSeismicSlopeCircleRadius { id: dec_json(&arg("id")?)?, new_circle_radius: dec_json(&arg("new-circle-radius")?)? })
}

/// ✍️ Prints `change-seismic-slope-circle-centre-z` as an op-text line.
pub fn print_change_seismic_slope_circle_centre_z(p: &ChangeSeismicSlopeCircleCentreZ) -> String {
    format!("change-seismic-slope-circle-centre-z id={} new-circle-centre-z={}", enc_json(&p.id), enc_json(&p.new_circle_centre_z))
}

/// 📖 Parses the arguments of `change-seismic-slope-circle-centre-z`.
pub fn parse_change_seismic_slope_circle_centre_z(args: &Args) -> Result<ChangeSeismicSlopeCircleCentreZ, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-circle-centre-z", k);
    Ok(ChangeSeismicSlopeCircleCentreZ { id: dec_json(&arg("id")?)?, new_circle_centre_z: dec_json(&arg("new-circle-centre-z")?)? })
}

/// ✍️ Prints `change-seismic-slope-circle-centre-x` as an op-text line.
pub fn print_change_seismic_slope_circle_centre_x(p: &ChangeSeismicSlopeCircleCentreX) -> String {
    format!("change-seismic-slope-circle-centre-x id={} new-circle-centre-x={}", enc_json(&p.id), enc_json(&p.new_circle_centre_x))
}

/// 📖 Parses the arguments of `change-seismic-slope-circle-centre-x`.
pub fn parse_change_seismic_slope_circle_centre_x(args: &Args) -> Result<ChangeSeismicSlopeCircleCentreX, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-circle-centre-x", k);
    Ok(ChangeSeismicSlopeCircleCentreX { id: dec_json(&arg("id")?)?, new_circle_centre_x: dec_json(&arg("new-circle-centre-x")?)? })
}

/// ✍️ Prints `change-seismic-slope-topographic-factor` as an op-text line.
pub fn print_change_seismic_slope_topographic_factor(p: &ChangeSeismicSlopeTopographicFactor) -> String {
    format!("change-seismic-slope-topographic-factor id={} new-topographic-amplification={}", enc_json(&p.id), enc_json(&p.new_topographic_amplification))
}

/// 📖 Parses the arguments of `change-seismic-slope-topographic-factor`.
pub fn parse_change_seismic_slope_topographic_factor(args: &Args) -> Result<ChangeSeismicSlopeTopographicFactor, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-topographic-factor", k);
    Ok(ChangeSeismicSlopeTopographicFactor { id: dec_json(&arg("id")?)?, new_topographic_amplification: dec_json(&arg("new-topographic-amplification")?)? })
}

/// ✍️ Prints `change-seismic-slope-cohesion` as an op-text line.
pub fn print_change_seismic_slope_cohesion(p: &ChangeSeismicSlopeCohesion) -> String {
    format!("change-seismic-slope-cohesion id={} new-cohesion={}", enc_json(&p.id), enc_json(&p.new_cohesion))
}

/// 📖 Parses the arguments of `change-seismic-slope-cohesion`.
pub fn parse_change_seismic_slope_cohesion(args: &Args) -> Result<ChangeSeismicSlopeCohesion, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-cohesion", k);
    Ok(ChangeSeismicSlopeCohesion { id: dec_json(&arg("id")?)?, new_cohesion: dec_json(&arg("new-cohesion")?)? })
}

/// ✍️ Prints `change-seismic-slope-friction-angle` as an op-text line.
pub fn print_change_seismic_slope_friction_angle(p: &ChangeSeismicSlopeFrictionAngle) -> String {
    format!("change-seismic-slope-friction-angle id={} new-friction-angle={}", enc_json(&p.id), enc_json(&p.new_friction_angle))
}

/// 📖 Parses the arguments of `change-seismic-slope-friction-angle`.
pub fn parse_change_seismic_slope_friction_angle(args: &Args) -> Result<ChangeSeismicSlopeFrictionAngle, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-friction-angle", k);
    Ok(ChangeSeismicSlopeFrictionAngle { id: dec_json(&arg("id")?)?, new_friction_angle: dec_json(&arg("new-friction-angle")?)? })
}

/// ✍️ Prints `change-seismic-slope-angle` as an op-text line.
pub fn print_change_seismic_slope_angle(p: &ChangeSeismicSlopeAngle) -> String {
    format!("change-seismic-slope-angle id={} new-angle={}", enc_json(&p.id), enc_json(&p.new_angle))
}

/// 📖 Parses the arguments of `change-seismic-slope-angle`.
pub fn parse_change_seismic_slope_angle(args: &Args) -> Result<ChangeSeismicSlopeAngle, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-angle", k);
    Ok(ChangeSeismicSlopeAngle { id: dec_json(&arg("id")?)?, new_angle: dec_json(&arg("new-angle")?)? })
}

/// ✍️ Prints `change-seismic-slope-height` as an op-text line.
pub fn print_change_seismic_slope_height(p: &ChangeSeismicSlopeHeight) -> String {
    format!("change-seismic-slope-height id={} new-height={}", enc_json(&p.id), enc_json(&p.new_height))
}

/// 📖 Parses the arguments of `change-seismic-slope-height`.
pub fn parse_change_seismic_slope_height(args: &Args) -> Result<ChangeSeismicSlopeHeight, String> {
    let arg = |k: &str| required(args, "change-seismic-slope-height", k);
    Ok(ChangeSeismicSlopeHeight { id: dec_json(&arg("id")?)?, new_height: dec_json(&arg("new-height")?)? })
}
