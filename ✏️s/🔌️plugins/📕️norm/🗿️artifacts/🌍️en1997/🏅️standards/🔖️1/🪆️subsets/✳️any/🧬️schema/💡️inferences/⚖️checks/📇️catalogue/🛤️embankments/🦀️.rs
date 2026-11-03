//! 🛤️ Fill embankments on soft ground, catalogue entry EMB-01 (EN 1997-1 section 12: undrained foundation failure, lateral spreading of the fill and the consolidation settlement under the centre line from the Osterberg stress of a trapezoidal fill, Das, Principles of Foundation Engineering). See [`super`].

use super::*;

const SUBLAYERS: usize = 20;

/// 📐️ Vertical stress at depth `z` under the centre line of a symmetric trapezoidal fill per unit fill pressure: crest half-width `half_crest`, slope run `run` (horizontal extent of one slope); Osterberg's closed form of one half, doubled.
pub fn osterberg_centre(half_crest: f64, run: f64, z: f64) -> f64 {
    let (b, a) = (half_crest, run);
    let alpha2 = (b / z).atan();
    let alpha1 = ((a + b) / z).atan() - alpha2;
    2.0 / std::f64::consts::PI * ((a + b) / a * (alpha1 + alpha2) - b / a * alpha2)
}

/// 📉️ Centre-line consolidation settlement [m] of a soft layer of thickness `t` and oedometric modulus `e_oed` under the fill pressure `q`, summed over 20 sublayers.
pub fn centre_settlement(q: f64, half_crest: f64, run: f64, t: f64, e_oed: f64) -> f64 {
    let dz = t / SUBLAYERS as f64;
    (0..SUBLAYERS).map(|i| q * osterberg_centre(half_crest, run, (i as f64 + 0.5) * dz) * dz / e_oed).sum()
}

fn emb_01(c: &Case) -> R {
    let (h, crest, slope, gamma) = (c.positive("h")?, c.positive("b_crest")?, c.positive("n_slope")?, c.positive("gamma")?);
    let (cu, t, e_oed) = (c.positive("cu_k")?, c.positive("t_soft")?, c.positive("e_oed")?);
    let (p, q) = (c.factors(), c.o("q", 0.0));
    let fill = gamma * h;
    let (run, base) = (slope * h, crest + 2.0 * slope * h);
    let cu_d = cu / p.gamma_cu;
    let phi_d = (rad(c.o("phi_deg", 30.0)).tan() / p.gamma_phi).atan();
    let ka = (std::f64::consts::FRAC_PI_4 - phi_d / 2.0).tan().powi(2);
    let thrust_g = 0.5 * ka * gamma * h * h;
    let thrust_q = ka * q * h;
    let s = centre_settlement(fill + q, crest / 2.0, run, t, e_oed);
    let mut out = vec![
        Outcome::pressure("bearing", "Fill pressure γG·γ·H + γQ·q vs (π + 2)·c_u,d / γR,v", "Dammlast γG·γ·H + γQ·q gegen (π + 2)·c_u,d / γR,v", p.gamma_g * fill + p.gamma_q * q, (std::f64::consts::PI + 2.0) * cu_d / p.gamma_r_v),
        Outcome::force("spreading", "Lateral thrust of the fill vs base shear c_u,d·B / γR,h", "Seitenschub der Schüttung gegen Sohlscherung c_u,d·B / γR,h", p.gamma_g * thrust_g + p.gamma_q * thrust_q, cu_d * base / p.gamma_r_h),
        Outcome::metre("s_centre", "Consolidation settlement under the centre line", "Konsolidationssetzung unter der Mittelachse", s),
        Outcome::pascal("q_fill", "Fill pressure γ·H", "Dammlast γ·H", fill),
        Outcome::metre("b_base", "Base width B = b + 2·n·H", "Aufstandsbreite B = b + 2·n·H", base),
        Outcome::pascal("sigma_z_mid", "Stress increase at mid-depth of the soft layer", "Spannungszuwachs in Schichtmitte der weichen Schicht", (fill + q) * osterberg_centre(crest / 2.0, run, t / 2.0)),
        Outcome::number("ka", "Active coefficient of the fill K_a", "Aktiver Beiwert der Schüttung K_a", ka),
    ];
    if let Some(limit) = c.opt("s_lim") {
        out.insert(0, Outcome::length("settlement", "Centre-line settlement vs limit", "Setzung der Mittelachse gegen Grenzwert", s, limit));
    }
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[Entry {
    id: "EMB-01",
    part: "EN 1997-1",
    clause: "12",
    en: "Embankment on soft ground: foundation failure, spreading and settlement",
    de: "Damm auf weichem Baugrund: Grundbruch, Seitenschub und Setzung",
    inputs: &["h", "b_crest", "n_slope", "gamma", "phi_deg", "cu_k", "t_soft", "e_oed", "q", "s_lim", "approach", "situation"],
    eval: emb_01,
}];
