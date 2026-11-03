//! 🕳️ Punching shear (§6.4), strut-and-tie models (§6.5) and partially loaded areas (§6.7) as catalogue entries P-01 … P-07, ST-01 … ST-04 and PA-01. See [`super`].

use super::*;
use std::f64::consts::PI;

#[path = "🕳️opening-tangents/🦀️.rs"]
pub mod opening_tangents;
pub use opening_tangents::*;

/// 🧭 Column position relative to the slab edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Interior,
    Edge,
    Corner,
    WallEnd,
}

/// 🔤 Position from its text key.
pub fn position_of(s: &str) -> Position {
    match s { "edge" => Position::Edge, "corner" => Position::Corner, "wall_end" => Position::WallEnd, _ => Position::Interior }
}

/// 📏 Basic control perimeter u1 at 2d (mm) of a rectangular column c1 × c2; `e1`, `e2` are the clear distances from the column face to the free slab edges, `None` for flush columns beyond which the perimeter is interior.
pub fn u1_rect(pos: Position, c1: f64, c2: f64, d: f64, e1: f64, e2: f64) -> f64 {
    let arc = |e: f64| if e >= 2.0 * d { PI * d } else { 2.0 * d * (e / (2.0 * d)).asin() };
    match pos {
        Position::Interior => 2.0 * (c1 + c2) + 4.0 * PI * d,
        Position::Edge | Position::WallEnd => if e1 >= 2.0 * d { u1_rect(Position::Interior, c1, c2, d, e1, e2) } else { c2 + 2.0 * c1 + 2.0 * PI * d + 2.0 * arc(e1) },
        Position::Corner => c1 + c2 + PI * d + arc(e1) + arc(e2),
    }
}

/// 🔩 Distance factor k of the outermost stud ring from u_out (6.4.5(4)), times d.
pub fn k_out(annex: AnnexChoice) -> f64 {
    ndp(annex, "ndp.en1992.punch-k-out")
}

/// 📏 Perimeter (mm) at the offset `a` from the face of a column flush with its free slab edges: interior all around, edge on three faces, corner on two (u1 is the value at 2d).
pub fn u_at_offset(pos: Position, c1: f64, c2: f64, a: f64) -> f64 {
    match pos {
        Position::Interior => 2.0 * (c1 + c2) + 2.0 * PI * a,
        Position::Edge | Position::WallEnd => c2 + 2.0 * c1 + PI * a,
        Position::Corner => c1 + c2 + 0.5 * PI * a,
    }
}

/// 📐 Offset from the column face at which the perimeter has the length `u` (inverse of [`u_at_offset`]), negative when `u` is shorter than the column perimeter.
pub fn offset_of_perimeter(pos: Position, c1: f64, c2: f64, u: f64) -> f64 {
    (u - u_at_offset(pos, c1, c2, 0.0)) / (u_at_offset(pos, c1, c2, 1.0) - u_at_offset(pos, c1, c2, 0.0))
}

/// 📏 Perimeter u0 at the column face.
pub fn u0_rect(pos: Position, c1: f64, c2: f64, d: f64) -> f64 {
    match pos {
        Position::Interior => 2.0 * (c1 + c2),
        Position::Edge | Position::WallEnd => (c2 + 3.0 * d).min(c2 + 2.0 * c1),
        Position::Corner => (3.0 * d).min(c1 + c2),
    }
}

/// 🧮 W1 of Eq. 6.40 for an interior rectangular column.
pub fn w1_interior(c1: f64, c2: f64, d: f64) -> f64 {
    c1 * c1 / 2.0 + c1 * c2 + 4.0 * c2 * d + 16.0 * d * d + 2.0 * PI * d * c1
}

/// 🧮 W1 for an edge column with the free edge parallel to c2 (6.4.3(4)).
pub fn w1_edge(c1: f64, c2: f64, d: f64) -> f64 {
    c2 * c2 / 4.0 + c1 * c2 + 4.0 * c1 * d + 8.0 * d * d + PI * d * c2
}

/// 🔢 k of Table 6.1 from c1/c2.
pub fn k_table_61(ratio: f64) -> f64 {
    interp(&[(0.5, 0.45), (1.0, 0.60), (2.0, 0.70), (3.0, 0.80)], ratio)
}

/// 📉 Opening shadow: perimeter length lost behind a rectangular opening of width `w` whose near side is `a` from the column centre, for an equivalent perimeter radius `r`; zero when the clear distance exceeds 6d.
pub fn opening_shadow(w: f64, a: f64, r: f64, clear: f64, d: f64) -> f64 {
    if clear > 6.0 * d || a <= 0.0 { 0.0 } else { (w * r / a).min(w) }
}

/// 🔻 Numerator of C_Rd,c (Eq. 6.47): the NDP value, reduced for interior columns with u_0/d below the NDP limit (DE NA 6.4.4, rows `ndp.en1992.punch-small-u0.*`; the limit is 0 in the EN annex, so nothing changes there).
pub fn c_rd_c_numerator(annex: AnnexChoice, pos: Position, u0_over_d: f64) -> f64 {
    let base = ndp(annex, "ndp.en1992.crd-c-punch");
    if pos == Position::Interior && u0_over_d < ndp(annex, "ndp.en1992.punch-small-u0.limit") {
        (base * (ndp(annex, "ndp.en1992.punch-small-u0.slope") * u0_over_d + ndp(annex, "ndp.en1992.punch-small-u0.offset"))).max(ndp(annex, "ndp.en1992.punch-small-u0.floor"))
    } else {
        base
    }
}

/// 💪 Punching resistance v_Rd,c (MPa) without reinforcement; `u0_over_d` is `f64::INFINITY` when no column perimeter applies; returns (v_Rd,c, v_min, k, ρ_l).
pub fn v_rd_c_punch(annex: AnnexChoice, fck: f64, d: f64, rho_y: f64, rho_z: f64, sigma_cp: f64, pos: Position, u0_over_d: f64) -> (f64, f64, f64, f64) {
    let gc = ndp(annex, "ndp.en1992.m-01.gamma-c-persistent");
    let k = (1.0 + (200.0 / d).sqrt()).min(2.0);
    let mut rho = (rho_y * rho_z).sqrt().min(0.02);
    if annex == AnnexChoice::De {
        let cap = 0.5 * f_cd(annex, fck, false) / f_yd(annex, 500.0, false);
        rho = rho.min(cap);
    }
    let vmin = shear_vmin(annex, d, gc) * k.powf(1.5) * fck.sqrt();
    let k1 = ndp(annex, "ndp.en1992.k1-punch");
    let v = (c_rd_c_numerator(annex, pos, u0_over_d) / gc * k * (100.0 * rho * fck).powf(1.0 / 3.0) + k1 * sigma_cp).max(vmin + k1 * sigma_cp);
    (v, vmin, k, rho)
}

fn shear_vmin(annex: AnnexChoice, d: f64, gc: f64) -> f64 {
    super::shear_torsion::vmin_coeff(annex, d, gc)
}

/// 🎯 Punching eccentricity factor β = 1 + k M_Ed/V_Ed u_1/W_1 (Eq. 6.39).
pub fn beta_moment(k: f64, m_ed: f64, v_ed: f64, u1: f64, w1: f64) -> f64 {
    1.0 + k * m_ed / v_ed * u1 / w1
}

/// 🎯 Punching factor β = 1 + 1.8 √((e_y/b_z)² + (e_z/b_y)²) of a rectangular column (Eq. 6.43).
pub fn beta_rectangular(ey: f64, ez: f64, by: f64, bz: f64) -> f64 {
    1.0 + 1.8 * ((ey / bz).powi(2) + (ez / by).powi(2)).sqrt()
}

/// 🎯 Rule of 6.4.3 that fixed the load increase factor β of one column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BetaRule {
    Simplified,
    NoEccentricity,
    InteriorOneAxis,
    InteriorBothAxes,
    EdgeInward,
    EdgeInwardParallel,
    EdgeOutward,
    CornerInward,
    CornerOutward,
}

impl BetaRule {
    /// 🏷 Clause and equation of the rule in English and German.
    pub fn label(self) -> (&'static str, &'static str) {
        match self {
            BetaRule::Simplified => ("fixed value of 6.4.3(6)", "fester Wert nach 6.4.3(6)"),
            BetaRule::NoEccentricity => ("no unbalanced moment, β = 1", "kein Stützenmoment, β = 1"),
            BetaRule::InteriorOneAxis => ("Eq. 6.39", "Gl. 6.39"),
            BetaRule::InteriorBothAxes => ("Eq. 6.43", "Gl. 6.43"),
            BetaRule::EdgeInward => ("Eq. 6.44, u₁/u₁* (reading of Figure 6.20 unverified)", "Gl. 6.44, u₁/u₁* (Lesart von Bild 6.20 ungeprüft)"),
            BetaRule::EdgeInwardParallel => ("Eq. 6.45, u₁/u₁* + k u₁/W₁ e_par (reading of Figure 6.20 unverified)", "Gl. 6.45, u₁/u₁* + k u₁/W₁ e_par (Lesart von Bild 6.20 ungeprüft)"),
            BetaRule::EdgeOutward => ("outward eccentricity: Eq. 6.39 about the perimeter centroid, full u₁, not below u₁/u₁* (reading unverified)", "Ausmitte nach außen: Gl. 6.39 um den Schwerpunkt des Rundschnitts, volles u₁, nicht unter u₁/u₁* (Lesart ungeprüft)"),
            BetaRule::CornerInward => ("Eq. 6.46, u₁/u₁* (reading of Figure 6.20 unverified)", "Gl. 6.46, u₁/u₁* (Lesart von Bild 6.20 ungeprüft)"),
            BetaRule::CornerOutward => ("outward eccentricity: Eq. 6.39 per axis about the perimeter centroid, full u₁, not below u₁/u₁* (reading unverified)", "Ausmitte nach außen: Gl. 6.39 je Achse um den Schwerpunkt des Rundschnitts, volles u₁, nicht unter u₁/u₁* (Lesart ungeprüft)"),
        }
    }
}

/// 🚫 Why 6.4.3(3) to (5) give no β for the column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BetaRefusal {
    NoPunchingForce,
    WallEnd,
}

impl BetaRefusal {
    /// 🏷 The reason in English and German.
    pub fn label(self) -> (&'static str, &'static str) {
        match self {
            BetaRefusal::NoPunchingForce => ("Unbalanced moments need a punching force V_Ed > 0 (e = M_Ed/V_Ed).", "Stützenmomente brauchen eine Durchstanzkraft V_Ed > 0 (e = M_Ed/V_Ed)."),
            BetaRefusal::WallEnd => ("6.4.3 gives no moment rule for wall ends; use the simplified method.", "6.4.3 enthält keine Momentenregel für Wandenden; vereinfachte Methode verwenden."),
        }
    }
}

/// 🎯 Load increase factor β with the rule that fixed it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BetaOutcome {
    pub beta: f64,
    pub rule: BetaRule,
}

/// 📏 Reduced control perimeter u₁* of an edge or corner column (Figure 6.20): the legs along the column sides are cut to min(1.5 d, 0.5 c) beside the rounded corners of radius 2 d; [`u1_rect`] without free-edge distances keeps full legs. Unverified reading of the figure.
pub fn u1_reduced(pos: Position, c1: f64, c2: f64, d: f64) -> f64 {
    let leg = |c: f64| (1.5 * d).min(0.5 * c);
    match pos {
        Position::Edge | Position::WallEnd => c2 + 2.0 * leg(c1) + 2.0 * PI * d,
        Position::Corner => leg(c1) + leg(c2) + PI * d,
        Position::Interior => u1_rect(pos, c1, c2, d, 0.0, 0.0),
    }
}

/// 🧭 W₁ = ∫|e| dl (Eq. 6.40) of the full control perimeter u₁ of a column flush with its free edges about the axis through the perimeter centroid normal to the eccentricity; `c_a` is the column side along the eccentricity, `c_b` the other one. Unverified reading of 6.4.3(3) for an outward eccentricity (M_Ed taken about the perimeter centroid).
pub fn perimeter_w1_centroidal(pos: Position, c_a: f64, c_b: f64, d: f64) -> f64 {
    let (h, r) = (c_a / 2.0, 2.0 * d);
    let arcs = if pos == Position::Corner { 1.0 } else { 2.0 };
    let spans = if pos == Position::Corner { 1.0 } else { 2.0 };
    let (front, arc_len) = (c_b, arcs * r * PI / 2.0);
    let total = spans * c_a + front + arc_len;
    let mean = (front * (h + r) + arc_len * (h + r * 2.0 / PI)) / total;
    let abs_span = |a: f64| spans * span_abs(-h, h, a);
    let abs_arc = |a: f64| {
        let k = h - a;
        let integral = if k >= 0.0 { k * PI / 2.0 + r } else if -k >= r { -(k * PI / 2.0 + r) } else {
            let phi = (-k / r).asin();
            let g = |x: f64| k * x - r * x.cos();
            -(g(phi) - g(0.0)) + (g(PI / 2.0) - g(phi))
        };
        arcs * r * integral
    };
    abs_span(mean) + front * (h + r - mean).abs() + abs_arc(mean)
}

fn span_abs(lo: f64, hi: f64, a: f64) -> f64 {
    if a <= lo { ((hi - a).powi(2) - (lo - a).powi(2)) / 2.0 } else if a >= hi { ((a - lo).powi(2) - (a - hi).powi(2)) / 2.0 } else { ((a - lo).powi(2) + (hi - a).powi(2)) / 2.0 }
}

/// 🎯 β from the unbalanced moments `m1` (eccentricity along c₁, perpendicular to the free edge of an edge column) and `m2` (along c₂) of a rectangular column, all lengths in one unit, `v_ed` and the moments in matching units (6.4.3(3) to (5)): interior columns use Eq. 6.39 for one axis and Eq. 6.43 for both; edge and corner columns with inward eccentricity use u₁/u₁* (Eq. 6.44, 6.46) plus k u₁/W₁ e_par for a moment parallel to the edge (Eq. 6.45); an outward eccentricity (negative moment) uses the general Eq. 6.39 with the full u₁ about the centroid of the perimeter (M_Ed taken about it) and never gives less than the inward value, so the punching check is always verified.
pub fn beta_unbalanced(pos: Position, c1: f64, c2: f64, d: f64, v_ed: f64, m1: f64, m2: f64) -> Result<BetaOutcome, BetaRefusal> {
    let moments = m1 != 0.0 || m2 != 0.0;
    if moments && v_ed <= 0.0 {
        return Err(BetaRefusal::NoPunchingForce);
    }
    let u1 = u1_rect(pos, c1, c2, d, 0.0, 0.0);
    match pos {
        Position::WallEnd => Err(BetaRefusal::WallEnd),
        Position::Interior if !moments => Ok(BetaOutcome { beta: 1.0, rule: BetaRule::NoEccentricity }),
        Position::Interior if m1 != 0.0 && m2 != 0.0 => Ok(BetaOutcome { beta: beta_rectangular(m1.abs() / v_ed, m2.abs() / v_ed, c1 + 4.0 * d, c2 + 4.0 * d), rule: BetaRule::InteriorBothAxes }),
        Position::Interior if m1 != 0.0 => Ok(BetaOutcome { beta: beta_moment(k_table_61(c1 / c2), m1.abs(), v_ed, u1, w1_interior(c1, c2, d)), rule: BetaRule::InteriorOneAxis }),
        Position::Interior => Ok(BetaOutcome { beta: beta_moment(k_table_61(c2 / c1), m2.abs(), v_ed, u1, w1_interior(c2, c1, d)), rule: BetaRule::InteriorOneAxis }),
        Position::Edge => {
            let base = u1 / u1_reduced(pos, c1, c2, d);
            let parallel = if m2 == 0.0 { 0.0 } else { k_table_61(c1 / (2.0 * c2)) * u1 / w1_edge(c1, c2, d) * m2.abs() / v_ed };
            let rule = if m2 == 0.0 { BetaRule::EdgeInward } else { BetaRule::EdgeInwardParallel };
            if m1 >= 0.0 {
                return Ok(BetaOutcome { beta: base + parallel, rule });
            }
            let general = beta_moment(k_table_61(c1 / c2), m1.abs(), v_ed, u1, perimeter_w1_centroidal(pos, c1, c2, d)) + parallel;
            Ok(BetaOutcome { beta: general.max(base + parallel), rule: BetaRule::EdgeOutward })
        }
        Position::Corner => {
            let base = u1 / u1_reduced(pos, c1, c2, d);
            if m1 >= 0.0 && m2 >= 0.0 {
                return Ok(BetaOutcome { beta: base, rule: BetaRule::CornerInward });
            }
            let axis = |c_a: f64, c_b: f64, m: f64| {
                beta_moment(k_table_61(c_a / c_b), (-m).max(0.0), v_ed, u1, perimeter_w1_centroidal(pos, c_a, c_b, d)) - 1.0
            };
            let general = 1.0 + axis(c1, c2, m1) + axis(c2, c1, m2);
            Ok(BetaOutcome { beta: general.max(base), rule: BetaRule::CornerOutward })
        }
    }
}

/// 🎯 Punching shear stress v_Ed = β V_Ed/(u_i d) (Eq. 6.38); any consistent unit system.
pub fn v_ed_punching(beta: f64, v_ed: f64, u: f64, d: f64) -> f64 {
    beta * v_ed / (u * d)
}

/// 🎯 Punching resistance v_Rd,c = max(C_Rd,c k (100 ρ_l f_ck)^(1/3) + k_1 σ_cp, v_min + k_1 σ_cp) (Eq. 6.47) with f_ck in MPa and every stress in MPa.
pub fn v_rd_c_from(c_rd_c: f64, k: f64, rho_l: f64, fck: f64, k1: f64, sigma_cp: f64, v_min: f64) -> f64 {
    (c_rd_c * k * (100.0 * rho_l * fck).powf(1.0 / 3.0) + k1 * sigma_cp).max(v_min + k1 * sigma_cp)
}

pub fn p01(c: &Case) -> R {
    let (c1, d) = (c.pos("c1")?, c.pos("d")?);
    let c2 = c.o("c2", c1);
    let pos = position_of(c.so("position", "interior"));
    let (e1, e2) = (c.o("e1", 0.0), c.o("e2", 0.0));
    let circular = c.o("circular", 0.0) > 0.0;
    let (u0, mut u1) = if circular { (PI * c1, PI * (c1 + 4.0 * d)) } else { (u0_rect(pos, c1, c2, d), u1_rect(pos, c1, c2, d, e1, e2)) };
    let lh = c.o("capital_l", 0.0);
    if lh > 0.0 && !circular { u1 = u1_rect(pos, c1 + 2.0 * lh, c2 + 2.0 * lh, d, e1, e2); }
    if let Some(w) = c.opt("opening_w") {
        u1 -= opening_shadow(w, c.o("opening_a", 0.0), u1 / (2.0 * PI), c.o("opening_clear", 1.0e9), d);
    }
    let mut out = vec![Outcome::mm("u0", "Perimeter u_0", "Rundschnitt u_0", u0, "column face".into()), Outcome::mm("u1", "Control perimeter u_1", "Kontrollrundschnitt u_1", u1, "2d from the loaded area".into())];
    out.push(Outcome::mm("uout_dist", "Outer perimeter offset k d", "Abstand äußerer Rundschnitt k d", c.ndp("ndp.en1992.punch-k-out") * d, "6.4.5(4)".into()));
    Ok(out)
}

/// 🧱 Load increase factor β: simplified NDP value or Eq. 6.38.
fn beta_of(c: &Case, pos: Position, d: f64) -> Result<(f64, String), String> {
    if c.o("beta_computed", 0.0) > 0.0 {
        let (c1, c2) = (c.pos("c1")?, c.o("c2", c.pos("c1")?));
        let (med, ved) = (c.n("m_ed")?, c.pos("v_ed")?);
        let u1 = u1_rect(pos, c1, c2, d, c.o("e1", 0.0), c.o("e2", 0.0));
        let k = k_table_61(c1 / c2);
        return Ok(match pos {
            Position::Corner => {
                let basic = u1_rect(Position::Interior, c1, c2, d, 0.0, 0.0);
                (basic / u1, "β = u_1/u_1* (corner column)".into())
            }
            Position::Interior => (1.0 + k * med / ved * u1 / w1_interior(c1, c2, d), "β = 1 + k M_Ed u_1/(V_Ed W_1)".into()),
            _ => {
                let u_red = c.o("u1_reduced", u1);
                (u1 / u_red + k * u1 / w1_edge(c1, c2, d) * c.o("e_par", med / ved), "β = u_1/u_1* + k (u_1/W_1) e_par".into())
            }
        });
    }
    let sym = match pos { Position::Interior => "ndp.en1992.beta-interior", Position::Edge => "ndp.en1992.beta-edge", Position::Corner => "ndp.en1992.beta-corner", Position::WallEnd => "ndp.en1992.beta-wall-end" };
    Ok((c.ndp(sym).max(1.0), format!("NDP {sym}")))
}

pub fn p02(c: &Case) -> R {
    let d = c.pos("d")?;
    let pos = position_of(c.so("position", "interior"));
    let (b, why) = beta_of(c, pos, d)?;
    Ok(vec![Outcome::num("beta", "Load increase factor β", "Lasterhöhungsfaktor β", b.max(1.0), why)])
}

pub struct Punch {
    ved_stress: f64,
    vrdc: f64,
    u1: f64,
    d: f64,
    beta: f64,
    fcd: f64,
    fck: f64,
}

pub fn punch_base(c: &Case) -> Result<Punch, String> {
    let (fck, d, ved) = (c.pos("fck")?, c.pos("d")?, c.pos("v_ed")?);
    let c1 = c.pos("c1")?;
    let c2 = c.o("c2", c1);
    let pos = position_of(c.so("position", "interior"));
    let (beta, _) = beta_of(c, pos, d)?;
    let u1 = if c.o("circular", 0.0) > 0.0 { PI * (c1 + 4.0 * d) } else { u1_rect(pos, c1, c2, d, c.o("e1", 0.0), c.o("e2", 0.0)) };
    let u1 = if pos == Position::Corner && c.o("beta_computed", 0.0) > 0.0 { u1_rect(Position::Interior, c1, c2, d, 0.0, 0.0) } else { u1 };
    let u0_over_d = if c.o("circular", 0.0) > 0.0 { PI * c1 / d } else { u0_rect(pos, c1, c2, d) / d };
    let (vrdc, ..) = v_rd_c_punch(c.annex, fck, d, c.n("rho_ly")?, c.n("rho_lz")?, c.o("sigma_cp", 0.0), pos, u0_over_d);
    Ok(Punch { ved_stress: beta * (ved - c.o("delta_v", 0.0)) / (u1 * d), vrdc, u1, d, beta: beta.max(1.0), fcd: f_cd(c.annex, fck, false), fck })
}

pub fn p03(c: &Case) -> R {
    let p = punch_base(c)?;
    let mut out = vec![
        Outcome::mpa("v_ed", "Shear stress v_Ed", "Schubspannung v_Ed", p.ved_stress, format!("β = {}, u_1 = {}", f(p.beta), f(p.u1))),
        Outcome::stress("util", "Punching utilisation without reinforcement", "Durchstanzausnutzung ohne Bewehrung", p.ved_stress, p.vrdc, "v_Ed ≤ v_Rd,c".into()),
    ];
    let nu = 0.6 * (1.0 - p.fck / 250.0);
    if c.de() {
        out.push(Outcome::stress("util_max", "Punching crushing limit (NA-DE)", "Durchstanz-Druckstrebe (NA-DE)", p.ved_stress, c.ndp("ndp.en1992.punch-vrdmax-factor") * p.vrdc, "v_Ed ≤ 1.4 v_Rd,c".into()));
    } else {
        let u0 = u0_rect(position_of(c.so("position", "interior")), c.pos("c1")?, c.o("c2", c.pos("c1")?), p.d);
        out.push(Outcome::stress("util_max", "Punching crushing limit at u_0", "Durchstanz-Druckstrebe bei u_0", p.beta * c.pos("v_ed")? / (u0 * p.d), 0.5 * nu * p.fcd, "v_Ed ≤ 0.5 ν f_cd".into()));
    }
    if let Some(a) = c.opt("a") {
        if a < 2.0 * p.d {
            out.push(Outcome::stress("util_near", "Punching utilisation at perimeter a < 2d", "Durchstanzausnutzung bei Rundschnitt a < 2d", p.ved_stress, p.vrdc * 2.0 * p.d / a, "v_Rd,c 2d/a".into()));
        }
    }
    Ok(out)
}

/// 🧱 Crushing limit v_Rd,max and the stress it is compared with: DE 1.4 v_Rd,c (1.96 with approval) against v_Ed at u_1, EC 0.5 ν f_cd against v_Ed,0 = β V_Ed/(u_0 d).
fn crushing_limit(c: &Case, p: &Punch, studs: bool) -> Result<(f64, f64), String> {
    if c.de() {
        let f = c.ndp("ndp.en1992.punch-vrdmax-factor") * if studs && c.o("approval", 0.0) > 0.0 { 1.4 } else { 1.0 };
        Ok((f * p.vrdc, p.ved_stress))
    } else {
        let (c1, pos) = (c.pos("c1")?, position_of(c.so("position", "interior")));
        let u0 = u0_rect(pos, c1, c.o("c2", c1), p.d);
        Ok((0.5 * 0.6 * (1.0 - p.fck / 250.0) * p.fcd, p.beta * (c.pos("v_ed")? - c.o("delta_v", 0.0)) / (u0 * p.d)))
    }
}

pub fn p04(c: &Case) -> R {
    let p = punch_base(c)?;
    let fyk = c.o("fyk", 500.0);
    let fywd = f_yd(c.annex, fyk, false);
    let fywd_ef = (250.0 + 0.25 * p.d).min(fywd);
    let sr = c.o("sr", 0.75 * p.d);
    let st = c.o("st", 1.5 * p.d);
    let alpha = c.o("alpha_deg", 90.0).to_radians();
    let asw_req = ((p.ved_stress - 0.75 * p.vrdc) * p.u1 * p.d / (1.5 * (p.d / sr) * fywd_ef * alpha.sin())).max(0.0);
    let u_out = p.beta * c.pos("v_ed")? / (p.vrdc * p.d);
    let c1 = c.pos("c1")?;
    let c2 = c.o("c2", c1);
    let r_out = (u_out - 2.0 * (c1 + c2)) / (2.0 * PI);
    let r_last = r_out - c.ndp("ndp.en1992.punch-k-out") * p.d;
    let rings = if r_last <= 0.5 * p.d { 1.0 } else { ((r_last - 0.5 * p.d) / sr).ceil() + 1.0 };
    let asw_min = 0.08 * p.fck.sqrt() / fyk * sr * st / (1.5 * alpha.sin() + alpha.cos());
    let mut out = vec![
        Outcome::num("fywd_ef", "Effective stud strength f_ywd,ef (MPa)", "Wirksame Dübelfestigkeit f_ywd,ef (MPa)", fywd_ef, "250 + 0.25 d ≤ f_ywd".into()),
        Outcome::mm2("asw_req", "Required A_sw per perimeter", "Erforderliches A_sw je Dübelkreis", asw_req, "v_Ed = 0.75 v_Rd,c + 1.5 (d/s_r) A_sw f_ywd,ef/(u_1 d) sin α".into()),
        Outcome::mm2("asw_min", "Minimum A_sw per perimeter", "Mindest-A_sw je Dübelkreis", asw_min, "0.08 √f_ck/f_yk s_r s_t/(1.5 sin α + cos α)".into()),
        Outcome::mm("u_out", "Outer perimeter u_out", "Äußerer Rundschnitt u_out", u_out, "β V_Ed/(v_Rd,c d)".into()),
        Outcome::mm("r_out", "Distance r_out from the column face", "Abstand r_out von der Stützenkante", r_out, "(u_out − u_0)/(2π)".into()),
        Outcome::num("rings", "Minimum number of stud rings", "Mindestanzahl Dübelkreise", rings, "outermost ring within k d of r_out".into()),
        Outcome::length("sr", "Radial stud spacing", "Radialer Dübelabstand", sr, 0.75 * p.d, "s_r ≤ 0.75 d".into()),
        Outcome::length("st", "Tangential stud spacing", "Tangentialer Dübelabstand", st, 1.5 * p.d, "s_t ≤ 1.5 d inside 2d".into()),
    ];
    if let Some(asw) = c.opt("asw") {
        let vrdcs = 0.75 * p.vrdc + 1.5 * (p.d / sr) * asw * fywd_ef * alpha.sin() / (p.u1 * p.d);
        out.push(Outcome::stress("util", "Punching utilisation with reinforcement", "Durchstanzausnutzung mit Bewehrung", p.ved_stress, vrdcs, format!("v_Rd,cs = {}", f(vrdcs))));
        out.push(Outcome::area_min("asw_prov", "Provided A_sw against minimum", "Vorhandenes A_sw gegen Minimum", asw, asw_min, "A_sw ≥ A_sw,min".into()));
        let (max, shown) = crushing_limit(c, &p, true)?;
        out.push(Outcome::stress("util_max", "Punching crushing limit", "Durchstanz-Druckstrebe", shown, max, "v_Ed ≤ v_Rd,max (DE at u_1, EC at u_0)".into()));
    }
    Ok(out)
}

pub fn p05(c: &Case) -> R {
    let p = punch_base(c)?;
    let (max, shown) = crushing_limit(c, &p, c.o("approval", 0.0) > 0.0)?;
    Ok(vec![Outcome::stress("util_max", "Stud crushing limit", "Dübel-Druckstrebe", shown, max, format!("v_Rd,max = {} MPa", f(max)))])
}

pub fn p06(c: &Case) -> R {
    let (fck, d, ved, sg) = (c.pos("fck")?, c.pos("d")?, c.pos("v_ed")?, c.o("sigma_gd", 0.0));
    let (c1, c2) = (c.pos("c1")?, c.o("c2", c.pos("c1")?));
    let beta = c.o("beta", 1.0);
    let (vrdc, ..) = v_rd_c_punch(c.annex, fck, d, c.n("rho_ly")?, c.n("rho_lz")?, 0.0, Position::Interior, f64::INFINITY);
    let (mut worst, mut at) = (0.0f64, 0.0);
    let n = 40;
    for i in 0..=n {
        let a = 0.5 * d + 1.5 * d * i as f64 / n as f64;
        let u = 2.0 * (c1 + c2) + 2.0 * PI * a;
        let dv = sg * (c1 * c2 + 2.0 * a * (c1 + c2) + PI * a * a);
        let util = beta * (ved - dv) / (u * d) / (vrdc * 2.0 * d / a);
        if util > worst { worst = util; at = a; }
    }
    Ok(vec![Outcome::ratio("util", "Foundation punching utilisation (governing perimeter)", "Fundament-Durchstanzausnutzung (maßgebender Rundschnitt)", worst, 1.0, format!("governing a = {} mm", f(at))), Outcome::mm("a_governing", "Governing perimeter distance a", "Maßgebender Rundschnittabstand a", at, "0.5d … 2d".into())])
}

pub fn p07(c: &Case) -> R {
    let p = punch_base(c)?;
    let vp = c.o("p_force", 0.0) * c.o("tendon_angle", 0.0).sin();
    let sigma_cp = c.o("sigma_cp", 0.0);
    Ok(vec![Outcome::stress("util", "Prestressed flat slab punching", "Durchstanzen vorgespannte Flachdecke", (p.ved_stress - vp / (p.u1 * p.d)).max(0.0), p.vrdc, format!("σ_cp = {}, V_p = {}", f(sigma_cp), f(vp)))])
}

/// 🧱 Strut stress limit σ_Rd,max for the strut kind: `free` (no transverse tension, f_cd) or `cracked` (0.6 ν' f_cd, NDP).
pub fn strut_limit(annex: AnnexChoice, fck: f64, kind: &str) -> f64 {
    let fcd = f_cd(annex, fck, false);
    let nu = 1.0 - fck / 250.0;
    if kind == "cracked" { ndp(annex, "ndp.en1992.strut-cracked") * nu * fcd } else { fcd }
}

pub fn st01(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let lim = strut_limit(c.annex, fck, c.so("kind", "free"));
    let mut v = vec![Outcome::mpa("sigma_rd_max", "Strut capacity σ_Rd,max", "Druckstrebentragfähigkeit σ_Rd,max", lim, c.so("kind", "free").to_string())];
    if let (Some(f_c), Some(b), Some(w)) = (c.opt("strut_force"), c.opt("b"), c.opt("w")) { v.push(Outcome::stress("util", "Strut utilisation", "Druckstrebenausnutzung", f_c / (b * w), lim, "σ_c = F_c/(b w)".into())); }
    Ok(v)
}

pub fn st02(c: &Case) -> R {
    let ftd = c.pos("f_td")?;
    let req = ftd / f_yd(c.annex, c.o("fyk", 500.0), false);
    let mut v = vec![Outcome::mm2("as_tie", "Required tie reinforcement", "Erforderliche Zugstrebenbewehrung", req, "F_td/f_yd".into())];
    if let Some(p) = c.opt("as_provided") { v.push(Outcome::area("util", "Tie utilisation", "Zugstrebenausnutzung", req, p, "A_s,req ≤ A_s,prov".into())); }
    Ok(v)
}

pub fn st03(c: &Case) -> R {
    let fck = c.pos("fck")?;
    let fcd = f_cd(c.annex, fck, false);
    let nu = 1.0 - fck / 250.0;
    let (sym, name) = match c.so("node", "CCC") { "CCT" => ("ndp.en1992.node-k2", "CCT"), "CTT" => ("ndp.en1992.node-k3", "CTT"), _ => ("ndp.en1992.node-k1", "CCC") };
    let conf = c.o("confinement", 1.0).clamp(1.0, 3.0);
    let lim = c.ndp(sym) * nu * fcd * conf;
    let mut v = vec![Outcome::mpa("sigma_rd_max", &format!("Node capacity {name}"), &format!("Knotentragfähigkeit {name}"), lim, format!("k = {}", f(c.ndp(sym))))];
    if let Some(s) = c.opt("sigma_c") { v.push(Outcome::stress("util", "Nodal stress utilisation", "Knotenspannungsausnutzung", s, lim, "σ_c ≤ σ_Rd,max".into())); }
    Ok(v)
}

/// 🧮 Statically determinate (or least-squares) plane truss: member axial forces, tension positive.
pub fn solve_truss(x: &[f64], y: &[f64], mi: &[usize], mj: &[usize], fx: &[f64], fy: &[f64], sup: &[(usize, usize)]) -> Option<Vec<f64>> {
    let (n, m, r) = (x.len(), mi.len(), sup.len());
    if 2 * n != m + r { return None; }
    let mut a = vec![vec![0.0; m + r]; 2 * n];
    for k in 0..m {
        let (i, j) = (mi[k], mj[k]);
        let (dx, dy) = (x[j] - x[i], y[j] - y[i]);
        let l = (dx * dx + dy * dy).sqrt();
        a[2 * i][k] += dx / l;
        a[2 * i + 1][k] += dy / l;
        a[2 * j][k] -= dx / l;
        a[2 * j + 1][k] -= dy / l;
    }
    for (k, (nd, dof)) in sup.iter().enumerate() { a[2 * nd + dof][m + k] = 1.0; }
    let mut b: Vec<f64> = (0..n).flat_map(|i| [-fx[i], -fy[i]]).collect();
    let dim = 2 * n;
    for col in 0..dim {
        let piv = (col..dim).max_by(|&p, &q| a[p][col].abs().partial_cmp(&a[q][col].abs()).unwrap_or(std::cmp::Ordering::Equal))?;
        if a[piv][col].abs() < 1e-12 { return None; }
        a.swap(col, piv);
        b.swap(col, piv);
        for row in 0..dim {
            if row != col {
                let fct = a[row][col] / a[col][col];
                for k in col..dim { let v = a[col][k]; a[row][k] -= fct * v; }
                b[row] -= fct * b[col];
            }
        }
    }
    Some((0..m).map(|k| b[k] / a[k][k]).collect())
}

pub fn st04(c: &Case) -> R {
    let (x, y, mi, mj) = (c.l("node_x")?, c.l("node_y")?, c.l("member_i")?, c.l("member_j")?);
    let zero = vec![0.0; x.len()];
    let fxv = c.l("load_x").map(|v| v.to_vec()).unwrap_or(zero.clone());
    let fyv = c.l("load_y").map(|v| v.to_vec()).unwrap_or(zero);
    let sn = c.l("support_node")?;
    let sd = c.l("support_dof")?;
    let sup: Vec<(usize, usize)> = sn.iter().zip(sd.iter()).map(|(a, b)| (*a as usize, *b as usize)).collect();
    let mi: Vec<usize> = mi.iter().map(|v| *v as usize).collect();
    let mj: Vec<usize> = mj.iter().map(|v| *v as usize).collect();
    let forces = solve_truss(x, y, &mi, &mj, &fxv, &fyv, &sup).ok_or("truss is not statically determinate or unstable")?;
    let fyd = f_yd(c.annex, c.o("fyk", 500.0), false);
    let mut out = Vec::new();
    for (k, fv) in forces.iter().enumerate() {
        let (i, j) = (mi[k] as usize, mj[k] as usize);
        let theta = ((y[j] - y[i]).abs()).atan2((x[j] - x[i]).abs()).to_degrees();
        out.push(Outcome::n(&format!("force_{k}"), &format!("Member {k} axial force"), &format!("Stab {k} Normalkraft"), *fv, if *fv >= 0.0 { format!("tie, θ = {}°", f(theta)) } else { format!("strut, θ = {}°", f(theta)) }));
        if *fv > 0.0 { out.push(Outcome::mm2(&format!("as_{k}"), &format!("Tie {k} required area"), &format!("Zugstrebe {k} erforderliche Fläche"), fv / fyd, "F/f_yd".into())); }
    }
    Ok(out)
}

pub fn pa01(c: &Case) -> R {
    let (fck, a0, a1) = (c.pos("fck")?, c.pos("ac0")?, c.pos("ac1")?);
    let fcd = f_cd(c.annex, fck, false);
    let frdu = (a0 * fcd * (a1 / a0).sqrt()).min(3.0 * fcd * a0);
    let mut v = vec![Outcome::n("f_rdu", "Bearing resistance F_Rdu", "Teilflächentragfähigkeit F_Rdu", frdu, format!("A_c1/A_c0 = {}", f(a1 / a0)))];
    if let Some(fed) = c.opt("f_ed") { v.push(Outcome::force("util", "Bearing utilisation", "Teilflächenausnutzung", fed, frdu, "F_Ed ≤ F_Rdu".into())); }
    if let (Some(fed), Some(a), Some(h)) = (c.opt("f_ed"), c.opt("a"), c.opt("h")) {
        let k = if c.o("full_discontinuity", 0.0) > 0.0 { 0.7 } else { 1.0 };
        let t = 0.25 * (1.0 - k * a / h) * fed;
        v.push(Outcome::n("t_split", "Splitting tension T", "Spaltzugkraft T", t, "0.25 (1 − k a/h) F".into()));
        v.push(Outcome::mm2("as_split", "Splitting reinforcement", "Spaltzugbewehrung", t / f_yd(c.annex, c.o("fyk", 500.0), false), "T/f_yd".into()));
    }
    Ok(v)
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("P-01", "DIN EN 1992-1-1", "6.4.2", "Basic control perimeters", "Rundschnitte", &["c1", "c2", "d", "position", "e1", "e2", "circular", "capital_l", "opening_w", "opening_a", "opening_clear"], &["ndp.en1992.punch-k-out"], p01),
    entry_of("P-02", "DIN EN 1992-1-1", "6.4.3", "Load increase factor beta", "Lasterhöhungsfaktor beta", &["c1", "c2", "d", "position", "beta_computed", "m_ed", "v_ed", "e_par", "u1_reduced"], &["ndp.en1992.beta-interior", "ndp.en1992.beta-edge", "ndp.en1992.beta-corner", "ndp.en1992.beta-wall-end"], p02),
    entry_of("P-03", "DIN EN 1992-1-1", "6.4.4", "Punching without shear reinforcement", "Durchstanzen ohne Durchstanzbewehrung", &["fck", "d", "v_ed", "c1", "c2", "position", "rho_ly", "rho_lz", "sigma_cp", "delta_v", "a"], &["ndp.en1992.crd-c-punch", "ndp.en1992.k1-punch", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.punch-vrdmax-factor", "ndp.en1992.beta-interior", "ndp.en1992.beta-edge", "ndp.en1992.beta-corner", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], p03),
    entry_of("P-04", "DIN EN 1992-1-1", "6.4.5", "Punching shear reinforcement", "Durchstanzbewehrung", &["fck", "d", "v_ed", "c1", "c2", "position", "rho_ly", "rho_lz", "fyk", "sr", "st", "asw", "alpha_deg"], &["ndp.en1992.crd-c-punch", "ndp.en1992.k1-punch", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.gamma-s", "ndp.en1992.punch-vrdmax-factor", "ndp.en1992.punch-k-out", "ndp.en1992.beta-interior", "ndp.en1992.beta-edge", "ndp.en1992.beta-corner", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], p04),
    entry_of("P-05", "DIN EN 1992-1-1", "6.4.5(4)", "Shear studs with approval", "Dübelleiste mit Zulassung", &["fck", "d", "v_ed", "c1", "rho_ly", "rho_lz", "approval"], &["ndp.en1992.punch-vrdmax-factor", "ndp.en1992.crd-c-punch", "ndp.en1992.k1-punch", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.beta-interior", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], p05),
    entry_of("P-06", "DIN EN 1992-1-1", "6.4.4(2)", "Punching of foundations", "Durchstanzen von Fundamenten", &["fck", "d", "v_ed", "c1", "c2", "rho_ly", "rho_lz", "sigma_gd", "beta"], &["ndp.en1992.crd-c-punch", "ndp.en1992.k1-punch", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], p06),
    entry_of("P-07", "DIN EN 1992-1-1", "6.4.4", "Prestressed flat slab punching", "Durchstanzen vorgespannte Flachdecke", &["fck", "d", "v_ed", "c1", "rho_ly", "rho_lz", "sigma_cp", "p_force", "tendon_angle"], &["ndp.en1992.crd-c-punch", "ndp.en1992.k1-punch", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.beta-interior", "ndp.en1992.vmin-shear", "ndp.en1992.vmin-shear-d800"], p07),
    entry_of("ST-01", "DIN EN 1992-1-1", "6.5.2", "Concrete strut capacity", "Druckstreben", &["fck", "kind", "strut_force", "b", "w"], &["ndp.en1992.strut-cracked", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], st01),
    entry_of("ST-02", "DIN EN 1992-1-1", "6.5.3", "Ties", "Zugstreben", &["f_td", "fyk", "as_provided"], &["ndp.en1992.gamma-s"], st02),
    entry_of("ST-03", "DIN EN 1992-1-1", "6.5.4", "Nodes", "Knoten", &["fck", "node", "confinement", "sigma_c"], &["ndp.en1992.node-k1", "ndp.en1992.node-k2", "ndp.en1992.node-k3", "ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent"], st03),
    entry_of("ST-04", "DIN EN 1992-1-1", "6.5.1", "Strut-and-tie model solver", "Stabwerkmodell-Löser", &["node_x", "node_y", "member_i", "member_j", "load_x", "load_y", "support_node", "support_dof", "fyk"], &["ndp.en1992.gamma-s"], st04),
    entry_of("PA-01", "DIN EN 1992-1-1", "6.7", "Partially loaded areas", "Teilflächenbelastung", &["fck", "ac0", "ac1", "f_ed", "a", "h", "full_discontinuity", "fyk"], &["ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-s"], pa01),
];
