use super::*;

#[path = "../../../🧪️tests/🧰️construction-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(FIXTURE, COMPUTES), 29);
}

#[test]
fn the_table_and_the_fixture_cover_exactly_the_catalogue_kinds_of_brep_solid() {
    support::assert_category("brep.solid", COMPUTES, FIXTURE);
}

#[test]
fn equal_inputs_give_equal_shape_bytes() {
    assert_eq!(support::assert_deterministic(FIXTURE, COMPUTES), 17);
}

#[test]
fn a_job_cancelled_between_two_steps_answers_cancelled_and_releases_its_session() {
    assert!(support::assert_cancellable(FIXTURE, COMPUTES) >= 10);
}

#[test]
fn a_solid_job_yields_to_the_scheduler_at_fuel_one() {
    let (_, cases) = support::cases(FIXTURE);
    let case = cases.iter().find(|case| case["name"] == "loft two unit squares").expect("loft case");
    let kind = support::kind_of("brep.solid.loft");
    let mut job = loft(kind, support::inputs_of(kind, case));
    let mut working = 0;
    while let WidgetStep::Working { .. } = job.step(1) {
        working += 1;
        assert!(working < 1000, "the job finishes");
    }
    assert!(working >= 2, "import, kernel job and export are separate slices, found {working}");
}
