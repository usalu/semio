//! 🧩️ Building details: geometry-derived regularity, seismic joint, non-structural element and capacity design checks, and the v_s,30 ground classification of the site. See [`super`].

use super::*;

type Building = semio_s_artifact_norm_en1998_model::En1998Building;

fn status_of(ok: bool) -> semio_s_plugin_norm_compliance::CheckStatus {
    if ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail }
}

/// 🌍️ Checks the ground type derived by Table 3.1 from the site inputs (v_s,30, N_SPT, c_u, soft clay, liquefiable and alluvium fields) against the stated EN ground type or DE-NA ground combination; S1 and S2 have no spectrum letter and need a special study (EN 1998-1 3.1.2(4)).
pub(super) fn check_site_ground(report: &mut CheckReport, document: &En1998Snapshot, annex: AnnexChoice) {
    let (v, source) = part_1::effective_v_s30(document);
    let Some(derived) = part_1::ground_type_table_3_1(&part_1::ground_inputs_of_site(&document.site, v)) else { return };
    let class = derived.spectrum_letter();
    let (stated, path) = if matches!(annex, AnnexChoice::De) {
        (format!("{:?}", na_de::GroundCombo::from(document.site.de_ground_combo)).chars().next().unwrap_or('?'), "site.deGroundCombo")
    } else {
        (document.site.en_ground_type.letter(), "site.enGroundType")
    };
    let ok = class.is_none() || class == Some(stated);
    let name = derived.name();
    let mut check = CheckResult::assess(
        "en1998.site.groundClass",
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "3.1.2"),
        SubjectRef::new("", "site.vS30MS", lc("v_s,30", "v_s,30")),
        lc("Ground type from the site inputs (Table 3.1)", "Baugrundklasse aus den Standorteingaben (Tab. 3.1)"),
    )
    .status(if class.is_none() { semio_s_plugin_norm_compliance::CheckStatus::Warning } else { status_of(ok) })
    .annex(annex)
    .ndps(&part_1::GROUND_NDP_IDS)
    .explanation(lc(
        &format!("v_s,30 = {v:.0} m/s ({}), N_SPT = {:.0}, c_u = {:.0} kPa give ground type {name}; stated class {stated}{}", match &source { part_1::GroundSource::FoundationLayers { foundation_id } => format!("soil layers of {foundation_id}"), part_1::GroundSource::Site => "site input".to_owned() }, document.site.n_spt, document.site.c_u_kpa, if class.is_none() { "; no spectrum letter, special study required" } else { "" }),
        &format!("v_s,30 = {v:.0} m/s ({}), N_SPT = {:.0}, c_u = {:.0} kPa ergeben Baugrundklasse {name}; angegebene Klasse {stated}{}", match &source { part_1::GroundSource::FoundationLayers { foundation_id } => format!("Bodenschichten von {foundation_id}"), part_1::GroundSource::Site => "Standorteingabe".to_owned() }, document.site.n_spt, document.site.c_u_kpa, if class.is_none() { "; kein Spektrumbuchstabe, Sonderuntersuchung erforderlich" } else { "" }),
    ));
    if !ok {
        let letter = class.unwrap_or('?');
        let options: Vec<String> = if matches!(annex, AnnexChoice::De) {
            ["A-R", "B-R", "C-R", "B-T", "C-T", "C-S"].iter().filter(|c| c.starts_with(letter)).map(|c| (*c).to_string()).collect()
        } else {
            vec![letter.to_string()]
        };
        check = check.remedy(Remedy::one_of(
            SubjectRef::new("", path, lc("Ground class", "Baugrundklasse")),
            options,
            lc("Choose the ground class that matches the site inputs or correct them (boundary values take the lower class).", "Baugrundklasse passend zu den Standorteingaben wählen oder diese korrigieren (Grenzwerte gehören zur unteren Klasse)."),
        ));
    }
    report.push(check.build());
}

/// 🎖️ Reports the importance factor γ_I of the site (EN 1998-1 4.2.5, Table 4.3) and the reduction factor ν of the damage limitation used by the building against the recommended value of the importance class (SEI-10).
pub(super) fn check_importance(report: &mut CheckReport, annex: AnnexChoice, gamma_i: f64, building: &Building, bid: &str, bpath: &str, blabel: &LocalizedCopy) {
    let (id, classes) = if gamma_i >= 1.2 - 1e-9 { ("ndp.en1998.en1.importance.nu-iii-iv", "III, IV") } else { ("ndp.en1998.en1.importance.nu-i-ii", "I, II") };
    let recommended = semio_s_plugin_norm_compliance::ndp_number(id, annex);
    let differs = (building.nu - recommended).abs() > 1e-9;
    report.push(
        CheckResult::assess(format!("en1998.1.{bid}.importance"), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "4.2.5"), SubjectRef::new(bid, format!("{bpath}.nu"), blabel.clone()), lc("Importance class: γ_I and damage limitation factor ν", "Bedeutungskategorie: γ_I und Abminderungsbeiwert ν der Schadensbegrenzung"))
            .informative(q_dim(gamma_i))
            .annex(annex)
            .explanation(lc(
                &format!("γ_I = {gamma_i:.2} (importance classes {classes}); ν = {:.2}, recommended {recommended:.2}{}", building.nu, if differs { " (differs, e.g. by a national choice)" } else { "" }),
                &format!("γ_I = {gamma_i:.2} (Bedeutungskategorien {classes}); ν = {:.2}, empfohlen {recommended:.2}{}", building.nu, if differs { " (weicht ab, z. B. durch nationale Festlegung)" } else { "" }),
            ))
            .ndp(id)
            .build(),
    );
}

/// 🧩️ Checks the plan and elevation geometry, the joint to the next building, the non-structural element and the capacity design node of one building.
#[allow(clippy::too_many_arguments)]
pub(super) fn check_building_details(
    annex: AnnexChoice,
    report: &mut CheckReport,
    gamma_i: f64,
    a_g: f64,
    s: f64,
    building: &Building,
    next: Option<&Building>,
    bid: &str,
    bpath: &str,
    blabel: &LocalizedCopy,
    t1: f64,
) {
    check_importance(report, annex, gamma_i, building, bid, bpath, blabel);
    let total_height: f64 = building.storeys.iter().map(|x| x.height_m).sum();
    let slender = part_1::plan_slenderness_utilization(building.plan_width_m, building.plan_length_m);
    let recess = part_1::plan_envelope_utilization(building.plan_envelope_gap_ratio);
    let setback = part_1::setback_utilization(building.setback_ratio);
    let geometry = [
        ("plan.slenderness", "4.2.3.2", slender, "planWidthM", lc("Plan slenderness λ = Lmax / Lmin ≤ 4", "Grundrissschlankheit λ = Lmax / Lmin ≤ 4"), lc("Shorten the longer plan side or separate the building by a seismic joint.", "Längere Grundrissseite kürzen oder das Gebäude durch eine Erdbebenfuge trennen.")),
        ("plan.envelope", "4.2.3.2", recess, "planEnvelopeGapRatio", lc("Plan recess: area outside the outline within the convex envelope ≤ 5 %", "Grundrissaussparung: Fläche zwischen Grundriss und konvexer Hülle ≤ 5 %"), lc("Reduce the recess or re-entrant corner so that it stays within 5 % of the floor area.", "Aussparung oder einspringende Ecke auf höchstens 5 % der Geschossfläche verkleinern.")),
        ("elevation.setback", "4.2.3.3", setback, "setbackRatio", lc("Set-back per floor ≤ 20 % of the plan dimension below", "Rücksprung je Geschoss ≤ 20 % der darunterliegenden Grundrissabmessung"), lc("Reduce the set-back to at most 20 % of the plan dimension below.", "Rücksprung auf höchstens 20 % der darunterliegenden Grundrissabmessung verringern.")),
    ];
    for (tag, clause, util, field, title, action) in geometry {
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.{tag}"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", clause),
            SubjectRef::new(bid, format!("{bpath}.{field}"), blabel.clone()),
            title,
        )
        .utilization(q_dim(util.min(1e9)), q_dim(1.0))
        .annex(annex)
        .explanation(lc(&format!("utilisation {util:.3} (limit 1)"), &format!("Ausnutzung {util:.3} (Grenze 1)")));
        if util > 1.0 && field != "planWidthM" {
            let current = if field == "planEnvelopeGapRatio" { building.plan_envelope_gap_ratio } else { building.setback_ratio };
            check = check.remedy(Remedy::at_most(SubjectRef::new(bid, format!("{bpath}.{field}"), lc("Geometry ratio", "Geometrieanteil")), q_dim(current), q_dim(if field == "planEnvelopeGapRatio" { 0.05 } else { 0.20 }), action));
        } else if util > 1.0 {
            let (long_field, long_value) = if building.plan_width_m >= building.plan_length_m { ("planWidthM", building.plan_width_m) } else { ("planLengthM", building.plan_length_m) };
            check = check.remedy(Remedy::at_most(SubjectRef::new(bid, format!("{bpath}.{long_field}"), lc("Longer plan side", "Längere Grundrissseite")), q_len(long_value), q_len(4.0 * building.plan_width_m.min(building.plan_length_m)), action));
        }
        report.push(check.build());
    }

    let derived_plan = slender <= 1.0 && recess <= 1.0;
    let derived_elevation = setback <= 1.0;
    let consistent = (!building.plan_regular || derived_plan) && (!building.elevation_regular || derived_elevation);
    let mut claim = CheckResult::assess(
        format!("en1998.1.{bid}.regularity.derived"),
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "4.2.3"),
        SubjectRef::new(bid, format!("{bpath}.planRegular"), blabel.clone()),
        lc("Claimed regularity against the geometry", "Beanspruchte Regelmässigkeit gegen die Geometrie"),
    )
    .status(status_of(consistent))
    .annex(annex)
    .explanation(lc(
        &format!("geometry allows regular plan: {derived_plan}, regular elevation: {derived_elevation}; claimed plan {}, elevation {}", building.plan_regular, building.elevation_regular),
        &format!("Geometrie erlaubt regelmässigen Grundriss: {derived_plan}, regelmässigen Aufriss: {derived_elevation}; beansprucht Grundriss {}, Aufriss {}", building.plan_regular, building.elevation_regular),
    ));
    if building.plan_regular && !derived_plan {
        claim = claim.remedy(Remedy::exactly(SubjectRef::new(bid, format!("{bpath}.planRegular"), lc("Plan regularity", "Regelmässigkeit im Grundriss")), q_dim(1.0), q_dim(0.0), lc("The plan geometry is irregular: drop the regularity claim.", "Der Grundriss ist unregelmässig: Regelmässigkeitsanspruch fallen lassen.")));
    }
    if building.elevation_regular && !derived_elevation {
        claim = claim.remedy(Remedy::exactly(SubjectRef::new(bid, format!("{bpath}.elevationRegular"), lc("Elevation regularity", "Regelmässigkeit im Aufriss")), q_dim(1.0), q_dim(0.0), lc("The set-back is too large: drop the regularity claim.", "Der Rücksprung ist zu gross: Regelmässigkeitsanspruch fallen lassen.")));
    }
    report.push(claim.build());

    if let (Some(next), true) = (next, building.joint_gap_m > 0.0) {
        let drift = |x: &Building| x.storeys.iter().map(|st| st.drift_x_m).sum::<f64>();
        let required = part_1::seismic_joint_gap_m(drift(building), drift(next));
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.joint.gap"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.7"),
            SubjectRef::new(bid, format!("{bpath}.jointGapM"), blabel.clone()),
            lc("Seismic joint d ≥ √(d₁² + d₂²)", "Erdbebenfuge d ≥ √(d₁² + d₂²)"),
        )
        .utilization(q_len(required), floor_limit(q_len(building.joint_gap_m)))
        .annex(annex)
        .explanation(lc(&format!("gap {:.3} m, required {required:.3} m next to {}", building.joint_gap_m, next.id), &format!("Fuge {:.3} m, erforderlich {required:.3} m neben {}", building.joint_gap_m, next.id)));
        if required > building.joint_gap_m {
            check = check.remedy(Remedy::at_least(SubjectRef::new(bid, format!("{bpath}.jointGapM"), lc("Joint gap", "Fugenbreite")), q_len(building.joint_gap_m), q_len(required), lc("Widen the seismic joint.", "Erdbebenfuge verbreitern.")));
        }
        report.push(check.build());
    }

    if building.nonstructural_weight_n > 0.0 {
        let alpha = gamma_i * a_g / 9.81;
        let f_a = part_1::non_structural_force_n(alpha, s, building.nonstructural_height_m, total_height, building.nonstructural_period_s, t1, building.nonstructural_weight_n, 1.0, building.nonstructural_q);
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.nonstructural.force"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.3.5.2"),
            SubjectRef::new(bid, format!("{bpath}.nonstructuralResistanceN"), blabel.clone()),
            lc("Non-structural element: seismic force F_a against anchorage resistance", "Nichttragendes Bauteil: Erdbebenkraft F_a gegen Verankerungswiderstand"),
        )
        .utilization(q_force(f_a), floor_limit(q_force(building.nonstructural_resistance_n)))
        .annex(annex)
        .explanation(lc(&format!("F_a = {f_a:.0} N (W_a {:.0} N, z {:.1} m, T_a {:.2} s, q_a {:.1}, γ_a 1.0)", building.nonstructural_weight_n, building.nonstructural_height_m, building.nonstructural_period_s, building.nonstructural_q), &format!("F_a = {f_a:.0} N (W_a {:.0} N, z {:.1} m, T_a {:.2} s, q_a {:.1}, γ_a 1,0)", building.nonstructural_weight_n, building.nonstructural_height_m, building.nonstructural_period_s, building.nonstructural_q)));
        if f_a > building.nonstructural_resistance_n {
            check = check.remedy(Remedy::at_least(SubjectRef::new(bid, format!("{bpath}.nonstructuralResistanceN"), lc("Anchorage resistance", "Verankerungswiderstand")), q_force(building.nonstructural_resistance_n), q_force(f_a), lc("Strengthen the anchorage of the element.", "Verankerung des Bauteils verstärken.")));
        }
        report.push(check.build());
    }

    for member in &building.members {
        check_member_materials(report, annex, bid, bpath, member);
    }

    let ductile_frame = building.systems.iter().any(|x| matches!(x.system_type, semio_s_artifact_norm_en1998_model::SystemType::Frame | semio_s_artifact_norm_en1998_model::SystemType::Dual) && x.ductility_class != semio_s_artifact_norm_en1998_model::DuctilityClass::Dcl);
    if building.beam_moment_sum_nm > 0.0 && ductile_frame {
        let demand = 1.3 * building.beam_moment_sum_nm;
        let mut check = CheckResult::assess(
            format!("en1998.1.{bid}.capacityDesign.nodes"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.3"),
            SubjectRef::new(bid, format!("{bpath}.columnMomentSumNm"), blabel.clone()),
            lc("Strong column, weak beam: ΣM_Rc ≥ 1.3 ΣM_Rb", "Starke Stütze, schwacher Riegel: ΣM_Rc ≥ 1,3 ΣM_Rb"),
        )
        .utilization(q_mom(demand), floor_limit(q_mom(building.column_moment_sum_nm)))
        .annex(annex)
        .explanation(lc(&format!("ΣM_Rc = {:.0} Nm, 1.3·ΣM_Rb = {demand:.0} Nm, ratio {:.3}", building.column_moment_sum_nm, part_1::strong_column_ratio(building.column_moment_sum_nm, building.beam_moment_sum_nm)), &format!("ΣM_Rc = {:.0} Nm, 1,3·ΣM_Rb = {demand:.0} Nm, Verhältnis {:.3}", building.column_moment_sum_nm, part_1::strong_column_ratio(building.column_moment_sum_nm, building.beam_moment_sum_nm))));
        if building.column_moment_sum_nm < demand {
            check = check.remedy(Remedy::at_least(SubjectRef::new(bid, format!("{bpath}.columnMomentSumNm"), lc("Column moment resistance", "Stützenmomententragfähigkeit")), q_mom(building.column_moment_sum_nm), q_mom(demand), lc("Increase the column resistance or reduce the beam resistance at the node.", "Stützenwiderstand erhöhen oder Riegelwiderstand am Knoten verringern.")));
        }
        report.push(check.build());
    }
}

/// 🔩️ Checks the capacity design rule ΣM_Rc ≥ 1.3 ΣM_Rb (EN 1998-1 4.4.2.3) at every joint row of building `bid`: not applicable outside ductile frames (DCM, DCH) and without beam resistance.
pub(super) fn check_capacity_joints(report: &mut CheckReport, annex: AnnexChoice, building: &Building, rows: &[semio_s_artifact_norm_en1998_model::En1998Joint], bid: &str) {
    let ductile_frame = building.systems.iter().any(|x| matches!(x.system_type, semio_s_artifact_norm_en1998_model::SystemType::Frame | semio_s_artifact_norm_en1998_model::SystemType::Dual) && x.ductility_class != semio_s_artifact_norm_en1998_model::DuctilityClass::Dcl);
    for joint in joint_rows::joints_of(rows, bid) {
        let jid = &joint.id;
        let path = format!("joints[id={jid}]");
        let subject = |field: &str, label: LocalizedCopy| SubjectRef::new(jid.clone(), format!("{path}.{field}"), label);
        let base = CheckResult::assess(
            format!("en1998.1.{bid}.capacityDesign.joint.{jid}"),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "4.4.2.3"),
            subject("columnMomentRdNm", lc(&format!("Joint {jid}"), &format!("Knoten {jid}"))),
            lc("Strong column, weak beam at the joint: ΣM_Rc ≥ 1.3 ΣM_Rb", "Starke Stütze, schwacher Riegel am Knoten: ΣM_Rc ≥ 1,3 ΣM_Rb"),
        );
        if !ductile_frame || joint.beam_moment_rd_nm <= 0.0 {
            report.push(base.annex(annex).not_applicable(lc("Applies to the joints of ductile frames (DCM, DCH) with beam resistance entered.", "Gilt für Knoten duktiler Rahmen (DCM, DCH) mit eingegebenem Riegelwiderstand.")).build());
            continue;
        }
        if joint_rows::is_top_storey_exempt(building, joint) {
            report.push(base.annex(annex).not_applicable(lc("Joints of the top storey of a multi-storey building are exempt from the capacity design rule (EN 1998-1 4.4.2.3(4); target spec en1997-en1998-target.md line 482, SEI-50 \"exempt: top floor\"; code text not yet verified).", "Knoten des obersten Geschosses eines mehrgeschossigen Gebäudes sind von der Kapazitätsbemessungsregel ausgenommen (EN 1998-1 4.4.2.3(4); Zielspezifikation en1997-en1998-target.md Zeile 482, SEI-50 \"ausgenommen: oberstes Geschoss\"; Normtext noch nicht geprüft).")).build());
            continue;
        }
        let (demand, ratio) = (joint_rows::demand_nm(joint), joint_rows::ratio(joint));
        let mut check = base
            .utilization(q_mom(demand), floor_limit(q_mom(joint.column_moment_rd_nm)))
            .annex(annex)
            .explanation(lc(&format!("ΣM_Rc = {:.0} Nm, 1.3·ΣM_Rb = {demand:.0} Nm, ratio {ratio:.3}", joint.column_moment_rd_nm), &format!("ΣM_Rc = {:.0} Nm, 1,3·ΣM_Rb = {demand:.0} Nm, Verhältnis {ratio:.3}", joint.column_moment_rd_nm)));
        if !joint_rows::is_satisfied(joint) {
            check = check
                .remedy(Remedy::at_least(subject("columnMomentRdNm", lc("Column moment resistance", "Stützenmomententragfähigkeit")), q_mom(joint.column_moment_rd_nm), q_mom(demand), lc("Increase the column resistance at the joint.", "Stützenwiderstand am Knoten erhöhen.")))
                .remedy(Remedy::at_most(subject("beamMomentRdNm", lc("Beam moment resistance", "Riegelmomententragfähigkeit")), q_mom(joint.beam_moment_rd_nm), q_mom(joint.column_moment_rd_nm / part_1::STRONG_COLUMN_FACTOR), lc("Reduce the beam resistance at the joint.", "Riegelwiderstand am Knoten verringern.")));
        }
        report.push(check.build());
    }
}
