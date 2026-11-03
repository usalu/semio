//! 🦴️ EN 1999-1-1 §6.7.6/§6.7.7 transverse web stiffeners of plate girders: pure clause functions. See [`super`].

use crate::*;
use semio_s_artifact_norm_en1999_model::TransverseStiffener;
use semio_s_plugin_norm_compliance::AnnexChoice;

/// 🔖️ Registry keys (below `ndp.en1999.web-stiffener.`) of the five structure constants; recalled from EN 1993-1-5 9.1 and 9.3.3 and unverified.
pub const KEYS: [&str; 5] = ["strip-factor", "inertia-factor-short", "inertia-factor-long", "aspect-limit", "buckling-length-factor"];

/// 🦴 The structure constants: web strip factor (15), required inertia factors (1.5 and 0.75), aspect limit (√2), buckling length factor (0.75).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constants {
    pub strip: f64,
    pub short: f64,
    pub long: f64,
    pub aspect: f64,
    pub length: f64,
}

/// 🦴 The constants of one annex choice, `None` while any of them is not registered as a number.
pub fn constants(annex: AnnexChoice) -> Option<Constants> {
    let n = |key: &str| semio_s_plugin_norm_compliance::ndp_number_opt(&format!("ndp.en1999.web-stiffener.{key}"), annex);
    Some(Constants { strip: n(KEYS[0])?, short: n(KEYS[1])?, long: n(KEYS[2])?, aspect: n(KEYS[3])?, length: n(KEYS[4])? })
}

/// 🦴 Panel widths `(before, after)` on both sides of the stiffener at `x` on a girder of `length` whose stiffeners stand at `positions` (the girder ends close the outer panels).
pub fn panels(length: f64, positions: &[f64], x: f64) -> (f64, f64) {
    let before = positions.iter().copied().filter(|v| *v < x).fold(0.0, f64::max);
    let after = positions.iter().copied().filter(|v| *v > x).fold(length, f64::min);
    (x - before, after - x)
}

/// 🦴 Panel edges of the web of a girder of `length`: the girder start, every distinct transverse stiffener position inside the girder in ascending order, the girder end; empty while no stiffener stands inside (web stiffened at the supports only).
pub fn web_edges(length: f64, positions: &[f64]) -> Vec<f64> {
    let mut inner: Vec<f64> = positions.iter().copied().filter(|p| *p > 0.0 && *p < length).collect();
    if inner.is_empty() {
        return Vec::new();
    }
    inner.sort_by(|a, b| a.total_cmp(b));
    inner.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    std::iter::once(0.0).chain(inner).chain(std::iter::once(length)).collect()
}

/// 🦴 Panel widths `a` of the web of a girder of `length` between adjacent transverse stiffeners and the girder ends (the supports), from the stiffener `positions`; empty while no stiffener stands inside the girder (see [`web_edges`]).
pub fn web_panels(length: f64, positions: &[f64]) -> Vec<f64> {
    web_edges(length, positions).windows(2).map(|w| w[1] - w[0]).collect()
}

/// 🦴 Panel widths the clause functions are evaluated for: the [`web_panels`], or the single `0.0` (no transverse stiffener, `a` unbounded) while there are none.
pub fn spacings(panels: &[f64]) -> Vec<f64> {
    if panels.is_empty() { vec![0.0] } else { panels.to_vec() }
}

/// 🦴 Widest panel of [`web_panels`], `0.0` while there are none.
pub fn widest(panels: &[f64]) -> f64 {
    panels.iter().copied().fold(0.0, f64::max)
}

/// 🦴 The transverse stiffeners `(id, position)` of the girder `girder_id`.
pub fn rows_of(rows: &[TransverseStiffener], girder_id: &str) -> Vec<(String, f64)> {
    rows.iter().filter(|s| s.girder_id == girder_id).map(|s| (s.id.clone(), s.position)).collect()
}

/// 🦴 The stiffener of `rows` whose smallest move `(id, from, to)` makes the panels of the girder of `length` pass `accept`; `to` is searched on a grid of 1/400 of the length, `None` while no single move helps (a stiffener has to be inserted).
pub fn nearest_move(length: f64, rows: &[(String, f64)], accept: impl Fn(&[f64]) -> bool) -> Option<(String, f64, f64)> {
    const STEPS: usize = 400;
    let mut best: Option<(String, f64, f64)> = None;
    for (k, (id, from)) in rows.iter().enumerate() {
        for step in 1..STEPS {
            let to = length * step as f64 / STEPS as f64;
            let positions: Vec<f64> = rows.iter().enumerate().map(|(j, (_, p))| if j == k { to } else { *p }).collect();
            if accept(&web_panels(length, &positions)) && best.as_ref().is_none_or(|(_, f, t)| (to - from).abs() < (t - f).abs()) {
                best = Some((id.clone(), *from, to));
            }
        }
    }
    best
}

/// 🦴 The position of the one additional stiffener that makes the panels of the girder of `length` pass `accept`, the grid position (1/400 of the length) closest to the middle of the widest panel; `None` while a single insert does not help.
pub fn minimal_insert(length: f64, rows: &[(String, f64)], accept: impl Fn(&[f64]) -> bool) -> Option<f64> {
    const STEPS: usize = 400;
    let positions: Vec<f64> = rows.iter().map(|(_, p)| *p).collect();
    let edges = web_edges(length, &positions);
    let centre = if edges.len() < 2 { 0.5 * length } else {
        let k = edges.windows(2).map(|w| w[1] - w[0]).enumerate().fold((0, 0.0), |b, (i, a)| if a > b.1 { (i, a) } else { b }).0;
        0.5 * (edges[k] + edges[k + 1])
    };
    (1..STEPS)
        .map(|step| length * step as f64 / STEPS as f64)
        .filter(|x| !positions.iter().any(|p| (p - x).abs() < 1e-9) && accept(&web_panels(length, &[positions.clone(), vec![*x]].concat())))
        .min_by(|a, b| (a - centre).abs().total_cmp(&(b - centre).abs()))
}

/// 🦴 The unused row id `stiffener-<girder>-<n>` for a stiffener inserted on `girder_id`.
pub fn fresh_id(girder_id: &str, rows: &[(String, f64)]) -> String {
    (rows.len() + 1..).map(|n| format!("stiffener-{girder_id}-{n}")).find(|id| !rows.iter().any(|(r, _)| r == id)).unwrap_or_default()
}

/// 🦴 Positions of the transverse stiffeners of the girder `girder_id`.
pub fn positions_of(rows: &[TransverseStiffener], girder_id: &str) -> Vec<f64> {
    rows.iter().filter(|s| s.girder_id == girder_id).map(|s| s.position).collect()
}

/// 🦴 Effective cross-section of a transverse stiffener about its centroidal axis parallel to the web: the web strip `strip × t_w` plus one flat bar (`one` side) or two (`two` sides, symmetric).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effective {
    pub strip: f64,
    pub area: f64,
    pub inertia: f64,
    pub centroid: f64,
}

/// 🦴 [`Effective`] of the bar `width × thickness` with the web strip of `half_strip` on each side of the bar plus the bar thickness.
pub fn effective_section(width: f64, thickness: f64, two_sided: bool, t_w: f64, half_strip: f64) -> Effective {
    let strip = 2.0 * half_strip + thickness;
    let (a_s, i_s) = (strip * t_w, strip * t_w.powi(3) / 12.0);
    let (a_b, i_b, d) = (width * thickness, thickness * width.powi(3) / 12.0, 0.5 * (t_w + width));
    if two_sided {
        Effective { strip, area: a_s + 2.0 * a_b, inertia: i_s + 2.0 * (i_b + a_b * d * d), centroid: 0.0 }
    } else {
        let y_c = a_b * d / (a_s + a_b).max(1e-30);
        Effective { strip, area: a_s + a_b, inertia: i_s + a_s * y_c * y_c + i_b + a_b * (d - y_c).powi(2), centroid: y_c }
    }
}

/// 🦴 Required second moment of area of the stiffener: `factor_short·h_w³·t_w³/a²` below the aspect limit a/h_w, `factor_long·h_w·t_w³` at or above it.
pub fn required_inertia(h_w: f64, t_w: f64, a: f64, k: &Constants) -> f64 {
    if a / h_w.max(1e-12) < k.aspect { k.short * h_w.powi(3) * t_w.powi(3) / (a * a).max(1e-18) } else { k.long * h_w * t_w.powi(3) }
}

/// 🦴 Axial force of the stiffener: the part of the shear force `V_Ed` that the post-critical web `ρ_v·h_w·t_w·f_o/(√3·γ_M1)` does not carry, plus the local load `F_Ed` introduced at the stiffener.
pub fn axial_demand(v_ed: f64, rho_v: f64, h_w: f64, t_w: f64, f_o: f64, f_loc: f64, gamma_m1: f64) -> f64 {
    (v_ed.abs() - rho_v * h_w * t_w * f_o / (3.0f64.sqrt() * gamma_m1)).max(0.0) + f_loc.abs()
}

/// 🦴 Flexural buckling resistance of the stiffener with its web strip as a strut of length `k.length·h_w`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Strut {
    pub l_cr: f64,
    pub lambda: f64,
    pub chi: f64,
    pub n_rd: f64,
}

/// 🦴 [`Strut`] of an [`Effective`] section with the buckling curve of the alloy class.
pub fn strut(eff: &Effective, h_w: f64, f_o: f64, class_a: bool, gamma_m1: f64, k: &Constants) -> Strut {
    let l_cr = k.length * h_w;
    let lambda = part_1_1::lambda_bar(l_cr, eff.inertia, eff.area, f_o);
    let (alpha, lambda_0) = part_1_1::buckling_curve(class_a);
    let chi = part_1_1::chi_from_lambda(lambda, alpha, lambda_0);
    Strut { l_cr, lambda, chi, n_rd: chi * eff.area * f_o / gamma_m1 }
}
