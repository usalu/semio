//! 🧪️ Mutation aggregate — the kind vocabulary stays honest against the enum, and every builder is
//! reachable through the aggregate.

use super::*;
use protocol::Mutation;
use crate::schema::snapshot::{BitmapColor, BitmapInput, BitmapSnapshot};
use protocol::SemanticMutation;

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        input: BitmapInput { width: 2, height: 1, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: ([0, 1]).to_vec() },
        ..BitmapSnapshot::default()
    }
}

fn every_variant() -> Vec<BitmapMutation> {
    vec![
        change_seed(1),
        resize_input(2, 2),
        set_input_pixels(0, 0, 1, 1, ([1]).to_vec()),
        add_palette_color(2, BitmapColor::opaque(1, 1, 1)),
        change_palette_color(0, BitmapColor::opaque(2, 2, 2)),
        remove_palette_color(1),
        resize_output(2, 2, true),
        change_model(3, 2, false, None),
        pin_pixel(0, 0, 0),
        unpin_pixel(0, 0),
        paint_input_stroke(vec![BitmapStrokePoint { x: 0, y: 0 }, BitmapStrokePoint { x: 1, y: 0 }], 1),
    ]
}

#[test]
fn the_kind_vocabulary_matches_the_enum_variant_order() {
    let kinds: Vec<&str> = every_variant().iter().map(|mutation| mutation.semantics().kind).collect();
    assert_eq!(kinds, KINDS, "KINDS must list every variant, in declaration order");
}

#[test]
fn every_kind_is_distinct() {
    let mut seen = KINDS.to_vec();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), KINDS.len());
}

#[test]
fn every_variant_carries_a_label_and_a_descriptor_that_agree_with_its_kind() {
    for mutation in every_variant() {
        let label = mutation.label();
        for terminology in semio_framework_ui_locale::Terminology::ALL {
            for locale in semio_framework_ui_locale::Locale::ALL {
                assert!(!label.resolve(terminology, locale).is_empty(), "{:?} label is empty for {terminology:?}/{locale:?}", mutation.semantics().kind);
            }
        }
        assert_eq!(mutation.descriptor().semantic_kind, mutation.semantics().kind);
    }
}

#[test]
fn every_variant_round_trips_its_single_line_text_op() {
    for mutation in every_variant() {
        let line = crate::io::text::mutations::print_op(&mutation);
        assert!(!line.is_empty());
        assert_eq!(crate::io::text::mutations::parse_op(&line).expect("op line parses"), mutation, "op line: {line}");
    }
}

#[test]
fn an_unknown_op_line_is_refused() {
    assert!(crate::io::text::mutations::parse_op("collapse-everything now").is_err());
}

#[test]
fn applying_a_refused_mutation_leaves_the_document_untouched() {
    let base = scene();
    let mut snapshot = base.clone();
    let outcome = <BitmapMutation as Mutation<BitmapSnapshot>>::diff(&unpin_pixel(9, 9), &base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.target-missing"));
    apply_bitmap_mutation(&mut snapshot, &unpin_pixel(9, 9)).expect("an empty refused diff still applies as a no-op");
    assert_eq!(snapshot, base);
}
