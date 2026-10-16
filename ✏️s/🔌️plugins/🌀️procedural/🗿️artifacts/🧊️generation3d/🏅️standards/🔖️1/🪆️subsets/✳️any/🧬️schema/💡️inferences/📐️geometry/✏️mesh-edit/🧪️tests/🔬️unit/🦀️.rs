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
    assert_eq!(h::run_fixture(catalogue(), FIXTURE, COMPUTES), 16);
}
