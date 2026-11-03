use super::*;
use checks::{catalogue_demo_cases, En1992Cases};
use norm::design_case::DesignCase;
use scene::projection_kit::design_case::forms;
use scene::projection_kit::test_support::{assert_case_laws, assert_handle_laws, assert_scene_laws, entity, revision};

fn document() -> En1992Snapshot {
    En1992Snapshot { cases: catalogue_demo_cases(), ..En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De) }
}

fn built_with(document: &En1992Snapshot, placements: &[Placement]) -> Built {
    let mut report = CheckReport::default();
    for case in &document.cases {
        report.extend(checks::evaluate_design_case(&DesignCase::from(case), document.annex));
    }
    build(&SceneInput { document, report: &report, placements, revision: revision() })
}

#[test]
fn every_demo_case_stands_in_the_scene_with_finite_geometry_and_bound_grips() {
    let document = document();
    assert!(!document.cases.is_empty());
    let built = built_with(&document, &[]);
    assert_scene_laws(&built.scene);
    assert_handle_laws(&built.handles);
    let ids: Vec<&str> = document.cases.iter().map(|c| c.id.as_str()).collect();
    assert_case_laws(&ids, &built.scene, &built.handles);
}

#[test]
fn every_catalogue_entry_declares_a_form_and_its_keys_are_declared_inputs() {
    use norm::design_case::CaseCatalogue;
    for entry in En1992Cases::entries() {
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
    let first = document.cases.iter().find(|c| forms::declared::<En1992Cases>(c).is_some()).expect("a case");
    let placed = [Placement { entity_id: first.id.clone(), origin: scene::Vec3::new(400.0, 500.0, 0.0), yaw: 0.0 }];
    let built = built_with(&document, &placed);
    let body = entity(&built.scene, &first.id);
    assert!(matches!(&body.geometry, scene::Geometry::Solid(solid) if matches!(&solid.shape, scene::SolidShape::Box { frame, .. } | scene::SolidShape::Cylinder { frame, .. } if (frame.origin.x - 400.0).abs() < 1.5 && (frame.origin.y - 500.0).abs() < 1.5)));
}

#[test]
fn the_dimensions_of_the_drawing_follow_their_driving_inputs() {
    scene::projection_kit::test_support::assert_case_dimension_laws::<En1992Cases>(&document().cases);
}

#[test]
fn the_default_places_of_the_cases_overlap_nothing() {
    let document = document();
    let layout = Layout::of(&document, &[]);
    scene::projection_kit::test_support::assert_case_layout_clear::<En1992Cases>(&document.cases, &|id| layout.case(id).cloned(), &forms::no_extras, &[]);
}
