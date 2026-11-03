//! ⚓️ Embedded walls, anchors and dewatering, catalogue entries RW-07, ANC-01 … ANC-04, DEW-01 … DEW-03 (EN 1997-1 §8, §9.7, §10, DIN 1054, EAB, DIN EN 1537).

use super::*;

/// 📐️ Horizontal earth-pressure coefficients (K_ah, K_ph) of a vertical wall with wall friction magnitudes δ_a and δ_p.
pub fn horizontal_coefficients(phi_deg: f64, delta_a_deg: f64, delta_p_deg: f64) -> (f64, f64) {
    (earth_pressure::ka_coulomb(phi_deg, delta_a_deg, 0.0, 0.0) * rad(delta_a_deg).cos(), earth_pressure::kp_curved(phi_deg, delta_p_deg) * rad(delta_p_deg).cos())
}

struct Wall {
    gamma: f64,
    q: f64,
    kah: f64,
    kph: f64,
    h: f64,
    gamma_g: f64,
    gamma_re: f64,
}

impl Wall {
    fn active(&self, z: f64) -> f64 {
        self.gamma_g * self.kah * (self.gamma * z + self.q)
    }
    fn passive(&self, z: f64) -> f64 {
        if z <= self.h { 0.0 } else { self.kph * self.gamma * (z - self.h) / self.gamma_re }
    }
    fn moment_about(&self, point: f64, depth: f64) -> f64 {
        let n = 2000;
        let dz = depth / n as f64;
        (0..n).map(|i| {
            let z = (i as f64 + 0.5) * dz;
            (self.active(z) - self.passive(z)) * (z - point) * dz
        }).sum()
    }
    fn resultants(&self, depth: f64) -> (f64, f64) {
        let n = 2000;
        let dz = depth / n as f64;
        let (mut a, mut p) = (0.0, 0.0);
        for i in 0..n {
            let z = (i as f64 + 0.5) * dz;
            a += self.active(z) * dz;
            p += self.passive(z) * dz;
        }
        (a, p)
    }
}

fn solve_embedment(w: &Wall, point: f64, ref_toe: bool) -> Result<f64, String> {
    let g = |t: f64| if ref_toe { -w.moment_about(w.h + t, w.h + t) } else { w.moment_about(point, w.h + t) };
    let (mut lo, mut hi) = (0.01, 4.0 * w.h + 5.0);
    gate(g(lo) > 0.0 && g(hi) < 0.0, "no equilibrium embedment found")?;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if g(mid) > 0.0 { lo = mid } else { hi = mid }
    }
    Ok(0.5 * (lo + hi))
}

fn rw_07(c: &Case) -> R {
    let (h, gamma, phi) = (c.positive("h")?, c.positive("gamma")?, c.positive("phi_deg")?);
    let (da, dp, q) = (c.o("delta_a_deg", 0.0), c.o("delta_p_deg", 0.0), c.o("q", 0.0));
    let p = c.factors();
    let phi = (rad(phi).tan() / p.gamma_phi).atan().to_degrees();
    let (kah, kph) = horizontal_coefficients(phi, da, dp);
    let wall = Wall { gamma, q, kah, kph, h, gamma_g: p.gamma_g, gamma_re: p.gamma_r_e };
    let cantilever = c.so("kind", "free_earth") == "cantilever";
    let (t0, support) = if cantilever {
        (solve_embedment(&wall, 0.0, true)?, 0.0)
    } else {
        let a = c.positive("prop_depth")?;
        gate(a < h, "prop depth must lie above the dredge level")?;
        let t = solve_embedment(&wall, a, false)?;
        let (act, pas) = wall.resultants(h + t);
        (t, act - pas)
    };
    let t_design = if cantilever { 1.2 * t0 } else { t0 };
    let depth = h + t_design;
    let n = 4000;
    let dz = depth / n as f64;
    let prop = c.o("prop_depth", f64::INFINITY);
    let (mut shear, mut moment, mut m_max, mut applied) = (0.0, 0.0, 0.0_f64, cantilever);
    for i in 0..n {
        let z = (i as f64 + 0.5) * dz;
        shear += (wall.active(z) - wall.passive(z)) * dz;
        if !applied && z >= prop {
            shear -= support;
            applied = true;
        }
        moment += shear * dz;
        m_max = m_max.max(moment.abs());
    }
    let mut out = vec![Outcome::newton_metre("m_max_d", "Maximum design bending moment", "Maximales Bemessungsmoment", m_max), Outcome::metre("t0", "Equilibrium embedment t0", "Gleichgewichtseinbindung t0", t0), Outcome::metre("t_design", "Design embedment (cantilever ×1.2)", "Bemessungseinbindung (Kragwand ×1,2)", t_design)];
    if !cantilever {
        out.push(Outcome::newton("prop_force", "Prop/anchor force A_d per metre", "Stützen-/Ankerkraft A_d je m", support));
    }
    if let (Some(cu), Some(nc)) = (c.opt("cu_k"), c.opt("nc")) {
        let demand = p.gamma_g * gamma * h + p.gamma_q * q;
        out.insert(0, Outcome::pressure("basal_heave", "Basal heave γH + q vs c_u·N_c", "Sohlhebung γH + q gegen c_u·N_c", demand, cu / p.gamma_cu * nc));
    }
    Ok(out)
}

fn anc_01(c: &Case) -> R {
    let r = c.series("ra");
    gate(!r.is_empty(), "series ra1…ran of suitability test pull-out resistances required")?;
    let p = c.factors();
    let xi = c.o("xi_a", 1.10);
    let (mean, min, rk) = piles::characteristic_from_series(&r, xi, xi);
    let rd = rk / p.gamma_a;
    let mut out = vec![Outcome::newton("mean", "Mean test resistance", "Mittelwert Eignungsprüfung", mean), Outcome::newton("min", "Minimum test resistance", "Minimalwert Eignungsprüfung", min), Outcome::newton("r_a_k", "R_a,k", "R_a,k", rk), Outcome::newton("r_a_d", "R_a,d = R_a,k/γa", "R_a,d = R_a,k/γa", rd)];
    if let Some(a) = c.opt("a_d") {
        out.insert(0, Outcome::force("pullout", "Anchor force A_d vs R_a,d", "Ankerkraft A_d gegen R_a,d", a, rd));
    }
    Ok(out)
}

fn anc_02(c: &Case) -> R {
    let (d, lf, tau) = (c.positive("d")?, c.positive("l_f")?, c.positive("tau_f")?);
    let rk = std::f64::consts::PI * d * lf * tau;
    let rd = rk / c.factors().gamma_a;
    let mut out = vec![Outcome::newton("r_a_k", "R_a,k = π·D·L_f·τ_f", "R_a,k = π·D·L_f·τ_f", rk), Outcome::newton("r_a_d", "R_a,d", "R_a,d", rd)];
    if let Some(a) = c.opt("a_d") {
        out.insert(0, Outcome::force("pullout", "Anchor force A_d vs R_a,d", "Ankerkraft A_d gegen R_a,d", a, rd));
    }
    Ok(out)
}

fn anc_03(c: &Case) -> R {
    let (e_h, theta) = (c.positive("e_h_d")?, c.positive("theta_deg")?);
    let a_d = e_h / rad(theta).cos();
    let a_k = c.o("a_k", a_d);
    let mut out = vec![Outcome::newton("a_d", "Anchor force A_d = E_h,d/cos θ", "Ankerkraft A_d = E_h,d/cos θ", a_d), Outcome::newton("proof", "Proof load 1.25·A_k", "Prüfkraft 1,25·A_k", 1.25 * a_k), Outcome::newton("lock_off", "Lock-off load 0.9·A_k", "Festlegekraft 0,9·A_k", 0.9 * a_k)];
    if let (Some(t_w), Some(z_m), Some(x_m)) = (c.opt("t_w"), c.opt("z_m"), c.opt("x_m")) {
        let (gamma, phi, e1, e2) = (c.positive("gamma")?, rad(c.positive("phi_deg")?), c.o("e1", 0.0), c.positive("e2")?);
        let weight = gamma * x_m * (t_w + z_m) / 2.0 + c.o("q", 0.0) * x_m;
        let len = (x_m * x_m + (z_m - t_w).powi(2)).sqrt();
        let (dx, dy) = (x_m / len, -(z_m - t_w) / len);
        let (nx, ny) = ((z_m - t_w) / len, x_m / len);
        let (qx, qy) = (phi.cos() * nx + phi.sin() * dx, phi.cos() * ny + phi.sin() * dy);
        let th = rad(theta);
        let det = th.cos() * qy + th.sin() * qx;
        gate(det.abs() > 1e-12, "degenerate Kranz geometry")?;
        let a_max = (weight * qx - (e2 - e1) * qy) / det;
        out.push(Outcome::newton("a_kranz", "Kranz maximum anchor force", "Kranz maximale Ankerkraft", a_max));
        out.push(Outcome::ratio("kranz", "Required A_k vs Kranz capacity", "Erforderliche A_k gegen Kranz-Tragfähigkeit", a_k * c.o("eta_kranz", 1.5), a_max));
    }
    Ok(out)
}

fn anc_04(c: &Case) -> R {
    let (a_d, p_t) = (c.positive("a_d")?, c.positive("p_t_k")?);
    let gamma_s = c.o("gamma_s", 1.15);
    let l_min = c.o("l_min", if c.so("tendon", "strand") == "bar" { 3.0 } else { 4.5 });
    let mut out = vec![Outcome::force("tendon", "A_d vs P_t,d = P_t,k/γs", "A_d gegen P_t,d = P_t,k/γs", a_d, p_t / gamma_s)];
    if let Some(lf) = c.opt("l_free") {
        out.push(Outcome::length("free_length", "Minimum free length vs provided", "Mindestfreispiel gegen vorhanden", l_min, lf));
        if let (Some(e_t), Some(a_t)) = (c.opt("e_t"), c.opt("a_t")) {
            gate(e_t > 0.0 && a_t > 0.0, "tendon modulus and area must be positive")?;
            out.push(Outcome::info(QuantityKind::Length, "elongation", "Elastic elongation δ = A_d·l_free/(E·A_t)", "Elastische Längung δ = A_d·l_frei/(E·A_t)", a_d * lf / (e_t * a_t)));
        }
    }
    Ok(out)
}

fn dew_radius(c: &Case, s: f64, k: f64, big_h: f64) -> f64 {
    if let Some(r) = c.opt("r_influence") {
        return r;
    }
    if c.so("radius", "sichardt") == "kusakin" { 575.0 * s * (big_h * k).sqrt() } else { 3000.0 * s * k.sqrt() }
}

fn dew_01(c: &Case) -> R {
    let (k, big_h, h, r) = (c.positive("k")?, c.positive("h_aquifer")?, c.positive("h_well")?, c.positive("r_well")?);
    gate(h < big_h, "water level in the well must be below the undisturbed head")?;
    let s = big_h - h;
    let radius = dew_radius(c, s, k, big_h);
    let q = if c.so("aquifer", "unconfined") == "confined" {
        2.0 * std::f64::consts::PI * k * c.positive("thickness")? * s / (radius / r).ln()
    } else {
        std::f64::consts::PI * k * (big_h * big_h - h * h) / (radius / r).ln()
    };
    let mut out = vec![Outcome::number("radius", "Radius of influence R", "Reichweite R", radius), Outcome::number("q", "Discharge Q (m³/s)", "Förderstrom Q (m³/s)", q), Outcome::number("q_m3h", "Discharge Q (m³/h)", "Förderstrom Q (m³/h)", q * 3600.0)];
    if let Some(per_well) = c.opt("q_well") {
        out.push(Outcome::number("wells", "Required number of wells", "Erforderliche Brunnenanzahl", (q / per_well).ceil()));
    }
    Ok(out)
}

fn dew_02(c: &Case) -> R {
    let (k, big_h, h, area) = (c.positive("k")?, c.positive("h_aquifer")?, c.positive("h_well")?, c.positive("area")?);
    gate(h < big_h, "water level in the pit must be below the undisturbed head")?;
    let s = big_h - h;
    let radius = dew_radius(c, s, k, big_h);
    let r_eq = (area / std::f64::consts::PI).sqrt();
    let q = std::f64::consts::PI * k * (big_h * big_h - h * h) / (radius / r_eq).ln();
    let mut out = vec![Outcome::metre("r_eq", "Equivalent well radius", "Ersatzbrunnenradius", r_eq), Outcome::number("q", "Discharge Q (m³/s)", "Förderstrom Q (m³/s)", q)];
    let xs = c.series("x");
    let ys = c.series("y");
    let qs = c.series("qw");
    if !xs.is_empty() && xs.len() == ys.len() && xs.len() == qs.len() {
        let (px, py, d) = (c.n("px")?, c.n("py")?, c.positive("thickness")?);
        let drawdown: f64 = xs.iter().zip(&ys).zip(&qs).map(|((x, y), qj)| qj / (2.0 * std::f64::consts::PI * k * d) * (radius / (x - px).hypot(y - py).max(c.o("r_well", 0.1))).ln()).sum();
        out.push(Outcome::metre("drawdown", "Superposed drawdown at the point", "Überlagerte Absenkung am Punkt", drawdown));
    }
    Ok(out)
}

fn dew_03(c: &Case) -> R {
    let (dh, h, e) = (c.positive("delta_h")?, c.positive("h")?, c.positive("e_oed")?);
    let d_sigma = c.gamma_w() * dh;
    Ok(vec![Outcome::pascal("d_sigma", "Effective stress increase γw·Δh", "Zunahme der effektiven Spannung γw·Δh", d_sigma), Outcome::metre("s", "Settlement from drawdown", "Setzung infolge Absenkung", d_sigma * h / e)])
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "RW-07", part: "EN 1997-1", clause: "9.7", en: "Embedded retaining walls", de: "Verankerte und eingespannte Wände", inputs: &["h", "gamma", "phi_deg", "delta_a_deg", "delta_p_deg", "q", "kind", "prop_depth", "cu_k", "nc", "approach", "situation"], eval: rw_07 },
    Entry { id: "ANC-01", part: "EN 1997-1", clause: "8.5", en: "Anchor pull-out from suitability tests", de: "Ankerherausziehwiderstand aus Eignungsprüfungen", inputs: &["ra1", "xi_a", "a_d", "approach", "situation"], eval: anc_01 },
    Entry { id: "ANC-02", part: "EN 1997-1", clause: "8.5", en: "Anchor fixed length, empirical", de: "Ankerverpresskörper, Erfahrungswerte", inputs: &["d", "l_f", "tau_f", "a_d", "approach", "situation"], eval: anc_02 },
    Entry { id: "ANC-03", part: "EN 1997-1", clause: "8.6", en: "Anchor force, proof load and external stability (Kranz)", de: "Ankerkraft, Prüfkraft und äußere Standsicherheit (Kranz)", inputs: &["e_h_d", "theta_deg", "a_k", "t_w", "z_m", "x_m", "gamma", "phi_deg", "e1", "e2", "q", "eta_kranz"], eval: anc_03 },
    Entry { id: "ANC-04", part: "DIN EN 1537", clause: "8.7", en: "Anchor tendon and free length", de: "Ankerstahl und freie Länge", inputs: &["a_d", "p_t_k", "gamma_s", "l_min", "l_free", "tendon", "e_t", "a_t"], eval: anc_04 },
    Entry { id: "DEW-01", part: "EN 1997-1", clause: "10", en: "Single well discharge", de: "Einzelbrunnen", inputs: &["k", "h_aquifer", "h_well", "r_well", "aquifer", "thickness", "radius", "r_influence", "q_well"], eval: dew_01 },
    Entry { id: "DEW-02", part: "EN 1997-1", clause: "10", en: "Well group and drawdown superposition", de: "Brunnengruppe und Absenkungsüberlagerung", inputs: &["k", "h_aquifer", "h_well", "area", "radius", "r_influence", "x1", "y1", "qw1", "px", "py", "thickness", "r_well"], eval: dew_02 },
    Entry { id: "DEW-03", part: "EN 1997-1", clause: "10", en: "Settlement from groundwater drawdown", de: "Setzung infolge Grundwasserabsenkung", inputs: &["delta_h", "h", "e_oed", "gamma_w"], eval: dew_03 },
];
