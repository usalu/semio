use super::*;
use norm::design_case::{CaseCatalogue, DesignCase};
use scene::projection_kit::design_case::forms;
use scene::projection_kit::test_support::{assert_case_laws, assert_handle_laws, assert_scene_laws, entity, revision};
use checks::case_catalogue::{catalogue_demo_cases, evaluate_design_case, En1994Cases};

fn document() -> En1994Snapshot {
    En1994Snapshot { cases: catalogue_demo_cases(), ..En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) }
}

fn projected(document: &En1994Snapshot, placements: &[Placement]) -> StructuralScene {
    let mut report = CheckReport::default();
    for case in &document.cases {
        report.extend(evaluate_design_case(&DesignCase::from(case), document.annex));
    }
    project(&SceneInput { document, report: &report, placements, revision: revision() })
}

#[test]
fn every_demo_case_stands_in_the_scene_with_finite_geometry_and_bound_grips() {
    let document = document();
    assert!(!document.cases.is_empty());
    let scene = projected(&document, &[]);
    assert_scene_laws(&scene);
    let selection: Vec<EntityRef> = document.cases.iter().map(|c| EntityRef::new(scene::EntityKind::Solid, &c.id, format!("cases[id={}]", c.id))).collect();
    let grips = handles(&document, &[], &selection);
    assert_handle_laws(&grips);
    let ids: Vec<&str> = document.cases.iter().map(|c| c.id.as_str()).collect();
    assert_case_laws(&ids, &scene, &grips);
}

#[test]
fn every_catalogue_entry_declares_a_form_and_its_keys_are_declared_inputs() {
    for entry in En1994Cases::entries() {
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
fn a_persisted_placement_moves_the_case() {
    let document = document();
    let first = document.cases.iter().find(|c| forms::declared::<En1994Cases>(c).is_some()).expect("a case");
    let placed = [Placement { entity_id: first.id.clone(), origin: scene::Vec3::new(400.0, 500.0, 0.0), yaw: 0.0 }];
    let scene = projected(&document, &placed);
    let body = entity(&scene, &first.id);
    assert!(matches!(&body.geometry, scene::Geometry::Solid(solid) if matches!(&solid.shape, scene::SolidShape::Box { frame, .. } | scene::SolidShape::Cylinder { frame, .. } if (frame.origin.x - 400.0).abs() < 1.5 && (frame.origin.y - 500.0).abs() < 1.5)));
}

#[test]
fn the_dimensions_of_the_drawing_follow_their_driving_inputs() {
    scene::projection_kit::test_support::assert_case_dimension_laws::<En1994Cases>(&document().cases);
}

#[test]
fn the_default_places_of_the_cases_overlap_nothing() {
    let document = document();
    let layout = Layout::of(&document, &[]);
    scene::projection_kit::test_support::assert_case_layout_clear::<En1994Cases>(&document.cases, &|id| layout.case(id).cloned(), &forms::no_extras, &[]);
}

#[test]
fn every_evaluated_case_has_a_result_diagram_bound_to_its_checks() {
    let document = document();
    let report = semio_s_artifact_norm_en1994_inferences::evaluate(&document);
    let list = diagrams::diagrams(&document, &report, None);
    let mut found = 0;
    for case in &document.cases {
        let id = format!("case-results:{}", case.id);
        let Some(d) = list.iter().find(|d| d.id == id) else { continue };
        found += 1;
        scene::Validate::validate(d).unwrap_or_else(|v| panic!("{id}: {v:?}"));
        assert!(d.table_twin.rows() > 0);
        assert!(d.markers.iter().all(|m| m.bound_check.as_ref().is_some_and(|c| report.checks.iter().any(|k| &k.id == c))));
        assert_eq!(diagrams::diagrams(&document, &report, Some(&case.id)).len(), 1);
    }
    assert!(found > 0);
}
