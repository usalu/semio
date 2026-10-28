//! 🧪️ Portable graph refusal and parent-content normalization witnesses.

use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;
use protocol::{DiffAlgebra, Mutation};
use semio_s_artifact_layout_layout_model::validation::validate_document;

fn decode(value: &serde_json::Value) -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn graph_validation_witnesses_guard_replacement_and_preserve_undo() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json")).unwrap();
    let base = decode(&corpus["base"]);
    for case in corpus["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut value = corpus["base"].clone();
        for patch in case["patch"].as_array().unwrap() {
            assert_eq!(patch["op"], "replace");
            *value.pointer_mut(patch["path"].as_str().unwrap()).unwrap() = patch["value"].clone();
        }
        let document = decode(&value);
        let valid = case["valid"].as_bool().unwrap();
        assert_eq!(validate_document(&document).is_ok(), valid, "{name}: domain validator");
        let mutation = LayoutMutation::ReplaceDocument(replace_document::ReplaceDocument { document: document.clone() });
        let saved = base.clone();
        let outcome = mutation.diff(&base);
        let after = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        assert_eq!(after, if valid { document } else { base.clone() }, "{name}: publication");
        if !valid { assert!(!outcome.messages().is_empty(), "{name}: refusal explains invariant"); }
        assert_eq!(base, saved, "{name}: immutable base");
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base, "{name}: inverse");
        let mut restored = after;
        for inverse in mutation.inverse(&base).unwrap().into_iter().rev() { restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
        assert_eq!(restored, base, "{name}: semantic inverse");
    }
}

#[test]
fn parent_creation_and_replacement_share_content_guards_and_normalization() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json")).unwrap();
    let base = decode(&corpus["base"]);
    for case in corpus["parents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let page: semio_s_artifact_layout_layout_model::ParentPage = semio_framework_pack_json::from_json_str(&case["page"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let valid = case["valid"].as_bool().unwrap();
        let create = LayoutMutation::CreateParentPage(create_parent_page::CreateParentPage { page: page.clone(), index: None });
        let outcome = create.diff(&base);
        let after = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        if valid {
            let expected: semio_s_artifact_layout_layout_model::ParentPage = semio_framework_pack_json::from_json_str(&case["normalizedPage"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            assert_eq!(after.parent_pages.last(), Some(&expected), "{name}: normalized create");
        } else {
            assert_eq!(after, base, "{name}: invalid create unchanged");
            assert!(!outcome.messages().is_empty(), "{name}: create refusal");
        }
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base, "{name}: create inverse");
        let mut restored = after;
        for inverse in create.inverse(&base).unwrap().into_iter().rev() { restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
        assert_eq!(restored, base, "{name}: semantic create inverse");
        let set = LayoutMutation::SetParentPageContent(set_parent_page_content::SetParentPageContent { id: base.parent_pages[0].id.clone(), layers: page.layers.clone(), frames: page.frames.clone() });
        let outcome = set.diff(&base);
        let after = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        if valid {
            assert_eq!(after.parent_pages[0].layers, protocol::apply_diff_cold(create.diff(&base).diff(), &base).unwrap().parent_pages.last().unwrap().layers, "{name}: shared normalization");
        } else {
            assert_eq!(after, base, "{name}: invalid replacement unchanged");
            assert!(!outcome.messages().is_empty(), "{name}: set refusal");
        }
        assert_eq!(protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap(), base, "{name}: parent inverse");
        let mut restored = after;
        for inverse in set.inverse(&base).unwrap().into_iter().rev() { restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
        assert_eq!(restored, base, "{name}: semantic content inverse");
    }
}

#[test]
fn nonfinite_native_numbers_are_refused_without_publication() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json")).unwrap();
    let base = decode(&corpus["base"]);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let edits: &[fn(&mut LayoutSnapshot, f64)] = &[
            |doc, value| doc.pages[0].width = value,
            |doc, value| doc.grid.baseline_offset = value,
            |doc, value| doc.paragraph_styles[0].tracking = value,
            |doc, value| doc.paragraph_styles[0].leading = value,
            |doc, value| doc.character_styles[0].font_size = Some(value),
            |doc, value| doc.character_styles[0].color = Some([value as f32; 4]),
            |doc, value| doc.pages[0].margins.left = value,
            |doc, value| doc.pages[0].columns.gutter = value,
            |doc, value| doc.parent_pages[0].height = value,
            |doc, value| doc.pages[0].overrides[0].bounds.as_mut().unwrap().x = value,
            |doc, value| if let semio_s_artifact_layout_layout_model::Frame::Text { bounds, .. } = &mut doc.pages[0].frames[0] { bounds.rotation = value; },
            |doc, value| if let semio_s_artifact_layout_layout_model::Frame::Text { inset, .. } = &mut doc.pages[0].frames[0] { inset.y = value; },
            |doc, value| if let semio_s_artifact_layout_layout_model::Frame::Rect { fill, .. } = &mut doc.parent_pages[0].frames[0] { *fill = Some([value as f32; 4]); },
        ];
        for edit in edits {
            let mut document = base.clone();
            edit(&mut document, value);
            assert!(validate_document(&document).is_err());
            let outcome = replace_document::diff_replace_document(&replace_document::ReplaceDocument { document }, &base);
            assert_eq!(outcome.diff(), &LayoutDiff::default());
        }
        let mut page = base.parent_pages[0].clone();
        page.id = "new-parent".into();
        match &mut page.frames[0] { semio_s_artifact_layout_layout_model::Frame::Rect { bounds, .. } => bounds.rotation = value, _ => unreachable!() }
        let outcome = create_parent_page::diff_create_parent_page(&create_parent_page::CreateParentPage { page, index: None }, &base);
        assert_eq!(outcome.diff(), &LayoutDiff::default());
    }
}
