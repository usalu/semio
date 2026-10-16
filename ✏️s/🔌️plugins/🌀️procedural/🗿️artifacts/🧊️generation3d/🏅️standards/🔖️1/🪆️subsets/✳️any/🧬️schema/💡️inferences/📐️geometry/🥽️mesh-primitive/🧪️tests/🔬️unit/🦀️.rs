use super::*;
use semio_s_artifact_procedural_3d_io_text::snapshot::catalogue::catalogue;
use semio_s_artifact_procedural_3d_geometry_mesh_harness as h;

fn admitted()->Vec<ComputeEntry> {COMPUTES.iter().chain(semio_s_artifact_procedural_3d_io_geometry::mesh_source::COMPUTES).copied().collect()}

const FIXTURE: &str = include_str!("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🥽️mesh-primitive/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    h::oracle::assert_category(catalogue(), "mesh.primitive", &admitted(), FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_at_unbounded_fuel_and_on_repetition() {
    assert_eq!(h::run_fixture(catalogue(), FIXTURE, &admitted()), 15);
}
