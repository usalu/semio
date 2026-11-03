//! 🔗️ EN 1998 tie beams mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.

#![allow(unused_imports)]

use semio_s_artifact_norm_en1998_mutations_text_kit::{dec_json, enc_json, opt_index, required, Args};
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_displacement_limit::ChangeTieBeamDisplacementLimit;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_relative_displacement::ChangeTieBeamRelativeDisplacement;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_steel_design_strength::ChangeTieBeamSteelDesignStrength;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_steel_area::ChangeTieBeamSteelArea;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_depth::ChangeTieBeamDepth;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_width::ChangeTieBeamWidth;
use semio_s_artifact_norm_en1998_mut_tie_beams::change_tie_beam_span::ChangeTieBeamSpan;

/// ✍️ Prints `change-tie-beam-displacement-limit` as an op-text line.
pub fn print_change_tie_beam_displacement_limit(p: &ChangeTieBeamDisplacementLimit) -> String {
    format!("change-tie-beam-displacement-limit id={} new-displacement-limit={}", enc_json(&p.id), enc_json(&p.new_displacement_limit))
}

/// 📖 Parses the arguments of `change-tie-beam-displacement-limit`.
pub fn parse_change_tie_beam_displacement_limit(args: &Args) -> Result<ChangeTieBeamDisplacementLimit, String> {
    let arg = |k: &str| required(args, "change-tie-beam-displacement-limit", k);
    Ok(ChangeTieBeamDisplacementLimit { id: dec_json(&arg("id")?)?, new_displacement_limit: dec_json(&arg("new-displacement-limit")?)? })
}

/// ✍️ Prints `change-tie-beam-relative-displacement` as an op-text line.
pub fn print_change_tie_beam_relative_displacement(p: &ChangeTieBeamRelativeDisplacement) -> String {
    format!("change-tie-beam-relative-displacement id={} new-relative-displacement={}", enc_json(&p.id), enc_json(&p.new_relative_displacement))
}

/// 📖 Parses the arguments of `change-tie-beam-relative-displacement`.
pub fn parse_change_tie_beam_relative_displacement(args: &Args) -> Result<ChangeTieBeamRelativeDisplacement, String> {
    let arg = |k: &str| required(args, "change-tie-beam-relative-displacement", k);
    Ok(ChangeTieBeamRelativeDisplacement { id: dec_json(&arg("id")?)?, new_relative_displacement: dec_json(&arg("new-relative-displacement")?)? })
}

/// ✍️ Prints `change-tie-beam-steel-design-strength` as an op-text line.
pub fn print_change_tie_beam_steel_design_strength(p: &ChangeTieBeamSteelDesignStrength) -> String {
    format!("change-tie-beam-steel-design-strength id={} new-steel-design-strength={}", enc_json(&p.id), enc_json(&p.new_steel_design_strength))
}

/// 📖 Parses the arguments of `change-tie-beam-steel-design-strength`.
pub fn parse_change_tie_beam_steel_design_strength(args: &Args) -> Result<ChangeTieBeamSteelDesignStrength, String> {
    let arg = |k: &str| required(args, "change-tie-beam-steel-design-strength", k);
    Ok(ChangeTieBeamSteelDesignStrength { id: dec_json(&arg("id")?)?, new_steel_design_strength: dec_json(&arg("new-steel-design-strength")?)? })
}

/// ✍️ Prints `change-tie-beam-steel-area` as an op-text line.
pub fn print_change_tie_beam_steel_area(p: &ChangeTieBeamSteelArea) -> String {
    format!("change-tie-beam-steel-area id={} new-steel-area={}", enc_json(&p.id), enc_json(&p.new_steel_area))
}

/// 📖 Parses the arguments of `change-tie-beam-steel-area`.
pub fn parse_change_tie_beam_steel_area(args: &Args) -> Result<ChangeTieBeamSteelArea, String> {
    let arg = |k: &str| required(args, "change-tie-beam-steel-area", k);
    Ok(ChangeTieBeamSteelArea { id: dec_json(&arg("id")?)?, new_steel_area: dec_json(&arg("new-steel-area")?)? })
}

/// ✍️ Prints `change-tie-beam-depth` as an op-text line.
pub fn print_change_tie_beam_depth(p: &ChangeTieBeamDepth) -> String {
    format!("change-tie-beam-depth id={} new-depth={}", enc_json(&p.id), enc_json(&p.new_depth))
}

/// 📖 Parses the arguments of `change-tie-beam-depth`.
pub fn parse_change_tie_beam_depth(args: &Args) -> Result<ChangeTieBeamDepth, String> {
    let arg = |k: &str| required(args, "change-tie-beam-depth", k);
    Ok(ChangeTieBeamDepth { id: dec_json(&arg("id")?)?, new_depth: dec_json(&arg("new-depth")?)? })
}

/// ✍️ Prints `change-tie-beam-width` as an op-text line.
pub fn print_change_tie_beam_width(p: &ChangeTieBeamWidth) -> String {
    format!("change-tie-beam-width id={} new-width={}", enc_json(&p.id), enc_json(&p.new_width))
}

/// 📖 Parses the arguments of `change-tie-beam-width`.
pub fn parse_change_tie_beam_width(args: &Args) -> Result<ChangeTieBeamWidth, String> {
    let arg = |k: &str| required(args, "change-tie-beam-width", k);
    Ok(ChangeTieBeamWidth { id: dec_json(&arg("id")?)?, new_width: dec_json(&arg("new-width")?)? })
}

/// ✍️ Prints `change-tie-beam-span` as an op-text line.
pub fn print_change_tie_beam_span(p: &ChangeTieBeamSpan) -> String {
    format!("change-tie-beam-span id={} new-span={}", enc_json(&p.id), enc_json(&p.new_span))
}

/// 📖 Parses the arguments of `change-tie-beam-span`.
pub fn parse_change_tie_beam_span(args: &Args) -> Result<ChangeTieBeamSpan, String> {
    let arg = |k: &str| required(args, "change-tie-beam-span", k);
    Ok(ChangeTieBeamSpan { id: dec_json(&arg("id")?)?, new_span: dec_json(&arg("new-span")?)? })
}
