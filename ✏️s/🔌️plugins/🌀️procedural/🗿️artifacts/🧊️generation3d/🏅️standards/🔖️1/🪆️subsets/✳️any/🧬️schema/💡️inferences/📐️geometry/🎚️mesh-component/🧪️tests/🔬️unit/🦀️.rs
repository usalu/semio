use super::*;
use super::super::mesh_support::harness as h;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn the_compute_table_and_the_fixture_cover_exactly_the_catalogue_kinds() {
    h::oracle::assert_category("mesh.component", COMPUTES, FIXTURE);
}

#[test]
fn every_fixture_case_holds_at_fuel_one_at_unbounded_fuel_and_on_repetition() {
    assert_eq!(h::run_fixture(FIXTURE, COMPUTES), 15);
}
