//! 🧪️ The sheet rows: papers and dates read as text, every field reads and the writable ones become the mutation that sets exactly that field, a new sheet, viewport or revision applies cleanly, and the inferred rows read off the layout.

use super::*;
use crate::standards::v1::subsets::any::io::export::sheets::testkit::{house_with_sheets, inferred};
use protocol::Mutation;

fn applied(base: &ModelSnapshot, mutation: &ModelMutation) -> ModelSnapshot {
    let (diff, messages) = mutation.diff(base).into_parts();
    assert!(!messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)), "{messages:?}");
    protocol::apply_diff(&diff, base).expect("the diff applies")
}

#[test]
fn a_paper_reads_as_an_iso_size_or_as_two_sides() {
    assert_eq!(parse_paper("A3"), Some(Paper::iso(IsoSize::A3)));
    assert_eq!(parse_paper(" a0 "), Some(Paper::iso(IsoSize::A0)));
    assert_eq!(parse_paper("600 × 400"), Some(Paper::Custom { width: 600.0, height: 400.0 }));
    assert_eq!(parse_paper("600x400"), Some(Paper::Custom { width: 600.0, height: 400.0 }));
    assert_eq!(parse_paper("A5"), None);
    assert_eq!(parse_paper("wide × tall"), None);
}

#[test]
fn every_row_of_the_three_kinds_reads_every_field_of_the_house_sheets() {
    let model = house_with_sheets();
    for row in [&SHEET, &VIEWPORT, &REVISION] {
        for id in (row.ids)(&model) {
            assert!((row.name)(&model, &id).is_some(), "{} {id}", row.kind);
            for field in row.fields {
                assert!((field.read)(&model, &id).is_some(), "{} {id} cannot read {}", row.kind, field.key);
            }
        }
    }
    assert_eq!((SHEET.ids)(&model), ["sh-plans", "sh-sections", "sh-elevations"].map(String::from));
}

#[test]
fn the_sheet_fields_write_exactly_their_own_field() {
    let model = house_with_sheets();
    let write = |key: &str, value: &str| -> Option<ModelMutation> { SHEET.fields.iter().find(|field| field.key == key).and_then(|field| field.write).and_then(|write| write(&model, "sh-plans", value)) };
    let Some(ModelMutation::SetSheet(set)) = write("paper", "A1") else { panic!("a set-sheet") };
    assert_eq!((set.paper, set.name, set.number), (Some(Paper::iso(IsoSize::A1)), None, None));
    let Some(ModelMutation::SetSheet(set)) = write("orientation", "Portrait") else { panic!("a set-sheet") };
    assert_eq!(set.orientation, Some(Orientation::Portrait));
    let Some(ModelMutation::SetSheet(set)) = write("drawn_by", " MK ") else { panic!("a set-sheet") };
    assert_eq!(set.drawn_by.as_deref(), Some("MK"));
    assert!(write("paper", "A9").is_none() && write("orientation", "tilted").is_none());
    let applied = applied(&model, &write("paper", "600 × 400").expect("a custom paper"));
    assert_eq!(applied.sheets["sh-plans"].paper, Paper::Custom { width: 600.0, height: 400.0 });
}

#[test]
fn the_viewport_fields_move_scale_crop_and_label_a_viewport() {
    let model = house_with_sheets();
    let write = |key: &str, value: &str| -> Option<ModelMutation> { VIEWPORT.fields.iter().find(|field| field.key == key).and_then(|field| field.write).and_then(|write| write(&model, "vp-ground", value)) };
    let moved = applied(&model, &write("position", "50, 60").expect("a position"));
    assert_eq!(moved.viewports["vp-ground"].position, Point2 { x: 50.0, y: 60.0 });
    let scaled = applied(&model, &write("scale", "50").expect("a scale"));
    assert_eq!(scaled.viewports["vp-ground"].scale, 50);
    let cropped = applied(&model, &write("crop", "0, 0 → 6, 4").expect("a crop"));
    assert!(cropped.viewports["vp-ground"].crop.is_some());
    assert!(applied(&cropped, &VIEWPORT.fields.iter().find(|field| field.key == "crop").and_then(|field| field.write).and_then(|write| write(&cropped, "vp-ground", "")).expect("an empty crop clears")).viewports["vp-ground"].crop.is_none());
    let labelled = applied(&model, &write("label", "Entrance floor").expect("a label"));
    assert_eq!(labelled.viewports["vp-ground"].label.as_deref(), Some("Entrance floor"));
    assert!(write("scale", "many").is_none() && write("position", "50").is_none());
}

#[test]
fn a_revision_row_is_named_by_its_mark_and_description_and_edited_by_field() {
    let model = house_with_sheets();
    assert_eq!((REVISION.name)(&model, "rev-a").as_deref(), Some("A Issued for permit"));
    let write = REVISION.fields.iter().find(|field| field.key == "description").and_then(|field| field.write).expect("a writable description");
    let edited = applied(&model, &write(&model, "rev-a", "Issued for tender").expect("a mutation"));
    assert_eq!(edited.sheet_revisions["rev-a"].description, "Issued for tender");
    assert!(REVISION.fields.iter().find(|field| field.key == "sheet").is_some_and(|field| field.write.is_none()));
}

#[test]
fn a_new_sheet_takes_the_next_number_and_the_project_in_its_title_block() {
    let mut model = house_with_sheets();
    model.project.name = "House on the hill".into();
    model.project.author = "UG".into();
    let created = create_sheet(&model, "sh-new", "", "Details").expect("a create-sheet");
    let ModelMutation::CreateSheet(payload) = &created else { panic!("a create-sheet") };
    assert_eq!((payload.sheet.number.as_str(), payload.sheet.name.as_str(), payload.sheet.project.as_str(), payload.sheet.drawn_by.as_str()), ("A-102", "Details", "House on the hill", "UG"));
    assert_eq!(next_number(&ModelSnapshot::default()), "A-101");
    assert!(applied(&model, &created).sheets.contains_key("sh-new"));
}

#[test]
fn a_new_viewport_shows_the_next_view_that_is_not_on_the_sheet_at_its_scale() {
    let mut model = house_with_sheets();
    model.views.get_mut("v-plan-st-base").unwrap().scale = 200;
    let created = create_viewport(&model, "vp-new", "sh-plans", "").expect("a create-viewport");
    let ModelMutation::CreateViewport(payload) = &created else { panic!("a create-viewport") };
    assert!(["vp-ground", "vp-first"].iter().all(|placed| model.viewports[*placed].view != payload.viewport.view));
    assert_eq!(payload.viewport.scale, scale_of(&model, &payload.viewport.view));
    assert_eq!((payload.viewport.position.x, payload.viewport.position.y), (50.0, 50.0));
    assert!(applied(&model, &created).viewports.contains_key("vp-new"));
    assert_eq!(create_viewport(&model, "vp-new", "sh-gone", "").err(), Some("bim.create.sheet-missing"));
    assert_eq!(scale_of(&model, "v-plan-st-base"), 200);
    model.views.get_mut("v-plan-st-base").unwrap().scale = 5000;
    assert_eq!(scale_of(&model, "v-plan-st-base"), 100);
}

#[test]
fn a_new_revision_takes_the_next_free_letter() {
    let model = house_with_sheets();
    assert_eq!(next_mark(&model, "sh-plans"), "C");
    assert_eq!(next_mark(&model, "sh-sections"), "A");
    let created = create_revision(&model, "rev-new", "sh-plans", "Stair moved").expect("a create-sheet-revision");
    let ModelMutation::CreateSheetRevision(payload) = &created else { panic!("a create-sheet-revision") };
    assert_eq!((payload.sheet_revision.number.as_str(), payload.sheet_revision.description.as_str()), ("C", "Stair moved"));
    assert!(applied(&model, &created).sheet_revisions.contains_key("rev-new"));
}

#[test]
fn the_inferred_rows_read_the_size_the_scales_and_the_window() {
    let model = house_with_sheets();
    let inference = inferred(&model);
    let read = |row: &EntityKind, key: &str, id: &str| row.inferred.iter().find(|inferred| inferred.key == key).and_then(|inferred| (inferred.read)(&model, &inference, id));
    assert_eq!(read(&SHEET, "size", "sh-plans").as_deref(), Some("420 × 297"));
    assert_eq!(read(&SHEET, "scales", "sh-plans").as_deref(), Some("1:100"));
    assert_eq!(read(&SHEET, "viewports", "sh-plans").as_deref(), Some("2"));
    assert!(read(&VIEWPORT, "window", "vp-ground").is_some_and(|text| text.contains(" × ")));
    assert_eq!(read(&SHEET, "size", "sh-gone"), None);
}

#[test]
fn the_choices_list_the_orientations_the_sheets_and_the_views_that_draw_on_paper() {
    let model = house_with_sheets();
    let labels = &BimLabels::NATIVE_EN;
    assert_eq!(orientation_choices(&model, labels).into_iter().map(|(value, _)| value).collect::<Vec<_>>(), ["Landscape", "Portrait"]);
    assert_eq!(sheet_choices(&model, labels).into_iter().map(|(id, _)| id).collect::<Vec<_>>(), ["sh-plans", "sh-sections", "sh-elevations"]);
    let views: Vec<String> = view_choices(&model, labels).into_iter().map(|(id, _)| id).collect();
    assert!(views.contains(&"v-plan-st-ground".to_string()) && !views.contains(&"v-3d".to_string()));
}
