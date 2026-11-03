//! 🧮️ Wiring tests of the wave 3 norm checks: each family evaluates the rows of a document against the numbers of the python oracles (`🔮️oracles/🔩️bolt-group-ic`, `🪜️effective-length`, `⏳️stress-range`, `🪛️fasteners`, `🥫️forming-gain`, `🧪️ductility`), the remedies solve their check and the unit fold equals the full evaluation.

use crate::{check_full_steel_structure, En1993Units};
use semio_s_artifact_norm_en1993_model::{
    AnalysisKind, BoltGroupMethod, BucklingLength, BucklingPlane, En1993Snapshot, FastenedJoint, FastenerKind, FatigueStress, FormingGain, FormingProcess, FrameSway, MaterialDuctility, NodeARestraint, NodeBRestraint, StressRelief,
};
use semio_s_plugin_norm_compliance::incremental_evaluation::{audit_reads, evaluate_units, UnitSource};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, RemedyBound};

fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing check {id}"))
}

fn close(a: f64, b: f64, tolerance: f64, what: &str) {
    assert!((a - b).abs() <= tolerance * b.abs().max(1e-12), "{what}: {a} vs {b}");
}

fn base() -> En1993Snapshot {
    En1993Snapshot::special_parts_showcase()
}

fn stress_range(id: &str, relief: StressRelief, cycles: f64) -> FatigueStress {
    FatigueStress { id: id.into(), detail_id: "fat-1".into(), max_case_id: "g-permanent".into(), min_case_id: "q-imposed".into(), fibre_z: 0.12, fibre_y: 0.0, relief, concentration: 1.0, thickness: 0.017, cycles }
}

fn buckling(id: &str, ga: f64, gb: f64, sway: FrameSway) -> BucklingLength {
    BucklingLength { id: id.into(), member_id: "member-b1".into(), plane: BucklingPlane::MajorAxis, sway, restraint_a: NodeARestraint::Framed, column_stiffness_a: ga * 5.63e-5, beam_stiffness_a: 5.63e-5, restraint_b: NodeBRestraint::Framed, column_stiffness_b: gb * 5.63e-5, beam_stiffness_b: 5.63e-5 }
}

fn screws(id: &str) -> FastenedJoint {
    FastenedJoint { id: id.into(), member_id: "cf-1".into(), kind: FastenerKind::SelfTappingScrew, diameter: 0.0055, washer_diameter: 0.016, count: 4, per_row: 2, head_thickness: 0.00075, base_thickness: 0.0015, head_strength: 360.0e6, base_strength: 360.0e6, end_distance: 0.03, edge_distance: 0.015, pitch: 0.03, gauge: 0.03, sheet_width: 0.1, shear_force: 4000.0, tension_force: 1200.0, shear_resistance: 0.0 }
}

#[test]
fn the_stress_range_row_matches_the_oracle_and_its_remedies_solve_the_damage() {
    let mut doc = base();
    doc.fatigue_stresses.push(stress_range("fs", StressRelief::StressRelieved, 1.0e7));
    let r = check_full_steel_structure(&doc);
    close(find(&r, "en1993.1-9.5.range.fs").utilization, 0.11409677267523886, 1e-9, "damage n / N_R of the 29.5 MPa range");
    doc.fatigue_stresses[0].cycles = 2.0e8;
    let failing = check_full_steel_structure(&doc);
    let check = find(&failing, "en1993.1-9.5.range.fs");
    assert!(check.utilization > 1.0);
    let cycles = check.remedies.iter().find(|m| m.bound == RemedyBound::AtMost).expect("cycles remedy");
    doc.fatigue_stresses[0].cycles = cycles.required.value;
    assert!(find(&check_full_steel_structure(&doc), "en1993.1-9.5.range.fs").utilization <= 1.0);
    assert!(check.remedies.iter().any(|m| m.bound == RemedyBound::OneOf && m.target.path.ends_with(".category")), "better category");
}

#[test]
fn the_size_effect_and_the_stress_concentration_scale_the_design_range() {
    let mut doc = base();
    let mut thick = stress_range("fs", StressRelief::AsWelded, 1.0e7);
    thick.thickness = 0.040;
    thick.concentration = 1.5;
    doc.fatigue_stresses.push(thick);
    let u = find(&check_full_steel_structure(&doc), "en1993.1-9.5.range.fs").utilization;
    let plain = {
        let mut d = base();
        d.fatigue_stresses.push(stress_range("fs", StressRelief::AsWelded, 1.0e7));
        find(&check_full_steel_structure(&d), "en1993.1-9.5.range.fs").utilization
    };
    assert!(u > plain, "k_f 1.5 and k_s 0.91 raise the damage ({u} against {plain})");
}

#[test]
fn the_buckling_length_follows_the_nomogram_and_the_remedy_sets_the_member_length() {
    let mut doc = base();
    doc.buckling_lengths.push(buckling("bl", 1.0, 1.0, FrameSway::NonSway));
    let r = check_full_steel_structure(&doc);
    close(find(&r, "en1993.1-1.6.3.1.3.lcr.bl").utilization, 0.7742650686480754, 1e-9, "K G=1 non-sway over the entered L = 4 m");
    doc.buckling_lengths[0] = buckling("bl", 1.0, 1.0, FrameSway::Sway);
    if let Some(m) = doc.members.iter_mut().find(|m| m.id == "member-b1") {
        m.buckling_length_y = 4.0;
    }
    let sway = check_full_steel_structure(&doc);
    let check = find(&sway, "en1993.1-1.6.3.1.3.lcr.bl");
    close(check.utilization, 1.3172751026289118, 1e-9, "sway factor against the entered length");
    let remedy = check.remedies.iter().find(|m| m.bound == RemedyBound::AtLeast).expect("buckling length remedy");
    if let Some(m) = doc.members.iter_mut().find(|m| m.id == "member-b1") {
        m.buckling_length_y = remedy.required.value;
    }
    assert!(find(&check_full_steel_structure(&doc), "en1993.1-1.6.3.1.3.lcr.bl").utilization <= 1.0);
}

#[test]
fn a_sway_column_pinned_at_both_ends_is_a_mechanism_with_an_advisory() {
    let mut doc = base();
    let mut row = buckling("bl", 1.0, 1.0, FrameSway::Sway);
    row.restraint_a = NodeARestraint::Pinned;
    row.restraint_b = NodeBRestraint::Pinned;
    doc.buckling_lengths.push(row);
    let check = find(&check_full_steel_structure(&doc), "en1993.1-1.6.3.1.3.lcr.bl").clone();
    assert!(check.utilization > 1.0 && check.remedies.iter().all(|m| !m.applicable));
}

#[test]
fn the_fastened_joint_matches_the_oracle_for_shear_tension_interaction_net_and_spacing() {
    let mut doc = base();
    doc.fastened_joints.push(screws("fj"));
    let r = check_full_steel_structure(&doc);
    close(find(&r, "en1993.1-3.8.3.shear.fj").utilization, 0.3330730243182193, 1e-9, "bearing");
    close(find(&r, "en1993.1-3.8.3.tension.fj").utilization, 0.2805836139169473, 1e-9, "pull-out");
    close(find(&r, "en1993.1-3.8.3.interaction.fj").utilization, 0.6136566382351666, 1e-9, "interaction");
    close(find(&r, "en1993.1-3.8.3.net.fj").utilization, 0.20807324178110692, 1e-9, "net section");
    assert!(find(&r, "en1993.1-3.8.3.spacing.fj").utilization < 1.0);
}

#[test]
fn more_fasteners_solve_an_overloaded_joint_and_a_small_edge_distance_is_lengthened() {
    let mut doc = base();
    let mut row = screws("fj");
    row.shear_force = 20000.0;
    row.edge_distance = 0.005;
    doc.fastened_joints.push(row);
    let r = check_full_steel_structure(&doc);
    let shear = find(&r, "en1993.1-3.8.3.shear.fj");
    assert!(shear.utilization > 1.0);
    doc.fastened_joints[0].count = shear.remedies[0].required.value as u32;
    doc.fastened_joints[0].edge_distance = find(&r, "en1993.1-3.8.3.spacing.fj").remedies[0].required.value;
    let solved = check_full_steel_structure(&doc);
    assert!(find(&solved, "en1993.1-3.8.3.shear.fj").utilization <= 1.0);
    assert!(find(&solved, "en1993.1-3.8.3.spacing.fj").utilization <= 1.0 + 1e-9);
}

#[test]
fn a_blind_rivet_in_tension_is_not_assessed_without_a_manufacturer_value() {
    let mut doc = base();
    let mut row = screws("fj");
    row.kind = FastenerKind::BlindRivet;
    doc.fastened_joints.push(row);
    let r = check_full_steel_structure(&doc);
    assert_eq!(find(&r, "en1993.1-3.8.3.tension.fj").status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
    assert!(r.checks.iter().all(|c| c.id != "en1993.1-3.8.3.interaction.fj"));
}

#[test]
fn the_forming_gain_limits_the_yield_strength_the_design_uses() {
    let mut doc = base();
    doc.forming_gains.push(FormingGain { id: "fg".into(), member_id: "cf-1".into(), forming: FormingProcess::RollForming, ultimate_strength: 420.0e6, gross_area: 0.0006, bends: 4, used_yield: 370.0e6 });
    close(find(&check_full_steel_structure(&doc), "en1993.1-3.3.2.2.fya.fg").utilization, 370.0e6 / 379.4e6, 1e-9, "used over f_ya");
    doc.forming_gains[0].used_yield = 400.0e6;
    let r = check_full_steel_structure(&doc);
    let check = find(&r, "en1993.1-3.3.2.2.fya.fg");
    assert!(check.utilization > 1.0);
    close(check.remedies[0].required.value, 379.4e6, 1e-9, "f_ya remedy");
}

#[test]
fn a_member_with_a_large_corner_radius_gets_no_work_hardening_from_its_bends() {
    let mut doc = base();
    if let Some(m) = doc.cold_formed_members.iter_mut().find(|m| m.id == "cf-1") {
        m.corner_radius = 0.05;
    }
    doc.forming_gains.push(FormingGain { id: "fg".into(), member_id: "cf-1".into(), forming: FormingProcess::RollForming, ultimate_strength: 420.0e6, gross_area: 0.0006, bends: 4, used_yield: 360.0e6 });
    let check = find(&check_full_steel_structure(&doc), "en1993.1-3.3.2.2.fya.fg").clone();
    assert!(check.utilization > 1.0, "r > 5 t: f_ya stays at f_yb = 350 MPa");
}

#[test]
fn the_ductility_row_checks_the_elongation_and_the_strain_and_gates_plastic_analysis() {
    let mut doc = base();
    doc.material_ductility.push(MaterialDuctility { id: "md".into(), material_id: "mat-s355".into(), elongation: 0.20, ultimate_strain: 0.12 });
    if let Some(m) = doc.members.iter_mut().find(|m| m.id == "member-b1") {
        m.analysis = AnalysisKind::Plastic;
    }
    let r = check_full_steel_structure(&doc);
    close(find(&r, "en1993.3.2.2.elongation.md").utilization, 0.75, 1e-9, "15 % over 20 %");
    close(find(&r, "en1993.3.2.2.strain.md").utilization, 0.2113095238095238, 1e-9, "15 eps_y over eps_u");
    close(find(&r, "en1993.5.6.ductility.member-b1").utilization, 0.7656862745098041, 1e-9, "f_u/f_y governs");
    doc.material_ductility[0].ultimate_strain = 0.02;
    let low = check_full_steel_structure(&doc);
    let gate = find(&low, "en1993.5.6.ductility.member-b1");
    close(gate.utilization, 1.2678571428571428, 1e-9, "eps_u governs");
    assert!(gate.remedies.iter().any(|m| m.bound == RemedyBound::OneOf && m.options == vec!["Elastic".to_owned()]));
}

#[test]
fn the_instantaneous_centre_check_replaces_the_elastic_check_of_its_plate() {
    let mut doc = base();
    let mut plate = doc.joint_plates.iter().find(|p| p.id == "plate-fin").cloned().expect("fin plate");
    plate.id = "plate-ic".into();
    plate.bolts_per_row = 2;
    plate.gauge = 0.1;
    plate.eccentricity = 0.15;
    plate.group_method = BoltGroupMethod::InstantaneousCentre;
    doc.joint_plates.push(plate);
    let r = check_full_steel_structure(&doc);
    close(find(&r, "en1993.1-8.3.12.ic.plate-ic").utilization, 1.0 / 7.591173736544717, 1e-6, "1 / load factor of the oracle");
    assert!(r.checks.iter().all(|c| c.id != "en1993.1-8.3.6.plate.plate-ic"), "the elastic check of this plate is replaced");
    assert!(r.checks.iter().any(|c| c.id == "en1993.1-8.3.6.plate.plate-fin"), "the elastic plates keep their check");
}

#[test]
fn an_overloaded_instantaneous_centre_group_offers_the_eccentricity_that_solves_it() {
    let mut doc = base();
    let mut plate = doc.joint_plates.iter().find(|p| p.id == "plate-fin").cloned().expect("fin plate");
    plate.id = "plate-ic".into();
    plate.bolts_per_row = 2;
    plate.gauge = 0.1;
    plate.eccentricity = 3.0;
    plate.group_method = BoltGroupMethod::InstantaneousCentre;
    doc.joint_plates.push(plate);
    let r = check_full_steel_structure(&doc);
    let check = find(&r, "en1993.1-8.3.12.ic.plate-ic");
    assert!(check.utilization > 1.0);
    let remedy = check.remedies.iter().find(|m| m.bound == RemedyBound::AtMost).expect("eccentricity remedy");
    doc.joint_plates.last_mut().expect("plate").eccentricity = remedy.required.value;
    assert!(find(&check_full_steel_structure(&doc), "en1993.1-8.3.12.ic.plate-ic").utilization <= 1.0);
}

#[test]
fn the_unit_fold_equals_the_full_evaluation_and_the_new_units_declare_their_reads() {
    let mut doc = base();
    doc.fatigue_stresses.push(stress_range("fs", StressRelief::StressRelieved, 1.0e7));
    doc.buckling_lengths.push(buckling("bl", 1.0, 1.0, FrameSway::NonSway));
    doc.fastened_joints.push(screws("fj"));
    doc.forming_gains.push(FormingGain { id: "fg".into(), member_id: "cf-1".into(), forming: FormingProcess::RollForming, ultimate_strength: 420.0e6, gross_area: 0.0006, bends: 4, used_yield: 0.0 });
    doc.material_ductility.push(MaterialDuctility { id: "md".into(), material_id: "mat-s355".into(), elongation: 0.20, ultimate_strain: 0.12 });
    assert_eq!(evaluate_units::<En1993Units>(&doc), check_full_steel_structure(&doc));
    for unit in <En1993Units as UnitSource>::units(&doc).iter() {
        assert_eq!(audit_reads(unit, &doc), Vec::<String>::new(), "undeclared reads of {:?}", unit.id);
    }
}
