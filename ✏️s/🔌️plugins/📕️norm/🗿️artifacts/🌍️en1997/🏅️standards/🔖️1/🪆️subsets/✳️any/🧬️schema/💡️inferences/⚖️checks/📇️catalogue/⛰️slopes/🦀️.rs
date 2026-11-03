//! ⛰️ Slope and overall stability, catalogue entries SLOPE-01 … SLOPE-05 (EN 1997-1 §11, DIN 4084, DIN 1054 GZ 1C).

use super::*;

pub use semio_s_plugin_norm_mechanics::geotechnics::slope::{bishop, fellenius, janbu, slices, Circle, Slice};

fn slope_factors(c: &Case) -> AnnexParams {
    resolve_params(parse_design_approach(c.so("approach", "geo3")), c.annex, c.so("situation", "bsP"))
}

fn circle_input(c: &Case) -> Result<Circle, String> {
    Ok(Circle {
        h: c.positive("h")?,
        beta_deg: c.positive("beta_deg")?,
        xc: c.n("xc")?,
        yc: c.positive("yc")?,
        r: c.positive("r")?,
        gamma: c.positive("gamma")?,
        gamma_sat: c.o("gamma_sat", c.n("gamma")?),
        gwl_level: c.o("gwl_level", -1.0e3),
        ru: c.o("ru", 0.0),
        q: c.o("q", 0.0) * slope_factors(c).gamma_q,
    })
}

fn design_strength(c: &Case) -> (f64, f64, AnnexParams) {
    let p = slope_factors(c);
    ((rad(c.o("phi_k_deg", 0.0)).tan()) / p.gamma_phi, c.o("c_k", 0.0) / p.gamma_c, p)
}

fn method_outcomes(key: &str, en: &str, de: &str, f: Option<f64>) -> Result<Vec<Outcome>, String> {
    let f = f.ok_or("slip circle does not cut the slope")?;
    Ok(vec![Outcome::ratio(key, en, de, 1.0, f), Outcome::number("f", "Factor of safety F", "Sicherheit F", f)])
}

fn slope_01(c: &Case) -> R {
    if let (Some(arc), Some(w), Some(xw)) = (c.opt("arc_deg"), c.opt("w"), c.opt("x_w")) {
        let p = slope_factors(c);
        let (r, cu) = (c.positive("r")?, c.positive("cu_k")?);
        let m_r = cu * r * rad(arc) * r;
        let m_rd = m_r / p.gamma_cu;
        return Ok(vec![Outcome::moment("undrained_circle", "Undrained circle M_D vs M_R,d", "Undränierter Kreis M_D gegen M_R,d", w * xw, m_rd), Outcome::newton_metre("m_r_k", "Characteristic resisting moment", "Charakteristisches Widerstandsmoment", m_r)]);
    }
    let s = circle_input(c)?;
    let (tan_phi, cd, _) = design_strength(c);
    method_outcomes("bishop", "Bishop utilisation 1/F", "Bishop-Ausnutzung 1/F", bishop(&s, tan_phi, cd, c.o("slices", 60.0) as usize))
}

fn slope_01b(c: &Case) -> R {
    let s = circle_input(c)?;
    let (tan_phi, cd, _) = design_strength(c);
    method_outcomes("fellenius", "Fellenius utilisation 1/F", "Fellenius-Ausnutzung 1/F", fellenius(&s, tan_phi, cd, c.o("slices", 60.0) as usize))
}

fn slope_02(c: &Case) -> R {
    let s = circle_input(c)?;
    let (tan_phi, cd, _) = design_strength(c);
    method_outcomes("janbu", "Janbu utilisation 1/F", "Janbu-Ausnutzung 1/F", janbu(&s, tan_phi, cd, c.o("slices", 60.0) as usize))
}

fn slope_03(c: &Case) -> R {
    let (beta, p) = (rad(c.positive("beta_deg")?), slope_factors(c));
    let tan_phi_k = rad(c.positive("phi_k_deg")?).tan();
    let tan_phi_d = tan_phi_k / p.gamma_phi;
    let (gamma, gamma_sat) = (c.o("gamma", 0.0), c.o("gamma_sat", c.o("gamma", 0.0)));
    let seep = c.o("seepage", 0.0) > 0.5;
    let f_k = if seep {
        (gamma_sat - c.gamma_w()) / gamma_sat * tan_phi_k / beta.tan()
    } else if let (Some(ck), Some(z)) = (c.opt("c_k"), c.opt("z")) {
        ck / (gamma * z * beta.sin() * beta.cos()) + tan_phi_k / beta.tan()
    } else {
        tan_phi_k / beta.tan()
    };
    let mu = if seep {
        beta.tan() / ((gamma_sat - c.gamma_w()) / gamma_sat * tan_phi_d)
    } else if let (Some(ck), Some(z)) = (c.opt("c_k"), c.opt("z")) {
        1.0 / (ck / p.gamma_c / (gamma * z * beta.sin() * beta.cos()) + tan_phi_d / beta.tan())
    } else {
        beta.tan() / tan_phi_d
    };
    Ok(vec![Outcome::ratio("infinite_slope", "Infinite slope utilisation", "Ausnutzung unendlich langer Hang", mu, 1.0), Outcome::number("f_k", "Characteristic factor of safety", "Charakteristische Sicherheit", f_k)])
}

fn slope_04(c: &Case) -> R {
    let (h, beta, gamma) = (c.positive("h")?, rad(c.positive("beta_deg")?), c.positive("gamma")?);
    let (tan_phi, cd, _) = design_strength(c);
    let mut best = f64::INFINITY;
    let mut best_theta = 0.0;
    for i in 1..2000 {
        let theta = beta * i as f64 / 2000.0;
        let w = 0.5 * gamma * h * h * (1.0 / theta.tan() - 1.0 / beta.tan());
        let l = h / theta.sin();
        let f = (cd * l + w * theta.cos() * tan_phi) / (w * theta.sin());
        if f < best {
            best = f;
            best_theta = theta;
        }
    }
    Ok(vec![Outcome::ratio("wedge", "Plane wedge utilisation 1/F", "Ausnutzung ebener Keil 1/F", 1.0, best), Outcome::number("f", "Minimum factor of safety F", "Minimale Sicherheit F", best), Outcome::number("theta_deg", "Critical plane angle (°)", "Kritischer Gleitflächenwinkel (°)", best_theta.to_degrees())])
}

/// 🧩️ Bishop forces on one slice at the converged factor of safety: driving `W sin α`, mobilised base shear `T = (c' b / cos α + N' tan φ') / F` and effective base normal `N' = (W − u − c' b tan α / F) / m_α`, all per metre of section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliceForces {
    pub slice: Slice,
    pub driving: f64,
    pub shear: f64,
    pub normal: f64,
}

/// 🧩️ The factor of safety of a circle with the force polygon of every slice (Σ T = Σ W sin α at the converged factor).
#[derive(Clone, Debug, PartialEq)]
pub struct SlipAnalysis {
    pub factor: f64,
    pub slices: Vec<SliceForces>,
}

/// 🧩️ Simplified Bishop analysis with per-slice forces, the read-only twin of [`bishop`] used by the scene diagrams.
pub fn bishop_forces(s: &Circle, tan_phi: f64, c: f64, n: usize) -> Option<SlipAnalysis> {
    let factor = bishop(s, tan_phi, c, n)?;
    let forces = slices(s, n)?
        .into_iter()
        .map(|k| {
            let m = k.alpha.cos() + k.alpha.sin() * tan_phi / factor;
            let normal = ((k.w - k.u).max(0.0) - c * k.b * k.alpha.tan() / factor) / m;
            SliceForces { slice: k, driving: k.w * k.alpha.sin(), shear: (c * k.b / k.alpha.cos() + normal * tan_phi) / factor, normal }
        })
        .collect();
    Some(SlipAnalysis { factor, slices: forces })
}

/// 🎯️ Factor of safety for shifted centres of a circle of fixed radius: `(shift, F)` pairs along x and along z (`steps` per side, `reach` in metres), `None` entries where no sliding mass develops; the live curve of the centre drag.
pub fn centre_sweep(s: &Circle, tan_phi: f64, c: f64, n: usize, reach: f64, steps: usize) -> (Vec<(f64, Option<f64>)>, Vec<(f64, Option<f64>)>) {
    let shifts: Vec<f64> = (0..=2 * steps).map(|i| reach * (i as f64 / steps as f64 - 1.0)).collect();
    let along_x = shifts.iter().map(|d| (*d, bishop(&Circle { xc: s.xc + d, ..*s }, tan_phi, c, n))).collect();
    let along_z = shifts.iter().map(|d| (*d, bishop(&Circle { yc: s.yc + d, ..*s }, tan_phi, c, n))).collect();
    (along_x, along_z)
}

/// 🔎️ Coarse deterministic search of the circle with the smallest Bishop factor of safety: centres on a grid above the slope, radii through and past the toe; `n` slices per circle.
pub fn critical_circle(proto: &Circle, tan_phi: f64, c: f64, n: usize) -> Option<(Circle, f64)> {
    let run = proto.h / rad(proto.beta_deg).tan();
    let mut best: Option<(Circle, f64)> = None;
    for ix in 0..7 {
        for iy in 0..6 {
            let (xc, yc) = (-0.5 * proto.h + (run + proto.h) * ix as f64 / 6.0, 0.6 * proto.h + 1.8 * proto.h * iy as f64 / 5.0);
            let to_toe = (run - xc).hypot(yc);
            for k in [1.0, 1.08, 1.18, 1.3] {
                let circle = Circle { xc, yc, r: to_toe * k, ..*proto };
                if let Some(f) = bishop(&circle, tan_phi, c, n) {
                    if f.is_finite() && f > 0.0 && best.as_ref().is_none_or(|b| f < b.1) {
                        best = Some((circle, f));
                    }
                }
            }
        }
    }
    best
}

/// 📐️ Bishop factor of the circle extruded to a slip body of finite width with cohesive end planes: `(F_2D, F_3D, end gain, volume)`; the end faces carry the design cohesion only (frictionless, no normal stress), F_3D = F_2D + 2·c_d·Σ(A_i·ρ_i)/(r·Σ W_i sin α_i·B) with ρ_i the distance of the slice centroid from the circle axis.
pub fn bishop_extruded(s: &Circle, tan_phi: f64, c: f64, width: f64, n: usize) -> Option<(f64, f64, f64, f64)> {
    let f2 = bishop(s, tan_phi, c, n)?;
    let sl = slices(s, n)?;
    let drive: f64 = sl.iter().map(|k| k.w * k.alpha.sin()).sum();
    let (moment_arm_area, area): (f64, f64) = sl.iter().fold((0.0, 0.0), |(m, a), k| {
        let rho = ((k.x - s.xc).powi(2) + (k.yb + 0.5 * k.h - s.yc).powi(2)).sqrt();
        (m + k.b * k.h * rho, a + k.b * k.h)
    });
    let gain = 2.0 * c * moment_arm_area / (s.r * drive * width);
    Some((f2, f2 + gain, gain, area * width))
}

/// 🧩️ Spencer factors `(F_m, F_f)` of the circle for the constant interslice inclination `lambda = tan θ`: moment equilibrium about the centre and horizontal force equilibrium, each solved on its own factor; `None` where the iteration diverges. At `lambda = 0` the moment factor is the simplified Bishop factor.
pub fn spencer_pair(sl: &[Slice], tan_phi: f64, c: f64, lambda: f64) -> Option<(f64, f64)> {
    let drive: f64 = sl.iter().map(|k| k.w * k.alpha.sin()).sum();
    if drive <= 0.0 {
        return None;
    }
    let forces = |k: &Slice, f: f64| {
        let (ca, sa) = (k.alpha.cos(), k.alpha.sin());
        let normal = ((k.w - k.u / ca * (ca + lambda * sa) - c * k.b / ca * (sa - lambda * ca) / f) / (ca + lambda * sa + tan_phi / f * (sa - lambda * ca))).max(0.0);
        (c * k.b / ca + normal * tan_phi, normal + k.u / ca)
    };
    let solve = |step: &dyn Fn(f64) -> f64| {
        let mut f = 1.5;
        for _ in 0..300 {
            let next = step(f);
            if !next.is_finite() || next <= 0.0 {
                return None;
            }
            if (next - f).abs() < 1e-10 {
                return Some(next);
            }
            f = next;
        }
        None
    };
    let moment = solve(&|f| sl.iter().map(|k| forces(k, f).0).sum::<f64>() / drive)?;
    let force = solve(&|f| sl.iter().map(|k| forces(k, f).0 * k.alpha.cos()).sum::<f64>() / sl.iter().map(|k| forces(k, f).1 * k.alpha.sin()).sum::<f64>())?;
    Some((moment, force))
}

/// 🧩️ Spencer's method: the interslice inclination `lambda = tan θ` in [-1, 1] nearest to zero at which the moment and the force factor agree, bracketed on a 0.05 grid and bisected; `(F, lambda)` or `None` without a sliding mass or a root.
pub fn spencer(s: &Circle, tan_phi: f64, c: f64, n: usize) -> Option<(f64, f64)> {
    let sl = slices(s, n)?;
    let gap = |l: f64| spencer_pair(&sl, tan_phi, c, l).map(|(m, f)| m - f);
    let grid: Vec<f64> = (0..=40).map(|i| -1.0 + 0.05 * f64::from(i)).collect();
    let (lo, hi) = grid.windows(2).filter_map(|w| Some((w[0], w[1], gap(w[0])?, gap(w[1])?))).filter(|(_, _, a, b)| a * b <= 0.0).min_by(|x, y| (x.0 + x.1).abs().total_cmp(&(y.0 + y.1).abs())).map(|(a, b, _, _)| (a, b))?;
    let (mut lo, mut hi) = (lo, hi);
    let sign = gap(lo)?;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if gap(mid)? * sign > 0.0 { lo = mid } else { hi = mid }
    }
    let lambda = 0.5 * (lo + hi);
    Some((spencer_pair(&sl, tan_phi, c, lambda)?.0, lambda))
}

fn slope_06(c: &Case) -> R {
    let s = circle_input(c)?;
    let (tan_phi, cd, _) = design_strength(c);
    let n = c.o("slices", 60.0) as usize;
    let (f, lambda) = spencer(&s, tan_phi, cd, n).ok_or("no interslice inclination closes moment and force equilibrium for this circle")?;
    let mut out = vec![Outcome::ratio("spencer", "Spencer utilisation 1/F", "Spencer-Ausnutzung 1/F", 1.0, f), Outcome::number("f", "Spencer factor of safety F", "Spencer-Sicherheit F", f), Outcome::number("theta_deg", "Interslice force inclination θ (°)", "Neigung θ der Lamellenkräfte (°)", lambda.atan().to_degrees())];
    if let Some(fb) = bishop(&s, tan_phi, cd, n) {
        out.push(Outcome::number("f_bishop", "Simplified Bishop factor F", "Vereinfachter Bishop-Faktor F", fb));
    }
    Ok(out)
}

fn slope_05(c: &Case) -> R {
    let s = circle_input(c)?;
    let (tan_phi, cd, _) = design_strength(c);
    let width = c.positive("width")?;
    let (f2, f3, gain, volume) = bishop_extruded(&s, tan_phi, cd, width, c.o("slices", 60.0) as usize).ok_or("slip circle does not cut the slope")?;
    Ok(vec![
        Outcome::ratio("bishop_3d", "Extruded slip body utilisation 1/F₃D", "Ausnutzung extrudierter Gleitkörper 1/F₃D", 1.0, f3),
        Outcome::number("f_2d", "Plane-strain factor F₂D", "Ebener Sicherheitsfaktor F₂D", f2),
        Outcome::number("f_3d", "Extruded factor F₃D", "Extrudierter Sicherheitsfaktor F₃D", f3),
        Outcome::number("end_gain", "Gain from cohesive end planes", "Zuwachs aus kohäsiven Stirnflächen", gain),
        Outcome::number("volume", "Slip body volume (m³)", "Volumen des Gleitkörpers (m³)", volume),
    ])
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "SLOPE-01", part: "EN 1997-1", clause: "11.5", en: "Overall stability, simplified Bishop", de: "Gesamtstandsicherheit, Bishop", inputs: &["h", "beta_deg", "xc", "yc", "r", "gamma", "gamma_sat", "gwl_level", "ru", "q", "phi_k_deg", "c_k", "slices", "arc_deg", "w", "x_w", "cu_k", "approach", "situation"], eval: slope_01 },
    Entry { id: "SLOPE-01B", part: "EN 1997-1", clause: "11.5", en: "Overall stability, Fellenius", de: "Gesamtstandsicherheit, Fellenius", inputs: &["h", "beta_deg", "xc", "yc", "r", "gamma", "gamma_sat", "gwl_level", "ru", "q", "phi_k_deg", "c_k", "slices", "approach", "situation"], eval: slope_01b },
    Entry { id: "SLOPE-02", part: "EN 1997-1", clause: "11.5", en: "Overall stability, Janbu simplified", de: "Gesamtstandsicherheit, Janbu", inputs: &["h", "beta_deg", "xc", "yc", "r", "gamma", "gamma_sat", "gwl_level", "ru", "q", "phi_k_deg", "c_k", "slices", "approach", "situation"], eval: slope_02 },
    Entry { id: "SLOPE-03", part: "EN 1997-1", clause: "11.5", en: "Infinite slope", de: "Unendlich langer Hang", inputs: &["beta_deg", "phi_k_deg", "c_k", "z", "gamma", "gamma_sat", "seepage", "gamma_w", "approach", "situation"], eval: slope_03 },
    Entry { id: "SLOPE-04", part: "EN 1997-1", clause: "11.5", en: "Plane wedge failure", de: "Ebener Gleitkeil", inputs: &["h", "beta_deg", "gamma", "phi_k_deg", "c_k", "approach", "situation"], eval: slope_04 },
    Entry { id: "SLOPE-05", part: "EN 1997-1", clause: "11.5", en: "Overall stability, extruded 3D slip body with end planes", de: "Gesamtstandsicherheit, extrudierter 3D-Gleitkörper mit Stirnflächen", inputs: &["h", "beta_deg", "xc", "yc", "r", "gamma", "gamma_sat", "gwl_level", "ru", "q", "phi_k_deg", "c_k", "width", "slices", "approach", "situation"], eval: slope_05 },
    Entry { id: "SLOPE-06", part: "EN 1997-1", clause: "11.5", en: "Overall stability, Spencer full equilibrium", de: "Gesamtstandsicherheit, Spencer mit vollständigem Gleichgewicht", inputs: &["h", "beta_deg", "xc", "yc", "r", "gamma", "gamma_sat", "gwl_level", "ru", "q", "phi_k_deg", "c_k", "slices", "approach", "situation"], eval: slope_06 },
];
