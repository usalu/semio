use crate::editor::shooting::config::ShootingConfig;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_ui_contract::BuiltNode;
#[allow(unused_imports)]
use crate::editor::shooting::terminology::ShootingLabels;
#[allow(unused_imports)]
use crate::{editor::shooting::ui_helpers_and_io::shooting_action, editor::shooting::ui_helpers_and_io::ui_capacity_error, editor::shooting::ui_helpers_and_io::ui_label, editor::shooting::ui_helpers_and_io::ui_text, editor::shooting::ui_helpers_and_io::ui_value_list, editor::shooting::ui_helpers_and_io::ui_value_map, editor::shooting::ui_helpers_and_io::ui_value_text};
#[allow(unused_imports)]
use semio_framework_plugin::ui_node_list;
#[allow(unused_imports)]
use crate::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_plugin::{plugin_app_close_prelude::Buildable, plugin_app_close_prelude::HasBase, plugin_app_close_prelude::HasChildren, plugin_app_close_prelude::InputKind, plugin_app_close_prelude::Trigger};
#[allow(unused_imports)]
use semio_framework_plugin::tree_item;
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_desc;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabKind;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTreeBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
#[allow(unused_imports)]
use semio_framework_ui_contract::input;
use crate::editor::shooting::panels::inspection::*;

fn project(node: BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire inspected semantic tree");
    serde_json::from_str(&text).expect("independent semantic JSON oracle")
}

fn view_state(locale: &str) -> semio_framework_plugin::ViewModel {
    let locale = match locale {
        "en" => semio_framework_ui_locale::Locale::En,
        "de" => semio_framework_ui_locale::Locale::De,
        value => panic!("unexpected fixture locale {value}"),
    };
    semio_framework_plugin::ViewModel { locale, ..semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native) }
}

#[test]
fn shooting_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../✏️editor/📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️panels.json")).expect("neutral UI vectors");
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let cfg = ShootingConfig::default();
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = crate::editor::shooting::terminology::shooting_play_labels(&view_state(row["locale"].as_str().expect("locale")));
        let node = render(&snapshot, &cfg, labels).expect("shot inspector");
        let fields = &node.children[0].children;
        let bindings: Vec<serde_json::Value> = [0, 3, 4].into_iter().map(|i| serde_json::to_value(&fields[i].children[0].bindings[0]).expect("independent binding oracle")).collect();
        let tree = project(node);
        assert_eq!(tree["children"][0]["component"]["label"], row["shot"]);
        let fields = tree["children"][0]["children"].as_array().expect("shot fields");
        assert_eq!(serde_json::Value::Array(fields.iter().map(|field| field["component"]["label"].clone()).collect()), row["fields"]);
        for (index, binding) in bindings.iter().enumerate() {
            assert_eq!(binding["args"]["field"], vectors["editableFields"][index]);
            assert_eq!(binding["args"]["shotIds"][0], snapshot.active_shot_id);
            assert_eq!(binding["trigger"], "change");
        }
    }
    for node in [crate::editor::shooting::modes::edit::windows::scene::render(&snapshot, &cfg, "select").expect("editor scene"), crate::viewer::shooting::modes::view::windows::scene::render(&snapshot).expect("viewer scene")] {
        let scene: semio_framework_plugin::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("assembled scene");
        let frame: serde_json::Value = serde_json::from_str(scene.frame_json.as_deref().expect("active frame")).expect("independent frame oracle");
        assert_eq!(frame, vectors["frame"]);
        project(node);
    }
    let node = crate::editor::shooting::modes::edit::windows::icon::render(&snapshot, &cfg).expect("icon scene");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("icon surface") };
    let scene: semio_framework_plugin::IconRenderScene = semio_framework_ui_scene::decode(props).expect("packed icon");
    let request: serde_json::Value = serde_json::from_str(&scene.request_json).expect("independent icon request oracle");
    assert_eq!(request["assetUrl"], vectors["iconAssetUrl"]);
    project(node);
    snapshot.shots.clear();
    snapshot.active_shot_id.clear();
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = crate::editor::shooting::terminology::shooting_play_labels(&view_state(row["locale"].as_str().expect("locale")));
        let tree = project(render(&snapshot, &cfg, labels).expect("empty inspector"));
        assert_eq!(tree["children"][0]["component"]["label"], row["summary"]);
    }
}
