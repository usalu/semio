//! 🧱️ Earth pressure and gravity walls, catalogue entries RW-01 … RW-06, RW-08 … RW-10 (EN 1997-1 §9 and Annex C, DIN 4085, DIN 1054 §9).

use super::*;
use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{thrust, Acceleration, Interface, Side, Water};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// 📐️ Coulomb active coefficient K_a (RW-01); α wall batter, β ground slope, δ wall friction, all degrees.
pub fn ka_coulomb(phi_deg: f64, delta_deg: f64, alpha_deg: f64, beta_deg: f64) -> f64 {
    let (p, d, a, b) = (rad(phi_deg), rad(delta_deg), rad(alpha_deg), rad(beta_deg));
    let root = ((p + d).sin() * (p - b).sin().max(0.0) / ((a + d).cos() * (a - b).cos())).sqrt();
    (p - a).cos().powi(2) / (a.cos().powi(2) * (a + d).cos() * (1.0 + root).powi(2))
}

/// 📐️ Coulomb passive coefficient with a plane failure surface, δ as the positive mobilised wall friction magnitude (valid up to |δ| ≈ φ/3).
pub fn kp_coulomb(phi_deg: f64, delta_deg: f64, alpha_deg: f64, beta_deg: f64) -> f64 {
    let (p, d, a, b) = (rad(phi_deg), rad(delta_deg), rad(alpha_deg), rad(beta_deg));
    let root = ((p + d).sin() * (p + b).sin() / ((d - a).cos() * (a - b).cos())).sqrt();
    (p + a).cos().powi(2) / (a.cos().powi(2) * (d - a).cos() * (1.0 - root).powi(2))
}

/// 📐️ Rankine active coefficient for ground slope β (RW-02); β = 0 gives (1 − sin φ)/(1 + sin φ).
pub fn ka_rankine(phi_deg: f64, beta_deg: f64) -> f64 {
    let (p, b) = (rad(phi_deg), rad(beta_deg));
    let root = (b.cos().powi(2) - p.cos().powi(2)).max(0.0).sqrt();
    b.cos() * (b.cos() - root) / (b.cos() + root)
}

/// 📐️ Rankine passive coefficient (1 + sin φ)/(1 − sin φ) for level ground.
pub fn kp_rankine(phi_deg: f64) -> f64 {
    let s = rad(phi_deg).sin();
    (1.0 + s) / (1.0 - s)
}

/// 📐️ At-rest coefficient K0 = (1 − sin φ)·√OCR, ground slope β raises it by (1 + sin β) (RW-04).
pub fn k0(phi_deg: f64, ocr: f64, beta_deg: f64) -> f64 {
    (1.0 - rad(phi_deg).sin()) * ocr.max(1.0).sqrt() * (1.0 + rad(beta_deg).sin())
}

fn spiral_wedge(xo: f64, yo: f64, tan_phi: f64) -> Option<(f64, f64)> {
    let r_b = xo.hypot(yo);
    let th0 = (-yo).atan2(-xo);
    let pos = |th: f64| {
        let r = r_b * (th * tan_phi).exp();
        (xo + r * (th0 + th).cos(), yo + r * (th0 + th).sin())
    };
    let mut prev = 0.0;
    let mut th = 0.0;
    let mut exit = None;
    while th < 6.0 {
        th += 0.01;
        let (x, y) = pos(th);
        if x < -1e-9 {
            return None;
        }
        if y >= 1.0 {
            let (mut lo, mut hi) = (prev, th);
            for _ in 0..50 {
                let mid = 0.5 * (lo + hi);
                if pos(mid).1 >= 1.0 { hi = mid } else { lo = mid }
            }
            exit = Some(hi);
            break;
        }
        prev = th;
    }
    let te = exit?;
    let n = 240;
    let mut pts = vec![(0.0, 1.0), (0.0, 0.0)];
    for i in 1..=n {
        pts.push(pos(te * i as f64 / n as f64));
    }
    if let Some(last) = pts.last_mut() {
        last.1 = 1.0;
    }
    pts.push((0.0, 1.0));
    let (mut m, mut a) = (0.0, 0.0);
    for w in pts.windows(2) {
        let ((x1, y1), (x2, y2)) = (w[0], w[1]);
        m += ((x1 - xo).powi(2) + (x1 - xo) * (x2 - xo) + (x2 - xo).powi(2)) / 6.0 * (y2 - y1);
        a += 0.5 * (x1 * y2 - x2 * y1);
    }
    Some((m, a))
}

/// 📐️ Passive coefficient K_p for a curved (logarithmic spiral) failure surface behind a vertical wall with level ground; the minimum of the spiral moment equilibrium over the pole position.
/// The wall friction is limited to 2/3 φ for curved failure surfaces. Matches Rankine at δ = 0 and Caquot-Kerisel within 3 % up to δ = φ/2 (conservative), 8 % above at δ = φ.
pub fn kp_curved(phi_deg: f64, delta_deg: f64) -> f64 {
    static CACHE: OnceLock<Mutex<HashMap<(u64, u64), f64>>> = OnceLock::new();
    let key = (phi_deg.to_bits(), delta_deg.abs().min(phi_deg.abs() * 2.0 / 3.0).to_bits());
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(value) = cache.lock().ok().and_then(|map| map.get(&key).copied()) {
        return value;
    }
    let value = kp_curved_search(phi_deg, delta_deg);
    if let Ok(mut map) = cache.lock() {
        map.insert(key, value);
    }
    value
}

fn kp_curved_search(phi_deg: f64, delta_deg: f64) -> f64 {
    let delta = delta_deg.abs().min(phi_deg.abs() * 2.0 / 3.0);
    if delta < 1e-9 || phi_deg <= 0.0 {
        return kp_rankine(phi_deg.max(0.0));
    }
    let (tan_phi, d) = (rad(phi_deg).tan(), rad(delta));
    let value = |xo: f64, yo: f64| -> f64 {
        let Some((m, a)) = spiral_wedge(xo, yo, tan_phi) else { return f64::INFINITY };
        let den = yo * d.cos() + xo * d.sin() - d.cos() / 3.0;
        if den <= 1e-9 || a <= 1e-9 { f64::INFINITY } else { 2.0 * m / den }
    };
    let (mut cx, mut cy, mut wx, mut wy) = (-5.0, 3.0, 7.0, 3.0);
    let mut best = (f64::INFINITY, cx, cy);
    for _ in 0..7 {
        for i in -9..=9 {
            for j in -9..=9 {
                let xo = cx + wx * i as f64 / 9.0;
                let yo = (cy + wy * j as f64 / 9.0).max(0.05);
                let v = value(xo, yo);
                if v < best.0 {
                    best = (v, xo, yo);
                }
            }
        }
        cx = best.1;
        cy = best.2;
        wx *= 0.35;
        wy *= 0.35;
    }
    best.0
}

fn design_phi(c: &Case, phi_deg: f64) -> f64 {
    (rad(phi_deg).tan() / c.factors().gamma_phi).atan().to_degrees()
}

fn rw_01(c: &Case) -> R {
    let (gamma, h, phi) = (c.positive("gamma")?, c.positive("h")?, c.positive("phi_deg")?);
    let (delta, alpha, beta, q) = (c.o("delta_deg", 0.0), c.o("alpha_deg", 0.0), c.o("beta_deg", 0.0), c.o("q", 0.0));
    let ka = ka_coulomb(phi, delta, alpha, beta);
    let ea = 0.5 * gamma * h * h * ka + ka * q * h;
    let wall_angle = rad(alpha + delta);
    Ok(vec![
        Outcome::number("ka", "Coulomb K_a", "Coulomb K_a", ka),
        Outcome::pascal("e_top", "Pressure at top", "Erddruck oben", ka * q),
        Outcome::pascal("e_bot", "Pressure at base", "Erddruck unten", ka * (gamma * h + q)),
        Outcome::newton("e_a", "Resultant E_a per metre", "Resultierende E_a je m", ea),
        Outcome::newton("e_ah", "Horizontal component E_ah", "Horizontalkomponente E_ah", ea * wall_angle.cos()),
        Outcome::newton("e_av", "Vertical component E_av", "Vertikalkomponente E_av", ea * wall_angle.sin()),
    ])
}

fn rw_02(c: &Case) -> R {
    let (gamma, h, phi) = (c.positive("gamma")?, c.positive("h")?, c.positive("phi_deg")?);
    let (beta, cohesion, z) = (c.o("beta_deg", 0.0), c.o("c", 0.0), c.o("z", c.n("h")?));
    let ka = ka_rankine(phi, beta);
    let z0 = if cohesion > 0.0 { 2.0 * cohesion / (gamma * ka.sqrt()) } else { 0.0 };
    let e_z = (ka * gamma * z - 2.0 * cohesion * ka.sqrt()).max(0.0);
    let thrust = if h > z0 { 0.5 * (ka * gamma * h - 2.0 * cohesion * ka.sqrt()) * (h - z0) } else { 0.0 };
    Ok(vec![
        Outcome::number("ka", "Rankine K_a", "Rankine K_a", ka),
        Outcome::metre("z0", "Tension zone depth z0", "Zugzonentiefe z0", z0),
        Outcome::pascal("e_z", "Pressure at depth z", "Erddruck in Tiefe z", e_z),
        Outcome::newton("e_a", "Active thrust without tension zone", "Aktiver Erddruck ohne Zugzone", thrust),
    ])
}

fn rw_03(c: &Case) -> R {
    let (gamma, t, phi) = (c.positive("gamma")?, c.positive("t")?, c.positive("phi_deg")?);
    let (delta, cohesion, q) = (c.o("delta_deg", 0.0), c.o("c", 0.0), c.o("q", 0.0));
    let f = c.factors();
    let phi = design_phi(c, phi);
    let cohesion = cohesion / f.gamma_c;
    let kp = kp_curved(phi, delta);
    let ep_k = 0.5 * gamma * t * t * kp + 2.0 * cohesion * t * kp.sqrt() + q * kp * t;
    let g_re = f.gamma_r_e;
    let mobilised = c.o("mobilisation", 1.0).clamp(0.0, 1.0);
    Ok(vec![
        Outcome::number("kp_rankine", "Rankine K_p", "Rankine K_p", kp_rankine(phi)),
        Outcome::number("kp_coulomb", "Coulomb K_p (plane surface)", "Coulomb K_p (ebene Gleitfläche)", kp_coulomb(phi, delta.abs(), 0.0, 0.0)),
        Outcome::number("kp", "K_p curved failure surface", "K_p gekrümmte Gleitfläche", kp),
        Outcome::newton("e_p_k", "E_p,k per metre", "E_p,k je m", ep_k),
        Outcome::newton("e_p_d", "E_p,d = mobilisation·E_p,k/γR,e", "E_p,d = Mobilisierung·E_p,k/γR,e", mobilised * ep_k / g_re),
    ])
}

fn rw_04(c: &Case) -> R {
    let (phi, gamma, h) = (c.positive("phi_deg")?, c.positive("gamma")?, c.positive("h")?);
    let (ocr, beta, q) = (c.o("ocr", 1.0), c.o("beta_deg", 0.0), c.o("q", 0.0));
    let k = k0(phi, ocr, beta);
    Ok(vec![
        Outcome::number("k0", "At-rest K_0", "Ruhedruckbeiwert K_0", k),
        Outcome::pascal("e_bot", "Pressure at base", "Erddruck unten", k * (gamma * h + q)),
        Outcome::newton("e_0", "At-rest thrust E_0", "Ruhedruckkraft E_0", 0.5 * k * gamma * h * h + k * q * h),
    ])
}

fn rw_05(c: &Case) -> R {
    let (h, ka) = (c.positive("h")?, c.positive("ka")?);
    let (gamma, gamma_sat) = (c.positive("gamma")?, c.o("gamma_sat", c.n("gamma")? + 2000.0));
    let (z_w, q, gw) = (c.o("z_w", h), c.o("q", 0.0), c.gamma_w());
    let dry = z_w.min(h);
    let wet = (h - dry).max(0.0);
    let sigma_eff_base = gamma * dry + (gamma_sat - gw) * wet;
    let e_earth_base = ka * (sigma_eff_base + q);
    let u_base = gw * wet;
    let f_earth = ka * q * h + 0.5 * ka * gamma * dry * dry + ka * gamma * dry * wet + 0.5 * ka * (gamma_sat - gw) * wet * wet;
    let f_water = 0.5 * gw * wet * wet;
    let mut out = vec![
        Outcome::pascal("e_earth_base", "Effective earth pressure at base", "Effektiver Erddruck unten", e_earth_base),
        Outcome::pascal("u_base", "Pore pressure at base", "Porenwasserdruck unten", u_base),
        Outcome::newton("f_earth", "Earth pressure resultant", "Erddruckresultierende", f_earth),
        Outcome::newton("f_water", "Water pressure resultant", "Wasserdruckresultierende", f_water),
    ];
    if let (Some(b_strip), Some(a), Some(z)) = (c.opt("strip_b"), c.opt("strip_a"), c.opt("strip_z")) {
        let q_strip = c.n("strip_q")?;
        let theta1 = ((a + b_strip) / z).atan();
        let theta2 = (a / z).atan();
        let beta_angle = theta1 - theta2;
        let dh = 2.0 * q_strip / std::f64::consts::PI * (beta_angle - beta_angle.sin() * (beta_angle + 2.0 * theta2).cos());
        out.push(Outcome::pascal("strip_dsigma_h", "Strip-load horizontal pressure (Boussinesq)", "Streifenlast-Horizontaldruck (Boussinesq)", dh));
    }
    Ok(out)
}

fn rw_06(c: &Case) -> R {
    let b = c.positive("b")?;
    let (w, x_w) = (c.positive("w")?, c.positive("x_w")?);
    let (eah_g, arm_g) = (c.n("eah_g")?, c.positive("arm_g")?);
    let (eah_q, arm_q) = (c.o("eah_q", 0.0), c.o("arm_q", 1.0));
    let (eav, x_v) = (c.o("eav", 0.0), c.o("x_v", b));
    let phi_base = c.positive("phi_base_deg")?;
    let p = c.factors();
    let u = c.upl();
    let h_d = p.gamma_g * eah_g + p.gamma_q * eah_q;
    let v_k = w + eav;
    let r_slide = v_k * rad(phi_base).tan() / p.gamma_phi / p.gamma_r_h;
    let m_dst = u.gamma_equ_dst * eah_g * arm_g + u.gamma_equ_q * eah_q * arm_q;
    let m_stb = u.gamma_equ_stb * (w * x_w + eav * x_v);
    let m_k = w * x_w + eav * x_v - eah_g * arm_g;
    let e_perm = b / 2.0 - m_k / v_k;
    let m_total = m_k - eah_q * arm_q;
    let e_total = b / 2.0 - m_total / v_k;
    let sigma_mean = v_k / b;
    let (sigma_max, sigma_min) = if e_total.abs() <= b / 6.0 {
        (sigma_mean * (1.0 + 6.0 * e_total.abs() / b), sigma_mean * (1.0 - 6.0 * e_total.abs() / b))
    } else {
        (2.0 * v_k / (3.0 * (b / 2.0 - e_total.abs())), 0.0)
    };
    Ok(vec![
        Outcome::force("sliding", "Sliding H_d vs R_d", "Gleiten H_d gegen R_d", h_d, r_slide),
        Outcome::moment("equ", "Overturning EQU M_dst vs M_stb", "Kippen EQU M_dst gegen M_stb", m_dst, m_stb),
        Outcome::length("e_perm", "Eccentricity of permanent resultant vs B/6", "Ausmitte ständig gegen B/6", e_perm.abs(), b / 6.0),
        Outcome::length("e_total", "Eccentricity of total resultant vs B/3", "Ausmitte gesamt gegen B/3", e_total.abs(), b / 3.0),
        Outcome::pascal("sigma_max", "Base pressure maximum", "Sohlpressung maximal", sigma_max),
        Outcome::pascal("sigma_min", "Base pressure minimum", "Sohlpressung minimal", sigma_min),
    ])
}

fn rw_08(c: &Case) -> R {
    let (gamma, h, phi) = (c.positive("gamma")?, c.positive("h")?, c.positive("phi_deg")?);
    let (delta, alpha, beta) = (c.o("delta_deg", 0.0), c.o("alpha_deg", 0.0), c.o("beta_deg", 0.0));
    let (kh, kv) = (c.positive("kh")?, c.o("kv", 0.0));
    let ka = ka_coulomb(phi, delta, alpha, beta);
    let interface = Interface { phi_deg: phi, delta_deg: delta, wall_deg: 90.0 - alpha, slope_deg: beta };
    let load = thrust(Side::Active, &interface, gamma, h, Water::NONE, Acceleration { kh, kv }).ok_or("no limit equilibrium: φ − θ − β < 0")?;
    Ok(vec![
        Outcome::number("kae", "Mononobe-Okabe K_ae (more onerous of ±kv)", "Mononobe-Okabe K_ae (ungünstigeres ±kv)", load.coefficient),
        Outcome::number("ka", "Static K_a", "Statisches K_a", ka),
        Outcome::newton("e_ae", "Seismic thrust E_ae = ½γ(1 ± kv)H²K_ae", "Seismischer Erddruck E_ae", load.total()),
        Outcome::newton("delta_e", "Dynamic increment ΔE", "Dynamischer Zuwachs ΔE", load.total() - 0.5 * gamma * h * h * ka),
    ])
}

fn rw_09(c: &Case) -> R {
    let (k, sigma_v, s_v) = (c.positive("k")?, c.positive("sigma_v")?, c.positive("s_v")?);
    let (f_star, b) = (c.positive("f_star")?, c.o("b", 1.0));
    let t_max = k * sigma_v * s_v;
    Ok(vec![Outcome::newton("t_max", "Reinforcement tension T_max = K·σv·Sv", "Bewehrungszug T_max = K·σv·Sv", t_max), Outcome::metre("l_e", "Embedment length L_e = T/(2·b·f*·σv)", "Verankerungslänge L_e", t_max / (2.0 * b * f_star * sigma_v))])
}

fn rw_10(c: &Case) -> R {
    let (q_line, b, z, ka) = (c.positive("q_total")?, c.positive("b")?, c.o("z", 0.0), c.positive("ka")?);
    let spread = b + 2.0 * z;
    let q_z = q_line / spread;
    Ok(vec![Outcome::pascal("q_z", "Surcharge at depth z (45° spread)", "Auflast in Tiefe z (45°-Ausbreitung)", q_z), Outcome::pascal("e_h", "Horizontal pressure K_a·q(z)", "Horizontaldruck K_a·q(z)", ka * q_z)])
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "RW-01", part: "EN 1997-1", clause: "9.5", en: "Active earth pressure, Coulomb", de: "Aktiver Erddruck, Coulomb", inputs: &["gamma", "h", "phi_deg", "delta_deg", "alpha_deg", "beta_deg", "q"], eval: rw_01 },
    Entry { id: "RW-02", part: "EN 1997-1", clause: "9.5", en: "Active earth pressure, Rankine", de: "Aktiver Erddruck, Rankine", inputs: &["gamma", "h", "phi_deg", "beta_deg", "c", "z"], eval: rw_02 },
    Entry { id: "RW-03", part: "EN 1997-1", clause: "9.5", en: "Passive earth pressure, curved failure surface", de: "Passiver Erddruck, gekrümmte Gleitfläche", inputs: &["gamma", "t", "phi_deg", "delta_deg", "c", "q", "mobilisation", "approach", "situation"], eval: rw_03 },
    Entry { id: "RW-04", part: "EN 1997-1", clause: "9.5", en: "At-rest earth pressure", de: "Ruhedruck", inputs: &["phi_deg", "gamma", "h", "ocr", "beta_deg", "q"], eval: rw_04 },
    Entry { id: "RW-05", part: "EN 1997-1", clause: "9.3", en: "Surcharges and water pressure", de: "Auflasten und Wasserdruck", inputs: &["h", "ka", "gamma", "gamma_sat", "z_w", "q", "gamma_w", "strip_q", "strip_b", "strip_a", "strip_z"], eval: rw_05 },
    Entry { id: "RW-06", part: "EN 1997-1", clause: "9.7", en: "Gravity and cantilever wall verifications", de: "Nachweise Schwergewichts- und Winkelstützwand", inputs: &["b", "w", "x_w", "eah_g", "arm_g", "eah_q", "arm_q", "eav", "x_v", "phi_base_deg", "approach", "situation"], eval: rw_06 },
    Entry { id: "RW-08", part: "EN 1998-5", clause: "7.3", en: "Seismic active earth pressure, Mononobe-Okabe", de: "Seismischer aktiver Erddruck, Mononobe-Okabe", inputs: &["gamma", "h", "phi_deg", "delta_deg", "alpha_deg", "beta_deg", "kh", "kv"], eval: rw_08 },
    Entry { id: "RW-09", part: "EN 1997-1", clause: "9.8", en: "Reinforced soil internal stability", de: "Innere Standsicherheit bewehrter Erde", inputs: &["k", "sigma_v", "s_v", "f_star", "b"], eval: rw_09 },
    Entry { id: "RW-10", part: "EN 1997-1", clause: "9.5", en: "Traffic surcharge at the crest", de: "Verkehrslast an der Wandkrone", inputs: &["q_total", "b", "z", "ka"], eval: rw_10 },
];
