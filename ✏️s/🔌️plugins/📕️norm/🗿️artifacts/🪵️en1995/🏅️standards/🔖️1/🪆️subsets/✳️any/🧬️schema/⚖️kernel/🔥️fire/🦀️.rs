//! 🔥️ Fire design of EN 1995-1-2 as catalogue entries T-100 … T-105: charring, reduced cross-section, zero-strength layer, protected members, connections and assemblies. Units: N, mm, N/mm², minutes. See [`super`].

use super::*;

/// 🔥 Charring rates `(β₀, β_n)` in mm/min per EN 1995-1-2 Table 3.1; hardwood interpolates between ρ_k = 290 and 450 kg/m³, panels use the density and thickness corrections k_ρ, k_h.
pub fn charring_rates(material: &str, rho_k: f64, hp: f64) -> Option<(f64, f64)> {
    match material {
        "softwood_solid" => Some((0.65, 0.80)),
        "glulam" => Some((0.65, 0.70)),
        "lvl" => Some((0.65, 0.70)),
        "clt" => Some((0.65, 0.65)),
        "hardwood" => {
            let t = ((rho_k - 290.0) / 160.0).clamp(0.0, 1.0);
            Some((0.65 - 0.15 * t, 0.70 - 0.15 * t))
        }
        "panel" => {
            let b0 = 0.9 * (450.0 / rho_k).sqrt() * (20.0 / hp).sqrt();
            Some((b0, b0))
        }
        _ => None,
    }
}

fn product_kfi(row: &StrengthRow) -> f64 {
    let key = match row.product {
        TimberProduct::Solid => "solid",
        TimberProduct::Glulam => "glulam",
        TimberProduct::Clt => "clt",
        TimberProduct::Lvl => "lvl",
    };
    ndp_common(&format!("ndp.en1995.k-fi.{key}"))
}

fn material_key(c: &Case) -> String {
    c.text.get("char_material").cloned().unwrap_or_else(|| match c.s("class").ok().and_then(strength_row) {
        Some(r) if r.hardwood => "hardwood".into(),
        Some(r) => match r.product {
            TimberProduct::Glulam => "glulam".into(),
            TimberProduct::Lvl => "lvl".into(),
            TimberProduct::Clt => "clt".into(),
            TimberProduct::Solid => "softwood_solid".into(),
        },
        None => "softwood_solid".into(),
    })
}

fn t100(c: &Case) -> R {
    let key = material_key(c);
    let (b0, bn) = charring_rates(&key, c.o("rho_k", 350.0), c.o("h_p", 20.0)).ok_or_else(|| format!("unknown material {key}"))?;
    let t = c.n("t_min")?;
    let mut out = vec![
        Outcome::info("beta_0", "One-dimensional charring rate β₀", "Eindimensionale Abbrandrate β₀", QuantityKind::Dimensionless, b0, format!("β₀({key}) = {} mm/min", f(b0))),
        Outcome::info("beta_n", "Notional charring rate β_n", "Ideelle Abbrandrate β_n", QuantityKind::Dimensionless, bn, format!("β_n({key}) = {} mm/min", f(bn))),
        Outcome::info("d_char_0", "d_char,0 = β₀·t", "d_char,0 = β₀·t", QuantityKind::Length, b0 * t / 1e3, format!("{} mm at t = {} min", f(b0 * t), f(t))),
        Outcome::info("d_char_n", "d_char,n = β_n·t", "d_char,n = β_n·t", QuantityKind::Length, bn * t / 1e3, format!("{} mm at t = {} min", f(bn * t), f(t))),
    ];
    if c.o("series", 0.0) > 0.5 {
        for m in [15.0, 30.0, 60.0, 90.0, 120.0] {
            out.push(Outcome::info(&format!("series_{}", m as u32), &format!("d_char,n at {m} min"), &format!("d_char,n bei {m} min"), QuantityKind::Length, bn * m / 1e3, format!("{} mm", f(bn * m))));
        }
    }
    Ok(out)
}

/// 🔥 Effective charring depth d_ef = d_char + k₀·d₀ with d₀ = 7 mm and k₀ = t/20 for t < 20 min, otherwise 1 (EN 1995-1-2 4.2.2).
pub fn d_ef_mm(d_char: f64, t_min: f64) -> f64 {
    d_char + (t_min / 20.0).min(1.0) * 7.0
}

struct Residual {
    b: f64,
    h: f64,
    def: f64,
}

fn residual(c: &Case, t: f64, zsl: bool, rates: (f64, f64)) -> Result<Residual, String> {
    let (b, h) = (c.n("b")?, c.n("h")?);
    let dchar = if zsl { rates.0 } else { rates.1 } * t;
    let def = d_ef_mm(dchar, t);
    let n = |k: &str, d: f64| c.o(k, d);
    Ok(Residual { b: b - def * (n("face_left", 1.0) + n("face_right", 1.0)), h: h - def * (n("face_top", 0.0) + n("face_bottom", 1.0)), def })
}

fn fire_utilisation(c: &Case, m: &Mat, t: f64, zsl: bool, rates: (f64, f64)) -> Result<(f64, Residual), String> {
    let r = residual(c, t, zsl, rates)?;
    if r.b <= 0.0 || r.h <= 0.0 {
        return Ok((9.99, r));
    }
    let kfi = product_kfi(&m.row);
    let (a, w) = (r.b * r.h, r.b * r.h * r.h / 6.0);
    let mut u = 0.0;
    let fm = kfi * m.row.f_m_k;
    if let Some(mfi) = c.opt("m_fi") {
        u += mfi / w / fm;
    }
    if let Some(n) = c.opt("n_c_fi") {
        u = u.max((n / a / (kfi * m.row.f_c_0_k)).powi(2) + c.o("m_fi", 0.0) / w / fm);
    }
    if let Some(n) = c.opt("n_t_fi") {
        u = u.max(n / a / (kfi * m.row.f_t_0_k) + c.o("m_fi", 0.0) / w / fm);
    }
    Ok((u, r))
}

fn fire_method(c: &Case, zsl: bool) -> R {
    let m = Mat::load(c)?;
    let key = material_key(c);
    let rates = charring_rates(&key, c.o("rho_k", m.row.rho_k), c.o("h_p", 20.0)).ok_or("unknown charring material")?;
    let t = c.n("t_req")?;
    let (u, r) = fire_utilisation(c, &m, t, zsl, rates)?;
    let (mut lo, mut hi) = (0.0_f64, 240.0_f64);
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if fire_utilisation(c, &m, mid, zsl, rates)?.0 <= 1.0 { lo = mid } else { hi = mid }
    }
    let other = fire_utilisation(c, &m, t, !zsl, rates)?;
    Ok(vec![
        Outcome::ratio("fire_resistance", if zsl { "Zero-strength layer: utilisation at required time" } else { "Reduced cross-section: utilisation at required time" }, if zsl { "Nullfestigkeitsschicht: Ausnutzung zur geforderten Zeit" } else { "Querschnittsabminderung: Ausnutzung zur geforderten Zeit" }, u, 1.0, format!("d_ef={} mm, b_fi={} mm, h_fi={} mm", f(r.def), f(r.b), f(r.h))),
        Outcome::ratio("fire_time", "Required fire resistance time vs achievable", "Geforderte gegen erreichbare Feuerwiderstandsdauer", t, lo.max(1e-6), format!("t_R = {} min", f(lo))),
        Outcome::info("method_comparison", "Other method utilisation", "Ausnutzung der anderen Methode", QuantityKind::Dimensionless, other.0, format!("η = {} with d_ef = {} mm", f(other.0), f(other.1.def))),
    ])
}

fn t101(c: &Case) -> R {
    fire_method(c, false)
}

fn t102(c: &Case) -> R {
    fire_method(c, true)
}

/// 🛡️ Protection times of a gypsum board layer in minutes: start of charring behind it t_ch = 2.8·h_p − 14 (EN 1995-1-2 3.4.3.3); failure time t_f = t_ch for type F boards and 0.8·t_ch otherwise.
pub fn cladding_times(hp: f64, board: &str) -> (f64, f64) {
    let tch = (2.8 * hp - 14.0).max(0.0);
    (tch, if board == "F" { tch } else { 0.8 * tch })
}

/// 🧭 What the protection diagram of T-103 needs: the basic charring rate in mm/min, the board thickness in mm and type, the start of charring and the failure time of the cladding in minutes; [`ProtectedProbe::depth_at`] gives the char depth at any time.
#[derive(Clone, Debug)]
pub struct ProtectedProbe {
    pub b0: f64,
    pub hp: f64,
    pub board: String,
    pub t_ch: f64,
    pub t_f: f64,
    pub t_min: f64,
}

impl ProtectedProbe {
    /// 🧭 Char depth in mm after `minutes` behind the cladding: none until t_ch, k₂β₀ until the failure time t_f, then twice β₀ (EN 1995-1-2 3.4.3.3).
    pub fn depth_at(&self, minutes: f64) -> f64 {
        let (tch, tf) = (self.t_ch, self.t_f.max(self.t_ch));
        let k2 = 1.0 - 0.018 * self.hp;
        if minutes <= tch { 0.0 } else if minutes <= tf { k2 * self.b0 * (minutes - tch) } else { k2 * self.b0 * (tf - tch) + 2.0 * self.b0 * (minutes - tf) }
    }
}

/// 🧭 The [`ProtectedProbe`] of a T-103 case.
pub fn protected_probe(c: &Case) -> Result<ProtectedProbe, String> {
    let (hp, t_min) = (c.n("h_p")?, c.n("t_min")?);
    let (b0, _) = charring_rates(&material_key(c), c.o("rho_k", 400.0), 20.0).ok_or("unknown material")?;
    let board = c.so("board", "F");
    let (t_ch, t_f) = cladding_times(hp, board);
    Ok(ProtectedProbe { b0, hp, board: board.into(), t_ch, t_f, t_min })
}

fn t103(c: &Case) -> R {
    let p = protected_probe(c)?;
    let (tch, tf, k2) = (p.t_ch, p.t_f, 1.0 - 0.018 * p.hp);
    let d = p.depth_at(p.t_min);
    Ok(vec![
        Outcome::info("t_ch", "Start of charring t_ch", "Beginn des Abbrands t_ch", QuantityKind::Time, tch * 60.0, format!("t_ch = 2.8 h_p − 14 = {} min", f(tch))),
        Outcome::info("t_f", "Cladding failure t_f", "Versagen der Bekleidung t_f", QuantityKind::Time, tf * 60.0, format!("t_f = {} min", f(tf))),
        Outcome::info("d_char_protected", "Char depth at t", "Abbrandtiefe zur Zeit t", QuantityKind::Length, d / 1e3, format!("{} mm (k₂={})", f(d), f(k2))),
    ])
}

fn t104(c: &Case) -> R {
    let (rk, t) = (c.n("f_v_rk")?, c.n("t_req")?);
    let kfi = if c.so("side_members", "timber") == "steel" { 1.05 } else { 1.15 };
    let rd = kfi * rk;
    let beta = c.o("beta_n", 0.7);
    let afi = beta * t * c.o("k_flux", 1.5);
    let mut out = vec![Outcome::info("a_fi", "Additional edge distance a_fi", "Zusätzlicher Randabstand a_fi", QuantityKind::Length, afi / 1e3, format!("a_fi = β_n t k_flux = {} mm", f(afi)))];
    if let Some(e) = c.opt("f_fi_d") {
        out.push(Outcome::force("connection_fire", "Connection resistance in fire", "Verbindungstragfähigkeit im Brandfall", e, rd * c.o("n_ef", 1.0) * c.o("shear_planes", 1.0), format!("F_v,Rd,fi = k_fi F_v,Rk/γ_M,fi = {} N", f(rd))));
        if let Some(r20) = c.opt("f_rd_20") {
            let eta = e / r20;
            out.push(Outcome::info("eta", "Load level η = E_fi,d / R_d", "Ausnutzungsgrad η = E_fi,d / R_d", QuantityKind::Dimensionless, eta, format!("η = {} N / {} N", f(e), f(r20))));
            if let Some(k) = c.opt("k_fire").or_else(|| ndp_number_opt("ndp.en1995.fire.connection-reduced-load-k", c.annex)) {
                let tfi = if eta >= 1.0 { 0.0 } else { -eta.ln() / k };
                out.push(Outcome::ratio("connection_fire_time", "Required time against reduced-load fire resistance", "Geforderte Zeit gegen Feuerwiderstand nach dem Verfahren mit abgeminderter Last", t, tfi.max(1e-6), format!("t_fi,d = −ln(η)/k = {} min with k = {} 1/min (constant of the reduced load method, to be confirmed against the printed standard)", f(tfi), f(k))));
            }
        }
    }
    if let Some(prov) = c.opt("a_provided") {
        out.push(Outcome::length("edge_distance_fire", "Edge distance incl. a_fi", "Randabstand inkl. a_fi", c.o("a_required", 0.0) + afi, prov, "a_req + a_fi ≤ a_provided".into()));
    }
    Ok(out)
}

fn t105(c: &Case) -> R {
    let n = c.n("n_layers")? as usize;
    let mut sum = 0.0;
    for k in 1..=n {
        let hp = c.n(&format!("h{k}"))?;
        sum += match c.so(&format!("kind{k}"), "gypsum_f") {
            "timber" => hp / c.o("beta_0", 0.65),
            "insulation" => 0.5 * hp.powf(1.4),
            "gypsum_a" => cladding_times(hp, "A").0,
            _ => cladding_times(hp, "F").0,
        };
    }
    Ok(vec![Outcome::ratio("separating_function", "Separating function t_ins ≥ t_req", "Raumabschluss t_ins ≥ t_req", c.n("t_req")?, sum.max(1e-6), format!("t_ins = Σ t_prot,i = {} min", f(sum)))])
}

/// 🔥 Residual section of the reduced-properties method (EN 1995-1-2 4.2.3): width and depth after d_char,n, the exposed perimeter over the area `p/A_r` in 1/m and the factors k_mod,fi for bending, compression and tension; linear in time up to 20 min (to be confirmed against the printed standard).
#[derive(Clone, Copy, Debug)]
pub struct Reduced {
    pub b: f64,
    pub h: f64,
    pub area: f64,
    pub ratio: f64,
    pub k_bend: f64,
    pub k_comp: f64,
    pub k_ten: f64,
}

/// 🔥 Reduced-properties factors for the exposed `faces = [left, right, top, bottom]` (1 exposed, 0 not) of a rectangle b × h after `t_min` minutes with the notional rate β_n in mm/min.
pub fn reduced_properties(b: f64, h: f64, faces: [f64; 4], beta_n: f64, t_min: f64) -> Reduced {
    let d = beta_n * t_min;
    let (br, hr) = (b - d * (faces[0] + faces[1]), h - d * (faces[2] + faces[3]));
    if br <= 0.0 || hr <= 0.0 {
        return Reduced { b: br.max(0.0), h: hr.max(0.0), area: 0.0, ratio: f64::INFINITY, k_bend: 0.0, k_comp: 0.0, k_ten: 0.0 };
    }
    let ratio = ((faces[0] + faces[1]) * hr + (faces[2] + faces[3]) * br) / (br * hr) * 1e3;
    let time = (t_min / ndp_common("ndp.en1995.fire.reduced-time-limit")).min(1.0);
    let k = |divisor: &str| (1.0 - time * ratio / ndp_common(divisor)).max(0.0);
    Reduced { b: br, h: hr, area: br * hr, ratio, k_bend: k("ndp.en1995.fire.reduced-bending-divisor"), k_comp: k("ndp.en1995.fire.reduced-compression-divisor"), k_ten: k("ndp.en1995.fire.reduced-tension-divisor") }
}

fn reduced_utilisation(c: &Case, m: &Mat, t: f64, beta_n: f64) -> Result<(f64, Reduced), String> {
    let faces = [c.o("face_left", 1.0), c.o("face_right", 1.0), c.o("face_top", 0.0), c.o("face_bottom", 1.0)];
    let r = reduced_properties(c.n("b")?, c.n("h")?, faces, beta_n, t);
    let (m_fi, n_c, n_t) = (c.opt("m_fi"), c.opt("n_c_fi"), c.opt("n_t_fi"));
    if m_fi.is_none() && n_c.is_none() && n_t.is_none() {
        return Err("missing input m_fi, n_c_fi or n_t_fi".into());
    }
    if r.area <= 0.0 {
        return Ok((9.99, r));
    }
    let kfi = product_kfi(&m.row);
    let share = |load: f64, capacity: f64| if capacity <= 1e-9 { 9.99 } else { load / capacity };
    let bending = share(m_fi.unwrap_or(0.0) / (r.b * r.h * r.h / 6.0), r.k_bend * kfi * m.row.f_m_k);
    let mut u = bending;
    if let Some(n) = n_c {
        u = u.max(share(n / r.area, r.k_comp * kfi * m.row.f_c_0_k).powi(2) + bending);
    }
    if let Some(n) = n_t {
        u = u.max(share(n / r.area, r.k_ten * kfi * m.row.f_t_0_k) + bending);
    }
    Ok((u, r))
}

fn t106(c: &Case) -> R {
    let m = Mat::load(c)?;
    let rates = charring_rates(&material_key(c), c.o("rho_k", m.row.rho_k), c.o("h_p", 20.0)).ok_or("unknown charring material")?;
    let t = c.n("t_req")?;
    let (u, r) = reduced_utilisation(c, &m, t, rates.1)?;
    let (mut lo, mut hi) = (0.0_f64, 240.0_f64);
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if reduced_utilisation(c, &m, mid, rates.1)?.0 <= 1.0 { lo = mid } else { hi = mid }
    }
    let cross = fire_utilisation(c, &m, t, false, rates)?;
    let steps = format!("p/A_r = {} 1/m, b_r = {} mm, h_r = {} mm", f(r.ratio), f(r.b), f(r.h));
    Ok(vec![
        Outcome::info("k_mod_fi_bending", "Reduction factor k_mod,fi for bending", "Abminderungsbeiwert k_mod,fi für Biegung", QuantityKind::Dimensionless, r.k_bend, steps.clone()),
        Outcome::info("k_mod_fi_compression", "Reduction factor k_mod,fi for compression", "Abminderungsbeiwert k_mod,fi für Druck", QuantityKind::Dimensionless, r.k_comp, steps.clone()),
        Outcome::info("k_mod_fi_tension", "Reduction factor k_mod,fi for tension and stiffness", "Abminderungsbeiwert k_mod,fi für Zug und Steifigkeit", QuantityKind::Dimensionless, r.k_ten, steps.clone()),
        Outcome::ratio("fire_resistance", "Reduced properties: utilisation at required time", "Reduzierte Eigenschaften: Ausnutzung zur geforderten Zeit", u, 1.0, steps),
        Outcome::ratio("fire_time", "Required fire resistance time vs achievable", "Geforderte gegen erreichbare Feuerwiderstandsdauer", t, lo.max(1e-6), format!("t_R = {} min", f(lo))),
        Outcome::info("method_comparison", "Reduced cross-section utilisation", "Ausnutzung nach reduziertem Querschnitt", QuantityKind::Dimensionless, cross.0, format!("η = {} with d_ef = {} mm", f(cross.0), f(cross.1.def))),
    ])
}

/// 🌳 Char depth in mm of a CLT layup after `t_end` minutes with the layer thicknesses from the exposed face; with `fall_off` the char of a layer drops at its glue line and the next layer chars `factor` times faster until `protect` mm of new char protect it. Returns the depth and the minutes at which each glue line was reached (research model after Schmid/Frangi, to be confirmed against EN 1995-1-2:2025).
pub fn clt_char_depth(layers: &[f64], beta: f64, t_end: f64, fall_off: bool, factor: f64, protect: f64) -> (f64, Vec<f64>) {
    let total: f64 = layers.iter().sum();
    let (mut t, mut x, mut local, mut fell, mut k) = (0.0_f64, 0.0_f64, 0.0_f64, false, 0_usize);
    let mut times = Vec::new();
    while beta > 0.0 && k < layers.len() && t < t_end {
        let edge: f64 = layers[..=k].iter().sum();
        let accelerated = fell && local < protect;
        let rate = if accelerated { factor * beta } else { beta };
        let stop = if accelerated { edge.min(x + protect - local) } else { edge };
        let dt = (stop - x) / rate;
        if t + dt >= t_end {
            x += rate * (t_end - t);
            break;
        }
        t += dt;
        local += stop - x;
        x = stop;
        if x >= edge - 1e-9 {
            times.push(t);
            fell = fall_off;
            local = 0.0;
            k += 1;
        }
    }
    (x.min(total), times)
}

/// 🌳 Surviving longitudinal parts `(z_bottom, z_top)` of a layup `(thickness, longitudinal)` from the exposed face once `d_ef` mm are lost.
pub fn clt_residual_parts(layers: &[(f64, bool)], d_ef: f64) -> Vec<(f64, f64)> {
    let mut z: f64 = 0.0;
    let mut out = Vec::new();
    for (t, longitudinal) in layers {
        let (lo, hi) = (z.max(d_ef), z + t);
        if *longitudinal && hi > lo {
            out.push((lo, hi));
        }
        z += t;
    }
    out
}

/// 🌳 Extreme-fibre bending stress per mm width of the rigid residual longitudinal parts under `moment` in N·mm/mm (γ = 1: an upper bound of the residual stiffness).
pub fn clt_residual_stress(parts: &[(f64, f64)], moment: f64) -> Option<f64> {
    let area: f64 = parts.iter().map(|p| p.1 - p.0).sum();
    if area <= 0.0 {
        return None;
    }
    let zs = parts.iter().map(|p| (p.1 - p.0) * (p.0 + p.1) / 2.0).sum::<f64>() / area;
    let inertia: f64 = parts.iter().map(|p| (p.1 - p.0).powi(3) / 12.0 + (p.1 - p.0) * ((p.0 + p.1) / 2.0 - zs).powi(2)).sum();
    let reach = parts.iter().map(|p| (p.0 - zs).abs().max((p.1 - zs).abs())).fold(0.0_f64, f64::max);
    Some(moment * reach / inertia)
}

fn clt_layers(c: &Case) -> Result<Vec<(f64, bool)>, String> {
    (1..=c.n("n_layers")? as usize).map(|k| Ok((c.n(&format!("t{k}"))?, c.n(&format!("o{k}"))? > 0.5))).collect()
}

fn t107(c: &Case) -> R {
    let layers = clt_layers(c)?;
    let thick: Vec<f64> = layers.iter().map(|l| l.0).collect();
    let row = strength_row(c.so("class", "CLT100")).ok_or("unknown lamella class")?;
    let (beta, factor, protect, on) = (c.o("beta_0", 0.65), c.o("fall_off_factor", ndp_common("ndp.en1995.fire.clt-fall-off-factor")), c.o("protect_mm", ndp_common("ndp.en1995.fire.clt-fall-off-protect")), c.o("fall_off", 1.0) > 0.5);
    let t = c.n("t_req")?;
    let (depth, times) = clt_char_depth(&thick, beta, t, on, factor, protect);
    let (plain, _) = clt_char_depth(&thick, beta, t, false, factor, protect);
    let d_ef = d_ef_mm(depth, t);
    let utilisation = |time: f64| -> Option<f64> {
        let m_fi = c.opt("m_fi")?;
        let d = clt_char_depth(&thick, beta, time, on, factor, protect).0;
        Some(clt_residual_stress(&clt_residual_parts(&layers, d_ef_mm(d, time)), m_fi).map_or(9.99, |s| s / (product_kfi(&row) * row.f_m_k)))
    };
    let mut out = vec![
        Outcome::info("char_depth", "Char depth with fall-off", "Abbrandtiefe mit Abfallen", QuantityKind::Length, depth / 1e3, format!("d_char = {} mm after {} min, {} glue lines reached", f(depth), f(t), times.len())),
        Outcome::info("char_depth_plain", "Char depth without fall-off", "Abbrandtiefe ohne Abfallen", QuantityKind::Length, plain / 1e3, format!("β₀ t = {} mm", f(plain))),
        Outcome::info("d_ef", "Effective charring depth d_ef", "Wirksame Abbrandtiefe d_ef", QuantityKind::Length, d_ef / 1e3, format!("d_ef = d_char + k₀ d₀ = {} mm", f(d_ef))),
    ];
    for (i, time) in times.iter().enumerate() {
        out.push(Outcome::info(&format!("glue_line_{}", i + 1), &format!("Glue line {} reached", i + 1), &format!("Klebfuge {} erreicht", i + 1), QuantityKind::Time, time * 60.0, format!("{} min", f(*time))));
    }
    if let Some(u) = utilisation(t) {
        let (mut lo, mut hi) = (0.0_f64, 240.0_f64);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if utilisation(mid).is_some_and(|v| v <= 1.0) { lo = mid } else { hi = mid }
        }
        out.push(Outcome::ratio("fire_resistance", "Residual layers: bending utilisation at required time", "Restlagen: Biegeausnutzung zur geforderten Zeit", u, 1.0, "σ = M c / I of the rigid residual longitudinal layers ≤ k_fi f_m,k".into()));
        out.push(Outcome::ratio("fire_time", "Required fire resistance time vs achievable", "Geforderte gegen erreichbare Feuerwiderstandsdauer", t, lo.max(1e-6), format!("t_R = {} min", f(lo))));
    }
    Ok(out)
}

/// 🗂 Catalogue rows of the fire group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-100", part: "EN 1995-1-2", clause: "3.4.2", en: "Charring depth", de: "Abbrandtiefe", inputs: &["class", "char_material", "t_min", "rho_k", "h_p", "series"], eval: t100 },
    Entry { id: "T-101", part: "EN 1995-1-2", clause: "4.2.2", en: "Reduced cross-section method", de: "Methode des reduzierten Querschnitts", inputs: &["class", "b", "h", "t_req", "face_left", "face_right", "face_top", "face_bottom", "m_fi", "n_c_fi", "n_t_fi"], eval: t101 },
    Entry { id: "T-102", part: "EN 1995-1-2", clause: "4.2.2", en: "Zero-strength layer method", de: "Methode der Nullfestigkeitsschicht", inputs: &["class", "b", "h", "t_req", "m_fi"], eval: t102 },
    Entry { id: "T-103", part: "EN 1995-1-2", clause: "3.4.3", en: "Protected members, claddings", de: "Geschützte Bauteile, Bekleidungen", inputs: &["class", "h_p", "t_min", "board"], eval: t103 },
    Entry { id: "T-104", part: "EN 1995-1-2", clause: "6.2", en: "Connections in fire", de: "Verbindungen im Brandfall", inputs: &["f_v_rk", "t_req", "side_members", "beta_n", "f_fi_d", "f_rd_20", "k_fire", "a_required", "a_provided"], eval: t104 },
    Entry { id: "T-105", part: "EN 1995-1-2", clause: "Annex E", en: "Fire-separating assemblies", de: "Raumabschließende Bauteile", inputs: &["n_layers", "h1", "kind1", "t_req"], eval: t105 },
    Entry { id: "T-106", part: "EN 1995-1-2", clause: "4.2.3", en: "Reduced properties method", de: "Methode der reduzierten Eigenschaften", inputs: &["class", "b", "h", "t_req", "face_left", "face_right", "face_top", "face_bottom", "m_fi", "n_c_fi", "n_t_fi"], eval: t106 },
    Entry { id: "T-107", part: "EN 1995-1-2", clause: "Annex CLT", en: "CLT charring with fall-off", de: "BSP-Abbrand mit Abfallen der Kohleschicht", inputs: &["class", "n_layers", "t1", "o1", "t_req", "beta_0", "fall_off", "fall_off_factor", "protect_mm", "m_fi"], eval: t107 },
];
