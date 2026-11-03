//! 🔩️ Joint catalogue-gap checks: block tearing, bolt head punching and the effective width of plates welded to unstiffened flanges of EN 1993-1-8. See [`super`].

use super::*;

fn subject(joint: &SteelJoint, leaf: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&joint.id, format!("joints[id={}].{leaf}", joint.id), loc(en, de))
}

fn governing_uls(document: &En1993Snapshot, joint: &SteelJoint) -> (f64, f64) {
    let pick = |value: fn(&semio_s_artifact_norm_en1993_model::JointForceAction) -> f64| {
        let forces: Vec<(String, f64)> = joint.actions.iter().map(|a| (a.load_case_id.clone(), value(a))).collect();
        governing_scalar(&combine_scalar_forces(document, &forces), "uls").map_or(0.0, |c| c.value)
    };
    (pick(|a| a.shear), pick(|a| a.tension))
}

fn member_fy(document: &En1993Snapshot, joint: &SteelJoint) -> f64 {
    document.members.iter().find(|m| m.id == joint.member_id).and_then(|m| find_material(document, &m.material_id)).map_or(joint.plate_fu / 1.44, |m| m.fy)
}

fn check_block_tearing(annex: AnnexChoice, params: AnnexParams, joint: &SteelJoint, shear_ed: f64, fy: f64, report: &mut CheckReport) {
    let d0 = clauses::joints::hole_diameter(joint.bolt_diameter * 1e3) / 1e3;
    let p2 = if joint.gauge > 0.0 { joint.gauge } else { joint.pitch };
    let (a_nt, a_nv) = clauses::joints::block_tearing_areas(joint.bolt_rows, joint.bolts_per_row, joint.pitch, p2, joint.end_distance, joint.edge_distance, d0, joint.plate_thickness);
    let v_rd = clauses::joints::block_tearing(joint.plate_fu, fy, a_nt, a_nv, true, params.gamma_m0, params.gamma_m2);
    let mut b = CheckResult::assess(
        format!("en1993.1-8.3.10.block.{}", joint.id),
        "DIN EN 1993-1-8",
        ClauseId::new("EN 1993-1-8", "3", "3.10.2"),
        subject(joint, "plateThickness", "Plate thickness", "Blechdicke"),
        loc("Block tearing of the bolted plate", "Blockversagen des geschraubten Blechs"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-405.1")
    .ndps(&[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2])
    .utilization(force_n(shear_ed), force_n(v_rd))
    .explanation(loc(
        &format!("V_Ed={:.1} kN, V_eff,1,Rd={:.1} kN (concentric: f_u·A_nt/γ_M2 with A_nt={:.0} mm², f_y·A_nv/(√3·γ_M0) with A_nv={:.0} mm², {}×{} bolts, d_0={:.0} mm).", shear_ed / 1e3, v_rd / 1e3, a_nt * 1e6, a_nv * 1e6, joint.bolt_rows, joint.bolts_per_row, d0 * 1e3),
        &format!("V_Ed={:.1} kN, V_eff,1,Rd={:.1} kN (zentrisch: f_u·A_nt/γ_M2 mit A_nt={:.0} mm², f_y·A_nv/(√3·γ_M0) mit A_nv={:.0} mm², {}×{} Schrauben, d_0={:.0} mm).", shear_ed / 1e3, v_rd / 1e3, a_nt * 1e6, a_nv * 1e6, joint.bolt_rows, joint.bolts_per_row, d0 * 1e3),
    ));
    if shear_ed > v_rd {
        let t_req = joint.plate_thickness * shear_ed / v_rd.max(1e-9);
        b = b.remedy(Remedy::at_least(
            subject(joint, "plateThickness", "Plate thickness", "Blechdicke"),
            length_m(joint.plate_thickness),
            length_m(t_req),
            loc(&format!("Increase the plate thickness to at least {:.1} mm (V_eff,Rd is linear in t).", t_req * 1e3), &format!("Blechdicke auf mindestens {:.1} mm erhöhen (V_eff,Rd ist linear in t).", t_req * 1e3)),
        ));
    }
    report.push(b.build());
}

fn check_punching(annex: AnnexChoice, params: AnnexParams, joint: &SteelJoint, tension_ed: f64, report: &mut CheckReport) {
    let n_bolts = (joint.bolt_rows * joint.bolts_per_row).max(1) as f64;
    let size = clauses::joints::bolt_size_designation(joint.bolt_diameter);
    let across_flats = semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1993.1-8.3.6.head-nut.{size}.across-flats"), annex) / 1e3;
    let across_points = semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1993.1-8.3.6.head-nut.{size}.across-points"), annex) / 1e3;
    let d_m = clauses::joints::punching_mean_diameter(across_flats, across_points);
    let b_p_rd = clauses::joints::bp_rd(d_m, joint.plate_thickness, joint.plate_fu, params.gamma_m2);
    let per_bolt = tension_ed / n_bolts;
    let mut b = CheckResult::assess(
        format!("en1993.1-8.3.6.punching.{}", joint.id),
        "DIN EN 1993-1-8",
        ClauseId::new("EN 1993-1-8", "3", "3.6.1/Table 3.4"),
        subject(joint, "plateThickness", "Plate thickness", "Blechdicke"),
        loc("Punching shear of bolt heads and nuts", "Durchstanzen von Schraubenköpfen und Muttern"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-402.1")
    .ndps(&[AnnexParams::GAMMA_M2])
    .utilization(force_n(per_bolt), force_n(b_p_rd))
    .explanation(loc(
        &format!("F_t,Ed per bolt={:.1} kN, B_p,Rd=0.6·π·d_m·t_p·f_u/γ_M2={:.1} kN (d_m=(across points {:.2} + across flats {:.2} mm)/2={:.2} mm of the nut, UNVERIFIED product dimensions; t_p={:.1} mm).", per_bolt / 1e3, b_p_rd / 1e3, across_points * 1e3, across_flats * 1e3, d_m * 1e3, joint.plate_thickness * 1e3),
        &format!("F_t,Ed je Schraube={:.1} kN, B_p,Rd=0,6·π·d_m·t_p·f_u/γ_M2={:.1} kN (d_m=(Eckenmaß {:.2} + Schlüsselweite {:.2} mm)/2={:.2} mm der Mutter, UNGEPRÜFTE Produktmaße; t_p={:.1} mm).", per_bolt / 1e3, b_p_rd / 1e3, across_points * 1e3, across_flats * 1e3, d_m * 1e3, joint.plate_thickness * 1e3),
    ));
    if per_bolt > b_p_rd {
        let t_req = joint.plate_thickness * per_bolt / b_p_rd.max(1e-9);
        b = b.remedy(Remedy::at_least(
            subject(joint, "plateThickness", "Plate thickness", "Blechdicke"),
            length_m(joint.plate_thickness),
            length_m(t_req),
            loc(&format!("Increase the plate thickness to at least {:.1} mm or add washers.", t_req * 1e3), &format!("Blechdicke auf mindestens {:.1} mm erhöhen oder Unterlegscheiben vorsehen.", t_req * 1e3)),
        ));
    }
    report.push(b.ndp_reads().build());
}

fn check_flange_weld(annex: AnnexChoice, joint: &SteelJoint, member_fy: f64, report: &mut CheckReport) {
    let s = 2.0_f64.sqrt() * joint.weld_throat;
    let fy_f = if joint.column_fy > 0.0 { joint.column_fy } else { member_fy };
    let fy_p = if joint.plate_fy > 0.0 { joint.plate_fy } else { member_fy };
    let b_eff = clauses::joints::weld_flange_beff(joint.column_web_thickness, s, joint.column_flange_thickness, joint.plate_thickness, fy_f, fy_p, joint.plate_width);
    let required = fy_p / joint.plate_fu * joint.plate_width;
    let mut b = CheckResult::assess(
        format!("en1993.1-8.4.11.beff.{}", joint.id),
        "DIN EN 1993-1-8",
        ClauseId::new("EN 1993-1-8", "4", "4.10"),
        subject(joint, "columnFlangeThickness", "Flange thickness", "Flanschdicke"),
        loc("Effective width of the plate welded to an unstiffened flange", "Mittragende Breite des an einen unausgesteiften Flansch geschweißten Blechs"),
    )
    .annex(annex)
    .ndp("ndp.en1993.v-413.1")
    .minimum(length_m(b_eff), length_m(required))
    .explanation(loc(
        &format!("b_eff=t_w+2s+7k·t_f={:.1} mm (s={:.1} mm, k=min(1,(t_f/t_p)(f_y,f/f_y,p))={:.2}) against f_y,p/f_u,p·b_p={:.1} mm; below it the joint must be stiffened.", b_eff * 1e3, s * 1e3, (joint.column_flange_thickness / joint.plate_thickness * fy_f / fy_p).min(1.0), required * 1e3),
        &format!("b_eff=t_w+2s+7k·t_f={:.1} mm (s={:.1} mm, k=min(1,(t_f/t_p)(f_y,f/f_y,p))={:.2}) gegenüber f_y,p/f_u,p·b_p={:.1} mm; darunter ist der Anschluss auszusteifen.", b_eff * 1e3, s * 1e3, (joint.column_flange_thickness / joint.plate_thickness * fy_f / fy_p).min(1.0), required * 1e3),
    ));
    if b_eff + 1e-12 < required {
        let t_f = clauses::joints::weld_flange_thickness_for(required, joint.column_web_thickness, s, joint.plate_thickness, fy_f, fy_p);
        b = b.remedy(Remedy::at_least(
            subject(joint, "columnFlangeThickness", "Flange thickness", "Flanschdicke"),
            length_m(joint.column_flange_thickness),
            length_m(t_f),
            loc(&format!("Increase the flange thickness to at least {:.1} mm or stiffen the flange opposite the plate.", t_f * 1e3), &format!("Flanschdicke auf mindestens {:.1} mm erhöhen oder den Flansch gegenüber dem Blech aussteifen.", t_f * 1e3)),
        ));
    }
    report.push(b.build());
}

/// 🔩️ Appends the joint catalogue-gap checks of every admitted joint.
pub(super) fn check_joints(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for joint in document.joints.iter().filter(|j| scope.admits("vidJoints", &j.id)) {
        let (shear_ed, tension_ed) = governing_uls(document, joint);
        let fy = member_fy(document, joint);
        if joint.kind == semio_s_artifact_norm_en1993_model::JointKind::Bolted && joint.bolt_rows > 0 && joint.bolts_per_row > 0 {
            if shear_ed > 0.0 {
                check_block_tearing(annex, params, joint, shear_ed, fy, report);
            }
            if tension_ed > 0.0 {
                check_punching(annex, params, joint, tension_ed, report);
            }
        }
        if joint.kind == semio_s_artifact_norm_en1993_model::JointKind::Welded && joint.column_flange_thickness > 0.0 && joint.plate_thickness > 0.0 && joint.plate_width > 0.0 && joint.weld_throat > 0.0 && joint.plate_fu > 0.0 {
            check_flange_weld(annex, joint, fy, report);
        }
    }
}
