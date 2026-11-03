//! 🥫️ EN 1999 thin-wall checks: cold-formed sheeting (EN 1999-1-4) and shells (EN 1999-1-5).

use super::*;
use super::part_1_4_1_5 as thin;
use super::part_en1990::GoverningEffects;

//#region 🔖️ComplianceHelpers
fn sheet_checks(sheet: &ColdFormedSheet, rib: Option<thin::Rib>, annex: AnnexChoice, params: &na_de::AnnexParams, alloy: &part_1_1::AlloyProps, g: &GoverningEffects) -> Vec<CheckResult> {
    let (fo, g1) = (alloy.f_o_pa, params.gamma_m1);
    let (b, t) = (sheet.width, sheet.thickness);
    let eps = part_1_1::epsilon(fo);
    let class_a = alloy.buckling_class_a;
    let widest = rib.map_or(b, |r| r.top.max(r.bottom).max(r.web_length()));
    let beta_eps = widest / t.max(1e-12) / eps;
    let limits = part_1_1::class_limits(class_a, false, sheet.welded);
    let rho = part_1_1::rho_c(beta_eps, class_a, false, sheet.welded);
    let rho_haz = if sheet.welded { alloy.rho_o_haz } else { 1.0 };
    let w_el = b * t * t / 6.0;
    let flex = rib.map(|r| thin::rib_sheet_flexure(&r, b, t, fo, class_a, sheet.welded, rho_haz, g1));
    let (a_eff, n_rd) = flex.map_or_else(|| { let a = rho * b * t * rho_haz; (a, a * fo / g1) }, |f| (f.a_eff, f.n_rd));
    let m_rd = flex.map_or(rho_haz * w_el * fo / g1, |f| f.m_sag.min(f.m_hog));
    let inertia = flex.map_or(b * t.powi(3) / 12.0, |f| f.inertia);
    let (n, m, v) = (g.n_ed.abs(), g.m_y_ed.hypot(g.m_z_ed), g.v_y_ed.hypot(g.v_z_ed));
    let u_nm = n / n_rd.max(1e-12) + m / m_rd.max(1e-12);
    let label = loc(&format!("Sheet {}", sheet.id), &format!("Blech {}", sheet.id));
    let subject = |path: &str| SubjectRef::new(&sheet.id, format!("coldFormed[id={}].{path}", sheet.id), label.clone());
    let tag_en = format!("(ULS {}, lead {}); {}", g.combination, g.action_id, actions_digest(&sheet.actions));
    let tag_de = format!("(GZT {}, führend {}); {}", g.combination, g.action_id, actions_digest(&sheet.actions));
    let cl = |s: &str| ClauseId::new("EN 1999-1-4", format!("§{s}"), s);
    let part = "DIN EN 1999-1-4";
    let mut out = Vec::new();
    let mut mk = |suffix: &str, sec: &str, title: (&str, &str), computed: Quantity, limit: Quantity, expl: String, u: f64, extra: Vec<Remedy>, lead: &str| {
        let mut rem = extra;
        rem.extend(leaf_remedies("coldFormed", &sheet.id, &sheet.actions, u, &label, "Reduce the sheet load", "Blechlast senken", lead));
        out.push(utilization_check(format!("{suffix}.{}", sheet.id), part, cl(sec), subject("thickness"), loc(title.0, title.1), computed, limit, annex,
            loc(&format!("{expl} {tag_en}"), &format!("{expl} {tag_de}")), rem));
    };
    let thick = |u: f64| -> Vec<Remedy> {
        if u <= 1.0 { return Vec::new(); }
        vec![Remedy::at_least(subject("thickness"), q_length(t), q_length(t * u * 1.05), loc("Increase the sheet thickness.", "Blechdicke erhöhen."))]
    };
    mk("en1999.1-4.axial", "5.5", if rib.is_some() { ("Axial resistance N_Rd of the profiled sheet (effective thickness, HAZ)", "Normalkraftwiderstand N_Rd des Profilblechs (wirksame Dicke, WEZ)") } else { ("Axial resistance N_Rd (effective width, HAZ)", "Normalkraftwiderstand N_Rd (wirksame Breite, WEZ)") }, q_force(n), q_force(n_rd),
        format!("β/ε = {beta_eps:.2} (limits {:.1}/{:.1}/{:.1}), ρ_c = {rho:.4}, ρ_o,haz = {rho_haz:.2}, A_eff = {:.1} mm², N_Rd = {:.1} kN", limits[0], limits[1], limits[2], a_eff * 1e6, n_rd / 1e3), n / n_rd.max(1e-12), thick(n / n_rd.max(1e-12)), "nK");
    let bend_text = flex.map_or_else(|| format!("W_el = {:.1} mm³, ρ_o,haz = {rho_haz:.2}, M_Rd = {:.3} kNm", w_el * 1e9, m_rd / 1e3), |f| format!("per rib p = {:.0} mm: W_eff,sag = {:.0} mm³, W_eff,hog = {:.0} mm³; M_Rd,sag = {:.3} kNm, M_Rd,hog = {:.3} kNm per b = {:.0} mm, ρ_o,haz = {rho_haz:.2}", rib.map_or(0.0, |r| r.pitch) * 1e3, f.w_sag * 1e9, f.w_hog * 1e9, f.m_sag / 1e3, f.m_hog / 1e3, b * 1e3));
    mk("en1999.1-4.bend", "6.1", if rib.is_some() { ("Bending resistance M_Rd of the effective rib (sagging and hogging)", "Biegewiderstand M_Rd der wirksamen Rippe (Feld und Stütze)") } else { ("Bending resistance M_Rd (W_el = b·t²/6)", "Biegewiderstand M_Rd (W_el = b·t²/6)") }, q_moment(m), q_moment(m_rd),
        bend_text, m / m_rd.max(1e-12), thick(m / m_rd.max(1e-12)), "mYK");
    let (v_rd, shear_text) = rib.map_or_else(|| { let v_rd = b * t * fo / (3.0f64.sqrt() * g1); (v_rd, format!("A_v = {:.1} mm², V_Rd = {:.1} kN", b * t * 1e6, v_rd / 1e3)) }, |r| {
        let v_rd = thin::rib_sheet_shear(&r, b, t, fo, thin::sheet_web_shear_constants(), g1);
        (v_rd, format!("two webs of s_w = {:.1} mm (φ = {:.1}°), h = {:.1} mm: V_Rd = 2·h·t·ρ_v·f_o/(√3·γ_M1) = {:.1} kN per b = {:.0} mm", r.web_length() * 1e3, r.web_angle().to_degrees(), r.height * 1e3, v_rd / 1e3, b * 1e3))
    });
    mk("en1999.1-4.shear", "6.1.5", if rib.is_some() { ("Web shear resistance V_Rd of the profiled sheet", "Stegquerkraftwiderstand V_Rd des Profilblechs") } else { ("Shear resistance V_Rd (A_v = b·t)", "Querkraftwiderstand V_Rd (A_v = b·t)") }, q_force(v), q_force(v_rd),
        shear_text, v / v_rd.max(1e-12), thick(v / v_rd.max(1e-12)), "vZK");
    mk("en1999.1-4.nm", "6.1.4", ("Combined axial force and bending", "Interaktion Normalkraft und Biegung"), q_dim(u_nm), q_dim(1.0),
        format!("N/N_Rd + M/M_Rd = {u_nm:.3}"), u_nm, thick(u_nm), "nK");
    let w_k: f64 = sheet.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1999_model::ActionKind::Fire).map(|a| a.g_k_line + a.q_k_line).sum::<f64>().abs();
    let delta = 5.0 * w_k * sheet.span.powi(4) / (384.0 * na_de::E_PA * inertia.max(1e-30));
    let delta_lim = sheet.span / params.deflection_ratio.max(1.0);
    let u_defl = delta / delta_lim.max(1e-12);
    let defl_remedies = if u_defl > 1.0 { vec![Remedy::at_least(subject("thickness"), q_length(t), q_length(t * u_defl.cbrt() * 1.05), loc("Increase the sheet thickness for the deflection limit.", "Blechdicke für die Durchbiegungsgrenze erhöhen.")), Remedy::at_most(subject("span"), q_length(sheet.span), q_length(sheet.span / u_defl.powf(0.25) * 0.99), loc("Shorten the span for the deflection limit.", "Spannweite für die Durchbiegungsgrenze verkürzen."))] } else { Vec::new() };
    mk("en1999.1-4.defl", "7.2", (if rib.is_some() { "SLS deflection δ = 5·w·L⁴/(384·E·I_eff) of the effective rib" } else { "SLS deflection δ = 5·w·L⁴/(384·E·I) of the flat strip" }, if rib.is_some() { "GZG Durchbiegung δ = 5·w·L⁴/(384·E·I_eff) der wirksamen Rippe" } else { "GZG Durchbiegung δ = 5·w·L⁴/(384·E·I) des ebenen Streifens" }), q_length(delta), q_length(delta_lim),
        format!("w_k = {:.2} kN/m, I = {:.1} mm⁴, δ = {:.2} mm against L/{:.0} = {:.2} mm", w_k / 1e3, inertia * 1e12, delta * 1e3, params.deflection_ratio, delta_lim * 1e3), u_defl, defl_remedies, "gKLine");
    let mut local = CheckResult::assess(format!("en1999.1-4.local.{}", sheet.id), part, cl("5.5"), subject("width"), loc("Local buckling of the flat element", "Lokales Beulen des ebenen Elements"))
        .utilization(q_dim(beta_eps), q_dim(limits[2]))
        .annex(annex)
        .explanation(loc(&format!("β/ε = {beta_eps:.2} against the class 3 limit {:.1}; ρ_c = {rho:.4} {tag_en}", limits[2]), &format!("β/ε = {beta_eps:.2} gegenüber der Klasse-3-Grenze {:.1}; ρ_c = {rho:.4} {tag_de}", limits[2])));
    if beta_eps > limits[2] {
        let t_class3 = t * beta_eps / limits[2];
        local = local.status(semio_s_plugin_norm_compliance::CheckStatus::Warning).remedy(Remedy::at_least(subject("thickness"), q_length(t), q_length(t_class3), loc(&format!("Increase the sheet thickness from {:.2} mm to at least {:.2} mm (or narrow the flat element) to reach class 3.", t * 1e3, t_class3 * 1e3), &format!("Blechdicke von {:.2} mm auf mindestens {:.2} mm erhöhen (oder das ebene Element verkleinern), um Klasse 3 zu erreichen.", t * 1e3, t_class3 * 1e3))));
    }
    out.push(local.ndps(&[na_de::AnnexParams::GAMMA_M1]).ndp_reads().build());
    out
}

pub fn check_cold_formed(sheet: &ColdFormedSheet, material: &AluminiumMaterial, profile: Option<&semio_s_artifact_norm_en1999_model::SheetProfile>, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy_for_thickness(&material.designation, sheet.thickness) else {
        return vec![alloy_unknown(format!("en1999.3.2.alloy.{}", sheet.id), &sheet.id, material, annex)];
    };
    let rib = profile.map(|p| thin::Rib { pitch: p.pitch, top: p.top_flange, bottom: p.bottom_flange, height: p.web_height }).filter(thin::Rib::is_valid);
    let proxy = part_en1990::sheet_proxy(sheet);
    let sets: Vec<Vec<CheckResult>> = part_en1990::uls_combinations(&proxy, &params, reliability_class).iter().map(|g| sheet_checks(sheet, rib, annex, &params, &alloy, g)).collect();
    worst_by_id(sets)
}

fn shell_checks(shell: &AluminiumShell, rings: &[f64], annex: AnnexChoice, params: &na_de::AnnexParams, alloy: &part_1_1::AlloyProps, g: &GoverningEffects) -> Vec<CheckResult> {
    let g1 = params.gamma_m1;
    let fo = if shell.welded { alloy.f_o_pa * alloy.rho_o_haz } else { alloy.f_o_pa };
    let q = thin::shell_quality_q(shell.fabrication_class);
    let bays = thin::ring_bays(shell.length, rings);
    let (r, t, l) = (shell.radius, shell.thickness, bays.iter().copied().fold(0.0, f64::max));
    let (sx, st, tau) = (g.n_ed.abs(), g.m_y_ed.hypot(g.m_z_ed), g.v_y_ed.hypot(g.v_z_ed));
    let alpha_x = thin::alpha_x(r, t, q);
    let s_x_cr = thin::sigma_x_rcr(r, t, l);
    let s_t_cr = thin::sigma_theta_rcr(r, t, l);
    let tau_cr = thin::tau_rcr(r, t, l);
    let lam_x = (fo / s_x_cr).sqrt();
    let lam_t = (fo / s_t_cr).sqrt();
    let lam_tau = (fo / (3.0f64.sqrt() * tau_cr)).sqrt();
    let chi_x = thin::shell_chi_axial(lam_x, alpha_x);
    let chi_t = thin::shell_chi_hoop_shear(lam_t);
    let chi_tau = thin::shell_chi_hoop_shear(lam_tau);
    let r_x = chi_x * fo / g1;
    let r_t = chi_t * fo / g1;
    let r_tau = chi_tau * fo / (3.0f64.sqrt() * g1);
    let u_int = thin::shell_interaction(sx, st, tau, r_x, r_t, r_tau, chi_x, chi_t, chi_tau);
    let label = loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id));
    let tag_en = format!("(ULS {}, lead {}); {}", g.combination, g.action_id, actions_digest(&shell.actions));
    let tag_de = format!("(GZT {}, führend {}); {}", g.combination, g.action_id, actions_digest(&shell.actions));
    let cl = ClauseId::new("EN 1999-1-5", "§5.3", "5.3");
    let mut out = Vec::new();
    let mut mk = |suffix: &str, title: (&str, &str), computed: Quantity, limit: Quantity, expl: String, u: f64, lead: &str| {
        let mut rem = Vec::new();
        if u > 1.0 {
            rem.push(Remedy::at_least(SubjectRef::new(&shell.id, format!("shells[id={}].thickness", shell.id), label.clone()), q_length(t), q_length(t * (u * 1.05).powf(0.8).max(1.01 * 1.0)), loc("Increase the shell thickness.", "Schalendicke erhöhen.")));
        }
        rem.extend(leaf_remedies("shells", &shell.id, &shell.actions, u, &label, "Reduce the shell stress", "Schalenspannung senken", lead));
        out.push(utilization_check(format!("{suffix}.{}", shell.id), "DIN EN 1999-1-5", cl.clone(), SubjectRef::new(&shell.id, format!("shells[id={}].thickness", shell.id), label.clone()), loc(title.0, title.1), computed, limit, annex,
            loc(&format!("{expl} {tag_en}"), &format!("{expl} {tag_de}")), rem));
    };
    mk("en1999.1-5.buckle", ("Axial shell buckling σ_x,Rd", "Axiales Schalenbeulen σ_x,Rd"), q_stress(sx), q_stress(r_x),
        format!("σ_x,Rcr = {:.1} MPa, λ̄_x = {lam_x:.3}, α_x = {alpha_x:.3} (fabrication class {:?}, Q = {q:.0}), f_o = {:.0} MPa, χ_x = {chi_x:.3}, σ_x,Rd = {:.1} MPa", s_x_cr / 1e6, shell.fabrication_class, fo / 1e6, r_x / 1e6), sx / r_x.max(1e-9), "nK");
    mk("en1999.1-5.ring", ("Circumferential shell buckling σ_θ,Rd", "Umfangsbeulen der Schale σ_θ,Rd"), q_stress(st), q_stress(r_t),
        format!("σ_θ,Rcr = {:.1} MPa, λ̄_θ = {lam_t:.3}, χ_θ = {chi_t:.3}, σ_θ,Rd = {:.1} MPa", s_t_cr / 1e6, r_t / 1e6), st / r_t.max(1e-9), "mYK");
    mk("en1999.1-5.shear", ("Shear shell buckling τ_Rd", "Schubbeulen der Schale τ_Rd"), q_stress(tau), q_stress(r_tau),
        format!("τ_Rcr = {:.1} MPa, λ̄_τ = {lam_tau:.3}, χ_τ = {chi_tau:.3}, τ_Rd = {:.1} MPa", tau_cr / 1e6, r_tau / 1e6), tau / r_tau.max(1e-9), "vZK");
    let omega = thin::omega(l, r, t);
    let (rings_en, rings_de) = if bays.len() > 1 { (format!("The longest of {} bays between {} ring stiffeners ({:.3} m of {:.3} m) governs; the ring section is checked by `stiffener` and its required stiffness is an unverified registry row. Thickness steps are not modelled.", bays.len(), bays.len() - 1, l, shell.length), format!("Maßgebend ist das längste von {} Feldern zwischen {} Ringsteifen ({:.3} m von {:.3} m); der Ringquerschnitt wird mit `stiffener` geprüft, die erforderliche Steifigkeit ist eine ungeprüfte Registerzeile. Dickensprünge sind nicht modelliert.", bays.len(), bays.len() - 1, l, shell.length)) } else { ("No ring stiffeners or thickness steps are modelled.".to_owned(), "Ringsteifen und Dickensprünge sind nicht modelliert.".to_owned()) };
    let (regime_en, regime_de, short, long) = if omega < 1.7 { ("short cylinder (ω < 1.7: the end-restraint branch of C_x applies)", "kurzer Zylinder (ω < 1,7: Zweig der Endeinspannung von C_x)", true, false) } else if omega > 0.5 * r / t { ("long cylinder (ω > 0.5·r/t: C_x is reduced towards 0.6)", "langer Zylinder (ω > 0,5·r/t: C_x sinkt auf 0,6)", false, true) } else { ("medium-length cylinder (C_x = 1)", "Zylinder mittlerer Länge (C_x = 1)", false, false) };
    let mut regime = CheckResult::assess(format!("en1999.1-5.regime.{}", shell.id), "DIN EN 1999-1-5", ClauseId::new("EN 1999-1-5", "§5.2", "5.2"), SubjectRef::new(&shell.id, format!("shells[id={}].length", shell.id), label.clone()), loc("Length regime of the shell and its geometry", "Längenbereich der Schale und ihre Geometrie"))
        .annex(annex)
        .explanation(loc(&format!("ω = L/√(r·t) = {omega:.2}, r/t = {:.0}: {regime_en}; the HAZ factor ρ_o,haz = {:.2} {} the axial and circumferential resistances ({}). {rings_en} {tag_en}", r / t, alloy.rho_o_haz, if shell.welded { "reduces" } else { "does not apply to" }, if shell.welded { "welded shell" } else { "unwelded shell" }), &format!("ω = L/√(r·t) = {omega:.2}, r/t = {:.0}: {regime_de}; der WEZ-Faktor ρ_o,haz = {:.2} {} die axialen und Umfangswiderstände ({}). {rings_de} {tag_de}", r / t, alloy.rho_o_haz, if shell.welded { "mindert" } else { "gilt nicht für" }, if shell.welded { "geschweißte Schale" } else { "ungeschweißte Schale" })));
    if short || long {
        let target = if short { 1.7 * (r * t).sqrt() * 1.02 } else { 0.5 * r / t * (r * t).sqrt() * 0.98 };
        regime = regime.status(semio_s_plugin_norm_compliance::CheckStatus::Warning).remedy(if short { Remedy::at_least(SubjectRef::new(&shell.id, format!("shells[id={}].length", shell.id), label.clone()), q_length(l), q_length(target), loc("Lengthen the shell into the medium range.", "Schale in den mittleren Längenbereich verlängern.")) } else { Remedy::at_most(SubjectRef::new(&shell.id, format!("shells[id={}].length", shell.id), label.clone()), q_length(l), q_length(target), loc("Shorten the shell into the medium range.", "Schale in den mittleren Längenbereich verkürzen.")) });
    }
    let regime_check = regime.build();
    mk("en1999.1-5.int", ("Interaction of the membrane stresses", "Interaktion der Membranspannungen"), q_dim(u_int), q_dim(1.0),
        format!("(σ_x/σ_x,Rd)^k_x − k_i·… + (σ_θ/σ_θ,Rd)^k_θ + (τ/τ_Rd)^k_τ = {u_int:.3}"), u_int, "nK");
    out.push(regime_check);
    out
}

pub fn check_shell(shell: &AluminiumShell, material: &AluminiumMaterial, rings: &[f64], annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy_for_thickness(&material.designation, shell.thickness) else {
        return vec![alloy_unknown(format!("en1999.3.2.alloy.{}", shell.id), &shell.id, material, annex)];
    };
    let proxy = part_en1990::shell_proxy(shell);
    let sets: Vec<Vec<CheckResult>> = part_en1990::uls_combinations(&proxy, &params, reliability_class).iter().map(|g| shell_checks(shell, rings, annex, &params, &alloy, g)).collect();
    worst_by_id(sets)
}
//#endregion 🔖️ComplianceHelpers
