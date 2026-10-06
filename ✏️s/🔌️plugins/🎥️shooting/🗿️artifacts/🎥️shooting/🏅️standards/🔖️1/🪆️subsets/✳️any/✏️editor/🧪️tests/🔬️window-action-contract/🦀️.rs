use super::*;

#[test]
fn shooting_window_actions_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️window-actions.json")).expect("neutral window vectors");
    let document = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        let view_state = semio_framework_plugin::ViewModel { locale, ..semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native) };
        let labels = shooting_play_labels(&view_state);
        for (kind, measures) in [("scene", scene_window::window_measures(&document, labels)), ("icon", icon_window::window_measures(&document, labels))] {
            let json = serde_json::to_value(measures).expect("independent window oracle");
            let actions = json
                .as_array()
                .expect("measures")
                .iter()
                .map(|measure| {
                    assert_eq!(measure["onChange"]["controllerId"], vectors["controller"]);
                    measure["onChange"]["action"].clone()
                })
                .collect::<Vec<_>>();
            assert_eq!(serde_json::Value::Array(actions), vectors[kind]);
        }
        let scene = scene_window::engagement(&document, labels);
        let scene_input = scene.input.expect("camera input");
        assert!(scene_input.on_change.is_none() && scene_input.value.is_none(), "typing the camera label publishes nothing: the host holds the line until submit");
        assert_eq!(scene_input.on_submit.expect("camera submit").action, vectors["sceneSubmit"].as_str().expect("scene submit"));
        let icon = icon_window::engagement(&document, labels);
        assert_eq!(icon.input.expect("shot input").on_change.expect("shot change").action, vectors["iconChange"].as_str().expect("icon change"));
    }
}
