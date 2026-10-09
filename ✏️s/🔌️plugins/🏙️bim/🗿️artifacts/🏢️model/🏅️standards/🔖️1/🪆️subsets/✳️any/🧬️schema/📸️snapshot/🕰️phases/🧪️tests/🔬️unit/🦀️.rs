use super::*;

#[test]
fn every_phase_round_trips_through_its_key() {
    for phase in Phase::ALL {
        assert_eq!(Phase::parse(phase.key()), Some(phase));
        assert_eq!(Phase::parse(&format!(" {:?} ", phase)), Some(phase));
    }
    assert_eq!(Phase::parse("planned"), None);
    assert_eq!(Phase::parse(""), None);
}

#[test]
fn a_freshly_authored_element_is_new_construction() {
    assert_eq!(Phase::default(), Phase::New);
}

#[test]
fn the_keys_are_distinct_and_the_order_is_the_life_of_a_project() {
    let keys: Vec<&str> = Phase::ALL.iter().map(|phase| phase.key()).collect();
    assert_eq!(keys, ["existing", "new", "demolished", "temporary"]);
}
