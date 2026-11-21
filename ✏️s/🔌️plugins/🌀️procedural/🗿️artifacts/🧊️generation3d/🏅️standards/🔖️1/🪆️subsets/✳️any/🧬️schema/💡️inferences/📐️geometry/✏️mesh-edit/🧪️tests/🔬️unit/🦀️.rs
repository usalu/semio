use super::*;
use semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue;
use semio_s_artifact_procedural_3d_geometry_mesh_harness as h;

const FIXTURE: &str = include_str!("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✏️mesh-edit/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    h::oracle::assert_category(catalogue(), "mesh.edit", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_at_unbounded_fuel_and_on_repetition() {
    assert_eq!(h::run_fixture(catalogue(), FIXTURE, COMPUTES), 44);
}

#[test]
fn cancelled_mesh_pair_jobs_publish_no_partial_output() {
    use semio_s_artifact_procedural_3d_geometry_compute::{testing::step_with_fuel, WidgetStep};

    let (_, cases) = h::oracle::cases(FIXTURE);
    for id in ["mesh.edit.combine", "mesh.edit.intersect", "mesh.edit.difference", "mesh.edit.union"] {
        let case = cases.iter().find(|case| case["kind"] == id).unwrap();
        let kind = catalogue().kind(id).unwrap();
        let compute = COMPUTES.iter().find(|entry| entry.id == id).unwrap();
        let mut job = (compute.start)(kind, h::inputs_of(kind, case));
        let mut reached_production = false;
        for _ in 0..1024 {
            let WidgetStep::Working { progress } = step_with_fuel(&mut *job, 1) else {
                panic!("{id} published before retained production could be cancelled");
            };
            if progress > 0.2 && progress < 0.6 {
                reached_production = true;
                break;
            }
        }
        assert!(reached_production, "{id}");
        job.cancel();
        let WidgetStep::Done(evaluation) = step_with_fuel(&mut *job, 1) else {
            panic!("{id} must settle after cancellation");
        };
        assert!(evaluation.outputs.is_empty(), "{id}");
        assert_eq!(evaluation.fault.unwrap().code, semio_s_artifact_procedural_3d_mesh_support::CANCELLED, "{id}");
    }
}

#[test]
fn advanced_mesh_difference_cancels_during_authored_projection() {
    use semio_s_artifact_procedural_3d_geometry_compute::{testing::step_with_fuel, WidgetStep};

    let (_, cases) = h::oracle::cases(FIXTURE);
    let kind = catalogue().kind("mesh.edit.difference").unwrap();
    for case in cases.iter().filter(|case| case.get("cancelAfter").is_some()) {
        let threshold = case["cancelAfter"].as_f64().unwrap() as f32;
        let mut job = difference(kind, h::inputs_of(kind, case));
        let mut reached = false;
        for _ in 0..65_536 {
            let WidgetStep::Working { progress } = step_with_fuel(&mut *job, 1) else {
                panic!("{} published before projection could be cancelled", case["name"]);
            };
            if progress > threshold {
                assert!(progress < 0.6, "{} arrived in import", case["name"]);
                reached = true;
                break;
            }
        }
        assert!(reached, "{}", case["name"]);
        job.cancel();
        let WidgetStep::Done(result) = step_with_fuel(&mut *job, 1) else { panic!("cancelled Difference did not settle") };
        assert!(result.outputs.is_empty());
        assert_eq!(result.fault.unwrap().code, semio_s_artifact_procedural_3d_mesh_support::CANCELLED);
    }
}
