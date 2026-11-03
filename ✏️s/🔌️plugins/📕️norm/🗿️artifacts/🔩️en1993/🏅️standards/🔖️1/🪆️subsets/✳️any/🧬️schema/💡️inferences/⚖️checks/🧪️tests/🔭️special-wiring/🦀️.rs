//! 🔭️ Wiring tests: the special-part clause functions run per entity inside the EN 1993 evaluation with their check ids, remedies and incremental units.

use crate::{check_full_steel_structure, En1993Units};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{audit_reads, evaluate_units, UnitSource};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus};
use std::collections::BTreeSet;

const SHOWCASE_IDS: &[&str] = &[
    "en1993.1-3.5.5.dist.cf-1",
    "en1993.1-3.6.1.5.cf-1",
    "en1993.1-3.6.1.7.cf-1",
    "en1993.1-3.6.1.9.cf-1",
    "en1993.1-3.6.1.11.cf-1",
    "en1993.1-3.10.1.5.cf-1",
    "en1993.1-4.5.5.member-ss",
    "en1993.1-4.fire.fire-ss",
    "en1993.1-10.t.member-b1",
    "en1993.1-10.t.member-ss",
    "en1993.1-5.4.5.pp-1",
    "en1993.1-5.3.sl.pp-1",
    "en1993.1-5.6.a.pp-1",
    "en1993.1-5.7.2.pp-1",
    "en1993.1-5.7.1.pp-1",
    "en1993.1-5.9.3.pp-1",
    "en1993.1-5.9.2.pp-1",
    "en1993.1-7.lateral.pp-1",
    "en1993.1-5.8.pp-1",
    "en1993.1-5.10.pp-1",
    "en1993.1-6.8.thin.silo-1",
    "en1993.1-6.8.5.x.silo-1",
    "en1993.1-6.8.5.theta.silo-1",
    "en1993.1-6.8.5.tau.silo-1",
    "en1993.1-6.8.5.int.silo-1",
    "en1993.1-6.8.2.vm.silo-1",
    "en1993.4-1.5.2.ph.silo-1",
    "en1993.4-1.5.2.nx.silo-1",
    "en1993.4-2.6.silo-1",
    "en1993.4-3.5.t.pipe-1",
    "en1993.4-3.5.vm.pipe-1",
    "en1993.4-3.5.pcr.pipe-1",
    "en1993.1-8.6.3.mj.joint-ep",
    "en1993.1-8.5.2.sj.joint-ep",
    "en1993.1-8.6.2.base.joint-bp",
    "en1993.1-11.6.2.cable.ten-1",
    "en1993.1-8.3.6.rod.ten-1",
    "en1993.1-8.3.13.pin.ten-1",
    "en1993.1-8.3.13.pinb.ten-1",
    "en1993.2.9.5.br-1",
    "en1993.3-1.5.lambda.tw-1",
    "en1993.3-2.vortex.tw-1",
    "en1993.3-2.ovalling.tw-1",
    "en1993.5.5.2.m.pile-1",
    "en1993.5.5.2.v.pile-1",
    "en1993.5.5.2.il.pile-1",
    "en1993.6.5.7.vm.crane-1",
    "en1993.6.9.fat.crane-1",
];

const NEW_ONLY_PREFIXES: &[&str] = &["en1993.1-3.5.5.", "en1993.1-3.6.1.", "en1993.1-3.10.", "en1993.1-4.5.5.", "en1993.1-4.fire.", "en1993.1-5.4.5", "en1993.1-5.3.", "en1993.1-5.6.", "en1993.1-5.7.", "en1993.1-5.8.", "en1993.1-5.9.", "en1993.1-5.10.", "en1993.1-6.8.", "en1993.4-1.5.2.", "en1993.4-2.", "en1993.4-3.", "en1993.1-8.6.", "en1993.1-8.5.2.", "en1993.1-11.6.2.", "en1993.1-8.3.13.", "en1993.1-8.3.6.rod.", "en1993.2.9.5.", "en1993.3-1.5.", "en1993.3-2.", "en1993.5.5.2.", "en1993.6.5.7.", "en1993.6.9."];

fn showcase() -> CheckReport {
    check_full_steel_structure(&En1993Snapshot::special_parts_showcase())
}

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"))
}

fn close(a: f64, b: f64, rel: f64, what: &str) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: {a} vs {b}");
}

#[test]
fn every_special_part_check_is_present_for_the_showcase_subject() {
    let report = showcase();
    let ids: BTreeSet<&str> = report.checks.iter().map(|c| c.id.as_str()).collect();
    for id in SHOWCASE_IDS {
        assert!(ids.contains(id), "missing {id}");
    }
    assert_eq!(ids.len(), report.checks.len(), "check ids are unique");
}

#[test]
fn special_part_results_are_finite_and_every_failure_carries_a_remedy() {
    for check in showcase().checks.iter().filter(|c| SHOWCASE_IDS.contains(&c.id.as_str())) {
        assert!(check.utilization.is_finite() && check.utilization >= 0.0, "{}: {}", check.id, check.utilization);
        assert!(!check.explanation.en.is_empty() && !check.explanation.de.is_empty(), "{}", check.id);
        if check.status == CheckStatus::Fail {
            assert!(!check.remedies.is_empty(), "{} fails without a remedy", check.id);
        }
    }
}

#[test]
fn unverified_clause_values_block_a_compliant_verdict() {
    let report = showcase();
    assert!(report.unverified().count() > 0);
    assert!(!report.complies());
    for id in ["en1993.1-3.5.5.dist.cf-1", "en1993.1-6.8.5.x.silo-1", "en1993.5.5.2.m.pile-1"] {
        assert!(find(&report, id).ndps.iter().all(|n| n.starts_with("ndp.en1993.")), "{id}");
    }
}

#[test]
fn default_subjects_emit_no_special_part_check() {
    for doc in [En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::noncompliant_overloaded_frame()] {
        for check in check_full_steel_structure(&doc).checks {
            assert!(!NEW_ONLY_PREFIXES.iter().any(|p| check.id.starts_with(p)), "unexpected {}", check.id);
        }
    }
}

#[test]
fn brittle_fracture_uses_the_member_temperature_and_table_2_1() {
    let report = showcase();
    close(find(&report, "en1993.1-10.t.member-b1").utilization, 17.0 / 55.0, 1e-9, "S355 J2 at -20 degC");
    assert_eq!(find(&report, "en1993.1-10.t.member-ss").status, CheckStatus::NotApplicable);
}

#[test]
fn tank_course_and_web_shear_match_hand_formulas() {
    let report = showcase();
    close(find(&report, "en1993.4-2.6.silo-1").utilization, (1000.0 * 9.81 * 4.0 * 3.0 / 235.0e6) / 0.008, 1e-9, "tank course");
    let lambda_w: f64 = 0.346 * 150.0 / 3.0 * (350.0_f64 / 210000.0).sqrt();
    assert!(lambda_w <= 0.83);
    close(find(&report, "en1993.1-3.6.1.5.cf-1").utilization, 6000.0 / (150.0 * 3.0 * 0.58 * 350.0), 1e-9, "cold-formed web shear");
}

#[test]
fn cold_formed_axial_bending_adds_the_two_utilisations() {
    let report = showcase();
    let axial = find(&report, "en1993.1-3.cf.cf-1").utilization;
    close(find(&report, "en1993.1-3.6.1.9.cf-1").utilization, axial + 800.0 / 3000.0, 1e-9, "N_Ed/N_c,Rd + M_Ed/M_c,Rd");
}

#[test]
fn cold_formed_web_crippling_bending_and_restraint_follow_the_clause_functions() {
    use crate::clauses::cold_formed::*;
    let report = showcase();
    let (alpha, la) = web_crippling_alpha(2, 50.0);
    let rw = web_crippling_rd(alpha, 3.0, 350.0, 210000.0, 4.0, la, 90.0, 1.0);
    let u7 = find(&report, "en1993.1-3.6.1.7.cf-1").utilization;
    assert!(u7 * rw / 4000.0 >= 1.0 - 1e-12, "gamma_M1 applies to R_w,Rd");
    close(find(&report, "en1993.1-3.6.1.11.cf-1").utilization, bending_crippling_utilisation(800.0, 3000.0, 4000.0, 4000.0 / u7), 1e-9, "M + F");
    let c_d = c_d_series(4000.0, c_d_web_bending(210000.0, 3.0, 150.0));
    close(find(&report, "en1993.1-3.10.1.5.cf-1").utilization, 1000.0 / c_d, 1e-9, "C_D");
}

#[test]
fn v906_thin_shell_gate_compares_the_wall_slenderness_with_one_twentieth() {
    let report = showcase();
    let gate = find(&report, "en1993.1-6.8.thin.silo-1");
    close(gate.utilization, (8.0 / 3000.0) * 20.0, 1e-9, "t/r over 1/20");
    assert_eq!(gate.status, CheckStatus::Pass);
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.silo_shells.iter_mut().find(|s| s.id == "silo-1").expect("silo row").thickness = 0.3;
    let thick = find(&check_full_steel_structure(&doc), "en1993.1-6.8.thin.silo-1").clone();
    assert_eq!(thick.status, CheckStatus::Fail);
    assert_eq!(thick.remedies.len(), 1);
}

#[test]
fn v903_v904_v905_special_part_checks_run_for_their_subjects() {
    let report = showcase();
    for id in ["en1993.1-7.lateral.pp-1", "en1993.5.5.2.m.pile-1", "en1993.5.5.2.v.pile-1", "en1993.5.5.2.il.pile-1", "en1993.4-3.5.t.pipe-1", "en1993.4-3.5.vm.pipe-1", "en1993.4-3.5.pcr.pipe-1"] {
        assert!(find(&report, id).utilization.is_finite(), "{id}");
    }
}

#[test]
fn v902_high_strength_bending_runs_for_materials_from_s460() {
    let mut doc = En1993Snapshot::compliant_heb240_frame();
    assert!(!check_full_steel_structure(&doc).checks.iter().any(|c| c.id.starts_with("en1993.1-12.hss.")));
    doc.materials[0].fy = 460.0e6;
    doc.materials[0].fu = 550.0e6;
    assert!(check_full_steel_structure(&doc).checks.iter().any(|c| c.id == "en1993.1-12.hss.member-b1"));
}

#[test]
fn sheet_pile_corrosion_uses_the_interpolated_loss_of_both_faces() {
    let report = showcase();
    let t_net: f64 = 12.0 - 2.0 * 1.5;
    close(find(&report, "en1993.5.5.2.il.pile-1").utilization, 400.0 / (0.8 * 355.0 * t_net / 3.0_f64.sqrt()), 1e-9, "interlock shear");
}

#[test]
fn the_ordered_fold_of_units_equals_the_full_evaluation_for_the_showcase() {
    let doc = En1993Snapshot::special_parts_showcase();
    assert_eq!(evaluate_units::<En1993Units>(&doc), check_full_steel_structure(&doc));
}

#[test]
fn every_unit_declares_the_reads_of_the_special_part_checks() {
    let doc = En1993Snapshot::special_parts_showcase();
    for unit in <En1993Units as UnitSource>::units(&doc) {
        assert_eq!(audit_reads(&unit, &doc), Vec::<String>::new(), "undeclared reads of {:?}", unit.id);
    }
}

#[test]
fn storey_checks_follow_the_hand_formulas_of_en_1993_1_1_section_5() {
    let report = showcase();
    let alpha = find(&report, "en1993.1-1.5.2.alpha.storey-1");
    close(alpha.utilization, 3.0 / 100.0, 1e-9, "alpha_cr 100 gives 3/100");
    assert_eq!(alpha.status, CheckStatus::UnverifiedBasis, "Eq. 5.2 alone cannot prove its validity conditions");
    let phi = 0.005 * (0.5_f64 * 1.5).sqrt();
    let sway = find(&report, "en1993.1-1.5.3.2.sway.storey-1");
    assert_eq!(sway.status, CheckStatus::Info);
    close(sway.computed.value, phi * 4.0e5, 1e-9, "equivalent horizontal force");
    assert!(sway.explanation.en.contains("0.00433") && sway.explanation.de.contains("0.00433"));
}

#[test]
fn a_soft_storey_fails_with_drift_and_load_remedies_and_bracing_is_checked() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    let storey = &mut doc.storeys[0];
    storey.drift = 0.1;
    storey.bracing_load = 1.0e6;
    storey.bracing_length = 6.0;
    storey.bracing_deflection = 0.02;
    let report = check_full_steel_structure(&doc);
    let alpha = find(&report, "en1993.1-1.5.2.alpha.storey-1");
    assert_eq!(alpha.status, CheckStatus::Fail);
    close(alpha.utilization, 3.0 / 2.0, 1e-9, "alpha_cr 2 gives 3/2");
    assert_eq!(alpha.remedies.len(), 2);
    let bracing = find(&report, "en1993.1-1.5.3.3.bracing.storey-1");
    close(bracing.utilization, 0.02 / (6.0 / 500.0), 1e-9, "bracing deflection against L/500");
    assert_eq!(bracing.status, CheckStatus::Fail);
}

#[test]
fn storey_rows_alone_are_a_subject_and_default_frames_have_no_storey_check() {
    let doc = En1993Snapshot { members: Vec::new(), joints: Vec::new(), ..En1993Snapshot::special_parts_showcase() };
    assert!(check_full_steel_structure(&doc).checks.iter().any(|c| c.id.starts_with("en1993.1-1.5.2.alpha.")));
    for doc in [En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::noncompliant_overloaded_frame()] {
        assert!(!check_full_steel_structure(&doc).checks.iter().any(|c| c.id.contains(".5.2.alpha.")));
    }
}

#[test]
fn restraint_mcr_by_finite_elements_matches_the_independent_galerkin_oracle() {
    let report = showcase();
    let check = find(&report, "en1993.1-1.6.3.2.mcrfe.restraint-1");
    close(check.utilization, 0.33349531573886737, 1.5e-2, "M_Ed/M_b,Rd with the FE M_cr of a UDL on a fork-fork HEB 240 over 4 m and W_pl,y of the class 1 section");
    assert_eq!(check.status, CheckStatus::UnverifiedBasis, "the resisting-modulus rule is cited as an unverified registry entry");
    assert!(check.explanation.en.contains("finite elements") && check.explanation.de.contains("Finiten-Elementen"));
}

#[test]
fn a_load_on_the_top_flange_lowers_mcr_and_fails_with_a_spring_and_a_moment_remedy() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.restraints[0].load_height = 0.12;
    doc.restraints[0].design_moment = 300_000.0;
    let report = check_full_steel_structure(&doc);
    let check = find(&report, "en1993.1-1.6.3.2.mcrfe.restraint-1");
    close(check.utilization, 1.057569338227297, 2e-2, "top flange loading");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_eq!(check.remedies.len(), 2);
}

#[test]
fn the_general_method_equals_the_lateral_torsional_check_for_pure_bending() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.restraints[0].general_method = 1;
    let report = check_full_steel_structure(&doc);
    close(find(&report, "en1993.1-1.6.3.4.general.restraint-1").utilization, find(&report, "en1993.1-1.6.3.2.mcrfe.restraint-1").utilization, 1e-6, "general method and 6.3.2.2 for N_Ed = 0");
}

#[test]
fn the_stable_length_next_to_a_plastic_hinge_follows_equation_6_67() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.restraints[0].hinge_to_restraint = 14.0;
    let report = check_full_steel_structure(&doc);
    let check = find(&report, "en1993.1-1.6.3.5.lm.restraint-1");
    close(check.utilization, 14000.0 / 11529.290821604922, 1e-3, "L_m of HEB 240 with C_1 = 2.752 (psi = -1), i_z = 60.8 mm");
    assert_eq!(check.status, CheckStatus::Fail);
}

#[test]
fn the_battened_chord_force_matches_the_independent_oracle() {
    let report = showcase();
    let check = find(&report, "en1993.1-1.6.4.chord.built-up-1");
    close(check.utilization, 0.1301102538395026, 1e-9, "N_ch,Ed / N_b,Rd of the battened showcase member");
    assert_eq!(check.status, CheckStatus::Pass);
    close(check.computed.value, 225095.7570726212, 1e-9, "chord design force");
}

#[test]
fn an_unstable_built_up_member_is_reported_and_closely_spaced_members_obey_table_6_9() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.built_up_members[0].design_axial = 5.0e7;
    let unstable = check_full_steel_structure(&doc);
    assert_eq!(find(&unstable, "en1993.1-1.6.4.chord.built-up-1").status, CheckStatus::Fail);
    let mut doc = En1993Snapshot::special_parts_showcase();
    let row = &mut doc.built_up_members[0];
    row.kind = semio_s_artifact_norm_en1993_model::BuiltUpKind::Closely;
    row.min_radius = 0.04;
    row.chain_spacing = 0.9;
    row.back_to_back = 1;
    let report = check_full_steel_structure(&doc);
    close(find(&report, "en1993.1-1.6.4.4.spacing.built-up-1").utilization, 0.9 / (15.0 * 0.04), 1e-12, "Table 6.9 angles back to back");
    assert!(report.checks.iter().all(|c| c.id != "en1993.1-1.6.4.chord.built-up-1"));
}

#[test]
fn the_eccentric_weld_group_follows_the_elastic_vector_method() {
    let report = showcase();
    let check = find(&report, "en1993.1-8.4.5.group.weld-group-1");
    close(check.utilization, 0.6572474878554899, 1e-9, "F_w,Ed / F_w,Rd of two 200 mm lines at 100 mm, e = 150 mm");
    close(check.computed.value, 860.113898485164, 1e-9, "resultant force per length in N/mm");
    assert_eq!(check.status, CheckStatus::Pass);
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.weld_groups[0].eccentricity = 0.4;
    let failing = check_full_steel_structure(&doc);
    let check = find(&failing, "en1993.1-8.4.5.group.weld-group-1");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_eq!(check.remedies.len(), 2);
}

#[test]
fn parametric_section_properties_match_the_polygon_oracle_and_the_catalogue_row() {
    let report = showcase();
    let check = find(&report, "en1993.sections.param.param-1");
    close(check.utilization, 0.7169811320754717, 1e-9, "largest deviation (area, root radii) over the 5 % tolerance");
    assert_eq!(check.status, CheckStatus::Pass);
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.parametric_sections[0].flange_thickness = 0.012;
    let failing = check_full_steel_structure(&doc);
    let check = find(&failing, "en1993.sections.param.param-1");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_eq!(check.remedies.len(), 1);
}

#[test]
fn the_rhs_gap_joint_matches_the_independent_formulas_and_flags_the_validity_range() {
    let report = showcase();
    let check = find(&report, "en1993.1-8.7.4.gap.hollow-1");
    close(check.utilization, 0.11492077502989248, 1e-9, "chord face failure governs the showcase K joint");
    assert!(check.explanation.en.contains("inside the validity range"));
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.hollow_joints[0].gap = 0.2;
    doc.hollow_joints[0].brace1_force = 600000.0;
    let failing = check_full_steel_structure(&doc);
    let check = find(&failing, "en1993.1-8.7.4.gap.hollow-1");
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(check.explanation.en.contains("OUTSIDE the validity range"));
    assert_eq!(check.remedies.len(), 2);
}

#[test]
fn the_fixed_base_portal_eigenvalue_replaces_the_horne_formula_and_matches_the_blessed_cases() {
    use crate::clauses::global::{alpha_cr_portal_fixed, horne_excess, horne_tolerance};
    for (ib, horne, eigen) in [(8e-4, 11.70, 9.97), (6e-5, 7.19, 6.39)] {
        let got = alpha_cr_portal_fixed(210e9, 8e-5, ib, 4.0, 6.0, 1.0e6).unwrap();
        close(got, eigen, 2e-2, "closed-form sway eigenvalue against the blessed scipy and PyNite P-Delta value");
        assert!(horne_excess(horne, got) > horne_tolerance(), "Eq. 5.2 overshoots the eigenvalue of the portal by more than the accepted tolerance");
    }
}

fn frame_report() -> CheckReport {
    check_full_steel_structure(&En1993Snapshot::frame_portal_showcase())
}

#[test]
fn the_frame_model_check_reports_the_kernel_eigenvalue_and_the_required_analysis() {
    let doc = En1993Snapshot::frame_portal_showcase();
    let report = frame_report();
    let check = find(&report, "en1993.1-1.5.2.frame.bar-1");
    let alpha = crate::frame_eigen::alpha_cr(&doc).unwrap();
    close(check.utilization, 3.0 / alpha, 1e-9, "utilisation is the minimum alpha_cr over the computed one");
    assert!(check.explanation.en.contains("Kernel eigenvalue of 4 nodes and 3 bars") && check.explanation.de.contains("Kern-Eigenwert mit 4 Knoten und 3 Stäben"));
}

#[test]
fn semi_rigid_beam_ends_take_the_secant_stiffness_of_the_end_plate_joint_of_their_member() {
    let mut doc = En1993Snapshot::frame_portal_showcase();
    let mut joint = En1993Snapshot::special_parts_showcase().joints.into_iter().find(|j| j.kind == semio_s_artifact_norm_en1993_model::JointKind::EndPlate).unwrap();
    joint.member_id = doc.frame_bars[1].member_id.clone();
    doc.joints.push(joint.clone());
    let secant = crate::joint_secant(&doc, &joint).expect("end plate joint has a component result");
    let summary = crate::joint_row_summary(&doc, &joint.id).unwrap();
    assert!(secant > 0.0 && secant <= summary.s_ini * (1.0 + 1e-12), "S_j = S_j,ini/mu never exceeds S_j,ini");
    let rigid = crate::frame_eigen::alpha_cr(&doc).unwrap();
    for bar in [1_usize] {
        doc.frame_bars[bar].release_start = semio_s_artifact_norm_en1993_model::FrameBarRelease::Pinned;
        doc.frame_bars[bar].release_end = semio_s_artifact_norm_en1993_model::FrameBarRelease::Pinned;
    }
    let pinned = crate::frame_eigen::alpha_cr(&doc).unwrap();
    for bar in [1_usize] {
        doc.frame_bars[bar].release_start = semio_s_artifact_norm_en1993_model::FrameBarRelease::SemiRigid;
        doc.frame_bars[bar].release_end = semio_s_artifact_norm_en1993_model::FrameBarRelease::SemiRigid;
    }
    let semi = crate::joint_frame::alpha_cr(&doc).unwrap();
    assert!(pinned < semi && semi <= rigid * (1.0 + 1e-6), "{pinned} < {semi} <= {rigid}");
    assert!(crate::frame_eigen::alpha_cr(&doc).is_err(), "without the joint stiffness a semi-rigid end is refused");
}

#[test]
fn a_slender_frame_fails_with_a_load_remedy_and_a_mechanism_is_not_applicable() {
    let mut doc = En1993Snapshot::frame_portal_showcase();
    doc.frame_nodes.iter_mut().for_each(|n| n.load_z *= 20.0);
    let report = check_full_steel_structure(&doc);
    let check = find(&report, "en1993.1-1.5.2.frame.bar-1");
    assert_eq!(check.status, CheckStatus::Fail);
    assert_eq!(check.remedies.len(), 1);
    doc.frame_nodes.iter_mut().for_each(|n| n.support = semio_s_artifact_norm_en1993_model::FrameNodeSupport::Free);
    let mechanism = check_full_steel_structure(&doc);
    assert_eq!(find(&mechanism, "en1993.1-1.5.2.frame.bar-1").status, CheckStatus::NotApplicable);
}

#[test]
fn the_storey_check_uses_the_frame_eigenvalue_and_the_frame_units_fold_to_the_full_evaluation() {
    let mut doc = En1993Snapshot::frame_portal_showcase();
    doc.storeys = En1993Snapshot::special_parts_showcase().storeys;
    let report = check_full_steel_structure(&doc);
    let frame = find(&report, "en1993.1-1.5.2.frame.bar-1").utilization;
    close(find(&report, "en1993.1-1.5.2.alpha.storey-1").utilization, frame, 1e-9, "storey alpha_cr is the frame sway eigenvalue");
    assert_eq!(evaluate_units::<En1993Units>(&doc), report);
    for unit in <En1993Units as UnitSource>::units(&doc) {
        assert_eq!(audit_reads(&unit, &doc), Vec::<String>::new(), "undeclared reads of {:?}", unit.id);
    }
}

#[test]
fn a_sway_sensitive_frame_gets_second_order_rows_with_the_imperfections_of_5_3_2() {
    let mut doc = En1993Snapshot::frame_portal_showcase();
    doc.frame_nodes.iter_mut().for_each(|n| n.load_z *= 6.0);
    let alpha = crate::frame_eigen::alpha_cr(&doc).unwrap();
    assert!(alpha < 10.0 && alpha > 3.0, "alpha_cr {alpha}");
    let report = check_full_steel_structure(&doc);
    for bar in ["bar-1", "bar-2", "bar-3"] {
        let check = find(&report, &format!("en1993.1-1.5.2.second-order.{bar}"));
        assert!(check.explanation.en.contains("5.2.2(3)c") && check.explanation.en.contains("sway imperfection φ=") && check.explanation.de.contains("Schiefstellung φ="));
    }
    let imperfect = find(&report, "en1993.1-1.5.2.second-order.bar-1").utilization;
    let plain = crate::frame_eigen::solve_second_order(&doc).unwrap();
    assert!(plain.bar("bar-1").unwrap().second.m > 0.0 && imperfect > 0.0);
    assert!(report.checks.iter().filter(|c| c.id.starts_with("en1993.1-1.5.2.member.")).all(|c| c.explanation.en.contains("5.2.2(3)b") && c.explanation.en.contains("6.3.3")));
}

#[test]
fn the_lateral_pressure_row_is_k_p_b_squared_over_t_squared() {
    let report = check_full_steel_structure(&En1993Snapshot::special_parts_showcase());
    let check = find(&report, "en1993.1-7.lateral.pp-1");
    close(check.computed.value, 0.5 * 0.02 * 600.0_f64.powi(2) / 12.0_f64.powi(2) * 1e6, 1e-9, "sigma = k p b^2/t^2 of 25 MPa");
    assert!(check.explanation.en.contains("not verified") && check.explanation.de.contains("ungeprüft"));
}

#[test]
fn the_stiffener_torsion_row_takes_it_ip_and_iw_from_the_outline() {
    let report = check_full_steel_structure(&En1993Snapshot::special_parts_showcase());
    let check = find(&report, "en1993.1-5.9.2.pp-1");
    assert!(check.explanation.en.contains("analyse_section") && check.explanation.en.contains("I_w="));
    let mut doc = En1993Snapshot::special_parts_showcase();
    doc.plated_panels[0].stiffener_shape = semio_s_artifact_norm_en1993_model::StiffenerShape::None;
    assert!(check_full_steel_structure(&doc).checks.iter().all(|c| c.id != "en1993.1-5.9.2.pp-1"));
}

#[test]
fn the_rainflow_history_is_counted_into_the_damage_blocks() {
    let report = check_full_steel_structure(&semio_s_artifact_norm_en1993_showcase::full_showcase());
    let check = find(&report, "en1993.1-9.fat.fat-rf");
    assert!(check.explanation.en.contains("7 rainflow blocks of 11 history points"), "{}", check.explanation.en);
    let blocks = crate::clauses::fatigue_fire::rainflow(&[0.0, 80.0, 10.0, 60.0, 20.0, 70.0, 0.0, 90.0, 30.0, 50.0, 0.0]);
    let mut sorted = blocks.clone();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    assert_eq!(sorted, vec![(20.0, 1.0), (40.0, 1.0), (60.0, 1.0), (80.0, 0.5), (80.0, 0.5), (90.0, 0.5), (90.0, 0.5)]);
}

#[test]
fn the_hybrid_girder_is_classified_with_the_web_strength() {
    let report = check_full_steel_structure(&semio_s_artifact_norm_en1993_showcase::full_showcase());
    let check = find(&report, "en1993.1-1.5.5.hybrid.member-hybrid");
    assert!(check.explanation.en.contains("f_y,w=355") && check.explanation.en.contains("f_y,f=460"));
}

#[test]
fn the_bimoment_row_adds_the_warping_stress_to_the_bending_stress() {
    let report = check_full_steel_structure(&semio_s_artifact_norm_en1993_showcase::full_showcase());
    let check = report.checks.iter().find(|c| c.id.starts_with("en1993.6.2.7.w.")).expect("bimoment row");
    assert!(check.explanation.en.contains("B_Ed,max") && check.explanation.en.contains("not verified against the print"));
}

#[test]
fn the_warping_support_of_the_member_changes_the_bimoment_row() {
    let mut doc = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let utilisation = |doc: &En1993Snapshot| check_full_steel_structure(doc).checks.iter().find(|c| c.id.starts_with("en1993.6.2.7.w.") && c.subject.entity_id == "member-torsion").map(|c| c.utilization).expect("bimoment row");
    let fixed = utilisation(&doc);
    doc.members.iter_mut().find(|m| m.id == "member-torsion").expect("member").warping_support = semio_s_artifact_norm_en1993_model::WarpingSupport::Cantilever;
    let cantilever = utilisation(&doc);
    doc.members.iter_mut().find(|m| m.id == "member-torsion").expect("member").warping_support = semio_s_artifact_norm_en1993_model::WarpingSupport::Fork;
    let fork = utilisation(&doc);
    assert!(fixed != fork && cantilever != fork && cantilever > fork, "fixed {fixed}, cantilever {cantilever}, fork {fork}");
}

#[test]
fn the_compression_zone_check_takes_the_least_of_web_compression_panel_shear_and_beam_resistance() {
    let doc = En1993Snapshot::special_parts_showcase();
    let joint = doc.joints.iter().find(|j| j.kind == semio_s_artifact_norm_en1993_model::JointKind::EndPlate && j.moment_demand != 0.0).expect("end plate joint with a design moment");
    let summary = crate::joint_row_summary(&doc, &joint.id).unwrap();
    close(summary.beam_compression, joint.beam_moment_resistance * 1e3 / (joint.compression_lever * 1e3), 1e-12, "F_c,fb,Rd = M_c,Rd/(h - t_fb)");
    close(summary.compression_cap, summary.column_web_compression.min(summary.panel_shear).min(summary.beam_compression), 1e-12, "cap is the least compression component");
    let report = check_full_steel_structure(&doc);
    let check = find(&report, &format!("en1993.1-8.6.2.comp.{}", joint.id));
    close(check.utilization, joint.moment_demand.abs() * 1e3 / (summary.z * 1e3) / summary.compression_cap, 1e-9, "F_c,Ed = M_j,Ed/z over the cap");
    close(find(&report, "en1993.1-8.6.2.row.row-1").utilization, 0.77363079437377, 1e-6, "independent oracle: F_tr,Ed = 205.1 kN over min(column flange, end plate, column web) = 265.2 kN");
    close(find(&report, "en1993.1-8.6.2.row.row-2").utilization, 0.4261645205053071, 1e-6, "independent oracle: F_tr,Ed = 136.8 kN over the column flange in bending 320.9 kN (T-stub mode 2)");
    close(check.utilization, 0.8535036680184495, 1e-6, "independent oracle: F_c,Ed = 347.7 kN over the beam flange and web in compression F_c,fb,Rd = 407.4 kN");
    close(summary.beam_compression, 407407.4074074074, 1e-9, "F_c,fb,Rd = 110 kNm / 270 mm");
    assert!(check.explanation.en.contains("governing") && check.explanation.de.contains("maßgebend"));
}

#[test]
fn the_plastic_analysis_gate_needs_class_1_a_restraint_at_the_hinge_and_the_stable_length() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    let member = doc.restraints[0].member_id.clone();
    doc.members.iter_mut().find(|m| m.id == member).unwrap().analysis = semio_s_artifact_norm_en1993_model::AnalysisKind::Plastic;
    doc.restraints[0].hinge_to_restraint = 0.0;
    let id = format!("en1993.5.6.plastic.{member}");
    assert_eq!(find(&check_full_steel_structure(&doc), &id).status, CheckStatus::NotApplicable, "no hinge distance, no verdict");
    doc.restraints[0].hinge_to_restraint = 14.0;
    let report = check_full_steel_structure(&doc);
    let gate = find(&report, &id);
    assert!(gate.utilization >= 14000.0 / 11529.290821604922 * (1.0 - 1e-3), "the stable length 6.3.5.3 governs: {}", gate.utilization);
    assert_eq!(gate.status, CheckStatus::Fail);
    assert_eq!(gate.remedies.len(), 1);
    doc.members.iter_mut().find(|m| m.id == member).unwrap().analysis = semio_s_artifact_norm_en1993_model::AnalysisKind::Elastic;
    assert!(check_full_steel_structure(&doc).checks.iter().all(|c| c.id != id), "elastic analysis has no plastic gate");
}

#[test]
fn w3_a_thin_beam_web_governs_the_row_and_the_independent_kernel_value_is_reproduced() {
    use crate::clauses::joint_components::{ft_wb_rd, RowComponent};
    let mut doc = En1993Snapshot::special_parts_showcase();
    let joint_id = doc.joints.iter().find(|j| j.kind == semio_s_artifact_norm_en1993_model::JointKind::EndPlate && j.moment_demand != 0.0).expect("end plate joint").id.clone();
    let before = crate::joint_row_summary(&doc, &joint_id).unwrap();
    assert!(before.component_forces.iter().all(|f| f[3].is_infinite()), "beam web not given: no fourth component");
    let joint = doc.joints.iter_mut().find(|j| j.id == joint_id).unwrap();
    joint.beam_web_thickness = 0.0025;
    joint.beam_web_fy = 355.0e6;
    let after = crate::joint_row_summary(&doc, &joint_id).unwrap();
    for (row, forces) in after.component_forces.iter().enumerate() {
        let b_eff = forces[3] * 1.0 / (2.5 * 355.0);
        close(ft_wb_rd(b_eff, 2.5, 355.0, 1.0), forces[3], 1e-12, "F_t,wb,Rd = b_eff t_wb f_y,wb / gamma_M0");
        assert!(forces[3] <= forces[0].max(forces[1]).max(forces[2]) || after.governing[row] == RowComponent::BeamWebTension);
        assert!(after.row_forces[row] <= forces[3] + 1e-6, "the plastic row force never exceeds the beam web in tension");
    }
    assert!(after.governing.contains(&RowComponent::BeamWebTension));
    let report = check_full_steel_structure(&doc);
    assert!(find(&report, "en1993.1-8.6.2.row.row-1").utilization > 0.77363079437377);
}
