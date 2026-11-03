//! 🪚️ Members with varying section or curved shape and notches of EN 1995-1-1 6.4 and 6.5 as catalogue entries T-30 … T-33, T-45, T-46. Units: N, mm, N/mm². See [`super`].

use super::*;

/// 🧮 Tapered-beam factor k_m,α of EN 1995-1-1 6.38 (tension edge tapered) and 6.39 (compression edge tapered).
pub fn km_alpha(tension_side: bool, alpha_rad: f64, fmd: f64, fvd: f64, f_perp_d: f64) -> f64 {
    let t = alpha_rad.tan();
    let shear = if tension_side { 0.75 } else { 1.5 };
    1.0 / (1.0 + (fmd / (shear * fvd) * t).powi(2) + (fmd / f_perp_d * t * t).powi(2)).sqrt()
}

/// 📈 The bending-moment line of a member, normalised to its peak and signed relative to it (positive where the moment has the sign of the peak, negative where it turns against it), solved by [`semio_s_plugin_norm_mechanics::MomentLine`] for a uniform load plus a point load at the middle of each span (at the tip of a cantilever); `span` is the member length in mm and the coordinate `xi` in 0..1 runs over the whole member from the left support or the fixed end. See EN 1995-1-1 6.4.2 (the critical section of a varying section depends on the moment line).
#[derive(Clone, Debug)]
pub struct MomentLine {
    pub span: f64,
    line: semio_s_plugin_norm_mechanics::MomentLine,
    peak: f64,
}

fn peak_of(line: &semio_s_plugin_norm_mechanics::MomentLine) -> f64 {
    (0..line.spans().len()).flat_map(|j| {
        let (lo, hi) = line.extremes(j);
        [lo, hi]
    }).fold(0.0, |best, m| if m.abs() > best.abs() { m } else { best })
}

impl MomentLine {
    /// 📈 The line of `spans` equal spans of `l` mm with `share` of the peak moment stemming from the point loads and the rest from the uniform load (each part scaled to unit peak); `None` for no span or a non-positive `l`.
    pub fn build(supports: BeamSupports, spans: usize, l: f64, share: f64) -> Option<Self> {
        use semio_s_plugin_norm_mechanics::{MomentLine as Statics, PointLoad, SpanLoad};
        if spans == 0 || !(l.is_finite() && l > 0.0) {
            return None;
        }
        let at = if supports == BeamSupports::Cantilever { l } else { 0.5 * l };
        let solve = |udl: f64, force: f64| {
            let load = SpanLoad { udl, points: if force == 0.0 { Vec::new() } else { vec![PointLoad { at, force }] } };
            Statics::solve(&vec![l; spans], &vec![load; spans], supports).ok()
        };
        let share = share.clamp(0.0, 1.0);
        let (udl, point) = (peak_of(&solve(1.0, 0.0)?).abs(), peak_of(&solve(0.0, 1.0)?).abs());
        let line = solve((1.0 - share) / udl, share / point)?;
        let peak = peak_of(&line);
        Some(Self { span: l * spans as f64, line, peak })
    }

    /// 📈 The line of a `load_case` of the catalogue input of that name (`udl_ss` and `udl` simply supported, `point_mid`, `cantilever_udl`, `cantilever_point`, `two_span_udl` and `two_span_point` for two equal spans); `share` is the part of the peak moment from a point load for the uniform cases; `None` for an unknown case.
    pub fn of(load_case: &str, share: f64, l: f64) -> Option<Self> {
        let (supports, spans) = match load_case {
            "udl_ss" | "udl" | "point_mid" => (BeamSupports::Pinned, 1),
            "cantilever_udl" | "cantilever_point" => (BeamSupports::Cantilever, 1),
            "two_span_udl" | "two_span_point" => (BeamSupports::Pinned, 2),
            _ => return None,
        };
        Self::build(supports, spans, l, if load_case.ends_with("point") || load_case == "point_mid" { 1.0 } else { share })
    }

    /// 📈 The moment at `xi` of the member length relative to the peak, negative where it turns against it.
    pub fn signed(&self, xi: f64) -> f64 {
        self.line.moment_at(xi.clamp(0.0, 1.0) * self.line.length()) / self.peak
    }
}

/// 📈 `M(xi) / M_peak` of [`MomentLine::of`] (unit span) for a `load_case`.
pub fn moment_signed(load_case: &str, point_share: f64, xi: f64) -> Option<f64> {
    MomentLine::of(load_case, point_share, 1.0).map(|line| line.signed(xi))
}

/// 📈 `|M(xi)| / |M|max` of [`moment_signed`].
pub fn moment_shape(load_case: &str, point_share: f64, xi: f64) -> Option<f64> {
    moment_signed(load_case, point_share, xi).map(f64::abs)
}

/// 🪚️ One section of a tapered beam at distance `x` from the shallow end (mm, N/mm², N·mm): depth, moment, edge stress, bending strength at that depth and the utilisations of the tapered and of the parallel edge.
#[derive(Clone, Copy, Debug)]
pub struct TaperSection {
    pub x: f64,
    pub tension: bool,
    pub h: f64,
    pub m: f64,
    pub sigma: f64,
    pub fmd: f64,
    pub k: f64,
    pub tapered: f64,
    pub parallel: f64,
}

/// 🪚️ Design strengths and edge stress of a T-30 case in N/mm² (0 without `m_d`): the inputs of the k_m,α curve and of the section sweep. `sd`, `fmd` and `h` are those of the shallow section at x = 0; `tension` says whether the tapered edge is in tension where the moment has the sign of its peak, it is the other state where the moment line turns against the peak (the fields of a continuous beam), so both edge states are swept. The case must carry the moment `line` (span `l`, each span of a continuous beam, and `load_case`): a constant moment is not a moment line and has no fallback.
#[derive(Clone, Debug)]
pub struct TaperProbe {
    pub tension: bool,
    pub fmd: f64,
    pub fvd: f64,
    pub f_perp_d: f64,
    pub f_t90_d: f64,
    pub f_c90_d: f64,
    pub sd: f64,
    pub alpha_deg: f64,
    pub b: f64,
    pub h: f64,
    pub m_d: f64,
    pub line: MomentLine,
    product: TimberProduct,
    rho_k: f64,
    s_lvl: f64,
    kh_fixed: Option<f64>,
    fm_d: f64,
}

/// 🪚️ Grid points of the section sweep before the local refinement.
const TAPER_SWEEP: usize = 2000;

impl TaperProbe {
    fn fmd_at(&self, h: f64) -> f64 {
        self.kh_fixed.unwrap_or_else(|| size_factor_kh(self.product, self.rho_k, h, self.s_lvl)) * self.fm_d
    }

    /// 🪚️ Moment magnitude at distance `x` from the shallow end in N·mm.
    pub fn moment_at(&self, x: f64) -> f64 {
        self.m_d * self.line.signed(x / self.line.span).abs()
    }

    /// 🪚️ The section at distance `x` (depth h + x tan α): σ = 6 M / (b h²) of EN 1995-1-1 6.38 against k_m,α f_m,d with k_h and f_m,d of that depth, the edge state of the tapered edge (tension where the moment keeps the sign of its peak, else compression, 6.38/6.39), and against f_m,d (6.37).
    pub fn section(&self, x: f64) -> TaperSection {
        let h = self.h + x * self.alpha_deg.to_radians().tan();
        let (m, fmd) = (self.moment_at(x), self.fmd_at(h));
        let tension = self.tension == (self.line.signed(x / self.line.span) >= 0.0);
        let sigma = 6.0 * m / (self.b * h * h);
        let k = km_alpha(tension, self.alpha_deg.to_radians(), fmd, self.fvd, if tension { self.f_t90_d } else { self.f_c90_d });
        TaperSection { x, h, m, sigma, fmd, k, tension, tapered: sigma / (k * fmd), parallel: sigma / fmd }
    }

    /// 🪚️ The governing section: the one with the highest tapered-edge utilisation along the member (grid, then golden-section refinement; the grid carries the kinks of the moment line at midspan loads and supports).
    pub fn governing(&self) -> TaperSection {
        let step = self.line.span / TAPER_SWEEP as f64;
        let at = |i: usize| self.section(step * i as f64).tapered;
        let best = (0..=TAPER_SWEEP).fold(0, |b, i| if at(i) > at(b) { i } else { b });
        let (mut lo, mut hi) = (step * best.saturating_sub(1) as f64, (step * (best + 1) as f64).min(self.line.span));
        let phi = 0.5 * (5.0_f64.sqrt() - 1.0);
        for _ in 0..80 {
            let (a, b) = (hi - phi * (hi - lo), lo + phi * (hi - lo));
            if self.section(a).tapered < self.section(b).tapered { lo = a } else { hi = b }
        }
        let refined = self.section(0.5 * (lo + hi));
        let edge = self.section(step * best as f64);
        if refined.tapered >= edge.tapered { refined } else { edge }
    }
}

/// 🪚️ The [`TaperProbe`] of a T-30 case; a case without the span `l` and a known `load_case` is refused (no constant-moment fallback).
pub fn taper_probe(c: &Case) -> Result<TaperProbe, String> {
    let m = Mat::load(c)?;
    let (b, h) = (c.n("b")?, c.n("h")?);
    let tension = c.so("side", "tension") == "tension";
    let load_case = c.so("load_case", "");
    let line = c
        .opt("l")
        .and_then(|l| MomentLine::of(load_case, c.o("point_share", 0.0), l))
        .ok_or("the moment line of the member is unknown: give the span l and the load case")?;
    let m_d = c.o("m_d", 0.0);
    let (f_t90_d, f_c90_d) = (m.fd(m.row.f_t_90_k), m.fd(m.row.f_c_90_k));
    Ok(TaperProbe {
        tension,
        fmd: m.kh_bending(c, h) * m.fd(m.row.f_m_k),
        fvd: m.fd(m.row.f_v_k),
        f_perp_d: if tension { f_t90_d } else { f_c90_d },
        f_t90_d,
        f_c90_d,
        sd: 6.0 * m_d / (b * h * h),
        alpha_deg: c.n("alpha_deg")?,
        b,
        h,
        m_d,
        line,
        product: m.row.product,
        rho_k: m.row.rho_k,
        s_lvl: c.o("s_lvl", 0.12),
        kh_fixed: c.opt("kh"),
        fm_d: m.fd(m.row.f_m_k),
    })
}

fn t30(c: &Case) -> R {
    let p = taper_probe(c)?;
    let g = p.governing();
    let (sd, fmd, k) = (g.sigma, g.fmd, g.k);
    let at = format!(" at x = {} mm, h = {} mm, M = {} N·mm", f(g.x), f(g.h), f(g.m));
    let edge = if g.tension { "tension" } else { "compression" };
    Ok(vec![
        Outcome::stress("tapered_edge", "Tapered edge stress (6.38/6.39)", "Spannung am angeschnittenen Rand (6.38/6.39)", sd, k * fmd, format!("σ_m,α,d = {} ≤ k_m,α·f_m,d = {}·{} ({edge} edge){at}", f(sd), f(k), f(fmd))),
        Outcome::stress("parallel_edge", "Parallel edge stress (6.37)", "Spannung am parallelen Rand (6.37)", sd, fmd, format!("σ_m,0,d = {} ≤ f_m,d = {}{at}", f(sd), f(fmd))),
        Outcome::ratio("taper_angle", "Taper angle α ≤ 10°", "Neigungswinkel α ≤ 10°", p.alpha_deg, 10.0, "α in degrees".into()),
        Outcome::info("governing_section", "Governing section x from the shallow end", "Maßgebender Schnitt x vom flachen Ende", QuantityKind::Length, g.x / 1e3, format!("x = {} mm of {} mm (moment line of the load case, EN 1995-1-1 6.4.2)", f(g.x), f(p.line.span))),
    ])
}

/// 🧮 Apex bending factor k_l of EN 1995-1-1 6.43 – 6.47 for pitch angle α_ap and ratio h_ap/r.
pub fn apex_kl(alpha_ap: f64, hr: f64) -> f64 {
    let t = alpha_ap.tan();
    let (k1, k2, k3, k4) = (1.0 + 1.4 * t + 5.4 * t * t, 0.35 - 8.0 * t, 0.6 + 8.3 * t - 7.8 * t * t, 6.0 * t * t);
    k1 + k2 * hr + k3 * hr * hr + k4 * hr.powi(3)
}

/// 🧮 Apex tension-perpendicular factor k_p of EN 1995-1-1 6.48 – 6.51.
pub fn apex_kp(alpha_ap: f64, hr: f64) -> f64 {
    let t = alpha_ap.tan();
    let (k5, k6, k7) = (0.2 * t, 0.25 - 1.5 * t + 2.6 * t * t, 2.1 * t - 4.0 * t * t);
    k5 + k6 * hr + k7 * hr * hr
}

/// 🧮 Curvature factor k_r of EN 1995-1-1 6.49: 1 for r_in/t ≥ 240, otherwise 0.76 + 0.001·r_in/t.
pub fn apex_kr(r_in: f64, t: f64) -> f64 {
    if r_in / t >= 240.0 { 1.0 } else { 0.76 + 0.001 * r_in / t }
}

struct Apex {
    hr: f64,
    alpha: f64,
}

fn apex(c: &Case) -> Result<Apex, String> {
    let (h_ap, r_in) = (c.n("h_ap")?, c.n("r_in")?);
    Ok(Apex { hr: h_ap / (r_in + 0.5 * h_ap), alpha: c.o("alpha_ap_deg", 0.0).to_radians() })
}

fn t31(c: &Case) -> R {
    let m = Mat::load(c)?;
    let ap = apex(c)?;
    let (b, h_ap) = (c.n("b")?, c.n("h_ap")?);
    let kl = apex_kl(ap.alpha, ap.hr);
    let kr = apex_kr(c.n("r_in")?, c.n("t_lam")?);
    let sd = kl * 6.0 * c.n("m_ap")? / (b * h_ap * h_ap);
    let fmd = m.kh_bending(c, h_ap) * m.fd(m.row.f_m_k);
    Ok(vec![Outcome::stress("apex_bending", "Apex bending stress (6.41)", "Biegespannung im Firstbereich (6.41)", sd, kr * fmd, format!("h_ap/r={} k_l={} k_r={} σ_m,d={} ≤ k_r f_m,d={}", f(ap.hr), f(kl), f(kr), f(sd), f(kr * fmd)))])
}

fn apex_volume(c: &Case, ap: &Apex) -> Result<f64, String> {
    let (b, h_ap) = (c.n("b")?, c.n("h_ap")?);
    let v_zone = c.opt("v_apex").unwrap_or(b * h_ap * h_ap * (1.0 - 0.25 * ap.alpha.tan()));
    Ok(v_zone.min(2.0 / 3.0 * c.n("v_beam")?))
}

fn t32(c: &Case) -> R {
    let m = Mat::load(c)?;
    let ap = apex(c)?;
    let (b, h_ap) = (c.n("b")?, c.n("h_ap")?);
    let pitched_cambered = c.so("shape", "double_tapered") == "pitched_cambered";
    let kdis = if pitched_cambered { 1.7 } else { 1.4 };
    let kp = apex_kp(ap.alpha, ap.hr);
    let v = apex_volume(c, &ap)?;
    let kvol = (1e7 / v).powf(0.2);
    let mut sd = kp * 6.0 * c.n("m_ap")? / (b * h_ap * h_ap);
    if pitched_cambered {
        sd -= 0.6 * c.o("p_d", 0.0) / b;
    }
    let fd = kdis * kvol * m.fd(m.row.f_t_90_k);
    let mut out = vec![Outcome::stress("apex_tension90", "Apex tension perpendicular (6.50)", "Querzug im Firstbereich (6.50)", sd, fd, format!("k_p={} k_dis={} k_vol={} V={} mm³ σ_t,90,d={} ≤ {}", f(kp), f(kdis), f(kvol), f(v), f(sd), f(fd)))];
    if let Some(tau) = c.opt("tau_d") {
        let u = tau / m.fd(m.row.f_v_k) + sd / fd;
        out.push(Outcome::ratio("apex_shear_tension", "Shear and tension perpendicular (6.53)", "Schub und Querzug (6.53)", u, 1.0, format!("τ_d/f_v,d + σ_t,90,d/(k_dis k_vol f_t,90,d) = {}", f(u))));
    }
    Ok(out)
}

fn t33(c: &Case) -> R {
    let m = Mat::load(c)?;
    let ap = apex(c)?;
    let (b, h_ap) = (c.n("b")?, c.n("h_ap")?);
    let sd = apex_kp(ap.alpha, ap.hr) * 6.0 * c.n("m_ap")? / (b * h_ap * h_ap);
    let ft = 0.5 * sd * b * h_ap;
    let r_ax = c.n("n_rods")? * c.n("r_ax_k")? * m.kmod / c.o("gamma_m_conn", 1.3);
    Ok(vec![Outcome::force("apex_reinforcement", "Apex reinforcement tension force", "Zugkraft der Firstverstärkung", ft, r_ax, format!("F_t,90,d = 0.5·σ_t,90,d·b·h_ap = {} N ≤ ΣR_ax,d = {} N", f(ft), f(r_ax)))])
}

/// 🧮 Notch factor k_v of EN 1995-1-1 6.62 for a notch on the support side; 1.0 otherwise.
pub fn notch_kv(h: f64, alpha: f64, x: f64, incl: f64, kn: f64, support_side: bool) -> f64 {
    if !support_side || alpha >= 1.0 {
        return 1.0;
    }
    let sh = h.sqrt();
    (kn * (1.0 + 1.1 * incl.powf(1.5) / sh) / (sh * ((alpha * (1.0 - alpha)).sqrt() + 0.8 * x / h * (1.0 / alpha - alpha * alpha).sqrt()))).min(1.0)
}

fn kn_of(product: TimberProduct) -> f64 {
    match product {
        TimberProduct::Lvl => 4.5,
        TimberProduct::Glulam => 6.5,
        _ => 5.0,
    }
}

/// ✂️ Geometry and shear strength of a T-45 case (`v_d` 0 when absent): the inputs of the k_v curve (N, mm, N/mm²).
#[derive(Clone, Copy, Debug)]
pub struct NotchProbe {
    pub b: f64,
    pub h: f64,
    pub hef: f64,
    pub x: f64,
    pub incl: f64,
    pub kn: f64,
    pub support: bool,
    pub kcr: f64,
    pub fvd: f64,
    pub v_d: f64,
}

/// ✂️ The [`NotchProbe`] of a T-45 case.
pub fn notch_probe(c: &Case) -> Result<NotchProbe, String> {
    let m = Mat::load(c)?;
    let (b, h, hef) = (c.n("b")?, c.n("h")?, c.n("h_ef")?);
    let alpha = hef / h;
    if !(alpha > 0.0 && alpha <= 1.0) || c.o("x", 0.0) < 0.0 {
        return Err("notch geometry requires 0 < h_ef/h ≤ 1 and x ≥ 0".into());
    }
    let kcr = c.opt("k_cr").unwrap_or_else(|| k_cr_factor(c.annex, m.row.product, m.row.f_v_k));
    Ok(NotchProbe { b, h, hef, x: c.o("x", 0.0), incl: c.o("incl", 0.0), kn: kn_of(m.row.product), support: c.so("side", "support") == "support", kcr, fvd: m.fd(m.row.f_v_k), v_d: c.o("v_d", 0.0) })
}

impl NotchProbe {
    /// ✂️ The notch factor k_v at the remaining depth ratio `alpha` = h_ef/h.
    pub fn kv(&self, alpha: f64) -> f64 {
        notch_kv(self.h, alpha, self.x, self.incl, self.kn, self.support)
    }
}

/// ✂️ The NA-DE bounds of an unreinforced support-side notch: h_ef ≥ 0.5·h and x ≤ 0.4·h.
pub const NOTCH_NA_MIN_RATIO: f64 = 0.5;

fn t45(c: &Case) -> R {
    let p = notch_probe(c)?;
    let alpha = p.hef / p.h;
    let kv = p.kv(alpha);
    let tau = 1.5 * p.v_d / (p.kcr * p.b * p.hef);
    let mut out = vec![Outcome::stress("notch_shear", "Notch shear (6.60)", "Schub an der Ausklinkung (6.60)", tau, kv * p.fvd, format!("α={} k_v={} k_cr={} τ_d={} ≤ k_v f_v,d = {}", f(alpha), f(kv), f(p.kcr), f(tau), f(kv * p.fvd)))];
    if c.de() {
        out.push(Outcome::ratio("notch_depth_na", "NA-DE notch depth h_ef ≥ 0.5 h", "NA-DE Ausklinkungstiefe h_ef ≥ 0,5 h", NOTCH_NA_MIN_RATIO, alpha, "unreinforced support-side notch".into()));
        out.push(Outcome::ratio("notch_position_na", "NA-DE notch position x ≤ 0.4 h", "NA-DE Lage der Ausklinkung x ≤ 0,4 h", p.x, 0.4 * p.h, "x/h".into()));
    }
    Ok(out)
}

/// 🧮 Tension force at a support notch F_t,90,d = 1.3·V_d·[3(1−α)² − 2(1−α)³] (DIN EN 1995-1-1/NA NCI to 6.5.2).
pub fn notch_ft90(v_d: f64, alpha: f64) -> f64 {
    let a = 1.0 - alpha;
    1.3 * v_d * (3.0 * a * a - 2.0 * a.powi(3))
}

fn t46(c: &Case) -> R {
    let m = Mat::load(c)?;
    let alpha = c.n("h_ef")? / c.n("h")?;
    let ft = notch_ft90(c.n("v_d")?, alpha);
    let r = c.n("n_rods")? * c.n("r_ax_k")? * m.kmod / c.o("gamma_m_conn", 1.3);
    Ok(vec![Outcome::force("notch_reinforcement", "Reinforced notch tension force", "Zugkraft verstärkte Ausklinkung", ft, r, format!("F_t,90,d = {} N ≤ ΣR_ax,d = {} N", f(ft), f(r)))])
}

/// 🗂 Catalogue rows of the varying-section and notch group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-30", part: "EN 1995-1-1", clause: "6.4.2", en: "Single tapered beam", de: "Einfach geneigter Träger", inputs: &["class", "m_d", "b", "h", "alpha_deg", "side", "l", "load_case", "point_share"], eval: t30 },
    Entry { id: "T-31", part: "EN 1995-1-1", clause: "6.4.3", en: "Double tapered, curved and pitched cambered beam", de: "Satteldach-, gekrümmter und gekrümmter Satteldachträger", inputs: &["class", "m_ap", "b", "h_ap", "r_in", "t_lam", "alpha_ap_deg"], eval: t31 },
    Entry { id: "T-32", part: "EN 1995-1-1", clause: "6.4.3", en: "Tension perpendicular at apex", de: "Querzug im Firstbereich", inputs: &["class", "m_ap", "b", "h_ap", "r_in", "alpha_ap_deg", "v_beam", "shape", "p_d", "tau_d"], eval: t32 },
    Entry { id: "T-33", part: "EN 1995-1-1", clause: "6.4.3", en: "Reinforced apex zone", de: "Verstärkter Firstbereich", inputs: &["class", "m_ap", "b", "h_ap", "r_in", "n_rods", "r_ax_k"], eval: t33 },
    Entry { id: "T-45", part: "EN 1995-1-1", clause: "6.5.2", en: "Notch at support", de: "Ausklinkung am Auflager", inputs: &["class", "v_d", "b", "h", "h_ef", "x", "incl", "side"], eval: t45 },
    Entry { id: "T-46", part: "EN 1995-1-1", clause: "6.5.2", en: "Reinforced notch", de: "Verstärkte Ausklinkung", inputs: &["class", "v_d", "h", "h_ef", "n_rods", "r_ax_k"], eval: t46 },
];
