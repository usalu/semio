use super::*;

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::editor::bim::inference::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

fn through_the_house() -> BimSectionWindowConfig {
    BimSectionWindowConfig { start_x: -1.0, start_y: 3.0, end_x: 9.0, end_y: 3.0, ..BimSectionWindowConfig::default() }
}

#[semio_framework_async_macros::async_test]
async fn a_section_through_the_demo_cuts_the_two_side_walls() {
    let (_, inference) = demo();
    let cuts = cuts(&inference, &through_the_house());
    let elements: std::collections::BTreeSet<&str> = cuts.iter().map(|cut| cut.element.as_str()).collect();
    assert_eq!(elements, std::collections::BTreeSet::from(["w-east", "w-west"]));
    assert!(cuts.iter().all(|cut| cut.closed), "the cut of a closed solid is a closed chain");
}

#[semio_framework_async_macros::async_test]
async fn the_cut_poche_has_one_closed_chain_per_wall_layer_spanning_the_wall_height_and_thickness() {
    let (_, inference) = demo();
    let east: Vec<Cut> = cuts(&inference, &through_the_house()).into_iter().filter(|cut| cut.element == "w-east").collect();
    assert_eq!(east.len(), 2, "the two-layer wall cuts into one chain per layer");
    let points: Vec<[f64; 2]> = east.iter().flat_map(|cut| cut.points.iter().copied()).collect();
    let (u_min, u_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p[0]), hi.max(p[0])));
    let (v_min, v_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p[1]), hi.max(p[1])));
    assert!((u_max - u_min - 0.3).abs() < 1e-6, "the two layers add up to the 0.3 m wall, got {}", u_max - u_min);
    assert!((v_max - v_min - 3.0).abs() < 1e-6, "the storey top constraint makes the wall as high as its 3 m storey, got {}", v_max - v_min);
}

#[semio_framework_async_macros::async_test]
async fn a_section_beside_the_model_cuts_nothing() {
    let (_, inference) = demo();
    assert!(cuts(&inference, &BimSectionWindowConfig { start_y: 20.0, end_y: 20.0, ..through_the_house() }).is_empty());
    assert!(cuts(&inference, &BimSectionWindowConfig { end_x: 0.0, end_y: 0.0, start_x: 0.0, start_y: 0.0, ..through_the_house() }).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_records_carry_one_datum_per_storey_and_one_record_per_cut() {
    let (snapshot, inference) = demo();
    let records = records(&snapshot, &inference, &through_the_house(), "select");
    assert_eq!(records.len(), 1 + 2 * inference.storey_levels.len() + cuts(&inference, &through_the_house()).len());
    assert!(render(&snapshot, &inference, &through_the_house(), "select", 1).is_ok());
}
