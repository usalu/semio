use super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::house;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::compute_plan_linework;

#[test]
fn every_storey_with_a_plan_gets_a_slot_highest_level_first() {
    let model = house();
    let plans = compute_plan_linework(&model);
    let sheet = layout(&model, &plans);
    assert_eq!(sheet.slots.len(), model.storeys.len());
    assert_eq!(sheet.slots.iter().map(|slot| slot.level).collect::<Vec<_>>(), [2, 1, 0, -1]);
    assert_eq!(sheet.slots.iter().map(|slot| slot.storey.as_str()).collect::<Vec<_>>(), ["st-roof", "st-first", "st-ground", "st-base"]);
}

#[test]
fn slots_are_stacked_without_overlap_inside_the_fitted_sheet() {
    let model = house();
    let sheet = layout(&model, &compute_plan_linework(&model));
    assert_eq!(sheet.slots[0].y, MARGIN);
    for pair in sheet.slots.windows(2) {
        assert!((pair[1].y - (pair[0].y + pair[0].height + GAP)).abs() < 1e-9);
    }
    let last = sheet.slots.last().unwrap();
    assert!((sheet.height - (last.y + last.height + MARGIN)).abs() < 1e-9);
    let widest = sheet.slots.iter().map(|slot| slot.width).fold(0.0, f64::max);
    assert!((sheet.width - (widest + 2.0 * MARGIN)).abs() < 1e-9);
}

#[test]
fn a_slot_is_the_plan_bounds_at_the_drawing_scale_plus_padding_and_the_title_band() {
    let model = house();
    let plans = compute_plan_linework(&model);
    let sheet = layout(&model, &plans);
    for slot in &sheet.slots {
        let bounds = plans[&slot.storey].bounds;
        assert!((slot.width - ((bounds.max_x - bounds.min_x) * MM_PER_METRE + 2.0 * PADDING)).abs() < 1e-9);
        assert!((slot.height - ((bounds.max_y - bounds.min_y) * MM_PER_METRE + 2.0 * PADDING + TITLE_BAND)).abs() < 1e-9);
        assert_eq!(slot.frame.point(bounds.min_x, bounds.max_y), (PADDING, TITLE_BAND + PADDING));
        let corner = slot.frame.point(bounds.max_x, bounds.min_y);
        assert!((corner.0 - (slot.width - PADDING)).abs() < 2e-3 && (corner.1 - (slot.height - PADDING)).abs() < 2e-3);
    }
}

#[test]
fn a_model_without_plans_has_an_empty_sheet() {
    let sheet = layout(&crate::ModelSnapshot::default(), &Default::default());
    assert!(sheet.slots.is_empty());
    assert_eq!((sheet.width, sheet.height), (2.0 * MARGIN, 2.0 * MARGIN));
}
