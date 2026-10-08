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
