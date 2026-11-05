//! 🧪️ Mutation aggregate — the kind vocabulary stays honest against the enum, and every builder is
//! reachable through the aggregate.

use crate::standards::v1::subsets::any::schema::mutations::*;
use protocol::{ColdMutation, Mutation};
use crate::schema::snapshot::{BitmapPinnedPixel, BitmapColor, BitmapInput, BitmapSnapshot};
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
        pin_solution(vec![BitmapPinnedPixel { x: 0, y: 0, color: 1 }]),
        restore_pins(vec![BitmapPinnedPixel { x: 1, y: 0, color: 0 }]),
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
        for terminology in semio_ui_locale::Terminology::ALL {
            for locale in semio_ui_locale::Locale::ALL {
                assert!(!label.resolve(terminology, locale).is_empty(), "{:?} label is empty for {terminology:?}/{locale:?}", mutation.semantics().kind);
            }
        }
        assert_eq!(mutation.descriptor().semantic_kind, mutation.semantics().kind);
    }
}

#[test]
fn every_variant_round_trips_its_single_line_text_op() {
    for mutation in every_variant() {
        let line = crate::standards::v1::subsets::any::io::text::mutations::print_op(&mutation);
        assert!(!line.is_empty());
        assert_eq!(crate::standards::v1::subsets::any::io::text::mutations::parse_op(&line).expect("op line parses"), mutation, "op line: {line}");
    }
}

#[test]
fn an_unknown_op_line_is_refused() {
    assert!(crate::standards::v1::subsets::any::io::text::mutations::parse_op("collapse-everything now").is_err());
}

#[test]
fn applying_a_refused_mutation_leaves_the_document_untouched() {
    let base = scene();
    let mut snapshot = base.clone();
    let outcome = <BitmapMutation as ColdMutation<BitmapSnapshot>>::diff(&unpin_pixel(9, 9), &base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.target-missing"));
    vcs::apply_mutation(&snapshot, &unpin_pixel(9, 9)).map(|(applied_state, _)| { snapshot = applied_state; }).expect("an empty refused diff still applies as a no-op");
    assert_eq!(snapshot, base);
}

/// 📡️ The new batch columns are exact and preserve every pin through both carriers.
#[test]
fn pin_batch_wire_follows_the_portable_column_contract() {
    use crate::standards::v1::subsets::any::io::text::mutations::{BitmapOperationDsl, operation_from_dsl};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️pin-batch/🔣️.json")).expect("wire vectors");
    for kind in fixture["kinds"].as_array().unwrap() {
        for case in fixture["wireCases"].as_array().unwrap() {
            let column = |name: &str| case[name].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
            let (xs, ys, colors) = (column("xs"), column("ys"), column("colors"));
            let operation = if kind["variant"] == "PinSolution" { BitmapOperationDsl::PinSolution { xs, ys, colors } } else { BitmapOperationDsl::RestorePins { xs, ys, colors } };
            let decoded = operation_from_dsl(operation);
            if !case["valid"].as_bool().unwrap() { assert!(decoded.is_err(), "{}", case["id"]); continue; }
            let mutation = decoded.expect("exact columns");
            let actual = serde_json::Value::from(semio_framework_value::ToValue::to_value(&mutation));
            assert_eq!(actual[kind["variant"].as_str().unwrap()]["pins"], case["pins"]);
            let bytes = protocol::OpBinary::encode_op(&mutation).expect("batch binary");
            assert_eq!(u64::from(bytes[1]), kind["tag"].as_u64().unwrap());
            assert_eq!(<BitmapMutation as protocol::OpBinary>::decode_op(&bytes).unwrap(), mutation);
            let text = protocol::OpText::print_op(&mutation);
            assert_eq!(<BitmapMutation as protocol::OpText>::parse_op(&text).unwrap(), mutation);
        }
    }
}

/// 🧷️ Semantic assignment laws share their inputs and expected pins with the independent Python oracle.
#[test]
fn pin_batch_semantics_obey_the_portable_document_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️pin-batch/🔣️.json")).unwrap();
    for case in fixture["semanticCases"].as_array().unwrap() {
        let mut source = fixture["document"].clone();
        source["pinned"] = case["beforePins"].clone();
        let before: BitmapSnapshot = semio_framework_pack_json::from_json_str(&source.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation: BitmapMutation = semio_framework_pack_json::from_json_str(&case["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let outcome = <BitmapMutation as ColdMutation<BitmapSnapshot>>::diff(&mutation, &before);
        if let Some(code) = case["fault"].as_str() {
            assert!(outcome.messages().iter().any(|message| message.code.0 == code), "{}", case["id"]);
            assert_eq!(vcs::apply_mutation(&before, &mutation).unwrap().0, before);
            continue;
        }
        let after = vcs::apply_mutation(&before, &mutation).unwrap().0;
        let actual = serde_json::Value::from(semio_framework_value::ToValue::to_value(&after.pinned));
        assert_eq!(actual, case["afterPins"], "{}", case["id"]);
        let actual_messages: Vec<&str> = outcome.messages().iter().map(|message| message.code.0.as_str()).collect();
        let expected_messages: Vec<&str> = case["messages"].as_array().unwrap().iter().map(|message| message.as_str().unwrap()).collect();
        assert_eq!(actual_messages, expected_messages, "{}", case["id"]);
        let mut restored = after;
        for inverse in <BitmapMutation as ColdMutation<BitmapSnapshot>>::inverse(&mutation, &before).unwrap() { restored = vcs::apply_mutation(&restored, &inverse).unwrap().0; }
        assert_eq!(restored, before, "{}", case["id"]);
    }
}
