//! 🧱️ Materials, factors and cross-section checks of EN 1995-1-1 chapters 2, 3 and 6.1, 6.2, 6.6 as catalogue entries T-01 … T-18 and T-50. Units: N, mm, N/mm². See [`super`].

use super::*;

fn dur_list(c: &Case) -> LoadDuration {
    c.so("durations", c.so("duration", "medium")).split(',').map(parse_load_duration).max().unwrap_or(LoadDuration::Medium)
}

fn t01(c: &Case) -> R {
    let class = c.s("class")?;
    let r = strength_row(class).ok_or_else(|| format!("unknown strength class {class}"))?;
    let src = format!("{} {} ({})", r.source, r.name, if r.verified { "pinned" } else { "unverified" });
    let st = |k: &str, en: &str, de: &str, v: f64| Outcome::info(k, en, de, QuantityKind::Stress, v * 1e6, format!("{k} = {} N/mm², {src}", f(v)));
    Ok(vec![
        st("f_m_k", "Bending strength f_m,k", "Biegefestigkeit f_m,k", r.f_m_k),
        st("f_t_0_k", "Tension parallel f_t,0,k", "Zugfestigkeit parallel f_t,0,k", r.f_t_0_k),
        st("f_t_90_k", "Tension perpendicular f_t,90,k", "Zugfestigkeit quer f_t,90,k", r.f_t_90_k),
        st("f_c_0_k", "Compression parallel f_c,0,k", "Druckfestigkeit parallel f_c,0,k", r.f_c_0_k),
        st("f_c_90_k", "Compression perpendicular f_c,90,k", "Druckfestigkeit quer f_c,90,k", r.f_c_90_k),
        st("f_v_k", "Shear f_v,k", "Schubfestigkeit f_v,k", r.f_v_k),
        st("e_0_mean", "Modulus E_0,mean", "Elastizitätsmodul E_0,mean", r.e_0_mean),
        st("e_0_05", "Modulus E_0,05", "Elastizitätsmodul E_0,05", r.e_0_05),
        st("g_mean", "Shear modulus G_mean", "Schubmodul G_mean", r.g_mean),
        Outcome::info("rho_k", "Density ρ_k", "Rohdichte ρ_k", QuantityKind::Dimensionless, r.rho_k, format!("ρ_k = {} kg/m³, {src}", f(r.rho_k))),
    ])
}

fn t02(c: &Case) -> R {
    let class = c.s("class")?;
    let r = strength_row(class).ok_or_else(|| format!("unknown strength class {class}"))?;
    let material = c.so("material", product_key(r.product));
    let sc = c.o("sc", 1.0) as u8;
    let d = dur_list(c);
    let k = k_mod_for(material, sc, d).ok_or("k_mod not allowed for this service class")?;
    let mut out = vec![Outcome::info("k_mod", "Modification factor k_mod", "Modifikationsbeiwert k_mod", QuantityKind::Dimensionless, k, format!("k_mod({material}, SC{sc}, {d:?}) = {} (Table 3.1, shortest duration governs)", f(k)))];
    if let Some(k2) = c.opt("kmod_2") {
        let kc = (k * k2).sqrt();
        out.push(Outcome::info("k_mod_conn", "Connection k_mod", "Verbindungs-k_mod", QuantityKind::Dimensionless, kc, format!("k_mod = √(k_mod,1·k_mod,2) = √({}·{}) = {}", f(k), f(k2), f(kc))));
    }
    Ok(out)
}

fn t03(c: &Case) -> R {
    let material = c.s("material")?;
    let acc = c.so("situation", "persistent") == "accidental";
    let g = gamma_m_for(c.annex, material, acc).ok_or_else(|| format!("unknown material {material}"))?;
    let fire = if c.so("situation", "persistent") == "fire" { 1.0 } else { g };
    Ok(vec![Outcome::info("gamma_m", "Partial factor γ_M", "Teilsicherheitsbeiwert γ_M", QuantityKind::Dimensionless, fire, format!("γ_M({material}, {:?}) = {}", c.annex, f(fire)))])
}

fn t04(c: &Case) -> R {
    let material = c.s("material")?;
    let sc = c.o("sc", 1.0) as u8;
    let kdef = k_def_for(material, sc).ok_or("k_def not allowed for this service class")?;
    let mut out = vec![Outcome::info("k_def", "Deformation factor k_def", "Verformungsbeiwert k_def", QuantityKind::Dimensionless, kdef, format!("k_def({material}, SC{sc}) = {}", f(kdef)))];
    if let (Some(g), Some(q)) = (c.opt("u_inst_g"), c.opt("u_inst_q")) {
        let psi2 = c.o("psi_2", 0.3);
        let u = g * (1.0 + kdef) + q * (1.0 + psi2 * kdef);
        out.push(Outcome::info("u_fin", "Final deformation u_fin", "Enddeformation u_fin", QuantityKind::Length, u / 1e3, format!("u_fin = {}·(1+{}) + {}·(1+{}·{}) = {} mm", f(g), f(kdef), f(q), f(psi2), f(kdef), f(u))));
    }
    if let Some(k2) = c.opt("kdef_2") {
        let ke = 2.0 * (kdef * k2).sqrt();
        out.push(Outcome::info("k_def_conn", "Connection k_def", "Verbindungs-k_def", QuantityKind::Dimensionless, ke, format!("k_def = 2·√(k_def,1·k_def,2) = {}", f(ke))));
    }
    if let Some(em) = c.opt("e_mean") {
        let ef = em / (1.0 + c.o("psi_2", 0.3) * kdef);
        out.push(Outcome::info("e_fin", "Final modulus E_fin", "Endmodul E_fin", QuantityKind::Stress, ef * 1e6, format!("E_fin = E_mean/(1+ψ₂k_def) = {} N/mm²", f(ef))));
    }
    Ok(out)
}

fn t05(c: &Case) -> R {
    let class = c.s("class")?;
    let r = strength_row(class).ok_or_else(|| format!("unknown strength class {class}"))?;
    let h = c.n("h")?;
    let s = c.o("s_lvl", 0.12);
    let kh = size_factor_kh(r.product, r.rho_k, h, s);
    let mut out = vec![Outcome::info("k_h", "Size factor k_h", "Größenbeiwert k_h", QuantityKind::Dimensionless, kh, format!("k_h({:?}, h={} mm) = {}", r.product, f(h), f(kh)))];
    if r.product == TimberProduct::Lvl {
        let l = c.o("length", 3000.0);
        let kl = (3000.0 / l).powf(s / 2.0).min(1.1);
        out.push(Outcome::info("k_l", "LVL length factor k_l", "LVL-Längenbeiwert k_l", QuantityKind::Dimensionless, kl, format!("k_l = min((3000/l)^(s/2), 1.1) = {}", f(kl))));
    }
    if let Some(v) = c.opt("volume") {
        let kv = (1e7 / v).powf(0.2);
        out.push(Outcome::info("k_vol", "Volume factor k_vol", "Volumenbeiwert k_vol", QuantityKind::Dimensionless, kv, format!("k_vol = (V0/V)^0.2 = {}", f(kv))));
    }
    Ok(out)
}

fn t06(c: &Case) -> R {
    let m = Mat::load(c)?;
    let h = c.o("h", 600.0);
    let kh = m.kh_bending(c, h);
    let r = m.row;
    let d = |k: &str, en: &str, de: &str, fk: f64, factor: f64| {
        let v = factor * m.fd(fk);
        Outcome::info(k, en, de, QuantityKind::Stress, v * 1e6, format!("{k} = {}·k_mod·f_k/γ_M = {}·{}·{}/{} = {} N/mm²", f(factor), f(factor), f(m.kmod), f(fk), f(m.gamma), f(v)))
    };
    Ok(vec![
        d("f_m_d", "Design bending f_m,d", "Bemessungsbiegefestigkeit f_m,d", r.f_m_k, kh),
        d("f_t_0_d", "Design tension f_t,0,d", "Bemessungszugfestigkeit f_t,0,d", r.f_t_0_k, kh),
        d("f_t_90_d", "Design tension perpendicular f_t,90,d", "Bemessungszugfestigkeit quer f_t,90,d", r.f_t_90_k, 1.0),
        d("f_c_0_d", "Design compression f_c,0,d", "Bemessungsdruckfestigkeit f_c,0,d", r.f_c_0_k, 1.0),
        d("f_c_90_d", "Design compression perpendicular f_c,90,d", "Bemessungsdruckfestigkeit quer f_c,90,d", r.f_c_90_k, 1.0),
        d("f_v_d", "Design shear f_v,d", "Bemessungsschubfestigkeit f_v,d", r.f_v_k, 1.0),
    ])
}

fn tension_dim(c: &Case, m: &Mat, s: &Sec) -> f64 {
    c.opt("kh_dim").unwrap_or(if m.row.product == TimberProduct::Solid { s.b.max(s.h) } else { s.b })
}

fn t10(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let n = c.n("n_t")?;
    let a = c.opt("area_net").unwrap_or(s.a);
    let kh = m.kh_bending(c, tension_dim(c, &m, &s));
    let fd = kh * m.fd(m.row.f_t_0_k);
    let sd = n / a;
    Ok(vec![Outcome::stress("tension", "Tension parallel to grain (6.1.2)", "Zug parallel zur Faser (6.1.2)", sd, fd, format!("σ_t,0,d = {}/{} = {} ≤ f_t,0,d = k_h·k_mod·f_t,0,k/γ_M = {} (k_h={})", f(n), f(a), f(sd), f(fd), f(kh)))])
}

fn t11(c: &Case) -> R {
    let m = Mat::load(c)?;
    let sd = c.n("sigma_t_90")?;
    let kvol = c.opt("volume").map(|v| (1e7 / v).powf(0.2)).unwrap_or(1.0);
    let fd = kvol * m.fd(m.row.f_t_90_k);
    Ok(vec![Outcome::stress("tension90", "Tension perpendicular to grain (6.1.3)", "Zug quer zur Faser (6.1.3)", sd, fd, format!("σ_t,90,d = {} ≤ k_vol·f_t,90,d = {}·{} (k_mod={}, γ_M={})", f(sd), f(kvol), f(m.fd(m.row.f_t_90_k)), f(m.kmod), f(m.gamma)))])
}

fn t12(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let n = c.n("n_c")?;
    let fd = m.fd(m.row.f_c_0_k);
    let sd = n / s.a;
    let mut out = vec![Outcome::stress("compression", "Compression parallel to grain (6.1.4)", "Druck parallel zur Faser (6.1.4)", sd, fd, format!("σ_c,0,d = {}/{} = {} ≤ f_c,0,d = {}", f(n), f(s.a), f(sd), f(fd)))];
    if let (Some(ly), Some(lz)) = (c.opt("lef_y"), c.opt("lef_z")) {
        let lam = |l: f64, i: f64| (l / (i / s.a).sqrt() / std::f64::consts::PI) * (m.row.f_c_0_k / m.row.e_0_05).sqrt();
        let rel = lam(ly, s.iy).max(lam(lz, s.iz));
        out.push(Outcome::ratio("stability_required", "Stability check required if λ_rel > 0.3", "Stabilitätsnachweis erforderlich bei λ_rel > 0,3", if rel > 0.3 { 1.0 } else { 0.0 }, 1.0, format!("λ_rel,max = {} (limit 0.3)", f(rel))));
    }
    Ok(out)
}

/// 📏 Effective contact length l_ef = l + a₁ + a₂ with each spread min(30 mm, a, l, l₁/2) per EN 1995-1-1 6.1.5 (1).
pub fn bearing_length_ef(l: f64, a_left: f64, a_right: f64, l1: f64) -> f64 {
    let spread = |a: f64| 30.0_f64.min(a).min(l).min(l1 / 2.0).max(0.0);
    l + spread(a_left) + spread(a_right)
}

/// 🧱 k_c,90 of EN 1995-1-1 6.1.5 (4)/(5): 1.25/1.5 (solid continuous/discrete), 1.5/1.75 (glulam continuous/discrete, l ≤ 400 mm), otherwise 1.0.
pub fn k_c90_factor(product: TimberProduct, hardwood: bool, continuous: bool, l: f64, l1: f64, h: f64) -> f64 {
    if hardwood || l1 < 2.0 * h {
        return 1.0;
    }
    match (product, continuous) {
        (TimberProduct::Solid, true) => 1.25,
        (TimberProduct::Solid, false) if l <= 400.0 => 1.5,
        (TimberProduct::Glulam, true) => 1.5,
        (TimberProduct::Glulam, false) if l <= 400.0 => 1.75,
        _ => 1.0,
    }
}

fn t13(c: &Case) -> R {
    let m = Mat::load(c)?;
    let f_c90 = c.n("f_c90")?;
    let l = c.n("l")?;
    let b = c.n("b")?;
    let l1 = c.o("l1", 1e9);
    let h = c.o("h", 0.0);
    let lef = bearing_length_ef(l, c.o("a_left", 1e9), c.o("a_right", 1e9), l1);
    let kc = c.opt("k_c90").unwrap_or_else(|| k_c90_factor(m.row.product, m.row.hardwood, c.so("support", "discrete") == "continuous", l, l1, h));
    let sd = f_c90 / (b * lef);
    let fd = kc * m.fd(m.row.f_c_90_k);
    Ok(vec![Outcome::stress("bearing", "Compression perpendicular to grain (6.1.5)", "Druck quer zur Faser (6.1.5)", sd, fd, format!("l_ef = {} mm, σ_c,90,d = {}/({}·{}) = {} ≤ k_c,90·f_c,90,d = {}·{}", f(lef), f(f_c90), f(b), f(lef), f(sd), f(kc), f(m.fd(m.row.f_c_90_k))))])
}

fn t14(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let my = c.o("m_y", 0.0);
    let mz = c.o("m_z", 0.0);
    let km = s.km(c);
    let fmy = m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    let fmz = m.kh_bending(c, c.opt("kh_dim_z").unwrap_or(s.b)) * m.fd(m.row.f_m_k);
    let (sy, sz) = (my / s.wy, mz / s.wz);
    let u1 = sy / fmy + km * sz / fmz;
    let u2 = km * sy / fmy + sz / fmz;
    Ok(vec![
        Outcome::ratio("bending_6_11", "Biaxial bending (6.11)", "Zweiachsige Biegung (6.11)", u1, 1.0, format!("σ_m,y/f_m,y + k_m·σ_m,z/f_m,z = {}/{} + {}·{}/{} = {}", f(sy), f(fmy), f(km), f(sz), f(fmz), f(u1))),
        Outcome::ratio("bending_6_12", "Biaxial bending (6.12)", "Zweiachsige Biegung (6.12)", u2, 1.0, format!("k_m·σ_m,y/f_m,y + σ_m,z/f_m,z = {}", f(u2))),
    ])
}

/// ✂️ k_cr of EN 1995-1-1 6.1.7 (2) and DIN EN 1995-1-1/NA for `f_v_k` in N/mm² (see [`k_cr_value`]).
pub fn k_cr_factor(annex: AnnexChoice, product: TimberProduct, f_v_k: f64) -> f64 {
    k_cr_value(annex, product, f_v_k * 1e6)
}

fn t15(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let kcr = c.opt("k_cr").unwrap_or_else(|| k_cr_factor(c.annex, m.row.product, m.row.f_v_k));
    let mut v = c.n("v")?;
    if let (Some(vp), Some(a)) = (c.opt("v_point"), c.opt("a_support")) {
        let beta = (a / (2.0 * s.h)).min(1.0);
        v = (v - vp * (1.0 - beta)).max(0.0);
    }
    if v < 0.0 {
        return Err("shear force v must not be negative".into());
    }
    let tau = match c.opt("s_first") {
        Some(sm) => v * sm / (s.iy * kcr * s.b),
        None => 1.5 * v / (kcr * s.b * s.h),
    };
    let fd = m.fd(m.row.f_v_k);
    Ok(vec![Outcome::stress("shear", "Shear (6.1.7)", "Schub (6.1.7)", tau, fd, format!("τ_d = {}/(k_cr·b·h) with k_cr={} → {} ≤ f_v,d = {}", f(v), f(kcr), f(tau), f(fd)))])
}

/// 🌀 Torsion section factor α of a rectangle for h/b (EN 1995-1-1 6.1.8, W_t = α b² h, interpolated).
pub fn torsion_alpha(ratio: f64) -> f64 {
    const T: [(f64, f64); 5] = [(1.0, 0.208), (2.0, 0.246), (3.0, 0.267), (5.0, 0.291), (10.0, 0.312)];
    let r = ratio.max(1.0);
    if r >= 10.0 {
        return 1.0 / 3.0 - (1.0 / 3.0 - 0.312) * 10.0 / r;
    }
    for w in T.windows(2) {
        if r <= w[1].0 {
            return w[0].1 + (r - w[0].0) / (w[1].0 - w[0].0) * (w[1].1 - w[0].1);
        }
    }
    0.312
}

fn t16(c: &Case) -> R {
    let m = Mat::load(c)?;
    let mt = c.n("m_t")?;
    let (wt, ksh) = if c.so("shape", "rect") == "circle" {
        let d = c.n("d")?;
        (std::f64::consts::PI * d.powi(3) / 16.0, 1.2)
    } else {
        let (b, h) = (c.n("b")?, c.n("h")?);
        let (bs, hl) = (b.min(h), b.max(h));
        let ksh = if c.so("shape", "rect") == "rect" { (1.0 + 0.15 * hl / bs).min(2.0) } else { 1.0 };
        (torsion_alpha(hl / bs) * bs * bs * hl, ksh)
    };
    let tau = mt / wt;
    let fv = m.fd(m.row.f_v_k);
    let mut out = vec![Outcome::stress("torsion", "Torsion (6.1.8)", "Torsion (6.1.8)", tau, ksh * fv, format!("τ_tor,d = {}/{} = {} ≤ k_sh·f_v,d = {}·{}", f(mt), f(wt), f(tau), f(ksh), f(fv)))];
    if let Some(tv) = c.opt("tau_v") {
        let u = tau / (ksh * fv) + (tv / fv).powi(2);
        out.push(Outcome::ratio("torsion_shear", "Combined torsion and shear (6.15)", "Torsion und Schub kombiniert (6.15)", u, 1.0, format!("τ_tor/(k_sh f_v) + (τ_v/f_v)² = {}", f(u))));
    }
    Ok(out)
}

fn t17(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let km = s.km(c);
    let (sy, sz) = (c.o("m_y", 0.0) / s.wy, c.o("m_z", 0.0) / s.wz);
    let fmy = m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    let fmz = m.kh_bending(c, c.opt("kh_dim_z").unwrap_or(s.b)) * m.fd(m.row.f_m_k);
    let nt = c.o("n_t", 0.0);
    let nc = c.o("n_c", 0.0);
    if nt > 0.0 {
        let ft = m.kh_bending(c, tension_dim(c, &m, &s)) * m.fd(m.row.f_t_0_k);
        let st = nt / s.a;
        let (u1, u2) = (st / ft + sy / fmy + km * sz / fmz, st / ft + km * sy / fmy + sz / fmz);
        return Ok(vec![
            Outcome::ratio("tension_bending_6_17", "Tension and bending (6.17)", "Zug und Biegung (6.17)", u1, 1.0, format!("σ_t/f_t + σ_my/f_my + k_m σ_mz/f_mz = {}", f(u1))),
            Outcome::ratio("tension_bending_6_18", "Tension and bending (6.18)", "Zug und Biegung (6.18)", u2, 1.0, format!("σ_t/f_t + k_m σ_my/f_my + σ_mz/f_mz = {}", f(u2))),
        ]);
    }
    let fc = m.fd(m.row.f_c_0_k);
    let sc = nc / s.a;
    let (u1, u2) = ((sc / fc).powi(2) + sy / fmy + km * sz / fmz, (sc / fc).powi(2) + km * sy / fmy + sz / fmz);
    Ok(vec![
        Outcome::ratio("compression_bending_6_19", "Compression and bending (6.19)", "Druck und Biegung (6.19)", u1, 1.0, format!("(σ_c/f_c)² + σ_my/f_my + k_m σ_mz/f_mz = {}", f(u1))),
        Outcome::ratio("compression_bending_6_20", "Compression and bending (6.20)", "Druck und Biegung (6.20)", u2, 1.0, format!("(σ_c/f_c)² + k_m σ_my/f_my + σ_mz/f_mz = {}", f(u2))),
    ])
}

/// 🕳️ The hole diameter limit d ≤ 0.15·h of the NCI to EN 1995-1-1 5.2, shared by the check and the net-section diagram.
pub const HOLE_LIMIT_RATIO: f64 = 0.15;

fn t18(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let n = c.n("n_t")?;
    let holes = c.o("hole_d", 0.0) * c.o("hole_count", 0.0);
    let a_net = s.a - holes * s.b;
    let ft = m.kh_bending(c, tension_dim(c, &m, &s)) * m.fd(m.row.f_t_0_k);
    let mut out = vec![Outcome::stress("net_section", "Net section tension (5.2)", "Nettoquerschnitt Zug (5.2)", n / a_net, ft, format!("A_net = {} − {}·{} = {} mm²", f(s.a), f(holes), f(s.b), f(a_net)))];
    if let Some(d) = c.opt("hole_d") {
        out.push(Outcome::length("hole_limit", "Hole diameter limit d ≤ 0.15·h (NCI)", "Lochdurchmesser d ≤ 0,15·h (NCI)", d, HOLE_LIMIT_RATIO * s.h, format!("d = {} mm vs {HOLE_LIMIT_RATIO} h = {} mm", f(d), f(HOLE_LIMIT_RATIO * s.h))));
    }
    Ok(out)
}

/// 🧩 k_sys of EN 1995-1-1 6.6: 1.1 for at least four similar members at spacing ≤ 1.2 m joined by a load-distributing system, otherwise 1.0.
pub fn k_sys_factor(members: f64, spacing_m: f64, distributing: bool) -> f64 {
    if members >= 4.0 && spacing_m <= 1.2 && distributing { 1.1 } else { 1.0 }
}

fn t50(c: &Case) -> R {
    let m = Mat::load(c)?;
    let s = Sec::load(c)?;
    let ks = k_sys_factor(c.n("n_members")?, c.n("spacing_m")?, c.o("distributing", 1.0) > 0.5);
    let sy = c.n("m_y")? / s.wy;
    let fmd = ks * m.kh_bending(c, c.opt("kh_dim").unwrap_or(s.h)) * m.fd(m.row.f_m_k);
    Ok(vec![Outcome::stress("system_bending", "Bending with system strength (6.6)", "Biegung mit Systemfestigkeit (6.6)", sy, fmd, format!("k_sys = {}, σ_m,d = {} ≤ k_sys·f_m,d = {}", f(ks), f(sy), f(fmd)))])
}

/// 🗂 Catalogue rows of the material and cross-section group.
pub const ENTRIES: &[Entry] = &[
    Entry { id: "T-01", part: "EN 1995-1-1", clause: "3.2", en: "Strength class properties", de: "Festigkeitsklassen-Kennwerte", inputs: &["class"], eval: t01 },
    Entry { id: "T-02", part: "EN 1995-1-1", clause: "3.1.3", en: "Modification factor k_mod", de: "Modifikationsbeiwert k_mod", inputs: &["class", "sc", "duration", "kmod_2"], eval: t02 },
    Entry { id: "T-03", part: "EN 1995-1-1", clause: "2.4.1", en: "Partial factor γ_M", de: "Teilsicherheitsbeiwert γ_M", inputs: &["material", "situation"], eval: t03 },
    Entry { id: "T-04", part: "EN 1995-1-1", clause: "3.2", en: "Deformation factor k_def", de: "Verformungsbeiwert k_def", inputs: &["material", "sc", "u_inst_g", "u_inst_q", "psi_2"], eval: t04 },
    Entry { id: "T-05", part: "EN 1995-1-1", clause: "3.3", en: "Size factor k_h", de: "Größenbeiwert k_h", inputs: &["class", "h", "volume"], eval: t05 },
    Entry { id: "T-06", part: "EN 1995-1-1", clause: "2.4.1", en: "Design strengths", de: "Bemessungsfestigkeiten", inputs: &["class", "sc", "duration", "h"], eval: t06 },
    Entry { id: "T-10", part: "EN 1995-1-1", clause: "6.1.2", en: "Tension parallel to grain", de: "Zug parallel zur Faser", inputs: &["class", "n_t", "b", "h"], eval: t10 },
    Entry { id: "T-11", part: "EN 1995-1-1", clause: "6.1.3", en: "Tension perpendicular to grain", de: "Zug quer zur Faser", inputs: &["class", "sigma_t_90"], eval: t11 },
    Entry { id: "T-12", part: "EN 1995-1-1", clause: "6.1.4", en: "Compression parallel to grain", de: "Druck parallel zur Faser", inputs: &["class", "n_c", "b", "h"], eval: t12 },
    Entry { id: "T-13", part: "EN 1995-1-1", clause: "6.1.5", en: "Compression perpendicular to grain", de: "Druck quer zur Faser", inputs: &["class", "f_c90", "l", "b", "h", "l1", "support"], eval: t13 },
    Entry { id: "T-14", part: "EN 1995-1-1", clause: "6.1.6", en: "Bending", de: "Biegung", inputs: &["class", "m_y", "m_z", "b", "h"], eval: t14 },
    Entry { id: "T-15", part: "EN 1995-1-1", clause: "6.1.7", en: "Shear", de: "Schub", inputs: &["class", "v", "b", "h", "v_point", "a_support"], eval: t15 },
    Entry { id: "T-16", part: "EN 1995-1-1", clause: "6.1.8", en: "Torsion", de: "Torsion", inputs: &["class", "m_t", "b", "h", "tau_v"], eval: t16 },
    Entry { id: "T-17", part: "EN 1995-1-1", clause: "6.2.3", en: "Axial force and bending combined", de: "Normalkraft und Biegung kombiniert", inputs: &["class", "n_t", "n_c", "m_y", "m_z", "b", "h"], eval: t17 },
    Entry { id: "T-18", part: "EN 1995-1-1", clause: "5.2", en: "Net sections, notches and holes", de: "Nettoquerschnitte, Ausklinkungen und Löcher", inputs: &["class", "n_t", "b", "h", "hole_d", "hole_count"], eval: t18 },
    Entry { id: "T-50", part: "EN 1995-1-1", clause: "6.6", en: "System strength", de: "Systemfestigkeit", inputs: &["class", "m_y", "b", "h", "n_members", "spacing_m"], eval: t50 },
];
