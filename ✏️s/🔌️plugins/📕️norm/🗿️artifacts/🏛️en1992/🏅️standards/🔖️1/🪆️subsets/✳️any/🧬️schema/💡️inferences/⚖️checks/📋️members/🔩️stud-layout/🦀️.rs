//! 🔩️ Stud layout: the radial rails (rows), radial spacings and tangential spacings of the shear studs of a punching zone derived from the persisted `studRows` (EN 1992-1-1 6.4.5 and 9.4.3; limits UNVERIFIED against the standard text). A row is a rail along the ray at its angle from the column centre (column frame, `x` along `c1`, free slab edges flush at `+x` / `+y`); its k-th stud sits `s_0 + k s_r` from the column face. See [`super`].

use super::*;

const S0_MIN: f64 = 0.3;
const S0_MAX: f64 = 0.5;
const S_R_MAX: f64 = 0.75;
const S_T_NEAR: f64 = 1.5;
const S_T_FAR: f64 = 2.0;
const NEAR_ZONE: f64 = 2.0;
const EDGE_TOLERANCE: f64 = 1e-9;

/// 🔩 Derived layout of the studs of one member.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct StudLayout {
    pub(super) rows: usize,
    pub(super) studs_per_row: u32,
    pub(super) s0: f64,
    pub(super) s_r: f64,
    pub(super) s_t: f64,
    pub(super) s_t_radius: f64,
    pub(super) st_limit_factor: f64,
    pub(super) outside: bool,
}

impl StudLayout {
    fn terms(&self, d: f64) -> [(&'static str, f64); 4] {
        let low = if self.s0 < S0_MIN * d { S0_MIN * d / self.s0.max(1e-9) } else { 0.0 };
        [("s_0", (self.s0 / (S0_MAX * d).max(1e-9)).max(low)), ("s_r", self.s_r / (S_R_MAX * d).max(1e-9)), ("s_t", self.s_t / (self.st_limit_factor * d).max(1e-9)), ("slab edge", if self.outside { 1.0 + 1e-6 } else { 0.0 })]
    }

    /// 📈 Governing utilisation of the layout rules (largest of s_0, s_r, s_t; a stud beyond a free slab edge counts as failed).
    pub(super) fn utilisation(&self, d: f64) -> f64 {
        self.terms(d).iter().map(|t| t.1).fold(0.0, f64::max)
    }

    /// 🏷️ Name of the governing layout rule.
    pub(super) fn governing(&self, d: f64) -> &'static str {
        self.terms(d).iter().fold(("s_0", f64::MIN), |a, t| if t.1 > a.1 { *t } else { a }).0
    }
}

/// 📍 Position of the k-th stud of `row` in the column frame (m).
pub(super) fn stud_position(row: &semio_s_artifact_norm_en1992_model::StudRow, k: u32, c1: f64, c2: f64) -> (f64, f64) {
    row.position(k, c1, c2)
}

fn beyond_free_edge(position: Position, p: (f64, f64), c1: f64, c2: f64) -> bool {
    match position {
        Position::Interior => false,
        Position::Edge | Position::WallEnd => p.0 > c1 / 2.0 + EDGE_TOLERANCE,
        Position::Corner => p.0 > c1 / 2.0 + EDGE_TOLERANCE || p.1 > c2 / 2.0 + EDGE_TOLERANCE,
    }
}

/// 🔩 Layout of the stud rows of `member` around a column `c1 × c2` [m]; `None` without a row with studs. The tangential spacing at the k-th stud position is the chord between the k-th studs of angularly adjacent rows (the first and last row are adjacent around an interior column only); the radial spacing is the largest `s_r` of a row with two or more studs.
pub(super) fn stud_layout(member: &RcMember, position: Position, c1: f64, c2: f64) -> Option<StudLayout> {
    let mut rows: Vec<&semio_s_artifact_norm_en1992_model::StudRow> = member.stud_rows.iter().filter(|r| r.count > 0 && r.diameter > 0.0).collect();
    rows.sort_by(|a, b| a.angle.rem_euclid(2.0 * std::f64::consts::PI).total_cmp(&b.angle.rem_euclid(2.0 * std::f64::consts::PI)));
    let first = rows.first()?;
    let d = member.effective_depth;
    let studs = rows.iter().map(|r| r.count).max().unwrap_or(0);
    let s0 = rows.iter().map(|r| r.spacing_first).fold(first.spacing_first, f64::min);
    let s_r = rows.iter().filter(|r| r.count > 1).map(|r| r.spacing_radial).fold(0.0, f64::max);
    let mut outside = false;
    let (mut s_t, mut s_t_radius, mut factor) = (0.0, first.spacing_first, S_T_NEAR);
    for k in 0..studs {
        let at_k: Vec<(&semio_s_artifact_norm_en1992_model::StudRow, (f64, f64))> = rows.iter().filter(|r| r.count > k).map(|r| (*r, stud_position(r, k, c1, c2))).collect();
        outside |= at_k.iter().any(|(_, p)| beyond_free_edge(position, *p, c1, c2));
        let pairs = if matches!(position, Position::Interior) && at_k.len() > 2 { at_k.len() } else { at_k.len().saturating_sub(1) };
        for i in 0..pairs {
            let (a, b) = (at_k[i], at_k[(i + 1) % at_k.len()]);
            let chord = (a.1 .0 - b.1 .0).hypot(a.1 .1 - b.1 .1);
            let radius = a.0.spacing_first + f64::from(k) * a.0.spacing_radial;
            let limit = if radius <= NEAR_ZONE * d { S_T_NEAR } else { S_T_FAR };
            if chord / limit > s_t / factor {
                (s_t, s_t_radius, factor) = (chord, radius, limit);
            }
        }
    }
    Some(StudLayout { rows: rows.len(), studs_per_row: studs, s0, s_r, s_t, s_t_radius, st_limit_factor: factor, outside })
}
