use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, real, rows, string, tags, target};

#[test]
fn a_stair_aggregates_flights_that_carry_the_inferred_riser_and_tread_counts() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCSTAIR"), ["s-1", "s-2"]);
    let (stair, args) = rows(&document, "IFCSTAIR").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("s-1")).expect("s-1");
    assert_eq!(args[8].as_enum(), Some("STRAIGHT_RUN_STAIR"));
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(stair.id)).expect("the flights");
    let flights = aggregate[5].as_list().expect("flights");
    assert_eq!(flights.len(), 1);
    let flight = document.resolve(&flights[0]).and_then(|instance| instance.entity("IFCSTAIRFLIGHT")).expect("a flight");
    let risers = flight[8].as_real().expect("risers");
    assert_eq!(risers, (3.0f64 / 0.18).ceil(), "ceil(rise / max riser)");
    assert_eq!(flight[9].as_real(), Some(risers - 1.0));
    assert!((real(flight, 10) - 3.0 / risers).abs() < 1e-6);
}

#[test]
fn a_turning_stair_has_two_flights_and_the_quarter_turn_type() {
    let document = document(&house());
    let (stair, args) = rows(&document, "IFCSTAIR").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("s-2")).expect("s-2");
    assert_eq!(args[8].as_enum(), Some("QUARTER_TURN_STAIR"));
    let (_, aggregate) = rows(&document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(stair.id)).expect("the flights");
    assert_eq!(aggregate[5].as_list().map(<[_]>::len), Some(2));
    assert!(target(&document, args, 6).is_none(), "a decomposed stair carries no body of its own");
    for flight in aggregate[5].as_list().expect("flights") {
        let args = document.resolve(flight).and_then(|instance| instance.entity("IFCSTAIRFLIGHT")).expect("a flight");
        assert!(target(&document, args, 6).is_some(), "each flight carries its share of the brep");
    }
}

#[test]
fn a_railing_is_a_brep_with_a_length_and_height_quantity_set() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCRAILING"), ["rl-1"]);
    let (_, railing) = rows(&document, "IFCRAILING").into_iter().next().expect("a railing");
    assert_eq!(railing[8].as_enum(), Some("GUARDRAIL"));
    assert!(count(&document, "IFCFACETEDBREP") >= 3);
    let quantities: Vec<f64> = rows(&document, "IFCELEMENTQUANTITY")
        .iter()
        .filter(|(_, args)| string(args, 2).as_deref() == Some("Qto_RailingBaseQuantities"))
        .flat_map(|(_, args)| args[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCQUANTITYLENGTH")).map(|row| real(row, 3)).collect::<Vec<f64>>())
        .collect();
    assert_eq!(quantities, [4.0, 1.0]);
}

fn built() -> crate::ModelSnapshot {
    let mut model = house();
    model.stairs.get_mut("s-1").expect("stair").stringer = crate::StairStringer { kind: crate::StringerKind::Closed, width: 0.06, depth: 0.24 };
    let rail = model.railings.get_mut("rl-1").expect("railing");
    rail.baluster = Some(crate::Baluster { profile: crate::Profile::Rectangle { width: 0.02, depth: 0.02 }, spacing: 0.12 });
    rail.infill = crate::Infill::Panel { thickness: 0.02 };
    model
}

fn aggregated(document: &semio_s_artifact_stdio_ifc::part21::Part21Document, class: &str, tag: &str) -> Option<usize> {
    let (instance, _) = rows(document, class).into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(tag))?;
    rows(document, "IFCRELAGGREGATES").into_iter().find(|(_, relation)| relation[4].as_ref_id() == Some(instance.id)).and_then(|(_, relation)| relation[5].as_list().map(<[_]>::len))
}

#[test]
fn the_stringer_boards_of_a_stair_are_a_member_aggregated_next_to_its_flights() {
    let (plain, rich) = (document(&house()), document(&built()));
    assert!(tags(&rich, "IFCMEMBER").contains(&"s-1:stringer".to_string()));
    assert!(!tags(&plain, "IFCMEMBER").iter().any(|tag| tag.ends_with(":stringer")));
    assert_eq!(aggregated(&rich, "IFCSTAIR", "s-1"), Some(2), "the flight and the stringer member");
    assert_eq!(aggregated(&plain, "IFCSTAIR", "s-1"), Some(1), "a stair without a stringer aggregates its flight only");
}

#[test]
fn the_balusters_and_the_infill_of_a_railing_are_a_member_and_a_plate_aggregated_by_it() {
    let (plain, rich) = (document(&house()), document(&built()));
    assert!(tags(&rich, "IFCMEMBER").contains(&"rl-1:baluster".to_string()) && tags(&rich, "IFCPLATE").contains(&"rl-1:infill".to_string()));
    assert_eq!(aggregated(&rich, "IFCRAILING", "rl-1"), Some(2));
    assert_eq!(aggregated(&plain, "IFCRAILING", "rl-1"), None, "a plain railing aggregates nothing");
    assert_eq!(tags(&rich, "IFCRAILING"), ["rl-1"], "still one railing");
}
