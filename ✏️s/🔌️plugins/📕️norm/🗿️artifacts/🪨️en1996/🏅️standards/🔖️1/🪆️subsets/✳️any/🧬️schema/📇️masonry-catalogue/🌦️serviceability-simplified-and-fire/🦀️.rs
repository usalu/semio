//! 🌦️ Durability, execution, EN 1996-3 simplified method, fire and seismic entries of the EN 1996 catalogue: M-60 … M-95. See [`super`].

use super::*;

pub(super) const ENTRIES: &[Entry] = &[
    Entry { id: "M-60", part: "EN 1996-2", clause: "Annex B", en: "Exposure and durability", de: "Exposition und Dauerhaftigkeit", inputs: &["exposure", "material", "fm_pa", "frost_class", "cover_m"], eval: m60 },
    Entry { id: "M-61", part: "EN 1996-1-1", clause: "7, 8.5.1", en: "Deformation and slenderness limits", de: "Verformungs- und Schlankheitsgrenzen", inputs: &["span_m", "d_m", "deflection_m", "support", "sigma_s_pa", "limit_ratio"], eval: m61 },
    Entry { id: "M-70", part: "EN 1996-2", clause: "3, 8", en: "Execution, bond, minimum dimensions and chases", de: "Ausführung, Verband, Mindestmaße und Schlitze", inputs: &["bed_joint_m", "mortar", "overlap_m", "unit_h_m", "t_m", "pier_area_m2", "chase_depth_m", "diameter_m", "chase_width_m", "chase_length_m", "orientation"], eval: m70 },
    Entry { id: "M-80", part: "EN 1996-3", clause: "4.1", en: "Applicability of the simplified method", de: "Anwendbarkeit des vereinfachten Verfahrens", inputs: &["height_total_m", "span_m", "h_m", "t_m", "qk_pa", "l_m", "storeys"], eval: m80 },
    Entry { id: "M-81", part: "EN 1996-3", clause: "4.2", en: "Simplified vertical resistance Φs", de: "Vereinfachte Drucktragfähigkeit Φs", inputs: &["t_m", "hef_m", "span_m", "fk_pa", "gamma_m", "n_ed_n", "ab_m2", "a_m2", "ned_c_n"], eval: m81 },
    Entry { id: "M-82", part: "EN 1996-3", clause: "4.4", en: "Simplified shear", de: "Vereinfachter Schubnachweis", inputs: &["lc_m", "t_m", "sigma_d_pa", "fvk0_pa", "gamma_m", "v_ed_n"], eval: m82 },
    Entry { id: "M-83", part: "EN 1996-3", clause: "5", en: "Simplified lateral loading and basement walls", de: "Vereinfachte Horizontallast und Kellerwände", inputs: &["t_m", "h_m", "w_ed_pa", "fxk1_pa", "gamma_m", "sigma_d_pa", "he_m", "rho_e_n_m3", "n_min_n", "n_max_n", "fd_pa", "basement"], eval: m83 },
    Entry { id: "M-90", part: "EN 1996-1-2", clause: "4, Annex B", en: "Tabulated fire resistance", de: "Tabellarischer Feuerwiderstand", inputs: &["t_m", "rei", "material", "group", "mortar", "mu_fi", "plaster_m"], eval: m90 },
    Entry { id: "M-91", part: "EN 1996-1-2", clause: "3.3, Annex C", en: "Reduced cross-section fire method", de: "Brandbemessung mit reduziertem Querschnitt", inputs: &["t_m", "l_m", "a_theta_m", "k_theta", "fd_pa", "phi", "n_ed_fi_n"], eval: m91 },
    Entry { id: "M-95", part: "EN 1998-1", clause: "9.3", en: "Behaviour factor of masonry", de: "Verhaltensbeiwert Mauerwerk", inputs: &["system", "q", "k_diaphragm_n_m"], eval: m95 },
];

fn m60(c: &Case) -> R {
    let exposure = match c.s("exposure")? {
        "mx1" => ExposureClass::Mx1,
        "mx2" => ExposureClass::Mx2,
        "mx3" => ExposureClass::Mx3,
        "mx4" => ExposureClass::Mx4,
        "mx5" => ExposureClass::Mx5,
        other => return Err(format!("unknown exposure {other}")),
    };
    let need = required_mortar_mpa(exposure, c.material()?);
    let have = c.n("fm_pa")? / 1e6;
    let mut out = Vec::new();
    if need.is_finite() {
        out.push(Outcome::limit(QuantityKind::Stress, "mortar", "mortar strength ≥ requirement of the exposure class", "Mörtelfestigkeit ≥ Anforderung der Expositionsklasse", need * 1e6, have * 1e6, format!("{have:.1} MPa against {need:.1} MPa")).basis(&["ndp.en1996.durability.mortar"]).fix("fm_pa", QuantityKind::Stress, have * 1e6, need * 1e6, true));
    } else {
        out.push(Outcome::limit(QuantityKind::Dimensionless, "admissible", "material admissible for the exposure class", "Material für die Expositionsklasse zulässig", 2.0, 1.0, "combination inadmissible".into()).steps_de("Kombination unzulässig").basis(&["ndp.en1996.durability.mortar"]));
    }
    if let Some(f) = c.opt("frost_class") {
        let need_f = match exposure {
            ExposureClass::Mx1 => 0.0,
            ExposureClass::Mx2 => 1.0,
            _ => 2.0,
        };
        out.push(Outcome::limit(QuantityKind::Dimensionless, "frost", "freeze/thaw class F ≥ requirement", "Frostwiderstand F ≥ Anforderung", need_f, f.max(1e-9), format!("F{f:.0} against F{need_f:.0}")).basis(&["ndp.en1996.m60.exposure"]).fix("frost_class", QuantityKind::Dimensionless, f, need_f, true));
    }
    if let Some(cover) = c.opt("cover_m") {
        let need_c = semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1996.m60.cover", &[match exposure {
            ExposureClass::Mx1 => "mx1",
            ExposureClass::Mx2 => "mx2",
            ExposureClass::Mx3 => "mx3",
            ExposureClass::Mx4 => "mx4",
            ExposureClass::Mx5 => "mx5",
        }]);
        out.push(Outcome::limit(QuantityKind::Length, "cover", "reinforcement cover ≥ requirement", "Bewehrungsüberdeckung ≥ Anforderung", need_c, cover, format!("{cover:.3} m against {need_c:.3} m")).basis(&["ndp.en1996.m60.exposure", "ndp.en1996.m60.cover.mx1", "ndp.en1996.m60.cover.mx2", "ndp.en1996.m60.cover.mx3", "ndp.en1996.m60.cover.mx4", "ndp.en1996.m60.cover.mx5"]).fix("cover_m", QuantityKind::Length, cover, need_c, true));
    }
    Ok(out)
}

fn m61(c: &Case) -> R {
    let (span, d) = (c.n("span_m")?, c.n("d_m")?);
    let mut out = Vec::new();
    let (limit, name) = match c.so("support", "simple") {
        "cantilever" => (7.0, "cantilever"),
        "continuous" => (26.0, "continuous"),
        _ => (20.0, "simply supported"),
    };
    let factor = c.opt("sigma_s_pa").map_or(1.0, |s| 310e6 / s.max(1.0));
    out.push(Outcome::limit(QuantityKind::Dimensionless, "l-over-d", &format!("l/d ≤ {limit:.0}·(310/σ_s) ({name})"), &format!("l/d ≤ {limit:.0}·(310/σ_s) ({name})"), span / d, limit * factor, format!("l/d={:.2}, limit {:.2}", span / d, limit * factor)).basis(&["ndp.en1996.m61.slender"]).fix("d_m", QuantityKind::Length, d, ceil_to(span / (limit * factor), 1e-3), true));
    if let Ok(w) = c.n("deflection_m") {
        let ratio = c.o("limit_ratio", 500.0);
        out.push(Outcome::limit(QuantityKind::Length, "lintel-deflection", &format!("lintel deflection ≤ l/{ratio:.0}"), &format!("Sturzdurchbiegung ≤ l/{ratio:.0}"), w, span / ratio, format!("{:.2} mm against {:.2} mm", w * 1e3, span / ratio * 1e3)).basis(&["ndp.en1996.m61.slender"]));
    }
    Ok(out)
}

/// 🥪 Admissible bed-joint thickness range `(min, max)` [m] of a mortar type, read from the registry rows `ndp.en1996.m70.bed-joint.<general|thin>.<min|max>` (unverified): general purpose and lightweight mortar share the general band.
pub fn bed_joint_range_m(mortar: MortarType) -> (f64, f64) {
    let band = if matches!(mortar, MortarType::ThinLayer) { "thin" } else { "general" };
    (semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1996.m70.bed-joint", &[band, "min"]), semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1996.m70.bed-joint", &[band, "max"]))
}

fn m70(c: &Case) -> R {
    let basis = ["m70.execution"];
    let mut out = Vec::new();
    if let Ok(t_joint) = c.n("bed_joint_m") {
        let (lo, hi) = bed_joint_range_m(c.mortar()?);
        out.push(Outcome::limit(QuantityKind::Dimensionless, "bed-joint", "bed-joint thickness within the admissible range", "Lagerfugendicke im zulässigen Bereich", (t_joint / hi).max(lo / t_joint), 1.0, format!("{:.4} m within [{lo}, {hi}]", t_joint)).basis(&basis).fix("bed_joint_m", QuantityKind::Length, t_joint, t_joint.clamp(lo, hi), t_joint < lo));
    }
    if let (Ok(d), Ok(t_joint)) = (c.n("diameter_m"), c.n("bed_joint_m")) {
        let need = d + 0.005;
        out.push(Outcome::limit(QuantityKind::Length, "bed-joint-bar", "bed-joint thickness ≥ bar diameter + 5 mm for bed-joint reinforcement", "Lagerfugendicke ≥ Stabdurchmesser + 5 mm bei Lagerfugenbewehrung", need, t_joint, format!("{t_joint:.4} m against {need:.4} m")).basis(&basis).fix("bed_joint_m", QuantityKind::Length, t_joint, need, true));
    }
    if let (Ok(lol), Ok(hu)) = (c.n("overlap_m"), c.n("unit_h_m")) {
        let need = (normative_limit(&["overlap", "ratio"]) * hu).max(normative_limit(&["overlap", "min"]));
        out.push(Outcome::limit(QuantityKind::Length, "overlap", &format!("overlap l_ol ≥ max({} h_u, {:.0} mm)", normative_limit(&["overlap", "ratio"]), normative_limit(&["overlap", "min"]) * 1000.0), &format!("Überbindemaß l_ol ≥ max({} h_u; {:.0} mm)", normative_limit(&["overlap", "ratio"]).to_string().replace('.', ","), normative_limit(&["overlap", "min"]) * 1000.0), need, lol, format!("{lol:.3} m against {need:.3} m")).basis(&basis).fix("overlap_m", QuantityKind::Length, lol, need, true));
    }
    if let Ok(t) = c.n("t_m") {
        let need = normative_limit_for(&["min-thickness"], c.annex);
        out.push(Outcome::limit(QuantityKind::Length, "min-thickness", "load-bearing wall thickness ≥ minimum", "tragende Wanddicke ≥ Mindestmaß", need, t, format!("{t:.3} m against {need:.3} m")).basis(&basis).fix("t_m", QuantityKind::Length, t, need, true));
        if let Some(depth) = c.opt("chase_depth_m") {
            let kind = match c.so("orientation", "vertical") {
                "vertical" => ChaseKind::Vertical,
                "horizontal" => ChaseKind::Horizontal,
                "recess" => ChaseKind::Recess,
                other => return Err(format!("unknown chase orientation {other}")),
            };
            let (width, length) = (c.opt("chase_width_m"), c.opt("chase_length_m"));
            let limits = chase_limits(kind, t, length.unwrap_or(f64::INFINITY), f64::INFINITY, 1.0, c.annex);
            let allowed = limits.map_or(0.0, |l| l.depth_m);
            let table = limits.map_or("8.1/8.2", |l| l.table);
            let verdict = if depth <= allowed + 1e-9 { "within the table" } else { "outside the table, calculation required" };
            let verdict_de = if depth <= allowed + 1e-9 { "innerhalb der Tabelle" } else { "außerhalb der Tabelle, Nachweis erforderlich" };
            out.push(Outcome::warn(QuantityKind::Length, "chase", "chase depth within the table without calculation", "Schlitztiefe ohne Nachweis zulässig", depth, allowed.max(1e-9), format!("Table {table}: d={depth:.3} m against {allowed:.3} m, t_ef={:.3} m: {verdict}", t - depth)).steps_de(format!("Tabelle {table}: d={depth:.3} m gegenüber {allowed:.3} m, t_ef={:.3} m: {verdict_de}", t - depth)).basis(&basis).fix("chase_depth_m", QuantityKind::Length, depth, allowed, false));
            if let (Some(width), Some(max_width)) = (width, limits.and_then(|l| l.width_m)) {
                out.push(Outcome::warn(QuantityKind::Length, "chase-width", "chase width within the table without calculation", "Schlitzbreite ohne Nachweis zulässig", width, max_width.max(1e-9), format!("Table 8.1: w={width:.3} m against {max_width:.3} m")).steps_de(format!("Tabelle 8.1: b={width:.3} m gegenüber {max_width:.3} m")).basis(&basis).fix("chase_width_m", QuantityKind::Length, width, max_width, false));
            }
            if let (Some(width), Some(length)) = (width, length) {
                out.push(Outcome::info(QuantityKind::Area, "chase-area", "removed masonry area of the chase or recess", "entfernte Mauerwerksfläche des Schlitzes oder der Aussparung", width * length, format!("{kind:?}: {width:.3} m × {length:.3} m × {depth:.3} m deep; distance rules need the document chases (unverified)")).basis(&basis));
            }
        }
    }
    if let Some(area) = c.opt("pier_area_m2") {
        let need = normative_limit_for(&["pier-area"], c.annex);
        out.push(Outcome::limit(QuantityKind::Area, "pier-area", "pier area ≥ minimum", "Pfeilerfläche ≥ Mindestmaß", need, area, format!("{area:.3} m² against {need:.2} m²")).basis(&basis).fix("pier_area_m2", QuantityKind::Area, area, need, true));
    }
    if out.is_empty() {
        return Err("no execution input given".into());
    }
    Ok(out)
}

fn m80(c: &Case) -> R {
    let basis = ["m80.limits"];
    let (t, h) = (c.n("t_m")?, c.n("h_m")?);
    let storeys = c.o("storeys", 1.0);
    let limit = |key: &str| normative_limit(&["simplified", key]);
    let h_max = if t >= limit("thick-wall") { limit("height-ratio") * t } else { limit("height-thin") };
    let t_min = if storeys > limit("storey-threshold") { limit("min-thickness-tall") } else { limit("min-thickness") };
    let mut out = vec![
        Outcome::limit(QuantityKind::Length, "height-limit", "clear storey height within the limit", "lichte Geschosshöhe innerhalb der Grenze", h, h_max, format!("h={h:.2} m against {h_max:.2} m")).basis(&basis).fix("h_m", QuantityKind::Length, h, h_max, false),
        Outcome::limit(QuantityKind::Length, "thickness-limit", "thickness ≥ minimum", "Dicke ≥ Mindestmaß", t_min, t, format!("t={t:.3} m against {t_min:.3} m")).basis(&basis).fix("t_m", QuantityKind::Length, t, t_min, true),
    ];
    if let Ok(ht) = c.n("height_total_m") {
        out.push(Outcome::limit(QuantityKind::Length, "building-height", &format!("building height ≤ {:.0} m", limit("building-height")), &format!("Gebäudehöhe ≤ {:.0} m", limit("building-height")), ht, limit("building-height"), format!("H={ht:.1} m")).basis(&basis));
    }
    if let Ok(s) = c.n("span_m") {
        out.push(Outcome::limit(QuantityKind::Length, "span", &format!("floor span ≤ {:.0} m", limit("floor-span")), &format!("Deckenstützweite ≤ {:.0} m", limit("floor-span")), s, limit("floor-span"), format!("l_f={s:.2} m")).basis(&basis));
    }
    if let Ok(q) = c.n("qk_pa") {
        out.push(Outcome::limit(QuantityKind::Stress, "imposed", &format!("imposed load ≤ {:.0} kN/m²", limit("imposed-load") / 1000.0), &format!("Nutzlast ≤ {:.0} kN/m²", limit("imposed-load") / 1000.0), q, limit("imposed-load"), format!("q_k={q:.0} Pa")).basis(&basis));
    }
    if let Ok(l) = c.n("l_m") {
        out.push(Outcome::limit(QuantityKind::Length, "wall-length", &format!("wall length ≥ {} m and area ≥ {} m²", limit("min-length"), limit("min-area")), &format!("Wandlänge ≥ {} m und Fläche ≥ {} m²", limit("min-length").to_string().replace('.', ","), limit("min-area").to_string().replace('.', ",")), limit("min-length").max(limit("min-area") / t), l, format!("l={l:.2} m")).basis(&basis).fix("l_m", QuantityKind::Length, l, limit("min-length").max(limit("min-area") / t), true));
    }
    Ok(out)
}

fn m81(c: &Case) -> R {
    let (t, hef) = (c.n("t_m")?, c.n("hef_m")?);
    let lambda = hef / t;
    let phi_1 = phi_s(lambda);
    let phi_2 = c.opt("span_m").map_or(1.0, |s| phi_2_slab_span(c.annex, s));
    let phi = phi_1.min(phi_2);
    let fd = c.f_d_pa()?;
    let n_rd = phi * t * fd;
    let n_ed = c.n("n_ed_n")?;
    let mut out = vec![
        Outcome::limit(QuantityKind::Dimensionless, "slenderness", "h_ef/t ≤ 21", "h_ef/t ≤ 21", lambda, 21.0, format!("λ={lambda:.3}")).basis(&["ndp.en1996.m81.phis"]).fix("t_m", QuantityKind::Length, t, ceil_to(hef / 21.0, 1e-3), true),
        Outcome::info(QuantityKind::Dimensionless, "phi-s", "Φ_s = min(Φ₁, Φ₂)", "Φ_s = min(Φ₁, Φ₂)", phi, format!("Φ₁=0.85−0.0011 λ²={phi_1:.4}, Φ₂={phi_2:.4}")).basis(&["ndp.en1996.m81.phis", "ndp.en1996.m13.phi1"]),
        Outcome::limit(QuantityKind::Force, "n-rd", "N_Ed ≤ Φ_s t f_d", "N_Ed ≤ Φ_s t f_d", n_ed, n_rd.max(1e-9), format!("N_Rd={n_rd:.0} N/m")).basis(&["ndp.en1996.m81.phis"]).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| n_ed <= phi_s(hef / x).min(phi_2) * x * fd).unwrap_or(3.0), true),
    ];
    if let (Ok(ab), Ok(a), Ok(f)) = (c.n("ab_m2"), c.n("a_m2"), c.n("ned_c_n")) {
        let cap = 1.5 * fd * ab;
        out.push(Outcome::limit(QuantityKind::Force, "concentrated", "N_Edc ≤ 1.5 f_d A_b with A_b ≤ 0.2 A", "N_Edc ≤ 1,5 f_d A_b mit A_b ≤ 0,2 A", f, cap.max(1e-9), format!("A_b/A={:.3}", ab / a)).basis(&["ndp.en1996.m81.phis"]).fix("ab_m2", QuantityKind::Area, ab, ceil_to(f / (1.5 * fd), 1e-4), true));
        out.push(Outcome::limit(QuantityKind::Area, "bearing-ratio", "A_b ≤ 0.2 A", "A_b ≤ 0,2 A", ab, 0.2 * a, format!("A_b={ab:.4} m²")).basis(&["ndp.en1996.m81.phis"]).fix("ab_m2", QuantityKind::Area, ab, 0.2 * a, false));
    }
    Ok(out)
}

fn m82(c: &Case) -> R {
    let (lc, t, v) = (c.n("lc_m")?, c.n("t_m")?, c.n("v_ed_n")?);
    let sigma = c.o("sigma_d_pa", 0.0);
    let fvk = c.n("fvk0_pa")? + 0.4 * sigma;
    let v_rd = fvk / c.gamma_m() * t * lc;
    Ok(vec![
        Outcome::limit(QuantityKind::Length, "lc-min", "l_c ≥ 0.3 m", "l_c ≥ 0,3 m", 0.3, lc, format!("l_c={lc:.3} m")).basis(&["ndp.en1996.m82.shear"]).fix("lc_m", QuantityKind::Length, lc, 0.3, true),
        Outcome::limit(QuantityKind::Force, "v-rd", "V_Ed ≤ f_vd t l_c", "V_Ed ≤ f_vd t l_c", v, v_rd.max(1e-9), format!("f_vk={:.3} MPa, V_Rd={v_rd:.0} N", fvk / 1e6)).basis(&["ndp.en1996.m82.shear"]).fix("t_m", QuantityKind::Length, t, ceil_to(v / (fvk / c.gamma_m() * lc), 1e-3), true),
    ])
}

fn m83(c: &Case) -> R {
    let (t, h, w) = (c.n("t_m")?, c.n("h_m")?, c.n("w_ed_pa")?);
    let gamma = c.gamma_m();
    let fxd1 = c.n("fxk1_pa")? / gamma + c.o("sigma_d_pa", 0.0);
    let m_ed = w * h * h / 8.0;
    let mut out = vec![Outcome::limit(QuantityKind::Moment, "wind-panel", "wind on the panel M_Ed ≤ (f_xd1 + σ_d) Z", "Wind auf Wandfeld M_Ed ≤ (f_xd1 + σ_d) Z", m_ed, (fxd1 * t * t / 6.0).max(1e-9), format!("M_Ed={m_ed:.1} N·m/m")).fix("t_m", QuantityKind::Length, t, solve_threshold(t, 3.0, 1e-3, |x| m_ed <= fxd1 * x * x / 6.0).unwrap_or(3.0), true)];
    if c.o("basement", 0.0) > 0.0 {
        let (he, rho_e) = (c.n("he_m")?, c.n("rho_e_n_m3")?);
        let fd = c.f_d_pa()?;
        let (n_min, n_max) = (c.n("n_min_n")?, c.n("n_max_n")?);
        let req = rho_e * h * he * he / (20.0 * t);
        out.push(Outcome::limit(QuantityKind::Force, "basement-n-min", "N_min ≥ ρ_e h h_e²/(20 t)", "N_min ≥ ρ_e h h_e²/(20 t)", req, n_min.max(1e-9), format!("N_min,req={req:.0} N/m")).basis(&["ndp.en1996.m34.basement"]).fix("n_min_n", QuantityKind::Force, n_min, ceil_to(req, 1.0), true));
        out.push(Outcome::limit(QuantityKind::Force, "basement-n-max", "N_max ≤ 0.45 t f_d", "N_max ≤ 0,45 t f_d", n_max, (0.45 * t * fd).max(1e-9), "crushing under maximum normal force".into()).steps_de("Zerdrücken unter maximaler Normalkraft").basis(&["ndp.en1996.m34.basement"]).fix("t_m", QuantityKind::Length, t, ceil_to(n_max / (0.45 * fd), 1e-3), true));
    }
    Ok(out)
}

fn m90(c: &Case) -> R {
    let (t, rei) = (c.n("t_m")?, c.n("rei")? as u32);
    let material = c.material()?;
    let mu = c.o("mu_fi", 1.0);
    let plaster = c.o("plaster_m", 0.0);
    let t_eff = t + plaster;
    let t_min = fire_min_thickness_m(rei, material);
    let achieved = fire_achievable_rei(t_eff, material);
    Ok(vec![
        Outcome::limit(QuantityKind::Length, "t-min", "t ≥ t_min(REI)", "t ≥ t_min(REI)", t_min, t_eff, format!("t_min={:.0} mm for REI {rei}", t_min * 1e3)).basis(&["ndp.en1996.m90.fire"]).fix("t_m", QuantityKind::Length, t, ceil_to((t_min - plaster).max(0.0), 5e-3), true),
        Outcome::limit(QuantityKind::Dimensionless, "mu-fi", "μ_fi ≤ 1", "μ_fi ≤ 1", mu, 1.0, format!("μ_fi={mu:.3}")).basis(&["ndp.en1996.m90.fire"]).fix("mu_fi", QuantityKind::Dimensionless, mu, 1.0, false),
        Outcome::info(QuantityKind::Dimensionless, "achievable-rei", "achievable REI class", "erreichbare REI-Klasse", achieved as f64, format!("REI {achieved}")).basis(&["ndp.en1996.m90.fire"]),
    ])
}

fn m91(c: &Case) -> R {
    let (t, l, a_theta, k) = (c.n("t_m")?, c.n("l_m")?, c.n("a_theta_m")?, c.n("k_theta")?);
    let fd = c.f_d_pa()?;
    let t_red = (t - 2.0 * a_theta).max(0.0);
    let n_rd = c.o("phi", 1.0) * k * fd * t_red * l;
    let n_ed = c.n("n_ed_fi_n")?;
    Ok(vec![
        Outcome::info(QuantityKind::Length, "t-red", "reduced thickness t − 2 a_θ", "reduzierte Dicke t − 2 a_θ", t_red, format!("t_red={t_red:.4} m")).basis(&["ndp.en1996.m91.fire"]),
        Outcome::limit(QuantityKind::Force, "n-rd-fi", "N_Ed,fi ≤ Φ k_θ f_d t_red l", "N_Ed,fi ≤ Φ k_θ f_d t_red l", n_ed, n_rd.max(1e-9), format!("N_Rd,fi={n_rd:.0} N")).basis(&["ndp.en1996.m91.fire"]).fix("t_m", QuantityKind::Length, t, ceil_to(2.0 * a_theta + n_ed / (c.o("phi", 1.0) * k * fd * l).max(1e-9), 1e-3), true),
    ])
}

fn m95(c: &Case) -> R {
    let system = c.s("system")?;
    let (lo, hi, rows) = behaviour_factor_range(c.annex, system).ok_or_else(|| format!("unknown system {system}"))?;
    let mut out = vec![
        Outcome::info(QuantityKind::Dimensionless, "q-min", "lower q range", "untere q-Grenze", lo, format!("q ∈ [{lo}, {hi}]")).basis(&rows),
        Outcome::info(QuantityKind::Dimensionless, "q-max", "upper q range", "obere q-Grenze", hi, format!("q ∈ [{lo}, {hi}]")).basis(&rows),
    ];
    if let Some(k) = c.opt("k_diaphragm_n_m") {
        out.push(Outcome::info(QuantityKind::Dimensionless, "diaphragm-stiffness", "weakest declared floor diaphragm stiffness G t", "kleinste angegebene Schubsteifigkeit G t der Deckenscheibe", k, format!("G t = {k:.3e} N/m; the rigid or flexible classification of EN 1998-1 4.3.1(4) compares the diaphragm deflection with the wall drift and needs the walls: it is the document check en1996.9.3.diaphragm.x and .y")).basis(&["ndp.en1996.m95.diaphragm-ratio"]));
    }
    if let Some(q) = c.opt("q") {
        out.push(Outcome::limit(QuantityKind::Dimensionless, "q-used", "assumed q within the range", "angesetztes q innerhalb der Spanne", q, hi, format!("q={q}")).basis(&rows).fix("q", QuantityKind::Dimensionless, q, hi, false));
    }
    Ok(out)
}
