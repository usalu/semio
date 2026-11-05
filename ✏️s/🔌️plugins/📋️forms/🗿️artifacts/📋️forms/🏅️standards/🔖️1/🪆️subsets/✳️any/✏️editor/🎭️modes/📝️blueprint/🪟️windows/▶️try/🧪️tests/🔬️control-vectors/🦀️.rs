use crate::editor::forms::support::FORMS_PLAY_WINDOW_TRY;
use semio_framework_pack_json::{Object, Value};
use crate::editor::forms::modes::blueprint::windows::try_wizard::{control, render_try_question};
use crate::editor::forms::modes::blueprint::windows::try_wizard::*;
use semio_framework_test_fixtures::fixture_text;
#[test]
fn semantic_question_controls_match_the_language_neutral_vectors() {
    let view = semio_plugin::ViewModel {
        window_id: Some("try-a".into()),
        window_instances: vec![semio_plugin::ViewWindowInstance { id: "try-a".into(), window_kind_id: FORMS_PLAY_WINDOW_TRY.into() }],
        ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🧫️fixtures/🔣️controls.json")).unwrap();
    for vector in vectors["cases"].as_array().unwrap() {
        let question = &vector["question"];
        let question = crate::editor::forms::questions::question_shell(question["id"].as_str().unwrap().into(), question["label"].as_str().unwrap().into(), question["kind"].as_str().unwrap().into());
        let node = render_try_question(&question, &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&view), &view).unwrap();
        let control = if question.kind == "note" { &node } else { node.children.get(0).unwrap() };
        let actual = serde_json::to_value(control).unwrap();
        assert_eq!(actual["component"]["type"], vector["component"], "{}", question.kind);
        if vector["interactive"].as_bool().unwrap() {
            assert_eq!(actual["accessibility"]["label"], vector["question"]["label"]);
            assert_eq!(actual["bindings"][0]["trigger"], "change");
            assert_eq!(actual["bindings"][0]["args"]["key"], vector["question"]["id"]);
            assert_eq!(actual["bindings"][0]["args"]["windowId"], "try-a");
        }
    }
}

/// 🌐️ Unsupported kinds retain their identifier in every selected locale and terminology.
#[test]
fn unsupported_question_messages_follow_language_neutral_vectors() {
    let source = fixture_text!("../../../🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🧫️fixtures/🔣️controls.json");
    let oracle: serde_json::Value = serde_json::from_str(source).unwrap();
    let parsed = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&parsed)).unwrap(), oracle);
    let vector = &oracle["unsupported"];
    let question = &vector["question"];
    let question = crate::editor::forms::questions::question_shell(question["id"].as_str().unwrap().into(), question["label"].as_str().unwrap().into(), question["kind"].as_str().unwrap().into());
    for case in vector["cases"].as_array().unwrap() {
        let view = semio_plugin::ViewModel::new(semio_ui_locale::Locale::parse(case["locale"].as_str().unwrap()).unwrap(), semio_ui_locale::Terminology::parse(case["terminology"].as_str().unwrap()).unwrap());
        let node = render_try_question(&question, &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&view), &view).unwrap();
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(&serde_json::to_string(&case["expected"]).unwrap()), "{}: {json}", case);
    }
}
