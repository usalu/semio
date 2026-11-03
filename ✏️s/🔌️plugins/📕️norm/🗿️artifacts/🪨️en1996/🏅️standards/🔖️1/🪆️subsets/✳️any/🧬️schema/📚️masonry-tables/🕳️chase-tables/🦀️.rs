//! 🕳️ Chase and recess tables of EN 1996-1-1 8.6.2/8.6.3: Table 8.1 (vertical chases and recesses by wall thickness, depth and width) and Table 8.2 (horizontal and inclined chases by wall thickness and length) with their distance rules, every value a registry row `ndp.en1996.chase.*` (unverified). See [`super`].

use super::*;
use semio_s_artifact_norm_en1996_model::ChaseKind;
use semio_s_plugin_norm_compliance::ndp_cell;

const BANDS: [&str; 5] = ["b1", "b2", "b3", "b4", "b5"];

/// 🗂️ Value of the chase registry row `ndp.en1996.chase.<keys…>`.
pub fn chase_rule(keys: &[&str], annex: AnnexChoice) -> f64 {
    ndp_cell("ndp.en1996.chase", keys, annex)
}

/// 🧱️ Thickness band of the chase tables for a wall thickness, `None` below the thinnest tabulated wall.
pub fn chase_band(thickness_m: f64, annex: AnnexChoice) -> Option<&'static str> {
    (thickness_m + 1e-9 >= chase_rule(&["t-lower"], annex)).then(|| BANDS[..4].iter().copied().find(|b| thickness_m <= chase_rule(&["band", *b, "t-upper"], annex) + 1e-9).unwrap_or(BANDS[4]))
}

/// 📏️ Dimensions of a chase or recess that need no calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChaseLimits {
    pub depth_m: f64,
    pub width_m: Option<f64>,
    pub table: &'static str,
}

/// 🕳️ Limits of a chase from Table 8.1 (vertical chases and recesses, depth and width) or Table 8.2 (horizontal chases, depth by length); `None` when the wall is thinner than the tables, so the chase needs calculation. The larger vertical chase below the lower third of the storey height applies to a vertical chase whose top is at `top_m` above the floor in a storey of `storey_m`.
pub fn chase_limits(kind: ChaseKind, thickness_m: f64, length_m: f64, top_m: f64, storey_m: f64, annex: AnnexChoice) -> Option<ChaseLimits> {
    let band = chase_band(thickness_m, annex)?;
    Some(match kind {
        ChaseKind::Horizontal => {
            let column = if length_m <= chase_rule(&["h", "short-length"], annex) + 1e-9 { "depth-short" } else { "depth-long" };
            ChaseLimits { depth_m: chase_rule(&["h", band, column], annex), width_m: None, table: "8.2" }
        }
        ChaseKind::Vertical | ChaseKind::Recess => {
            let (depth, width) = (chase_rule(&["v", band, "depth"], annex), chase_rule(&["v", band, "width"], annex));
            let low = kind == ChaseKind::Vertical && thickness_m + 1e-9 >= chase_rule(&["v", "low-third", "t-min"], annex) && top_m <= chase_rule(&["v", "low-third", "height-ratio"], annex) * storey_m + 1e-9;
            if low {
                ChaseLimits { depth_m: depth.max(chase_rule(&["v", "low-third", "depth"], annex)), width_m: Some(width.max(chase_rule(&["v", "low-third", "width"], annex))), table: "8.1" }
            } else {
                ChaseLimits { depth_m: depth, width_m: Some(width), table: "8.1" }
            }
        }
    })
}

/// 📐️ Smallest horizontal distance two chases or recesses of the given kinds, widths and lengths need between them; `longer_m` is the larger length of two horizontal chases, `wider_m` the larger width of two recesses.
pub fn chase_gap_m(a: ChaseKind, b: ChaseKind, wider_m: f64, longer_m: f64, annex: AnnexChoice) -> f64 {
    match (a, b) {
        (ChaseKind::Horizontal, ChaseKind::Horizontal) => chase_rule(&["h", "spacing-factor"], annex) * longer_m,
        (ChaseKind::Recess, _) | (_, ChaseKind::Recess) => chase_rule(&["recess", "spacing-factor"], annex) * wider_m,
        _ => chase_rule(&["v", "spacing"], annex),
    }
}

/// 🚪️ Smallest horizontal distance of a chase or recess to an opening: 0.5 m for a horizontal chase, twice the recess width for a recess, else the vertical chase spacing.
pub fn chase_opening_gap_m(kind: ChaseKind, width_m: f64, annex: AnnexChoice) -> f64 {
    match kind {
        ChaseKind::Horizontal => chase_rule(&["h", "opening-distance"], annex),
        ChaseKind::Recess => chase_rule(&["recess", "spacing-factor"], annex) * width_m,
        ChaseKind::Vertical => chase_rule(&["v", "spacing"], annex),
    }
}

/// 📐️ Largest cumulative width of vertical chases and recesses of a wall of the given length.
pub fn chase_cumulative_width_m(wall_length_m: f64, annex: AnnexChoice) -> f64 {
    chase_rule(&["width-ratio"], annex) * wall_length_m
}

/// 🧱️ Thinnest tabulated wall (band start, in whole millimetres) in which a chase of the given depth and width needs no calculation, `None` when no band allows it.
pub fn chase_min_thickness_m(kind: ChaseKind, depth_m: f64, width_m: f64, length_m: f64, top_m: f64, storey_m: f64, annex: AnnexChoice) -> Option<f64> {
    let starts = std::iter::once(chase_rule(&["t-lower"], annex)).chain(BANDS[..4].iter().map(|b| chase_rule(&["band", *b, "t-upper"], annex) + 1e-3));
    starts.into_iter().find(|t| chase_limits(kind, *t, length_m, top_m, storey_m, annex).is_some_and(|l| l.depth_m + 1e-9 >= depth_m && l.width_m.is_none_or(|w| w + 1e-9 >= width_m)))
}

/// 🧱️ Lower edge of the thinnest tabulated wall followed by the upper edge of each closed thickness band [m]; the last band is open upwards.
pub fn chase_band_edges_m(annex: AnnexChoice) -> Vec<f64> {
    std::iter::once(chase_rule(&["t-lower"], annex)).chain(BANDS[..4].iter().map(|b| chase_rule(&["band", *b, "t-upper"], annex))).collect()
}
