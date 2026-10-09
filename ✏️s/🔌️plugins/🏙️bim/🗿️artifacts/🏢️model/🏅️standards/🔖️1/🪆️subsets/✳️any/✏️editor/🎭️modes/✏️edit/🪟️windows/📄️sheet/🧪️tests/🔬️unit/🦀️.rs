//! 🧪️ The sheet window: which sheet it shows, the records of the paper, the viewports and the marks, the placement of a view's records into a window, the picking of viewports and the accessible labels in both languages.

use super::*;
use crate::standards::v1::subsets::any::io::export::sheets::testkit::{house_with_sheets, inferred};
use crate::standards::v1::subsets::any::schema::inferences::sheet_layout::{ViewMap, PaperRect};

fn ids(records: &[DslValue]) -> Vec<String> {
    records.iter().filter_map(|record| record.get("id").and_then(DslValue::as_str).map(str::to_string)).collect()
}

#[test]
fn the_window_is_a_canvas_with_the_element_domain_and_the_utilities_of_the_sheet() {
    let definition = definition();
    assert_eq!((definition.id.as_str(), definition.body_key.as_str(), definition.surface_kind), (WINDOW_KIND_ID, BODY_KEY, SurfaceKind::Canvas2d));
    assert_eq!(definition.interactions.len(), 1);
}

#[test]
fn the_window_shows_its_sheet_while_it_exists_else_the_first_in_print_order() {
    let model = house_with_sheets();
    let shown = |sheet: &str| active_sheet(&model, &BimSheetWindowConfig { sheet: sheet.into(), ..BimSheetWindowConfig::default() });
    assert_eq!(shown("sh-sections").as_deref(), Some("sh-sections"));
    assert_eq!(shown("sh-gone").as_deref(), Some("sh-plans"));
    assert_eq!(shown("").as_deref(), Some("sh-plans"));
    assert_eq!(active_sheet(&crate::ModelSnapshot::default(), &BimSheetWindowConfig::default()), None);
}

#[test]
fn the_records_hold_the_paper_every_viewport_drawing_the_marks_and_the_meta_record_first() {
    let model = house_with_sheets();
    let inference = inferred(&model);
    let layout = inference.sheet_layouts.get("sh-plans");
    let records = records(layout, &inference, &[], "select", &BimLabels::NATIVE_EN);
    assert_eq!(records[0].get("id").and_then(DslValue::as_str), Some("meta:utility"));
    let named = ids(&records);
    assert!(named.contains(&"paper".to_string()));
    for viewport in ["vp-ground", "vp-first"] {
        assert!(named.iter().any(|id| id.starts_with(&format!("{viewport}:"))), "{viewport}");
    }
    for expected in ["ink:frame:0", "ink:title-block:0"] {
        assert!(named.iter().any(|id| id.starts_with(&expected[..expected.len() - 1])), "{expected}");
    }
    assert!(!named.iter().any(|id| id.starts_with("selected:")));
}

#[test]
fn a_selected_viewport_is_outlined_in_the_accent_colour_and_a_finding_in_red() {
    let model = house_with_sheets();
    let inference = inferred(&model);
    let layout = inference.sheet_layouts.get("sh-plans");
    let selected = records(layout, &inference, &["vp-first".to_string()], "select", &BimLabels::NATIVE_EN);
    assert!(ids(&selected).contains(&"selected:vp-first".to_string()));
    assert!(!ids(&selected).contains(&"selected:vp-ground".to_string()));
    let mut crowded = house_with_sheets();
    crowded.viewports.get_mut("vp-first").unwrap().position = crate::Point2 { x: 40.0, y: 40.0 };
    let inference = inferred(&crowded);
    let crowded = records(inference.sheet_layouts.get("sh-plans"), &inference, &[], "select", &BimLabels::NATIVE_EN);
    assert!(ids(&crowded).iter().any(|id| id.starts_with("finding:")), "two windows on top of each other raise a finding");
}

#[test]
fn a_missing_sheet_paints_the_empty_text_in_both_languages() {
    let inference = crate::ModelInference::default();
    for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
        let found = records(None, &inference, &[], "select", labels);
        let text = found.iter().find(|record| record.get("id").and_then(DslValue::as_str) == Some("empty")).expect("the empty text");
        assert_eq!(text.get("text").and_then(|row| row.get("content")).and_then(DslValue::as_str), Some(labels.empty_sheet.as_str()));
    }
    assert_ne!(BimLabels::NATIVE_EN.empty_sheet.as_str(), BimLabels::NATIVE_DE.empty_sheet.as_str());
}

#[test]
fn the_records_of_a_view_are_renamed_below_the_viewport_and_scaled_into_its_window() {
    let placed = PlacedViewport {
        viewport: "vp-1".into(),
        view: "v-1".into(),
        label: "Plan".into(),
        kind: crate::ViewKind::Plan,
        scale: 100,
        window: PaperRect { x: 30.0, y: 40.0, width: 90.0, height: 70.0 },
        map: ViewMap { min_x: -0.5, max_y: 6.5, mm: 10.0 },
        empty: false,
        cropped: false,
    };
    let record = path_record("wall-1", "node", path(&[[0.0, 0.0], [8.0, -6.0]], false), Paint { fill: None, stroke: Some((INK, 0.025)), dash: None });
    let moved = placed_records(vec![record], &placed);
    assert_eq!(moved[0].get("id").and_then(DslValue::as_str), Some("vp-1:wall-1"));
    let matrix: Vec<f64> = moved[0].get("transform").and_then(DslValue::as_array).expect("a transform").iter().filter_map(DslValue::as_f64).collect();
    assert_eq!(matrix, [10.0, 0.0, 0.0, 10.0, 30.0 + 5.0, 40.0 + 65.0]);
    let (x, y) = (matrix[0] * 8.0 + matrix[4], matrix[3] * -6.0 + matrix[5]);
    let (paper_x, paper_y) = placed.map.point(&placed.window, 8.0, 6.0);
    assert!((x - paper_x).abs() < 1e-9 && (y - paper_y).abs() < 1e-9, "({x}, {y}) against ({paper_x}, {paper_y})");
}

#[test]
fn a_transform_the_record_already_had_is_composed_not_replaced() {
    let placed = PlacedViewport { viewport: "vp-1".into(), view: "v-1".into(), label: String::new(), kind: crate::ViewKind::Plan, scale: 50, window: PaperRect { x: 10.0, y: 10.0, width: 50.0, height: 50.0 }, map: ViewMap { min_x: 0.0, max_y: 0.0, mm: 20.0 }, empty: false, cropped: false };
    let rotated = text_record("note", (1.0, 2.0), "x", 0.2, INK);
    let moved = placed_records(vec![rotated], &placed);
    let matrix: Vec<f64> = moved[0].get("transform").and_then(DslValue::as_array).expect("a transform").iter().filter_map(DslValue::as_f64).collect();
    assert_eq!(matrix, [20.0, 0.0, 0.0, 20.0, 20.0 * 1.0 + 10.0, 20.0 * 2.0 + 10.0]);
}

#[test]
fn the_topmost_window_under_the_pointer_is_picked_and_a_miss_picks_nothing() {
    let model = house_with_sheets();
    let inference = inferred(&model);
    let layout = inference.sheet_layouts.get("sh-plans").expect("a layout");
    let ground = layout.viewports.iter().find(|placed| placed.viewport == "vp-ground").expect("the ground plan");
    let inside = [ground.window.x + ground.window.width / 2.0, ground.window.y + ground.window.height / 2.0];
    assert_eq!(pick(layout, inside, 0.0).map(|placed| placed.viewport.as_str()), Some("vp-ground"));
    assert!(pick(layout, [1.0, 1.0], 0.0).is_none());
    assert_eq!(pick(layout, [ground.window.x - 1.0, inside[1]], 2.0).map(|placed| placed.viewport.as_str()), Some("vp-ground"));
    assert_eq!(handle_of(&ground.window), [ground.window.right(), ground.window.bottom()]);
}

#[test]
fn the_headings_come_from_the_labels_of_the_language() {
    let english = title_labels(&BimLabels::NATIVE_EN);
    let german = title_labels(&BimLabels::NATIVE_DE);
    assert_ne!(english.drawn_by, german.drawn_by);
    assert_ne!(english.revision_description, german.revision_description);
    assert!(english.project != english.title && english.number != english.date);
}
