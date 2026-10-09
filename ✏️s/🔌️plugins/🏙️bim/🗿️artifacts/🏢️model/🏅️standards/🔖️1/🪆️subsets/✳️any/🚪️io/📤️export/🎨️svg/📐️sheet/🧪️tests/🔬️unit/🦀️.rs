use super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::house;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::compute_view_linework;

#[test]
fn every_drawn_view_gets_a_slot_plans_highest_level_first_then_ceiling_plans_sections_and_elevations() {
    let model = house();
    let sheet = layout(&model, &compute_view_linework(&model));
    assert_eq!(sheet.slots.len(), model.views.values().filter(|view| is_drawn(view.kind)).count());
    let plans: Vec<_> = sheet.slots.iter().filter(|slot| slot.kind == ViewKind::Plan).map(|slot| slot.storey.as_deref().unwrap()).collect();
    assert_eq!(plans, ["st-roof", "st-first", "st-ground", "st-base"]);
    let kinds: Vec<_> = sheet.slots.iter().map(|slot| slot.kind).collect();
    assert_eq!(kinds, [vec![ViewKind::Plan; 4], vec![ViewKind::Section; 2], vec![ViewKind::Elevation; 4]].concat());
    assert!(sheet.slots.iter().all(|slot| slot.kind != ViewKind::Perspective), "a camera has no slot");
}

#[test]
fn slots_are_stacked_without_overlap_inside_the_fitted_sheet() {
    let model = house();
    let sheet = layout(&model, &compute_view_linework(&model));
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
fn a_slot_is_the_drawing_bounds_at_the_scale_of_its_view_plus_padding_and_the_title_band() {
    let model = house();
    let drawings = compute_view_linework(&model);
    let sheet = layout(&model, &drawings);
    for slot in &sheet.slots {
        let bounds = drawings[&slot.view].lines.bounds;
        let mm = mm_per_metre(model.views[&slot.view].scale);
        assert_eq!(slot.frame.mm, mm);
        assert!((slot.width - ((bounds.max_x - bounds.min_x) * mm + 2.0 * PADDING)).abs() < 1e-9);
        assert!((slot.height - ((bounds.max_y - bounds.min_y) * mm + 2.0 * PADDING + TITLE_BAND)).abs() < 1e-9);
        assert_eq!(slot.frame.point(bounds.min_x, bounds.max_y), (PADDING, TITLE_BAND + PADDING));
        let corner = slot.frame.point(bounds.max_x, bounds.min_y);
        assert!((corner.0 - (slot.width - PADDING)).abs() < 2e-3 && (corner.1 - (slot.height - PADDING)).abs() < 2e-3);
    }
}

#[test]
fn a_model_without_views_has_an_empty_sheet() {
    let sheet = layout(&crate::ModelSnapshot::default(), &Default::default());
    assert!(sheet.slots.is_empty());
    assert_eq!((sheet.width, sheet.height), (2.0 * MARGIN, 2.0 * MARGIN));
}
