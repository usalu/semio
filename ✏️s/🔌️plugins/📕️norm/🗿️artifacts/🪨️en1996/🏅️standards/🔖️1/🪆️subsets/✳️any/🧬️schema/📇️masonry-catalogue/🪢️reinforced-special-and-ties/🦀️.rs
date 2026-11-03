//! 🪢️ Reinforced, confined, prestressed masonry, ties and anchors of the EN 1996 catalogue: M-40 … M-52. See [`super`].

use super::*;
use super::numerical_models::{relocation, MnSection, Support};

/// 🔑️ Path keys of the movable rows of a row of supports, in position order (a document case binds the key `row_i` to the position field of its `i`-th row).
pub const ROW_KEYS: [&str; 16] = ["row_0", "row_1", "row_2", "row_3", "row_4", "row_5", "row_6", "row_7", "row_8", "row_9", "row_10", "row_11", "row_12", "row_13", "row_14", "row_15"];

/// 📍️ The outcome with the one-click move of a support that brings the largest interval of the row within `limit` (`end_factor` times at the wall ends), or the outcome unchanged when the case carries no row positions (`pos_m`, `l_m`) or no single move helps.
fn move_fix(c: &Case, outcome: Outcome, limit: f64, end_factor: f64, what: (&str, &str)) -> Outcome {
    let (Some(pos), Ok(l)) = (c.vec.get("pos_m"), c.n("l_m")) else { return outcome };
    if pos.len() > ROW_KEYS.len() {
        return outcome;
    }
    let mut points = vec![Support { at: 0.0, movable: false }];
    points.extend(pos.iter().enumerate().map(|(i, &at)| Support { at, movable: c.paths.contains_key(ROW_KEYS[i]) }));
    points.push(Support { at: l, movable: false });
    let last = points.len() - 2;
    let limits: Vec<f64> = (0..=last).map(|i| if i == 0 || i == last { limit * end_factor } else { limit }).collect();
    match relocation(&points, &limits) {
        Some((j, to)) => {
            let from = points[j].at;
            outcome.fix(ROW_KEYS[j - 1], QuantityKind::Length, from, to, to > from).fix_note(format!("Move the {} from x = {from:.3} m to x = {to:.3} m so that no spacing exceeds {limit:.3} m.", what.0), format!("{} von x = {from:.3} m nach x = {to:.3} m verschieben, damit kein Abstand {limit:.3} m überschreitet.", what.1))
        }
        None => outcome,
    }
}

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "M-40", part: "EN 1996-1-1", clause: "6.6.1, 6.6.2", en: "Reinforced masonry in bending", de: "Bewehrtes Mauerwerk auf Biegung", inputs: &["b_m", "d_m", "as_m2", "fyd_pa", "fd_pa", "m_ed_nm"], eval: m40 },
    Entry { id: "M-41", part: "EN 1996-1-1", clause: "6.7.2", en: "Reinforced masonry in shear", de: "Bewehrtes Mauerwerk auf Querkraft", inputs: &["b_m", "d_m", "fvk_pa", "fd_pa", "asw_m2", "s_m", "fyd_pa", "v_ed_n", "av_m"], eval: m41 },
    Entry { id: "M-42", part: "EN 1996-1-1", clause: "6.7", en: "Deep beams and M-N interaction of reinforced walls", de: "Wandträger und M-N-Interaktion bewehrter Wände", inputs: &["b_m", "l_m", "h_m", "m_ed_nm", "n_ed_n", "fd_pa", "fyd_pa", "bar_y_m", "bar_a_m2"], eval: m42 },
    Entry { id: "M-43", part: "EN 1996-1-1", clause: "8.7", en: "Confined masonry", de: "Eingefasstes Mauerwerk", inputs: &["col_a_m2", "col_min_m", "col_as_m2", "col_spacing_m", "beam_a_m2", "v_ed_n", "l_m", "t_m", "fvk_pa", "fd_pa", "n_ed_n"], eval: m43 },
    Entry { id: "M-44", part: "EN 1996-1-1", clause: "3.4, 6.8", en: "Prestressed masonry", de: "Vorgespanntes Mauerwerk", inputs: &["p0_n", "ap_m2", "fpk_pa", "ep_pa", "e_pa", "phi_inf", "eps_cs", "rho_relax", "a_m2", "n_ed_n", "fd_pa", "fxk1_pa", "z_m3"], eval: m44 },
    Entry { id: "M-50", part: "EN 1996-1-1", clause: "8.5.2.3", en: "Wall ties in cavity walls", de: "Drahtanker in zweischaligen Wänden", inputs: &["n_per_m2", "cavity_m", "h_wall_m", "w_ed_pa", "t1_m", "t2_m", "tie_rd_n"], eval: m50 },
    Entry { id: "M-51", part: "EN 1996-1-1", clause: "8.5.1", en: "Wall-floor anchors and ring beams", de: "Wand-Decken-Anker und Ringbalken", inputs: &["w_ed_pa", "h_trib_m", "s_anchor_m", "anchor_rk_n", "ring_tie_n", "gamma_m"], eval: m51 },
    Entry { id: "M-45", part: "EN 1996-1-1", clause: "8.2.2", en: "Anchorage of reinforcement bars", de: "Verankerung der Bewehrungsstäbe", inputs: &["diameter_m", "provided_m", "fyd_pa", "gamma_m"], eval: m45 },
    Entry { id: "M-52", part: "EN 1996-2", clause: "2.3", en: "Movement joint spacing", de: "Dehnfugenabstand", inputs: &["spacing_m", "material"], eval: m52 },
];

fn m40(c: &Case) -> R {
    let (b, d, a_s) = (c.n("b_m")?, c.n("d_m")?, c.n("as_m2")?);
    let (fyd, fd, m_ed) = (c.o("fyd_pa", f_yd_b500_pa(c.annex)), c.f_d_pa()?, c.n("m_ed_nm")?);
    let z = (d * (1.0 - 0.5 * a_s * fyd / (b * d * fd))).min(0.95 * d);
    let m_steel = a_s * fyd * z;
    let m_cap = 0.4 * fd * b * d * d;
    let m_rd = m_steel.min(m_cap);
    let (rho_min, rho_max) = (0.0005 * b * d, 0.04 * b * d);
    Ok(vec![
        Outcome::info(QuantityKind::Length, "z", "lever arm z", "Hebelarm z", z, format!("z={:.2} mm ≤ 0.95 d={:.2} mm", z * 1e3, 0.95 * d * 1e3)).basis(&["ndp.en1996.m40.flexure"]),
        Outcome::limit(QuantityKind::Moment, "m-rd", "M_Ed ≤ min(A_s f_yd z, 0.4 f_d b d²)", "M_Ed ≤ min(A_s f_yd z, 0,4 f_d b d²)", m_ed, m_rd.max(1e-9), format!("M_Rd,steel={:.1} kN·m, cap={:.1} kN·m", m_steel / 1e3, m_cap / 1e3)).basis(&["ndp.en1996.m40.flexure"]).fix("as_m2", QuantityKind::Area, a_s, ceil_to(m_ed / (fyd * z.max(1e-9)), 1e-6), true),
        Outcome::limit(QuantityKind::Area, "as-min", "A_s ≥ 0.05 % b d", "A_s ≥ 0,05 % b d", rho_min, a_s.max(1e-12), format!("A_s,min={rho_min:.6} m²")).basis(&["ndp.en1996.m40.flexure"]).fix("as_m2", QuantityKind::Area, a_s, ceil_to(rho_min, 1e-6), true),
        Outcome::limit(QuantityKind::Area, "as-max", "A_s ≤ 4 % b d", "A_s ≤ 4 % b d", a_s, rho_max, format!("A_s,max={rho_max:.6} m²")).basis(&["ndp.en1996.m40.flexure"]).fix("as_m2", QuantityKind::Area, a_s, rho_max, false),
    ])
}

fn m41(c: &Case) -> R {
    let (b, d, fd, v) = (c.n("b_m")?, c.n("d_m")?, c.f_d_pa()?, c.n("v_ed_n")?);
    let gamma = c.gamma_m();
    let enh = c.opt("av_m").map_or(1.0, |av| (2.0 * d / av.max(1e-9)).clamp(1.0, 2.0));
    let v1 = c.n("fvk_pa")? / gamma * b * d * enh;
    let v2 = match (c.opt("asw_m2"), c.opt("s_m")) {
        (Some(asw), Some(s)) => 0.9 * asw * c.o("fyd_pa", f_yd_b500_pa(c.annex)) * d / s,
        _ => 0.0,
    };
    let cap = 0.25 * fd * b * d;
    let v_rd = (v1 + v2).min(cap);
    Ok(vec![
        Outcome::info(QuantityKind::Force, "v-rd1", "V_Rd1 = f_vd b d", "V_Rd1 = f_vd b d", v1, format!("enhancement {enh:.2}")).basis(&["ndp.en1996.m41.shear"]),
        Outcome::info(QuantityKind::Force, "v-rd2", "V_Rd2 = 0.9 A_sw f_yd d/s", "V_Rd2 = 0,9 A_sw f_yd d/s", v2, format!("V_Rd2={v2:.0} N")).basis(&["ndp.en1996.m41.shear"]),
        Outcome::limit(QuantityKind::Force, "v-rd", "V_Ed ≤ min(V_Rd1 + V_Rd2, 0.25 f_d b d)", "V_Ed ≤ min(V_Rd1 + V_Rd2, 0,25 f_d b d)", v, v_rd.max(1e-9), format!("V_Rd={v_rd:.0} N, cap={cap:.0} N")).basis(&["ndp.en1996.m41.shear"]).fix("d_m", QuantityKind::Length, d, solve_threshold(d, 5.0, 1e-3, |x| v <= (c.n("fvk_pa").unwrap_or(0.0) / gamma * b * x * enh + if let (Some(asw), Some(s)) = (c.opt("asw_m2"), c.opt("s_m")) { 0.9 * asw * c.o("fyd_pa", f_yd_b500_pa(c.annex)) * x / s } else { 0.0 }).min(0.25 * fd * b * x)).unwrap_or(5.0), true),
    ])
}

fn m42(c: &Case) -> R {
    let (b, l, fd) = (c.n("b_m")?, c.n("l_m")?, c.f_d_pa()?);
    let fyd = c.o("fyd_pa", f_yd_b500_pa(c.annex));
    let m_ed = c.n("m_ed_nm")?;
    let mut out = Vec::new();
    if let Ok(h) = c.n("h_m") {
        if h >= 0.5 * l {
            let z = 0.7 * l;
            let fs = m_ed / z;
            out.push(Outcome::info(QuantityKind::Length, "z-deep", "deep-beam lever arm z = 0.7 l", "Wandträger Hebelarm z = 0,7 l", z, format!("F_s=M/z={fs:.0} N")).basis(&["ndp.en1996.m42.deep"]));
            out.push(Outcome::info(QuantityKind::Area, "as-deep", "required tie area A_s = F_s/f_yd", "erforderliche Zugbewehrung A_s = F_s/f_yd", fs / fyd, format!("A_s={:.6} m²", fs / fyd)).basis(&["ndp.en1996.m42.deep"]));
        }
    }
    let (ys, areas) = (c.v("bar_y_m")?, c.v("bar_a_m2")?);
    if ys.len() != areas.len() || ys.is_empty() {
        return Err("bar_y_m and bar_a_m2 must have equal non-zero length".into());
    }
    let section = MnSection { b, depth: l, f_d: fd, f_yd: fyd, e_s: 200e9, bars: ys.iter().copied().zip(areas.iter().copied()).collect() };
    let n_ed = c.o("n_ed_n", 0.0);
    let m_rd = section.m_rd(n_ed);
    out.push(Outcome::limit(QuantityKind::Moment, "m-rd-n", "M_Ed ≤ M_Rd(N_Ed) of the M-N diagram", "M_Ed ≤ M_Rd(N_Ed) des M-N-Diagramms", m_ed, m_rd.max(1e-9), format!("M_Rd(N={n_ed:.0} N)={m_rd:.0} N·m, ε_mu=0.0035, block 0.8 x")).basis(&["ndp.en1996.m42.deep"]));
    Ok(out)
}

fn m43(c: &Case) -> R {
    let basis = ["m43.confined"];
    let (a_col, side, as_col, spacing) = (c.n("col_a_m2")?, c.n("col_min_m")?, c.n("col_as_m2")?, c.n("col_spacing_m")?);
    let as_min = (0.008 * a_col).max(200e-6);
    let mut area = Outcome::limit(QuantityKind::Area, "col-area", "tie column area ≥ 0.02 m²", "Fläche Einfassungsstütze ≥ 0,02 m²", 0.02, a_col, format!("{a_col:.4} m²")).basis(&basis).fix("col_a_m2", QuantityKind::Area, a_col, 0.02, true);
    if let (Ok(side), Ok(other)) = (c.n("col_a_side_m"), c.n("col_a_other_m")) {
        let required = ceil_to(0.02 / other.max(1e-9), 1e-3).max(side);
        area = area.fix("col_a_side_m", QuantityKind::Length, side, required, true).fix_note(format!("Widen the smallest tie column from {:.0} mm to {:.0} mm: with its {:.0} mm depth the area reaches the required 0.02 m².", side * 1e3, required * 1e3, other * 1e3), format!("Kleinste Einfassungsstütze von {:.0} mm auf {:.0} mm verbreitern: bei {:.0} mm Tiefe erreicht die Fläche die geforderten 0,02 m².", side * 1e3, required * 1e3, other * 1e3));
    }
    let spacing_outcome = Outcome::limit(QuantityKind::Length, "col-spacing", "tie column spacing ≤ 4 m", "Stützenabstand ≤ 4 m", spacing, 4.0, format!("{spacing:.2} m")).basis(&basis).fix("col_spacing_m", QuantityKind::Length, spacing, 4.0, false);
    let spacing_outcome = move_fix(c, spacing_outcome, 4.0, 1.0, ("tie column", "Einfassungsstütze"));
    let mut out = vec![
        area,
        Outcome::limit(QuantityKind::Length, "col-side", "tie column least side ≥ 0.15 m", "kleinste Seite Einfassungsstütze ≥ 0,15 m", 0.15, side, format!("{side:.3} m")).basis(&basis).fix("col_min_m", QuantityKind::Length, side, 0.15, true),
        Outcome::limit(QuantityKind::Area, "col-reinforcement", "column reinforcement ≥ max(0.8 %, 200 mm²)", "Stützenbewehrung ≥ max(0,8 %; 200 mm²)", as_min, as_col.max(1e-12), format!("A_s,min={as_min:.6} m²")).basis(&basis).fix("col_as_m2", QuantityKind::Area, as_col, ceil_to(as_min, 1e-6), true),
        spacing_outcome,
    ];
    if let (Ok(v), Ok(l), Ok(t), Ok(fvk)) = (c.n("v_ed_n"), c.n("l_m"), c.n("t_m"), c.n("fvk_pa")) {
        let v_rd = fvk / c.gamma_m() * l * t;
        out.push(Outcome::limit(QuantityKind::Force, "shear", "V_Ed ≤ f_vd l t of the confined wall", "V_Ed ≤ f_vd l t der eingefassten Wand", v, v_rd.max(1e-9), format!("V_Rd={v_rd:.0} N")).basis(&basis).fix("t_m", QuantityKind::Length, t, ceil_to(v / (fvk / c.gamma_m() * l), 1e-3), true));
    }
    Ok(out)
}

/// 🧱️ The compression outcome with the thickness that gives the required bearing area `A ≥ (N_Ed + P_0)/f_d` of the net wall section, when the case carries the wall thickness `t_m` bound to the document.
fn bearing_fix(c: &Case, outcome: Outcome, required_area: f64, area: f64) -> Outcome {
    let Some(t) = c.opt("t_m").filter(|_| c.paths.contains_key("t_m")) else { return outcome };
    let required = ceil_to(t * required_area / area.max(1e-9), 1e-3);
    outcome.fix("t_m", QuantityKind::Length, t, required, true).fix_note(format!("Thicken the wall to {:.0} mm: the net bearing area grows from {area:.4} m² to {:.4} m², the required (N_Ed + P_0)/f_d.", required * 1e3, required_area), format!("Wand auf {:.0} mm verdicken: die Netto-Lagerfläche wächst von {area:.4} m² auf {:.4} m², erforderlich (N_Ed + P_0)/f_d.", required * 1e3, required_area))
}

fn m44(c: &Case) -> R {
    let basis = ["m43.prestress"];
    let (p0, ap, fpk, ep) = (c.n("p0_n")?, c.n("ap_m2")?, c.n("fpk_pa")?, c.n("ep_pa")?);
    let (e, phi, a) = (c.n("e_pa")?, c.o("phi_inf", 1.5), c.n("a_m2")?);
    let sigma_p0 = p0 / ap;
    let sigma_m = p0 / a;
    let d_creep = phi * sigma_m * ep / e;
    let d_shrink = c.o("eps_cs", 0.0) * ep;
    let d_relax = c.o("rho_relax", 0.0) * sigma_p0;
    let p_eff = p0 - (d_creep + d_shrink + d_relax) * ap;
    let sigma_eff = p_eff / a;
    let n_ed = c.o("n_ed_n", 0.0);
    let fd = c.f_d_pa()?;
    let mut out = vec![
        Outcome::limit(QuantityKind::Stress, "initial-stress", "σ_p0 ≤ 0.75 f_pk", "σ_p0 ≤ 0,75 f_pk", sigma_p0, 0.75 * fpk, format!("σ_p0={:.1} MPa", sigma_p0 / 1e6)).basis(&basis).fix("p0_n", QuantityKind::Force, p0, 0.75 * fpk * ap, false),
        Outcome::info(QuantityKind::Force, "p-eff", "effective prestress force after losses", "wirksame Vorspannkraft nach Verlusten", p_eff, format!("losses: creep {:.1}, shrinkage {:.1}, relaxation {:.1} MPa", d_creep / 1e6, d_shrink / 1e6, d_relax / 1e6)).basis(&basis),
        bearing_fix(c, Outcome::limit(QuantityKind::Stress, "compression", "(N_Ed + P_eff)/A ≤ f_d", "(N_Ed + P_eff)/A ≤ f_d", (n_ed + p0) / a, fd, "initial prestress plus load against the design strength".into()).steps_de("Vorspannung im Anfangszustand plus Last gegenüber der Bemessungsfestigkeit").basis(&basis).fix("a_m2", QuantityKind::Area, a, ceil_to((n_ed + p0) / fd, 1e-4), true), (n_ed + p0) / fd, a),
    ];
    if let (Ok(fxk1), Ok(z)) = (c.n("fxk1_pa"), c.n("z_m3")) {
        let m_rd = (fxk1 / c.gamma_m() + sigma_eff) * z;
        out.push(Outcome::info(QuantityKind::Moment, "m-rd", "flexural resistance (f_xd1 + σ_p,eff) Z", "Biegewiderstand (f_xd1 + σ_p,eff) Z", m_rd, format!("σ_p,eff={:.3} MPa, M_Rd={m_rd:.0} N·m", sigma_eff / 1e6)).basis(&basis));
    }
    Ok(out)
}

fn m50(c: &Case) -> R {
    let (n, cavity) = (c.n("n_per_m2")?, c.o("cavity_m", 0.1));
    let required = if c.de() { if c.o("h_wall_m", 0.0) <= 12.0 { 5.0 } else { 7.0 } } else if cavity >= 0.1 { 3.0 } else { 2.5 };
    let mut density = Outcome::limit(QuantityKind::Dimensionless, "density", "ties per m² ≥ required", "Anker je m² ≥ erforderlich", required, n.max(1e-9), format!("n={n:.2}/m² against {required:.1}/m²")).basis(&["ndp.en1996.m50.ties"]).fix("n_per_m2", QuantityKind::Dimensionless, n, required, true);
    if let Ok(face) = c.n("face_m2") {
        let count = (required * face).ceil();
        let pitch = (1.0 / required).sqrt();
        density = density.fix_note(format!("Provide at least {count:.0} wall ties over the {face:.2} m² face, one per {:.0} mm square grid, at {required:.1} per m².", pitch * 1e3), format!("Mindestens {count:.0} Drahtanker auf der {face:.2} m² großen Fläche anordnen, je ein Anker pro {:.0} mm Quadratraster, {required:.1} je m².", pitch * 1e3));
    }
    let mut out = vec![density];
    if let (Ok(w), Ok(t1), Ok(t2), Ok(rd)) = (c.n("w_ed_pa"), c.n("t1_m"), c.n("t2_m"), c.n("tie_rd_n")) {
        let share = t1.powi(3) / (t1.powi(3) + t2.powi(3));
        let f = w * share / n.max(1e-9);
        let mut force = Outcome::limit(QuantityKind::Force, "tie-force", "F_tie,Ed = w_Ed t₁³/(t₁³+t₂³) / n ≤ F_tie,Rd", "F_tie,Ed = w_Ed t₁³/(t₁³+t₂³) / n ≤ F_tie,Rd", f, rd, format!("leaf share {share:.3}, F_tie,Ed={f:.0} N")).basis(&["ndp.en1996.m50.ties"]).fix("n_per_m2", QuantityKind::Dimensionless, n, ceil_to(w * share / rd, 0.01), true);
        if c.paths.contains_key("tie_rd_n") {
            let required_rd = ceil_to(f, 1.0);
            force = force.fix("tie_rd_n", QuantityKind::Force, rd, required_rd, true).fix_note(format!("Use ties of design resistance F_tie,Rd ≥ {required_rd:.0} N (the tie force at {n:.2} ties per m²)."), format!("Anker mit Bemessungswiderstand F_tie,Rd ≥ {required_rd:.0} N verwenden (Ankerkraft bei {n:.2} Ankern je m²)."));
        }
        out.push(force);
    }
    Ok(out)
}

fn m51(c: &Case) -> R {
    let (w, h, s, rk) = (c.n("w_ed_pa")?, c.n("h_trib_m")?, c.n("s_anchor_m")?, c.n("anchor_rk_n")?);
    let f = w * h * s;
    let rd = rk / c.o("gamma_m", 1.5);
    let s_max = rd / (w * h).max(1e-9);
    let anchor = Outcome::limit(QuantityKind::Force, "anchor", "F_anchor,Ed = w_Ed h s ≤ R_k/γ_M", "F_Anker,Ed = w_Ed h s ≤ R_k/γ_M", f, rd, format!("F_Ed={f:.0} N, R_d={rd:.0} N; required anchor spacing s ≤ R_d/(w_Ed h)={s_max:.3} m")).basis(&["ndp.en1996.m51.anchor"]).fix("s_anchor_m", QuantityKind::Length, s, s_max, false);
    let mut out = vec![move_fix(c, anchor, s_max, 0.5, ("anchor", "Anker"))];
    if let Ok(tie) = c.n("ring_tie_n") {
        out.push(Outcome::limit(QuantityKind::Force, "ring-tie", "ring beam tie force ≥ 30 kN", "Ringbalken Zugkraft ≥ 30 kN", 30_000.0, tie, format!("{tie:.0} N")).basis(&["ndp.en1996.m51.anchor"]).fix("ring_tie_n", QuantityKind::Force, tie, 30_000.0, true));
    }
    Ok(out)
}

const F_BOK_PA: f64 = 2.0e6;

fn m45(c: &Case) -> R {
    let (phi, provided) = (c.n("diameter_m")?, c.n("provided_m")?);
    let fyd = c.o("fyd_pa", f_yd_b500_pa(c.annex));
    let f_bd = F_BOK_PA / c.o("gamma_m", 1.5);
    let l_b = phi * fyd / (4.0 * f_bd);
    Ok(vec![
        Outcome::info(QuantityKind::Length, "lb", "anchorage length l_b = φ f_yd/(4 f_bd)", "Verankerungslänge l_b = φ f_yd/(4 f_bd)", l_b, format!("f_bd={:.2} MPa", f_bd / 1e6)).basis(&["ndp.en1996.m45.bond"]),
        Outcome::limit(QuantityKind::Length, "anchorage", "anchorage length l_b ≤ provided length", "Verankerungslänge l_b ≤ vorhandene Länge", l_b, provided.max(1e-9), format!("l_b=φ f_yd/(4 f_bd)={l_b:.3} m, f_bd={:.2} MPa", f_bd / 1e6)).basis(&["ndp.en1996.m45.bond"]).fix("provided_m", QuantityKind::Length, provided, l_b, true),
    ])
}

/// 📏️ Recommended movement joint spacing [m] by unit material (clay 12, calcium silicate 8, concrete and aerated concrete 6; recalled, unverified: `ndp.en1996.m52.joints`).
pub fn joint_spacing_limit_m(material: UnitMaterial) -> f64 {
    match material {
        UnitMaterial::Clay => 12.0,
        UnitMaterial::CalciumSilicate => 8.0,
        UnitMaterial::Concrete | UnitMaterial::Aerated => 6.0,
    }
}

fn m52(c: &Case) -> R {
    let limit = joint_spacing_limit_m(c.material()?);
    let s = c.n("spacing_m")?;
    Ok(vec![Outcome::warn(QuantityKind::Length, "spacing", "movement joint spacing within the recommendation", "Dehnfugenabstand innerhalb der Empfehlung", s, limit, format!("{s:.1} m against {limit:.0} m")).basis(&["ndp.en1996.m52.joints"])])
}
