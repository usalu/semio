use super::*;
#[test]
fn semantic_question_controls_match_the_language_neutral_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️controls.json")).unwrap();
    for vector in vectors["cases"].as_array().unwrap() {
        let question = &vector["question"];
        let question = crate::editor::forms::commands::add_question::question_shell(question["id"].as_str().unwrap().into(), question["label"].as_str().unwrap().into(), question["kind"].as_str().unwrap().into());
        let node = render_try_question(&question, &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&semio_framework_plugin::ViewModel::default())).unwrap();
        let control = if question.kind == "note" { &node } else { node.children.get(0).unwrap() };
        let actual = serde_json::to_value(control).unwrap();
        assert_eq!(actual["component"]["type"], vector["component"], "{}", question.kind);
        if vector["interactive"].as_bool().unwrap() {
            assert_eq!(actual["accessibility"]["label"], vector["question"]["label"]);
            assert_eq!(actual["bindings"][0]["trigger"], "change");
            assert_eq!(actual["bindings"][0]["args"]["key"], vector["question"]["id"]);
        }
    }
}
