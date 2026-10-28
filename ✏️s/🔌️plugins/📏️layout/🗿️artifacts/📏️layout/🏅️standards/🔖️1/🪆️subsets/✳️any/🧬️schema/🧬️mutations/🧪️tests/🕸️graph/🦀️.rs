//! 🧪️ Portable semantic graph preservation and exact inverse laws.

use super::*;
use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _, Mutation as _, OpBinary, OpText};
use semio_s_artifact_layout_layout_model::validation::validate_document;

#[test]
fn semantic_graph_mutations_publish_valid_graphs_and_restore_exact_state() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let decode = |value: &serde_json::Value| semio_framework_pack_json::from_json_str::<LayoutSnapshot>(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let base = decode(&case["before"]);
        let expected = decode(&case["after"]);
        let mutation: LayoutMutation = semio_framework_pack_json::from_json_str(&case["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(validate_document(&base).is_ok(), "{name}: valid base");
        let saved = base.clone();
        let outcome = mutation.diff(&base);
        let after = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        assert_eq!(after, expected, "{name}: portable output");
        assert!(validate_document(&after).is_ok(), "{name}: canonical published graph");
        assert_eq!(base, saved, "{name}: immutable input");
        if !case["accepted"].as_bool().unwrap() {
            assert!(!outcome.messages().is_empty(), "{name}: refusal explanation");
            assert!(mutation.inverse(&base).unwrap().is_empty(), "{name}: refused inverse has no side effects");
        }
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base, "{name}: central diff inverse");
        let mut restored = after;
        for inverse in mutation.inverse(&base).unwrap().into_iter().rev() {
            let inverse_outcome = inverse.diff(&restored);
            restored = protocol::apply_diff_cold(inverse_outcome.diff(), &restored).unwrap();
            assert!(validate_document(&restored).is_ok(), "{name}: valid intermediate semantic undo");
        }
        assert_eq!(restored, base, "{name}: exact semantic inverse including formatting and spread order");
        let encoded = semio_framework_pack_json::to_json_string(outcome.diff());
        let decoded: LayoutDiff = semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(decoded, *outcome.diff(), "{name}: sparse diff transport");
        assert_eq!(LayoutMutation::parse_op(&mutation.print_op()).unwrap(), mutation, "{name}: mutation text transport");
        assert_eq!(LayoutMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation, "{name}: mutation binary transport");
    }
}

#[test]
fn native_nonfinite_candidates_and_overflowing_character_splices_are_refused() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json")).unwrap();
    let base: LayoutSnapshot = semio_framework_pack_json::from_json_str(&corpus["cases"][0]["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut frame = base.pages[0].frames[1].clone();
        if let semio_s_artifact_layout_layout_model::Frame::Text { id, bounds, .. } = &mut frame { *id = "nonfinite-frame".into(); bounds.rotation = value; }
        let operation = LayoutMutation::CreateFrame(create_frame::CreateFrame { page_id: base.pages[0].id.clone(), frame, index: None, layer_id: None });
        assert_eq!(operation.diff(&base).diff(), &LayoutDiff::default());
        assert!(operation.inverse(&base).unwrap().is_empty());
        let mut page = base.pages[0].clone();
        page.id = "nonfinite-page".into();
        page.frames.clear();
        for layer in &mut page.layers { layer.object_ids.clear(); }
        page.margins.left = value;
        let operation = LayoutMutation::CreatePage(create_page::CreatePage { page, index: None });
        assert_eq!(operation.diff(&base).diff(), &LayoutDiff::default());
        assert!(operation.inverse(&base).unwrap().is_empty());
    }
    for (offset, delete) in [(usize::MAX, 1), (1, usize::MAX)] {
        let operation = LayoutMutation::EditStory(edit_story::EditStory { id: "story-1".into(), offset, delete, insert: "x".into() });
        assert_eq!(operation.diff(&base).diff(), &LayoutDiff::default());
        assert!(operation.inverse(&base).unwrap().is_empty());
    }
}

#[test]
fn portable_ieee_gutter_witnesses_refuse_native_nonfinite_values() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json")).unwrap();
    for case in corpus["wordCases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let base: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(validate_document(&base).is_ok(), "{name}: valid base");
        let mut mutation: LayoutMutation = semio_framework_pack_json::from_json_str(&case["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let bits = u64::from_str_radix(case["binary64"].as_str().unwrap(), 16).unwrap();
        match &mut mutation {
            LayoutMutation::UpdatePageColumns(payload) => { assert_eq!(case["field"], "gutter"); payload.gutter = f64::from_bits(bits); }
            _ => panic!("{name}: unsupported portable numeric witness"),
        }
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.diff(), &LayoutDiff::default(), "{name}: no publication");
        assert!(!outcome.messages().is_empty(), "{name}: refusal explains invariant");
        assert!(mutation.inverse(&base).unwrap().is_empty(), "{name}: no rejected inverse effects");
    }
}

fn word_at_mut<'a>(mut value: &'a mut semio_framework_value::DslValue, pointer: &str) -> &'a mut semio_framework_value::DslValue {
    for token in pointer.split('/').skip(1) {
        value = match value {
            semio_framework_value::DslValue::Array(rows) => &mut rows[token.parse::<usize>().unwrap()],
            semio_framework_value::DslValue::Object(rows) => &mut rows.iter_mut().find(|(key, _)| key == token).unwrap().1,
            _ => panic!("word witness pointer leaves its record"),
        };
    }
    value
}

fn native_word(width: u64, bits: &str) -> f64 {
    if width == 32 { f64::from(f32::from_bits(u32::from_str_radix(bits, 16).unwrap())) } else { f64::from_bits(u64::from_str_radix(bits, 16).unwrap()) }
}

fn read_word(value: &semio_framework_value::DslValue, width: u64) -> u64 {
    let semio_framework_value::DslValue::Number(number) = value else { panic!("numeric word witness") };
    if width == 32 { u64::from((number.as_f64() as f32).to_bits()) } else { number.as_f64().to_bits() }
}

#[test]
fn portable_semantic_words_publish_signed_zero_and_restore_exact_bits() {
    use semio_framework_value::{DslValue, Number, FromValue, ToValue};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🟰️words/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let width = case["width"].as_u64().unwrap();
        let mut before = DslValue::from(&case["before"]);
        *word_at_mut(&mut before, case["target"].as_str().unwrap()) = DslValue::Number(Number::Float(native_word(width, case["beforeBits"].as_str().unwrap())));
        let base = LayoutSnapshot::from_value(before).unwrap();
        let mut payload = DslValue::from(&case["mutation"]);
        *word_at_mut(&mut payload, case["field"].as_str().unwrap()) = DslValue::Number(Number::Float(native_word(width, case["afterBits"].as_str().unwrap())));
        let mutation = LayoutMutation::from_value(payload).unwrap();
        assert_eq!(LayoutMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation, "{name}: exact operation transport");
        let outcome = mutation.diff(&base);
        let transported = LayoutDiff::from_value(outcome.diff().to_value()).unwrap();
        let after = protocol::apply_diff_cold(&transported, &base).unwrap();
        let mut value = after.to_value();
        let expected = u64::from_str_radix(case["afterBits"].as_str().unwrap(), 16).unwrap();
        assert_eq!(read_word(word_at_mut(&mut value, case["target"].as_str().unwrap()), width), expected, "{name}: exact publication");
        assert_ne!(after, base, "{name}: distinct native word is a state change");
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base, "{name}: central inverse");
        let mut restored = after;
        for inverse in mutation.inverse(&base).unwrap().into_iter().rev() { restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
        assert_eq!(restored, base, "{name}: exact semantic inverse");
    }
}

#[test]
fn whole_replacement_observes_same_and_different_opaque_nan_words() {
    use semio_framework_value::{DslValue, Number};
    let graph: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json")).unwrap();
    let original: LayoutSnapshot = semio_framework_pack_json::from_json_str(&graph["cases"][0]["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap().iter().filter(|case| case["width"] == 64) {
        let mut base = original.clone();
        base.data_fields = Some(semio_framework_value_dictionary::QuestionDictionary { entries: vec![semio_framework_value_dictionary::QuestionDictionaryEntry { question_id: "word".into(), value: DslValue::Number(Number::Float(native_word(64, case["left"].as_str().unwrap()))) }] });
        let mut document = base.clone();
        document.data_fields.as_mut().unwrap().entries[0].value = DslValue::Number(Number::Float(native_word(64, case["right"].as_str().unwrap())));
        let mutation = LayoutMutation::ReplaceDocument(replace_document::ReplaceDocument { document: document.clone() });
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.diff() == &LayoutDiff::default(), case["equal"].as_bool().unwrap());
        let after = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        assert_eq!(after, document);
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base);
    }
}

#[test]
fn persisted_model_equality_observes_the_neutral_ieee_word_corpus() {
    let graph: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json")).unwrap();
    let base: LayoutSnapshot = semio_framework_pack_json::from_json_str(&graph["cases"][0]["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut left = base.clone(); let mut right = base.clone();
        if case["width"] == 64 {
            if let semio_s_artifact_layout_layout_model::Frame::Text { bounds, .. } = &mut left.pages[0].frames[0] { bounds.x = native_word(64, case["left"].as_str().unwrap()); }
            if let semio_s_artifact_layout_layout_model::Frame::Text { bounds, .. } = &mut right.pages[0].frames[0] { bounds.x = native_word(64, case["right"].as_str().unwrap()); }
        } else {
            left.character_styles[0].color = Some([native_word(32, case["left"].as_str().unwrap()) as f32, 0.0, 0.0, 1.0]);
            right.character_styles[0].color = Some([native_word(32, case["right"].as_str().unwrap()) as f32, 0.0, 0.0, 1.0]);
        }
        assert_eq!(left == right, case["equal"].as_bool().unwrap(), "{}", case["name"]);
    }
}

#[test]
fn portable_ieee_candidates_refuse_nonfinite_scalar_and_color_words() {
    use semio_framework_value::{DslValue, Number, FromValue};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🟰️words/🔣️.json")).unwrap();
    for case in corpus["refusals"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let base: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut payload = DslValue::from(&case["mutation"]);
        *word_at_mut(&mut payload, case["field"].as_str().unwrap()) = DslValue::Number(Number::Float(native_word(case["width"].as_u64().unwrap(), case["bits"].as_str().unwrap())));
        let mutation = LayoutMutation::from_value(payload).unwrap();
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.diff(), &LayoutDiff::default(), "{name}: no publication");
        assert!(!outcome.messages().is_empty(), "{name}: explained refusal");
        assert!(mutation.inverse(&base).unwrap().is_empty(), "{name}: no refused inverse");
    }
}

#[test]
fn portable_page_drawing_events_and_inverses_preserve_owner_and_graph(){
 let _declaration=semio_s_artifact_layout_layout_host::declaration().unwrap();
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap().iter().filter(|case|case["mutation"].get("SetPageBackgroundDrawing").is_some()){
  let read=|v:&serde_json::Value|semio_framework_pack_json::from_json_str::<LayoutSnapshot>(&v.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let base=read(&case["before"]);let expected=read(&case["after"]);let command:LayoutMutation=semio_framework_pack_json::from_json_str(&case["mutation"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let outcome=command.diff(&base);let after=protocol::apply_diff_cold(outcome.diff(),&base).unwrap();assert_eq!(after,expected);assert!(validate_document(&after).is_ok());assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base),&after).unwrap(),base);let mut undo=after;for inverse in command.inverse(&base).unwrap().into_iter().rev(){undo=protocol::apply_diff_cold(inverse.diff(&undo).diff(),&undo).unwrap();}assert_eq!(undo,base);let wire=semio_framework_pack_json::to_json_string(outcome.diff());let decoded:LayoutDiff=semio_framework_pack_json::from_json_str(&wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(decoded,*outcome.diff());
 }
}

#[test]
fn page_drawing_invalid_and_unchanged_commands_have_no_semantic_inverse(){
 let _declaration=semio_s_artifact_layout_layout_host::declaration().unwrap();let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json")).unwrap();let fixture=corpus["cases"].as_array().unwrap().iter().find(|c|c["name"]=="same page Drawing is a no-op").unwrap();let base:LayoutSnapshot=semio_framework_pack_json::from_json_str(&fixture["before"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let command:LayoutMutation=semio_framework_pack_json::from_json_str(&fixture["mutation"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(command.diff(&base).diff(),&LayoutDiff::default());assert!(command.inverse(&base).unwrap().is_empty());
 for zero_viewport in [false,true]{let mut invalid=command.clone();let LayoutMutation::SetPageBackgroundDrawing(payload)=&mut invalid else{panic!("page Drawing command")};let child=payload.drawing.as_mut().unwrap();if zero_viewport{semio_s_artifact_layout_layout_model::set_drawing_canvas_size(&mut child.content,0.0,0.0).unwrap();}else{child.handle.target.dialect.subset="other".into();}assert_eq!(invalid.diff(&base).diff(),&LayoutDiff::default());assert!(invalid.inverse(&base).unwrap().is_empty());}
}
