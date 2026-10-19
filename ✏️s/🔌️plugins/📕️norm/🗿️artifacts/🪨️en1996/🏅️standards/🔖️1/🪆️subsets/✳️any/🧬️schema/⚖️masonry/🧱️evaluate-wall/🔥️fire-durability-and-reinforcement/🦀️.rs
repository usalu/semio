//! 🔥️ Fire, durability and reinforcement phases: the fire, bed joint, reinforced masonry and basement checks of the wall evaluation. See [`super`].

use super::*;

/// 🔥 Checks the fire resistance of the wall with the combination of actions.
pub(super) fn check_fire(
    annex: AnnexChoice,
    masonry_class: MasonryClass,
    default_situation: DesignSituation,
    wall: &MasonryWall,
    out: &mut Vec<CheckResult>,
    path_t: &str,
    lambda: f64,
    a: f64,
    f_k: f64,
    t: f64,
) {
    {
        let params = AnnexParams::for_document(annex, masonry_class, default_situation);
        let f_d = f_k / params.gamma_m();
        let n_rd_cold = phi_s(annex, lambda).max(phi_m(lambda, 0.05 * t, t)) * f_d * a;
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
            wall_subject(wall, path_t),
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
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t_min.max(t * alpha)),
                    loc("Increase thickness for fire / reduce α.", "Dicke für Brandschutz erhöhen / α reduzieren."),
                ));
        } else {
            b = b.remedy(Remedy::at_least(
                wall_subject(wall, path_t),
                Quantity::length_m(t),
                Quantity::length_m(t_min.max(t)),
                loc("Increase thickness for fire / reduce α.", "Dicke für Brandschutz erhöhen / α reduzieren."),
            ));
        }
        out.push(b.build());
    }
}

/// 💧 Checks the mortar class required by the exposure and the bed joint.
pub(super) fn check_durability(annex: AnnexChoice, wall: &MasonryWall, out: &mut Vec<CheckResult>, path_mortar: &str) {
    {
        let need = required_mortar_mpa(wall.exposure, wall.unit_material);
        let have = wall.mortar_class.f_m_mpa();
        let mut b = CheckResult::assess(
            format!("en1996.2.durability.{}", wall.id),
            "EN 1996-2",
            clause("EN 1996-2", "Annex B", "B"),
            wall_subject(wall, path_mortar),
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
                    wall_subject(wall, path_mortar),
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
}

/// 🪢 Checks the reinforced masonry rules.
pub(super) fn check_reinforced_masonry(annex: AnnexChoice, wall: &MasonryWall, out: &mut Vec<CheckResult>, path_as_h: &str, a: f64) {
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
                    wall_subject(wall, path_as_h),
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
                    wall_subject(wall, path_as_h),
                    Quantity::new(QuantityKind::Area, as_h),
                    Quantity::new(QuantityKind::Area, as_min),
                    loc("Provide minimum reinforcement.", "Mindestbewehrung vorsehen."),
                ));
        }
        out.push(b.build());
    }
}

/// 🏚 Checks the simplified EN 1996-3 rules and the basement earth pressure.
pub(super) fn check_simplified_and_basement(
    annex: AnnexChoice,
    masonry_class: MasonryClass,
    default_situation: DesignSituation,
    storeys: u32,
    wall: &MasonryWall,
    out: &mut Vec<CheckResult>,
    path_t: &str,
    path_basement: &str,
    lambda: f64,
    a: f64,
    f_k: f64,
    t: f64,
) {
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
                    wall_subject(wall, path_t),
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
                wall_subject(wall, path_t),
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
                    wall_subject(wall, path_t),
                    Quantity::length_m(t),
                    Quantity::length_m(t * (n_ed / n_rd.max(1e-12)).max(1.0)),
                    loc("Increase thickness (simplified N_Rd).", "Dicke erhöhen (vereinfachtes N_Rd)."),
                ));
            }
            out.push(b.build());
        }

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
                wall_subject(wall, path_basement),
                loc("Basement wall earth pressure", "Kellerwand Erddruck"),
            )
            .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("N_Ed,min={n_min:.0} N (≥ {n_min_req:.0}); N_Ed,max={n_max:.0} N (≤ {n_rd_s:.0})."),
                &format!("N_Ed,min={n_min:.0} N (≥ {n_min_req:.0}); N_Ed,max={n_max:.0} N (≤ {n_rd_s:.0}) (Kellerwand §4.5)."),
            ));
            b = b.remedy(Remedy::at_least(
                wall_subject(wall, path_t),
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
                    wall_subject(wall, path_basement),
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
}
