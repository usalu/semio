//! 🧱️ The wall evaluation of the EN 1996 masonry checks: the clause-by-clause compliance assessment of one wall. See [`super`].

use super::*;

//#region ⚖️Evaluate
pub(super) fn evaluate_wall(
    annex: AnnexChoice,
    masonry_class: MasonryClass,
    default_situation: DesignSituation,
    storeys: u32,
    wall: &MasonryWall,
) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let path_t = wall_path(wall, "thicknessM");
        let path_support = wall_path(wall, "supportSides");
    let path_fb = wall_path(wall, "fBPa");
    let path_mortar = wall_path(wall, "mortarClass");
    let path_mortar_f = wall_path(wall, "mortarStrengthPa");
    let path_mu = wall_path(wall, "mu");
    let path_bearing = wall_path(wall, "slabBearingDepthM");
    let path_as_h = wall_path(wall, "asHorizontalM2");
    let path_unit_h = wall_path(wall, "unitHeightM");
    let path_density = wall_path(wall, "densityKgM3");
    let path_phi_inf = wall_path(wall, "phiInfinity");
    let path_basement = wall_path(wall, "isBasement");

    let lambda = slenderness(wall);
    let a = area_m2(wall);
    let f_k = f_k_pa(annex, wall);
    let t = wall.thickness_m;

    // Slenderness
    {
        let mut b = CheckResult::assess(
            format!("en1996.5.5.1.slenderness.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§5.5.1", "5.5.1"),
            wall_subject(wall, &path_t),
            loc("Slenderness λ ≤ 27", "Schlankheit λ ≤ 27"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, lambda), Quantity::new(QuantityKind::Dimensionless, 27.0))
        .annex(annex)
        .explanation(loc(
            &format!("λ=h_ef/t_ef={lambda:.2} with L_eff={:.3} m (openings).", effective_length_m(wall)),
            &format!("λ=h_ef/t_ef={lambda:.2} mit L_eff={:.3} m (Öffnungen).", effective_length_m(wall)),
        ));
        let t_req = h_ef_m(wall) / 27.0;
        b = b
            .remedy(Remedy::at_least(
                wall_subject(wall, &path_t),
                Quantity::length_m(t),
                Quantity::length_m(t_req),
                loc("Increase thickness so λ ≤ 27.", "Dicke erhöhen, damit λ ≤ 27."),
            ))
            .remedy(Remedy::exactly(
                wall_subject(wall, &path_support),
                Quantity::new(QuantityKind::Dimensionless, wall.support_sides as f64),
                Quantity::new(QuantityKind::Dimensionless, 4.0),
                loc("Increase to 4-sided support.", "Auf 4-seitige Halterung erhöhen."),
            ));
        out.push(b.build());
    }

    // Material conformance (replaces tautological f_k ≥ 0.5 MPa)
    {
        let (ok, en, de) = material_conformance_ok(wall);
        let mut b = CheckResult::assess(
            format!("en1996.3.1.material.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§3.1", "3.1"),
            wall_subject(wall, &path_mortar_f),
            loc("Unit/mortar conformance (DE NA)", "Stein/Mörtel-Konformität (DE NA)"),
        )
        .annex(annex)
        .explanation(loc(&en, &de));
        if ok {
            b = b.utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0));
        } else {
            let path_joint = wall_path(wall, "bedJointThicknessM");
            let path_unit_w = wall_path(wall, "unitWidthM");
            let joint_target = if matches!(wall.mortar_type, MortarType::ThinLayer) {
                0.002
            } else {
                0.010
            };
            let fb_target_mpa = match wall.unit_material {
                UnitMaterial::Aerated => wall.f_b_pa.max(2.0e6).min(10.0e6) / 1e6,
                UnitMaterial::Clay => wall.f_b_pa.max(6.0e6) / 1e6,
                _ => wall.f_b_pa.max(6.0e6) / 1e6,
            };
            b = b
                .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_mortar_f),
                    Quantity::new(QuantityKind::Stress, wall.mortar_strength_pa),
                    Quantity::new(QuantityKind::Stress, wall.mortar_class.f_m_mpa() * 1e6),
                    loc("Align mortarStrengthPa with mortarClass.", "mortarStrengthPa an mortarClass anpassen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_fb),
                    Quantity::new(QuantityKind::Stress, wall.f_b_pa),
                    Quantity::stress_mpa(fb_target_mpa.max(6.0)),
                    loc("Raise unit strength f_b into admissible range.", "f_b in zulässigen Bereich anheben."),
                ))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_unit_h),
                    Quantity::length_m(wall.unit_height_m),
                    Quantity::length_m(0.113),
                    loc("Use admissible unit height for group/shape factor δ.", "Zulässige Steinhöhe für Formfaktor δ wählen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_unit_w),
                    Quantity::length_m(wall.unit_width_m),
                    Quantity::length_m((wall.unit_height_m / 2.4).max(0.100)),
                    loc("Widen unit so group aspect and dimensions conform.", "Steinsicht verbreitern für Gruppen-/Abmessungskonformität."),
                ))
                .remedy(Remedy::exactly(
                    wall_subject(wall, &path_joint),
                    Quantity::length_m(wall.bed_joint_thickness_m),
                    Quantity::length_m(joint_target),
                    loc(
                        "Set bed-joint thickness to admissible GP (10 mm) or thin-layer (2 mm) value.",
                        "Lagerfugendicke auf zulässigen NM- (10 mm) bzw. DBM-Wert (2 mm) setzen.",
                    ),
                ));
        }
        out.push(b.build());
    }

    if wall.load_cases.is_empty() {
        out.push(
            CheckResult::assess(
                format!("en1996.loads.empty.{}", wall.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§2", "2"),
                wall_subject(wall, &wall_path(wall, "loadCases")),
                loc("Load cases present", "Lastfälle vorhanden"),
            )
            .not_applicable(loc("No load cases on wall.", "Keine Lastfälle an der Wand."))
            .annex(annex)
            .build(),
        );
        return out;
    }

    for lc in &wall.load_cases {
        let effects = design_effects(annex, wall, lc, default_situation);
        let sit = situation_from_str(&lc.design_situation);
        let params = AnnexParams::for_document(annex, masonry_class, if matches!(sit, DesignSituation::Persistent) { default_situation } else { sit });
        let gamma_m = params.gamma_m();
        let f_d = f_k / gamma_m;

        // Compression §6.1.2
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
                wall_subject(wall, &path_t),
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
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t_req),
                    loc("Increase thickness.", "Dicke erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_fb),
                    Quantity::new(QuantityKind::Stress, wall.f_b_pa),
                    Quantity::new(QuantityKind::Stress, fb_req),
                    loc("Increase f_b.", "f_b erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_bearing),
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
                    wall_subject(wall, &path_phi_inf),
                    Quantity::new(QuantityKind::Dimensionless, wall.phi_infinity),
                    Quantity::new(QuantityKind::Dimensionless, (wall.phi_infinity * 0.7).max(0.5)),
                    loc("Reduce φ∞ to cut creep eccentricity e_k.", "φ∞ senken, Kriechexzentrizität e_k reduzieren."),
                ))
                .remedy(Remedy::at_most(
                    wall_subject(wall, &path_density),
                    Quantity::new(QuantityKind::Mass, wall.density_kg_m3),
                    Quantity::new(QuantityKind::Mass, (wall.density_kg_m3 * 0.9).max(600.0)),
                    loc("Reduce masonry density (self-weight in N_Ed).", "Rohdichte senken (Eigengewicht in N_Ed)."),
                ));
            out.push(b.build());

            // Mid-height station always reported (§6.1.2 Φ_m with e_mk incl. creep e_k from φ∞).
            {
                let n_rd_mid = phi_mid * f_d * a;
                let bm = CheckResult::assess(
                    format!("en1996.6.1.2.compression.mid.{}.{}", wall.id, lc.id),
                    "EN 1996-1-1",
                    clause("EN 1996-1-1", "§6.1.2", "6.1.2"),
                    wall_subject(wall, &path_phi_inf),
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
                    wall_subject(wall, &path_phi_inf),
                    Quantity::new(QuantityKind::Dimensionless, wall.phi_infinity),
                    Quantity::new(QuantityKind::Dimensionless, (wall.phi_infinity * 0.7).max(0.5)),
                    loc("Reduce φ∞ to cut creep eccentricity e_k.", "φ∞ senken, Kriechexzentrizität e_k reduzieren."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (ge.n_mid / n_rd_mid.max(1e-12)).max(1.0)),
                    loc("Increase thickness for mid-height compression.", "Dicke für Druck in Wandmitte erhöhen."),
                ));
                out.push(bm.build());
            }

            // Declared member eccentricities (excl. slab-bearing offset handled in Φ) ≤ 0.2·t (§6.1.2.2).
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

        // Shear + sliding §6.2
        {
            let ge = governing_shear(&effects);
            let n_ed = ge.n_mid;
            let v_ed = ge.v;
            let sigma_d = n_ed / a.max(1e-9);
            let f_vk = f_vk_pa(annex, wall.mortar_class, wall.unit_material, sigma_d, wall.f_b_pa);
            let v_rd_masonry = f_vk / gamma_m * a;
            // Sliding: V_Rd,slide = μ · N_Ed / γ_M (friction) — DE NA §6.2; governing = min(masonry, sliding).
            let v_rd_slide = (wall.mu * n_ed / gamma_m.max(1.0)).max(1e-9);
            let v_rd = v_rd_masonry.min(v_rd_slide);
            let mut b = CheckResult::assess(
                format!("en1996.6.2.shear.{}.{}", wall.id, lc.id),
                "EN 1996-1-1",
                clause("EN 1996-1-1", "§6.2", "6.2"),
                wall_subject(wall, &path_t),
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
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (v_ed / v_rd.max(1e-12)).max(1.0)),
                    loc("Increase thickness for shear/sliding.", "Dicke für Schub/Gleiten erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_mu),
                    Quantity::new(QuantityKind::Dimensionless, wall.mu),
                    Quantity::new(QuantityKind::Dimensionless, (wall.mu * 1.25).min(0.8)),
                    loc("Increase friction coefficient μ.", "Reibungsbeiwert μ erhöhen."),
                ));
            out.push(b.build());
        }

        // Lateral flexure §6.3 (+ horizontal reinforcement)
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
                let m_ed = w_ed * wall.height_m.powi(2) / 8.0 * effective_length_m(wall); // N·m per wall length strip integrated
                let z = section_modulus_m3(wall);
                let f_xd = f_xk_pa(1, wall.mortar_class, wall.unit_material) / gamma_m;
                let z_s = 0.9 * wall.thickness_m;
                let as_h = wall.as_horizontal_m2.max(0.0);
                let f_yd = wall.f_yd_pa.max(0.0);
                // Bed-joint reinforcement §6.6 — As,h only counted with declared f_yd.
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
                    wall_subject(wall, &path_t),
                    loc("Lateral flexure M_Ed ≤ M_Rd", "Plattenbiegung M_Ed ≤ M_Rd"),
                )
                .utilization(Quantity::new(QuantityKind::Moment, m_ed), Quantity::new(QuantityKind::Moment, m_rd.max(1e-9)))
                .annex(annex)
                .explanation(loc(
                    &format!("{}; w_Ed={w_ed:.0} Pa; M_Ed={m_ed:.0} N·m; M_Rd={m_rd:.0} N·m (As,h={as_h:.6} m², f_yd={:.0} Pa, M_Rd,s={m_rd_reinf:.0}).", ge.combo_en, f_yd),
                    &format!("{}; w_Ed={w_ed:.0} Pa; M_Ed={m_ed:.0} N·m; M_Rd={m_rd:.0} N·m (As,h={as_h:.6} m², f_yd={:.0} Pa, M_Rd,s={m_rd_reinf:.0}) (Plattenbiegung).", ge.combo_de, f_yd),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (m_ed / m_rd.max(1e-12)).max(1.0).sqrt() * 1.01),
                    loc("Increase thickness for lateral flexure.", "Dicke für Plattenbiegung erhöhen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_as_h),
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

        // Concentrated §6.1.3
        for c in &lc.concentrated {
            let ge = governing_compression(&effects);
            let beta = concentrated_beta(c.bearing_length_m, wall.height_m, wall.length_m);
            let n_rdc = beta * f_d * c.bearing_area_m2.max(c.bearing_length_m * t);
            // Design concentrated = γ_Q * F_k (permanent portion already in G path separately)
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
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (f_ed / n_rdc.max(1e-12)).max(1.0)),
                    loc("Increase wall thickness for concentrated bearing.", "Wanddicke für Teilflächenpressung erhöhen."),
                ));
            out.push(b.build());
        }
    }

    // Fire 1-2 with α
    {
        let params = AnnexParams::for_document(annex, masonry_class, default_situation);
        let f_d = f_k / params.gamma_m();
        let n_rd_cold = phi_s(annex, lambda).max(phi_m(lambda, 0.05 * t, t)) * f_d * a;
        // Fire combination: G + ψ1,1·Q (simplified η_fi ≈ 0.7 for buildings) → N_Ed,fi
        let mut n_ed_fi: f64 = 0.0;
        for lc in &wall.load_cases {
            let g = lc.g_k_slab_n + self_weight_n(wall) + lc.concentrated.iter().map(|c| c.force_n).sum::<f64>();
            let q = lc.q_k_imposed_pa * lc.tributary_area_m2 + lc.q_k_snow_pa * lc.tributary_area_m2;
            n_ed_fi = n_ed_fi.max(g + 0.7 * q);
        }
        let alpha = n_ed_fi / n_rd_cold.max(1e-9);
        let t_min = fire_min_thickness_na(wall.fire_rei_min, wall.unit_material, wall.unit_group, wall.mortar_type, alpha);
        let mut b = CheckResult::assess(
            format!("en1996.1-2.fire.{}", wall.id),
            "EN 1996-1-2",
            clause("EN 1996-1-2", "§4", "4"),
            wall_subject(wall, &path_t),
            loc("Fire resistance tabulated (α)", "Feuerwiderstand tabellarisch (α)"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("REI {}; α=N_Ed,fi/N_Rd={alpha:.3}; t={t:.3} m; t_min(α)={t_min:.3} m.", wall.fire_rei_min),
            &format!("REI {}; α=N_Ed,fi/N_Rd={alpha:.3}; t={t:.3} m; t_min(α)={t_min:.3} m (tabellarisch).", wall.fire_rei_min),
        ));
        b = b.minimum(Quantity::length_m(t), Quantity::length_m(t_min));
        if alpha > 1.0 {
            b = b
                .utilization(Quantity::new(QuantityKind::Dimensionless, alpha), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t_min.max(t * alpha)),
                    loc("Increase thickness for fire / reduce α.", "Dicke für Brandschutz erhöhen / α reduzieren."),
                ));
        } else {
            b = b.remedy(Remedy::at_least(
                wall_subject(wall, &path_t),
                Quantity::length_m(t),
                Quantity::length_m(t_min.max(t)),
                loc("Increase thickness for fire / reduce α.", "Dicke für Brandschutz erhöhen / α reduzieren."),
            ));
        }
        out.push(b.build());
    }

    // Durability / bed joint
    {
        let need = required_mortar_mpa(wall.exposure, wall.unit_material);
        let have = wall.mortar_class.f_m_mpa();
        let mut b = CheckResult::assess(
            format!("en1996.2.durability.{}", wall.id),
            "EN 1996-2",
            clause("EN 1996-2", "Annex B", "B"),
            wall_subject(wall, &path_mortar),
            loc("Exposure / mortar durability", "Exposition / Mörtelbeständigkeit"),
        )
        .annex(annex);
        if !need.is_finite() {
            b = b
                .utilization(Quantity::new(QuantityKind::Dimensionless, 2.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .explanation(loc("Material/exposure combination inadmissible.", "Material/Exposition unzulässig."))
                .remedy(Remedy::one_of(
                    wall_subject(wall, &wall_path(wall, "exposure")),
                    vec!["Mx1".into(), "Mx2".into(), "Mx3".into()],
                    loc("Reduce exposure class.", "Expositionsklasse reduzieren."),
                ));
        } else {
            b = b
                .minimum(Quantity::new(QuantityKind::Stress, have * 1e6), Quantity::new(QuantityKind::Stress, need * 1e6))
                .explanation(loc(
                    &format!("Required mortar ≥ {need:.1} MPa for {:?}; have {have:.1} MPa.", wall.exposure),
                    &format!("Erforderlicher Mörtel ≥ {need:.1} MPa für {:?}; vorhanden {have:.1} MPa.", wall.exposure),
                ))
                .remedy(Remedy::one_of(
                    wall_subject(wall, &path_mortar),
                    vec!["M10".into(), "M15".into(), "M20".into()],
                    loc("Upgrade mortar class.", "Mörtelklasse erhöhen."),
                ));
        }
        out.push(b.build());

        let joint_ok = wall.bed_joint_thickness_m >= 0.006 && wall.bed_joint_thickness_m <= 0.015
            || (matches!(wall.mortar_type, MortarType::ThinLayer) && wall.bed_joint_thickness_m <= 0.003);
        let mut bj = CheckResult::assess(
            format!("en1996.2.bedjoint.{}", wall.id),
            "EN 1996-2",
            clause("EN 1996-2", "§8", "8"),
            wall_subject(wall, &wall_path(wall, "bedJointThicknessM")),
            loc("Bed-joint thickness", "Lagerfugendicke"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("t_bed={:.3} m; mortar={:?}.", wall.bed_joint_thickness_m, wall.mortar_type),
            &format!("t_Lager={:.3} m; Mörtel={:?}.", wall.bed_joint_thickness_m, wall.mortar_type),
        ));
        if joint_ok {
            bj = bj.utilization(Quantity::new(QuantityKind::Dimensionless, 0.5), Quantity::new(QuantityKind::Dimensionless, 1.0));
        } else {
            bj = bj
                .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .remedy(Remedy::at_most(
                    wall_subject(wall, &wall_path(wall, "bedJointThicknessM")),
                    Quantity::length_m(wall.bed_joint_thickness_m),
                    Quantity::length_m(0.015),
                    loc("Bring bed joint into 6–15 mm (GP) or ≤3 mm (thin).", "Lagerfuge auf 6–15 mm (NM) bzw. ≤3 mm (DBM) bringen."),
                ));
        }
        out.push(bj.build());
    }

    // Reinforced masonry §6.6 / §9 — always evaluate so As,v / As,h / f_yd / reinforced are live inputs
    {
        let as_min = 0.0005 * a;
        let as_v = wall.as_vertical_m2.max(0.0);
        let as_h = wall.as_horizontal_m2.max(0.0);
        let as_have = as_v + as_h;
        let mut b = CheckResult::assess(
            format!("en1996.6.6.reinforced.{}", wall.id),
            "EN 1996-1-1",
            clause("EN 1996-1-1", "§6.6", "6.6"),
            wall_subject(wall, &wall_path(wall, "asVerticalM2")),
            loc("Reinforced masonry As / f_yd", "Bewehrtes Mauerwerk As / f_yd"),
        )
        .annex(annex)
        .explanation(loc(
            &format!(
                "reinforced={}; As,v={as_v:.6} m²; As,h={as_h:.6} m²; f_yd={:.0} Pa; min As≈{as_min:.6} m².",
                wall.reinforced, wall.f_yd_pa
            ),
            &format!(
                "bewehrt={}; As,v={as_v:.6} m²; As,h={as_h:.6} m²; f_yd={:.0} Pa; min As≈{as_min:.6} m².",
                wall.reinforced, wall.f_yd_pa
            ),
        ));
        if !wall.reinforced {
            // Unreinforced: As and f_yd must stay at zero — any steel area without the flag fails.
            let stray = as_have + wall.f_yd_pa;
            if stray > 0.0 {
                b = b
                    .utilization(Quantity::new(QuantityKind::Dimensionless, 1.5), Quantity::new(QuantityKind::Dimensionless, 1.0))
                    .remedy(Remedy::exactly(
                        wall_subject(wall, &wall_path(wall, "reinforced")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        loc("Set reinforced=true when As or f_yd is provided.", "reinforced=true setzen, wenn As oder f_yd angegeben."),
                    ));
            } else {
                b = b.utilization(Quantity::new(QuantityKind::Dimensionless, 0.2), Quantity::new(QuantityKind::Dimensionless, 1.0));
            }
        } else if as_have < as_min || wall.f_yd_pa <= 0.0 {
            b = b
                .minimum(Quantity::new(QuantityKind::Area, as_have), Quantity::new(QuantityKind::Area, as_min.max(1e-9)))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_as_h),
                    Quantity::new(QuantityKind::Area, as_h),
                    Quantity::new(QuantityKind::Area, as_min),
                    loc("Provide minimum reinforcement.", "Mindestbewehrung vorsehen."),
                ))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &wall_path(wall, "fYdPa")),
                    Quantity::new(QuantityKind::Stress, wall.f_yd_pa),
                    Quantity::new(QuantityKind::Stress, 435e6),
                    loc("Declare reinforcement f_yd.", "Bewehrungs-f_yd angeben."),
                ));
        } else {
            b = b
                .minimum(Quantity::new(QuantityKind::Area, as_have), Quantity::new(QuantityKind::Area, as_min.max(1e-9)))
                .remedy(Remedy::at_least(
                    wall_subject(wall, &path_as_h),
                    Quantity::new(QuantityKind::Area, as_h),
                    Quantity::new(QuantityKind::Area, as_min),
                    loc("Provide minimum reinforcement.", "Mindestbewehrung vorsehen."),
                ));
        }
        out.push(b.build());
    }

    // EN 1996-3 / NA simplified + basement
    {
        let part = "DIN EN 1996-3";
        let mut reasons_en = Vec::new();
        let mut reasons_de = Vec::new();
        if storeys > 3 {
            reasons_en.push(format!("storeys={storeys}>3"));
            reasons_de.push(format!("Geschosse={storeys}>3"));
        }
        if lambda > 27.0 {
            reasons_en.push(format!("λ={lambda:.1}>27"));
            reasons_de.push(format!("λ={lambda:.1}>27"));
        }
        if !matches!(wall.wall_type, WallType::LoadBearing) {
            reasons_en.push("wall not load-bearing".into());
            reasons_de.push("Wand nicht tragend".into());
        }
        // Table NA.A.1
        let q_k_max: f64 = wall.load_cases.iter().map(|lc| lc.q_k_imposed_pa).fold(0.0, f64::max);
        if q_k_max > 5_000.0 {
            reasons_en.push(format!("q_k={:.0} Pa > 5 kN/m²", q_k_max));
            reasons_de.push(format!("q_k={:.0} Pa > 5 kN/m²", q_k_max));
        }
        let span_max: f64 = wall.load_cases.iter().map(|lc| lc.slab_span_m).fold(0.0, f64::max);
        if span_max > 6.0 {
            reasons_en.push(format!("slab span={span_max:.2} m > 6 m"));
            reasons_de.push(format!("Deckenstützweite={span_max:.2} m > 6 m"));
        }
        if wall.thickness_m < 0.115 {
            reasons_en.push(format!("t={:.3} m < 115 mm", wall.thickness_m));
            reasons_de.push(format!("t={:.3} m < 115 mm", wall.thickness_m));
        }
        if wall.height_m > 3.0 && wall.thickness_m < 0.175 {
            reasons_en.push("height>3 m requires t≥175 mm (NA.A.1)".into());
            reasons_de.push("Höhe>3 m erfordert t≥175 mm (NA.A.1)".into());
        }
        let building_h = storeys as f64 * wall.height_m;
        if building_h > 12.0 {
            reasons_en.push(format!("approx. building height={building_h:.1} m > 12 m"));
            reasons_de.push(format!("Gebäudehöhe≈{building_h:.1} m > 12 m"));
        }

        if !reasons_en.is_empty() {
            out.push(
                CheckResult::assess(
                    format!("en1996.3.simplified.{}", wall.id),
                    part,
                    clause("EN 1996-3", "§4.2", "4.2"),
                    wall_subject(wall, &path_t),
                    loc("Simplified method applicability", "Anwendbarkeit vereinfachtes Verfahren"),
                )
                .not_applicable(loc(
                    &format!("Outside DIN EN 1996-3/NA Table NA.A.1: {}.", reasons_en.join("; ")),
                    &format!("Außerhalb DIN EN 1996-3/NA Tabelle NA.A.1: {}.", reasons_de.join("; ")),
                ))
                .annex(annex)
                .build(),
            );
        } else {
            let params = AnnexParams::for_document(annex, masonry_class, default_situation);
            let f_d = f_k / params.gamma_m();
            let phis = phi_s(annex, lambda);
            let n_rd = phis * f_d * a;
            let mut n_ed = 0.0_f64;
            let mut combo_en = String::new();
            let mut combo_de = String::new();
            for lc in &wall.load_cases {
                let eff = design_effects(annex, wall, lc, default_situation);
                let ge = governing_compression(&eff);
                if ge.n_bot > n_ed {
                    n_ed = ge.n_bot;
                    combo_en = ge.combo_en.clone();
                    combo_de = ge.combo_de.clone();
                }
            }
            let mut b = CheckResult::assess(
                format!("en1996.3.simplified.{}", wall.id),
                part,
                clause("EN 1996-3", "§4.2", "4.2"),
                wall_subject(wall, &path_t),
                loc("Simplified compression N_Rd", "Vereinfachte Drucktragfähigkeit N_Rd"),
            )
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!("{combo_en}; Φ_s={phis:.3}; N_Rd={n_rd:.0} N; N_Ed={n_ed:.0} N."),
                &format!("{combo_de}; Φ_s={phis:.3}; N_Rd={n_rd:.0} N; N_Ed={n_ed:.0} N (vereinfacht)."),
            ));
            if n_ed > n_rd {
                b = b.remedy(Remedy::at_least(
                    wall_subject(wall, &path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (n_ed / n_rd.max(1e-12)).max(1.0)),
                    loc("Increase thickness (simplified N_Rd).", "Dicke erhöhen (vereinfachtes N_Rd)."),
                ));
            }
            out.push(b.build());
        }

        // Basement earth-pressure §4.5 / DE NA
        if wall.is_basement {
            let mut n_min = f64::INFINITY;
            let mut n_max = 0.0_f64;
            let mut h_ed = 0.0_f64;
            for lc in &wall.load_cases {
                let eff = design_effects(annex, wall, lc, default_situation);
                for e in &eff {
                    n_min = n_min.min(e.n_bot);
                    n_max = n_max.max(e.n_bot);
                    h_ed = h_ed.max(e.v);
                }
            }
            if !n_min.is_finite() { n_min = 0.0; }
            // DE NA simplified earth-pressure: N_Ed,min ≥ 0.2·H_Ed·h/t  and N_Ed,max ≤ N_Rd,s
            let n_min_req = 0.2 * h_ed * wall.height_m / t.max(1e-6);
            let params = AnnexParams::for_document(annex, masonry_class, default_situation);
            let n_rd_s = phi_s(annex, lambda) * (f_k / params.gamma_m()) * a;
            let u1 = if n_min_req > 0.0 { n_min_req / n_min.max(1e-9) } else { 0.0 };
            let u2 = n_max / n_rd_s.max(1e-9);
            let u = u1.max(u2);
            let mut b = CheckResult::assess(
                format!("en1996.3.basement.{}", wall.id),
                part,
                clause("EN 1996-3", "§4.5", "4.5"),
                wall_subject(wall, &path_basement),
                loc("Basement wall earth pressure", "Kellerwand Erddruck"),
            )
            .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("N_Ed,min={n_min:.0} N (≥ {n_min_req:.0}); N_Ed,max={n_max:.0} N (≤ {n_rd_s:.0})."),
                &format!("N_Ed,min={n_min:.0} N (≥ {n_min_req:.0}); N_Ed,max={n_max:.0} N (≤ {n_rd_s:.0}) (Kellerwand §4.5)."),
            ));
            b = b.remedy(Remedy::at_least(
                wall_subject(wall, &path_t),
                Quantity::length_m(t),
                Quantity::length_m(t * 1.2),
                loc("Thicken basement wall / increase permanent load.", "Kellerwand verdicken / ständige Last erhöhen."),
            ));
            out.push(b.build());
        } else if wall.load_cases.iter().any(|lc| lc.h_k_earth_n > 0.0) {
            out.push(
                CheckResult::assess(
                    format!("en1996.3.basement.{}", wall.id),
                    part,
                    clause("EN 1996-3", "§4.5", "4.5"),
                    wall_subject(wall, &path_basement),
                    loc("Basement wall earth pressure", "Kellerwand Erddruck"),
                )
                .not_applicable(loc(
                    "Earth pressure given but isBasement=false — mark wall as basement or clear h_k_earth.",
                    "Erddruck angegeben, aber isBasement=false — als Kellerwand kennzeichnen oder h_k_earth löschen.",
                ))
                .annex(annex)
                .build(),
            );
        }
    }

    out
}
//#endregion
