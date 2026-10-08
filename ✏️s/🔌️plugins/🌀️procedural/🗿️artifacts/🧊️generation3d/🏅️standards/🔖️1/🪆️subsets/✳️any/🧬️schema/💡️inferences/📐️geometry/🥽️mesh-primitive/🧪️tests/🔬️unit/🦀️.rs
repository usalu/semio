use super::*;
use super::super::mesh_support::harness as h;

fn admitted()->Vec<ComputeEntry> {COMPUTES.iter().chain(crate::standards::v1::subsets::any::io::geometry::mesh_source::COMPUTES).copied().collect()}

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    h::oracle::assert_category("mesh.primitive", &admitted(), FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_at_unbounded_fuel_and_on_repetition() {
    assert_eq!(h::run_fixture(FIXTURE, &admitted()), 15);
}
