//! 🛋️ Serviceability of EN 1995-1-1 chapter 7 as catalogue entries T-60 … T-65: instantaneous and final deflection, slip, floor vibration frequency, unit point load deflection and unit impulse velocity response. See [`super`].

use super::*;
use std::f64::consts::PI;

/// 📐 Bending plus shear midspan or tip deflection in mm for a standard load case (E, G in N/mm², sections in mm).
pub fn deflection_case(case: &str, load: f64, l: f64, e: f64, i: f64, g: f64, a: f64, kappa: f64) -> f64 {
    let (ei, ga) = (e * i, g * a);
    match case {
        "udl_ss" => 5.0 * load * l.powi(4) / (384.0 * ei) + kappa * load * l * l / (8.0 * ga),
        "point_mid" => load * l.powi(3) / (48.0 * ei) + kappa * load * l / (4.0 * ga),
        "cantilever_udl" => load * l.powi(4) / (8.0 * ei) + kappa * load * l * l / (2.0 * ga),
        "cantilever_point" => load * l.powi(3) / (3.0 * ei) + kappa * load * l / ga,
        _ => 0.0,
    }
}

/// 🔢 Midspan deflection of a simply supported beam under a uniform load with linearly varying depth, by unit-load integration (bending and shear), in mm.
pub fn deflection_tapered_udl(q: f64, l: f64, b: f64, h_a: f64, h_b: f64, e: f64, g: f64, kappa: f64) -> f64 {
    let n = 4000;
    let dx = l / n as f64;
    let mut w = 0.0;
    for k in 0..n {
        let x = (k as f64 + 0.5) * dx;
        let h = h_a + (h_b - h_a) * x / l;
        let (mom, shear) = (q * x * (l - x) / 2.0, q * (l / 2.0 - x));
        let (um, us) = (if x < l / 2.0 { x / 2.0 } else { (l - x) / 2.0 }, if x < l / 2.0 { 0.5 } else { -0.5 });
        w += (mom * um / (e * b * h.powi(3) / 12.0) + kappa * shear * us / (g * b * h)) * dx;
    }
    w
}

struct Defl {
    wg: f64,
    wq: f64,
    leff: f64,
}

fn deflections(c: &Case) -> Result<(Defl, Mat), String> {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let (l, case) = (c.n("l")?, c.so("load_case", "udl_ss"));
    if case.starts_with("two_span") {
        return Err(format!("load case {case} only defines a moment line"));
    }
    let kappa = c.o("kappa", 1.2);
    let (e, g) = (m.row.e_0_mean, m.row.g_mean);
    let one = |load: f64| {
        if case == "udl_ss_tapered" {
            deflection_tapered_udl(load, l, s.b, c.o("h_a", s.h), c.o("h_b", s.h), e, g, kappa)
        } else {
            deflection_case(case, load, l, e, s.iy, g, s.a, kappa)
        }
    };
    let leff = if case.starts_with("cantilever") { 2.0 * l } else { l };
    Ok((Defl { wg: one(c.n("load_g")?), wq: one(c.o("load_q", 0.0)), leff }, m))
}

fn t60(c: &Case) -> R {
    let (d, _) = deflections(c)?;
    let div = c.opt("limit_divisor").unwrap_or(300.0);
    Ok(vec![
        Outcome::info("w_inst_g", "w_inst,G permanent", "w_inst,G ständig", QuantityKind::Length, d.wg / 1e3, format!("w_inst,G = {} mm", f(d.wg))),
        Outcome::length("w_inst_q", "w_inst,Q variable (7.2)", "w_inst,Q veränderlich (7.2)", d.wq, d.leff / div, format!("w_inst,Q = {} mm ≤ L_eff/{} = {} mm", f(d.wq), f(div), f(d.leff / div))),
    ])
}

/// 🧭 What the deflection diagram of T-61 needs: the instantaneous parts in mm, the combination factors, the camber and the two limits in mm; [`DeflectionProbe::at`] gives the final and net final deflection at another k_def.
#[derive(Clone, Copy, Debug)]
pub struct DeflectionProbe {
    pub w_g: f64,
    pub w_q: f64,
    pub w_q2: f64,
    pub psi_2: f64,
    pub psi_0_2: f64,
    pub psi_2_2: f64,
    pub k_def: f64,
    pub w_c: f64,
    pub limit_fin: f64,
    pub limit_net: f64,
}

impl DeflectionProbe {
    /// 🧭 w_fin = w_G(1 + k_def) + w_Q(1 + ψ₂ k_def) + w_Q2(ψ₀ + ψ₂ k_def) and w_net,fin = w_fin − w_c (EN 1995-1-1 2.2.3, 7.2) in mm.
    pub fn at(&self, k_def: f64) -> (f64, f64) {
        let fin = self.w_g * (1.0 + k_def) + self.w_q * (1.0 + self.psi_2 * k_def) + self.w_q2 * (self.psi_0_2 + self.psi_2_2 * k_def);
        (fin, fin - self.w_c)
    }
}

/// 🧭 The [`DeflectionProbe`] of a T-61 case; `k_def` is the input or Table 3.2 for the material and service class.
pub fn deflection_probe(c: &Case) -> Result<DeflectionProbe, String> {
    let (d, m) = deflections(c)?;
    let sc = c.o("sc", 1.0) as u8;
    let k_def = c.opt("k_def").or_else(|| k_def_for(c.so("material", product_key(m.row.product)), sc)).ok_or("k_def not allowed")?;
    let div_net = c.opt("limit_net_divisor").unwrap_or(if c.de() { 200.0 } else { 250.0 });
    Ok(DeflectionProbe { w_g: d.wg, w_q: d.wq, w_q2: c.o("w_q2", 0.0), psi_2: c.o("psi_2", 0.3), psi_0_2: c.o("psi_0_2", 0.0), psi_2_2: c.o("psi_2_2", 0.0), k_def, w_c: c.o("w_c", 0.0), limit_fin: d.leff / c.o("limit_fin_divisor", 150.0), limit_net: d.leff / div_net })
}

fn t61(c: &Case) -> R {
    let p = deflection_probe(c)?;
    let (w_fin, net) = p.at(p.k_def);
    Ok(vec![
        Outcome::length("w_fin", "Final deflection w_fin", "Enddurchbiegung w_fin", w_fin, p.limit_fin, format!("w_fin = w_G(1+k_def) + w_Q(1+ψ₂k_def) = {} mm", f(w_fin))),
        Outcome::length("w_net_fin", "Net final deflection w_net,fin (7.2)", "Netto-Enddurchbiegung w_net,fin (7.2)", net, p.limit_net, format!("w_net,fin = {} − w_c = {} mm ≤ {} mm", f(w_fin), f(net), f(p.limit_net))),
    ])
}

fn t62(c: &Case) -> R {
    let (l, q) = (c.n("l")?, c.n("load_g")?);
    let parts = components::parts_from(c)?;
    let k = c.n("k_ser")?;
    let g = gamma_beam(&parts, k, l);
    let w = 5.0 * q * l.powi(4) / (384.0 * g.ei_ef);
    let rigid = g.ei_rigid;
    let w0 = 5.0 * q * l.powi(4) / (384.0 * rigid);
    Ok(vec![
        Outcome::length("w_slip", "Deflection with slip (EI)_ef", "Durchbiegung mit Nachgiebigkeit (EI)_ef", w, l / c.o("limit_divisor", 300.0), format!("(EI)_ef = {} N·mm², w = {} mm (rigid {} mm)", f(g.ei_ef), f(w), f(w0))),
        Outcome::ratio("slip_stiffness_ratio", "(EI)_ef/(EI)_rigid", "(EI)_ef/(EI)_starr", g.ei_ef / rigid.max(1.0), 1.0, "ratio ≤ 1".into()),
    ])
}

/// 🌊 Fundamental floor frequency f₁ = π/(2ℓ²)·√((EI)_ℓ/m) (EN 1995-1-1 7.5), ℓ in m, (EI)_ℓ in N·m²/m, m in kg/m².
pub fn floor_f1(span: f64, ei_l: f64, mass: f64) -> f64 {
    PI / (2.0 * span * span) * (ei_l / mass).sqrt()
}

fn t63(c: &Case) -> R {
    let (l, ei, m) = (c.n("span_m")?, c.n("ei_l")?, c.n("mass_kg_m2")?);
    let f1 = floor_f1(l, ei, m);
    let lim = c.opt("f1_limit").unwrap_or_else(|| ndp_number("ndp.en1995.floor.f1-limit", c.annex));
    let mut out = vec![Outcome::ratio("f1", "Fundamental frequency f₁ ≥ limit (7.3.3)", "Eigenfrequenz f₁ ≥ Grenze (7.3.3)", lim, f1, format!("f₁ = π/(2·{}²)·√({}/{}) = {} Hz, limit {} Hz", f(l), f(ei), f(m), f(f1), f(lim)))];
    if c.de() && f1 >= lim && f1 < ndp_number("ndp.en1995.floor.na-de-additional-criterion", c.annex) {
        out.push(Outcome::info("additional_criterion", "6 Hz ≤ f₁ < 8 Hz: unit-load criterion T-64 mandatory", "6 Hz ≤ f₁ < 8 Hz: Einzellastkriterium T-64 erforderlich", QuantityKind::Dimensionless, 1.0, "NCI to 7.3.3".into()));
    }
    Ok(out)
}

/// 📏 Deflection in mm of a floor strip under a point load F: w = F·ℓ³/(48·(EI)_ℓ·b_ef) with b_ef = min(b, ℓ/1.1·((EI)_b/(EI)_ℓ)^0.25) (ℓ, b in m).
pub fn unit_point_deflection(force: f64, span: f64, width: f64, ei_l: f64, ei_b: f64) -> f64 {
    let bef = width.min(span / 1.1 * (ei_b / ei_l).powf(0.25));
    force * span.powi(3) / (48.0 * ei_l * bef) * 1e3
}

fn t64(c: &Case) -> R {
    let force = c.o("force_n", 1000.0);
    let w = unit_point_deflection(force, c.n("span_m")?, c.n("width_m")?, c.n("ei_l")?, c.n("ei_b")?) * 1000.0 / force;
    let a = c.opt("a_mm_per_kn").unwrap_or_else(|| ndp_number("ndp.en1995.floor.unit-load-a", c.annex));
    Ok(vec![Outcome::ratio("w_unit", "Unit point load deflection w ≤ a (7.3.3)", "Verformung unter Einzellast w ≤ a (7.3.3)", w, a, format!("w = {} mm per kN ≤ a = {}", f(w), f(a)))])
}

/// 🌊 Velocity response v = 4(0.4 + 0.6·n₄₀)/(m·b·ℓ + 200) with n₄₀ = {[(40/f₁)² − 1]·(b/ℓ)⁴·(EI)_ℓ/(EI)_b}^0.25 (EN 1995-1-1 7.6, 7.7); zero modes for f₁ ≥ 40 Hz.
pub fn velocity_response(f1: f64, span: f64, width: f64, mass: f64, ei_l: f64, ei_b: f64) -> (f64, f64) {
    let n40 = (((40.0 / f1).powi(2) - 1.0).max(0.0) * (width / span).powi(4) * ei_l / ei_b).powf(0.25);
    (4.0 * (0.4 + 0.6 * n40) / (mass * width * span + 200.0), n40)
}

/// 🏢 Exact first frequency of a simply supported orthotropic floor plate without torsion, f₁ = π/2·√((EI)_ℓ/m)·√(1/ℓ⁴ + (EI)_b/((EI)_ℓ b⁴)) (ℓ span and b width in m, stiffnesses in N·m²/m, m in kg/m²); the mode is [`floor_plate_mode`].
pub fn floor_plate_f1(span: f64, width: f64, ei_l: f64, ei_b: f64, mass: f64) -> f64 {
    PI / 2.0 * (ei_l / mass).sqrt() * (span.powi(-4) + ei_b / ei_l * width.powi(-4)).sqrt()
}

/// 🏢 First mode shape sin(πx/ℓ)·sin(πy/b) of the floor plate at the plan point (x, y) in m, largest value 1.
pub fn floor_plate_mode(x: f64, y: f64, span: f64, width: f64) -> f64 {
    (PI * x / span).sin() * (PI * y / width).sin()
}

/// 🌡 Peak acceleration per unit impulse at the antinode in m/s² per N·s: the unit impulse velocity response v of 7.3.3 (with the beam frequency of the check) times the angular frequency 2π f₁ of the plate mode; the value at a plan point is this times |φ|.
pub fn floor_plate_peak_acceleration(span: f64, width: f64, ei_l: f64, ei_b: f64, mass: f64) -> f64 {
    let (v, _) = velocity_response(floor_f1(span, ei_l, mass), span, width, mass, ei_l, ei_b);
    2.0 * PI * floor_plate_f1(span, width, ei_l, ei_b, mass) * v
}

fn t65(c: &Case) -> R {
    let (l, bw, ei_l, ei_b, m, z) = (c.n("span_m")?, c.n("width_m")?, c.n("ei_l")?, c.n("ei_b")?, c.n("mass_kg_m2")?, c.o("zeta", 0.01));
    let f1 = floor_f1(l, ei_l, m);
    let (v, n40) = velocity_response(f1, l, bw, m, ei_l, ei_b);
    let lim = c.o("b_vel", 100.0).powf(f1 * z - 1.0);
    Ok(vec![Outcome::ratio("velocity", "Unit impulse velocity v ≤ b^(f₁ζ−1) (7.3.3)", "Einheitsimpuls-Geschwindigkeit v ≤ b^(f₁ζ−1) (7.3.3)", v * 1e3, lim * 1e3, format!("f₁={} Hz n₄₀={} v={} m/(Ns²) limit {}", f(f1), f(n40), f(v), f(lim)))])
}

/// ⏳ Creep factor φ(t) = k_def (1 − e^(−t/τ)) of the deflection time curve; the end value φ(∞) = k_def is the normative one of EN 1995-1-1 2.3.2.2, the time constant τ is the registry value `ndp.en1995.creep.time-constant-years` (a modelling assumption, unverified).
pub fn creep_factor(k_def: f64, years: f64) -> f64 {
    k_def * (1.0 - (-years.max(0.0) / ndp_common("ndp.en1995.creep.time-constant-years")).exp())
}

/// ⏳ Deflection at age `years` of a member with instantaneous permanent and variable parts: w(t) = w_G (1 + φ) + w_Q (1 + ψ₂ φ); at t = 0 the instantaneous sum, for t → ∞ the final deflection of 7.2 (2).
pub fn creep_deflection(w_g: f64, w_q: f64, psi_2: f64, k_def: f64, years: f64) -> f64 {
    let phi = creep_factor(k_def, years);
    w_g * (1.0 + phi) + w_q * (1.0 + psi_2 * phi)
}

/// 📏 Midspan deflection in m of a simply supported floor strip of width 1 m under the area load `q` in N/m²: 5 q ℓ⁴/(384 (EI)_ℓ) with (EI)_ℓ in N·m²/m.
pub fn floor_strip_deflection(q: f64, span: f64, ei_l: f64) -> f64 {
    5.0 * q * span.powi(4) / (384.0 * ei_l)
}

/// 🗂 Catalogue rows of the serviceability group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-60", part: "EN 1995-1-1", clause: "7.2", en: "Instantaneous deflection", de: "Anfangsdurchbiegung", inputs: &["class", "l", "b", "h", "load_case", "load_g", "load_q", "h_a", "h_b"], eval: t60 },
    Entry { id: "T-61", part: "EN 1995-1-1", clause: "7.2", en: "Final and net final deflection", de: "Enddurchbiegung und Netto-Enddurchbiegung", inputs: &["class", "sc", "l", "b", "h", "load_case", "load_g", "load_q", "psi_2", "w_c"], eval: t61 },
    Entry { id: "T-62", part: "EN 1995-1-1", clause: "7.2", en: "Slip contribution", de: "Nachgiebigkeit der Verbindung", inputs: &["l", "load_g", "k_ser", "a1", "i1", "e1", "h1", "a2", "i2", "e2", "h2", "a3", "i3", "e3", "h3", "s1", "s3"], eval: t62 },
    Entry { id: "T-63", part: "EN 1995-1-1", clause: "7.3.3", en: "Floor vibration frequency", de: "Eigenfrequenz der Decke", inputs: &["span_m", "ei_l", "mass_kg_m2", "f1_limit"], eval: t63 },
    Entry { id: "T-64", part: "EN 1995-1-1", clause: "7.3.3", en: "Unit point load deflection", de: "Verformung unter Einheitslast", inputs: &["span_m", "width_m", "ei_l", "ei_b", "force_n", "a_mm_per_kn"], eval: t64 },
    Entry { id: "T-65", part: "EN 1995-1-1", clause: "7.3.3", en: "Unit impulse velocity response", de: "Einheitsimpuls-Geschwindigkeitsantwort", inputs: &["span_m", "width_m", "ei_l", "ei_b", "mass_kg_m2", "zeta", "b_vel"], eval: t65 },
];
