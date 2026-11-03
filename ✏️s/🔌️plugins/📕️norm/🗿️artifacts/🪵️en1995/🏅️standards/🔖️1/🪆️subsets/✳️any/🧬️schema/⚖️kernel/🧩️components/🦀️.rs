//! 🧩️ Components and assemblies of EN 1995-1-1 chapter 9 and Annexes B and C as catalogue entries T-90 … T-98: γ-method, glued thin-web and thin-flange beams, spaced columns, diaphragms, trusses, bracing, CLT and the timber behaviour factor. Units: N, mm, N/mm². See [`super`].

use super::*;
use std::f64::consts::PI;

/// 🧱 One part of a mechanically jointed beam in the γ-method (Annex B): area, inertia, modulus, depth and fastener spacing to the web.
#[derive(Clone, Copy, Debug)]
pub struct Part {
    pub a: f64,
    pub i: f64,
    pub e: f64,
    pub h: f64,
    pub s: f64,
}

/// 🧮 Result of the γ-method: γᵢ, distances aᵢ, (EI)_ef and the rigid (EI) with γ = 1.
#[derive(Clone, Copy, Debug)]
pub struct GammaOut {
    pub gamma: [f64; 3],
    pub a: [f64; 3],
    pub ei_ef: f64,
    pub ei_rigid: f64,
}

fn arrange(parts: &[Part]) -> [Part; 3] {
    let zero = Part { a: 0.0, i: 0.0, e: 0.0, h: 0.0, s: 1.0 };
    [parts[0], parts[1], parts.get(2).copied().unwrap_or(zero)]
}

fn gamma_with(p: &[Part; 3], g: [f64; 3]) -> ([f64; 3], f64) {
    let sum: f64 = (0..3).map(|i| g[i] * p[i].e * p[i].a).sum();
    let a2 = (g[0] * p[0].e * p[0].a * (p[0].h + p[1].h) - g[2] * p[2].e * p[2].a * (p[1].h + p[2].h)) / (2.0 * sum);
    let a = [(p[0].h + p[1].h) / 2.0 - a2, a2, (p[1].h + p[2].h) / 2.0 + a2];
    let ei = (0..3).map(|i| p[i].e * p[i].i + g[i] * p[i].e * p[i].a * a[i] * a[i]).sum();
    (a, ei)
}

/// 🧮 γ-method of EN 1995-1-1 Annex B for two or three parts (part 2 is the web, γ₂ = 1) under a sinusoidal load with slip modulus `k` per fastener row.
pub fn gamma_beam(parts: &[Part], k: f64, l: f64) -> GammaOut {
    let p = arrange(parts);
    let gi = |q: &Part| if q.a > 0.0 { 1.0 / (1.0 + PI * PI * q.e * q.a * q.s / (k * l * l)) } else { 0.0 };
    let gamma = [gi(&p[0]), 1.0, gi(&p[2])];
    let (a, ei_ef) = gamma_with(&p, gamma);
    let (_, ei_rigid) = gamma_with(&p, [1.0, 1.0, if p[2].a > 0.0 { 1.0 } else { 0.0 }]);
    GammaOut { gamma, a, ei_ef, ei_rigid }
}

/// 🏗️ Fibre stresses of one part of a jointed beam in N/mm², tension positive: at its top and its bottom face.
#[derive(Clone, Copy, Debug, Default)]
pub struct FibreStress {
    pub top: f64,
    pub bottom: f64,
}

/// 🏗️ Stresses and slip of a mechanically jointed beam under a sinusoidal load with the design moment `m` and shear `v` at the support (EN 1995-1-1 Annex B): part 1 on top, part 3 at the bottom, fibre stresses of every part, the fastener force of the interfaces of part 1 and 3 and the slip `F/K` they produce.
#[derive(Clone, Copy, Debug)]
pub struct JointProfile {
    pub gamma: [f64; 3],
    pub ei_ef: f64,
    pub fibres: [FibreStress; 3],
    pub fastener: [f64; 3],
    pub slip: [f64; 3],
}

impl JointProfile {
    /// 🏗️ Slip of the interface of part `i` (0 or 2) at `x` along the span `l`: V(x) follows cos(πx/l), so u(x) = u₀·|cos(πx/l)|.
    pub fn slip_at(&self, i: usize, x: f64, l: f64) -> f64 {
        self.slip[i] * (PI * x / l).cos().abs()
    }
}

/// 🏗️ The [`JointProfile`] of `parts` with slip modulus `k` per fastener row; with `rigid` the interfaces carry no slip (γ = 1) for the comparison. Part heights are measured on the neutral axis as in [`gamma_beam`]: the centroids lie at +a₁, −a₂, −a₃ from the neutral axis, so σ = −M·Eᵢ(γᵢ zᵢ ± hᵢ/2)/(EI)_ef.
pub fn joint_profile(parts: &[Part], k: f64, l: f64, m: f64, v: f64, rigid: bool) -> JointProfile {
    let p = arrange(parts);
    let g = gamma_beam(parts, k, l);
    let (gamma, a, ei) = if rigid {
        let unit = [1.0, 1.0, if p[2].a > 0.0 { 1.0 } else { 0.0 }];
        let (a, ei) = gamma_with(&p, unit);
        (unit, a, ei)
    } else {
        (g.gamma, g.a, g.ei_ef)
    };
    let z = [a[0], -a[1], -a[2]];
    let fibre = |i: usize| FibreStress { top: -m * p[i].e * (gamma[i] * z[i] + p[i].h / 2.0) / ei, bottom: -m * p[i].e * (gamma[i] * z[i] - p[i].h / 2.0) / ei };
    let force = |i: usize| gamma[i] * p[i].e * p[i].a * a[i] * p[i].s * v / ei;
    let fastener = [force(0), 0.0, force(2)];
    let slip = if rigid || k <= 0.0 { [0.0; 3] } else { [fastener[0].abs() / k, 0.0, fastener[2].abs() / k] };
    JointProfile { gamma, ei_ef: ei, fibres: [fibre(0), fibre(1), fibre(2)], fastener, slip }
}

/// 🏗️ The ultimate slip modulus K_u = 2/3 K_ser of EN 1995-1-1 2.2.2 (2) for the verifications at the ultimate limit state, K_ser being the modulus of the serviceability verifications (the same rule as the output `k_u` of T-83).
pub fn ultimate_slip_modulus(k_ser: f64) -> f64 {
    2.0 / 3.0 * k_ser
}

/// 🏗️ True when a part of modulus `e` in N/mm² is stiffer than every strength class of the tables, i.e. a mineral (concrete) flange of a timber-concrete composite beam and no timber part; used only to pick the material of the 3D picture.
pub fn is_mineral_part(e: f64) -> bool {
    strength_class_names().iter().filter_map(|n| strength_row(n)).all(|r| e > r.e_0_mean)
}

/// 🧱 Parts from the keys a1,i1,e1,h1,s1 / a2,i2,e2,h2 / a3,i3,e3,h3,s3 (part 3 optional).
pub fn parts_from(c: &Case) -> Result<Vec<Part>, String> {
    let get = |n: u8, need_s: bool| -> Result<Part, String> {
        Ok(Part { a: c.n(&format!("a{n}"))?, i: c.n(&format!("i{n}"))?, e: c.n(&format!("e{n}"))?, h: c.n(&format!("h{n}"))?, s: if need_s { c.n(&format!("s{n}"))? } else { 1.0 } })
    };
    let mut v = vec![get(1, true)?, get(2, false)?];
    if c.opt("a3").is_some() {
        v.push(get(3, true)?);
    }
    Ok(v)
}

fn t90(c: &Case) -> R {
    let parts = parts_from(c)?;
    let (k, l, mom, shear) = (c.n("k")?, c.n("l")?, c.n("m_d")?, c.o("v_d", 0.0));
    let g = gamma_beam(&parts, k, l);
    let p = arrange(&parts);
    let (ei, e2) = (g.ei_ef, p[1].e);
    let sig = |i: usize| g.gamma[i] * p[i].e * g.a[i] * mom / ei;
    let sigm = |i: usize| 0.5 * p[i].e * p[i].h * mom / ei;
    let ratio = |i: usize, key: &str| (sig(i).abs() / c.o(&format!("fd_a{key}"), f64::INFINITY)) + sigm(i).abs() / c.o(&format!("fd_m{key}"), f64::INFINITY);
    let mut out = vec![Outcome::info("ei_ef", "Effective stiffness (EI)_ef", "Wirksame Biegesteifigkeit (EI)_ef", QuantityKind::Dimensionless, ei, format!("γ₁={} γ₃={} a₁={} a₂={} a₃={} (EI)_ef={} N·mm²", f(g.gamma[0]), f(g.gamma[2]), f(g.a[0]), f(g.a[1]), f(g.a[2]), f(ei)))];
    for (i, key) in [(0usize, "1"), (1, "2"), (2, "3")] {
        if p[i].a > 0.0 && c.opt(&format!("fd_a{key}")).is_some() {
            out.push(Outcome::ratio(&format!("fibre_{key}"), &format!("Part {key}: axial plus bending (B.3)"), &format!("Teil {key}: Normalkraft plus Biegung (B.3)"), ratio(i, key), 1.0, format!("σ_{key}={} σ_m,{key}={}", f(sig(i)), f(sigm(i)))));
        }
    }
    if let Some(fv) = c.opt("fv_d_2") {
        let tau = 0.5 * e2 * (p[1].h / 2.0 + g.a[1]).powi(2) * shear / ei;
        out.push(Outcome::stress("web_shear", "Web shear τ₂,max (Annex B)", "Stegschub τ₂,max", tau, fv, format!("τ₂,max = {} N/mm²", f(tau))));
    }
    if shear > 0.0 {
        let (u, ku) = (joint_profile(&parts, k, l, mom, shear, false), ultimate_slip_modulus(k));
        let uls = gamma_beam(&parts, ku, l);
        out.push(Outcome::info("ei_ef_u", "Effective stiffness (EI)_ef with K_u = 2/3 K_ser", "Wirksame Biegesteifigkeit (EI)_ef mit K_u = 2/3 K_ser", QuantityKind::Dimensionless, uls.ei_ef, format!("γ₁={} (EI)_ef,u={} N·mm² for the verifications at the ultimate limit state", f(uls.gamma[0]), f(uls.ei_ef))));
        out.push(Outcome::info("slip_support", "Slip of the fasteners at the support u = F/K", "Schlupf der Verbindungsmittel am Auflager u = F/K", QuantityKind::Length, u.slip[0].max(u.slip[2]) / 1e3, format!("u = {} mm at the support, zero at midspan", f(u.slip[0].max(u.slip[2])))));
    }
    out.push(Outcome::info("slip_efficiency", "Efficiency (EI)_ef / (EI) of the rigid composite", "Wirkungsgrad (EI)_ef / (EI) des starren Verbunds", QuantityKind::Dimensionless, g.ei_ef / g.ei_rigid, format!("(EI)_ef/(EI)_rigid = {}", f(g.ei_ef / g.ei_rigid))));
    if let Some(q) = c.opt("q_d") {
        let w = 5.0 * q * l.powi(4) / (384.0 * ei);
        out.push(Outcome::length("deflection", "Deflection with slip, 5 q l⁴/(384 (EI)_ef)", "Durchbiegung mit Nachgiebigkeit, 5 q l⁴/(384 (EI)_ef)", w, l / c.o("limit_divisor", 300.0), format!("w = {} mm, rigid {} mm", f(w), f(5.0 * q * l.powi(4) / (384.0 * g.ei_rigid)))));
    }
    for (i, key) in [(0usize, "1"), (2, "3")] {
        if let Some(rd) = c.opt(&format!("r_d{key}")) {
            let fi = g.gamma[i] * p[i].e * p[i].a * g.a[i] * p[i].s * shear / ei;
            out.push(Outcome::force(&format!("fastener_{key}"), &format!("Fastener load part {key}"), &format!("Verbindungsmittelkraft Teil {key}"), fi, rd, format!("F_{key} = {} N", f(fi))));
        }
    }
    Ok(out)
}

struct Thin {
    ei: f64,
    z: f64,
}

fn thin_i_section(bf: f64, hf: f64, bw: f64, hw: f64, n_web: f64) -> Thin {
    let z = (hw + hf) / 2.0;
    Thin { ei: 2.0 * (bf * hf.powi(3) / 12.0 + bf * hf * z * z) + n_web * bw * hw.powi(3) / 12.0, z }
}

fn t91(c: &Case) -> R {
    let m = Mat::load(c)?;
    let (bf, hf, bw, hw) = (c.n("b_f")?, c.n("h_f")?, c.n("b_w")?, c.n("h_w")?);
    let nw = c.o("e_w", 1.0) / c.o("e_f", 1.0);
    let t = thin_i_section(bf, hf, bw, hw, nw);
    let (mom, v) = (c.n("m_d")?, c.o("v_d", 0.0));
    let s_f = bf * hf * t.z;
    let s_max = s_f + nw * bw * hw * hw / 8.0;
    let mean = mom * t.z / t.ei;
    let ext = mom * (hw / 2.0 + hf) / t.ei;
    let fwv = c.o("f_v_web_d", m.fd(m.row.f_v_k));
    let mut out = vec![
        Outcome::stress("flange_mean_c", "Flange mean stress (9.1.1)", "Mittlere Gurtspannung (9.1.1)", mean, m.fd(m.row.f_c_0_k).min(m.fd(m.row.f_t_0_k)), format!("σ_f,mean = {} N/mm²", f(mean))),
        Outcome::stress("flange_max", "Flange extreme fibre stress (9.1.1)", "Gurtrandspannung (9.1.1)", ext, m.kh_bending(c, hf) * m.fd(m.row.f_m_k), format!("σ_f,max = {} N/mm²", f(ext))),
        Outcome::stress("web_shear", "Web shear stress (9.1.1)", "Stegschubspannung (9.1.1)", v * s_max / (t.ei * bw), fwv, format!("S_max = {} mm³, (EI) = {} N·mm²", f(s_max), f(t.ei))),
        Outcome::ratio("web_slenderness", "Web slenderness h_w ≤ 70 b_w", "Stegschlankheit h_w ≤ 70 b_w", hw, 70.0 * bw, "web buckling limit".into()),
    ];
    if let Some(fg) = c.opt("f_glue_d") {
        out.push(Outcome::stress("glue_line", "Glue line shear", "Klebfugenschub", v * s_f / (t.ei * c.o("b_glue", bw)), fg, "τ_gl = V S_f/(I b_glue)".into()));
    }
    out.push(Outcome::info("ei", "Section stiffness (EI)", "Querschnittssteifigkeit (EI)", QuantityKind::Dimensionless, t.ei, format!("{} N·mm²", f(t.ei))));
    Ok(out)
}

/// 🧮 Effective flange outstand in mm on each side: min(0.1·ℓ, k·h_f, outstand) with k = 20 (compression) or 25 (tension) per EN 1995-1-1 9.1.2 Table 9.1.
pub fn flange_outstand(span: f64, hf: f64, outstand: f64, compression: bool) -> f64 {
    (0.1 * span).min(if compression { 20.0 } else { 25.0 } * hf).min(outstand)
}

fn t92(c: &Case) -> R {
    let m = Mat::load(c)?;
    let (span, hf, bw, hw, out_w) = (c.n("l")?, c.n("h_f")?, c.n("b_w")?, c.n("h_w")?, c.n("outstand")?);
    let n = c.o("e_f", 1.0) / c.o("e_w", 1.0);
    let mom = c.n("m_d")?;
    let bc = bw + 2.0 * flange_outstand(span, hf, out_w, true);
    let bt = bw + 2.0 * flange_outstand(span, hf, out_w, false);
    let comp_top = c.so("flange_side", "compression") == "compression";
    let bef = if comp_top { bc } else { bt };
    let af = n * bef * hf;
    let aw = bw * hw;
    let y_f = hw + hf / 2.0;
    let na = (af * y_f + aw * hw / 2.0) / (af + aw);
    let i = n * bef * hf.powi(3) / 12.0 + af * (y_f - na).powi(2) + bw * hw.powi(3) / 12.0 + aw * (hw / 2.0 - na).powi(2);
    let sf = n * mom * (y_f - na) / i;
    let sw = mom * na / i;
    let ffl = c.n("f_flange_d")?;
    Ok(vec![
        Outcome::length("b_ef", "Effective flange width b_ef", "Mitwirkende Gurtbreite b_ef", bef, bef.max(1.0), format!("b_c,ef={} mm b_t,ef={} mm", f(bc), f(bt))),
        Outcome::stress("flange", "Panel flange stress (9.1.2)", "Plattenspannung im Gurt (9.1.2)", sf.abs(), ffl, format!("σ_f = {} N/mm², (EI)={}", f(sf), f(i))),
        Outcome::stress("web_edge", "Web extreme fibre stress (9.1.2)", "Stegrandspannung (9.1.2)", sw.abs(), m.fd(m.row.f_t_0_k).max(m.fd(m.row.f_c_0_k)), format!("σ_w = {} N/mm²", f(sw))),
    ])
}

fn t93(c: &Case) -> R {
    let m = Mat::load(c)?;
    let (lef, i_tot, l1, i1, n) = (c.n("lef")?, c.n("i_tot")?, c.n("l1")?, c.n("i1")?, c.n("n_parts")?);
    let eta = c.opt("eta").unwrap_or_else(|| ndp_common(match c.so("joint", "nailed") {
        "glued" => "ndp.en1995.annex-c.eta.glued",
        "bolted" => "ndp.en1995.annex-c.eta.bolted",
        "connectors" => "ndp.en1995.annex-c.eta.connectors",
        _ => "ndp.en1995.annex-c.eta.nailed",
    }));
    let lam = lef / i_tot;
    let lam1 = l1 / i1;
    let lef_eff = (lam * lam + eta * n / 2.0 * lam1 * lam1).sqrt();
    let rel = lef_eff / PI * (m.row.f_c_0_k / m.row.e_0_05).sqrt();
    let kc = kc_factor(rel, if m.row.product == TimberProduct::Solid { 0.2 } else { 0.1 });
    let sc = c.n("n_c")? / c.n("area")?;
    Ok(vec![
        Outcome::info("lambda_ef", "Effective slenderness λ_ef", "Wirksamer Schlankheitsgrad λ_ef", QuantityKind::Dimensionless, lef_eff, format!("λ_ef = √(λ² + η n/2 λ₁²) = {}, rigid λ = {}", f(lef_eff), f(lam))),
        Outcome::info("lambda_rel", "Relative slenderness λ_rel,c", "Bezogener Schlankheitsgrad λ_rel,c", QuantityKind::Dimensionless, rel, format!("λ_rel,c = {}", f(rel))),
        Outcome::info("k_c", "Buckling factor k_c", "Knickbeiwert k_c", QuantityKind::Dimensionless, kc, format!("k_c = {}", f(kc))),
        Outcome::stress("spaced_column", "Mechanically jointed column (Annex C)", "Mechanisch verbundener Druckstab (Anhang C)", sc, kc * m.fd(m.row.f_c_0_k), format!("η={} λ_ef={} λ_rel={} k_c={}", f(eta), f(lef_eff), f(rel), f(kc))),
    ])
}

/// 🧮 Racking resistance per wall panel by Method A: F_i,v,Rd = F_f,Rd·b_i·c_i/s with c_i = 1 for b_i ≥ b₀ = h/2 else b_i/b₀ (EN 1995-1-1 9.2.4.2).
pub fn wall_panel_racking(ff_rd: f64, b_i: f64, h: f64, s: f64) -> f64 {
    let b0 = h / 2.0;
    ff_rd * b_i * (b_i / b0).min(1.0) / s
}

/// 🧮 Method factor of the wall racking resistance: 1 for Method A, k_d·k_q·k_n·k_op of the registry (or the case) for Method B (EN 1995-1-1 9.2.4.3).
pub fn racking_factor(c: &Case) -> f64 {
    if c.so("method", "A") == "B" { c.opt("k_d").unwrap_or_else(|| ndp_common("ndp.en1995.diaphragm.method-b-k-d")) * c.opt("k_q").unwrap_or_else(|| ndp_common("ndp.en1995.diaphragm.method-b-k-q")) * c.opt("k_n").unwrap_or_else(|| ndp_common("ndp.en1995.diaphragm.method-b-k-n")) * c.opt("k_op").unwrap_or_else(|| ndp_common("ndp.en1995.diaphragm.method-b-k-op")) } else { 1.0 }
}

fn t94(c: &Case) -> R {
    let mut out = Vec::new();
    if let (Some(ff), Some(bi), Some(h), Some(s)) = (c.opt("f_f_rd"), c.opt("b_i"), c.opt("h"), c.opt("s")) {
        let kk = racking_factor(c);
        let fi = kk * c.o("n_panels", 1.0) * wall_panel_racking(ff, bi, h, s);
        if let Some(fed) = c.opt("f_v_ed") {
            out.push(Outcome::force("racking", "Wall racking resistance (9.2.4)", "Wandscheibentragfähigkeit (9.2.4)", fed, fi, format!("Method {}: ΣF_i,v,Rd = {} N", c.so("method", "A"), f(fi))));
            if let Some(hd) = c.opt("r_hold_down") {
                let ft = fed * h / bi - c.o("q_v", 0.0) * bi / 2.0;
                out.push(Outcome::force("hold_down", "Hold-down tension", "Zuganker", ft.max(0.0), hd, format!("F_t,Ed = F h/b − q b/2 = {} N", f(ft))));
            }
        }
    }
    if let (Some(fd), Some(l), Some(q_rd)) = (c.opt("f_d"), c.opt("l_dia"), c.opt("q_v_rd")) {
        let q = fd / l;
        out.push(Outcome::force("diaphragm_shear", "Diaphragm shear flow (9.2.3)", "Schubfluss der Scheibe (9.2.3)", q, q_rd, format!("q_v,Ed = {} N/mm", f(q))));
        if let (Some(mom), Some(b), Some(nrd)) = (c.opt("m_dia"), c.opt("b_dia"), c.opt("n_chord_rd")) {
            out.push(Outcome::force("chord", "Chord force", "Gurtkraft", mom / b, nrd, "N = M/b".into()));
        }
    }
    if out.is_empty() {
        return Err("missing input f_f_rd, b_i, h, s or f_d, l_dia, q_v_rd".into());
    }
    Ok(out)
}

/// 🧮 Stabilising load of a series of compression members q_d = k_l·N_d/(k_f,3·l) in N/m with k_l = min(1, √(15/l)), l in m, `n` the number of members to be braced and `nd` the design compression force N_d of each member (EN 1995-1-1 9.2.5.3(1), Expressions 9.37 and 9.38); k_f,3 (30 recommended) and the constant 15 are registry rows `ndp.en1995.bracing.*`, unverified for the DE annex. The single-member force F_d = N_d/k_f,1 (50, solid) or N_d/k_f,2 (80, glulam and LVL) is 9.2.5.2(3), Expression 9.35.
pub fn bracing_qd(n: f64, nd: f64, l_m: f64) -> f64 {
    (ndp_common("ndp.en1995.bracing.kl-reference-length") / l_m).sqrt().min(1.0) * n * nd / (ndp_common("ndp.en1995.bracing.stabilising-divisor") * l_m)
}

fn t95(c: &Case) -> R {
    let (n, nd, l) = (c.n("n_braced")?, c.n("n_d")?, c.n("l_m")?);
    let qd = bracing_qd(n, nd, l);
    let fd = nd / ndp_common(if c.so("product", "solid") == "solid" { "ndp.en1995.bracing.force-divisor-solid" } else { "ndp.en1995.bracing.force-divisor-engineered" });
    let mut out = vec![
        Outcome::info("q_d", "Stabilising load q_d", "Stabilisierungslast q_d", QuantityKind::Dimensionless, qd, format!("q_d = k_l n N_d/(k_f,3 l) = {} N/m", f(qd))),
        Outcome::info("f_d", "Stabilising force F_d", "Stabilisierungskraft F_d", QuantityKind::Force, fd, format!("F_d = {} N", f(fd))),
    ];
    if let Some(r) = c.opt("q_rd") {
        out.push(Outcome::ratio("bracing_capacity", "Bracing system capacity", "Tragfähigkeit des Aussteifungssystems", qd, r, "q_d ≤ q_Rd (N/m)".into()));
    }
    Ok(out)
}

/// 🪢 T-96 bracing force ΣF_d = q_d·l and the horizontal deflection of the bracing system F/k_br against l/500 (EN 1995-1-1 9.2.5.3(2)); `k_br` is the lateral stiffness of the whole bracing system for the resultant ΣF_d, the clause names no spring layout.
fn t96(c: &Case) -> R {
    let (n, nd, l) = (c.n("n_braced")?, c.n("n_d")?, c.n("l_m")?);
    let fd = bracing_qd(n, nd, l) * l;
    let mut out = vec![Outcome::force("bracing_force", "Bracing force", "Aussteifungskraft", fd, c.n("f_rd")?, format!("ΣF_d = q_d·l = {} N", f(fd)))];
    if let Some(k) = c.opt("k_br") {
        out.push(Outcome::length("bracing_deflection", "Bracing deflection ≤ L/500", "Aussteifungsverformung ≤ L/500", fd / k, l * 1e3 / ndp_common("ndp.en1995.bracing.deflection-limit-divisor"), format!("δ = F/k = {} mm", f(fd / k))));
    }
    Ok(out)
}

/// 🧮 γ-method for symmetric CLT per metre width: returns (EI_ef, γ per layer, centre distance per layer) in N·mm² per mm... layers `(thickness, longitudinal)`, E and rolling shear modulus in N/mm², span in mm.
pub fn clt_gamma(layers: &[(f64, bool)], e: f64, g_r: f64, span: f64) -> (f64, Vec<f64>, Vec<f64>) {
    let n = layers.len();
    let total: f64 = layers.iter().map(|l| l.0).sum();
    let mut z = Vec::new();
    let mut acc = 0.0;
    for l in layers {
        z.push(acc + l.0 / 2.0 - total / 2.0);
        acc += l.0;
    }
    let mid = (n - 1) as f64 / 2.0;
    let mut gam = vec![0.0; n];
    for i in 0..n {
        if !layers[i].1 {
            continue;
        }
        let (lo, hi) = ((i as f64).min(mid) as usize, (i as f64).max(mid) as usize);
        let mut d = 0.0;
        for k in lo..=hi {
            if k != i && !layers[k].1 {
                d += if k as f64 == mid { layers[k].0 / 2.0 } else { layers[k].0 };
            }
        }
        gam[i] = if d == 0.0 { 1.0 } else { 1.0 / (1.0 + PI * PI * e * layers[i].0 * d / (g_r * span * span)) };
    }
    let ei = (0..n).filter(|&i| layers[i].1).map(|i| e * layers[i].0.powi(3) / 12.0 + gam[i] * e * layers[i].0 * z[i] * z[i]).sum();
    (ei, gam, z)
}

fn t97(c: &Case) -> R {
    let n = c.n("n_layers")? as usize;
    let mut layers = Vec::new();
    for k in 1..=n {
        layers.push((c.n(&format!("t{k}"))?, c.n(&format!("o{k}"))? > 0.5));
    }
    let row = strength_row(c.so("class", "CLT100")).ok_or("unknown lamella class")?;
    let (kmod, gm) = (c.o("kmod", 0.8), c.o("gamma_m", gamma_m_for(c.annex, "clt", false).unwrap_or(f64::NAN)));
    let g_r = c.o("g_r", ndp_common("ndp.en1995.clt.rolling-shear-modulus"));
    let span = c.n("l")?;
    let (ei, gam, z) = clt_gamma(&layers, row.e_0_mean, g_r, span);
    let (mom, v) = (c.o("m_d", 0.0), c.o("v_d", 0.0));
    let total: f64 = layers.iter().map(|l| l.0).sum();
    let mut smax: f64 = 0.0;
    for i in 0..n {
        if layers[i].1 {
            smax = smax.max(row.e_0_mean * (gam[i] * z[i].abs() + layers[i].0 / 2.0) * mom / ei);
        }
    }
    let mut static_moment = 0.0;
    let mut tau_r: f64 = 0.0;
    for i in 0..n {
        if layers[i].1 {
            static_moment += gam[i] * row.e_0_mean * layers[i].0 * z[i];
        }
        if z[i] <= 0.0 && !layers[i].1 {
            tau_r = tau_r.max(v * static_moment.abs() / ei);
        }
    }
    let mut out = vec![
        Outcome::info("ei_ef", "Effective bending stiffness per mm width", "Wirksame Biegesteifigkeit je mm Breite", QuantityKind::Dimensionless, ei, format!("(EI)_ef = {} N·mm²/mm, γ = {:?}", f(ei), gam.iter().map(|g| (g * 1000.0).round() / 1000.0).collect::<Vec<_>>())),
        Outcome::stress("bending", "Outer layer bending stress", "Randspannung Decklage", smax, kmod * row.f_m_k / gm, format!("σ = {} N/mm²", f(smax))),
        Outcome::stress("rolling_shear", "Rolling shear in cross layers", "Rollschub in Querlagen", tau_r, kmod * c.o("f_r_k", ndp_common("ndp.en1995.clt.rolling-shear-strength")) / gm, format!("τ_R = {} N/mm²", f(tau_r))),
        Outcome::stress("gross_shear", "Gross shear", "Brutto-Schub", 1.5 * v / total, kmod * c.o("f_v_clt_k", ndp_common("ndp.en1995.clt.gross-shear-strength")) / gm, "τ = 1.5 V/h".into()),
    ];
    if let Some(q) = c.opt("q_k") {
        let w = 5.0 * q * span.powi(4) / (384.0 * ei);
        out.push(Outcome::length("deflection", "CLT deflection", "BSP-Durchbiegung", w, span / c.o("limit_divisor", 300.0), format!("w = {} mm", f(w))));
    }
    Ok(out)
}

fn t98(c: &Case) -> R {
    let system = c.s("system")?;
    let class = c.so("ductility", "DCM");
    let table: &[(&str, [f64; 3])] = &[("portal_dowelled", [1.5, 2.5, 4.0]), ("nailed_wall", [1.5, 3.0, 5.0]), ("glued_wall", [1.5, 2.0, 2.0]), ("truss_nailed", [1.5, 2.0, 3.0]), ("cantilever", [1.5, 2.0, 2.0])];
    let q = table.iter().find(|r| r.0 == system).map(|r| r.1[["DCL", "DCM", "DCH"].iter().position(|x| *x == class).unwrap_or(1)]).ok_or_else(|| format!("unknown system {system}"))?;
    Ok(vec![Outcome::info("q", "Behaviour factor q (EN 1998-1 Table 8.1)", "Verhaltensbeiwert q (EN 1998-1 Tabelle 8.1)", QuantityKind::Dimensionless, q, format!("q({system}, {class}) = {}; k_mod instantaneous", f(q)))])
}

/// 🗂 Catalogue rows of the component group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-90", part: "EN 1995-1-1", clause: "B", en: "Mechanically jointed beams (γ-method)", de: "Nachgiebig verbundene Biegeträger (γ-Verfahren)", inputs: &["a1", "i1", "e1", "h1", "s1", "a2", "i2", "e2", "h2", "a3", "i3", "e3", "h3", "s3", "k", "l", "m_d", "v_d", "q_d", "fd_a1", "fd_m1", "r_d1"], eval: t90 },
    Entry { id: "T-91", part: "EN 1995-1-1", clause: "9.1.1", en: "Glued thin-webbed beams", de: "Geklebte Träger mit dünnem Steg", inputs: &["class", "b_f", "h_f", "b_w", "h_w", "e_f", "e_w", "m_d", "v_d", "f_glue_d"], eval: t91 },
    Entry { id: "T-92", part: "EN 1995-1-1", clause: "9.1.2", en: "Glued thin-flanged beams", de: "Geklebte Träger mit dünnem Gurt", inputs: &["class", "l", "h_f", "b_w", "h_w", "outstand", "e_f", "e_w", "m_d", "f_flange_d"], eval: t92 },
    Entry { id: "T-93", part: "EN 1995-1-1", clause: "C", en: "Mechanically jointed columns", de: "Mechanisch verbundene Druckstäbe", inputs: &["class", "lef", "i_tot", "l1", "i1", "n_parts", "joint", "n_c", "area"], eval: t93 },
    Entry { id: "T-94", part: "EN 1995-1-1", clause: "9.2.4", en: "Roof, floor and wall diaphragms", de: "Dach-, Decken- und Wandscheiben", inputs: &["f_f_rd", "b_i", "h", "s", "n_panels", "method", "f_v_ed", "r_hold_down", "f_d", "l_dia", "q_v_rd"], eval: t94 },
    Entry { id: "T-95", part: "EN 1995-1-1", clause: "9.2.5", en: "Trusses and stabilising loads", de: "Fachwerkträger und Stabilisierungslasten", inputs: &["n_braced", "n_d", "l_m", "product", "q_rd", "kind", "span", "height", "bays", "chord_b", "chord_h", "web_b", "web_h"], eval: t95 },
    Entry { id: "T-96", part: "EN 1995-1-1", clause: "9.2.5", en: "Bracing systems and restraints", de: "Aussteifungssysteme und Halterungen", inputs: &["n_braced", "n_d", "l_m", "f_rd", "k_br", "kind", "spacing", "member_b", "member_h", "brace_b", "brace_h"], eval: t96 },
    Entry { id: "T-97", part: "EN 1995-1-1", clause: "Annex CLT", en: "Cross-laminated timber", de: "Brettsperrholz", inputs: &["n_layers", "t1", "o1", "l", "m_d", "v_d", "q_k", "class", "g_r"], eval: t97 },
    Entry { id: "T-98", part: "EN 1998-1", clause: "8", en: "Timber behaviour factor q", de: "Verhaltensbeiwert q für Holz", inputs: &["system", "ductility"], eval: t98 },
];
