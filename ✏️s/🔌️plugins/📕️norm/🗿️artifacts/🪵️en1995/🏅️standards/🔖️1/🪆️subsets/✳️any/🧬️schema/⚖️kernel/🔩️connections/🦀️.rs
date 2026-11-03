//! 🔩️ Connections of EN 1995-1-1 chapter 8 as catalogue entries T-70 … T-83: embedment and yield moment, Johansen European Yield Model, spacings, effective number, axial resistance, combined loading, splitting, block shear, connectors and stiffness. Units: N, mm, N/mm². See [`super`].

use super::*;
use std::f64::consts::PI;

/// 🔩 Fastener kinds understood by the connection entries.
fn kind_of(c: &Case) -> &str {
    c.so("kind", "dowel")
}

/// 🧮 Embedment strength f_h,k in N/mm² (EN 1995-1-1 8.15 – 8.33): nails, staples, bolts/dowels/screws, plywood and OSB/particleboard.
pub fn embedment_fh(kind: &str, material: &str, rho_k: f64, d: f64, alpha_deg: f64, wood: &str, t_panel: f64) -> f64 {
    match (material, kind) {
        ("plywood", _) => 0.11 * rho_k * d.powf(-0.3),
        ("osb" | "particleboard", _) => 50.0 * d.powf(-0.6) * t_panel.powf(0.2),
        (_, "nail" | "staple") if d <= 8.0 => 0.082 * rho_k * d.powf(-0.3),
        (_, "nail_predrilled") if d <= 8.0 => 0.082 * (1.0 - 0.01 * d) * rho_k,
        (_, "screw") if d <= 6.0 => 0.082 * rho_k * d.powf(-0.3),
        _ => {
            let f0 = 0.082 * (1.0 - 0.01 * d) * rho_k;
            let k90 = match wood {
                "lvl" => 1.30 + 0.015 * d,
                "hardwood" => 0.90 + 0.015 * d,
                _ => 1.35 + 0.015 * d,
            };
            let a = alpha_deg.to_radians();
            f0 / (k90 * a.sin().powi(2) + a.cos().powi(2))
        }
    }
}

/// 🧮 Characteristic yield moment M_y,Rk in N·mm (EN 1995-1-1 8.14, 8.30, 8.34, 8.7).
pub fn yield_moment(kind: &str, f_u: f64, d: f64, d_root: f64) -> f64 {
    match kind {
        "nail_square" => 0.45 * f_u * d.powf(2.6),
        "staple" => 240.0 * d.powf(2.6),
        "screw" if d <= 6.0 => 0.15 * f_u * (1.1 * d_root).powf(2.6),
        _ => 0.3 * f_u * d.powf(2.6),
    }
}

/// 🪢 Share of the Johansen part that the rope effect F_ax,Rk/4 may add (EN 1995-1-1 8.2.2 (2)).
pub fn rope_share(kind: &str) -> f64 {
    match kind {
        "nail" | "nail_predrilled" => 0.15,
        "nail_square" => 0.25,
        "nail_other" => 0.5,
        "screw" => 1.0,
        "bolt" => 0.25,
        _ => 0.0,
    }
}

fn rope(j: f64, fax: f64, share: f64) -> f64 {
    j + (fax / 4.0).min(share * j)
}

/// ⚙️ Mode result: label and characteristic load per shear plane in N.
pub type Modes = Vec<(&'static str, f64)>;

/// 🔩 Timber-to-timber single shear modes (a) – (f) of EN 1995-1-1 8.6.
pub fn modes_tt_single(t1: f64, t2: f64, d: f64, fh1: f64, fh2: f64, my: f64, fax: f64, share: f64) -> Modes {
    let beta = fh2 / fh1;
    let r = t2 / t1;
    let c = fh1 * t1 * d / (1.0 + beta) * ((beta + 2.0 * beta * beta * (1.0 + r + r * r) + beta.powi(3) * r * r).sqrt() - beta * (1.0 + r));
    let dm = 1.05 * fh1 * t1 * d / (2.0 + beta) * ((2.0 * beta * (1.0 + beta) + 4.0 * beta * (2.0 + beta) * my / (fh1 * d * t1 * t1)).sqrt() - beta);
    let e = 1.05 * fh1 * t2 * d / (1.0 + 2.0 * beta) * ((2.0 * beta * beta * (1.0 + beta) + 4.0 * beta * (1.0 + 2.0 * beta) * my / (fh1 * d * t2 * t2)).sqrt() - beta);
    let fm = 1.15 * (2.0 * beta / (1.0 + beta)).sqrt() * (2.0 * my * fh1 * d).sqrt();
    vec![("a", fh1 * t1 * d), ("b", fh2 * t2 * d), ("c", rope(c, fax, share)), ("d", rope(dm, fax, share)), ("e", rope(e, fax, share)), ("f", rope(fm, fax, share))]
}

/// 🔩 Timber-to-timber double shear modes (g) – (k) of EN 1995-1-1 8.6, per shear plane.
pub fn modes_tt_double(t1: f64, t2: f64, d: f64, fh1: f64, fh2: f64, my: f64, fax: f64, share: f64) -> Modes {
    let beta = fh2 / fh1;
    let j = 1.05 * fh1 * t1 * d / (2.0 + beta) * ((2.0 * beta * (1.0 + beta) + 4.0 * beta * (2.0 + beta) * my / (fh1 * d * t1 * t1)).sqrt() - beta);
    let k = 1.15 * (2.0 * beta / (1.0 + beta)).sqrt() * (2.0 * my * fh1 * d).sqrt();
    vec![("g", fh1 * t1 * d), ("h", 0.5 * fh2 * t2 * d), ("j", rope(j, fax, share)), ("k", rope(k, fax, share))]
}

fn thick_plate_modes(t: f64, d: f64, fh: f64, my: f64, fax: f64, share: f64, names: [&'static str; 3]) -> Modes {
    let one = fh * t * d * ((2.0 + 4.0 * my / (fh * d * t * t)).sqrt() - 1.0);
    let two = 2.3 * (my * fh * d).sqrt();
    vec![(names[0], fh * t * d), (names[1], rope(one, fax, share)), (names[2], rope(two, fax, share))]
}

/// 🔩 Steel-to-timber modes of EN 1995-1-1 8.2.3: `config` is `single`, `double_center` (steel in the middle) or `double_outer` (steel outside); intermediate plate thickness interpolates between thin and thick.
pub fn modes_steel_timber(config: &str, t_timber: f64, t_steel: f64, d: f64, fh: f64, my: f64, fax: f64, share: f64) -> Modes {
    let ratio = ((t_steel / d - 0.5) / 0.5).clamp(0.0, 1.0);
    let mix = |thin: &Modes, thick: &Modes| -> Modes { thin.iter().zip(thick.iter()).map(|(a, b)| (b.0, a.1 + ratio * (b.1 - a.1))).collect() };
    match config {
        "double_center" => thick_plate_modes(t_timber, d, fh, my, fax, share, ["f", "g", "h"]),
        "double_outer" => {
            let thin: Modes = vec![("j", 0.5 * fh * t_timber * d), ("k", rope(1.15 * (2.0 * my * fh * d).sqrt(), fax, share))];
            let thick: Modes = vec![("l", 0.5 * fh * t_timber * d), ("m", rope(2.3 * (my * fh * d).sqrt(), fax, share))];
            mix(&thin, &thick)
        }
        _ => {
            let thin: Modes = vec![("a", 0.4 * fh * t_timber * d), ("b", rope(1.15 * (2.0 * my * fh * d).sqrt(), fax, share))];
            let thick = thick_plate_modes(t_timber, d, fh, my, fax, share, ["c", "d", "e"]);
            if ratio <= 0.0 {
                return thin;
            }
            if ratio >= 1.0 {
                return thick;
            }
            let tmin = thin.iter().map(|m| m.1).fold(f64::INFINITY, f64::min);
            let kmin = thick.iter().map(|m| m.1).fold(f64::INFINITY, f64::min);
            vec![("interpolated", tmin + ratio * (kmin - tmin))]
        }
    }
}

fn governing(modes: &Modes) -> (&'static str, f64) {
    modes.iter().copied().fold(("", f64::INFINITY), |a, m| if m.1 < a.1 { m } else { a })
}

fn fastener_inputs(c: &Case) -> Result<(f64, f64, f64, f64), String> {
    let d = c.n("d")?;
    let rho = |k: &str, cls: &str| -> Result<f64, String> {
        match c.opt(k) {
            Some(r) => Ok(r),
            None => strength_row(c.s(cls)?).map(|r| r.rho_k).ok_or_else(|| format!("unknown strength class for {cls}")),
        }
    };
    let r1 = rho("rho_k1", "class")?;
    let r2 = if c.opt("rho_k2").is_some() || c.text.contains_key("class_2") { rho("rho_k2", "class_2")? } else { r1 };
    Ok((d, r1, r2, c.n("f_u")?))
}

/// 🧭 What the embedment diagram of T-70 needs: the fastener, the member material and the load-to-grain angle; [`EmbedmentProbe::at`] gives f_h,k at another angle.
#[derive(Clone, Debug)]
pub struct EmbedmentProbe {
    pub kind: String,
    pub panel: String,
    pub wood: String,
    pub rho_k: f64,
    pub d: f64,
    pub t_panel: f64,
    pub alpha_deg: f64,
}

impl EmbedmentProbe {
    /// 🧭 f_h,α,k in N/mm² at the angle `alpha_deg` between load and grain (EN 1995-1-1 8.31); fasteners without an angle dependence return the same value.
    pub fn at(&self, alpha_deg: f64) -> f64 {
        embedment_fh(&self.kind, &self.panel, self.rho_k, self.d, alpha_deg, &self.wood, self.t_panel)
    }

    /// 🧭 True when the embedment strength depends on the angle to the grain.
    pub fn angle_dependent(&self) -> bool {
        (self.at(0.0) - self.at(90.0)).abs() > 1e-12
    }
}

/// 🧭 The [`EmbedmentProbe`] of a T-70 case.
pub fn embedment_probe(c: &Case) -> Result<EmbedmentProbe, String> {
    let (d, rho_k, _, _) = fastener_inputs(c)?;
    Ok(EmbedmentProbe { kind: kind_of(c).into(), panel: c.so("panel", "timber").into(), wood: c.so("wood", "softwood").into(), rho_k, d, t_panel: c.o("t_panel", 0.0), alpha_deg: c.o("alpha_deg", 0.0) })
}

fn t70(c: &Case) -> R {
    let (d, r1, _, fu) = fastener_inputs(c)?;
    let kind = kind_of(c);
    let fh = embedment_probe(c)?.at(c.o("alpha_deg", 0.0));
    let my = yield_moment(kind, fu, d, c.o("d_root", 0.7 * d));
    let mut out = Vec::new();
    if c.so("panel", "timber") == "plywood" && !matches!(kind, "nail" | "staple" | "nail_predrilled") {
        out.push(Outcome::info("plywood_bolt_unverified", "Plywood embedment of bolts and dowels: nail formula applied, unverified", "Lochleibung von Bolzen und Stabdübeln in Sperrholz: Nagelformel angesetzt, ungeprüft", QuantityKind::Dimensionless, 1.0, "f_h,k = 0.11 ρ_k d^-0.3 is the nail case; the bolt and dowel formula of EN 1995-1-1 8.5.1.3 (plywood) is not printed in the repo spec".into()));
    }
    out.extend(vec![
        Outcome::info("f_h_k", "Embedment strength f_h,k", "Lochleibungsfestigkeit f_h,k", QuantityKind::Stress, fh * 1e6, format!("f_h,k({kind}, ρ_k={}, d={}) = {} N/mm²", f(r1), f(d), f(fh))),
        Outcome::info("m_y_rk", "Yield moment M_y,Rk", "Fließmoment M_y,Rk", QuantityKind::Moment, my / 1e3, format!("M_y,Rk = {} N·mm", f(my))),
    ]);
    Ok(out)
}

fn modes_of(c: &Case, config: &str) -> Result<Modes, String> {
    let (d, r1, r2, fu) = fastener_inputs(c)?;
    let kind = kind_of(c);
    let wood = c.so("wood", "softwood");
    let fh1 = embedment_fh(kind, c.so("panel", "timber"), r1, d, c.o("alpha1_deg", 0.0), wood, c.o("t_panel", 0.0));
    let fh2 = embedment_fh(kind, c.so("panel_2", "timber"), r2, d, c.o("alpha2_deg", 0.0), wood, c.o("t_panel", 0.0));
    let my = yield_moment(kind, fu, d, c.o("d_root", 0.7 * d));
    let fax = c.o("f_ax_rk", 0.0);
    let share = rope_share(kind);
    let (t1, t2) = (c.n("t1")?, c.o("t2", 0.0));
    Ok(match config {
        "tt_single" => modes_tt_single(t1, t2, d, fh1, fh2, my, fax, share),
        "tt_double" => modes_tt_double(t1, t2, d, fh1, fh2, my, fax, share),
        other => modes_steel_timber(other, t1, c.o("t_steel", 0.0), d, fh1, my, fax, share),
    })
}

/// 🔩 Every failure mode of a T-71, T-72 or T-73 case with its configuration (`tt_single`, `tt_double`, `single`, `double_center`, `double_outer`): the letter of EN 1995-1-1 Figures 8.2 and 8.3 and F_v,Rk per shear plane in N.
pub fn case_modes(entry_id: &str, c: &Case) -> Result<(String, Modes), String> {
    let config = match entry_id {
        "T-71" => "tt_single".to_string(),
        "T-72" => "tt_double".to_string(),
        _ => c.so("config", "single").to_string(),
    };
    let modes = modes_of(c, &config)?;
    Ok((config, modes))
}

fn johansen(c: &Case, config: &str) -> R {
    let modes = modes_of(c, config)?;
    let (mode, rk) = governing(&modes);
    let kmod1 = Mat::load(c).map(|m| m.kmod).or_else(|_| c.n("kmod"))?;
    let kmod = c.opt("kmod_2").map(|k2| (kmod1 * k2).sqrt()).unwrap_or(kmod1);
    let gm = gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
    let planes = c.o("shear_planes", if config.starts_with("tt_double") || config.starts_with("double") { 2.0 } else { 1.0 });
    let nef = c.o("n_ef", 1.0);
    let rd = kmod * rk / gm;
    let total = rd * planes * nef;
    let list = modes.iter().map(|m| format!("({})={}", m.0, f(m.1))).collect::<Vec<_>>().join(" ");
    let mut out = vec![
        Outcome::info("f_v_rk", "Characteristic load per fastener and shear plane F_v,Rk", "Charakteristische Tragfähigkeit je Verbindungsmittel und Scherfuge F_v,Rk", QuantityKind::Force, rk, format!("modes {list}; governing ({mode}) F_v,Rk = {} N", f(rk))),
        Outcome::info("f_v_rd", "Design resistance F_v,Rd of the connection", "Bemessungswert F_v,Rd der Verbindung", QuantityKind::Force, total, format!("F_v,Rd = k_mod F_v,Rk/γ_M · planes · n_ef = {} N", f(total))),
    ];
    if let Some(fed) = c.opt("f_ed") {
        out.push(Outcome::force("lateral", "Lateral resistance", "Querkrafttragfähigkeit", fed, total, format!("F_Ed = {} N ≤ F_v,Rd = {} N", f(fed), f(total))));
    }
    Ok(out)
}

fn t71(c: &Case) -> R {
    johansen(c, "tt_single")
}

fn t72(c: &Case) -> R {
    johansen(c, "tt_double")
}

fn t73(c: &Case) -> R {
    johansen(c, c.so("config", "single"))
}

/// 📏 Minimum spacings and distances in mm per Tables 8.2, 8.4, 8.5 and 8.6: `(name, value)` for a1, a2, a3t, a3c, a4t, a4c.
pub fn spacing_table(kind: &str, d: f64, alpha_deg: f64, rho_k: f64) -> Vec<(&'static str, f64)> {
    let a = alpha_deg.to_radians();
    let (ca, sa) = (a.cos().abs(), a.sin().abs());
    let a3c_bolt = {
        let deg = alpha_deg.rem_euclid(360.0);
        if (150.0..210.0).contains(&deg) { 4.0 * d } else { ((1.0 + 6.0 * sa) * d).max(4.0 * d) }
    };
    match kind {
        "nail" | "staple" => {
            let k = if rho_k > 420.0 { 1.5 } else { 1.0 };
            if d < 5.0 {
                vec![("a1", (5.0 + 5.0 * ca) * d * k), ("a2", 5.0 * d * k), ("a3t", (10.0 + 5.0 * a.cos()) * d * k), ("a3c", 10.0 * d * k), ("a4t", (5.0 + 2.0 * sa) * d * k), ("a4c", 5.0 * d * k)]
            } else {
                vec![("a1", (5.0 + 7.0 * ca) * d * k), ("a2", 5.0 * d * k), ("a3t", (10.0 + 5.0 * a.cos()) * d * k), ("a3c", 10.0 * d * k), ("a4t", (5.0 + 5.0 * sa) * d * k), ("a4c", 5.0 * d * k)]
            }
        }
        "nail_predrilled" => vec![("a1", (3.0 + 2.0 * ca) * d), ("a2", 3.0 * d), ("a3t", (7.0 + 5.0 * a.cos()) * d), ("a3c", 7.0 * d), ("a4t", if d < 5.0 { (3.0 + 2.0 * sa) * d } else { (3.0 + 4.0 * sa) * d }), ("a4c", 3.0 * d)],
        "bolt" => vec![("a1", (4.0 + ca) * d), ("a2", 4.0 * d), ("a3t", (7.0 * d).max(80.0)), ("a3c", a3c_bolt), ("a4t", ((2.0 + 2.0 * sa) * d).max(3.0 * d)), ("a4c", 3.0 * d)],
        "screw_axial" => vec![("a1", 7.0 * d), ("a2", 5.0 * d), ("a1cg", 10.0 * d), ("a2cg", 4.0 * d), ("a3cg", 12.0 * d), ("a4cg", 3.0 * d)],
        _ => vec![("a1", (3.0 + 2.0 * ca) * d), ("a2", 3.0 * d), ("a3t", (7.0 * d).max(80.0)), ("a3c", a3c_bolt), ("a4t", ((2.0 + 2.0 * sa) * d).max(3.0 * d)), ("a4c", 3.0 * d)],
    }
}

fn t74(c: &Case) -> R {
    let kind = kind_of(c);
    let d = c.n("d")?;
    let rho = c.o("rho_k", 350.0);
    let table = spacing_table(kind, d, c.o("alpha_deg", 0.0), rho);
    let mut out = Vec::new();
    for (name, req) in table {
        if let Some(prov) = c.opt(name) {
            out.push(Outcome::length(name, &format!("Spacing/distance {name}"), &format!("Abstand/Randabstand {name}"), req, prov, format!("required {} mm ≤ provided {} mm ({kind}, α={}°)", f(req), f(prov), f(c.o("alpha_deg", 0.0)))));
        }
    }
    if out.is_empty() {
        return Err("missing input a1, a2, a3t, a3c, a4t or a4c".into());
    }
    Ok(out)
}

/// 🧮 Effective number n_ef: nails n^k_ef (8.3.1.1), bolts/dowels min(n, n^0.9·(a1/13d)^0.25) interpolated for load angle (8.5.1.1), axial screws n^0.9.
pub fn n_ef_value(kind: &str, n: f64, a1: f64, d: f64, alpha_deg: f64, predrilled: bool) -> f64 {
    match kind {
        "nail" | "nail_predrilled" | "staple" | "screw" => {
            let r = a1 / d;
            if predrilled || kind == "nail_predrilled" {
                let kef = if r >= 10.0 { 1.0 } else if r >= 7.0 { 0.85 + 0.15 * (r - 7.0) / 3.0 } else if r >= 4.0 { 0.7 + 0.15 * (r - 4.0) / 3.0 } else { 0.7 };
                return n.powf(kef);
            }
            let kef = if r >= 14.0 { 1.0 } else if r >= 10.0 { 0.85 + 0.15 * (r - 10.0) / 4.0 } else if r >= 7.0 { 0.7 + 0.15 * (r - 7.0) / 3.0 } else if r >= 4.0 { 0.5 + 0.2 * (r - 4.0) / 3.0 } else { 0.5 };
            n.powf(kef)
        }
        "screw_axial" => n.powf(0.9),
        _ => {
            let par = n.min(n.powf(0.9) * (a1 / (13.0 * d)).powf(0.25));
            let t = (alpha_deg.abs() / 90.0).min(1.0);
            par + t * (n - par)
        }
    }
}

fn t75(c: &Case) -> R {
    let kind = kind_of(c);
    let n = c.n("n")?;
    let nef = n_ef_value(kind, n, c.n("a1")?, c.n("d")?, c.o("alpha_deg", 0.0), c.o("predrilled", 0.0) > 0.5);
    let mut out = vec![Outcome::info("n_ef", "Effective number n_ef per row", "Wirksame Anzahl n_ef je Reihe", QuantityKind::Dimensionless, nef, format!("n_ef({kind}, n={}) = {}", f(n), f(nef)))];
    if let Some(fv) = c.opt("f_v_rk") {
        out.push(Outcome::info("f_v_ef_rk", "F_v,ef,Rk = n_ef·F_v,Rk", "F_v,ef,Rk = n_ef·F_v,Rk", QuantityKind::Force, nef * fv, format!("{} N", f(nef * fv))));
    }
    Ok(out)
}

/// 🪢 Rope-effect share for a table kind (see [`rope_share`]).
pub fn rope_share_of(kind: &str) -> f64 {
    rope_share(kind)
}

/// 🧮 Withdrawal resistance in N of a screw threaded over `l_ef` mm perpendicular to the grain.
pub fn screw_withdrawal_n(d: f64, l_ef: f64, rho_k: f64) -> f64 {
    screw_withdrawal(d, l_ef, rho_k, 90.0, 1.0)
}

/// 🧮 Screw axial resistance F_ax,α,Rk per EN 1995-1-1 8.38 with f_ax,k = 0.52·d^−0.5·l_ef^−0.1·ρ_a^0.8, k_d = min(d/8, 1) and the (ρ_k/ρ_a)^0.8 correction.
pub fn screw_withdrawal(d: f64, l_ef: f64, rho_k: f64, alpha_deg: f64, n_ef: f64) -> f64 {
    let rho_a: f64 = 350.0;
    let fax = 0.52 * d.powf(-0.5) * l_ef.powf(-0.1) * rho_a.powf(0.8);
    let a = alpha_deg.to_radians();
    n_ef * fax * d * l_ef * (d / 8.0).min(1.0) / (1.2 * a.cos().powi(2) + a.sin().powi(2)) * (rho_k / rho_a).powf(0.8)
}

fn t76(c: &Case) -> R {
    let kind = kind_of(c);
    let (d, rho) = (c.n("d")?, c.n("rho_k")?);
    let kmod = c.n("kmod")?;
    let gm = gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
    let (name, rk, note) = match kind {
        "nail" | "nail_predrilled" => {
            let (fax, fhead) = (20e-6 * rho * rho, 70e-6 * rho * rho);
            let (tpen, dh, t) = (c.n("t_pen")?, c.o("d_head", 2.0 * d), c.o("t_head_side", 0.0));
            let r = (fax * d * tpen).min(fax * d * t + fhead * dh * dh);
            ("nail", r, format!("f_ax,k={} f_head,k={} → min(f_ax d t_pen, f_ax d t + f_head d_h²) = {} N", f(fax), f(fhead), f(r)))
        }
        "bolt" => {
            let (dw, fc90) = (c.n("d_washer")?, c.n("f_c_90_k")?);
            let r = (3.0 * fc90 * PI / 4.0 * (dw * dw - (d + 1.0).powi(2))).min(c.o("f_tens_k", f64::INFINITY));
            ("bolt", r, format!("min(3 f_c,90,k A_washer, f_tens) = {} N", f(r)))
        }
        _ => {
            let r_thread = screw_withdrawal(d, c.n("l_ef")?, rho, c.o("alpha_deg", 90.0), c.o("n_ef", 1.0));
            let r_head = c.o("n_ef", 1.0) * c.o("f_head_k", 8.0) * c.o("d_head", 2.0 * d).powi(2) * (rho / 350.0).powf(0.8);
            let r = r_thread.min(r_head).min(c.o("f_tens_k", f64::INFINITY));
            ("screw", r, format!("withdrawal {} N, head pull-through {} N → {} N", f(r_thread), f(r_head), f(r)))
        }
    };
    let rd = kmod * rk / gm;
    let mut out = vec![Outcome::info("f_ax_rk", &format!("Axial resistance F_ax,Rk of {name}"), &format!("Axiale Tragfähigkeit F_ax,Rk ({name})"), QuantityKind::Force, rk, note)];
    if let Some(fed) = c.opt("f_ax_ed") {
        out.push(Outcome::force("axial", "Axial resistance", "Axiale Tragfähigkeit", fed, rd, format!("F_ax,Ed = {} ≤ F_ax,Rd = {}", f(fed), f(rd))));
    }
    Ok(out)
}

fn t77(c: &Case) -> R {
    let (fa, fv) = (c.n("f_ax_ed")? / c.n("f_ax_rd")?, c.n("f_v_ed")? / c.n("f_v_rd")?);
    let (u, tag) = match kind_of(c) {
        "screw" => (fa * fa + fv * fv, "quadratic (8.28)"),
        _ => (fa + fv, "linear (8.27)"),
    };
    Ok(vec![Outcome::ratio("combined", "Combined lateral and axial loading", "Kombinierte Quer- und Axialbeanspruchung", u, 1.0, format!("{tag}: F_ax/F_ax,Rd = {}, F_v/F_v,Rd = {} → {}", f(fa), f(fv), f(u)))])
}

/// 🧮 Splitting capacity F_90,Rk = 14·b·w·√(h_e/(1 − h_e/h)) in N (EN 1995-1-1 8.4), w = 1 for non-plate fasteners.
pub fn splitting_f90_rk(b: f64, w: f64, he: f64, h: f64) -> f64 {
    14.0 * b * w * (he / (1.0 - he / h)).sqrt()
}

fn t78(c: &Case) -> R {
    let mut out = Vec::new();
    if let Some(fed) = c.opt("f_90_ed") {
        let m = Mat::load(c)?;
        let rk = splitting_f90_rk(c.n("b")?, c.o("w", 1.0), c.n("he")?, c.n("h")?);
        let rd = m.kmod * rk / gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
        out.push(Outcome::force("splitting", "Splitting F_90,Ed ≤ F_90,Rd (8.4)", "Aufspalten F_90,Ed ≤ F_90,Rd (8.4)", fed, rd, format!("F_90,Rk = {} N", f(rk))));
    }
    if let (Some(at), Some(av)) = (c.opt("a_net_t"), c.opt("a_net_v")) {
        let m = Mat::load(c)?;
        let rk = (1.5 * at * m.row.f_t_0_k).max(0.7 * av * m.row.f_v_k);
        let rd = m.kmod * rk / gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
        out.push(Outcome::force("block_shear", "Block shear (A.1)", "Blockscherversagen (A.1)", c.n("f_ed")?, rd, format!("F_bs,Rk = max(1.5 A_net,t f_t,0,k, 0.7 A_net,v f_v,k) = {} N", f(rk))));
    }
    if out.is_empty() {
        return Err("missing input f_90_ed or a_net_t and a_net_v".into());
    }
    Ok(out)
}

fn t79(c: &Case) -> R {
    let (t, d0, fu) = (c.n("t_plate")?, c.n("d0")?, c.n("f_u_plate")?);
    let (e1, e2) = (c.n("e1")?, c.n("e2")?);
    let anet = c.n("a_net")?;
    let nu = 0.9 * anet * fu / 1.25;
    let k1 = (2.8 * e2 / d0 - 1.7).min(2.5);
    let ab = (e1 / (3.0 * d0)).min(c.o("f_u_bolt", 800.0) / fu).min(1.0);
    let fb = k1 * ab * fu * c.n("d")? * t / 1.25;
    Ok(vec![
        Outcome::force("plate_net", "Steel plate net section", "Stahllasche Nettoquerschnitt", c.n("f_ed")?, nu, format!("N_u,Rd = 0.9 A_net f_u/γ_M2 = {} N", f(nu))),
        Outcome::force("plate_bearing", "Steel plate bearing", "Stahllasche Lochleibung", c.n("f_ed")? / c.o("n_bolts", 1.0), fb, format!("F_b,Rd = k₁ α_b f_u d t/γ_M2 = {} N", f(fb))),
    ])
}

fn connector_k(c: &Case) -> (f64, f64, f64, f64) {
    (c.o("k1", 1.0), c.o("k2", 1.0), c.o("k3", 1.0), c.o("k4", 1.0))
}

fn t80(c: &Case) -> R {
    let (dc, he) = (c.n("d_c")?, c.n("h_e")?);
    let rho = c.n("rho_k")?;
    let (a3t, ta1, ta2) = (c.o("a3t", 1.5 * dc), c.o("t_a1", 3.0 * he), c.o("t_a2", 5.0 * he));
    let k1 = (ta1 / (3.0 * he)).min(ta2 / (5.0 * he)).min(1.0);
    let k2 = (a3t / (1.5 * dc)).min(1.0);
    let k3 = (rho / 350.0).min(1.75);
    let k4 = if c.so("members", "timber") == "steel" { 1.1 } else { 1.0 };
    let f0 = (k1 * k2 * k3 * k4 * 35.0 * dc.powf(1.5)).min(k1 * k3 * he * 31.5 * dc);
    let a = c.o("alpha_deg", 0.0).to_radians();
    let k90 = 1.3 + 0.001 * dc;
    let fa = f0 / (k90 * a.sin().powi(2) + a.cos().powi(2));
    let kmod = c.n("kmod")?;
    let rd = kmod * fa / gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
    let mut out = vec![Outcome::info("f_v_alpha_rk", "Connector resistance F_v,α,Rk", "Verbinderfestigkeit F_v,α,Rk", QuantityKind::Force, fa, format!("k1={} k2={} k3={} k4={} F_v,0,Rk={} N", f(k1), f(k2), f(k3), f(k4), f(f0)))];
    if let Some(fed) = c.opt("f_ed") {
        out.push(Outcome::force("connector", "Split ring/shear plate resistance", "Tragfähigkeit Ring-/Scheibendübel", fed, rd, format!("F_v,Rd = {} N", f(rd))));
    }
    Ok(out)
}

fn t81(c: &Case) -> R {
    let dc = c.n("d_c")?;
    let (k1, k2, k3, kt) = connector_k(c);
    let f0 = 18.0 * k1 * k2 * k3 * kt * dc.powf(1.5);
    let rd = c.n("kmod")? * f0 / gamma_m_for(c.annex, "connection", false).unwrap_or(f64::NAN);
    let mut out = vec![Outcome::info("f_v_0_rk", "Toothed-plate resistance F_v,0,Rk", "Tragfähigkeit Einpressdübel F_v,0,Rk", QuantityKind::Force, f0, format!("18 k1 k2 k3 k_t d_c^1.5 = {} N", f(f0)))];
    if let Some(fed) = c.opt("f_ed") {
        out.push(Outcome::force("toothed_plate", "Toothed-plate connection", "Einpressdübelverbindung", fed, rd, format!("F_v,Rd = {} N", f(rd))));
    }
    Ok(out)
}

fn t82(c: &Case) -> R {
    let fa = c.n("f_a_k")? * c.n("a_ef")?;
    let kmod = c.n("kmod")?;
    let gm = gamma_m_for(c.annex, "punched-plate", false).unwrap_or(f64::NAN);
    let steel = c.n("f_t_k_plate")? * c.n("l_plate")?;
    let r = (kmod * fa / gm).min(steel / gm);
    Ok(vec![Outcome::force("nail_plate", "Punched metal plate anchorage and steel", "Nagelplatte Verankerung und Stahl", c.n("f_ed")?, r, format!("anchorage {} N, plate {} N", f(kmod * fa / gm), f(steel / gm)))])
}

/// 🧮 Slip modulus K_ser per fastener and shear plane in N/mm (EN 1995-1-1 Table 7.1), timber-to-timber with ρ_m = √(ρ_m1·ρ_m2); steel-to-timber doubles it.
pub fn kser_value(kind: &str, rho_m: f64, d: f64, steel_to_timber: bool) -> f64 {
    let base = match kind {
        "nail" => rho_m.powf(1.5) * d.powf(0.8) / 30.0,
        "staple" => rho_m.powf(1.5) * d.powf(0.8) / 80.0,
        "split_ring" | "shear_plate" => rho_m.powf(1.5) * d / 2.0,
        "toothed_plate" => 1.5 * rho_m.powf(1.5) * d / 4.0,
        _ => rho_m.powf(1.5) * d / 23.0,
    };
    if steel_to_timber { 2.0 * base } else { base }
}

fn t83(c: &Case) -> R {
    let rho = match (c.opt("rho_m1"), c.opt("rho_m2")) {
        (Some(a), Some(b)) => (a * b).sqrt(),
        (Some(a), None) => a,
        _ => return Err("missing input rho_m1".into()),
    };
    let k = kser_value(kind_of(c), rho, c.n("d")?, c.o("steel_to_timber", 0.0) > 0.5);
    Ok(vec![
        Outcome::info("k_ser", "Slip modulus K_ser", "Verschiebungsmodul K_ser", QuantityKind::Dimensionless, k, format!("K_ser = {} N/mm", f(k))),
        Outcome::info("k_u", "Ultimate slip modulus K_u = 2/3 K_ser", "Verschiebungsmodul im GZT K_u = 2/3 K_ser", QuantityKind::Dimensionless, 2.0 / 3.0 * k, format!("K_u = {} N/mm", f(2.0 / 3.0 * k))),
    ])
}

/// 🗂 Catalogue rows of the connection group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-70", part: "EN 1995-1-1", clause: "8.5.1", en: "Embedment strength and yield moment", de: "Lochleibungsfestigkeit und Fließmoment", inputs: &["kind", "d", "rho_k1", "class", "f_u", "alpha_deg", "wood", "panel", "t_panel"], eval: t70 },
    Entry { id: "T-71", part: "EN 1995-1-1", clause: "8.2.2", en: "Timber-to-timber single shear (EYM)", de: "Holz-Holz einschnittig (Johansen)", inputs: &["kind", "d", "t1", "t2", "class", "class_2", "f_u", "f_ax_rk", "sc", "duration", "f_ed"], eval: t71 },
    Entry { id: "T-72", part: "EN 1995-1-1", clause: "8.2.2", en: "Timber-to-timber double shear (EYM)", de: "Holz-Holz zweischnittig (Johansen)", inputs: &["kind", "d", "t1", "t2", "class", "class_2", "f_u", "f_ax_rk", "sc", "duration", "f_ed"], eval: t72 },
    Entry { id: "T-73", part: "EN 1995-1-1", clause: "8.2.3", en: "Steel-to-timber (EYM)", de: "Stahl-Holz (Johansen)", inputs: &["config", "kind", "d", "t1", "t2", "t_steel", "class", "f_u", "f_ax_rk", "sc", "duration", "f_ed"], eval: t73 },
    Entry { id: "T-74", part: "EN 1995-1-1", clause: "8.3", en: "Spacing, edge and end distances", de: "Abstände, Rand- und Hirnholzabstände", inputs: &["kind", "d", "alpha_deg", "rho_k", "a1", "a2", "a3t", "a3c", "a4t", "a4c"], eval: t74 },
    Entry { id: "T-75", part: "EN 1995-1-1", clause: "8.1.2", en: "Effective number of fasteners", de: "Wirksame Anzahl der Verbindungsmittel", inputs: &["kind", "n", "a1", "d", "alpha_deg", "predrilled", "f_v_rk"], eval: t75 },
    Entry { id: "T-76", part: "EN 1995-1-1", clause: "8.3.2", en: "Axial resistance of nails, screws, bolts", de: "Axiale Tragfähigkeit von Nägeln, Schrauben, Bolzen", inputs: &["kind", "d", "rho_k", "t_pen", "l_ef", "alpha_deg", "n_ef", "f_ax_ed", "kmod"], eval: t76 },
    Entry { id: "T-77", part: "EN 1995-1-1", clause: "8.7.3", en: "Combined lateral and axial loading", de: "Kombinierte Quer- und Axialbeanspruchung", inputs: &["kind", "f_ax_ed", "f_ax_rd", "f_v_ed", "f_v_rd"], eval: t77 },
    Entry { id: "T-78", part: "EN 1995-1-1", clause: "8.1.4", en: "Splitting and block shear", de: "Aufspalten und Blockscheren", inputs: &["class", "f_90_ed", "b", "h", "he", "w", "a_net_t", "a_net_v", "f_ed"], eval: t78 },
    Entry { id: "T-79", part: "EN 1995-1-1", clause: "8", en: "Steel details inside timber connections", de: "Stahlteile in Holzverbindungen", inputs: &["f_ed", "t_plate", "d0", "d", "e1", "e2", "a_net", "f_u_plate"], eval: t79 },
    Entry { id: "T-80", part: "EN 1995-1-1", clause: "8.9", en: "Split rings and shear plates", de: "Ring- und Scheibendübel", inputs: &["d_c", "h_e", "rho_k", "alpha_deg", "f_ed", "kmod"], eval: t80 },
    Entry { id: "T-81", part: "EN 1995-1-1", clause: "8.10", en: "Toothed-plate connectors", de: "Einpressdübel", inputs: &["d_c", "k1", "k2", "k3", "k4", "f_ed", "kmod"], eval: t81 },
    Entry { id: "T-82", part: "EN 1995-1-1", clause: "8.8", en: "Punched metal plate fasteners", de: "Nagelplatten", inputs: &["f_a_k", "a_ef", "f_t_k_plate", "l_plate", "f_ed", "kmod"], eval: t82 },
    Entry { id: "T-83", part: "EN 1995-1-1", clause: "7.1", en: "Connection stiffness K_ser and K_u", de: "Verbindungssteifigkeit K_ser und K_u", inputs: &["kind", "rho_m1", "rho_m2", "d", "steel_to_timber"], eval: t83 },
];
