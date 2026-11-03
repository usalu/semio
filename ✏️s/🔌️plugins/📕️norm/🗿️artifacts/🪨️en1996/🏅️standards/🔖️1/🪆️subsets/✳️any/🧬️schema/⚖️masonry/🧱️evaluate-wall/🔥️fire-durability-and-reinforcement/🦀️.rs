//! 🔥️ Fire, durability and reinforcement phases: the fire, bed joint, reinforced masonry and basement checks of the wall evaluation. See [`super`].

use super::*;

/// 🔖️ Registry ids (`ndp.en1996.*`) the formulas of each check rest on, by data flow; tabulated cells are cited from the kernel read log and the strength factors from `strength_ndps`.
const FIRE_TABLE: &[&str] = &["ndp.en1996.m90.fire"];
const FIRE_LOAD: &[&str] = &["ndp.en1996.m06.e", "ndp.en1996.m07.gamma", "ndp.en1996.m10.rho2", "ndp.en1996.m12.einit", "ndp.en1996.psi"];
const DURABILITY: &[&str] = &["ndp.en1996.durability.mortar"];
const REINFORCED: &[&str] = &["ndp.en1996.m40.flexure"];
const SIMPLIFIED_LIMITS: &[&str] = &["ndp.en1996.m11.limit", "ndp.en1996.m80.limits"];
const SIMPLIFIED: &[&str] = &["ndp.en1996.m07.gamma", "ndp.en1996.m10.rho2", "ndp.en1996.m11.limit", "ndp.en1996.m80.limits", "ndp.en1996.m81.phis"];
const SLAB_NA: &[&str] = &["ndp.en1996.m13.phi1"];
const BASEMENT: &[&str] = &["ndp.en1996.m07.gamma", "ndp.en1996.m10.rho2", "ndp.en1996.m34.basement", "ndp.en1996.m81.phis"];
use super::load_case_checks::{flexure, strength_ndps};

fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

/// 🔥 Load ratio μ_fi = N_Ed,fi/N_Rd of the wall.
fn fire_load_ratio(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall) -> f64 {
    let gamma_m = AnnexParams::for_document(annex, masonry_class, default_situation).gamma_m();
    let t = wall.thickness_m;
    let n_rd = phi_m(slenderness(wall), 0.05 * t, t, k_e(annex)) * f_k_pa(annex, wall) / gamma_m * area_m2(wall);
    let n_ed_fi = wall
        .load_cases
        .iter()
        .map(|lc| {
            let (g, q_imp, _) = vertical_actions(lc);
            g + self_weight_n(wall) + psi1_imposed(annex, lc.imposed_category.as_str()) * q_imp
        })
        .fold(0.0, f64::max);
    n_ed_fi / n_rd.max(1e-9)
}

/// 🔥 Checks the fire resistance of the wall: the load ratio μ_fi and the tabulated minimum thickness.
pub(super) fn check_fire(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, wall: &MasonryWall, out: &mut Vec<CheckResult>) {
    let path_t = wall_path(wall, "thicknessM");
    let t = wall.thickness_m;
    if wall.fire_rei_min == 0 {
        out.push(
            CheckResult::assess(format!("en1996.1-2.fire.{}", wall.id), "EN 1996-1-2", clause("EN 1996-1-2", "§4", "4"), wall_subject(wall, &wall_path(wall, "fireReiMin")), loc("Fire resistance tabulated", "Feuerwiderstand tabellarisch"))
                .not_applicable(loc("No fire resistance class required.", "Keine Feuerwiderstandsklasse gefordert."))
                .annex(annex)
                .build(),
        );
        return;
    }
    let (mu, load_reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| fire_load_ratio(annex, masonry_class, default_situation, reliability_class, wall));
    let (t_min, table_reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| fire_min_thickness_m(wall.fire_rei_min, wall.unit_material));
    let gamma_ndp = AnnexParams::for_document(annex, masonry_class, default_situation).gamma_m_ndp();
    let text_en = format!("REI {}; μ_fi=N_Ed,fi/N_Rd={mu:.3}; t={t:.3} m; t_min={t_min:.3} m (tabular). Unverified NDP basis: m90.fire (base rows only, Annex B load-ratio columns not keyed).", wall.fire_rei_min);
    let text_de = format!("REI {}; μ_fi=N_Ed,fi/N_Rd={mu:.3}; t={t:.3} m; t_min={t_min:.3} m (tabellarisch). Ungeprüfte NDP-Grundlage: m90.fire (nur Basiszeilen, Spalten des Lastverhältnisses aus Anhang B nicht verschlüsselt).", wall.fire_rei_min);
    let mut b = CheckResult::assess(
        format!("en1996.1-2.fire.{}", wall.id),
        "EN 1996-1-2",
        clause("EN 1996-1-2", "§4", "4"),
        wall_subject(wall, &path_t),
        loc("Fire resistance tabulated thickness", "Feuerwiderstand tabellarische Dicke"),
    )
    .utilization(Quantity::length_m(t_min), Quantity::length_m(t))
    .annex(annex)
    .explanation(loc(&text_en, &text_de));
    if t_min > t {
        let ok = |x: f64| {
            fire_min_thickness_m(wall.fire_rei_min, wall.unit_material) <= x
        };
        if let Some(x) = solve_threshold(t, 3.0, 5e-3, ok) {
            b = b.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase thickness to the tabulated fire value.", "Dicke auf den tabellarischen Brandwert erhöhen.")));
        }
    }
    out.push(b.ndps(FIRE_TABLE).ndps(&table_reads).build());

    let mut bl = CheckResult::assess(
        format!("en1996.1-2.fire.load.{}", wall.id),
        "EN 1996-1-2",
        clause("EN 1996-1-2", "§4", "4"),
        wall_subject(wall, &path_t),
        loc("Fire load ratio μ_fi ≤ 1", "Brandlastverhältnis μ_fi ≤ 1"),
    )
    .utilization(dimensionless(mu), dimensionless(1.0))
    .annex(annex)
    .explanation(loc(&text_en, &text_de));
    if mu > 1.0 {
        let ok = |x: f64| fire_load_ratio(annex, masonry_class, default_situation, reliability_class, &MasonryWall { thickness_m: x, ..wall.clone() }) <= 1.0;
        if let Some(x) = solve_threshold(t, 3.0, 5e-3, ok) {
            bl = bl.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase thickness so that μ_fi ≤ 1.", "Dicke erhöhen, damit μ_fi ≤ 1.")));
        }
    }
    out.push(bl.ndps(FIRE_LOAD).ndps(&load_reads).ndps(&strength_ndps(annex, wall)).ndp(gamma_ndp).build());
}

/// 💧 Checks the mortar class required by the exposure and the bed joint.
pub(super) fn check_durability(annex: AnnexChoice, wall: &MasonryWall, out: &mut Vec<CheckResult>, path_mortar: &str) {
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
        let admissible = [ExposureClass::Mx1, ExposureClass::Mx2, ExposureClass::Mx3, ExposureClass::Mx4]
            .into_iter()
            .filter(|e| required_mortar_mpa(*e, wall.unit_material).is_finite())
            .map(|e| format!("{e:?}"))
            .collect::<Vec<_>>();
        b = b
            .status(CheckStatus::Fail)
            .explanation(loc("Material/exposure combination inadmissible.", "Material/Exposition unzulässig."))
            .remedy(Remedy::one_of(wall_subject(wall, &wall_path(wall, "exposure")), admissible, loc("Select an admissible exposure class.", "Zulässige Expositionsklasse wählen.")));
    } else {
        b = b
            .utilization(Quantity::new(QuantityKind::Stress, need * 1e6), Quantity::new(QuantityKind::Stress, have * 1e6))
            .explanation(loc(
                &format!("Required mortar ≥ {need:.1} MPa for {:?}; have {have:.1} MPa.", wall.exposure),
                &format!("Erforderlicher Mörtel ≥ {need:.1} MPa für {:?}; vorhanden {have:.1} MPa.", wall.exposure),
            ));
        if need > have {
            let options = [MortarClass::M1, MortarClass::M2_5, MortarClass::M5, MortarClass::M10, MortarClass::M15, MortarClass::M20]
                .into_iter()
                .filter(|m| m.f_m_mpa() >= need)
                .map(|m| format!("{m:?}"))
                .collect::<Vec<_>>();
            b = b.remedy(Remedy::one_of(wall_subject(wall, path_mortar), options, loc("Upgrade mortar class.", "Mörtelklasse erhöhen.")));
        }
    }
    out.push(b.ndps(DURABILITY).build());

    let (c, reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| material_conformance(wall));
    let path_joint = wall_path(wall, "bedJointThicknessM");
    let t_joint = wall.bed_joint_thickness_m;
    let mut bj = CheckResult::assess(
        format!("en1996.2.bedjoint.{}", wall.id),
        "EN 1996-2",
        clause("EN 1996-2", "§8", "8"),
        wall_subject(wall, &path_joint),
        loc("Bed-joint thickness", "Lagerfugendicke"),
    )
    .utilization(dimensionless(c.joint), dimensionless(1.0))
    .annex(annex)
    .explanation(loc(
        &format!("t_bed={t_joint:.4} m; admissible {:.4}…{:.4} m for {:?}.", c.joint_range.0, c.joint_range.1, wall.mortar_type),
        &format!("t_Lager={t_joint:.4} m; zulässig {:.4}…{:.4} m für {:?}.", c.joint_range.0, c.joint_range.1, wall.mortar_type),
    ));
    if c.joint > 1.0 {
        bj = bj.remedy(if t_joint > c.joint_range.1 {
            Remedy::at_most(wall_subject(wall, &path_joint), Quantity::length_m(t_joint), Quantity::length_m(c.joint_range.1), loc("Reduce the bed joint to the admissible maximum.", "Lagerfuge auf das zulässige Maximum verringern."))
        } else {
            Remedy::at_least(wall_subject(wall, &path_joint), Quantity::length_m(t_joint), Quantity::length_m(c.joint_range.0), loc("Increase the bed joint to the admissible minimum.", "Lagerfuge auf das zulässige Minimum vergrößern."))
        });
    }
    out.push(bj.ndps(&reads).build());
}

/// 🪢 Checks the reinforced masonry rules: declaration, design strength and the reinforcement ratio limits.
pub(super) fn check_reinforced_masonry(annex: AnnexChoice, wall: &MasonryWall, out: &mut Vec<CheckResult>, path_as_h: &str, a: f64) {
    let as_min = normative_limit(&["reinforcement", "min-ratio"]) * a;
    let as_max = normative_limit(&["reinforcement", "max-ratio"]) * a;
    let as_have = wall.as_vertical_m2.max(0.0) + wall.as_horizontal_m2.max(0.0);
    let path_reinforced = wall_path(wall, "reinforced");
    let mut b = CheckResult::assess(
        format!("en1996.6.6.reinforced.{}", wall.id),
        "EN 1996-1-1",
        clause("EN 1996-1-1", "§6.6", "6.6"),
        wall_subject(wall, &wall_path(wall, "asVerticalM2")),
        loc("Reinforced masonry As / f_yd", "Bewehrtes Mauerwerk As / f_yd"),
    )
    .annex(annex)
    .explanation(loc(
        &format!("reinforced={}; As={as_have:.6} m² within [{as_min:.6}, {as_max:.6}]; f_yd={:.0} Pa.", wall.reinforced, wall.f_yd_pa),
        &format!("bewehrt={}; As={as_have:.6} m² in [{as_min:.6}, {as_max:.6}]; f_yd={:.0} Pa.", wall.reinforced, wall.f_yd_pa),
    ));
    if !wall.reinforced {
        if as_have > 0.0 || wall.f_yd_pa > 0.0 {
            b = b.status(CheckStatus::Fail).remedy(Remedy::exactly(
                wall_subject(wall, &path_reinforced),
                dimensionless(0.0),
                dimensionless(1.0),
                loc("Set reinforced=true when As or f_yd is provided.", "reinforced=true setzen, wenn As oder f_yd angegeben."),
            ));
        } else {
            b = b.not_applicable(loc("Unreinforced masonry.", "Unbewehrtes Mauerwerk."));
        }
    } else if wall.f_yd_pa <= 0.0 {
        b = b.status(CheckStatus::Fail).remedy(Remedy::at_least(
            wall_subject(wall, &wall_path(wall, "fYdPa")),
            Quantity::new(QuantityKind::Stress, wall.f_yd_pa),
            Quantity::new(QuantityKind::Stress, f_yd_b500_pa(annex)),
            loc("Declare f_yd = f_yk/γ_S of the reinforcement.", "f_yd = f_yk/γ_S der Bewehrung angeben."),
        ));
    } else {
        let u = (as_min / as_have.max(1e-12)).max(as_have / as_max.max(1e-12));
        b = b.utilization(dimensionless(u), dimensionless(1.0));
        if as_have < as_min {
            b = b.remedy(Remedy::at_least(wall_subject(wall, path_as_h), Quantity::new(QuantityKind::Area, wall.as_horizontal_m2), Quantity::new(QuantityKind::Area, ceil_to(wall.as_horizontal_m2 + as_min - as_have, 1e-6)), loc("Provide minimum reinforcement.", "Mindestbewehrung vorsehen.")));
        } else if as_have > as_max {
            b = b.remedy(Remedy::at_most(wall_subject(wall, path_as_h), Quantity::new(QuantityKind::Area, wall.as_horizontal_m2), Quantity::new(QuantityKind::Area, (wall.as_horizontal_m2 - (as_have - as_max)).max(0.0)), loc("Reduce reinforcement to the maximum ratio.", "Bewehrung auf den Höchstgrad reduzieren.")));
        }
    }
    out.push(b.ndps(REINFORCED).build());
}

/// 🏚 Checks the simplified EN 1996-3 rules and the basement earth pressure.
pub(super) fn check_simplified_and_basement(annex: AnnexChoice, masonry_class: MasonryClass, default_situation: DesignSituation, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, storeys: u32, wall: &MasonryWall, out: &mut Vec<CheckResult>) {
    let part = "DIN EN 1996-3";
    let path_t = wall_path(wall, "thicknessM");
    let path_basement = wall_path(wall, "isBasement");
    let t = wall.thickness_m;
    let lambda = slenderness(wall);
    let a = area_m2(wall);
    let params = AnnexParams::for_document(annex, masonry_class, default_situation);
    let (gamma_m, gamma_ndp) = (params.gamma_m(), params.gamma_m_ndp());
    let f_d = f_k_pa(annex, wall) / gamma_m;
    let mut reasons: Vec<(String, String)> = Vec::new();
    let mut reason = |en: String, de: String| reasons.push((en, de));
    let building_h = storeys as f64 * wall.height_m;
    let limit = |key: &str| normative_limit(&["simplified", key]);
    if building_h > limit("building-height") {
        reason(format!("building height≈{building_h:.1} m > {:.0} m", limit("building-height")), format!("Gebäudehöhe≈{building_h:.1} m > {:.0} m", limit("building-height")));
    }
    if lambda > limit("slenderness") {
        reason(format!("h_ef/t={lambda:.1}>{:.0}", limit("slenderness")), format!("h_ef/t={lambda:.1}>{:.0}", limit("slenderness")));
    }
    if !matches!(wall.wall_type, WallType::LoadBearing) {
        reason("wall not load-bearing".into(), "Wand nicht tragend".into());
    }
    let q_k_max = wall.load_cases.iter().map(|lc| lc.q_k_imposed_pa).fold(0.0, f64::max);
    if q_k_max > limit("imposed-load") {
        reason(format!("q_k={q_k_max:.0} Pa > {:.0} kN/m²", limit("imposed-load") / 1000.0), format!("q_k={q_k_max:.0} Pa > {:.0} kN/m²", limit("imposed-load") / 1000.0));
    }
    let span_max = wall.load_cases.iter().map(|lc| lc.slab_span_m).fold(0.0, f64::max);
    if span_max > limit("floor-span") {
        reason(format!("slab span={span_max:.2} m > {:.0} m", limit("floor-span")), format!("Deckenstützweite={span_max:.2} m > {:.0} m", limit("floor-span")));
    }
    let t_min = if storeys as f64 > limit("storey-threshold") { limit("min-thickness-tall") } else { limit("min-thickness") };
    if t < t_min {
        reason(format!("t={t:.3} m < {:.0} mm", t_min * 1000.0), format!("t={t:.3} m < {:.0} mm", t_min * 1000.0));
    }
    let h_max = if t >= limit("thick-wall") { limit("height-ratio") * t } else { limit("height-thin") };
    if wall.height_m > h_max {
        reason(format!("clear height {:.2} m > {h_max:.2} m", wall.height_m), format!("lichte Höhe {:.2} m > {h_max:.2} m", wall.height_m));
    }
    if wall.length_m < limit("min-length") || wall.length_m * t < limit("min-area") {
        reason(format!("wall length < {} m or area < {} m²", limit("min-length"), limit("min-area")), format!("Wandlänge < {} m oder Fläche < {} m²", limit("min-length").to_string().replace('.', ","), limit("min-area").to_string().replace('.', ",")));
    }
    if !reasons.is_empty() {
        let en = reasons.iter().map(|r| r.0.clone()).collect::<Vec<_>>().join("; ");
        let de = reasons.iter().map(|r| r.1.clone()).collect::<Vec<_>>().join("; ");
        out.push(
            CheckResult::assess(format!("en1996.3.simplified.{}", wall.id), part, clause("EN 1996-3", "§4.2", "4.2"), wall_subject(wall, &path_t), loc("Simplified method applicability", "Anwendbarkeit vereinfachtes Verfahren"))
                .not_applicable(loc(&format!("Outside the EN 1996-3 applicability limits: {en}."), &format!("Außerhalb der Anwendungsgrenzen EN 1996-3: {de}.")))
                .annex(annex)
                .ndps(SIMPLIFIED_LIMITS)
                .build(),
        );
    } else {
        let simplified = |w: &MasonryWall| -> (f64, f64, String, String) {
            let (n_ed, en, de) = w.load_cases.iter().fold((0.0_f64, String::new(), String::new()), |acc, lc| {
                let eff = design_effects(annex, w, lc, default_situation, reliability_class);
                let ge = governing_compression(&eff);
                if ge.n_bot > acc.0 { (ge.n_bot, ge.combo_en.clone(), ge.combo_de.clone()) } else { acc }
            });
            let phi = phi_s(slenderness(w)).min(phi_2_slab_span(annex, span_max));
            (n_ed, phi * f_k_pa(annex, w) / gamma_m * area_m2(w), en, de)
        };
        let ((n_ed, n_rd, combo_en, combo_de), simplified_reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| simplified(wall));
        let mut b = CheckResult::assess(format!("en1996.3.simplified.{}", wall.id), part, clause("EN 1996-3", "§4.2", "4.2"), wall_subject(wall, &path_t), loc("Simplified compression N_Rd", "Vereinfachte Drucktragfähigkeit N_Rd"))
            .utilization(Quantity::new(QuantityKind::Force, n_ed), Quantity::new(QuantityKind::Force, n_rd.max(1e-9)))
            .annex(annex)
            .explanation(loc(
                &format!("{combo_en}; Φ_s=min(Φ₁,Φ₂)={:.3}; N_Rd={n_rd:.0} N; N_Ed={n_ed:.0} N.", n_rd / (f_d * a).max(1e-9)),
                &format!("{combo_de}; Φ_s=min(Φ₁,Φ₂)={:.3}; N_Rd={n_rd:.0} N; N_Ed={n_ed:.0} N (vereinfacht).", n_rd / (f_d * a).max(1e-9)),
            ));
        if n_ed > n_rd {
            let ok = |x: f64| {
                let (e, r, _, _) = simplified(&MasonryWall { thickness_m: x, ..wall.clone() });
                e <= r
            };
            if let Some(x) = solve_threshold(t, 3.0, 1e-3, ok) {
                b = b.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Increase thickness (simplified N_Rd).", "Dicke erhöhen (vereinfachtes N_Rd).")));
            }
        }
        out.push(b.ndps(SIMPLIFIED).ndps(&simplified_reads).ndps(&strength_ndps(annex, wall)).ndps(if annex == AnnexChoice::De { SLAB_NA } else { &[] }).ndp(gamma_ndp).build());
    }

    if wall.is_basement {
        let basement = |w: &MasonryWall| -> (f64, f64, f64, f64, f64) {
            let mut n_min = f64::INFINITY;
            let mut n_max = 0.0_f64;
            let mut m_ed = 0.0_f64;
            for lc in &w.load_cases {
                let f = flexure(annex, gamma_m, default_situation, reliability_class, w, lc);
                m_ed = m_ed.max(f.m_ed);
                for e in design_effects(annex, w, lc, default_situation, reliability_class) {
                    n_min = n_min.min(e.n_mid);
                    n_max = n_max.max(e.n_bot);
                }
            }
            let n_min = if n_min.is_finite() { n_min } else { 0.0 };
            let f_d = f_k_pa(annex, w) / gamma_m;
            let l_ef = effective_length_m(w);
            let m_rd = n_min * (w.thickness_m - n_min / (f_d * l_ef).max(1e-9)) / 2.0;
            let n_rd = phi_s(slenderness(w)) * f_d * area_m2(w);
            ((m_ed / m_rd.max(1e-9)).max(n_max / n_rd.max(1e-9)), m_ed, m_rd, n_min, n_max)
        };
        let ((u, m_ed, m_rd, n_min, n_max), basement_reads) = semio_s_plugin_norm_compliance::record_ndp_reads(|| basement(wall));
        let mut b = CheckResult::assess(format!("en1996.3.basement.{}", wall.id), part, clause("EN 1996-3", "§4.5", "4.5"), wall_subject(wall, &path_basement), loc("Basement wall earth pressure", "Kellerwand Erddruck"))
            .utilization(dimensionless(u), dimensionless(1.0))
            .annex(annex)
            .explanation(loc(
                &format!("M_Ed={m_ed:.0} N·m ≤ M_Rd(N_min)=N·(t − N/(f_d ℓ))/2={m_rd:.0} N·m with N_min={n_min:.0} N; N_Ed,max={n_max:.0} N."),
                &format!("M_Ed={m_ed:.0} N·m ≤ M_Rd(N_min)=N·(t − N/(f_d ℓ))/2={m_rd:.0} N·m mit N_min={n_min:.0} N; N_Ed,max={n_max:.0} N (Kellerwand §4.5)."),
            ));
        if u > 1.0 {
            let ok = |x: f64| basement(&MasonryWall { thickness_m: x, ..wall.clone() }).0 <= 1.0;
            if let Some(x) = solve_threshold(t, 3.0, 1e-3, ok) {
                b = b.remedy(Remedy::at_least(wall_subject(wall, &path_t), Quantity::length_m(t), Quantity::length_m(x), loc("Thicken the basement wall.", "Kellerwand verdicken.")));
            }
        }
        out.push(b.ndps(BASEMENT).ndps(&basement_reads.into_iter().filter(|id| !id.starts_with("ndp.en1996.fxk")).collect::<Vec<_>>()).ndps(&strength_ndps(annex, wall)).ndp(gamma_ndp).build());
    } else if wall.load_cases.iter().any(|lc| lc.h_k_earth_n > 0.0) {
        out.push(
            CheckResult::assess(format!("en1996.3.basement.{}", wall.id), part, clause("EN 1996-3", "§4.5", "4.5"), wall_subject(wall, &path_basement), loc("Basement wall earth pressure", "Kellerwand Erddruck"))
                .not_applicable(loc("Earth pressure given but isBasement=false — mark wall as basement or clear h_k_earth.", "Erddruck angegeben, aber isBasement=false — als Kellerwand kennzeichnen oder h_k_earth löschen."))
                .annex(annex)
                .build(),
        );
    }
}
