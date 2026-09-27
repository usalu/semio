//! 🧪️ Laws of the range-text operation: the Rust twin replays the language-neutral `semio.ui.scene.text-splice.v1` fixture
//! exactly as the TS twin does (edit → splice, application + inverse, concurrent folds in hub order, host rebase, UTF-8 ↔ scalar).
use super::{rebase_text_edits, TextSplice, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/✂️text-splice/🔣️.json")).expect("text splice fixture")
}

fn splice(value: &Value) -> TextSplice {
    serde_json::from_value(value.clone()).expect("fixture splice")
}

#[test]
fn the_fixture_pins_the_context_constants() {
    let fixture = fixture();
    assert_eq!(fixture["schema"], "semio.ui.scene.text-splice.v1");
    assert_eq!(fixture["contextScalars"], TEXT_SPLICE_CONTEXT_SCALARS);
    assert_eq!(fixture["minTwoSidedScalars"], TEXT_SPLICE_MIN_TWO_SIDED_SCALARS);
}

#[test]
fn every_edit_yields_the_fixture_splice_and_round_trips() {
    for row in fixture()["edits"].as_array().unwrap() {
        let (previous, next) = (row["previous"].as_str().unwrap(), row["next"].as_str().unwrap());
        let derived = TextSplice::from_edit(previous, next, TEXT_SPLICE_CONTEXT_SCALARS);
        let expected = if row["splice"].is_null() { None } else { Some(splice(&row["splice"])) };
        assert_eq!(derived, expected, "edit {}", row["id"]);
        if let Some(derived) = derived {
            assert_eq!(derived.apply(previous, TEXT_SPLICE_CONTEXT_SCALARS).text, next, "edit {} round trip", row["id"]);
        }
    }
}

#[test]
fn every_application_lands_results_and_inverts_like_the_fixture() {
    for row in fixture()["applications"].as_array().unwrap() {
        let (text, id) = (row["text"].as_str().unwrap(), &row["id"]);
        let applied = splice(&row["splice"]).apply(text, TEXT_SPLICE_CONTEXT_SCALARS);
        assert_eq!(serde_json::to_value(applied.located).unwrap(), row["located"], "apply {id} located");
        assert_eq!(applied.text, row["result"].as_str().unwrap(), "apply {id} result");
        assert_eq!(applied.inverse, splice(&row["inverse"]), "apply {id} inverse");
        if !applied.located.clamped {
            assert_eq!(applied.inverse.apply(&applied.text, TEXT_SPLICE_CONTEXT_SCALARS).text, text, "apply {id} undo");
        }
    }
}

#[test]
fn every_concurrent_fold_in_hub_order_matches_the_fixture() {
    for row in fixture()["concurrent"].as_array().unwrap() {
        let mut text = row["base"].as_str().unwrap().to_string();
        let mut clamped = Vec::new();
        for (index, entry) in row["order"].as_array().unwrap().iter().enumerate() {
            let applied = splice(&entry["splice"]).apply(&text, TEXT_SPLICE_CONTEXT_SCALARS);
            if applied.located.clamped {
                clamped.push(index);
            } else {
                assert_eq!(applied.inverse.apply(&applied.text, TEXT_SPLICE_CONTEXT_SCALARS).text, text, "concurrent {} #{index} undo", row["id"]);
            }
            text = applied.text;
        }
        assert_eq!(text, row["expected"]["text"].as_str().unwrap(), "concurrent {}", row["id"]);
        assert_eq!(serde_json::to_value(&clamped).unwrap(), row["expected"]["clamped"], "concurrent {} clamped", row["id"]);
    }
}

#[test]
fn every_host_rebase_matches_the_fixture() {
    for row in fixture()["rebases"].as_array().unwrap() {
        let unapplied: Vec<TextSplice> = row["unapplied"].as_array().unwrap().iter().map(splice).collect();
        let selection = &row["selection"];
        let (text, anchor, caret) = rebase_text_edits(row["remote"].as_str().unwrap(), &unapplied, row["local"].as_str().unwrap(), selection["anchor"].as_u64().unwrap() as usize, selection["caret"].as_u64().unwrap() as usize);
        assert_eq!((text.as_str(), anchor as u64, caret as u64), (row["expected"]["text"].as_str().unwrap(), row["expected"]["anchor"].as_u64().unwrap(), row["expected"]["caret"].as_u64().unwrap()), "rebase {}", row["id"]);
    }
}

#[test]
fn utf8_offsets_and_scalar_indices_agree_with_the_fixture() {
    for row in fixture()["offsets"].as_array().unwrap() {
        let text = row["text"].as_str().unwrap();
        let bytes = row["utf8"].as_u64().unwrap() as usize;
        let scalar = text.char_indices().take_while(|(offset, _)| *offset < bytes).count();
        assert_eq!(scalar as u64, row["scalar"].as_u64().unwrap(), "offset {text:?}@{bytes}");
    }
}
