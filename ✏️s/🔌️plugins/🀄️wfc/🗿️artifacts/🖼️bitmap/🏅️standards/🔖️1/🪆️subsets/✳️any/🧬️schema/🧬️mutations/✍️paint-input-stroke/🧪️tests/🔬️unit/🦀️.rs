//! 🧪️ `paint-input-stroke` laws beyond the committed quintets: the Bresenham cell walk, every outcome code the leaf
//! raises, its label in both languages, its text and binary spelling, and its editable payload.

use super::*;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::{BitmapSnapshot};

fn point(x: u32, y: u32) -> BitmapStrokePoint {
    BitmapStrokePoint { x, y }
}

fn base() -> BitmapSnapshot {
    crate::tests::fixtures::base()
}

fn codes(mutation: &BitmapMutation, snapshot: &BitmapSnapshot) -> Vec<(String, String)> {
    let outcome = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(mutation, snapshot);
    outcome.messages().iter().map(|message| (crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&message.level).trim_matches('"').to_string(), message.code.0.clone())).collect()
}

#[test]
fn consecutive_points_are_joined_by_an_inclusive_bresenham_line() {
    assert_eq!(stroke_cells(&[point(0, 0), point(3, 2)]), vec![point(0, 0), point(1, 1), point(2, 1), point(3, 2)]);
    assert_eq!(stroke_cells(&[point(3, 0), point(0, 0)]), vec![point(3, 0), point(2, 0), point(1, 0), point(0, 0)]);
    assert_eq!(stroke_cells(&[point(1, 1), point(1, 1), point(1, 2)]), vec![point(1, 1), point(1, 2)], "a repeated sample paints its cell once");
    assert!(stroke_cells(&[]).is_empty());
}

#[test]
fn a_stroke_paints_exactly_its_cells_with_the_brush_colour() {
    let mut snapshot = base();
    vcs::apply_mutation(&snapshot, &paint_input_stroke(vec![point(0, 0), point(3, 2)], 1)).map(|(applied_state, _)| { snapshot = applied_state; }).expect("the stroke applies");
    let painted = snapshot.input.indices().expect("the buffer decodes");
    let before = base().input.indices().expect("the base decodes");
    for y in 0..3u32 {
        for x in 0..4u32 {
            let index = (y * 4 + x) as usize;
            let on_stroke = [(0, 0), (1, 1), (2, 1), (3, 2)].contains(&(x, y));
            assert_eq!(painted[index], if on_stroke { 1 } else { before[index] }, "cell ({x}, {y})");
        }
    }
}

#[test]
fn an_unknown_palette_index_is_a_missing_target() {
    assert_eq!(codes(&paint_input_stroke(vec![point(0, 0)], 2), &base()), vec![("error".to_string(), "mutation.target-missing".to_string())]);
}

#[test]
fn a_stroke_with_no_cell_inside_the_sample_is_a_missing_target() {
    assert_eq!(codes(&paint_input_stroke(vec![point(9, 9), point(12, 9)], 0), &base()), vec![("error".to_string(), "mutation.target-missing".to_string())]);
    assert!(<BitmapMutation as protocol::Mutation<BitmapSnapshot>>::inverse(&paint_input_stroke(vec![point(9, 9)], 0), &base()).expect("valid retained mutation inverse fixture").is_empty(), "nothing to undo");
}

#[test]
fn a_stroke_that_leaves_the_sample_paints_the_rest_and_warns_partial() {
    let mutation = paint_input_stroke(vec![point(2, 0), point(5, 0)], 0);
    assert_eq!(codes(&mutation, &base()), vec![("warning".to_string(), "mutation.partial".to_string())]);
    let outcome = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation, &base());
    let region = &outcome.diff().input_regions[0];
    assert_eq!((region.x, region.y, region.width, region.height), (2, 0, 2, 1), "the region covers the inside cells only");
    assert_eq!(region.pixels.clone(), vec![0, 0]);
}

#[test]
fn a_stroke_over_cells_that_already_hold_its_colour_is_a_no_op() {
    assert_eq!(codes(&paint_input_stroke(vec![point(0, 0)], 0), &base()), vec![("warning".to_string(), "mutation.no-op".to_string())]);
}

#[test]
fn an_empty_or_oversized_stroke_breaks_the_invariant() {
    assert_eq!(codes(&paint_input_stroke(Vec::new(), 0), &base()), vec![("fatal".to_string(), "mutation.invariant".to_string())]);
    let oversized = vec![point(0, 0); BITMAP_STROKE_MAXIMUM_POINTS + 1];
    assert_eq!(codes(&paint_input_stroke(oversized, 0), &base()), vec![("fatal".to_string(), "mutation.invariant".to_string())]);
}

#[test]
fn the_inverse_restores_the_stroke_region() {
    let before = base();
    let mutation = paint_input_stroke(vec![point(0, 2), point(3, 0)], 1);
    let inverse = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::inverse(&mutation, &before).expect("valid retained mutation inverse fixture");
    let mut snapshot = before.clone();
    vcs::apply_mutation(&snapshot, &mutation).map(|(applied_state, _)| { snapshot = applied_state; }).expect("forward applies");
    assert_ne!(snapshot, before);
    for step in &inverse {
        vcs::apply_mutation(&snapshot, step).map(|(applied_state, _)| { snapshot = applied_state; }).expect("inverse step applies");
    }
    assert_eq!(snapshot, before);
}

#[test]
fn the_label_names_the_cells_and_the_colour_in_english_and_german() {
    let label = protocol::SemanticMutation::<BitmapSnapshot>::label(&paint_input_stroke(vec![point(0, 0), point(3, 2)], 1));
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Paint stroke of 4 cells in colour 1");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Strich mit 4 Zellen in Farbe 1 malen");
    let single = protocol::SemanticMutation::<BitmapSnapshot>::label(&paint_input_stroke(vec![point(2, 2)], 0));
    assert_eq!(single.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Strich mit 1 Zelle in Farbe 0 malen");
}

#[test]
fn the_stroke_round_trips_its_text_and_binary_spelling() {
    let mutation = paint_input_stroke(vec![point(0, 0), point(3, 2), point(1, 2)], 1);
    let line = protocol::OpText::print_op(&mutation);
    assert!(line.starts_with("paint-input-stroke"), "{line}");
    assert_eq!(<BitmapMutation as protocol::OpText>::parse_op(&line).expect("the line parses"), mutation);
    let bytes = protocol::OpBinary::encode_op(&mutation).expect("the stroke encodes");
    assert_eq!(<BitmapMutation as protocol::OpBinary>::decode_op(&bytes).expect("the stroke decodes"), mutation);
}

#[test]
fn the_stroke_is_editable_through_its_payload_value() {
    let mutation = paint_input_stroke(vec![point(0, 0), point(3, 2)], 1);
    let edited = protocol::Mutation::<BitmapSnapshot>::with_payload_value(&mutation, semio_framework_value::DslValue::from(&serde_json::json!({ "points": [{ "x": 0, "y": 0 }, { "x": 3, "y": 2 }], "color": 0 }))).expect("an edited payload decodes");
    assert_eq!(edited, paint_input_stroke(vec![point(0, 0), point(3, 2)], 0));
    assert!(protocol::Mutation::<BitmapSnapshot>::input_schema(&mutation).is_some_and(|schema| schema.contains("\"points\"") && schema.contains("\"color\"")));
}
