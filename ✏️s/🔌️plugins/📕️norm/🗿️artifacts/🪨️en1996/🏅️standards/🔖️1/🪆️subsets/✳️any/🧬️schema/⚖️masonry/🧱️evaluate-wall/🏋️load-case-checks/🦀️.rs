//! 🏋️ Load case checks: the per load case compression, shear, flexure and concentrated load checks of the wall evaluation. See [`super`].

use super::*;

/// 🏋 Checks every load case of the wall: compression, shear, lateral flexure and concentrated load.
pub(super) fn check_load_cases(
    annex: AnnexChoice,
    masonry_class: MasonryClass,
    default_situation: DesignSituation,
    wall: &MasonryWall,
    out: &mut Vec<CheckResult>,
    path_t: &str,
    path_fb: &str,
    path_mu: &str,
    path_bearing: &str,
    path_as_h: &str,
    path_density: &str,
    path_phi_inf: &str,
    lambda: f64,
    a: f64,
    f_k: f64,
    t: f64,
) {
    for lc in &wall.load_cases {
        let effects = design_effects(annex, wall, lc, default_situation);
        let sit = situation_from_str(&lc.design_situation);
        let params = AnnexParams::for_document(annex, masonry_class, if matches!(sit, DesignSituation::Persistent) { default_situation } else { sit });
        let gamma_m = params.gamma_m();
        let f_d = f_k / gamma_m;

        {
            let ge = governing_compression(&effects);
            let e_slab = eccentricity_from_slab_bearing_m(wall);
            let e_theta = eccentricity_from_slab_rotation_m(wall, lc, ge.n_top.max(ge.n_mid));
            let phi1 = phi_1_slab_span(lc.slab_span_m);
            let e_top = (wall.eccentricity_top_m + e_slab + e_theta).abs();
            let e_bot = wall.eccentricity_bottom_m.abs();
            let h_ef = h_ef_m(wall);
            let e_init = h_ef / 450.0;
            let e_i_top = (e_top + e_init).abs();
            let e_i_bot = (e_bot + e_init).abs();
            let e_m = 0.5 * (e_top + e_bot) + e_init;
            let e_k = creep_eccentricity_m(wall, e_m);
            let e_mk = (e_m + e_k).abs().max(0.05 * t);
            let phi_top = phi_i(e_i_top, t) * phi1;
            let phi_bot = phi_i(e_i_bot, t) * phi1;
            let phi_mid = phi_m(lambda, e_mk, t) * phi1;
            let stations = [("top", ge.n_top, phi_top), ("mid", ge.n_mid, phi_mid), ("bottom", ge.n_bot, phi_bot)];
            let (station, n_ed, phi) = stations.iter().copied().max_by(|a, b| (a.1 / a.2.max(1e-9)).partial_cmp(&(b.1 / b.2.max(1e-9))).unwrap()).unwrap();
            let n_rd = phi * f_d * a;
            let path_span = load_case_path(wall, lc, "slabSpanM");
            let mut b = CheckResult::assess(
                format!("en1996.6.1.2.compression.{}.{}", wall.id, lc.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.1.2", "6.1.2"),
                wall_subject(wall, path_t),
                loc("Compression N_Ed ≤ Φ·Φ₁·f_d·A", "Druck N_Ed ≤ Φ·Φ₁·f_d·A"),
            )
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!(
                    "{}; station={station}; Φ·Φ₁={phi:.3} (Φ₁={phi1:.3}, NDP 6.1.2.2/NA C); e_top={e_top:.4}; e_θ={e_theta:.4} (ℓ_f={:.2} m); e_bot={e_bot:.4}; e_mk={e_mk:.4} m (e_k={e_k:.4}, φ∞={:.2}, ρ={:.0}); N_Ed={n_ed:.0} N; N_Rd={n_rd:.0} N; e_bearing={e_slab:.4} m.",
                    ge.combo_en, lc.slab_span_m, wall.phi_infinity, wall.density_kg_m3
                ),
                &format!(
                    "{}; Stelle={station}; Φ·Φ₁={phi:.3} (Φ₁={phi1:.3}, NDP 6.1.2.2/NA C); e_oben={e_top:.4}; e_θ={e_theta:.4} (ℓ_f={:.2} m); e_unten={e_bot:.4}; e_mk={e_mk:.4} m (e_k={e_k:.4}, φ∞={:.2}, ρ={:.0}); N_Ed={n_ed:.0} N; N_Rd={n_rd:.0} N; e_Auflager={e_slab:.4} m.",
                    ge.combo_de, lc.slab_span_m, wall.phi_infinity, wall.density_kg_m3
                ),
            ));
            let t_req = t * (n_ed / n_rd.max(1e-12)).max(1.0);
            let fb_req = wall.f_b_pa * (n_ed / n_rd.max(1e-12)).max(1.0);
            b = b
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t_req),
                    loc("Increase thickness.", "Dicke erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_fb),
                    Quantity::new(QuantityKind::Stress, wall.f_b_pa),
                    Quantity::new(QuantityKind::Stress, fb_req),
                    loc("Increase f_b.", "f_b erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_bearing),
                    Quantity::length_m(wall.slab_bearing_depth_m),
                    Quantity::length_m((t / 3.0).max(wall.slab_bearing_depth_m)),
                    loc("Increase slab bearing depth to reduce eccentricity.", "Deckenauflager vertiefen, Exzentrizität reduzieren."),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, &path_span),
                    Quantity::length_m(lc.slab_span_m),
                    Quantity::length_m((lc.slab_span_m * 0.85).max(4.0)),
                    loc("Reduce slab span to raise Φ₁ / cut e_θ (NA 6.1.2.2).", "Deckenspannweite reduzieren (Φ₁/e_θ, NA 6.1.2.2)."),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, path_phi_inf),
                    Quantity::new(QuantityKind::Dimensionless, wall.phi_infinity),
                    Quantity::new(QuantityKind::Dimensionless, (wall.phi_infinity * 0.7).max(0.5)),
                    loc("Reduce φ∞ to cut creep eccentricity e_k.", "φ∞ senken, Kriechexzentrizität e_k reduzieren."),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, path_density),
                    Quantity::new(QuantityKind::Mass, wall.density_kg_m3),
                    Quantity::new(QuantityKind::Mass, (wall.density_kg_m3 * 0.9).max(600.0)),
                    loc("Reduce masonry density (self-weight in N_Ed).", "Rohdichte senken (Eigengewicht in N_Ed)."),
                ));
            out.push(b.build());

            {
                let n_rd_mid = phi_mid * f_d * a;
                let bm = CheckResult::assess(
                    format!("en1996.6.1.2.compression.mid.{}.{}", wall.id, lc.id),
                    "EN 1996-1-1",
                    clause("EN 1996-1-1", "§6.1.2", "6.1.2"),
                    wall_subject(wall, path_phi_inf),
                    loc("Mid-height compression N_Ed ≤ Φ_m·Φ₁·f_d·A", "Druck Wandmitte N_Ed ≤ Φ_m·Φ₁·f_d·A"),
                )
                .utilization(Quantity::new(QuantityKind::Force, ge.n_mid), Quantity::new(QuantityKind::Force, n_rd_mid.max(1e-9)))
                .annex(annex)
                .explanation(loc(
                    &format!(
                        "mid; Φ_m·Φ₁={phi_mid:.3}; e_mk={e_mk:.4} m; e_k={e_k:.4} m (φ∞={:.2}); N_Ed={:.0} N; N_Rd={n_rd_mid:.0} N.",
                        wall.phi_infinity, ge.n_mid
                    ),
                    &format!(
                        "Mitte; Φ_m·Φ₁={phi_mid:.3}; e_mk={e_mk:.4} m; e_k={e_k:.4} m (φ∞={:.2}); N_Ed={:.0} N; N_Rd={n_rd_mid:.0} N.",
                        wall.phi_infinity, ge.n_mid
                    ),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, path_phi_inf),
                    Quantity::new(QuantityKind::Dimensionless, wall.phi_infinity),
                    Quantity::new(QuantityKind::Dimensionless, (wall.phi_infinity * 0.7).max(0.5)),
                    loc("Reduce φ∞ to cut creep eccentricity e_k.", "φ∞ senken, Kriechexzentrizität e_k reduzieren."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (ge.n_mid / n_rd_mid.max(1e-12)).max(1.0)),
                    loc("Increase thickness for mid-height compression.", "Dicke für Druck in Wandmitte erhöhen."),
                ));
                out.push(bm.build());
            }

            {
                let e_lim = 0.2 * t;
                let e_bot_abs = wall.eccentricity_bottom_m.abs();
                let e_top_decl = wall.eccentricity_top_m.abs();
                let e_slab = eccentricity_from_slab_bearing_m(wall);
                let e_gov = e_bot_abs.max(e_top_decl);
                let be = CheckResult::assess(
                    format!("en1996.6.1.2.eccentricity.{}.{}", wall.id, lc.id),
                    "EN 1996-1-1",
                    clause("EN 1996-1-1", "§6.1.2.2", "6.1.2.2"),
                    wall_subject(wall, &wall_path(wall, "eccentricityBottomM")),
                    loc("Declared eccentricity e ≤ 0.2 t", "Angegebene Exzentrizität e ≤ 0,2 t"),
                )
                .utilization(Quantity::length_m(e_gov), Quantity::length_m(e_lim.max(1e-9)))
                .annex(annex)
                .explanation(loc(
                    &format!("e_top,decl={e_top_decl:.4} m; e_bot={e_bot_abs:.4} m; e_slab={e_slab:.4} m; limit 0.2t={e_lim:.4} m.",),
                    &format!("e_oben,ang={e_top_decl:.4} m; e_unten={e_bot_abs:.4} m; e_Decke={e_slab:.4} m; Grenze 0,2t={e_lim:.4} m.",),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, &wall_path(wall, "eccentricityBottomM")),
                    Quantity::length_m(e_bot_abs),
                    Quantity::length_m(e_lim),
                    loc("Reduce bottom eccentricity.", "Exzentrizität unten reduzieren."),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, &wall_path(wall, "eccentricityTopM")),
                    Quantity::length_m(e_top_decl),
                    Quantity::length_m(e_lim),
                    loc("Reduce top eccentricity.", "Exzentrizität oben reduzieren."),
                ));
                out.push(be.build());
            }
        }

        {
            let ge = governing_shear(&effects);
            let n_ed = ge.n_mid;
            let v_ed = ge.v;
            let sigma_d = n_ed / a.max(1e-9);
            let f_vk = f_vk_pa(annex, wall.mortar_class, wall.unit_material, sigma_d, wall.f_b_pa);
            let v_rd_masonry = f_vk / gamma_m * a;
            let v_rd_slide = (wall.mu * n_ed / gamma_m.max(1.0)).max(1e-9);
            let v_rd = v_rd_masonry.min(v_rd_slide);
            let mut b = CheckResult::assess(
                format!("en1996.6.2.shear.{}.{}", wall.id, lc.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.2", "6.2"),
                wall_subject(wall, path_t),
                loc("Shear / sliding V_Ed ≤ V_Rd", "Schub / Gleiten V_Ed ≤ V_Rd"),
            )
            .utilization(Quantity::new(QuantityKind::Force, v_ed), Quantity::new(QuantityKind::Force, v_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!(
                    "{}; V_Ed={v_ed:.0} N; V_Rd,masonry={v_rd_masonry:.0} N; V_Rd,slide=μ·N/γ_M={v_rd_slide:.0} N (μ={:.2}); f_vk={:.3} MPa.",
                    ge.combo_en, wall.mu, f_vk / 1e6
                ),
                &format!(
                    "{}; V_Ed={v_ed:.0} N; V_Rd,Mauerwerk={v_rd_masonry:.0} N; V_Rd,Gleiten=μ·N/γ_M={v_rd_slide:.0} N (μ={:.2}); f_vk={:.3} MPa.",
                    ge.combo_de, wall.mu, f_vk / 1e6
                ),
            ));
            b = b
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (v_ed / v_rd.max(1e-12)).max(1.0)),
                    loc("Increase thickness for shear/sliding.", "Dicke für Schub/Gleiten erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_mu),
                    Quantity::new(QuantityKind::Dimensionless, wall.mu),
                    Quantity::new(QuantityKind::Dimensionless, (wall.mu * 1.25).min(0.8)),
                    loc("Increase friction coefficient μ.", "Reibungsbeiwert μ erhöhen."),
                ));
            out.push(b.build());
        }

        {
            let ge = governing_lateral(&effects);
            let w_ed = ge.w_pa;
            if w_ed <= 0.0 && lc.h_k_earth_n <= 0.0 {
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
            } else {
                let m_ed = w_ed * wall.height_m.powi(2) / 8.0 * effective_length_m(wall);
                let z = section_modulus_m3(wall);
                let f_xd = f_xk_pa(1, wall.mortar_class, wall.unit_material) / gamma_m;
                let z_s = 0.9 * wall.thickness_m;
                let as_h = wall.as_horizontal_m2.max(0.0);
                let f_yd = wall.f_yd_pa.max(0.0);
                let m_rd_reinf = if as_h > 0.0 && f_yd > 0.0 {
                    as_h * f_yd * z_s * effective_length_m(wall)
                } else {
                    0.0
                };
                let m_rd = f_xd * z + m_rd_reinf;
                let mut b = CheckResult::assess(
                    format!("en1996.6.3.flexure.{}.{}", wall.id, lc.id),
                    "EN 1996-1-1",
                    clause("EN 1996-1-1", "§6.3", "6.3"),
                    wall_subject(wall, path_t),
                    loc("Lateral flexure M_Ed ≤ M_Rd", "Plattenbiegung M_Ed ≤ M_Rd"),
                )
                .utilization(Quantity::new(QuantityKind::Moment, m_ed), Quantity::new(QuantityKind::Moment, m_rd.max(1e-9)))
                .annex(annex)
                .explanation(loc(
                    &format!("{}; w_Ed={w_ed:.0} Pa; M_Ed={m_ed:.0} N·m; M_Rd={m_rd:.0} N·m (As,h={as_h:.6} m², f_yd={:.0} Pa, M_Rd,s={m_rd_reinf:.0}).", ge.combo_en, f_yd),
                    &format!("{}; w_Ed={w_ed:.0} Pa; M_Ed={m_ed:.0} N·m; M_Rd={m_rd:.0} N·m (As,h={as_h:.6} m², f_yd={:.0} Pa, M_Rd,s={m_rd_reinf:.0}) (Plattenbiegung).", ge.combo_de, f_yd),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (m_ed / m_rd.max(1e-12)).max(1.0).sqrt() * 1.01),
                    loc("Increase thickness for lateral flexure.", "Dicke für Plattenbiegung erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_as_h),
                    Quantity::new(QuantityKind::Area, as_h),
                    Quantity::new(QuantityKind::Area, (as_h + 50e-6).max(50e-6)),
                    loc("Add bed-joint reinforcement As,h.", "Lagerfugenbewehrung As,h ergänzen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &wall_path(wall, "fYdPa")),
                    Quantity::new(QuantityKind::Stress, f_yd),
                    Quantity::new(QuantityKind::Stress, 435e6),
                    loc("Declare f_yd for bed-joint reinforcement.", "f_yd für Lagerfugenbewehrung angeben."),
                ));
                if as_h > 0.0 && f_yd <= 0.0 {
                    b = b.utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0));
                }
                out.push(b.build());
            }
        }

        for c in &lc.concentrated {
            let ge = governing_compression(&effects);
            let beta = concentrated_beta(c.bearing_length_m, wall.height_m, wall.length_m);
            let n_rdc = beta * f_d * c.bearing_area_m2.max(c.bearing_length_m * t);
            let f_ed = gamma_q(annex, false) * c.force_n;
            let mut b = CheckResult::assess(
                format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.1.3", "6.1.3"),
                wall_subject(wall, &concentrated_path(wall, lc, &c.id, "bearingLengthM")),
                loc("Concentrated load bearing", "Teilflächenpressung"),
            )
            .utilization(Quantity::new(QuantityKind::Force, f_ed), Quantity::new(QuantityKind::Force, n_rdc.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!("{}; β={beta:.2}; F_Ed={f_ed:.0} N; N_Rdc={n_rdc:.0} N.", ge.combo_en),
                &format!("{}; β={beta:.2}; F_Ed={f_ed:.0} N; N_Rdc={n_rdc:.0} N (Teilflächenpressung).", ge.combo_de),
            ));
            let area_used = c.bearing_area_m2.max(c.bearing_length_m * t);
            let area_req = area_used * (f_ed / n_rdc.max(1e-12)).max(1.0) * 1.01;
            b = b
                .remedy(Remedy::at_least(
                    wall_subject(wall, &concentrated_path(wall, lc, &c.id, "bearingAreaM2")),
                    Quantity::new(QuantityKind::Area, c.bearing_area_m2),
                    Quantity::new(QuantityKind::Area, area_req),
                    loc("Increase bearing area for concentrated load.", "Auflagerfläche für Einzellast erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &concentrated_path(wall, lc, &c.id, "bearingLengthM")),
                    Quantity::length_m(c.bearing_length_m),
                    Quantity::length_m((area_req / t.max(1e-9)).max(c.bearing_length_m)),
                    loc("Increase bearing length.", "Auflagerlänge erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (f_ed / n_rdc.max(1e-12)).max(1.0)),
                    loc("Increase wall thickness for concentrated bearing.", "Wanddicke für Teilflächenpressung erhöhen."),
                ));
            out.push(b.build());
        }
    }
}
