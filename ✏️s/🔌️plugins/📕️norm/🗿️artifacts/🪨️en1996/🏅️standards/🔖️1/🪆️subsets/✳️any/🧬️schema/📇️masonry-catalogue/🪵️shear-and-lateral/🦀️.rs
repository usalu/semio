//! 🪵️ Shear and lateral load entries of the EN 1996 catalogue: M-20 … M-34 shear walls, bracing, one- and two-way bending, arching, wind and earth pressure, basement walls. See [`super`].

use super::*;
use super::numerical_models::{arching_pressure, yield_line_pressure, Edges};
use semio_s_artifact_norm_en1996_masonry_tables::annex_e::{self, Alpha};

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "M-20", part: "EN 1996-1-1", clause: "6.2", en: "Shear wall resistance with flanges", de: "Schubwandtragfähigkeit mit Flanschen", inputs: &["l_m", "t_m", "n_ed_n", "m_ed_nm", "v_ed_n", "fb_pa", "fm_pa", "fd_pa", "flange_t_m", "flange_len_m", "web_spacing_m", "height_total_m", "flanges"], eval: m20 },
    Entry { id: "M-21", part: "EN 1996-1-1", clause: "5.4", en: "Bracing wall layout and torsion", de: "Aussteifungsanordnung und Torsion", inputs: &["l_m", "t_m", "x_m", "vrd_n", "h_m", "e_pa", "v_ed_n", "b_m", "e0_m"], eval: m21 },
    Entry { id: "M-30", part: "EN 1996-1-1", clause: "6.3.1", en: "One-way vertical span", de: "Einachsig vertikal gespannte Wand", inputs: &["t_m", "h_m", "w_ed_pa", "fxk1_pa", "gamma_m", "sigma_d_pa", "support"], eval: m30 },
    Entry { id: "M-31", part: "EN 1996-1-1", clause: "6.3.2", en: "Two-way bending by yield lines", de: "Zweiachsige Biegung nach Bruchlinientheorie", inputs: &["t_m", "h_m", "l_m", "w_ed_pa", "fxk1_pa", "fxk2_pa", "gamma_m", "sigma_d_pa", "edges", "alpha1", "wall"], eval: m31 },
    Entry { id: "M-32", part: "EN 1996-1-1", clause: "6.3.3", en: "Lateral load with arching", de: "Horizontallast mit Gewölbewirkung", inputs: &["t_m", "la_m", "fd_pa", "w_ed_pa"], eval: m32 },
    Entry { id: "M-33", part: "EN 1991-1-4", clause: "5.2, 6.3", en: "Wind and earth pressure on walls", de: "Wind- und Erddruck auf Wände", inputs: &["qp_pa", "cpe", "k0", "gamma_soil_n_m3", "he_m"], eval: m33 },
    Entry { id: "M-34", part: "EN 1996-3", clause: "4.5", en: "Basement walls under earth pressure", de: "Kellerwände unter Erddruck", inputs: &["h_m", "t_m", "he_m", "rho_e_n_m3", "qk_pa", "n_min_n", "n_max_n", "fd_pa", "hydrostatic"], eval: m34 },
];

/// 🧱️ Effective flange overhang per side min(s/2, 6 t_f, 0.2 H).
fn flange_overhang(web_spacing: f64, t_f: f64, h_total: f64) -> f64 {
    (0.5 * web_spacing).min(6.0 * t_f).min(0.2 * h_total)
}

fn m20(c: &Case) -> R {
    let (l, t) = (c.n("l_m")?, c.n("t_m")?);
    let (n, m, v) = (c.n("n_ed_n")?, c.o("m_ed_nm", 0.0), c.n("v_ed_n")?);
    let e = m / n.max(1e-9);
    let lc = (3.0 * (l / 2.0 - e)).clamp(0.0, l);
    let sigma = n / (lc * t).max(1e-9);
    let (fb, fm) = (c.n("fb_pa")?, c.n("fm_pa")?);
    let class = [MortarClass::M1, MortarClass::M2_5, MortarClass::M5, MortarClass::M10, MortarClass::M15, MortarClass::M20].into_iter().rev().find(|k| k.f_m_mpa() <= fm / 1e6 + 1e-9).unwrap_or(MortarClass::M1);
    let fvk = (f_vk0_mpa(c.annex, c.material()?, c.mortar()?, class) * 1e6 + 0.4 * sigma).min(f_vlt_mpa(c.annex, c.group(), fb / 1e6, sigma / 1e6) * 1e6);
    let fvd = fvk / c.gamma_m();
    let v_rd = fvd * t * lc;
    let mut out = vec![
        Outcome::info(QuantityKind::Length, "lc", "compressed length l_c = 3(l/2 − e)", "Druckzonenlänge l_c = 3(l/2 − e)", lc, format!("e=M/N={e:.4} m, l_c={lc:.4} m, σ_d={:.3} MPa", sigma / 1e6)).basis(&["ndp.en1996.m20.stability"]),
        Outcome::limit(QuantityKind::Force, "v-rd", "V_Ed ≤ f_vd t l_c", "V_Ed ≤ f_vd t l_c", v, v_rd.max(1e-9), format!("f_vk={:.3} MPa, f_vd={:.3} MPa, V_Rd={v_rd:.0} N", fvk / 1e6, fvd / 1e6)).basis(&["ndp.en1996.m03.fvk0", "ndp.en1996.m20.stability"]).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| { let lcx = lc; let s = n / (lcx * x).max(1e-9); let fk = (f_vk0_mpa(c.annex, c.material().unwrap_or(UnitMaterial::Clay), c.mortar().unwrap_or(MortarType::GeneralPurpose), class) * 1e6 + 0.4 * s).min(f_vlt_mpa(c.annex, c.group(), fb / 1e6, s / 1e6) * 1e6); v <= fk / c.gamma_m() * x * lcx }).unwrap_or(3.0), true),
        Outcome::limit(QuantityKind::Length, "stability", "eccentricity e < l/2 (compressed zone exists)", "Ausmitte e < l/2 (Druckzone vorhanden)", e, l / 2.0, format!("e={e:.4} m against l/2={:.4} m", l / 2.0)).basis(&["ndp.en1996.m20.stability"]).fix("l_m", QuantityKind::Length, l, ceil_to(2.0 * e, 1e-3), true),
    ];
    if let (Ok(tf), Ok(s), Ok(ht)) = (c.n("flange_t_m"), c.n("web_spacing_m"), c.n("height_total_m")) {
        let rule = flange_overhang(s, tf, ht);
        let lengths: Vec<f64> = c.v("flange_len_m").map(<[f64]>::to_vec).unwrap_or_else(|_| vec![f64::INFINITY; c.o("flanges", 2.0) as usize]);
        let overhangs: Vec<f64> = lengths.iter().map(|len| rule.min(len.max(0.0))).collect();
        let b = overhangs.iter().sum::<f64>() / (overhangs.len() as f64).max(1.0);
        let (a_web, a_f) = (l * t, overhangs.iter().sum::<f64>() * tf);
        let a = a_web + a_f;
        let yc = (a_web * l / 2.0 + a_f * (l - tf / 2.0)) / a;
        let i_ = t * l.powi(3) / 12.0 + a_web * (yc - l / 2.0).powi(2) + overhangs.iter().map(|o| o * tf.powi(3) / 12.0 + o * tf * (l - tf / 2.0 - yc).powi(2)).sum::<f64>();
        out.push(Outcome::info(QuantityKind::Length, "flange-overhang", "effective flange overhang per side", "mitwirkende Gurtbreite je Seite", b, format!("b_ef=min(s/2, 6 t_f, 0.2 H, flange length)={b:.4} m")).basis(&["ndp.en1996.m20.flange"]));
        out.push(Outcome::info(QuantityKind::Area, "flanged-area", "flanged cross-section area", "Fläche des Querschnitts mit Flanschen", a, format!("A={a:.4} m², y_c={yc:.4} m, I={i_:.5} m⁴")).basis(&["ndp.en1996.m20.flange"]));
        out.push(Outcome::info(QuantityKind::Dimensionless, "flanged-i", "second moment of area I [m⁴]", "Flächenmoment I [m⁴]", i_, format!("I={i_:.5} m⁴")).basis(&["ndp.en1996.m20.flange"]));
    }
    Ok(out)
}

/// 🧭 The in-plane distribution of one storey shear over parallel bracing walls: stiffness k_i, centre of stiffness, accidental eccentricity, torsion moment and the share V_i of every wall (EN 1996-1-1, 5.4).
#[derive(Clone, Debug)]
pub struct Bracing {
    pub stiffness: Vec<f64>,
    pub centre: f64,
    pub eccentricity: f64,
    pub torsion: f64,
    pub direct: Vec<f64>,
    pub shares: Vec<f64>,
    pub resistances: Vec<f64>,
}

/// 🧮️ Computes the [`Bracing`] of a bracing case: cantilever bending plus shear flexibility per wall, direct share `V k_i / Σk`, torsion share `M_t k_i |x_i - x_c| / Σ k_i (x_i - x_c)²`; a case marked `diaphragm = flexible` carries the vector `trib` of tributary fractions and distributes the storey shear by them without torsion (EN 1998-1 4.3.1(4)).
pub fn bracing_distribution(c: &Case) -> Result<Bracing, String> {
    let (l, t, x, vrd) = (c.v("l_m")?, c.v("t_m")?, c.v("x_m")?, c.v("vrd_n")?);
    let n = l.len();
    if t.len() != n || x.len() != n || vrd.len() != n || n == 0 {
        return Err("vectors l_m, t_m, x_m, vrd_n must have equal non-zero length".into());
    }
    let (h, e_mod, v_ed) = (c.n("h_m")?, c.n("e_pa")?, c.n("v_ed_n")?);
    let stiffness: Vec<f64> = (0..n).map(|i| 1.0 / (h.powi(3) / (3.0 * e_mod * t[i] * l[i].powi(3) / 12.0) + h / (0.4 * e_mod * l[i] * t[i]))).collect();
    let sum_k: f64 = stiffness.iter().sum();
    let centre = stiffness.iter().zip(x).map(|(k, x)| k * x).sum::<f64>() / sum_k;
    let eccentricity = c.o("e0_m", 0.0) + 0.05 * c.o("b_m", 0.0);
    let torsion = v_ed * eccentricity;
    let j: f64 = stiffness.iter().zip(x).map(|(k, x)| k * (x - centre).powi(2)).sum();
    let flexible = c.so("diaphragm", "rigid") == "flexible";
    let direct: Vec<f64> = match c.vec.get("trib").filter(|t| flexible && t.len() == n) {
        Some(trib) => trib.iter().map(|f| v_ed * f).collect(),
        None => stiffness.iter().map(|k| v_ed * k / sum_k).collect(),
    };
    let torsion = if flexible { 0.0 } else { torsion };
    let shares = (0..n).map(|i| direct[i] + if j > 0.0 { torsion * stiffness[i] * (x[i] - centre).abs() / j } else { 0.0 }).collect();
    Ok(Bracing { stiffness, centre, eccentricity, torsion, direct, shares, resistances: vrd.to_vec() })
}

pub const WALL_LENGTH_KEYS: [&str; 8] = ["l_m_0", "l_m_1", "l_m_2", "l_m_3", "l_m_4", "l_m_5", "l_m_6", "l_m_7"];

/// 🔧️ The length of wall `index` at which its share no longer exceeds its resistance when the resistance grows with the length and the stiffness redistributes the storey shear, found by bisection up to ten times the present length.
fn length_for_share(c: &Case, index: usize) -> Option<f64> {
    let (l, vrd) = (c.v("l_m").ok()?, c.v("vrd_n").ok()?);
    let ratio_at = |scale: f64| {
        let mut lengths = l.to_vec();
        let mut resistances = vrd.to_vec();
        lengths[index] *= scale;
        resistances[index] *= scale;
        let b = bracing_distribution(&c.clone().with_v("l_m", &lengths).with_v("vrd_n", &resistances)).ok()?;
        Some(b.shares[index] / b.resistances[index].max(1e-9))
    };
    if ratio_at(10.0)? > 1.0 {
        return None;
    }
    let (mut lo, mut hi) = (1.0, 10.0);
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if ratio_at(mid)? > 1.0 { lo = mid } else { hi = mid }
    }
    Some(l[index] * hi)
}

fn m21(c: &Case) -> R {
    let b = bracing_distribution(c)?;
    let (worst, ratio) = (0..b.shares.len()).map(|i| (i, b.shares[i] / b.resistances[i].max(1e-9))).fold((0, 0.0), |a, r| if r.1 > a.1 { r } else { a });
    let (xc, ecc, torsion) = (b.centre, b.eccentricity, b.torsion);
    let mut limit = Outcome::limit(QuantityKind::Force, "governing-wall", &format!("governing wall {worst}: V_i ≤ V_Rd,i"), &format!("maßgebende Wand {worst}: V_i ≤ V_Rd,i"), b.shares[worst], b.resistances[worst].max(1e-9), format!("shares {:?} N, ratio {ratio:.3}", b.shares.iter().map(|s| s.round()).collect::<Vec<_>>())).basis(&["m21.torsion"]);
    if ratio > 1.0 && worst < WALL_LENGTH_KEYS.len() {
        if let Some(required) = length_for_share(c, worst) {
            limit = limit.fix(WALL_LENGTH_KEYS[worst], QuantityKind::Length, c.v("l_m")?[worst], ceil_to(required, 1e-2), true);
        }
    }
    Ok(vec![
        Outcome::info(QuantityKind::Length, "centre", "centre of stiffness", "Steifigkeitsmittelpunkt", xc, format!("x_c={xc:.4} m, e={ecc:.4} m, M_t={torsion:.0} N·m")).basis(&["m21.torsion"]),
        limit,
    ])
}

fn m30(c: &Case) -> R {
    let (t, h, w) = (c.n("t_m")?, c.n("h_m")?, c.n("w_ed_pa")?);
    let gamma = c.gamma_m();
    let fxd1 = c.n("fxk1_pa")? / gamma;
    let sigma = c.o("sigma_d_pa", 0.0);
    let m_ed = match c.so("support", "pinned") {
        "cantilever" => w * h * h / 2.0,
        "fixed" => w * h * h / 12.0,
        _ => w * h * h / 8.0,
    };
    let z = t * t / 6.0;
    let m_rd = (fxd1 + sigma) * z;
    Ok(vec![Outcome::limit(QuantityKind::Moment, "m-rd", "M_Ed ≤ (f_xd1 + σ_d) Z", "M_Ed ≤ (f_xd1 + σ_d) Z", m_ed, m_rd.max(1e-9), format!("M_Ed={m_ed:.1} N·m/m, f_xd1={:.4} MPa, Z={z:.5} m³/m, M_Rd={m_rd:.1} N·m/m", fxd1 / 1e6)).basis(&["ndp.en1996.m04.fxk"]).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| m_ed <= (fxd1 + sigma) * x * x / 6.0).unwrap_or(3.0), true)])
}

fn m31(c: &Case) -> R {
    let (t, h, l, w) = (c.n("t_m")?, c.n("h_m")?, c.n("l_m")?, c.n("w_ed_pa")?);
    let gamma = c.gamma_m();
    let sigma = c.o("sigma_d_pa", 0.0);
    let z = t * t / 6.0;
    let (fxd1, fxd2) = (c.n("fxk1_pa")? / gamma + sigma, c.n("fxk2_pa")? / gamma);
    let (m1, m2) = (fxd1 * z, fxd2 * z);
    let edges = Edges::parse(c.so("edges", "SSSS")).ok_or("edges must be four letters of F, S, X (top, bottom, left, right)")?;
    let w_rd = yield_line_pressure(edges, l, h, m1, m2);
    let mu = fxd1 / fxd2;
    let fixed = [edges_code_has_fixed(c.so("edges", "SSSS"))];
    let mut out = Vec::new();
    if fixed[0] {
        out.push(Outcome::warn(QuantityKind::Dimensionless, "fixed-hogging", "fixed edges assume hogging = sagging capacity", "eingespannte Ränder setzen Einspann- gleich Feldmoment an", 1.0, 0.5, "valid only with sufficient pre-compression; bare unreinforced masonry has no hogging reserve".into()).steps_de("nur bei ausreichender Vorpressung gültig; unbewehrtes Mauerwerk hat keine Reserve im Stützbereich").basis(&["ndp.en1996.m31.fixed-hogging"]));
    }
    out.extend(annex_e_outcomes(c, edges, mu, (t, h, l, w), m1, fxd1));
    out.extend(vec![
        Outcome::info(QuantityKind::Dimensionless, "mu", "orthogonal ratio μ = f_xd1/f_xd2", "Orthogonalitätsverhältnis μ", mu, format!("μ={mu:.4}")).basis(&["ndp.en1996.m31.yield-line"]),
        Outcome::info(QuantityKind::Dimensionless, "alpha1", "α₁ = m₁/(w_Rd l²)", "α₁ = m₁/(w_Rd l²)", m1 / (w_rd * l * l), format!("m₁={m1:.1}, m₂={m2:.1} N·m/m, w_Rd={w_rd:.1} Pa")).basis(&["ndp.en1996.m31.yield-line"]),
        Outcome::limit(QuantityKind::Stress, "w-rd", "w_Ed ≤ w_Rd (yield line)", "w_Ed ≤ w_Rd (Bruchlinien)", w, w_rd.max(1e-9), format!("w_Rd={w_rd:.1} Pa")).basis(&["ndp.en1996.m31.yield-line"]).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| w <= yield_line_pressure(edges, l, h, fxd1 * x * x / 6.0, fxd2 * x * x / 6.0)).unwrap_or(3.0), true),
    ]);
    Ok(out)
}

/// 📐️ Annex E part of M-31: the coefficient class of the edge condition, α₁ from the entered print value, the keyed table cell or the unverified yield-line stand-in, and M_Ed1 = α₁ w l² against M_Rd1 whenever α₁ was entered or keyed.
fn annex_e_outcomes(c: &Case, edges: Edges, mu: f64, (t, h, l, w): (f64, f64, f64, f64), m1: f64, fxd1: f64) -> Vec<Outcome> {
    let class = annex_e::class_code(edges);
    let basis: &[&'static str] = &["ndp.en1996.m31.yield-line", annex_e::basis_id(&class).unwrap_or("ndp.en1996.m31.yield-line")];
    let (alpha, source, source_de, entered) = match (c.opt("alpha1"), annex_e::alpha1(edges, mu, h / l)) {
        (Some(a), _) => (a, "entered from the print", "aus dem Druck eingegeben", true),
        (None, Alpha::Keyed(a)) => (a, "keyed Annex E cell", "übertragene Zelle aus Anhang E", true),
        (None, Alpha::Unkeyed { .. }) => (annex_e::standin_alpha1(edges, mu, h / l), "yield-line stand-in, Annex E cell not keyed (unverified)", "Bruchlinien-Ersatzwert, Zelle aus Anhang E nicht übertragen (ungeprüft)", false),
    };
    let mut out = vec![Outcome::info(QuantityKind::Dimensionless, "annex-e", "Annex E coefficient α₁", "Anhang-E-Beiwert α₁", alpha, format!("class {class}, h/l={:.3}, μ={mu:.3}: α₁={alpha:.5} ({source})", h / l)).steps_de(format!("Klasse {class}, h/l={:.3}, μ={mu:.3}: α₁={alpha:.5} ({source_de})", h / l)).basis(basis)];
    if entered {
        let m_ed = annex_e::design_moment(alpha, w, l);
        out.push(Outcome::limit(QuantityKind::Moment, "annex-e-m-ed", "M_Ed1 = α₁ w_Ed l² ≤ M_Rd1", "M_Ed1 = α₁ w_Ed l² ≤ M_Rd1", m_ed, m1.max(1e-9), format!("M_Ed1={m_ed:.1} N·m/m, M_Rd1={m1:.1} N·m/m")).basis(basis).fix("t_m", QuantityKind::Length, t, ceil_to((6.0 * m_ed / fxd1.max(1e-9)).sqrt(), 1e-3), true));
    }
    out
}

fn edges_code_has_fixed(code: &str) -> bool {
    code.chars().any(|c| c.eq_ignore_ascii_case(&'X'))
}

fn m32(c: &Case) -> R {
    let (t, la, fd, w) = (c.n("t_m")?, c.n("la_m")?, c.f_d_pa()?, c.n("w_ed_pa")?);
    let q = arching_pressure(fd, t, la);
    Ok(vec![
        Outcome::limit(QuantityKind::Stress, "q-lat", "w_Ed ≤ q_lat,d = f_d (t/l_a)²", "w_Ed ≤ q_lat,d = f_d (t/l_a)²", w, q.max(1e-9), format!("q_lat,d={q:.1} Pa")).basis(&["ndp.en1996.m32.arching"]).fix("t_m", QuantityKind::Length, t, ceil_to(la * (w / fd).sqrt(), 1e-3), true),
        Outcome::limit(QuantityKind::Dimensionless, "span-ratio", "l_a/t ≤ 25", "l_a/t ≤ 25", la / t, 25.0, format!("l_a/t={:.2}", la / t)).basis(&["ndp.en1996.m32.arching"]).fix("t_m", QuantityKind::Length, t, ceil_to(la / 25.0, 1e-3), true),
    ])
}

fn m33(c: &Case) -> R {
    let we = c.n("qp_pa")? * c.n("cpe")?;
    let mut out = vec![Outcome::info(QuantityKind::Stress, "we", "wind pressure w_e = q_p c_pe [Pa]", "Winddruck w_e = q_p c_pe [Pa]", we, format!("w_e={we:.1} Pa, ψ₀={}", semio_s_plugin_norm_compliance::action_factors::psi_row_of_key(c.annex, "wind").map_or(1.0, |p| p.psi_0)))];
    if let (Ok(k0), Ok(g), Ok(he)) = (c.n("k0"), c.n("gamma_soil_n_m3"), c.n("he_m")) {
        let e0 = k0 * g * he;
        out.push(Outcome::info(QuantityKind::Stress, "e0", "earth pressure at the base e_0 = K_0 γ h_e [Pa]", "Erddruck am Fuß e_0 = K_0 γ h_e [Pa]", e0, format!("e_0={e0:.1} Pa")));
        out.push(Outcome::info(QuantityKind::Force, "earth-resultant", "earth pressure resultant per metre", "Erddruckresultierende je Meter", 0.5 * e0 * he, format!("H={:.1} N/m", 0.5 * e0 * he)));
    }
    Ok(out)
}

fn m34(c: &Case) -> R {
    let (h, t, he, rho_e) = (c.n("h_m")?, c.n("t_m")?, c.n("he_m")?, c.n("rho_e_n_m3")?);
    let fd = c.f_d_pa()?;
    let n_req = rho_e * h * he * he / (20.0 * t);
    let (n_min, n_max) = (c.n("n_min_n")?, c.n("n_max_n")?);
    let basis = ["m34.basement"];
    let mut out = vec![
        Outcome::limit(QuantityKind::Length, "height", "clear height h ≤ 2.6 m", "lichte Höhe h ≤ 2,6 m", h, 2.6, format!("h={h:.2} m")).basis(&basis).fix("h_m", QuantityKind::Length, h, 2.6, false),
        Outcome::limit(QuantityKind::Length, "thickness", "wall thickness t ≥ 0.24 m", "Wanddicke t ≥ 0,24 m", 0.24, t, format!("t={t:.3} m")).basis(&basis).fix("t_m", QuantityKind::Length, t, 0.24, true),
        Outcome::limit(QuantityKind::Stress, "surcharge", "surface load q_k ≤ 5 kN/m²", "Verkehrslast q_k ≤ 5 kN/m²", c.o("qk_pa", 0.0), 5_000.0, "surface load next to the wall".into()).steps_de("Flächenlast neben der Wand").basis(&basis).fix("qk_pa", QuantityKind::Stress, c.o("qk_pa", 0.0), 5_000.0, false),
        Outcome::limit(QuantityKind::Force, "n-min", "N_min ≥ ρ_e h h_e²/(20 t)", "N_min ≥ ρ_e h h_e²/(20 t)", n_req, n_min.max(1e-9), format!("N_min,req={n_req:.0} N/m")).basis(&basis).fix("n_min_n", QuantityKind::Force, n_min, ceil_to(n_req, 1.0), true),
        Outcome::limit(QuantityKind::Force, "n-max", "N_max ≤ 0.45 t f_d", "N_max ≤ 0,45 t f_d", n_max, (0.45 * t * fd).max(1e-9), format!("N_Rd={:.0} N/m", 0.45 * t * fd)).basis(&basis).fix("t_m", QuantityKind::Length, t, ceil_to(n_max / (0.45 * fd), 1e-3), true),
    ];
    out.push(Outcome::limit(QuantityKind::Dimensionless, "hydrostatic", "no hydrostatic pressure", "kein hydrostatischer Druck", c.o("hydrostatic", 0.0), 0.5, "1 when groundwater acts on the wall".into()).steps_de("1, wenn Grundwasser auf die Wand wirkt").basis(&basis).fix("hydrostatic", QuantityKind::Dimensionless, c.o("hydrostatic", 0.0), 0.0, false));
    Ok(out)
}
