//! 🧪️ Materials of the EN 1996 catalogue: M-01 … M-07 unit, mortar and masonry properties. See [`super`].

use super::*;

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "M-01", part: "EN 1996-1-1", clause: "3.1", en: "Unit classification and normalised strength", de: "Steinklassifizierung und normierte Druckfestigkeit", inputs: &["fb_decl_pa", "unit_h_m", "unit_w_m", "hole_pct"], eval: m01 },
    Entry { id: "M-02", part: "EN 1996-1-1", clause: "3.6.1.2", en: "Characteristic compressive strength f_k", de: "Charakteristische Druckfestigkeit f_k", inputs: &["fb_pa", "fm_pa", "material", "group", "mortar", "class", "gamma_m"], eval: m02 },
    Entry { id: "M-03", part: "EN 1996-1-1", clause: "3.6.2", en: "Characteristic shear strength f_vk", de: "Charakteristische Schubfestigkeit f_vk", inputs: &["fb_pa", "fm_pa", "sigma_pa", "group", "mortar", "head_joint"], eval: m03 },
    Entry { id: "M-04", part: "EN 1996-1-1", clause: "3.6.3", en: "Flexural strength f_xk1, f_xk2", de: "Biegezugfestigkeit f_xk1, f_xk2", inputs: &["fm_pa", "material", "sigma_pa", "gamma_m"], eval: m04 },
    Entry { id: "M-05", part: "EN 1996-1-1", clause: "3.6.4", en: "Anchorage of reinforcement", de: "Verankerung der Bewehrung", inputs: &["diameter_m", "fyd_pa", "fbok_pa", "lb_avail_m", "gamma_m"], eval: m05 },
    Entry { id: "M-06", part: "EN 1996-1-1", clause: "3.7", en: "Elastic modulus, creep, shrinkage", de: "Elastizitätsmodul, Kriechen, Schwinden", inputs: &["fk_pa", "material"], eval: m06 },
    Entry { id: "M-07", part: "EN 1996-1-1", clause: "2.4.3", en: "Partial factors of masonry", de: "Teilsicherheitsbeiwerte Mauerwerk", inputs: &["class", "situation"], eval: m07 },
];

fn pa(v: f64) -> String {
    format!("{:.3} MPa", v / 1e6)
}

fn m01(c: &Case) -> R {
    let fb_decl = c.n("fb_decl_pa")?;
    let delta = shape_factor_delta(c.n("unit_h_m")?, c.n("unit_w_m")?);
    let mut out = vec![
        Outcome::info(QuantityKind::Dimensionless, "delta", "shape factor δ", "Formfaktor δ", delta, format!("δ(h={:.3} m, w={:.3} m)={delta:.3} (EN 772-1 Table A.1, bilinear)", c.n("unit_h_m")?, c.n("unit_w_m")?)).basis(&["ndp.en1996.m01.delta"]),
        Outcome::info(QuantityKind::Stress, "fb", "normalised strength f_b = δ f_b,decl", "normierte Druckfestigkeit f_b = δ f_b,decl", delta * fb_decl, format!("f_b={}", pa(delta * fb_decl))).basis(&["ndp.en1996.m01.delta"]),
    ];
    if let Some(h) = c.opt("hole_pct") {
        let g = if h <= 25.0 { 1.0 } else if h <= 55.0 { 2.0 } else { 3.0 + if c.o("web_shell_ok", 0.0) > 0.0 { 1.0 } else { 0.0 } };
        out.push(Outcome::info(QuantityKind::Dimensionless, "group", "unit group", "Steingruppe", g, format!("net hole volume {h:.0} % ⇒ group {g:.0}")));
    }
    Ok(out)
}

fn m02(c: &Case) -> R {
    let (fb, fm) = (c.n("fb_pa")?, c.n("fm_pa")?);
    let (material, group, mortar) = (c.material()?, c.group(), c.mortar()?);
    let f = fk_factors(c.annex, material, group, mortar);
    let fk = f_k_mpa_from(c.annex, material, group, mortar, fb / 1e6, fm / 1e6) * 1e6;
    let basis = fk_basis(c.annex, material, group, mortar);
    let fd = fk / c.gamma_m();
    let fm_cap = if matches!(group, UnitGroup::Group1) { 20e6 } else { 20e6_f64.min(2.0 * fb) };
    Ok(vec![
        Outcome::info(QuantityKind::Stress, "fk", "f_k = K f_b^α f_m^β", "f_k = K f_b^α f_m^β", fk, format!("K={:.2}, α={:.2}, β={:.2} ⇒ f_k={}", f.k, f.alpha, f.beta, pa(fk))).basis(&basis),
        Outcome::info(QuantityKind::Stress, "fd", "design strength f_d = f_k/γ_M", "Bemessungswert f_d = f_k/γ_M", fd, format!("γ_M={:.2} ⇒ f_d={}", c.gamma_m(), pa(fd))).basis(&basis),
        Outcome::warn(QuantityKind::Stress, "fk-cap", "f_k ≤ 0.8 f_b", "f_k ≤ 0.8 f_b", f.k * (fb / 1e6).powf(f.alpha) * (fm.min(fm_cap) / 1e6).powf(f.beta) * 1e6, 0.8 * fb, "uncapped f_k against 0.8 f_b".into()).basis(&["ndp.en1996.m02.caps"]),
        Outcome::warn(QuantityKind::Stress, "fm-cap", "f_m ≤ min(20, 2 f_b) MPa", "f_m ≤ min(20, 2 f_b) MPa", fm, fm_cap.max(1.0), format!("mortar strength {} against the cap {}", pa(fm), pa(fm_cap))).basis(&["ndp.en1996.m02.caps"]),
    ])
}

fn m03(c: &Case) -> R {
    let (fb, sigma) = (c.n("fb_pa")?, c.o("sigma_pa", 0.0));
    let class = c.n("fm_pa")?;
    let class_mpa = class / 1e6;
    let mortar_class = [MortarClass::M1, MortarClass::M2_5, MortarClass::M5, MortarClass::M10, MortarClass::M15, MortarClass::M20].into_iter().rev().find(|m| m.f_m_mpa() <= class_mpa + 1e-9).unwrap_or(MortarClass::M1);
    let fvk0 = f_vk0_mpa(c.annex, c.material()?, c.mortar()?, mortar_class) * 1e6;
    let unfilled = c.so("head_joint", "filled") == "unfilled";
    let base = if unfilled { 0.5 * fvk0 } else { fvk0 };
    let raw = base + 0.4 * sigma.max(0.0);
    let cap = f_vlt_mpa(c.annex, c.group(), fb / 1e6, sigma / 1e6) * 1e6;
    let fvk = raw.min(cap);
    let mut basis = vec!["m03.fvk0"];
    if c.de() {
        basis.extend(["m03.fvlt-de", "fvlt.de-k", match c.group() {
            UnitGroup::Group1 => "fvlt.de-share.g1",
            UnitGroup::Group2 => "fvlt.de-share.g2",
            UnitGroup::Group3 | UnitGroup::Group4 => "fvlt.de-share.g34",
        }]);
    } else {
        basis.extend(["m03.fvk-cap-en", "fvlt.f-b-factor"]);
    }
    if unfilled {
        basis.push("m03.head-joint");
    }
    Ok(vec![
        Outcome::info(QuantityKind::Stress, "fvk0", "initial shear strength f_vk0", "Haftscherfestigkeit f_vk0", fvk0, format!("f_vk0={} for {mortar_class:?}", pa(fvk0))).basis(&basis),
        Outcome::info(QuantityKind::Stress, "fvk", "f_vk = min(f_vk0 + 0.4 σ_d, f_vlt)", "f_vk = min(f_vk0 + 0.4 σ_d, f_vlt)", fvk, format!("raw {} cap {} ⇒ f_vk={}", pa(raw), pa(cap), pa(fvk))).basis(&basis),
        Outcome::info(QuantityKind::Dimensionless, "capped", "limit f_vlt governs", "Grenze f_vlt maßgebend", if raw > cap { 1.0 } else { 0.0 }, "1 when the cap governs".into()),
    ])
}

fn m04(c: &Case) -> R {
    let (fm, gamma) = (c.n("fm_pa")?, c.gamma_m());
    let material = c.material()?;
    let (x1, x2) = (f_xk_mpa(1, fm / 1e6, material) * 1e6, f_xk_mpa(2, fm / 1e6, material) * 1e6);
    let sigma = c.o("sigma_pa", 0.0);
    Ok(vec![
        Outcome::info(QuantityKind::Stress, "fxk1", "f_xk1 (failure plane parallel to bed joints)", "f_xk1 (Bruchebene parallel zur Lagerfuge)", x1, pa(x1)).basis(&["ndp.en1996.m04.fxk"]),
        Outcome::info(QuantityKind::Stress, "fxk2", "f_xk2 (failure plane perpendicular to bed joints)", "f_xk2 (Bruchebene senkrecht zur Lagerfuge)", x2, pa(x2)).basis(&["ndp.en1996.m04.fxk"]),
        Outcome::info(QuantityKind::Dimensionless, "mu", "orthogonal ratio μ = f_xk1/f_xk2", "Orthogonalitätsverhältnis μ = f_xk1/f_xk2", x1 / x2, format!("μ={:.3}", x1 / x2)).basis(&["ndp.en1996.m04.fxk"]),
        Outcome::info(QuantityKind::Stress, "fxd1-app", "apparent f_xd1 + σ_d", "scheinbares f_xd1 + σ_d", x1 / gamma + sigma, format!("f_xd1={} σ_d={}", pa(x1 / gamma), pa(sigma))).basis(&["ndp.en1996.m04.fxk"]),
    ])
}

fn m05(c: &Case) -> R {
    let (phi, fyd) = (c.n("diameter_m")?, c.n("fyd_pa")?);
    let fbd = c.o("fbok_pa", 2.0e6) / c.gamma_m();
    let lb = phi * fyd / (4.0 * fbd);
    let mut out = vec![Outcome::info(QuantityKind::Length, "lb", "anchorage length l_b = φ f_yd/(4 f_bd)", "Verankerungslänge l_b = φ f_yd/(4 f_bd)", lb, format!("f_bd={}, l_b={lb:.3} m", pa(fbd))).basis(&["ndp.en1996.m05.fbok"])];
    if let Some(avail) = c.opt("lb_avail_m") {
        out.push(Outcome::limit(QuantityKind::Length, "lb-avail", "l_b ≤ available length", "l_b ≤ vorhandene Länge", lb, avail, "required against available anchorage".into()).basis(&["ndp.en1996.m05.fbok"]).fix("lb_avail_m", QuantityKind::Length, avail, ceil_to(lb, 1e-3), true));
    }
    Ok(out)
}

fn m06(c: &Case) -> R {
    let fk = c.f_k_pa()?;
    let material = c.material()?;
    let e = k_e(c.annex) * fk;
    let phi = phi_infinity_table(material);
    let (eps, alpha_t) = shrinkage_thermal(material);
    Ok(vec![
        Outcome::info(QuantityKind::Stress, "e", "E = K_E f_k", "E = K_E f_k", e, format!("K_E={:.0} ⇒ E={}", k_e(c.annex), pa(e))).basis(&["ndp.en1996.m06.e"]),
        Outcome::info(QuantityKind::Stress, "g", "G = 0.4 E", "G = 0,4 E", 0.4 * e, pa(0.4 * e)).basis(&["ndp.en1996.m06.e"]),
        Outcome::info(QuantityKind::Dimensionless, "phi-inf", "final creep coefficient φ∞", "Endkriechzahl φ∞", phi, format!("φ∞={phi:.2}")).basis(&["ndp.en1996.m06.creep"]),
        Outcome::info(QuantityKind::Stress, "e-lt", "E_lt = E/(1+φ∞)", "E_lt = E/(1+φ∞)", e / (1.0 + phi), pa(e / (1.0 + phi))).basis(&["ndp.en1996.m06.e", "ndp.en1996.m06.creep"]),
        Outcome::info(QuantityKind::Dimensionless, "eps-cs", "shrinkage strain [mm/m]", "Schwinddehnung [mm/m]", eps, format!("ε_cs={eps:.2} mm/m")).basis(&["ndp.en1996.m06.creep"]),
        Outcome::info(QuantityKind::Dimensionless, "alpha-t", "thermal expansion α_t [1/K]", "Wärmedehnzahl α_t [1/K]", alpha_t, format!("α_t={alpha_t:.1e}")).basis(&["ndp.en1996.m06.creep"]),
    ])
}

fn m07(c: &Case) -> R {
    let accidental = matches!(c.so("situation", "persistent"), "accidental" | "transient" | "seismic");
    let gamma = c.gamma_m();
    Ok(vec![
        Outcome::info(QuantityKind::Dimensionless, "gamma-m", "γ_M of masonry", "γ_M Mauerwerk", gamma, format!("γ_M={gamma:.2} ({}{})", if c.de() { "NA-DE" } else { "EN" }, if accidental { "; reduced situation" } else { "" })).basis(&["ndp.en1996.m07.gamma"]),
        Outcome::info(QuantityKind::Dimensionless, "gamma-s", "γ_S of reinforcement", "γ_S Bewehrung", gamma_s(c.annex), format!("γ_S={}", gamma_s(c.annex))).basis(&["ndp.en1996.m07.gamma"]),
        Outcome::info(QuantityKind::Dimensionless, "gamma-m-fire", "γ_M in fire", "γ_M im Brandfall", 1.0, "γ_M,fi=1.0".into()).basis(&["ndp.en1996.m07.gamma"]),
    ])
}
