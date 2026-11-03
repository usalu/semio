//! 🏛️ Spread foundations, equilibrium and water: catalogue entries GEO-F01 … GEO-F04, EQU-01, UPL-01, HYD-01 (EN 1997-1 §6.5, §10, Annex D, DIN 4017, DIN 1054).

use super::*;

/// 📥️ Geometry, actions and design soil parameters of one Annex D bearing verification (SI units, degrees).
#[derive(Clone, Copy, Debug)]
pub struct BearingIn {
    pub b: f64,
    pub l: f64,
    pub d: f64,
    pub alpha_deg: f64,
    pub v: f64,
    pub hb: f64,
    pub hl: f64,
    pub e_b: f64,
    pub e_l: f64,
    pub phi_deg: f64,
    pub c: f64,
    pub gamma_below: f64,
    pub q_over: f64,
    pub include_depth: bool,
}

/// 📤️ Factors and resistance of one Annex D bearing verification; `r_over_a` is the resistance per effective area in Pa.
#[derive(Clone, Copy, Debug)]
pub struct BearingOut {
    pub nq: f64,
    pub nc: f64,
    pub ng: f64,
    pub sq: f64,
    pub sc: f64,
    pub sg: f64,
    pub bq: f64,
    pub bc: f64,
    pub iq: f64,
    pub ic: f64,
    pub ig: f64,
    pub b_eff: f64,
    pub l_eff: f64,
    pub a_eff: f64,
    pub r_over_a: f64,
    pub r: f64,
}

/// 📐️ Bearing factor N_q = e^(π tan φ)·tan²(45° + φ/2).
pub fn n_q(phi_deg: f64) -> f64 {
    let p = rad(phi_deg);
    (std::f64::consts::PI * p.tan()).exp() * (std::f64::consts::FRAC_PI_4 + p / 2.0).tan().powi(2)
}

/// 📐️ Bearing factor N_c = (N_q − 1)·cot φ; the Prandtl limit π + 2 at φ = 0.
pub fn n_c(phi_deg: f64) -> f64 {
    if phi_deg.abs() < 1e-9 { std::f64::consts::PI + 2.0 } else { (n_q(phi_deg) - 1.0) / rad(phi_deg).tan() }
}

/// 📐️ Bearing factor N_γ = 2·(N_q − 1)·tan φ (rough base).
pub fn n_gamma(phi_deg: f64) -> f64 {
    2.0 * (n_q(phi_deg) - 1.0) * rad(phi_deg).tan()
}

/// 📐️ Effective footing dimensions (B′ ≤ L′) after eccentricity e_B, e_L (Annex D.1).
pub fn effective_dimensions(b: f64, l: f64, e_b: f64, e_l: f64) -> (f64, f64) {
    let (bb, ll) = ((b - 2.0 * e_b.abs()).max(1e-6), (l - 2.0 * e_l.abs()).max(1e-6));
    if bb <= ll { (bb, ll) } else { (ll, bb) }
}

/// 📐️ Drained bearing resistance of Annex D.4; the optional depth factors d_q, d_c (`include_depth`) are a Hansen/DIN 4017 convention, not part of EN 1997-1 Annex D (EX97-01 … EX97-04).
pub fn bearing_drained(i: &BearingIn) -> Result<BearingOut, String> {
    gate(i.phi_deg > 0.5, "drained bearing needs φ′ > 0.5° (use the undrained entry)")?;
    let (b_eff, l_eff) = effective_dimensions(i.b, i.l, i.e_b, i.e_l);
    let swapped = i.b - 2.0 * i.e_b.abs() > i.l - 2.0 * i.e_l.abs();
    let (hb, hl) = if swapped { (i.hl, i.hb) } else { (i.hb, i.hl) };
    let a_eff = b_eff * l_eff;
    let phi = rad(i.phi_deg);
    let (nq, nc, ng) = (n_q(i.phi_deg), n_c(i.phi_deg), n_gamma(i.phi_deg));
    let ratio = b_eff / l_eff;
    let sq = 1.0 + ratio * phi.sin();
    let sg = 1.0 - 0.3 * ratio;
    let sc = (sq * nq - 1.0) / (nq - 1.0);
    let alpha = rad(i.alpha_deg);
    let bq = (1.0 - alpha * phi.tan()).max(0.0).powi(2);
    let bc = bq - (1.0 - bq) / (nc * phi.tan());
    let m_b = (2.0 + ratio) / (1.0 + ratio);
    let m_l = (2.0 + 1.0 / ratio) / (1.0 + 1.0 / ratio);
    let h = hb.hypot(hl);
    let cos2 = if h > 0.0 { (hl / h).powi(2) } else { 1.0 };
    let m = m_l * cos2 + m_b * (1.0 - cos2);
    let t = 1.0 - h / (i.v + a_eff * i.c / phi.tan());
    gate(t >= 0.0, "horizontal action exceeds V + A′c′cot φ′")?;
    let iq = t.powf(m);
    let ig = t.powf(m + 1.0);
    let ic = iq - (1.0 - iq) / (nc * phi.tan());
    let (dq, dc) = if i.include_depth {
        let dq = 1.0 + 2.0 * phi.tan() * (1.0 - phi.sin()).powi(2) * (i.d / b_eff).atan();
        (dq, dq - (1.0 - dq) / (nc * phi.tan()))
    } else {
        (1.0, 1.0)
    };
    let r_over_a = i.c * nc * bc * sc * ic * dc + i.q_over * nq * bq * sq * iq * dq + 0.5 * i.gamma_below * b_eff * ng * bq * sg * ig;
    Ok(BearingOut { nq, nc, ng, sq, sc, sg, bq, bc, iq, ic, ig, b_eff, l_eff, a_eff, r_over_a, r: r_over_a * a_eff })
}

/// 📐️ Undrained bearing resistance of Annex D.3 per effective area in Pa.
pub fn bearing_undrained(i: &BearingIn, cu: f64) -> Result<BearingOut, String> {
    let (b_eff, l_eff) = effective_dimensions(i.b, i.l, i.e_b, i.e_l);
    let a_eff = b_eff * l_eff;
    let h = i.hb.hypot(i.hl);
    gate(h <= a_eff * cu, "horizontal action exceeds A′·c_u")?;
    let bc = 1.0 - 2.0 * rad(i.alpha_deg) / (std::f64::consts::PI + 2.0);
    let sc = 1.0 + 0.2 * b_eff / l_eff;
    let ic = 0.5 * (1.0 + (1.0 - h / (a_eff * cu)).sqrt());
    let r_over_a = (std::f64::consts::PI + 2.0) * cu * bc * sc * ic + i.q_over;
    Ok(BearingOut { nq: 1.0, nc: std::f64::consts::PI + 2.0, ng: 0.0, sq: 1.0, sc, sg: 1.0, bq: 1.0, bc, iq: 1.0, ic, ig: 1.0, b_eff, l_eff, a_eff, r_over_a, r: r_over_a * a_eff })
}

/// 📐️ Bearing resistance of DIN 4017 (GEO-F03); returns the resistance per effective area in Pa.
pub fn bearing_din4017(i: &BearingIn) -> Result<BearingOut, String> {
    gate(i.phi_deg > 0.5, "DIN 4017 entry needs φ′ > 0.5°")?;
    let (b_eff, l_eff) = effective_dimensions(i.b, i.l, i.e_b, i.e_l);
    let a_eff = b_eff * l_eff;
    let phi = rad(i.phi_deg);
    let nd = n_q(i.phi_deg);
    let nb = (nd - 1.0) * phi.tan();
    let nc = n_c(i.phi_deg);
    let ratio = b_eff / l_eff;
    let sd = 1.0 + ratio * phi.sin();
    let sb = 1.0 - 0.3 * ratio;
    let sc = (sd * nd - 1.0) / (nd - 1.0);
    let h = i.hb.hypot(i.hl);
    let tan_delta = h / i.v;
    gate(tan_delta < 1.0, "H/V must stay below 1 for DIN 4017")?;
    let m = (2.0 + ratio) / (1.0 + ratio);
    let id = (1.0 - 0.7 * tan_delta).powf(m + 1.0);
    let ib = (1.0 - tan_delta).powf(m + 1.0);
    let ic = (id * nd - 1.0) / (nd - 1.0);
    let r_over_a = i.c * nc * sc * ic + i.q_over * nd * sd * id + i.gamma_below * b_eff * nb * sb * ib;
    Ok(BearingOut { nq: nd, nc, ng: 2.0 * nb, sq: sd, sc, sg: sb, bq: 1.0, bc: 1.0, iq: id, ic, ig: ib, b_eff, l_eff, a_eff, r_over_a, r: r_over_a * a_eff })
}

fn bearing_input(c: &Case, p: &AnnexParams, drained: bool) -> Result<BearingIn, String> {
    let (b, l, d) = (c.positive("b")?, c.positive("l")?, c.o("d", 0.0));
    let v = c.positive("v_d")?;
    let phi_k = c.o("phi_k_deg", 0.0);
    let phi_deg = if drained { (rad(phi_k).tan() / p.gamma_phi).atan().to_degrees() } else { 0.0 };
    let gamma2 = c.o("gamma2", c.o("gamma1", 0.0));
    Ok(BearingIn {
        b,
        l,
        d,
        alpha_deg: c.o("alpha_deg", 0.0),
        v,
        hb: c.o("hb_d", 0.0),
        hl: c.o("hl_d", 0.0),
        e_b: c.o("mb_d", 0.0) / v,
        e_l: c.o("ml_d", 0.0) / v,
        phi_deg,
        c: c.o("c_k", 0.0) / p.gamma_c,
        gamma_below: c.o("gamma1", gamma2),
        q_over: gamma2 * d,
        include_depth: c.o("include_depth", 0.0) > 0.5,
    })
}

fn bearing_outcomes(i: &BearingIn, o: &BearingOut, v_d: f64, g_rv: f64) -> Vec<Outcome> {
    let r_d = o.r / g_rv;
    vec![
        Outcome::force("bearing", "Bearing V_d vs R_d", "Grundbruch V_d gegen R_d", v_d, r_d),
        Outcome::number("nq", "N_q", "N_q", o.nq),
        Outcome::number("nc", "N_c", "N_c", o.nc),
        Outcome::number("ng", "N_γ", "N_γ", o.ng),
        Outcome::number("sq", "s_q", "s_q", o.sq),
        Outcome::number("sc", "s_c", "s_c", o.sc),
        Outcome::number("sg", "s_γ", "s_γ", o.sg),
        Outcome::number("iq", "i_q", "i_q", o.iq),
        Outcome::number("ic", "i_c", "i_c", o.ic),
        Outcome::number("ig", "i_γ", "i_γ", o.ig),
        Outcome::number("bq", "b_q", "b_q", o.bq),
        Outcome::number("bc", "b_c", "b_c", o.bc),
        Outcome::metre("b_eff", "Effective width B′", "Wirksame Breite B′", o.b_eff),
        Outcome::metre("l_eff", "Effective length L′", "Wirksame Länge L′", o.l_eff),
        Outcome::pascal("r_over_a", "Resistance per effective area R/A′", "Widerstand je wirksamer Fläche R/A′", o.r_over_a),
        Outcome::newton("r_k", "Characteristic resistance R_k", "Charakteristischer Widerstand R_k", o.r),
        Outcome::length("eccentricity_b", "Eccentricity e_B vs B/3", "Ausmitte e_B gegen B/3", i.e_b.abs(), i.b / 3.0),
        Outcome::length("eccentricity_l", "Eccentricity e_L vs L/3", "Ausmitte e_L gegen L/3", i.e_l.abs(), i.l / 3.0),
        Outcome::ratio("eccentricity_ellipse", "Core ellipse (e_B/B)² + (e_L/L)² vs 1/9", "Kernellipse (e_B/B)² + (e_L/L)² gegen 1/9", (i.e_b / i.b).powi(2) + (i.e_l / i.l).powi(2), 1.0 / 9.0),
    ]
}

fn geo_f01(c: &Case) -> R {
    let p = c.factors();
    let i = bearing_input(c, &p, true)?;
    let o = bearing_drained(&i)?;
    Ok(bearing_outcomes(&i, &o, i.v, p.gamma_r_v))
}

fn geo_f02(c: &Case) -> R {
    let p = c.factors();
    let i = bearing_input(c, &p, false)?;
    let cu = c.positive("cu_k")? / p.gamma_cu;
    let o = bearing_undrained(&i, cu)?;
    Ok(bearing_outcomes(&i, &o, i.v, p.gamma_r_v))
}

fn geo_f03(c: &Case) -> R {
    let p = c.factors();
    let i = bearing_input(c, &p, true)?;
    let o = bearing_din4017(&i)?;
    Ok(bearing_outcomes(&i, &o, i.v, p.gamma_r_v))
}

fn geo_f04(c: &Case) -> R {
    let p = c.factors();
    let h_d = p.gamma_g * c.n("h_g")? + p.gamma_q * c.o("h_q", 0.0);
    let r_t_k = match c.opt("cu_k") {
        Some(cu) => c.positive("a_eff")? * cu / p.gamma_cu,
        None => c.positive("v_k")? * (rad(c.positive("delta_deg")?).tan() / p.gamma_phi),
    };
    let passive = c.o("e_p_k", 0.0) * c.o("passive_share", 0.5) / p.gamma_r_e;
    let r_d = r_t_k / p.gamma_r_h + passive;
    Ok(vec![
        Outcome::force("sliding", "Sliding H_d vs R_d", "Gleiten H_d gegen R_d", h_d, r_d),
        Outcome::newton("r_t_k", "Base resistance R_t,k", "Sohlwiderstand R_t,k", r_t_k),
        Outcome::newton("h_d", "Design horizontal action H_d", "Bemessungshorizontalkraft H_d", h_d),
    ])
}

fn equ_01(c: &Case) -> R {
    let u = c.upl();
    let m_dst = u.gamma_equ_dst * c.n("m_dst_g")? + u.gamma_equ_q * c.o("m_dst_q", 0.0);
    let m_stb = u.gamma_equ_stb * c.positive("m_stb_g")? + c.o("m_tension", 0.0);
    Ok(vec![Outcome::moment("equ", "Overturning E_dst,d vs E_stb,d", "Kippen E_dst,d gegen E_stb,d", m_dst, m_stb)])
}

fn upl_01(c: &Case) -> R {
    let u = c.upl();
    let water = c.opt("head").map(|h| h * c.gamma_w() * c.o("area", 1.0)).unwrap_or(0.0);
    let g_dst = u.gamma_g_dst * (c.o("g_dst", 0.0) + water);
    let q_dst = u.gamma_q_dst * c.o("q_dst", 0.0);
    let g_stb = u.gamma_g_stb * c.positive("g_stb")?;
    let r_d = c.o("r_d", 0.0);
    let v_dst = g_dst + q_dst;
    let ballast = ((v_dst - r_d) / u.gamma_g_stb - c.n("g_stb")?).max(0.0);
    Ok(vec![
        Outcome::force("uplift", "Uplift V_dst,d vs G_stb,d + R_d", "Auftrieb V_dst,d gegen G_stb,d + R_d", v_dst, g_stb + r_d),
        Outcome::newton("ballast", "Required additional characteristic ballast", "Erforderliche zusätzliche charakteristische Auflast", ballast),
    ])
}

fn hyd_01(c: &Case) -> R {
    let u = c.upl();
    let (dh, l, gamma_p, gw) = (c.positive("delta_h")?, c.positive("l_flow")?, c.positive("gamma_prime")?, c.gamma_w());
    let i = dh / l;
    let s_d = u.gamma_hyd * i * gw;
    let g_d = u.gamma_hyd_stb * gamma_p;
    let mut out = vec![
        Outcome::force("heave", "Seepage force S_dst,d vs G′_stb,d", "Strömungskraft S_dst,d gegen G′_stb,d", s_d, g_d),
        Outcome::number("i", "Hydraulic gradient i", "Hydraulisches Gefälle i", i),
        Outcome::number("i_crit", "Critical gradient γ′/γw", "Kritisches Gefälle γ′/γw", gamma_p / gw),
        Outcome::number("safety", "Safety i_crit/i", "Sicherheit i_crit/i", gamma_p / gw / i),
    ];
    if let (Some(d15_f), Some(d85_b), Some(d15_b)) = (c.opt("d15_f"), c.opt("d85_b"), c.opt("d15_b")) {
        out.push(Outcome::ratio("filter_stability", "Filter stability d15,f vs 4·d85,b", "Filterstabilität d15,f gegen 4·d85,b", d15_f, 4.0 * d85_b));
        out.push(Outcome::ratio("filter_permeability", "Filter permeability 4·d15,b vs d15,f", "Filterdurchlässigkeit 4·d15,b gegen d15,f", 4.0 * d15_b, d15_f));
    }
    Ok(out)
}

pub const ENTRIES: &[Entry] = &[
    Entry { id: "GEO-F01", part: "EN 1997-1", clause: "6.5.2", en: "Bearing resistance, drained (Annex D.4)", de: "Grundbruch, dräniert (Anhang D.4)", inputs: &["b", "l", "d", "alpha_deg", "v_d", "hb_d", "hl_d", "mb_d", "ml_d", "phi_k_deg", "c_k", "gamma1", "gamma2", "include_depth", "approach", "situation"], eval: geo_f01 },
    Entry { id: "GEO-F02", part: "EN 1997-1", clause: "6.5.2", en: "Bearing resistance, undrained (Annex D.3)", de: "Grundbruch, undräniert (Anhang D.3)", inputs: &["b", "l", "d", "alpha_deg", "v_d", "hb_d", "hl_d", "mb_d", "ml_d", "cu_k", "gamma2", "approach", "situation"], eval: geo_f02 },
    Entry { id: "GEO-F03", part: "DIN 4017", clause: "6.5.2", en: "Bearing resistance, DIN 4017", de: "Grundbruch nach DIN 4017", inputs: &["b", "l", "d", "v_d", "hb_d", "hl_d", "mb_d", "ml_d", "phi_k_deg", "c_k", "gamma1", "gamma2", "approach", "situation"], eval: geo_f03 },
    Entry { id: "GEO-F04", part: "EN 1997-1", clause: "6.5.3", en: "Sliding resistance", de: "Gleitwiderstand", inputs: &["h_g", "h_q", "v_k", "delta_deg", "cu_k", "a_eff", "e_p_k", "passive_share", "approach", "situation"], eval: geo_f04 },
    Entry { id: "EQU-01", part: "EN 1997-1", clause: "2.4.7.2", en: "Overturning, loss of static equilibrium (EQU)", de: "Kippen, Lagesicherheit (EQU)", inputs: &["m_dst_g", "m_dst_q", "m_stb_g", "m_tension", "situation"], eval: equ_01 },
    Entry { id: "UPL-01", part: "EN 1997-1", clause: "10.2", en: "Uplift by water pressure (UPL)", de: "Auftrieb (UPL)", inputs: &["g_dst", "q_dst", "g_stb", "r_d", "head", "area", "gamma_w", "situation"], eval: upl_01 },
    Entry { id: "HYD-01", part: "EN 1997-1", clause: "10.3", en: "Hydraulic heave (HYD)", de: "Hydraulischer Grundbruch (HYD)", inputs: &["delta_h", "l_flow", "gamma_prime", "gamma_w", "d15_f", "d85_b", "d15_b", "situation"], eval: hyd_01 },
];
