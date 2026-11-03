//! 🏋️ Load case checks: the per load case compression, shear, flexure and concentrated load checks of the wall evaluation. See [`super`].

use super::*;

/// 🔖️ Registry ids (`ndp.en1996.*`) the formulas of each check rest on, by data flow: tabulated cells are cited from the kernel read log ([`record_ndp_reads`] around the state computation), the strength factors from [`strength_ndps`].
const COMPRESSION: &[&str] = &["ndp.en1996.m06.creep", "ndp.en1996.m06.e", "ndp.en1996.m07.gamma", "ndp.en1996.m10.rho2", "ndp.en1996.m12.einit"];
const SLAB_NA: &[&str] = &["ndp.en1996.m13.phi1"];
const ECCENTRICITY: &[&str] = &["ndp.en1996.m12.einit", "ndp.en1996.m12.eccentricity-limit"];
const SHEAR: &[&str] = &["ndp.en1996.m03.fvk0", "ndp.en1996.m07.gamma", "ndp.en1996.m20.stability"];
const FLEXURE: &[&str] = &["ndp.en1996.m04.fxk", "ndp.en1996.m07.gamma"];
const CONCENTRATED: &[&str] = &["ndp.en1996.m07.gamma", "ndp.en1996.m14.beta"];
const BEARING_RATIO: &[&str] = &["ndp.en1996.m14.beta"];

/// 🧱️ Registry ids behind the characteristic compressive strength f_k of the wall: the K, alpha, beta and shape factor cells read, the caps and the K row of the annex, and the shape factor table row.
pub(super) fn strength_ndps(annex: AnnexChoice, wall: &MasonryWall) -> Vec<&'static str> {
    let (_, mut ids) = semio_s_plugin_norm_compliance::record_ndp_reads(|| f_k_pa(annex, wall));
    ids.extend(fk_basis(annex, wall.unit_material, wall.unit_group, wall.mortar_type));
    ids.push("ndp.en1996.m01.delta");
    ids
}

/// 🔖️ Registry id of the shear-strength cap row of the annex.
fn shear_cap_ndp(annex: AnnexChoice) -> &'static str {
    match annex {
        AnnexChoice::En => "ndp.en1996.m03.fvk-cap-en",
        AnnexChoice::De => "ndp.en1996.m03.fvlt-de",
    }
}

/// ⚠️ Text naming the unverified NDP basis behind an NA-DE strength result.
fn unverified_note(annex: AnnexChoice, german: bool) -> &'static str {
    match (annex, german) {
        (AnnexChoice::De, false) => " Unverified NDP basis: m02.k-de (f_k from k, alpha, beta instead of the NA tables).",
        (AnnexChoice::De, true) => " Ungeprüfte NDP-Grundlage: m02.k-de (f_k aus k, alpha, beta statt NA-Tabellen).",
        _ => "",
    }
}

fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

/// 🏋 Checks every load case of the wall: compression, shear, lateral flexure and concentrated load.
pub(super) fn check_load_cases(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, out: &mut Vec<CheckResult>) {
    for lc in &wall.load_cases {
        let sit = DesignSituation::from(lc.design_situation);
        let params = AnnexParams::for_document(annex, masonry_class, if matches!(sit, DesignSituation::Persistent) { default_situation } else { sit });
        let (gamma_m, gamma_ndp) = (params.gamma_m(), params.gamma_m_ndp());
        check_wall_compression(annex, gamma_m, gamma_ndp, default_situation, reliability_class, wall, lc, out);
        check_wall_shear(annex, gamma_m, gamma_ndp, default_situation, reliability_class, wall, lc, out);
        check_wall_flexure(annex, gamma_m, gamma_ndp, default_situation, reliability_class, wall, lc, out);
        check_wall_concentrated_load(annex, gamma_m, gamma_ndp, default_situation, reliability_class, wall, lc, out);
    }
}

/// 🧱️ Compression state of the wall under one load case: demands, resistances and eccentricities per station.
struct Compression {
    ge: DesignEffects,
    station: &'static str,
    n_ed: f64,
    n_rd: f64,
    phi: f64,
    phi2: f64,
    phi_mid: f64,
    n_rd_mid: f64,
    e_top: f64,
    e_theta: f64,
    e_bot: f64,
    e_hm: f64,
    e_mk: f64,
    e_k: f64,
    e_slab: f64,
    sections: [(f64, f64); 3],
    capacity: f64,
}

impl Compression {
    fn utilization(&self) -> f64 {
        self.n_ed / self.n_rd.max(1e-9)
    }
    fn utilization_mid(&self) -> f64 {
        self.ge.n_mid / self.n_rd_mid.max(1e-9)
    }
}

fn compression(annex: AnnexChoice, gamma_m: f64, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase) -> Compression {
    let t = wall.thickness_m;
    let f_d = f_k_pa(annex, wall) / gamma_m;
    let a = area_m2(wall);
    let effects = design_effects(annex, wall, lc, default_situation, reliability_class);
    let ge = governing_compression(&effects).clone();
    let e_slab = eccentricity_from_slab_bearing_m(wall);
    let e_theta = eccentricity_from_slab_rotation_m(annex, wall, lc, ge.n_top.max(ge.n_mid));
    let phi2 = phi_2_slab_span(annex, lc.slab_span_m);
    let e_top = (wall.eccentricity_top_m + e_slab + e_theta).abs();
    let e_bot = wall.eccentricity_bottom_m.abs();
    let h_ef = h_ef_m(wall);
    let e_init = h_ef / normative_limit(&["eccentricity", "initial-divisor"]);
    let e_hm = ge.w_pa * wall.height_m.powi(2) / 8.0 * effective_length_m(wall) / ge.n_mid.max(1.0);
    let e_m = 0.5 * (e_top + e_bot) + e_init + e_hm;
    let e_k = creep_eccentricity_m(wall, e_m);
    let e_mk = (e_m + e_k).abs().max(normative_limit(&["eccentricity", "min-ratio"]) * t);
    let phi_top = phi_i(e_top + e_init, t) * phi2;
    let phi_bot = phi_i(e_bot + e_init, t) * phi2;
    let phi_mid = phi_m(slenderness(wall), e_mk, t, k_e(annex));
    let stations = [("top", ge.n_top, phi_top), ("mid", ge.n_mid, phi_mid), ("bottom", ge.n_bot, phi_bot)];
    let (station, n_ed, phi) = stations.iter().copied().max_by(|a, b| (a.1 / a.2.max(1e-9)).total_cmp(&(b.1 / b.2.max(1e-9)))).unwrap_or(stations[0]);
    Compression { n_rd: phi * f_d * a, n_rd_mid: phi_mid * f_d * a, sections: [(ge.n_bot, phi_bot), (ge.n_mid, phi_mid), (ge.n_top, phi_top)], capacity: f_d * a, ge, station, n_ed, phi, phi2, phi_mid, e_top, e_theta, e_bot, e_hm, e_mk, e_k, e_slab }
}

/// 🧱️ Checks the compression, the eccentricities and the slenderness of the wall under one load case.
fn check_wall_compression(annex: AnnexChoice, gamma_m: f64, gamma_ndp: &'static str, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase, out: &mut Vec<CheckResult>) {
    let t = wall.thickness_m;
    let path_t = wall_path(wall, "thicknessM");
    let path_fb = wall_path(wall, "fBPa");
    let path_bearing = wall_path(wall, "slabBearingDepthM");
    let path_density = wall_path(wall, "densityKgM3");
    let path_phi_inf = wall_path(wall, "phiInfinity");
    let path_span = load_case_path(wall, lc, "slabSpanM");
    let (c, reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| compression(annex, gamma_m, default_situation, reliability_class, wall, lc));
    let basis: Vec<&'static str> = [reads, strength_ndps(annex, wall), COMPRESSION.to_vec(), if annex == AnnexChoice::De { SLAB_NA.to_vec() } else { Vec::new() }, vec![gamma_ndp]].concat();
    let passes = |modify: &dyn Fn(&mut MasonryWall, &mut WallLoadCase, f64), x: f64, mid: bool| {
        let (mut w, mut l) = (wall.clone(), lc.clone());
        modify(&mut w, &mut l, x);
        let s = compression(annex, gamma_m, default_situation, reliability_class, &w, &l);
        (if mid { s.utilization_mid() } else { s.utilization() }) <= 1.0
    };
    let remedies = |mid: bool| -> Vec<Remedy> {
        let mut r = Vec::new();
        if let Some(x) = solve_threshold(t, 3.0, 1e-3, |x| passes(&|w, _, v| w.thickness_m = v, x, mid)) {
            r.push(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase thickness.", "Dicke erhöhen.")));
        }
        if !mid {
            if let Some(x) = solve_threshold(wall.f_b_pa, 1000e6, 1e5, |x| passes(&|w, _, v| w.f_b_pa = v, x, false)) {
                r.push(Remedy::at_least(wall_subject(wall, &path_fb), Quantity::new(QuantityKind::Stress, wall.f_b_pa), Quantity::new(QuantityKind::Stress, x), loc("Increase f_b.", "f_b erhöhen.")));
            }
            if let Some(x) = solve_threshold(wall.slab_bearing_depth_m, t, 1e-3, |x| passes(&|w, _, v| w.slab_bearing_depth_m = v, x, false)) {
                r.push(Remedy::at_least(wall_subject(wall, &path_bearing), Quantity::length_m(wall.slab_bearing_depth_m), Quantity::length_m(x), loc("Increase slab bearing depth to reduce eccentricity.", "Deckenauflager vertiefen, Exzentrizität reduzieren.")));
            }
            if let Some(x) = solve_threshold(lc.slab_span_m, 0.0, 1e-2, |x| passes(&|_, l, v| l.slab_span_m = v, x, false)) {
                r.push(Remedy::at_most(wall_subject(wall, &path_span), Quantity::length_m(lc.slab_span_m), Quantity::length_m(x), loc("Reduce slab span to raise Φ₂ / cut e_θ (NA 6.1.2.2).", "Deckenspannweite reduzieren (Φ₂/e_θ, NA 6.1.2.2).")));
            }
            if let Some(x) = solve_threshold(wall.density_kg_m3, 0.0, 1.0, |x| passes(&|w, _, v| w.density_kg_m3 = v, x, false)) {
                r.push(Remedy::at_most(wall_subject(wall, &path_density), Quantity::new(QuantityKind::Mass, wall.density_kg_m3), Quantity::new(QuantityKind::Mass, x), loc("Reduce masonry density (self-weight in N_Ed).", "Rohdichte senken (Eigengewicht in N_Ed).")));
            }
        }
        if let Some(x) = solve_threshold(wall.phi_infinity, 0.0, 1e-2, |x| passes(&|w, _, v| w.phi_infinity = v, x, mid)) {
            r.push(Remedy::at_most(wall_subject(wall, &path_phi_inf), dimensionless(wall.phi_infinity), dimensionless(x), loc("Reduce φ∞ to cut creep eccentricity e_k.", "φ∞ senken, Kriechexzentrizität e_k reduzieren.")));
        }
        r
    };
    let text = |station: &str, ge: &str| {
        format!(
            "{ge}; {station}; Φ={:.3} (Φ₂={:.3}); e_top={:.4}; e_θ={:.4} (ℓ_f={:.2} m); e_bot={:.4}; e_hm={:.4}; e_mk={:.4} m (e_k={:.4}, φ∞={:.2}); N_Ed={:.0} N; N_Rd={:.0} N; e_bearing={:.4} m.",
            c.phi, c.phi2, c.e_top, c.e_theta, lc.slab_span_m, c.e_bot, c.e_hm, c.e_mk, c.e_k, wall.phi_infinity, c.n_ed, c.n_rd, c.e_slab
        )
    };
    let mut b = CheckResult::assess(
        format!("en1996.6.1.2.compression.{}.{}", wall.id, lc.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§6.1.2", "6.1.2"),
        wall_subject(wall, &path_t),
        loc("Compression N_Ed ≤ Φ·Φ₂·f_d·A", "Druck N_Ed ≤ Φ·Φ₂·f_d·A"),
    )
    .utilization(Quantity::new(QuantityKind::Force, c.n_ed), Quantity::new(QuantityKind::Force, c.n_rd.max(1e-9)))
    .annex(annex)
    .explanation(loc(&format!("{}{}", text(c.station, &c.ge.combo_en), unverified_note(annex, false)), &format!("{}{}", text(c.station, &c.ge.combo_de), unverified_note(annex, true))));
    if c.utilization() > 1.0 {
        for r in remedies(false) {
            b = b.remedy(r);
        }
    }
    let sections = c.sections.iter().zip([0.0, 0.5, 1.0]).map(|(&(n, phi), s)| {
        Station::ratio_of(&[s], Quantity::new(QuantityKind::Force, n), Quantity::new(QuantityKind::Force, (phi * c.capacity).max(1e-9))).with(dimensionless(phi))
    });
    out.push(b.field("wall-phi", sections.collect()).ndps(&basis).build());

    let mut bm = CheckResult::assess(
        format!("en1996.6.1.2.compression.mid.{}.{}", wall.id, lc.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§6.1.2", "6.1.2"),
        wall_subject(wall, &path_phi_inf),
        loc("Mid-height compression N_Ed ≤ Φ_m·f_d·A", "Druck Wandmitte N_Ed ≤ Φ_m·f_d·A"),
    )
    .utilization(Quantity::new(QuantityKind::Force, c.ge.n_mid), Quantity::new(QuantityKind::Force, c.n_rd_mid.max(1e-9)))
    .annex(annex)
    .explanation(loc(
        &format!("mid; Φ_m={:.3}; e_mk={:.4} m; e_k={:.4} m (φ∞={:.2}); N_Ed={:.0} N; N_Rd={:.0} N.", c.phi_mid, c.e_mk, c.e_k, wall.phi_infinity, c.ge.n_mid, c.n_rd_mid),
        &format!("Mitte; Φ_m={:.3}; e_mk={:.4} m; e_k={:.4} m (φ∞={:.2}); N_Ed={:.0} N; N_Rd={:.0} N.", c.phi_mid, c.e_mk, c.e_k, wall.phi_infinity, c.ge.n_mid, c.n_rd_mid),
    ));
    if c.utilization_mid() > 1.0 {
        for r in remedies(true) {
            bm = bm.remedy(r);
        }
    }
    out.push(bm.ndps(&basis).build());

    let (limit_ratio, _) = semio_s_plugin_norm_compliance::record_ndp_reads(|| semio_s_plugin_norm_compliance::ndp_common("ndp.en1996.m12.eccentricity-limit"));
    let e_lim = limit_ratio * t;
    let e_decl = wall.eccentricity_bottom_m.abs().max(wall.eccentricity_top_m.abs());
    let mut be = CheckResult::assess(
        format!("en1996.6.1.2.eccentricity.{}.{}", wall.id, lc.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§6.1.2.2", "6.1.2.2"),
        wall_subject(wall, &wall_path(wall, "eccentricityBottomM")),
        loc(&format!("Declared eccentricity e ≤ {limit_ratio} t"), &format!("Angegebene Exzentrizität e ≤ {} t", format!("{limit_ratio}").replace('.', ","))),
    )
    .utilization(Quantity::length_m(e_decl), Quantity::length_m(e_lim.max(1e-9)))
    .annex(annex)
    .explanation(loc(
        &format!("e_top,decl={:.4} m; e_bot={:.4} m; limit {limit_ratio}t={e_lim:.4} m.", wall.eccentricity_top_m.abs(), wall.eccentricity_bottom_m.abs()),
        &format!("e_oben,ang={:.4} m; e_unten={:.4} m; Grenze {limit_ratio}t={e_lim:.4} m.", wall.eccentricity_top_m.abs(), wall.eccentricity_bottom_m.abs()),
    ));
    if e_decl > e_lim {
        be = be
            .remedy(Remedy::at_most(wall_subject(wall, &wall_path(wall, "eccentricityBottomM")), Quantity::length_m(wall.eccentricity_bottom_m.abs()), Quantity::length_m(e_lim), loc("Reduce bottom eccentricity.", "Exzentrizität unten reduzieren.")))
            .remedy(Remedy::at_most(wall_subject(wall, &wall_path(wall, "eccentricityTopM")), Quantity::length_m(wall.eccentricity_top_m.abs()), Quantity::length_m(e_lim), loc("Reduce top eccentricity.", "Exzentrizität oben reduzieren.")));
    }
    out.push(be.ndps(ECCENTRICITY).build());
}

/// 🪵️ Shear state: governing combination, demand and the masonry and friction resistances [N].
struct Shear {
    ge: DesignEffects,
    v_ed: f64,
    v_masonry: f64,
    v_slide: f64,
    f_vk: f64,
}

impl Shear {
    fn v_rd(&self) -> f64 {
        self.v_masonry.min(self.v_slide)
    }
}

fn shear(annex: AnnexChoice, gamma_m: f64, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase) -> Shear {
    let effects = design_effects(annex, wall, lc, default_situation, reliability_class);
    let mut ge = governing_shear(&effects).clone();
    let l = effective_length_m(wall);
    let v_in_plane = wall.in_plane_shear_n.abs();
    ge.v += v_in_plane;
    let e = ((ge.v * wall.height_m) + wall.in_plane_moment_nm.abs()) / ge.n_mid.max(1.0);
    let l_c = (3.0 * (l / 2.0 - e)).clamp(1e-3, l);
    let f_vk = f_vk_pa(annex, wall, ge.n_mid / (l_c * wall.thickness_m).max(1e-9));
    Shear { v_ed: ge.v, v_masonry: f_vk / gamma_m * l_c * wall.thickness_m, v_slide: (wall.mu * ge.n_mid / gamma_m).max(1e-9), f_vk, ge }
}

/// 🪵️ Checks the shear resistance of the wall under one load case.
fn check_wall_shear(annex: AnnexChoice, gamma_m: f64, gamma_ndp: &'static str, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase, out: &mut Vec<CheckResult>) {
    let t = wall.thickness_m;
    let path_t = wall_path(wall, "thicknessM");
    let path_mu = wall_path(wall, "mu");
    let (s, reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| shear(annex, gamma_m, default_situation, reliability_class, wall, lc));
    let text = |combo: &str| {
        format!("{combo}; V_Ed={:.0} N; V_Rd,masonry=f_vd·t·l_c={:.0} N; V_Rd,slide=μ·N/γ_M={:.0} N (μ={:.2}); f_vk={:.3} MPa.", s.v_ed, s.v_masonry, s.v_slide, wall.mu, s.f_vk / 1e6)
    };
    let mut b = CheckResult::assess(
        format!("en1996.6.2.shear.{}.{}", wall.id, lc.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§6.2", "6.2"),
        wall_subject(wall, &path_t),
        loc("Shear / sliding V_Ed ≤ V_Rd", "Schub / Gleiten V_Ed ≤ V_Rd"),
    )
    .utilization(Quantity::new(QuantityKind::Force, s.v_ed), Quantity::new(QuantityKind::Force, s.v_rd().max(1e-9)))
    .annex(annex)
    .explanation(loc(&text(&s.ge.combo_en), &text(&s.ge.combo_de)));
    if s.v_ed > s.v_rd() {
        let ok = |w: &MasonryWall| {
            let n = shear(annex, gamma_m, default_situation, reliability_class, w, lc);
            n.v_ed <= n.v_rd()
        };
        if let Some(x) = solve_threshold(t, 3.0, 1e-3, |x| ok(&MasonryWall { thickness_m: x, ..wall.clone() })) {
            b = b.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase thickness for shear/sliding.", "Dicke für Schub/Gleiten erhöhen.")));
        }
        if let Some(x) = solve_threshold(wall.mu, 1.0, 1e-2, |x| ok(&MasonryWall { mu: x, ..wall.clone() })) {
            b = b.remedy(Remedy::at_least(wall_subject(wall, &path_mu), dimensionless(wall.mu), dimensionless(x), loc("Increase friction coefficient μ.", "Reibungsbeiwert μ erhöhen.")));
        }
    }
    out.push(b.ndps(SHEAR).ndps(&reads).ndps(&["ndp.en1996.m01.delta", shear_cap_ndp(annex), gamma_ndp]).build());
}

/// 🔩️ Lateral flexure state of the wall: demand and resistance moments [N·m].
pub(super) struct Flexure {
    ge: DesignEffects,
    w_ed: f64,
    pub(super) m_ed: f64,
    m_masonry: f64,
    m_reinf: f64,
    sigma_d: f64,
}

impl Flexure {
    pub(super) fn m_rd(&self) -> f64 {
        self.m_masonry + self.m_reinf
    }
}

pub(super) fn flexure(annex: AnnexChoice, gamma_m: f64, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase) -> Flexure {
    let effects = design_effects(annex, wall, lc, default_situation, reliability_class);
    let ge = governing_lateral(&effects).clone();
    let exceptional = matches!(DesignSituation::from(lc.design_situation), DesignSituation::Accidental | DesignSituation::Seismic) || matches!(default_situation, DesignSituation::Accidental);
    let l_ef = effective_length_m(wall);
    let m_wind = ge.w_pa * wall.height_m.powi(2) / 8.0 * l_ef;
    let m_earth = gamma_q(annex, reliability_class, exceptional) * lc.h_k_earth_n.max(0.0) * wall.height_m * 2.0 / (9.0 * 3f64.sqrt());
    let f_d = f_k_pa(annex, wall) / gamma_m;
    let n_min = effects.iter().map(|e| e.n_mid).fold(f64::INFINITY, f64::min);
    let sigma_d = (n_min / area_m2(wall).max(1e-9)).clamp(0.0, 0.2 * f_d);
    let f_xd1 = f_xk_pa(1, wall) / gamma_m;
    let m_reinf = if wall.as_vertical_m2 > 0.0 && wall.f_yd_pa > 0.0 { wall.as_vertical_m2 * wall.f_yd_pa * 0.9 * wall.thickness_m } else { 0.0 };
    Flexure { w_ed: ge.w_pa, m_ed: m_wind + m_earth, m_masonry: (f_xd1 + sigma_d) * section_modulus_m3(wall), m_reinf, sigma_d, ge }
}

/// 🔩️ Checks the lateral flexure of the wall under one load case.
fn check_wall_flexure(annex: AnnexChoice, gamma_m: f64, gamma_ndp: &'static str, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase, out: &mut Vec<CheckResult>) {
    let t = wall.thickness_m;
    let path_t = wall_path(wall, "thicknessM");
    let path_as_v = wall_path(wall, "asVerticalM2");
    let path_fyd = wall_path(wall, "fYdPa");
    let (f, reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| flexure(annex, gamma_m, default_situation, reliability_class, wall, lc));
    if f.m_ed <= 0.0 {
        out.push(
            CheckResult::assess(
                format!("en1996.6.3.flexure.{}.{}", wall.id, lc.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.3", "6.3"),
                wall_subject(wall, &load_case_path(wall, lc, "qPWindPa")),
                loc("Lateral flexure", "Biegung aus der Ebene"),
            )
            .not_applicable(loc("No wind/earth lateral action.", "Keine Wind-/Erddruck-Einwirkung."))
            .annex(annex)
            .build(),
        );
        return;
    }
    let text = |combo: &str| {
        format!(
            "{combo}; w_Ed={:.0} Pa; M_Ed={:.0} N·m; M_Rd={:.0} N·m = (f_xd1+σ_d)·Z with σ_d={:.0} Pa plus M_Rd,s={:.0} (As,v={:.6} m², f_yd={:.0} Pa).",
            f.w_ed, f.m_ed, f.m_rd(), f.sigma_d, f.m_reinf, wall.as_vertical_m2, wall.f_yd_pa
        )
    };
    let mut b = CheckResult::assess(
        format!("en1996.6.3.flexure.{}.{}", wall.id, lc.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§6.3", "6.3"),
        wall_subject(wall, &path_t),
        loc("Lateral flexure M_Ed ≤ M_Rd", "Plattenbiegung M_Ed ≤ M_Rd"),
    )
    .utilization(Quantity::new(QuantityKind::Moment, f.m_ed), Quantity::new(QuantityKind::Moment, f.m_rd().max(1e-9)))
    .annex(annex)
    .explanation(loc(&text(&f.ge.combo_en), &text(&f.ge.combo_de)));
    if wall.as_vertical_m2 > 0.0 && wall.f_yd_pa <= 0.0 {
        b = b.status(CheckStatus::Fail).remedy(Remedy::at_least(
            wall_subject(wall, &path_fyd),
            Quantity::new(QuantityKind::Stress, wall.f_yd_pa),
            Quantity::new(QuantityKind::Stress, f_yd_b500_pa(annex)),
            loc("Declare f_yd = f_yk/γ_S for the bed-joint reinforcement.", "f_yd = f_yk/γ_S für die Lagerfugenbewehrung angeben."),
        ));
    } else if f.m_ed > f.m_rd() {
        let ok = |w: &MasonryWall| {
            let n = flexure(annex, gamma_m, default_situation, reliability_class, w, lc);
            n.m_ed <= n.m_rd()
        };
        if let Some(x) = solve_threshold(t, 3.0, 1e-3, |x| ok(&MasonryWall { thickness_m: x, ..wall.clone() })) {
            b = b.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase thickness for lateral flexure.", "Dicke für Plattenbiegung erhöhen.")));
        }
        let f_yd = if wall.f_yd_pa > 0.0 { wall.f_yd_pa } else { f_yd_b500_pa(annex) };
        let as_req = ceil_to((f.m_ed - f.m_masonry) / (f_yd * 0.9 * t), 1e-6);
        b = b.remedy(Remedy::at_least(wall_subject(wall, &path_as_v), Quantity::new(QuantityKind::Area, wall.as_vertical_m2), Quantity::new(QuantityKind::Area, as_req), loc("Add vertical reinforcement As,v for the vertical span.", "Vertikalbewehrung As,v für die vertikale Spannrichtung ergänzen.")));
        if wall.f_yd_pa <= 0.0 {
            b = b.remedy(Remedy::at_least(wall_subject(wall, &path_fyd), Quantity::new(QuantityKind::Stress, 0.0), Quantity::new(QuantityKind::Stress, f_yd_b500_pa(annex)), loc("Declare f_yd = f_yk/γ_S for the bed-joint reinforcement.", "f_yd = f_yk/γ_S für die Lagerfugenbewehrung angeben.")));
        }
    }
    let m_rd = f.m_rd().max(1e-9);
    let profile = (0..=10).map(|i| {
        let z = f64::from(i) / 10.0;
        let m = f.m_ed * 4.0 * z * (1.0 - z);
        Station::ratio_of(&[z], Quantity::new(QuantityKind::Moment, m), Quantity::new(QuantityKind::Moment, m_rd))
    });
    let reinforced: &[&str] = if wall.as_vertical_m2 > 0.0 && wall.f_yd_pa > 0.0 { &["ndp.en1996.m40.flexure"] } else { &[] };
    out.push(b.field("wall-lateral", profile.collect()).ndps(FLEXURE).ndps(&reads).ndps(&strength_ndps(annex, wall)).ndp(gamma_ndp).ndps(reinforced).build());
}

/// ⚙️ Concentrated load state: factor β, bearing and effective areas, demand and resistance [N].
struct Concentrated {
    beta: f64,
    f_ed: f64,
    n_rdc: f64,
    ab: f64,
    a_ef: f64,
}

/// ⚙️ Effective bearing area A_ef = l_efm·t with the load spread at 60° over half the wall height.
fn effective_bearing_area_m2(wall: &MasonryWall, c: &ConcentratedLoad) -> f64 {
    (c.bearing_length_m + wall.height_m / 3f64.sqrt()).min(effective_length_m(wall)) * wall.thickness_m
}

fn concentrated(gamma_m: f64, annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, c: &ConcentratedLoad) -> Concentrated {
    let ab = if c.bearing_area_m2 > 0.0 { c.bearing_area_m2 } else { c.bearing_length_m * wall.thickness_m };
    let a_ef = effective_bearing_area_m2(wall, c);
    let beta = concentrated_beta(c.distance_a1_m.max(0.0), wall.height_m, ab, a_ef);
    Concentrated { beta, f_ed: action_factors(annex, reliability_class, false).gamma_g_sup() * c.force_n, n_rdc: beta * f_k_pa(annex, wall) / gamma_m * ab, ab, a_ef }
}

/// ⚙️ Checks the concentrated loads on the wall under one load case.
fn check_wall_concentrated_load(annex: AnnexChoice, gamma_m: f64, gamma_ndp: &'static str, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, lc: &WallLoadCase, out: &mut Vec<CheckResult>) {
    let t = wall.thickness_m;
    let path_t = wall_path(wall, "thicknessM");
    let ge = governing_compression(&design_effects(annex, wall, lc, default_situation, reliability_class)).clone();
    for c in &lc.concentrated {
        let (k, reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| concentrated(gamma_m, annex, reliability_class, wall, c));
        let basis: Vec<&'static str> = [reads, strength_ndps(annex, wall), CONCENTRATED.to_vec(), vec![gamma_ndp]].concat();
        let path_area = concentrated_path(wall, lc, &c.id, "bearingAreaM2");
        let path_len = concentrated_path(wall, lc, &c.id, "bearingLengthM");
        let text = |combo: &str| format!("{combo}; β={:.3} (a1={:.2} m); A_b={:.4} m²; A_ef={:.4} m²; F_Ed=γ_G·F_k={:.0} N; N_Rdc={:.0} N.", k.beta, c.distance_a1_m, k.ab, k.a_ef, k.f_ed, k.n_rdc);
        let mut b = CheckResult::assess(
            format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§6.1.3", "6.1.3"),
            wall_subject(wall, &path_len),
            loc("Concentrated load bearing", "Teilflächenpressung"),
        )
        .utilization(Quantity::new(QuantityKind::Force, k.f_ed), Quantity::new(QuantityKind::Force, k.n_rdc.max(1e-9)))
        .annex(annex)
        .explanation(loc(&text(&ge.combo_en), &text(&ge.combo_de)));
        if k.f_ed > k.n_rdc {
            let mut remedied = false;
            let ok_area = |x: f64| {
                let n = concentrated(gamma_m, annex, reliability_class, wall, &ConcentratedLoad { bearing_area_m2: x, ..c.clone() });
                n.f_ed <= n.n_rdc
            };
            if let Some(x) = solve_threshold(c.bearing_area_m2, area_m2(wall), 1e-4, ok_area) {
                remedied = true;
                b = b.remedy(Remedy::at_least(wall_subject(wall, &path_area), Quantity::new(QuantityKind::Area, c.bearing_area_m2), Quantity::new(QuantityKind::Area, x), loc("Increase bearing area for concentrated load.", "Auflagerfläche für Einzellast erhöhen.")));
            }
            let ok_t = |x: f64| {
                let n = concentrated(gamma_m, annex, reliability_class, &MasonryWall { thickness_m: x, ..wall.clone() }, c);
                n.f_ed <= n.n_rdc
            };
            if let Some(x) = solve_threshold(t, 3.0, 1e-3, ok_t) {
                remedied = true;
                b = b.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase wall thickness for concentrated bearing.", "Wanddicke für Teilflächenpressung erhöhen.")));
            }
            if !remedied {
                let ok_len = |x: f64| {
                    let n = concentrated(gamma_m, annex, reliability_class, wall, &ConcentratedLoad { bearing_length_m: x, ..c.clone() });
                    n.f_ed <= n.n_rdc
                };
                if let Some(x) = solve_threshold(c.bearing_length_m, wall.length_m, 1e-3, ok_len) {
                    b = b.remedy(Remedy::at_least(wall_subject(wall, &path_len), Quantity::length_m(c.bearing_length_m), Quantity::length_m(x), loc("Increase bearing length (spreading length).", "Auflagerlänge erhöhen (Lastausbreitung).")));
                }
            }
        }
        out.push(b.ndps(&basis).build());

        let ratio = normative_limit(&["concentrated", "area-ratio"]);
        let mut ba = CheckResult::assess(
            format!("en1996.6.1.3.area.{}.{}.{}", wall.id, lc.id, c.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§6.1.3", "6.1.3"),
            wall_subject(wall, &path_area),
            loc(&format!("Bearing area A_b ≤ {ratio} A_ef"), &format!("Auflagerfläche A_b ≤ {} A_ef", ratio.to_string().replace('.', ","))),
        )
        .utilization(Quantity::new(QuantityKind::Area, k.ab), Quantity::new(QuantityKind::Area, (ratio * k.a_ef).max(1e-9)))
        .annex(annex)
        .status(CheckStatus::Pass)
        .warn_above(1.0)
        .explanation(loc(&format!("A_b={:.4} m²; A_ef={:.4} m²; above {ratio} A_ef the enhancement β is dropped (β = 1, EN 1996-1-1 6.1.3): advisory, the resistance N_Edc ≤ N_Rdc is verified by the concentrated-load check, so this check warns and never fails.", k.ab, k.a_ef), &format!("A_b={:.4} m²; A_ef={:.4} m²; über {} A_ef entfällt die Erhöhung β (β = 1, EN 1996-1-1 6.1.3): hinweisend, der Nachweis N_Edc ≤ N_Rdc erfolgt in der Einzellastprüfung, daher nur Warnung, nie Nichtbestanden.", k.ab, k.a_ef, ratio.to_string().replace('.', ","))));
        if k.ab > ratio * k.a_ef {
            ba = ba.remedy(Remedy::at_most(wall_subject(wall, &path_area), Quantity::new(QuantityKind::Area, c.bearing_area_m2), Quantity::new(QuantityKind::Area, floor_to(ratio * k.a_ef, 1e-5)), loc(&format!("Reduce the bearing area to {:.0} % of the effective area.", ratio * 100.0), &format!("Auflagerfläche auf {:.0} % der wirksamen Fläche verringern.", ratio * 100.0))));
            let ok_len = |x: f64| {
                let n = concentrated(gamma_m, annex, reliability_class, wall, &ConcentratedLoad { bearing_length_m: x, ..c.clone() });
                n.ab <= ratio * n.a_ef
            };
            if let Some(x) = solve_threshold(c.bearing_length_m, wall.length_m, 1e-3, ok_len) {
                ba = ba.remedy(Remedy::at_least(wall_subject(wall, &path_len), Quantity::length_m(c.bearing_length_m), Quantity::length_m(x), loc("Increase bearing length to widen the effective area.", "Auflagerlänge erhöhen, wirksame Fläche vergrößern.")));
            }
        }
        out.push(ba.ndps(BEARING_RATIO).build());
    }
}
