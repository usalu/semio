//! 🧮️ Document cases of the EN 1996 masonry checks: the catalogue entries that run from the wall model, bound to the wall fields so that every result and remedy resolves in the document. See [`super`].

use super::*;

#[path = "🧰️members/🦀️.rs"]
mod members;
pub use members::bracing_case;

fn material_key(m: UnitMaterial) -> &'static str {
    match m {
        UnitMaterial::Clay => "clay",
        UnitMaterial::CalciumSilicate => "calcium_silicate",
        UnitMaterial::Aerated => "aerated",
        UnitMaterial::Concrete => "concrete",
    }
}

fn mortar_key(m: MortarType) -> &'static str {
    match m {
        MortarType::GeneralPurpose => "gp",
        MortarType::ThinLayer => "tl",
        MortarType::Lightweight => "lw",
    }
}

fn group_number(g: UnitGroup) -> f64 {
    match g {
        UnitGroup::Group1 => 1.0,
        UnitGroup::Group2 => 2.0,
        UnitGroup::Group3 => 3.0,
        UnitGroup::Group4 => 4.0,
    }
}

fn class_number(c: MasonryClass) -> f64 {
    match c {
        MasonryClass::Class1 => 1.0,
        MasonryClass::Class2 => 2.0,
        MasonryClass::Class3 => 3.0,
        MasonryClass::Class4 => 4.0,
        MasonryClass::Class5 => 5.0,
    }
}

fn edge_code(wall: &MasonryWall) -> &'static str {
    match wall.support_sides {
        2 => "SSFF",
        3 => "SSSF",
        _ => "SSSS",
    }
}

/// 🧱️ The catalogue entries evaluated for one wall: strength chain, restraint and slenderness, alternative lateral resistance (two-way yield lines, arching), reinforced bending, movement joints and minimum thickness.
pub fn document_cases(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, storeys: u32, wall: &MasonryWall) -> Vec<(&'static str, Case)> {
    let gamma_m = AnnexParams::for_document(annex, masonry_class, default_situation).gamma_m();
    let t_path = wall_path(wall, "thicknessM");
    let bound = |suffix: &str| Case::new(&format!("{}{suffix}", wall.id), annex).bound(&wall.id, &t_path, loc(&wall.label_en, &wall.label_de)).with_path("t_m", &t_path);
    let strength = |c: Case| {
        c.with_n("fb_pa", f_b_normalised_pa(wall))
            .with_n("fm_pa", f_m_mpa(wall) * 1e6)
            .with_s("material", material_key(wall.unit_material))
            .with_s("mortar", mortar_key(wall.mortar_type))
            .with_n("group", group_number(wall.unit_group))
            .with_n("gamma_m", gamma_m)
    };
    let f_k = f_k_pa(annex, wall);
    let mut out: Vec<(&'static str, Case)> = vec![
        ("M-01", bound("").with_n("fb_decl_pa", wall.f_b_pa).with_n("unit_h_m", wall.unit_height_m).with_n("unit_w_m", wall.unit_width_m.min(wall.unit_length_m))),
        ("M-02", strength(bound(""))),
        ("M-04", bound("").with_n("fm_pa", f_m_mpa(wall) * 1e6).with_s("material", material_key(wall.unit_material)).with_n("gamma_m", gamma_m)),
        ("M-06", bound("").with_n("fk_pa", f_k).with_s("material", material_key(wall.unit_material))),
        ("M-10", bound("").with_n("h_m", wall.height_m).with_n("t_m", wall.thickness_m).with_n("sides", wall.support_sides as f64).with_n("l_m", wall.length_m).with_n("bearing_m", wall.slab_bearing_depth_m)),
        ("M-52", bound("").with_n("spacing_m", wall.length_m).with_s("material", material_key(wall.unit_material)).with_path("spacing_m", &wall_path(wall, "lengthM"))),
    ];
    if matches!(wall.wall_type, WallType::LoadBearing) {
        let mut execution = bound("")
            .with_path("bed_joint_m", &wall_path(wall, "bedJointThicknessM"))
            .with_n("t_m", wall.thickness_m)
            .with_n("bed_joint_m", wall.bed_joint_thickness_m)
            .with_n("unit_h_m", wall.unit_height_m)
            .with_s("mortar", mortar_key(wall.mortar_type));
        if wall.reinforced {
            if let Some(d) = wall.bars.iter().filter(|b| matches!(b.kind, BarKind::Shear)).map(|b| b.diameter_m).reduce(f64::max) {
                execution = execution.with_n("diameter_m", d);
            }
        }
        out.push(("M-70", execution));
    }
    for lc in &wall.load_cases {
        let w_k = (lc.q_p_wind_pa * lc.c_pe.abs()).max(0.0);
        let sit = DesignSituation::from(lc.design_situation);
        let exceptional = matches!(sit, DesignSituation::Accidental | DesignSituation::Seismic) || matches!(default_situation, DesignSituation::Accidental);
        let w_ed = gamma_q(annex, reliability_class, exceptional) * w_k;
        let n_min = design_effects(annex, wall, lc, default_situation, reliability_class).iter().map(|e| e.n_mid).fold(f64::INFINITY, f64::min);
        let sigma_d = (n_min / area_m2(wall).max(1e-9)).max(0.0);
        let f_d = f_k / gamma_m;
        if w_ed > 0.0 {
            let lateral = |suffix: &str| {
                bound(&format!(".{}{suffix}", lc.id))
                    .advisory()
                    .with_n("t_m", wall.thickness_m)
                    .with_n("h_m", wall.height_m)
                    .with_n("l_m", effective_length_m(wall))
                    .with_n("w_ed_pa", w_ed)
                    .with_n("gamma_m", gamma_m)
                    .with_n("sigma_d_pa", sigma_d.min(0.2 * f_d))
            };
            out.push(("M-31", lateral("").with_n("fxk1_pa", f_xk_pa(1, wall)).with_n("fxk2_pa", f_xk_pa(2, wall)).with_s("edges", edge_code(wall))));
            out.push(("M-32", lateral(".arch").with_n("la_m", wall.height_m).with_n("fd_pa", f_d)));
            out.push(("M-33", bound(&format!(".{}.load", lc.id)).with_n("qp_pa", lc.q_p_wind_pa).with_n("cpe", lc.c_pe)));
            out.push(("M-83", lateral(".simple").with_n("fxk1_pa", f_xk_pa(1, wall))));
        }
    }
    let span_max = wall.load_cases.iter().map(|lc| lc.slab_span_m).fold(0.0, f64::max);
    let q_max = wall.load_cases.iter().map(|lc| lc.q_k_imposed_pa).fold(0.0, f64::max);
    out.push((
        "M-80",
        bound(".simplified")
            .advisory()
            .with_path("h_m", &wall_path(wall, "heightM"))
            .with_path("l_m", &wall_path(wall, "lengthM"))
            .with_n("t_m", wall.thickness_m)
            .with_n("h_m", wall.height_m)
            .with_n("l_m", wall.length_m)
            .with_n("storeys", f64::from(storeys))
            .with_n("height_total_m", f64::from(storeys) * wall.height_m)
            .with_n("span_m", span_max)
            .with_n("qk_pa", q_max),
    ));
    let n_bot = wall.load_cases.iter().flat_map(|lc| design_effects(annex, wall, lc, default_situation, reliability_class)).map(|e| e.n_bot).fold(0.0, f64::max);
    let n_min = wall.load_cases.iter().flat_map(|lc| design_effects(annex, wall, lc, default_situation, reliability_class)).map(|e| e.n_mid).fold(f64::INFINITY, f64::min);
    let sigma_min = if n_min.is_finite() { (n_min / area_m2(wall).max(1e-9)).max(0.0) } else { 0.0 };
    if matches!(wall.wall_type, WallType::Shear) && n_bot > 0.0 {
        out.push((
            "M-81",
            bound(".simplified-vertical")
                .advisory()
                .with_n("t_m", wall.thickness_m)
                .with_n("hef_m", h_ef_m(wall))
                .with_n("span_m", span_max)
                .with_n("fk_pa", f_k)
                .with_n("gamma_m", gamma_m)
                .with_n("n_ed_n", n_bot / wall.length_m.max(1e-9)),
        ));
    }
    if matches!(wall.wall_type, WallType::Shear) && wall.in_plane_shear_n > 0.0 {
        out.push((
            "M-82",
            bound(".simplified-shear")
                .advisory()
                .with_path("lc_m", &wall_path(wall, "lengthM"))
                .with_path("v_ed_n", &wall_path(wall, "inPlaneShearN"))
                .with_n("lc_m", wall.length_m)
                .with_n("t_m", wall.thickness_m)
                .with_n("sigma_d_pa", sigma_min)
                .with_n("fvk0_pa", f_vk0_mpa(annex, wall.unit_material, wall.mortar_type, wall.mortar_class) * 1e6)
                .with_n("gamma_m", gamma_m)
                .with_n("v_ed_n", wall.in_plane_shear_n),
        ));
    }
    if wall.reinforced && wall.as_vertical_m2 > 0.0 && wall.f_yd_pa > 0.0 && wall.effective_depth_m > 0.0 {
        let l_ef = effective_length_m(wall);
        let m_ed = wall
            .load_cases
            .iter()
            .map(|lc| {
                let exceptional = matches!(DesignSituation::from(lc.design_situation), DesignSituation::Accidental | DesignSituation::Seismic) || matches!(default_situation, DesignSituation::Accidental);
                gamma_q(annex, reliability_class, exceptional) * (lc.q_p_wind_pa * lc.c_pe.abs()) * wall.height_m.powi(2) / 8.0
            })
            .fold(0.0, f64::max);
        out.push((
            "M-40",
            bound("")
                .with_n("b_m", 1.0)
                .with_n("d_m", wall.effective_depth_m)
                .with_n("as_m2", wall.as_vertical_m2 / l_ef)
                .with_n("fyd_pa", wall.f_yd_pa)
                .with_n("fd_pa", f_k / gamma_m)
                .with_n("m_ed_nm", m_ed)
,
        ));
    }
    out.extend(members::member_cases(annex, default_situation, reliability_class, wall, gamma_m, f_k));
    out.extend(cross_checks(annex, masonry_class, default_situation, reliability_class, wall, gamma_m, f_k, sigma_min));
    out
}

/// 🔁️ The catalogue derivations that restate a built-in wall check from the wall (shear strength, partial factors, slenderness, Φ at top, mid-height and bottom, wind with vertical load, concentrated loads, one-way span, tabulated fire): advisory, so they trace the clause without double-reporting a failure.
fn cross_checks(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, gamma_m: f64, f_k: f64, sigma_min: f64) -> Vec<(&'static str, Case)> {
    let t_path = wall_path(wall, "thicknessM");
    let bound = |suffix: &str| Case::new(&format!("{}{suffix}", wall.id), annex).bound(&wall.id, &t_path, loc(&wall.label_en, &wall.label_de)).with_path("t_m", &t_path).advisory();
    let (f_d, hef, l_net) = (f_k / gamma_m, h_ef_m(wall), (area_m2(wall) / wall.thickness_m.max(1e-9)).max(1e-9));
    let situation = if matches!(default_situation, DesignSituation::Accidental) { "accidental" } else { "persistent" };
    let mut out: Vec<(&'static str, Case)> = vec![
        ("M-03", bound(".shear-strength").with_n("fb_pa", f_b_normalised_pa(wall)).with_n("fm_pa", f_m_mpa(wall) * 1e6).with_n("sigma_pa", sigma_min).with_s("material", material_key(wall.unit_material)).with_s("mortar", mortar_key(wall.mortar_type)).with_n("group", group_number(wall.unit_group))),
        ("M-07", bound(".partial-factors").with_n("class", class_number(masonry_class)).with_s("situation", situation)),
        ("M-11", bound(".slenderness").with_n("hef_m", hef).with_n("t_m", t_ef_m(wall))),
    ];
    if let Some(lc) = wall.load_cases.iter().max_by(|a, b| design_effects(annex, wall, a, default_situation, reliability_class)[0].n_bot.total_cmp(&design_effects(annex, wall, b, default_situation, reliability_class)[0].n_bot)) {
        let e = &design_effects(annex, wall, lc, default_situation, reliability_class)[0];
        let (n_top, n_mid, n_bot) = (e.n_top / l_net, e.n_mid / l_net, e.n_bot / l_net);
        let (e_top, e_bot) = (wall.eccentricity_top_m + eccentricity_from_slab_bearing_m(wall), wall.eccentricity_bottom_m);
        let sections = |c: Case| c.with_n("t_m", wall.thickness_m).with_n("hef_m", hef).with_n("fd_pa", f_d).with_n("n_top_n", n_top).with_n("n_bot_n", n_bot).with_n("m_top_nm", n_top * e_top).with_n("m_bot_nm", n_bot * e_bot);
        let w = e.w_pa;
        out.push(("M-12", sections(bound(&format!(".{}.sections", lc.id)))));
        out.push(("M-13", sections(bound(&format!(".{}.mid", lc.id))).with_n("n_mid_n", n_mid).with_n("m_hm_nm", w * wall.height_m.powi(2) / 8.0).with_n("phi_inf", wall.phi_infinity).with_n("k_e", k_e(annex))));
        if w > 0.0 {
            out.push(("M-16", bound(&format!(".{}.wind", lc.id)).with_n("hef_m", hef).with_n("fd_pa", f_d).with_n("n_mid_n", n_mid).with_n("w_ed_pa", w).with_n("h_m", wall.height_m).with_n("phi_inf", wall.phi_infinity)));
            out.push(("M-30", bound(&format!(".{}.oneway", lc.id)).with_n("t_m", wall.thickness_m).with_n("h_m", wall.height_m).with_n("w_ed_pa", w).with_n("gamma_m", gamma_m).with_n("fxk1_pa", f_xk_pa(1, wall)).with_n("sigma_d_pa", sigma_min.min(0.2 * f_d))));
        }
        for c in lc.concentrated.iter().filter(|c| c.bearing_area_m2 > 0.0) {
            let spread = (c.bearing_length_m + wall.height_m / 3f64.sqrt()).min(wall.length_m);
            out.push((
                "M-14",
                bound(&format!(".{}.{}.concentrated", lc.id, c.id))
                    .with_path("ab_m2", &concentrated_path(wall, lc, &c.id, "bearingAreaM2"))
                    .with_n("ab_m2", c.bearing_area_m2)
                    .with_n("a1_m", c.distance_a1_m)
                    .with_n("hc_m", wall.height_m)
                    .with_n("lefm_m", spread)
                    .with_n("fd_pa", f_d)
                    .with_n("ned_c_n", c.force_n * action_factors(annex, reliability_class, false).gamma_g_sup()),
            ));
        }
    }
    if wall.fire_rei_min > 0 {
        out.push(("M-90", bound(".tabulated-fire").with_n("rei", f64::from(wall.fire_rei_min)).with_s("material", material_key(wall.unit_material)).with_n("t_m", wall.thickness_m)));
    }
    out
}

/// 🧮️ Results of the document cases of one wall, as ordinary check results.
pub fn document_case_results(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, storeys: u32, wall: &MasonryWall) -> Vec<CheckResult> {
    document_cases(annex, masonry_class, default_situation, reliability_class, storeys, wall).into_iter().flat_map(|(id, case)| evaluate_case(id, &case)).collect()
}
