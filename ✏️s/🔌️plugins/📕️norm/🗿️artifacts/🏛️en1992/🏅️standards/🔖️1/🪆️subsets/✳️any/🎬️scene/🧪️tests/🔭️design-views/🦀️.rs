//! 🔭️ Time and design-model views of the EN 1992 scene: prestress loss over time, creep and shrinkage curve, bar fatigue range and the strut-and-tie model of deep beams.

use super::*;
use model::{En1992Snapshot, MemberKind};
use scene::projection_kit::test_support::revision;
use scene::Validate;
use semio_s_artifact_norm_en1992_checks as checks;

fn built_of(document: &En1992Snapshot) -> Built {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    build(&SceneInput { document, report: &report, placements: &[], revision: revision() })
}

fn deep_beam() -> En1992Snapshot {
    let mut document = En1992Snapshot::compliant_office_frame();
    let member = document.members.iter_mut().find(|m| m.kind == MemberKind::Beam).expect("a beam");
    member.span = 1.5;
    member.height = 0.6;
    member.effective_depth = 0.54;
    document
}

#[test]
fn the_prestress_loss_grows_with_the_age_and_the_force_falls() {
    let document = En1992Snapshot::compliant_prestressed_beam();
    let member = document.members.iter().find(|m| m.prestress.is_some()).expect("a prestressed member");
    let rows = prestress_loss::loss_series(&document, member).expect("loss series");
    assert_eq!(rows.len(), tracks::CREEP_DAYS.len());
    assert!(rows.windows(2).all(|w| w[1].loss_mpa >= w[0].loss_mpa && w[1].force_n <= w[0].force_n));
    assert!(rows.iter().all(|r| r.relaxation_mpa >= 0.0 && r.loss_mpa >= r.relaxation_mpa * 0.8));
    let last = rows[rows.len() - 1];
    let ap = member.prestress.as_ref().map_or(0.0, |p| p.area) * 1.0e6;
    assert!((member.prestress.as_ref().map_or(0.0, |p| p.force) - last.loss_mpa * ap - last.force_n).abs() < 1.0e-6 * last.force_n.abs());
}

#[test]
fn the_prestress_track_and_field_are_in_the_scene_and_validate() {
    let built = built_of(&En1992Snapshot::compliant_prestressed_beam());
    assert!(built.scene.tracks.iter().any(|t| t.id == prestress_loss::TRACK));
    assert!(built.scene.entities.iter().any(|e| matches!(&e.appearance.channel, ColourChannel::Fields { fields, .. } if fields.iter().any(|f| f.id.starts_with(prestress_loss::FIELD_PREFIX)))));
    assert_eq!(built.scene.validate(), Ok(()));
    let diagram = built.diagrams.iter().find(|d| d.id.starts_with("prestress-loss:")).expect("prestress diagram");
    assert_eq!(diagram.validate(), Ok(()));
}

#[test]
fn the_creep_and_shrinkage_curve_ends_at_the_annex_b_values() {
    let document = En1992Snapshot::compliant_office_frame();
    let built = built_of(&document);
    let diagram = built.diagrams.iter().find(|d| d.id.starts_with("creep-shrinkage:")).expect("creep diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let member = &document.members[0];
    let fck = document.concrete(&member.concrete_grade_id).expect("grade").f_ck / 1.0e6;
    let h0 = tracks::notional_size(if member.kind == MemberKind::FlatSlab { member.span } else { member.width }, member.height);
    let phi = tracks::creep_phi(fck + 8.0, tracks::RELATIVE_HUMIDITY, h0, tracks::LOADING_AGE_DAYS, 18_250.0);
    let marker = diagram.markers.iter().find(|m| m.id == "reference:phi-50a").expect("50 year marker");
    assert_eq!(marker.label.en, format!("φ(50 a) = {phi:.2}"));
}

#[test]
fn the_fatigue_ranges_are_finite_and_tint_the_bars() {
    for document in [En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced()] {
        for member in document.members.iter().filter(|m| m.span > 0.0 && m.kind == MemberKind::Beam) {
            if let Some(ranges) = fatigue_range::bar_ranges(&document, member, member.width) {
                assert!(ranges.limit_mpa > 0.0 && ranges.bars.iter().all(|b| b.range_mpa.is_finite() && b.range_mpa >= 0.0));
            }
        }
        let built = built_of(&document);
        assert_eq!(built.scene.validate(), Ok(()));
        for entity in built.scene.entities.iter().filter(|e| e.entity.id.contains(":fatigue-")) {
            assert!(matches!(&entity.appearance.channel, ColourChannel::Fields { fields, .. } if fields.iter().any(|f| f.id == fatigue_range::FIELD)));
        }
    }
}

#[test]
fn the_deep_beam_truss_is_in_equilibrium_with_the_analytic_forces() {
    let document = deep_beam();
    let member = document.members.iter().find(|m| m.kind == MemberKind::Beam).expect("a beam");
    let truss = strut_and_tie::truss(&document, member).expect("a determinate truss");
    let reaction = truss.load / 2.0;
    let ((x0, z0), (_, _), (xa, za)) = (truss.nodes[0], truss.nodes[1], truss.nodes[2]);
    let (run, rise) = (xa - x0, za - z0);
    let tie = reaction * run / rise;
    let strut = -reaction * run.hypot(rise) / rise;
    assert!((truss.forces[0] - tie).abs() < 1.0e-6 * tie.abs());
    assert!((truss.forces[1] - strut).abs() < 1.0e-6 * strut.abs() && (truss.forces[2] - strut).abs() < 1.0e-6 * strut.abs());
    assert!(truss.tie_area > 0.0);
}

#[test]
fn the_deep_beam_scene_carries_two_struts_and_a_tie_and_slender_beams_none() {
    let built = built_of(&deep_beam());
    assert_eq!(built.scene.validate(), Ok(()));
    let ids: Vec<&str> = built.scene.entities.iter().map(|e| e.entity.id.as_str()).filter(|id| id.contains(":stm-")).collect();
    assert_eq!(ids.len(), 3, "{ids:?}");
    let slender = built_of(&En1992Snapshot::compliant_office_frame());
    assert!(!slender.scene.entities.iter().any(|e| e.entity.id.contains(":stm-")));
}

fn report_of(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    report
}

fn column_of<'a>(diagram: &'a scene::Diagram2d, name: &str) -> &'a [f64] {
    &diagram.table_twin.columns.iter().find(|c| c.header.en == name).unwrap_or_else(|| panic!("no column {name}")).values
}

#[test]
fn the_torsion_diagram_draws_the_two_limit_lines_through_the_design_point_of_the_check() {
    let document = En1992Snapshot::failing_under_reinforced();
    let report = report_of(&document);
    let built = build(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let diagram = built.diagrams.iter().find(|d| d.id == "torsion-interaction:beam-B1").expect("torsion diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let (t, v) = (column_of(diagram, "Torsion T_Ed")[0], column_of(diagram, "Shear V_Ed")[0]);
    let (t_max, v_max) = (column_of(diagram, "T_Rd,max")[0], column_of(diagram, "V_Rd,max")[0]);
    let (t_c, v_c) = (column_of(diagram, "T_Rd,c")[0], column_of(diagram, "V_Rd,c")[0]);
    let check = report.checks.iter().find(|c| c.id == "en1992.6.3.interaction.beam-B1").expect("interaction check");
    assert!((t / t_max + v / v_max - check.utilization).abs() < 1e-9 * check.utilization.max(1.0), "the marker is the utilisation of the report");
    assert!((column_of(diagram, "Interaction without reinforcement")[0] - (t / t_c + v / v_c)).abs() < 1e-9);
    let marker = diagram.markers.iter().find(|m| m.id == "design-marker").expect("design marker");
    assert!((marker.at.x - v).abs() < 1e-12 && (marker.at.y - t).abs() < 1e-12);
    assert!(report.checks.iter().any(|c| c.id == "en1992.6.3.torsion-links.beam-B1") && report.checks.iter().any(|c| c.id == "en1992.6.3.torsion-longitudinal.beam-B1"));
}

#[test]
fn the_torsion_links_are_checked_against_the_closed_form_of_eq_6_28() {
    let document = En1992Snapshot::failing_under_reinforced();
    let report = report_of(&document);
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let check = report.checks.iter().find(|c| c.id == "en1992.6.3.torsion-links.beam-B1").expect("links check");
    let stirrups = beam.stirrups.as_ref().expect("links");
    let leg = std::f64::consts::PI * (stirrups.diameter * 500.0).powi(2) / (stirrups.spacing * 1e3);
    let t_ed = checks::governing_by(&checks::combine_member_actions(beam, &document.action_factors()), "uls", |e| e.t_ed.abs()).expect("torsion").t_ed.abs() * 1e3;
    let (b, h) = (beam.width * 1e3, beam.height * 1e3);
    let t_ef = b * h / (2.0 * (b + h));
    let a_k = (b - t_ef) * (h - t_ef);
    let required_cot_one = t_ed / (2.0 * a_k * 500.0 / 1.15);
    assert!(check.utilization >= required_cot_one / leg / 3.0 - 1e-9, "the torsion share over the whole leg is a lower bound: the shear share only lowers the provided leg");
    assert!(check.utilization.is_finite() && check.utilization > 0.0);
}

#[test]
fn the_second_order_diagram_ends_on_the_amplified_point_of_the_slenderness_check() {
    let document = En1992Snapshot::compliant_office_frame();
    let report = report_of(&document);
    let built = build(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let diagram = built.diagrams.iter().find(|d| d.id == "second-order:col-C1").expect("second-order diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let check = report.checks.iter().find(|c| c.id == "en1992.5.8.slender.col-C1").expect("slenderness check");
    let marker = diagram.markers.iter().find(|m| m.id == "amplified").expect("amplified marker");
    assert!((marker.at.x * 1e3 - check.computed.value.abs()).abs() < 1e-9 * check.computed.value.abs().max(1.0));
    assert!((marker.utilisation.unwrap_or(f64::NAN) - check.utilization).abs() < 1e-9);
    let (first, total) = (column_of(diagram, "First-order moment M_0Ed")[0], column_of(diagram, "Amplified moment M_Ed")[0]);
    assert!(total >= first, "the second-order effects never reduce the moment");
}

#[test]
fn the_crack_width_curve_passes_through_the_report_value_and_starts_uncracked() {
    let document = En1992Snapshot::compliant_office_frame();
    let report = report_of(&document);
    let built = build(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let diagram = built.diagrams.iter().find(|d| d.id == "crack-width-moment:beam-B1").expect("crack width diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let widths = column_of(diagram, "Crack width w_k");
    assert!(widths.windows(2).all(|w| w[1] >= w[0] - 1e-12) && widths[0] == 0.0, "monotone from the uncracked state");
    let check = report.checks.iter().find(|c| c.id == "en1992.7.3.crack.beam-B1").expect("crack check");
    let marker = diagram.markers.iter().find(|m| m.id == "design").expect("design marker");
    assert!((marker.at.y - check.computed.value * 1e3).abs() < 1e-9 * (check.computed.value * 1e3).max(1.0));
}

#[test]
fn the_deep_beam_has_a_tie_check_a_truss_diagram_and_the_tie_area_of_the_check() {
    let document = deep_beam();
    let report = report_of(&document);
    let member = document.members.iter().find(|m| m.kind == MemberKind::Beam).expect("beam");
    let built = build(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let diagram = built.diagrams.iter().find(|d| d.id == format!("strut-and-tie:{}", member.id)).expect("truss diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let truss = strut_and_tie::truss(&document, member).expect("truss");
    let forces = column_of(diagram, "Axial force");
    assert!((forces[0] - truss.forces[0] / 1e3).abs() < 1e-9 && forces[1] < 0.0 && forces[2] < 0.0);
    let check = report.checks.iter().find(|c| c.id == format!("en1992.6.5.tie.{}", member.id)).expect("tie check");
    let sec = mechanics::member_sec(&document, member).expect("section");
    let provided = checks::Geo::of(&sec, member).a_s;
    assert!((check.utilization - truss.tie_area / provided).abs() < 1e-9 * check.utilization.max(1.0));
    let slender = report_of(&En1992Snapshot::compliant_office_frame());
    assert!(!slender.checks.iter().any(|c| c.id.starts_with("en1992.6.5.tie.")));
}

fn deep_beam_on_plates(width: f64) -> En1992Snapshot {
    let mut document = deep_beam();
    document.members.iter_mut().find(|m| m.kind == MemberKind::Beam).expect("a beam").bearing_width = width;
    document
}

struct Seen {
    id: String,
    utilization: f64,
    status: String,
}

fn check_with(report: &CheckReport, family: &str) -> Seen {
    let c = report.checks.iter().find(|c| c.id.starts_with(&format!("en1992.6.5.{family}."))).expect("a strut-and-tie check");
    Seen { id: c.id.clone(), utilization: c.utilization, status: format!("{:?}", c.status) }
}

#[test]
fn the_strut_and_node_checks_need_the_bearing_width_and_follow_it_and_the_equilibrium() {
    let without = report_of(&deep_beam());
    for family in ["strut", "node-cct", "node-ccc"] {
        assert_eq!(check_with(&without, family).status, "NotApplicable", "{family} without a bearing width");
    }
    let (narrow, wide) = (report_of(&deep_beam_on_plates(0.1)), report_of(&deep_beam_on_plates(0.2)));
    for family in ["strut", "node-cct", "node-ccc"] {
        let (u1, u2) = (check_with(&narrow, family).utilization, check_with(&wide, family).utilization);
        assert!(u2 > 0.0 && (u1 / u2 - 2.0).abs() < 1e-9, "{family}: stress is inversely proportional to the plate width");
    }
    let annex = deep_beam().annex;
    let case = |node: &str| checks::Case::new("t", annex).with_n("fck", 30.0).with_s("node", node);
    let limit = |id: &str, case: &checks::Case, key: &str| checks::evaluate_outcomes(id, case).expect("outcomes").into_iter().find(|o| o.key == key).expect("outcome").capacity;
    let (cct, ccc) = (limit("ST-03", &case("CCT"), "sigma_rd_max"), limit("ST-03", &case("CCC"), "sigma_rd_max"));
    let strut = limit("ST-01", &checks::Case::new("t", annex).with_n("fck", 30.0).with_s("kind", "cracked"), "sigma_rd_max");
    let (u_cct, u_ccc, u_strut) = (check_with(&wide, "node-cct").utilization, check_with(&wide, "node-ccc").utilization, check_with(&wide, "strut").utilization);
    assert!((u_ccc / u_cct - 2.0 * cct / ccc).abs() < 1e-9, "the load node carries the whole load, the support node half");
    assert!((u_cct / u_strut - strut / cct).abs() < 1e-9, "R = F_c sin(theta) at the support node");
}

#[test]
fn the_truss_diagram_and_the_scene_colour_the_nodes_by_the_node_checks() {
    let document = deep_beam_on_plates(0.02);
    let report = report_of(&document);
    let built = build(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    let member = document.members.iter().find(|m| m.kind == MemberKind::Beam).expect("beam");
    let diagram = built.diagrams.iter().find(|d| d.id == format!("strut-and-tie:{}", member.id)).expect("truss diagram");
    assert_eq!(diagram.validate(), Ok(()));
    for (marker, family) in [("node-0", "node-cct"), ("node-1", "node-cct"), ("node-2", "node-ccc")] {
        let node = diagram.markers.iter().find(|m| m.id == marker).expect("node marker");
        let check = check_with(&report, family);
        assert_eq!(node.bound_check.as_deref(), Some(check.id.as_str()));
        assert!((node.utilisation.expect("utilisation") - check.utilization).abs() < 1e-12);
    }
    assert!(check_with(&report, "node-cct").utilization > 1.0, "a 20 mm plate overstresses the support node");
    assert_eq!(format!("{:?}", diagram.markers.iter().find(|m| m.id == "node-0").expect("node").tone), "Error");
    assert!(built.scene.annotations.iter().any(|a| a.id().ends_with(":stm-node-0")) && built.scene.annotations.iter().any(|a| a.id().ends_with(":stm-node-2")));
}
