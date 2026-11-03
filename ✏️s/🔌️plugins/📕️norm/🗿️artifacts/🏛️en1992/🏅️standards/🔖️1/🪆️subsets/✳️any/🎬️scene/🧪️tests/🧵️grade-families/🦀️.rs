use super::*;
use checks::catalogue_demo_cases;
use model::En1992Snapshot;
use norm::design_case::DesignCase;
use scene::projection_kit::test_support::{assert_diagram_laws, assert_scene_laws, revision};
use scene::{Tone, Validate};

fn report_of(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    report.extend(checks::evaluate_material_cases(document));
    for case in &document.cases {
        report.extend(checks::evaluate_design_case(&DesignCase::from(case), document.annex));
    }
    report
}

fn built_of(document: &En1992Snapshot) -> (Built, CheckReport) {
    let report = report_of(document);
    let built = build(&SceneInput { document, report: &report, placements: &[], revision: revision() });
    (built, report)
}

fn diagram<'a>(built: &'a Built, id: &str) -> &'a Diagram2d {
    built.diagrams.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("no diagram {id}"))
}

fn marker<'a>(diagram: &'a Diagram2d, id: &str) -> &'a scene::Marker {
    diagram.markers.iter().find(|m| m.id == id).unwrap_or_else(|| panic!("no marker {id}"))
}

fn check<'a>(report: &'a CheckReport, id: &str) -> &'a norm::CheckResult {
    report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("no check {id}"))
}

#[test]
fn the_concrete_diagram_is_table_3_1_of_the_catalogue_and_its_grip_drags_f_ck() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, report) = built_of(&document);
    assert_diagram_laws(&built.diagrams);
    let grade = &document.concrete_grades[0];
    let g = diagram(&built, &format!("concrete-properties:{}", grade.id));
    let fck = grade.f_ck * 1.0e-6;
    let fcm = marker(g, "fcm");
    assert!((fcm.at.x - fck).abs() < 1e-9 && (fcm.at.y - (fck + 8.0)).abs() < 1e-9, "f_cm = f_ck + 8 of Table 3.1");
    assert_eq!(fcm.bound_check.as_deref(), Some(format!("en1992.m-02.fcm.{}", grade.id).as_str()));
    assert!((check(&report, &format!("en1992.m-02.fcm.{}", grade.id)).computed.value * 1.0e-6 - fcm.at.y).abs() < 1e-6);
    let grip = marker(g, "fck-grip").binding.as_ref().expect("grip");
    assert_eq!(grip.path, format!("concreteGrades[id={}].fCk", grade.id));
    assert!((grip.value_at(fck) - grade.f_ck).abs() <= 1.0e5);
    let table = |name: &str| g.table_twin.columns.iter().find(|c| c.header.en == name).expect("column").values.clone();
    assert_eq!(table("f_ck").len(), table("E_cm").len());
    assert!(table("f_ctm").windows(2).all(|w| w[1] > w[0]), "f_ctm grows with the class");
}

#[test]
fn the_steel_law_ends_at_k_f_yd_and_the_ductility_point_follows_the_grade_and_table_c1() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let id = document.reinforcement_grades[0].id.clone();
    let (built, report) = built_of(&document);
    let grade = document.reinforcement(&id).expect("grade");
    let law = diagram(&built, &format!("steel-law:{id}"));
    let ultimate = marker(law, "ultimate");
    let (eps_yd, eps_ud) = material_laws::steel_strains(&document, grade).expect("strains");
    assert!((ultimate.at.x - 0.9 * grade.eps_uk * 1.0e3).abs() < 1e-9 && (ultimate.at.x - eps_ud).abs() < 1e-9);
    let f_yd = material_laws::steel_stress(&document, grade, eps_yd, 1.0).expect("f_yd");
    assert!((ultimate.at.y - grade.k * f_yd).abs() < 1e-6 * f_yd, "inclined branch ends at k f_yd");
    let plane = diagram(&built, &format!("ductility-class:{id}"));
    let point = marker(plane, "grade");
    assert!(point.bound_check.as_deref().is_some_and(|c| c.starts_with("en1992.annexC.ductility.")));
    let (k_min, eps_min) = grade.ductility.table_c1_minima();
    let met = |d: &Diagram2d| d.table_twin.columns.iter().find(|c| c.header.en == "Class met (1 = yes)").expect("column").values.clone();
    let own = match grade.ductility {
        model::DuctilityClass::A => 0,
        model::DuctilityClass::B => 1,
        model::DuctilityClass::C => 2,
    };
    assert!(grade.k >= k_min && grade.eps_uk >= eps_min && met(plane)[own] == 1.0, "the own class is met");
    assert_eq!(point.binding.as_ref().expect("grip").path, format!("reinforcementGrades[id={id}].k"));
    document.reinforcement_grades[0].k = k_min - 0.05;
    let (failing, report_after) = built_of(&document);
    let after = marker(diagram(&failing, &format!("ductility-class:{id}")), "grade");
    assert_eq!(after.tone, Tone::Error, "k below Table C.1 is an error point");
    assert!(report.checks.iter().all(|c| !c.id.starts_with("en1992.annexC.") || c.utilization <= 1.0 + 1e-9));
    assert!(report_after.checks.iter().any(|c| c.id.starts_with("en1992.annexC.") && c.utilization > 1.0));
}

#[test]
fn the_member_carries_the_grade_labels_toned_by_the_grade_checks() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, _) = built_of(&document);
    assert_scene_laws(&built.scene);
    let member = &document.members[0];
    for tag in ["grade-concrete-label", "grade-ductility-label"] {
        assert!(built.scene.annotations.iter().any(|a| a.id() == format!("{}:{tag}", member.id)), "{tag}");
    }
}

#[test]
fn the_bond_diagrams_pass_through_the_report_values_and_the_marker_drags_the_diameter() {
    let document = En1992Snapshot::compliant_office_frame();
    let (built, report) = built_of(&document);
    let member = document.members.iter().find(|m| m.longitudinal.iter().any(|l| l.diameter > 0.0)).expect("member with bars");
    let layer = member.longitudinal.iter().find(|l| l.diameter > 0.0).expect("layer");
    let strength = diagram(&built, &format!("bond-strength:{}", member.id));
    let m = marker(strength, &format!("layer-{}", layer.id));
    let bound = check(&report, &format!("en1992.b-01.fbd.{}-{}", member.id, layer.id));
    assert!((m.at.y * 1.0e6 - bound.computed.value).abs() < 1e-6 * bound.computed.value.abs().max(1.0), "f_bd of the report");
    assert_eq!(m.bound_check.as_deref(), Some(bound.id.as_str()));
    let grip = m.binding.as_ref().expect("grip");
    assert!((grip.value_at(m.at.x) - layer.diameter).abs() < 1e-9);
    let anchorage = diagram(&built, &format!("anchorage-length:{}", member.id));
    let l_b = check(&report, &format!("en1992.b-02.lb_rqd.{}-{}", member.id, layer.id));
    assert!((marker(anchorage, &format!("layer-{}", layer.id)).at.y * 1.0e-3 - l_b.computed.value).abs() < 1e-9);
    assert_eq!(built.scene.validate(), Ok(()));
}

#[test]
fn the_anchorage_zone_has_the_required_length_of_the_anchorage_check_and_follows_the_diameter() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let member = document.members.iter().find(|m| m.longitudinal.iter().any(|l| l.diameter > 0.0 && l.anchorage_length > 0.0)).expect("member").clone();
    let layer = member.longitudinal.iter().find(|l| l.diameter > 0.0 && l.anchorage_length > 0.0).expect("layer").clone();
    let (built, report) = built_of(&document);
    let zone = scene::projection_kit::test_support::entity(&built.scene, &format!("{}:anchorage-{}-a", member.id, layer.id));
    let required = check(&report, &format!("en1992.8.4.anchorage.{}.{}", member.id, layer.id)).limit.value;
    let scene::Geometry::Member(g) = &zone.geometry else { panic!("member geometry") };
    let (a, b) = (&g.axis.points[0], &g.axis.points[1]);
    let length = a.distance(b);
    assert!((length - required.min((member.span - 2.0 * member.cover) / 2.0)).abs() < 1e-9, "{length} vs {required}");
    assert!(zone.utilisation.is_some());
    let bigger = document.members.iter_mut().find(|m| m.id == member.id).and_then(|m| m.longitudinal.iter_mut().find(|l| l.id == layer.id)).expect("layer");
    bigger.diameter *= 1.5;
    let (again, report_again) = built_of(&document);
    let longer = check(&report_again, &format!("en1992.8.4.anchorage.{}.{}", member.id, layer.id)).limit.value;
    assert!(longer > required, "a thicker bar needs a longer anchorage");
    let scene::Geometry::Member(g2) = &scene::projection_kit::test_support::entity(&again.scene, &format!("{}:anchorage-{}-a", member.id, layer.id)).geometry else { panic!("member geometry") };
    assert!(g2.axis.points[0].distance(&g2.axis.points[1]) > length - 1e-12);
}

fn tied() -> En1992Snapshot {
    let mut document = En1992Snapshot::compliant_office_frame();
    document.cases = catalogue_demo_cases().into_iter().filter(|c| c.entry_id == "DT-14").collect();
    document
}

#[test]
fn the_tying_case_draws_peripheral_internal_and_column_ties_and_the_diagram_is_the_check() {
    let document = tied();
    let case = document.cases.first().expect("a DT-14 case").clone();
    let (built, report) = built_of(&document);
    assert_scene_laws(&built.scene);
    let count = |tag: &str| built.scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("{}:tie-{tag}", case.id))).count();
    assert_eq!((count("peripheral"), count("internal"), count("column")), (4, 3, 4));
    let g = diagram(&built, &format!("tying-forces:{}", case.id));
    assert_diagram_laws(std::slice::from_ref(g));
    let util = check(&report, &format!("en1992.dt-14.util.{}", case.id));
    let design = marker(g, "design");
    assert_eq!(design.bound_check.as_deref(), Some(util.id.as_str()));
    assert!((design.utilisation.expect("utilisation") - util.utilization).abs() < 1e-9);
    let li = case.numbers.iter().find(|n| n.id == "li").expect("li").value;
    let end = ties::tie(document.annex, &case, "peripheral", Some(2.0 * li)).expect("peripheral").0 * 1.0e-3;
    let last = g.table_twin.columns[1].values.last().copied().expect("sweep");
    assert!((end - last).abs() < 1e-6 * end.abs().max(1.0), "the sweep ends at twice the span");
    assert!(g.series.iter().any(|s| matches!(s, scene::Series::Limit { id, .. } if id == "capacity")) || case.numbers.iter().all(|n| n.id != "as_provided"));
    let grip = design.binding.as_ref().expect("l_i grip");
    assert_eq!(grip.path, format!("cases[id={}].numbers[id=li].value", case.id));
}

#[test]
fn the_tie_of_the_case_is_full_and_the_others_are_ghosted() {
    let mut document = tied();
    let case = document.cases[0].id.clone();
    if let Some(t) = document.cases[0].texts.iter_mut().find(|t| t.id == "tie") {
        t.value = "internal".into();
    }
    let (built, _) = built_of(&document);
    let opacity = |tag: &str| scene::projection_kit::test_support::entity(&built.scene, &format!("{case}:{tag}")).appearance.opacity;
    assert!(opacity("tie-internal-0") > opacity("tie-peripheral-n") && opacity("tie-internal-0") > opacity("tie-column-x0"));
}

#[test]
fn the_prestress_limit_diagram_is_a16_and_the_marker_drags_the_force() {
    let document = En1992Snapshot::compliant_prestressed_beam();
    let (built, report) = built_of(&document);
    let member = document.members.iter().find(|m| m.prestress.is_some()).expect("prestressed");
    let g = diagram(&built, &format!("prestress-limit:{}", member.id));
    assert_diagram_laws(std::slice::from_ref(g));
    let util = check(&report, &format!("en1992.a-16.util.{}", member.id));
    let design = marker(g, "design");
    assert!((design.utilisation.expect("utilisation") - util.utilization).abs() < 1e-9);
    let (limit, _) = prestress_limit::limits(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() }, member).expect("limits");
    assert!((design.at.y / limit - util.utilization).abs() < 1e-9, "sigma_pm0 / limit is the utilisation");
    let grip = design.binding.as_ref().expect("grip");
    let force = member.prestress.as_ref().expect("prestress").force;
    assert!((grip.value_at(design.at.x) - force).abs() < 1.0);
    assert!(built.scene.entities.iter().any(|e| e.entity.id == format!("{}:tendon", member.id) && e.utilisation.is_some()));
}

#[test]
fn the_integrity_diagram_marks_a_dangling_reference_and_the_member_carries_the_label() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let clean = built_of(&document).0;
    assert!(diagram(&clean, "integrity:document").markers.iter().all(|m| m.tone == Tone::Info));
    assert!(!clean.scene.annotations.iter().any(|a| a.id().ends_with(":integrity-label")));
    let id = document.members[0].id.clone();
    document.members[0].reinforcement_grade_id = "missing-grade".into();
    let broken = built_of(&document).0;
    let g = diagram(&broken, "integrity:document");
    assert_eq!(g.markers.iter().filter(|m| m.tone == Tone::Error).count(), 1);
    assert!(g.markers.iter().any(|m| m.id == format!("ref-{id}-1") && m.tone == Tone::Error));
    assert!(broken.scene.annotations.iter().any(|a| a.id() == format!("{id}:integrity-label")));
    assert_eq!(g.table_twin.columns[2].values[0], 0.0);
}

#[test]
fn every_check_of_a_member_or_grade_row_is_bound_by_a_marker_of_some_diagram() {
    for document in [En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced(), En1992Snapshot::compliant_prestressed_beam(), En1992Snapshot::failing_prestressed_beam()] {
        let (built, report) = built_of(&document);
        assert_diagram_laws(&built.diagrams);
        let bound: std::collections::BTreeSet<&str> = built.diagrams.iter().flat_map(|d| d.markers.iter().filter_map(|m| m.bound_check.as_deref())).collect();
        let owners: std::collections::BTreeSet<&str> = document.members.iter().map(|m| m.id.as_str()).chain(document.concrete_grades.iter().map(|g| g.id.as_str())).chain(document.reinforcement_grades.iter().map(|g| g.id.as_str())).chain(document.prestress_steels.iter().map(|g| g.id.as_str())).chain(["annex"]).collect();
        let unbound: Vec<&str> = report.checks.iter().filter(|c| owners.contains(c.subject.entity_id.as_str()) && c.utilization.is_finite() && c.status != norm::CheckStatus::NotApplicable && !bound.contains(c.id.as_str())).map(|c| c.id.as_str()).collect();
        assert!(unbound.is_empty(), "{}: checks without a picture: {unbound:?}", document.title);
    }
}

#[test]
fn the_ledger_is_one_marker_per_check_with_the_utilisation_of_the_report() {
    let document = En1992Snapshot::failing_under_reinforced();
    let (built, report) = built_of(&document);
    let member = &document.members[0];
    let g = diagram(&built, &format!("check-ledger:{}", member.id));
    assert_eq!(g.validate(), Ok(()));
    for m in &g.markers {
        let id = m.bound_check.as_deref().expect("bound");
        assert!((m.utilisation.expect("utilisation") - check(&report, id).utilization).abs() < 1e-12);
    }
    assert!(g.markers.iter().any(|m| m.tone == Tone::Error), "a failing member has an error marker");
}
