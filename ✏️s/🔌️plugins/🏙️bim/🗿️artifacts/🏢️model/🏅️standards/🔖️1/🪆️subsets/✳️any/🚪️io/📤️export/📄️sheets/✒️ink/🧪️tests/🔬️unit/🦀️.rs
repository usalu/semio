//! 🧪️ The ink of a sheet: one frame, one caption per window, a cell of text per title block field and a row of text per revision.

use super::*;
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::layout_of;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use crate::{ModelSnapshot, Point2, Sheet, SheetRevision, View, ViewKind, Viewport};
use super::super::testkit::labels;
use std::collections::BTreeMap;


fn model(revisions: usize) -> ModelSnapshot {
    let mut model = ModelSnapshot::default();
    model.views.insert("v-ground".into(), View::of_storey("bldg-1", "Ground plan", ViewKind::Plan, "st-ground"));
    model.sheets.insert("sh-1".into(), Sheet { project: "House".into(), drawn_by: "UG".into(), date: "2026-10-09".into(), ..Sheet::standard("A-101", "Ground floor") });
    model.viewports.insert("vp-1".into(), Viewport::standard("sh-1", "v-ground", Point2 { x: 30.0, y: 40.0 }));
    for index in 0..revisions {
        let mark = char::from(b'A' + index as u8).to_string();
        model.sheet_revisions.insert(format!("r-{index}"), SheetRevision { sheet: "sh-1".into(), number: mark, date: "2026-10-01".into(), description: format!("Change {index}"), author: "UG".into() });
    }
    model
}

fn marks(revisions: usize) -> SheetMarks {
    let model = model(revisions);
    let drawings: BTreeMap<&str, &ViewLinework> = BTreeMap::new();
    sheet_marks(&layout_of(&model, "sh-1", &drawings), &labels())
}

fn texts(marks: &[Mark]) -> Vec<(&'static str, String)> {
    marks.iter().filter_map(|mark| if let Mark::Text { class, text, .. } = mark { Some((*class, text.clone())) } else { None }).collect()
}

#[test]
fn the_frame_is_one_rectangle_of_the_frame_width() {
    let marks = marks(0);
    assert_eq!(marks.frame, [Mark::Rect { class: "frame", rect: PaperRect { x: 20.0, y: 10.0, width: 390.0, height: 277.0 }, width: FRAME_WIDTH }]);
}

#[test]
fn every_window_gets_a_caption_with_its_label_and_scale_under_its_bottom_edge() {
    let marks = marks(0);
    assert_eq!(marks.captions.len(), 1);
    let Mark::Text { x, y, text, bold, anchor, .. } = &marks.captions[0] else { panic!("a text") };
    assert_eq!((*x, text.as_str(), *bold, *anchor), (30.0, "Ground plan   1:100", true, Anchor::Start));
    assert!((*y - (40.0 + 10.0 + 5.0)).abs() < 1e-9, "{y}");
}

#[test]
fn every_cell_prints_its_heading_and_its_value() {
    let marks = marks(0);
    let found = texts(&marks.title_block);
    assert_eq!(found.iter().filter(|(class, _)| *class == "cell-label").map(|(_, text)| text.as_str()).collect::<Vec<_>>(), ["Project", "Sheet no.", "Title", "Scale", "Drawn by", "Checked by", "Date", "Revision"]);
    assert_eq!(found.iter().filter(|(class, _)| *class == "cell-value").map(|(_, text)| text.as_str()).collect::<Vec<_>>(), ["House", "A-101", "Ground floor", "1:100", "UG", "", "2026-10-09", ""]);
    assert_eq!(marks.title_block.iter().filter(|mark| matches!(mark, Mark::Rect { class: "title-cell", .. })).count(), 8);
}

#[test]
fn the_title_and_the_number_print_large_and_bold() {
    let marks = marks(0);
    let sizes: Vec<(String, f64, bool)> = marks.title_block.iter().filter_map(|mark| if let Mark::Text { class: "cell-value", text, size, bold, .. } = mark { Some((text.clone(), *size, *bold)) } else { None }).collect();
    assert!(sizes.contains(&("A-101".to_string(), 5.0, true)) && sizes.contains(&("Ground floor".to_string(), 5.0, true)) && sizes.contains(&("House".to_string(), 4.0, false)));
}

#[test]
fn a_sheet_without_revisions_prints_no_table_and_one_with_revisions_prints_a_row_each() {
    assert!(marks(0).revisions.is_empty());
    let marks = marks(2);
    let found = texts(&marks.revisions);
    assert_eq!(found.iter().filter(|(class, _)| *class == "revision-heading").map(|(_, text)| text.as_str()).collect::<Vec<_>>(), ["Mark", "Date", "Description", "By"]);
    assert_eq!(found.iter().filter(|(class, _)| *class == "revision-text").map(|(_, text)| text.as_str()).collect::<Vec<_>>(), ["A", "2026-10-01", "Change 0", "UG", "B", "2026-10-01", "Change 1", "UG"]);
    assert_eq!(marks.revisions.iter().filter(|mark| matches!(mark, Mark::Rect { class: "revision-row", .. })).count(), 2);
    assert_eq!(marks.revisions.iter().filter(|mark| matches!(mark, Mark::Line { class: "revision-column", .. })).count(), 3);
}

#[test]
fn the_labels_name_every_field_and_column() {
    let labels = labels();
    assert_eq!(TitleField::ALL.map(|field| labels.field(field).to_string()), ["Project", "Sheet no.", "Title", "Scale", "Drawn by", "Checked by", "Date", "Revision"]);
    assert_eq!(labels.columns(), ["Mark", "Date", "Description", "By"]);
}
