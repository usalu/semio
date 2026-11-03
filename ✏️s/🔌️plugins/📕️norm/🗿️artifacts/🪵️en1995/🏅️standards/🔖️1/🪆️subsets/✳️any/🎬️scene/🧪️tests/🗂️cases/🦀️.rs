use super::*;
use norm::design_case::CaseCatalogue;
use scene::projection_kit::test_support::{assert_case_laws, assert_handle_laws, assert_scene_laws};
use schema::checks::timber::{catalogue_demo_document, En1995Cases};

fn demo() -> En1995Snapshot {
    catalogue_demo_document()
}

fn scene_of(document: &En1995Snapshot, placements: &[Placement]) -> (StructuralScene, CheckReport) {
    let report = schema::checks::timber::evaluate_structure(document);
    let scene = project(&SceneInput { document, report: &report, placements, revision: scene::projection_kit::test_support::revision() });
    (scene, report)
}

fn extras(document: &En1995Snapshot) -> impl Fn(&model::CaseRow, scene::projection_kit::design_case::forms::Size) -> scene::projection_kit::design_case::forms::Extras + '_ {
    move |case, size| entities::case_extras(document.annex, case, size)
}

#[test]
fn every_demo_case_stands_in_the_scene_with_bound_grips_that_write_its_own_rows() {
    let document = demo();
    assert!(document.cases.len() > 60, "the catalogue demo carries every case");
    let (scene, report) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    let selection: Vec<scene::EntityRef> = document.cases.iter().map(|c| scene::EntityRef::new(scene::EntityKind::Solid, &c.id, format!("cases[id={}]", c.id))).collect();
    let grips = handles(&document, &[], &selection);
    assert_handle_laws(&grips);
    let ids: Vec<&str> = document.cases.iter().map(|c| c.id.as_str()).collect();
    assert_case_laws(&ids, &scene, &grips);
    for case in &document.cases {
        assert!(report.checks.iter().any(|c| c.subject.entity_id == case.id), "{} has no live result", case.id);
    }
}

#[test]
fn the_shared_forms_draw_every_case_and_the_entity_engine_none() {
    let document = demo();
    let (scene, _) = scene_of(&document, &[]);
    for case in &document.cases {
        let body = scene.entities.iter().find(|e| e.entity.id == case.id).unwrap_or_else(|| panic!("{} is not in the scene", case.id));
        assert!(matches!(body.geometry, scene::Geometry::Solid(_)), "{}: the body of a case is a solid of its form", case.id);
    }
    assert!(!scene.entities.iter().any(|e| e.entity.collection == "cases" && matches!(e.geometry, scene::Geometry::Group { .. })), "no case is a group of the entity engine");
}

#[test]
fn the_migrated_entries_draw_their_primitives() {
    use scene::{Geometry, SolidShape};
    let document = demo();
    let (scene, _) = scene_of(&document, &[]);
    let parts = |entry: &str| -> Vec<&scene::SceneEntity> {
        let case = document.cases.iter().find(|c| c.entry_id == entry).unwrap_or_else(|| panic!("{entry} is in the demo"));
        scene.entities.iter().filter(|e| e.entity.id == case.id || e.entity.id.starts_with(&format!("{}:", case.id))).collect()
    };
    let shape = |entry: &str| match &parts(entry)[0].geometry {
        Geometry::Solid(solid) => solid.shape.clone(),
        other => panic!("{entry}: {other:?}"),
    };
    assert!(matches!(shape("T-30"), SolidShape::Prism { .. }) && matches!(shape("T-31"), SolidShape::Prism { .. }), "tapered and curved members are profile prisms");
    assert!(parts("T-31").iter().any(|e| e.entity.id.ends_with(":apex")), "the apex zone is a part of the curved member");
    assert!(matches!(shape("T-97"), SolidShape::Laminate { layers, .. } if layers.len() == 3) && matches!(shape("T-107"), SolidShape::Laminate { layers, .. } if layers.len() == 5), "CLT panels are ply stacks");
    assert!(parts("T-107").iter().any(|e| e.entity.id.contains(":char@")), "the CLT char slabs follow t_req");
    assert!(["T-101", "T-102", "T-106"].iter().all(|entry| parts(entry).iter().any(|e| e.entity.id.contains(":char@60")) && parts(entry).iter().any(|e| e.entity.id.ends_with(":residual"))), "fire sections carry char bands and the residual section");
    assert!(["T-63", "T-66", "T-124"].iter().all(|entry| parts(entry)[0].appearance.deformation.as_ref().is_some_and(|o| !o.samples.is_empty() && o.frequency.is_some())), "the strip mode is the ghost of the body");
    assert!(parts("T-65").iter().any(|e| matches!(e.geometry, Geometry::Surface(_)) && e.appearance.deformation.is_some()), "a floor plate carries the heat-mapped mode surface");
    assert!(parts("T-90").len() > 3 && parts("T-93").len() > 3 && parts("T-94").len() > 3 && parts("T-121").len() > 3 && parts("T-122").len() > 3, "the assemblies draw their sub-forms");
    assert!(parts("T-94").iter().any(|e| matches!(e.geometry, Geometry::Fastener(_))), "the diaphragm draws its nail lines");
}

#[test]
fn every_catalogue_entry_declares_a_form_and_its_keys_are_declared_inputs() {
    for entry in En1995Cases::entries() {
        let geometry = entry.geometry.unwrap_or_else(|| panic!("{} declares no geometry", entry.id));
        for (role, _) in geometry.aux {
            assert!(scene::projection_kit::design_case::forms::is_aux_role(geometry.form, role), "{}: {role} is no aux role of {:?}", entry.id, geometry.form);
        }
        for key in geometry.keys() {
            assert!(entry.inputs.contains(&key), "{}: geometry key {key} is not a declared input", entry.id);
        }
    }
}

#[test]
fn the_dimensions_of_the_drawing_follow_their_driving_inputs() {
    let document = demo();
    scene::projection_kit::test_support::assert_case_dimension_laws_with::<En1995Cases>(&document.cases, &extras(&document));
}

#[test]
fn a_persisted_placement_moves_a_form_case() {
    let document = demo();
    let (first, second) = (&document.cases[0], document.cases.iter().find(|c| c.entry_id == "T-97").expect("a ply stack case"));
    let placed = [Placement { entity_id: first.id.clone(), origin: scene::Vec3::new(400.0, 500.0, 0.0), yaw: 0.0 }, Placement { entity_id: second.id.clone(), origin: scene::Vec3::new(-300.0, 200.0, 0.0), yaw: 0.0 }];
    let layout = Layout::of(&document, &placed);
    assert_eq!(layout.case(&first.id).map(|pose| pose.origin.clone()), Some(scene::Vec3::new(400.0, 500.0, 0.0)));
    assert_eq!(layout.case(&second.id).map(|pose| pose.origin.clone()), Some(scene::Vec3::new(-300.0, 200.0, 0.0)));
    let (scene, _) = scene_of(&document, &placed);
    let body = scene.entities.iter().find(|e| e.entity.id == second.id).expect("body");
    assert!(matches!(&body.geometry, scene::Geometry::Solid(solid) if matches!(&solid.shape, scene::SolidShape::Laminate { frame, .. } if (frame.origin.x + 300.0).abs() < 3.0 && (frame.origin.y - 200.0).abs() < 3.0)));
}

#[test]
fn the_default_places_of_the_cases_overlap_neither_each_other_nor_the_typed_entities() {
    let mut combined = schema::checks::timber::entity_demo_document();
    combined.cases = demo().cases;
    for document in [demo(), combined] {
        let layout = Layout::of(&document, &[]);
        let ext = extras(&document);
        scene::projection_kit::test_support::assert_case_layout_clear::<En1995Cases>(&document.cases, &|id| layout.case(id).cloned(), &ext, layout.entity_rects());
    }
}


const SHARED_FORM_ENTRIES: [&str; 22] = ["T-30", "T-31", "T-32", "T-33", "T-63", "T-64", "T-65", "T-66", "T-90", "T-93", "T-94", "T-95", "T-96", "T-97", "T-101", "T-102", "T-106", "T-107", "T-120", "T-121", "T-122", "T-124"];

#[test]
fn the_twenty_two_kernel_cases_are_coloured_by_their_worst_check_and_carry_grips_on_their_inputs() {
    let document = demo();
    let (scene, report) = scene_of(&document, &[]);
    let selection: Vec<scene::EntityRef> = document.cases.iter().map(|c| scene::EntityRef::new(scene::EntityKind::Solid, &c.id, format!("cases[id={}]", c.id))).collect();
    let grips = handles(&document, &[], &selection);
    for entry in SHARED_FORM_ENTRIES {
        let Some(case) = document.cases.iter().find(|c| c.entry_id == entry) else { continue };
        let body = scene.entities.iter().find(|e| e.entity.id == case.id).unwrap_or_else(|| panic!("{entry} is drawn"));
        let worst = report.checks.iter().filter(|c| c.subject.entity_id == case.id && c.status != norm::CheckStatus::NotApplicable).count();
        if worst > 0 {
            assert!(body.utilisation.is_some(), "{entry}: the body carries the utilisation of its worst check");
            assert!(matches!(body.appearance.channel, scene::ColourChannel::Utilisation), "{entry}: coloured by utilisation");
        }
        assert!(grips.iter().any(|h| h.id.starts_with(&format!("{}:", case.id))), "{entry}: at least one grip on an editable dimension");
    }
}

#[test]
fn a_two_span_taper_is_drawn_and_gripped_over_both_spans() {
    let mut document = demo();
    let case = document.cases.iter_mut().find(|c| c.entry_id == "T-30").expect("T-30 in the demo");
    let span = case.numbers.iter().find(|n| n.id == "l").expect("span l").value / 1e3;
    let reach = |case: &model::CaseRow| entities::elevation(&entities::geo(case)).expect("tapered profile").0.iter().fold(0.0_f64, |m, p| m.max(p.x));
    assert!((reach(case) - span).abs() < 1e-9, "one span of the single-span case");
    case.texts.iter_mut().find(|t| t.id == "load_case").expect("load case").value = "two_span_udl".into();
    assert!((reach(case) - 2.0 * span).abs() < 1e-9, "the prism runs over both spans");
    let id = case.id.clone();
    let selection = vec![scene::EntityRef::new(scene::EntityKind::Solid, &id, format!("cases[id={id}]"))];
    let grips = handles(&document, &[], &selection);
    assert_handle_laws(&grips);
    let l = grips.iter().find(|h| h.id == format!("{id}:l")).expect("the l grip");
    let writes = l.writes_at(2.0 * span);
    assert!(writes[0].path.ends_with("numbers[id=l].value") && (writes[0].value - span * 1e3).abs() < 1e-6, "the end of the second span writes one span: {writes:?}");
}
