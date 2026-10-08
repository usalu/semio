use super::*;

#[path = "../../../🧪️tests/🧰️construction-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(FIXTURE, COMPUTES), 12);
}

#[test]
fn the_table_and_the_fixture_cover_exactly_the_catalogue_kinds_of_brep_boolean() {
    support::assert_category("brep.boolean", COMPUTES, FIXTURE);
}

#[test]
fn equal_inputs_give_equal_shape_bytes() {
    assert_eq!(support::assert_deterministic(FIXTURE, COMPUTES), 8);
}

#[test]
fn a_job_cancelled_between_two_steps_answers_cancelled_and_releases_its_session() {
    assert!(support::assert_cancellable(FIXTURE, COMPUTES) >= 3);
}
