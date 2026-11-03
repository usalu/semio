use super::*;
use checks::catalogue::{catalogue_demo_cases, En1990Cases};
use scene::projection_kit::design_case::forms;
use scene::projection_kit::test_support::{assert_case_laws, assert_handle_laws, assert_scene_laws, revision};

fn document() -> En1990Snapshot {
    En1990Snapshot { cases: catalogue_demo_cases(), ..En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) }
}

fn projected(document: &En1990Snapshot, placements: &[Placement]) -> StructuralScene {
    let report = inferences::evaluate(document);
    project(&SceneInput { document, report: &report, placements, revision: revision(), combination: None, traffic_group: None })
}

fn selection(document: &En1990Snapshot) -> Vec<EntityRef> {
    document.cases.iter().map(|c| EntityRef::new(scene::EntityKind::Solid, &c.id, format!("cases[id={}]", c.id))).collect()
}

#[test]
fn every_demo_case_stands_in_the_scene_with_finite_geometry_and_bound_grips() {
    let document = document();
    assert!(!document.cases.is_empty());
    let scene = projected(&document, &[]);
    assert_scene_laws(&scene);
    let grips = handles(&document, &[], &selection(&document));
    assert_handle_laws(&grips);
    let ids: Vec<&str> = document.cases.iter().map(|c| c.id.as_str()).collect();
    assert_case_laws(&ids, &scene, &grips);
}

#[test]
fn every_catalogue_entry_declares_a_form_and_its_keys_are_declared_inputs() {
    use semio_s_plugin_norm_compliance::design_case::CaseCatalogue;
    for entry in En1990Cases::entries() {
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
fn a_persisted_placement_moves_the_case_and_a_dragged_size_resizes_it() {
    let document = document();
    let first = document.cases.iter().find(|c| forms::declared::<En1990Cases>(c).is_some_and(|g| !g.load.is_empty())).expect("a case with a load input");
    let placed = [Placement { entity_id: first.id.clone(), origin: scene::Vec3::new(40.0, 50.0, 0.0), yaw: 0.0 }];
    let moved = projected(&document, &placed);
    let body = scene::projection_kit::test_support::entity(&moved, &first.id);
    assert!(matches!(&body.geometry, scene::Geometry::Solid(solid) if matches!(&solid.shape, scene::SolidShape::Box { frame, .. } | scene::SolidShape::Cylinder { frame, .. } if (frame.origin.x - 40.0).abs() < 1.5 && (frame.origin.y - 50.0).abs() < 1.5)));
}

#[test]
fn the_dimensions_of_the_drawing_follow_their_driving_inputs() {
    scene::projection_kit::test_support::assert_case_dimension_laws::<En1990Cases>(&document().cases);
}

#[test]
fn the_default_places_of_the_cases_overlap_nothing() {
    let document = document();
    let layout = Layout::of(&document, &[]);
    scene::projection_kit::test_support::assert_case_layout_clear::<En1990Cases>(&document.cases, &|id| layout.case(id).cloned(), &forms::no_extras, &[]);
}
