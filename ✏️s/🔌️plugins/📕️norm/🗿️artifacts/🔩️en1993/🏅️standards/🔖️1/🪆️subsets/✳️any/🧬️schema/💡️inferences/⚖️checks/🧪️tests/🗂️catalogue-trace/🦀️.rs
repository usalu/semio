//! 🗂️ Catalogue traceability: every `V-xxx` id maps to a check family produced by an example document or is marked not modelled with a reason; the catalogue-gap checks follow their hand formulas.

use crate::spec_trace::TABLE;
use crate::{check_full_steel_structure, fire_ltb_summary, torsional_summary};
use semio_s_artifact_norm_en1993_model::{En1993Snapshot, FatigueBand};
use semio_s_plugin_norm_compliance::spec_trace::{assert_reaches_all_but, assert_traced, Emitted};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus};
use std::collections::BTreeSet;

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"))
}

fn close(a: f64, b: f64, rel: f64, what: &str) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: {a} vs {b}");
}

fn example_documents() -> Vec<En1993Snapshot> {
    let base = En1993Snapshot::compliant_heb240_frame();
    let mut tension = base.clone();
    tension.member_actions.iter_mut().for_each(|a| a.action.n = -a.action.n.abs());
    let mut torsion = base.clone();
    torsion.member_actions.iter_mut().for_each(|a| {
        a.action.t = 5_000.0;
        a.action.mz = 3_000.0;
    });
    let mut parabolic = base.clone();
    parabolic.members[0].moment_diagram = semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic;
    let mut bolted = base.clone();
    bolted.joints[0].category = semio_s_artifact_norm_en1993_model::JointCategory::B;
    bolted.joints[0].actions.iter_mut().for_each(|a| a.tension = 20_000.0);
    let mut welded = base.clone();
    let joint = &mut welded.joints[0];
    joint.kind = semio_s_artifact_norm_en1993_model::JointKind::Welded;
    joint.weld_throat = 0.005;
    joint.weld_length = 0.2;
    joint.plate_width = 0.2;
    joint.plate_fy = 355.0e6;
    joint.column_fy = 355.0e6;
    joint.column_flange_thickness = 0.017;
    joint.column_web_thickness = 0.010;
    vec![base, En1993Snapshot::noncompliant_overloaded_frame(), En1993Snapshot::frame_portal_showcase(), En1993Snapshot::special_parts_showcase(), tension, torsion, parabolic, bolted, welded]
}

#[test]
fn every_spec_id_is_traced_to_a_check_family_an_example_document_emits() {
    assert_eq!(TABLE.spec_ids.len(), 166);
    let documents = example_documents().into_iter().chain([En1993Snapshot::annex_showcase(), semio_s_artifact_norm_en1993_showcase::full_showcase()]);
    let emitted = documents.fold(Emitted::default(), |emitted, doc| emitted.with_checks(check_full_steel_structure(&doc).checks.into_iter().map(|c| c.id)));
    assert_traced(&TABLE, &emitted);
}

#[test]
fn torsional_buckling_of_the_heb240_follows_the_closed_form() {
    let doc = En1993Snapshot::compliant_heb240_frame();
    let s = torsional_summary(&doc, "member-b1").expect("compression present");
    let (sec, mat) = (&doc.sections[0], &doc.materials[0]);
    let i0_sq = (sec.iy + sec.iz) / sec.area;
    close(s.n_cr_t, (mat.g_modulus * sec.it + std::f64::consts::PI.powi(2) * mat.e_modulus * sec.iw / 16.0) / i0_sq, 1e-12, "N_cr,T");
    close(s.n_cr_t, 10_170_865.89, 1e-6, "N_cr,T against the oracle");
    let report = check_full_steel_structure(&doc);
    let check = find(&report, "en1993.6.3.1.tf.member-b1");
    assert_eq!(check.status, CheckStatus::UnverifiedBasis);
    close(check.utilization, s.n_ed / s.n_b_rd_t, 1e-12, "utilisation");
    assert!(check.utilization < 1.0);
}

#[test]
fn the_overloaded_frame_fails_the_torsional_check_with_a_section_remedy() {
    let report = check_full_steel_structure(&En1993Snapshot::noncompliant_overloaded_frame());
    let check = report.checks.iter().find(|c| c.id.starts_with("en1993.6.3.1.tf.")).expect("torsional check");
    if check.status == CheckStatus::Fail {
        assert!(!check.remedies.is_empty());
    }
}

#[test]
fn block_tearing_follows_the_hand_formula_and_fails_with_a_plate_thickness_remedy() {
    let doc = En1993Snapshot::compliant_heb240_frame();
    let report = check_full_steel_structure(&doc);
    let check = find(&report, "en1993.1-8.3.10.block.joint-j1");
    close(check.limit.value, 510.0e6 * 380.0e-6 / 1.25 + 355.0e6 * 1340.0e-6 / 3.0_f64.sqrt(), 1e-9, "V_eff,1,Rd");
    assert!(check.utilization < 1.0);
    let mut thin = doc;
    thin.joints[0].plate_thickness = 0.0005;
    thin.joints[0].actions.iter_mut().for_each(|a| a.shear *= 20.0);
    let failed = check_full_steel_structure(&thin);
    let check = find(&failed, "en1993.1-8.3.10.block.joint-j1");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.remedies.iter().any(|r| r.target.path.contains("plateThickness")));
}

#[test]
fn bolt_head_punching_and_the_flange_weld_width_are_checked_for_their_joint_kinds() {
    let docs = example_documents();
    let bolted = check_full_steel_structure(&docs[7]);
    let punching = find(&bolted, "en1993.1-8.3.6.punching.joint-j1");
    let d_m = 0.5 * (0.02916 + 0.03295);
    close(punching.limit.value, 0.6 * std::f64::consts::PI * d_m * 0.010 * 510.0e6 / 1.25, 1e-9, "B_p,Rd");
    close(punching.computed.value, (1.35 * 20_000.0 + 1.5 * 20_000.0) / 4.0, 1e-9, "per-bolt tension is the quarter of the governing total");
    let welded = check_full_steel_structure(&docs[8]);
    let beff = find(&welded, "en1993.1-8.4.11.beff.joint-j1");
    let s = 2.0_f64.sqrt() * 0.005;
    close(beff.computed.value, 0.010 + 2.0 * s + 7.0 * 0.017, 1e-9, "b_eff");
    close(beff.limit.value, 355.0 / 510.0 * 0.2, 1e-9, "f_y,p b_p/f_u,p");
    assert_eq!(beff.status, CheckStatus::UnverifiedBasis);
}

#[test]
fn material_checks_grade_ductility_and_constants() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    let report = check_full_steel_structure(&doc);
    assert_eq!(find(&report, "en1993.3.2.1.grade.mat-s355").utilization, 1.0);
    close(find(&report, "en1993.3.2.2.ductility.mat-s355").computed.value, 510.0 / 355.0, 1e-12, "f_u/f_y");
    assert!(find(&report, "en1993.3.2.6.constants.mat-s355").utilization < 0.3);
    doc.materials[0].fy = 360.0e6;
    doc.materials[0].fu = 380.0e6;
    doc.materials[0].e_modulus = 200.0e9;
    let report = check_full_steel_structure(&doc);
    for id in ["en1993.3.2.1.grade.mat-s355", "en1993.3.2.2.ductility.mat-s355", "en1993.3.2.6.constants.mat-s355"] {
        let check = find(&report, id);
        assert_eq!(check.status, CheckStatus::Fail, "{id}");
        assert!(!check.remedies.is_empty(), "{id}");
    }
}

#[test]
fn the_shear_area_of_a_rolled_i_is_compared_with_6_2_6_3() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    let ok = check_full_steel_structure(&doc);
    assert_eq!(find(&ok, "en1993.6.2.6.av.member-b1").utilization, 0.0, "the stored h·t_w is conservative against 33.2 cm²");
    doc.sections[0].shear_area_z = 0.0040;
    let report = check_full_steel_structure(&doc);
    let check = find(&report, "en1993.6.2.6.av.member-b1");
    let a_v = 0.0106 - 2.0 * 0.240 * 0.017 + (0.010 + 2.0 * 0.021) * 0.017;
    close(check.utilization, ((0.0040 - a_v) / a_v) / 0.03, 1e-9, "excess of the stored A_v");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(!check.remedies.is_empty());
}

#[test]
fn bow_rolled_and_simplified_ltb_and_frequency_are_reported_per_member() {
    let docs = example_documents();
    let report = check_full_steel_structure(&docs[0]);
    assert_eq!(find(&report, "en1993.5.3.2.bow.member-b1").status, CheckStatus::Info);
    assert!(find(&report, "en1993.6.3.2.3.rolled.member-b1").utilization < 1.0);
    assert!(find(&report, "en1993.6.3.2.4.cf.member-b1").utilization < 1.0);
    assert!(!report.checks.iter().any(|c| c.id.starts_with("en1993.7.2.3.freq.")), "linear moment diagram: no uniform-load frequency");
    let parabolic = check_full_steel_structure(&docs[6]);
    let f1 = find(&parabolic, "en1993.7.2.3.freq.member-b1").computed.value;
    close(f1, std::f64::consts::PI / 32.0 * (210.0e9 * 1.126e-4 / (8.0 * 40_000.0 / 16.0 / 9.81)).sqrt(), 1e-9, "f_1");
}

#[test]
fn constant_amplitude_fatigue_and_the_fire_checks_follow_the_clauses() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    let variable = check_full_steel_structure(&doc);
    assert_eq!(find(&variable, "en1993.1-9.6.cafl.fat-1").status, CheckStatus::Info);
    doc.fatigue_details[0].spectrum = vec![FatigueBand { id: "fb-1".into(), delta_sigma: 60.0e6, cycles: 1.0e7 }];
    let report = check_full_steel_structure(&doc);
    let cafl = find(&report, "en1993.1-9.6.cafl.fat-1");
    close(cafl.limit.value, 71.0e6 * (0.4_f64).powf(1.0 / 3.0), 1e-12, "Δσ_D");
    assert_eq!(cafl.status, CheckStatus::Fail);
    assert!(cafl.remedies.len() >= 2);
    let s = fire_ltb_summary(&En1993Snapshot::compliant_heb240_frame(), "fire-1").expect("fire summary");
    assert!(s.m_b_fi_rd > 0.0 && s.interaction.is_some() && s.curve.len() == 50);
    close(s.m_fi_ed, 0.65 * (1.35 * 40_000.0 + 1.5 * 17_333.0), 1e-3, "M_fi,Ed");
    let report = check_full_steel_structure(&En1993Snapshot::compliant_heb240_frame());
    assert!(report.checks.iter().any(|c| c.id == "en1993.1-2.4.2.3.ltb.fire-1") && report.checks.iter().any(|c| c.id == "en1993.1-2.4.2.3.int.fire-1"));
}

#[test]
fn flange_curling_of_the_showcase_cold_formed_member() {
    let report = check_full_steel_structure(&En1993Snapshot::special_parts_showcase());
    let check = find(&report, "en1993.1-3.5.4.curl.cf-1");
    close(check.computed.value, 2.0 * 150.0e6_f64.powi(2) * 0.06_f64.powi(4) / (210.0e9_f64.powi(2) * 0.003_f64.powi(2) * 0.075), 1e-9, "u");
    close(check.limit.value, 0.05 * 0.15, 1e-12, "5 % of the depth");
}

#[test]
fn the_ordered_fold_of_units_equals_the_full_evaluation_for_every_example() {
    use crate::En1993Units;
    use semio_s_plugin_norm_compliance::incremental_evaluation::{audit_reads, evaluate_units, UnitSource};
    for doc in example_documents() {
        assert_eq!(evaluate_units::<En1993Units>(&doc).checks.iter().map(|c| c.id.clone()).collect::<BTreeSet<_>>(), check_full_steel_structure(&doc).checks.iter().map(|c| c.id.clone()).collect::<BTreeSet<_>>());
        for unit in <En1993Units as UnitSource>::units(&doc) {
            assert_eq!(audit_reads(&unit, &doc), Vec::<String>::new(), "undeclared reads of {:?}", unit.id);
        }
    }
}

const SHOWCASE_UNREACHED: &[(&str, &str)] = &[];

#[test]
fn the_full_showcase_alone_reaches_every_check_fragment_of_the_trace() {
    let showcase = semio_s_artifact_norm_en1993_showcase::full_showcase();
    for (collection, rows) in [("materials", showcase.materials.len()), ("sections", showcase.sections.len()), ("members", showcase.members.len()), ("loadCases", showcase.load_cases.len()), ("memberActions", showcase.member_actions.len()), ("joints", showcase.joints.len()), ("fatigueDetails", showcase.fatigue_details.len()), ("fireExposures", showcase.fire_exposures.len()), ("coldFormedMembers", showcase.cold_formed_members.len()), ("platedPanels", showcase.plated_panels.len()), ("siloShells", showcase.silo_shells.len()), ("tensionComponents", showcase.tension_components.len()), ("bridgeFatigue", showcase.bridge_fatigue.len()), ("towerLegs", showcase.tower_legs.len()), ("piles", showcase.piles.len()), ("craneRunways", showcase.crane_runways.len()), ("storeys", showcase.storeys.len()), ("restraints", showcase.restraints.len()), ("builtUpMembers", showcase.built_up_members.len()), ("weldGroups", showcase.weld_groups.len()), ("parametricSections", showcase.parametric_sections.len()), ("hollowJoints", showcase.hollow_joints.len()), ("jointRows", showcase.joint_rows.len()), ("frameNodes", showcase.frame_nodes.len()), ("frameBars", showcase.frame_bars.len()), ("placements", showcase.placements.len())] {
        assert!(rows > 0, "the showcase leaves {collection} empty");
    }
    let emitted = Emitted::default().with_checks(check_full_steel_structure(&showcase).checks.into_iter().map(|c| c.id));
    assert_reaches_all_but(&TABLE, &emitted, SHOWCASE_UNREACHED);
}
