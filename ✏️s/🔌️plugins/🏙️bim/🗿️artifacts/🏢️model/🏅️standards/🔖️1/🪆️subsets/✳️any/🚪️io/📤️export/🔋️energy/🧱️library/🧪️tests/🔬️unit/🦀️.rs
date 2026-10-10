use super::*;

#[test]
fn millimetres_are_short() {
    assert_eq!(millimetres(0.2), "200");
    assert_eq!(millimetres(0.0375), "37.5");
}

#[test]
fn a_schedule_name_ending_in_hours_is_a_weekday_window() {
    assert_eq!(Profile::of(Some("Office 08-18")), (Profile::Weekdays { from: 8, to: 18 }, true));
    assert_eq!(Profile::of(None), (Profile::Always, true));
    assert_eq!(Profile::of(Some("Always")), (Profile::Always, true));
    assert_eq!(Profile::of(Some("Residential")), (Profile::Always, false));
    assert_eq!(Profile::of(Some("Late 18-08")), (Profile::Always, false), "a window that wraps midnight is not understood");
    assert_eq!(Profile::of(Some("Odd 08-25")), (Profile::Always, false));
}

#[test]
fn schedules_are_interned() {
    let (mut book, mut next) = (ScheduleBook::default(), 0);
    let first = book.constant(20.0, &mut next);
    assert_eq!(book.constant(20.0, &mut next), first);
    let office = book.profile(Profile::Weekdays { from: 8, to: 18 }, &mut next);
    assert_eq!(book.profile(Profile::Weekdays { from: 8, to: 18 }, &mut next), office);
    assert_eq!(book.profile(Profile::Always, &mut next), book.constant(1.0, &mut next));
    assert_eq!((book.schedules.daily.len(), book.schedules.weekly.len()), (2, 1));
}

#[test]
fn a_material_at_a_thickness_and_a_stack_are_interned_and_a_layer_without_conductivity_has_no_id() {
    let mut model = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let id = model.materials.keys().next().cloned().expect("a material");
    let layer = |thickness| Layer { material: id.clone(), thickness, function: crate::LayerFunction::Structure };
    let (mut library, mut next) = (Library::default(), 0);
    let a = library.construction(&model, "A", &[layer(0.2)], &mut next).expect("a stack");
    assert_eq!(library.construction(&model, "B", &[layer(0.2)], &mut next), Some(a), "the same stack, the first name");
    assert_ne!(library.construction(&model, "C", &[layer(0.1)], &mut next), Some(a));
    assert_eq!(library.materials.len(), 2);
    model.materials.get_mut(&id).expect("the material").conductivity = 0.0;
    assert_eq!(Library::default().construction(&model, "D", &[layer(0.2)], &mut next), None);
    assert_eq!(library.curtain_host(&mut next), library.curtain_host(&mut next));
}
