//! 🪝️ Fastenings to concrete of EN 1992-4 as catalogue entries FA-01 … FA-14: load distribution, steel, cone, pull-out, splitting, blow-out, steel shear, pry-out, concrete edge, interaction and anchor reinforcement. See [`super`].

use super::*;
use semio_s_plugin_norm_mechanics::fastening as kernel;

pub struct Group {
    x: Vec<f64>,
    y: Vec<f64>,
    region: kernel::Region,
}

fn group(c: &Case) -> Result<Group, String> {
    let (x, y) = (c.l("ax").map(|v| v.to_vec()).unwrap_or_else(|_| vec![0.0]), c.l("ay").map(|v| v.to_vec()).unwrap_or_else(|_| vec![0.0]));
    if x.len() != y.len() { return Err("ax and ay differ in length".into()); }
    Ok(Group { x, y, region: kernel::Region { x_min: c.o("x_min", -1e9), x_max: c.o("x_max", 1e9), y_min: c.o("y_min", -1e9), y_max: c.o("y_max", 1e9) } })
}

impl Group {
    fn points(&self, subset: &[usize]) -> Vec<[f64; 2]> {
        subset.iter().map(|&i| [self.x[i], self.y[i]]).collect()
    }
}

fn fa01(c: &Case) -> R {
    let g = group(c)?;
    let n = g.x.len() as f64;
    let (ned, mx, my, ved) = (c.o("n_ed", 0.0), c.o("m_x", 0.0), c.o("m_y", 0.0), c.o("v_ed", 0.0));
    let (cx, cy) = (g.x.iter().sum::<f64>() / n, g.y.iter().sum::<f64>() / n);
    let sx: f64 = g.x.iter().map(|v| (v - cx) * (v - cx)).sum();
    let sy: f64 = g.y.iter().map(|v| (v - cy) * (v - cy)).sum();
    let mut ni: Vec<f64> = (0..g.x.len()).map(|i| ned / n + if sy > 0.0 { mx * (g.y[i] - cy) / sy } else { 0.0 } + if sx > 0.0 { my * (g.x[i] - cx) / sx } else { 0.0 }).collect();
    if c.so("method", "elastic") == "plate_edge" && mx != 0.0 {
        let yc = c.opt("y_compression").unwrap_or_else(|| g.y.iter().cloned().fold(f64::MAX, f64::min));
        let arm: Vec<f64> = g.y.iter().map(|y| (y - yc).max(0.0)).collect();
        let s2: f64 = arm.iter().map(|a| a * a).sum();
        let lam = (mx + ned * (c.o("y_n", 0.0) - yc)) / s2.max(1e-12);
        ni = arm.iter().map(|a| lam.max(0.0) * a).collect();
    } else {
        for v in ni.iter_mut() { if *v < 0.0 { *v = 0.0; } }
    }
    let mut out: Vec<Outcome> = ni.iter().enumerate().map(|(i, v)| Outcome::n(&format!("n_{i}"), &format!("Tension load of anchor {i}"), &format!("Zuglast Anker {i}"), *v, format!("method {}", c.so("method", "elastic")))).collect();
    out.push(Outcome::n("v_each", "Shear per anchor", "Querkraft je Anker", ved / n, "V/n".into()));
    out.push(Outcome::n("n_max", "Largest anchor tension", "Größte Ankerzugkraft", ni.iter().cloned().fold(0.0, f64::max), "".into()));
    Ok(out)
}

/// 🔩 γ_Ms for tension (N) or shear (V) failure of anchor steel.
pub fn gamma_ms(fuk: f64, fyk: f64, shear: bool) -> f64 {
    let n = |key: &str| semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1992.anchor-gamma-ms", &[key]);
    kernel::SteelPartialFactors { f_uk_max: n("f-uk-max"), ratio_max: n("ratio-max"), slope_n: n("n.slope"), min_n: n("n.min"), slope_v: n("v.slope"), min_v: n("v.min"), other: n("other") }.gamma_ms(fuk, fyk, shear)
}

fn fa02(c: &Case) -> R {
    let (as_, fuk, fyk) = (c.pos("as")?, c.pos("fuk")?, c.pos("fyk")?);
    let g = gamma_ms(fuk, fyk, false);
    let nrk = kernel::steel_n_rk_s(as_, fuk);
    let mut v = vec![Outcome::n("n_rk_s", "N_Rk,s", "N_Rk,s", nrk, "A_s f_uk".into()), Outcome::n("n_rd_s", "N_Rd,s", "N_Rd,s", nrk / g, format!("γ_Ms = {}", f(g)))];
    if let Some(n) = c.opt("n_ed") { v.push(Outcome::force("util", "Steel tension utilisation", "Stahlzugausnutzung", n, nrk / g, "N_Ed ≤ N_Rd,s".into())); }
    Ok(v)
}

fn k1_cone(c: &Case) -> f64 {
    let cracked = c.o("cracked", 1.0) > 0.0;
    let cast = c.so("type", "cast_in") == "cast_in";
    c.ndp(match (cast, cracked) { (true, true) => "ndp.en1992.cone-k-cast-cracked", (true, false) => "ndp.en1992.cone-k-cast-uncracked", (false, true) => "ndp.en1992.cone-k-post-cracked", (false, false) => "ndp.en1992.cone-k-post-uncracked" })
}

fn gamma_mc(c: &Case) -> f64 {
    c.ndp("ndp.en1992.m-01.gamma-c-persistent") * c.o("gamma_inst", c.ndp("ndp.en1992.gamma-inst"))
}

/// 🧮 Concrete cone resistance of the tension anchors `subset` (N_Rk,c, ratio A_c,N/A_0, ψ values) from the fastening kernel.
fn cone(c: &Case, g: &Group, subset: &[usize]) -> Result<(f64, f64, f64, f64, f64), String> {
    let (fck, hef) = (c.pos("fck")?, c.pos("hef")?);
    let anchors = g.points(subset);
    let input = kernel::ConeInput { f_ck: fck, h_ef: c.opt("hef_eff").unwrap_or(hef), k1: k1_cone(c), anchors: &anchors, region: g.region, dense_reinforcement: c.o("dense_reinforcement", 0.0) > 0.0, e_nx: c.o("e_nx", 0.0), e_ny: c.o("e_ny", 0.0), z_compression: c.o("z_compression", 0.0) };
    let r = kernel::cone_breakout(&input);
    Ok((r.n_rk_c, r.ratio, r.psi_s, r.psi_re, r.psi_ec))
}

fn all_idx(g: &Group) -> Vec<usize> {
    (0..g.x.len()).collect()
}

fn fa03(c: &Case) -> R {
    let g = group(c)?;
    let idx = all_idx(&g);
    let (nrk, ratio, ps, pre, pec) = cone(c, &g, &idx)?;
    let gm = gamma_mc(c);
    let mut v = vec![Outcome::n("n_rk_c", "N_Rk,c", "N_Rk,c", nrk, format!("A_c,N/A_0 = {}, ψ_s = {}, ψ_re = {}, ψ_ec = {}", f(ratio), f(ps), f(pre), f(pec))), Outcome::n("n_rd_c", "N_Rd,c", "N_Rd,c", nrk / gm, format!("γ_Mc = {}", f(gm)))];
    if let Some(n) = c.opt("n_ed") { v.push(Outcome::force("util", "Concrete cone utilisation", "Betonausbruchausnutzung", n, nrk / gm, "N_Ed ≤ N_Rd,c".into())); }
    Ok(v)
}

fn fa04(c: &Case) -> R {
    let (fck, d, dh) = (c.pos("fck")?, c.pos("d")?, c.pos("dh")?);
    let ah = kernel::head_bearing_area(d, dh);
    let k2 = c.ndp(if c.o("cracked", 1.0) > 0.0 { "ndp.en1992.pullout-k2-cracked" } else { "ndp.en1992.pullout-k2-uncracked" });
    let nrk = match c.opt("n_rk_p0") { Some(n0) => kernel::pullout_n_rk_p_scaled(n0, fck), None => kernel::pullout_n_rk_p(k2, fck, d, dh) };
    let gm = gamma_mc(c);
    let mut v = vec![Outcome::n("n_rk_p", "N_Rk,p", "N_Rk,p", nrk, format!("A_h = {}", f(ah))), Outcome::n("n_rd_p", "N_Rd,p", "N_Rd,p", nrk / gm, "".into())];
    if let Some(n) = c.opt("n_ed") { v.push(Outcome::force("util", "Pull-out utilisation", "Herausziehausnutzung", n, nrk / gm, "N_Ed ≤ N_Rd,p".into())); }
    Ok(v)
}

fn fa05(c: &Case) -> R {
    let g = group(c)?;
    let idx = all_idx(&g);
    let (nrc, ..) = cone(c, &g, &idx)?;
    let hef = c.pos("hef")?;
    let nrp = c.opt("n_rk_p").unwrap_or(f64::MAX);
    let (h, hmin) = (c.pos("h")?, c.pos("h_min")?);
    let psi_h = kernel::psi_h_sp(h, hmin, hef);
    let nrk = kernel::splitting_n_rk_sp(nrc, nrp, psi_h);
    let mut v = vec![Outcome::n("n_rk_sp", "N_Rk,sp", "N_Rk,sp", nrk, format!("ψ_h,sp = {}", f(psi_h)))];
    if c.o("splitting_reinforcement", 0.0) > 0.0 && c.o("cracked", 1.0) > 0.0 { v.push(Outcome::num("exempt", "Splitting verification not required (1 = yes)", "Spaltnachweis entfällt (1 = ja)", 1.0, "7.2.1.5(3)".into())); }
    if let Some(n) = c.opt("n_ed") { v.push(Outcome::force("util", "Splitting utilisation", "Spaltausnutzung", n, nrk / gamma_mc(c), "N_Ed ≤ N_Rd,sp".into())); }
    Ok(v)
}

fn fa06(c: &Case) -> R {
    let (fck, c1, hef, dh, d) = (c.pos("fck")?, c.pos("c1")?, c.pos("hef")?, c.pos("dh")?, c.pos("d")?);
    if c1 > 0.5 * hef { return Ok(vec![Outcome::num("applicable", "Blow-out not decisive (c_1 > 0.5 h_ef)", "Seitlicher Ausbruch nicht maßgebend (c_1 > 0,5 h_ef)", 0.0, "7.2.1.6".into())]); }
    let ah = std::f64::consts::PI / 4.0 * (dh * dh - d * d);
    let n0 = 8.7 * c1 * ah.sqrt() * fck.sqrt();
    let s2 = c.o("s2", 0.0);
    let ratio = (1.0 + s2 / (4.0 * c1)).max(1.0);
    let nrk = n0 * ratio * c.o("psi_h_nb", 1.0);
    let mut v = vec![Outcome::n("n_rk_cb", "N_Rk,cb", "N_Rk,cb", nrk, format!("N⁰ = {}", f(n0)))];
    if let Some(n) = c.opt("n_ed") { v.push(Outcome::force("util", "Blow-out utilisation", "Ausbruchausnutzung", n, nrk / gamma_mc(c), "N_Ed ≤ N_Rd,cb".into())); }
    Ok(v)
}

fn fa07(c: &Case) -> R {
    let (as_, fuk, fyk) = (c.pos("as")?, c.pos("fuk")?, c.pos("fyk")?);
    let g = gamma_ms(fuk, fyk, true);
    let k7 = if c.o("grout", 0.0) > 0.0 { 0.8 } else { 1.0 };
    let (vrk, why, why_de) = if let (Some(wel), Some(lz)) = (c.opt("w_el"), c.opt("lever")) {
        let n_util = c.o("n_ed", 0.0) / (kernel::steel_n_rk_s(as_, fuk) / gamma_ms(fuk, fyk, false)).max(1.0);
        (kernel::steel_v_rk_s_lever(c.o("alpha_m", 1.0), wel, fuk, lz, n_util), "with lever arm".to_string(), "mit Hebelarm".to_string())
    } else {
        (kernel::steel_v_rk_s(as_, fuk, k7), "without lever arm".to_string(), "ohne Hebelarm".to_string())
    };
    let mut v = vec![Outcome::n("v_rk_s", "V_Rk,s", "V_Rk,s", vrk, why).steps_de(why_de), Outcome::n("v_rd_s", "V_Rd,s", "V_Rd,s", vrk / g, format!("γ_Ms,V = {}", f(g)))];
    if let Some(vd) = c.opt("v_ed") { v.push(Outcome::force("util", "Steel shear utilisation", "Stahlquerkraftausnutzung", vd, vrk / g, "V_Ed ≤ V_Rd,s".into())); }
    Ok(v)
}

fn fa08(c: &Case) -> R {
    let g = group(c)?;
    let idx = all_idx(&g);
    let (nrk, ..) = cone(c, &g, &idx)?;
    let hef = c.pos("hef")?;
    let k8 = kernel::pryout_k8(hef);
    let vrk = kernel::pryout_v_rk_cp(nrk, hef);
    let mut v = vec![Outcome::n("v_rk_cp", "V_Rk,cp", "V_Rk,cp", vrk, format!("k_8 = {}", f(k8))), Outcome::n("v_rd_cp", "V_Rd,cp", "V_Rd,cp", vrk / gamma_mc(c), "".into())];
    if let Some(vd) = c.opt("v_ed") { v.push(Outcome::force("util", "Pry-out utilisation", "Betonausbruch lastabgewandt Ausnutzung", vd, vrk / gamma_mc(c), "V_Ed ≤ V_Rd,cp".into())); }
    Ok(v)
}

fn fa09(c: &Case) -> R {
    let (fck, c1, h, dn, hef) = (c.pos("fck")?, c.pos("c1")?, c.pos("h")?, c.pos("d_nom")?, c.pos("hef")?);
    let psi_re = match c.so("edge_reinforcement", "none") { "edge" => 1.2, "stirrups" => 1.4, _ => 1.0 };
    let k9 = if c.o("cracked", 1.0) > 0.0 { 1.7 } else { 2.4 };
    let input = kernel::EdgeInput { f_ck: fck, k9, d_nom: dn, h_ef: hef, c1, h, c2_left: c.o("c2_left", 1e9), c2_right: c.o("c2_right", 1e9), s2: c.o("s2", 0.0), alpha_v_deg: c.o("alpha_v_deg", 0.0), e_v: c.o("e_v", 0.0), psi_re };
    let (v0, ratio, vrk) = { let e = kernel::edge_breakout(&input); (e.v0_rk_c, e.ratio, e.v_rk_c) };
    let gm = gamma_mc(c);
    let mut v = vec![Outcome::n("v_rk_c", "V_Rk,c", "V_Rk,c", vrk, format!("V⁰ = {}, A_c,V/A_0 = {}", f(v0), f(ratio))), Outcome::n("v_rd_c", "V_Rd,c", "V_Rd,c", vrk / gm, "".into())];
    if let Some(vd) = c.opt("v_ed") { v.push(Outcome::force("util", "Concrete edge utilisation", "Betonkantenbruchausnutzung", vd, vrk / gm, "V_Ed ≤ V_Rd,c".into())); }
    Ok(v)
}

fn fa10(c: &Case) -> R {
    let (n, v) = (c.o("n_ed", 0.0), c.o("v_ed", 0.0));
    let mut out = Vec::new();
    if let (Some(ns), Some(vs)) = (c.opt("n_rd_s"), c.opt("v_rd_s")) { out.push(Outcome::ratio("util_steel", "Steel interaction", "Stahlinteraktion", kernel::interaction(n, ns, v, vs, 2.0), 1.0, "(N/N_Rd,s)² + (V/V_Rd,s)² ≤ 1".into())); }
    let nrd = ["n_rd_p", "n_rd_c", "n_rd_sp", "n_rd_cb"].iter().filter_map(|k| c.opt(k)).fold(f64::MAX, f64::min);
    let vrd = ["v_rd_c", "v_rd_cp"].iter().filter_map(|k| c.opt(k)).fold(f64::MAX, f64::min);
    if nrd < f64::MAX && vrd < f64::MAX {
        out.push(Outcome::ratio("util_concrete", "Concrete interaction", "Betoninteraktion", kernel::interaction(n, nrd, v, vrd, 1.5), 1.0, "(N/N_Rd)^1.5 + (V/V_Rd)^1.5 ≤ 1".into()));
        out.push(Outcome::ratio("util_linear", "Linear interaction", "Lineare Interaktion", n / nrd + v / vrd, 1.2, "N/N_Rd + V/V_Rd ≤ 1.2".into()));
    }
    if out.is_empty() { return Err("missing resistances".into()); }
    Ok(out)
}

fn fa11(c: &Case) -> R {
    let g = group(c)?;
    let ni = c.l("ni")?;
    let tens: Vec<usize> = (0..g.x.len()).filter(|&i| ni.get(i).copied().unwrap_or(0.0) > 0.0).collect();
    if tens.is_empty() { return Err("no tension anchor".into()); }
    let sum: f64 = tens.iter().map(|&i| ni[i]).sum();
    let (cx, cy) = (tens.iter().map(|&i| ni[i] * g.x[i]).sum::<f64>() / sum, tens.iter().map(|&i| ni[i] * g.y[i]).sum::<f64>() / sum);
    let (mx, my) = (tens.iter().map(|&i| g.x[i]).sum::<f64>() / tens.len() as f64, tens.iter().map(|&i| g.y[i]).sum::<f64>() / tens.len() as f64);
    let mut case = c.clone();
    case.num.insert("e_nx".into(), (cx - mx).abs());
    case.num.insert("e_ny".into(), (cy - my).abs());
    let (nrk, ratio, ..) = cone(&case, &g, &tens)?;
    Ok(vec![Outcome::mm("e_nx", "Eccentricity e_N,x", "Ausmitte e_N,x", (cx - mx).abs(), "tension anchors only".into()).steps_de("nur Zuganker"), Outcome::mm("e_ny", "Eccentricity e_N,y", "Ausmitte e_N,y", (cy - my).abs(), "".into()), Outcome::num("ac_ratio", "A_c,N/A_0 of the tension anchors", "A_c,N/A_0 der Zugankergruppe", ratio, "union of cone footprints".into()).steps_de("Vereinigung der Kegelgrundflächen"), Outcome::n("n_rk_c", "N_Rk,c of the group", "N_Rk,c der Gruppe", nrk, "".into()), Outcome::force("util", "Group cone utilisation", "Gruppenausbruch Ausnutzung", sum, nrk / gamma_mc(c), "ΣN_Ed ≤ N_Rd,c".into()), Outcome::n("n_max_steel", "Largest anchor load", "Größte Ankerlast", tens.iter().map(|&i| ni[i]).fold(0.0, f64::max), "governs steel failure".into()).steps_de("Stahlversagen maßgebend")])
}

fn fa12(c: &Case) -> R {
    let (d0, ded) = (c.pos("delta_n_rk_s_0")?, c.pos("delta_n_ed")?);
    let seis = c.o("alpha_seis", 1.0);
    let rd = d0 * seis / c.ndp("ndp.en1992.gamma-ms-fat");
    Ok(vec![Outcome::n("delta_n_rd", "ΔN_Rd,s", "ΔN_Rd,s", rd, "ETA ΔN_Rk,s,0,n/γ_Ms,fat".into()), Outcome::force("util", "Anchor fatigue range", "Ankerermüdung Schwingbreite", ded, rd, "ΔN_Ed ≤ ΔN_Rd,s".into())])
}

fn fa13(c: &Case) -> R {
    fa08(c)
}

fn fa14(c: &Case) -> R {
    let (ned, fyk) = (c.pos("n_ed_re")?, c.o("fyk", 500.0));
    let fyd = fyk / c.ndp("ndp.en1992.gamma-ms-re");
    let as_re = ned / fyd;
    let mut v = vec![Outcome::mm2("as_re", "Required anchor reinforcement A_s,re", "Erforderliche Rückhängebewehrung A_s,re", as_re, "N_Ed,re/f_yd".into())];
    if let Some(p) = c.opt("as_provided") { v.push(Outcome::area("util", "Anchor reinforcement utilisation", "Rückhängebewehrung Ausnutzung", as_re, p, "A_s,re ≤ A_s,prov".into())); }
    if let (Some(lbd), Some(ls)) = (c.opt("l_bd"), c.opt("l_s")) { v.push(Outcome::length("anchorage", "Anchorage in the cone", "Verankerung im Ausbruchkegel", lbd, ls, "l_bd ≤ l_s".into())); }
    Ok(v)
}

/// 🗂 Entries of this domain.
pub const ENTRIES: &[Entry] = &[
    entry_of("FA-01", "DIN EN 1992-4", "6.2.2", "Anchor load distribution", "Lastverteilung auf Anker", &["ax", "ay", "n_ed", "m_x", "m_y", "v_ed", "method", "y_compression", "y_n"], &[], fa01),
    entry_of("FA-02", "DIN EN 1992-4", "7.2.1.2", "Steel failure in tension", "Stahlversagen Zug", &["as", "fuk", "fyk", "n_ed"], &["ndp.en1992.anchor-gamma-ms.f-uk-max", "ndp.en1992.anchor-gamma-ms.ratio-max", "ndp.en1992.anchor-gamma-ms.n.slope", "ndp.en1992.anchor-gamma-ms.n.min", "ndp.en1992.anchor-gamma-ms.v.slope", "ndp.en1992.anchor-gamma-ms.v.min", "ndp.en1992.anchor-gamma-ms.other"], fa02),
    entry_of("FA-03", "DIN EN 1992-4", "7.2.1.4", "Concrete cone failure", "Betonausbruch", &["fck", "hef", "ax", "ay", "x_min", "x_max", "y_min", "y_max", "cracked", "type", "dense_reinforcement", "e_nx", "e_ny", "z_compression", "gamma_inst", "n_ed", "hef_eff"], &["ndp.en1992.cone-k-cast-cracked", "ndp.en1992.cone-k-cast-uncracked", "ndp.en1992.cone-k-post-cracked", "ndp.en1992.cone-k-post-uncracked", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst", "ndp.en1992.psi-m-n"], fa03),
    entry_of("FA-04", "DIN EN 1992-4", "7.2.1.5", "Pull-out failure", "Herausziehen", &["fck", "d", "dh", "cracked", "n_rk_p0", "n_ed"], &["ndp.en1992.pullout-k2-cracked", "ndp.en1992.pullout-k2-uncracked", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa04),
    entry_of("FA-05", "DIN EN 1992-4", "7.2.1.7", "Splitting failure", "Spalten", &["fck", "hef", "ax", "ay", "h", "h_min", "n_rk_p", "splitting_reinforcement", "cracked", "n_ed"], &["ndp.en1992.cone-k-cast-cracked", "ndp.en1992.cone-k-cast-uncracked", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa05),
    entry_of("FA-06", "DIN EN 1992-4", "7.2.1.8", "Blow-out failure", "Seitlicher Betonausbruch", &["fck", "c1", "hef", "dh", "d", "s2", "n_ed"], &["ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa06),
    entry_of("FA-07", "DIN EN 1992-4", "7.2.2.3", "Steel failure in shear", "Stahlversagen Querkraft", &["as", "fuk", "fyk", "w_el", "lever", "alpha_m", "n_ed", "grout", "v_ed"], &["ndp.en1992.anchor-shear-k", "ndp.en1992.anchor-gamma-ms.f-uk-max", "ndp.en1992.anchor-gamma-ms.ratio-max", "ndp.en1992.anchor-gamma-ms.n.slope", "ndp.en1992.anchor-gamma-ms.n.min", "ndp.en1992.anchor-gamma-ms.v.slope", "ndp.en1992.anchor-gamma-ms.v.min", "ndp.en1992.anchor-gamma-ms.other"], fa07),
    entry_of("FA-08", "DIN EN 1992-4", "7.2.2.4", "Concrete pry-out", "Betonausbruch auf der lastabgewandten Seite", &["fck", "hef", "ax", "ay", "cracked", "type", "v_ed"], &["ndp.en1992.cone-k-cast-cracked", "ndp.en1992.cone-k-cast-uncracked", "ndp.en1992.cone-k-post-cracked", "ndp.en1992.cone-k-post-uncracked", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa08),
    entry_of("FA-09", "DIN EN 1992-4", "7.2.2.5", "Concrete edge failure", "Betonkantenbruch", &["fck", "c1", "h", "d_nom", "hef", "c2_left", "c2_right", "s2", "alpha_v_deg", "e_v", "edge_reinforcement", "cracked", "v_ed"], &["ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa09),
    entry_of("FA-10", "DIN EN 1992-4", "7.2.3", "Combined tension and shear", "Kombinierte Zug- und Querkraftbeanspruchung", &["n_ed", "v_ed", "n_rd_s", "v_rd_s", "n_rd_p", "n_rd_c", "n_rd_sp", "n_rd_cb", "v_rd_c", "v_rd_cp"], &[], fa10),
    entry_of("FA-11", "DIN EN 1992-4", "7.2.1.3", "Group effects", "Gruppeneffekte", &["fck", "hef", "ax", "ay", "ni", "x_min", "x_max", "y_min", "y_max", "cracked", "type"], &["ndp.en1992.cone-k-cast-cracked", "ndp.en1992.cone-k-cast-uncracked", "ndp.en1992.cone-k-post-cracked", "ndp.en1992.cone-k-post-uncracked", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa11),
    entry_of("FA-12", "DIN EN 1992-4", "5", "Fatigue and seismic", "Ermüdung und Erdbeben", &["delta_n_rk_s_0", "delta_n_ed", "alpha_seis"], &["ndp.en1992.gamma-ms-fat"], fa12),
    entry_of("FA-13", "DIN EN 1992-4", "7.2.2.4", "Pry-out of groups", "Betonausbruch Gruppen lastabgewandt", &["fck", "hef", "ax", "ay", "cracked", "type", "v_ed"], &["ndp.en1992.cone-k-cast-cracked", "ndp.en1992.cone-k-cast-uncracked", "ndp.en1992.cone-k-post-cracked", "ndp.en1992.cone-k-post-uncracked", "ndp.en1992.m-01.gamma-c-persistent", "ndp.en1992.gamma-inst"], fa13),
    entry_of("FA-14", "DIN EN 1992-4", "7.2.1.8", "Anchor reinforcement", "Rückhängebewehrung", &["n_ed_re", "fyk", "as_provided", "l_bd", "l_s"], &["ndp.en1992.gamma-ms-re"], fa14),
];
