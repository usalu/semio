
use super::*;
use flow_extension_sdk::{build_manifest_json, evaluate_invoke_json, evaluate_json, flow_extension_topic_contribution};

fn channel_payload(out: &Dictionary, channel: &str) -> Dictionary {
    out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload")
}

/// 🌱️ Wire-shape twin of [`super::number_dictionary`], built with the first-party
/// `pack::json::Value` instead of `Dictionary`'s own `serde` codec — for JSON-text tests only.
fn json_number(value: f64) -> pack::json::Value {
    pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))])
}

#[semio_framework_async_macros::async_test]
async fn wall_element_emits_wall_schema() {
    let mut reg = Registry::new();
    register(&mut reg);
    let out = reg
        .dispatch("bim.element.wall", &Dictionary::new().insert("length", Value::Dictionary(number_dictionary(5.0))).insert("height", Value::Dictionary(number_dictionary(3.0))).insert("thickness", Value::Dictionary(number_dictionary(0.2))))
        .unwrap();
    let wall = channel_payload(&out, "wall");
    assert_eq!(wall.schema(), Some("wall"));
    assert_eq!(read_field_number(&wall, "length"), Some(5.0));
}

#[semio_framework_async_macros::async_test]
async fn assemble_story_splits_spaces() {
    let mut reg = Registry::new();
    register(&mut reg);
    let wall = channel_payload(
        &reg.dispatch("bim.element.wall", &Dictionary::new().insert("length", Value::Dictionary(number_dictionary(4.0))).insert("height", Value::Dictionary(number_dictionary(2.8))).insert("thickness", Value::Dictionary(number_dictionary(0.2))))
            .unwrap(),
        "wall",
    );
    let space = channel_payload(
        &reg.dispatch("bim.element.space", &Dictionary::new().insert("name", Value::Dictionary(text_dictionary("Lobby"))).insert("area", Value::Dictionary(number_dictionary(40.0))).insert("height", Value::Dictionary(number_dictionary(3.0))))
            .unwrap(),
        "space",
    );
    let slab = channel_payload(
        &reg.dispatch("bim.element.slab", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(10.0))).insert("depth", Value::Dictionary(number_dictionary(8.0))).insert("thickness", Value::Dictionary(number_dictionary(0.25))))
            .unwrap(),
        "slab",
    );
    let story = channel_payload(
        &reg.dispatch(
            "bim.assemble.story",
            &Dictionary::new()
                .insert("elevation", Value::Dictionary(number_dictionary(0.0)))
                .insert("height", Value::Dictionary(number_dictionary(3.0)))
                .insert("slab", Value::Dictionary(slab))
                .insert("elements", Value::Dictionary(Dictionary::new().insert("0", Value::Dictionary(wall)).insert("1", Value::Dictionary(space)))),
        )
        .unwrap(),
        "story",
    );
    assert_eq!(story.schema(), Some("story"));
    let elements = story.get("elements").and_then(|value| value.as_dictionary()).unwrap();
    let spaces = story.get("spaces").and_then(|value| value.as_dictionary()).unwrap();
    assert_eq!(list_indices(elements).len(), 1);
    assert_eq!(list_indices(spaces).len(), 1);
    assert_eq!(read_field_text(spaces.get("0").and_then(|value| value.as_dictionary()).unwrap(), "name"), Some("Lobby".into()));
}

#[semio_framework_async_macros::async_test]
async fn assemble_building_and_measure_floor_area() {
    let mut reg = Registry::new();
    register(&mut reg);
    let slab = channel_payload(
        &reg.dispatch("bim.element.slab", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(10.0))).insert("depth", Value::Dictionary(number_dictionary(8.0))).insert("thickness", Value::Dictionary(number_dictionary(0.25))))
            .unwrap(),
        "slab",
    );
    let story = channel_payload(
        &reg.dispatch("bim.assemble.story", &Dictionary::new().insert("height", Value::Dictionary(number_dictionary(3.0))).insert("slab", Value::Dictionary(slab)).insert("elements", Value::Dictionary(Dictionary::new()))).unwrap(),
        "story",
    );
    let building = channel_payload(
        &reg.dispatch("bim.assemble.building", &Dictionary::new().insert("name", Value::Dictionary(text_dictionary("Tower"))).insert("stories", Value::Dictionary(Dictionary::new().insert("0", Value::Dictionary(story))))).unwrap(),
        "building",
    );
    assert_eq!(building.schema(), Some("building"));
    let area = channel_payload(&reg.dispatch("bim.measure.floorArea", &Dictionary::new().insert("building", Value::Dictionary(building))).unwrap(), "floorArea");
    assert_eq!(area.schema(), Some("number"));
    assert_eq!(read_field_number(&area, "value"), Some(80.0));
}

#[semio_framework_async_macros::async_test]
async fn measure_gross_volume() {
    let mut reg = Registry::new();
    register(&mut reg);
    let slab = channel_payload(
        &reg.dispatch("bim.element.slab", &Dictionary::new().insert("width", Value::Dictionary(number_dictionary(10.0))).insert("depth", Value::Dictionary(number_dictionary(10.0))).insert("thickness", Value::Dictionary(number_dictionary(0.25))))
            .unwrap(),
        "slab",
    );
    let story = channel_payload(
        &reg.dispatch("bim.assemble.story", &Dictionary::new().insert("height", Value::Dictionary(number_dictionary(3.0))).insert("slab", Value::Dictionary(slab)).insert("elements", Value::Dictionary(Dictionary::new()))).unwrap(),
        "story",
    );
    let building = channel_payload(
        &reg.dispatch("bim.assemble.building", &Dictionary::new().insert("name", Value::Dictionary(text_dictionary("Block"))).insert("stories", Value::Dictionary(Dictionary::new().insert("0", Value::Dictionary(story))))).unwrap(),
        "building",
    );
    let volume = channel_payload(&reg.dispatch("bim.measure.grossVolume", &Dictionary::new().insert("building", Value::Dictionary(building))).unwrap(), "grossVolume");
    assert_eq!(read_field_number(&volume, "value"), Some(300.0));
}

#[semio_framework_async_macros::async_test]
async fn manifest_lists_bim_operators() {
    let json = build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
    assert!(json.contains("flow.extension"));
    assert!(json.contains("bim.element.wall"));
    assert!(json.contains("bim.assemble.building"));
    assert!(json.contains("bim.measure.floorArea"));
    assert!(json.contains("\"building\""));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_json_wall() {
    let reg = module_registry();
    let input_json = pack::json::to_string(&pack::json::object([("length".to_string(), json_number(4.0)), ("height".to_string(), json_number(2.8)), ("thickness".to_string(), json_number(0.2))]));
    let out_json = evaluate_json(&reg, "bim.element.wall", &input_json);
    let out = pack::json::parse(&out_json).unwrap();
    assert_eq!(out.get("wall").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("wall"));
}

#[semio_framework_async_macros::async_test]
async fn schema_component_round_trips_wall() {
    let mut reg = Registry::new();
    register(&mut reg);
    let built =
        reg.dispatch("bim.wall", &Dictionary::new().insert("length", Value::Dictionary(number_dictionary(5.0))).insert("height", Value::Dictionary(number_dictionary(3.0))).insert("thickness", Value::Dictionary(number_dictionary(0.2)))).unwrap();
    let wall = channel_payload(&built, "wall");
    let deconstructed = reg.dispatch("bim.wall", &Dictionary::new().insert("wall", Value::Dictionary(wall))).unwrap();
    assert_eq!(deconstructed.get("length").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(5.0));
}

#[semio_framework_async_macros::async_test]
async fn extension_bundle_extends_flow_and_evaluates() {
    use semio_framework_plugin::{ExtensionBundle, extension_activate, extension_invoke, extension_manifest, install_extension_bundle};

    let manifest_json = build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
    let flow_topic = flow_extension_topic_contribution("flow-play", "bim", "Bim", "bim", &manifest_json);
    let procedural3d_topic = flow_extension_topic_contribution("procedural3d-play", "bim", "Bim", "bim", &manifest_json);
    let bundle = ExtensionBundle::new("flow-extension-bim", "Bim", "0.1.0")
        .extends("flow")
        .contributes_topic(flow_topic.topic, flow_topic.payload)
        .contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload)
        .handler("evaluate", |req| Ok(evaluate_invoke_json(&module_registry(), req).unwrap()));
    install_extension_bundle(bundle).await;
    extension_activate().await.unwrap();
    let installed = extension_manifest().await;
    assert_eq!(installed.extension_id, "flow-extension-bim");
    assert_eq!(installed.extends, "flow");
    assert_eq!(installed.topic_contributions.len(), 2);
    assert_eq!(installed.topic_contributions[0].topic, "flow.extension");
    let input_json = pack::json::to_string(&pack::json::object([("length".to_string(), json_number(4.0)), ("height".to_string(), json_number(2.8)), ("thickness".to_string(), json_number(0.2))]));
    let req =
        pack::json::to_string(&pack::json::object([("operatorId".to_string(), pack::json::Value::from("bim.element.wall")), ("inputJson".to_string(), pack::json::Value::from(input_json)), ("nodeHash".to_string(), pack::json::Value::from(1_i64))]));
    let out_bytes = extension_invoke("evaluate", req.as_bytes()).await.unwrap();
    let out = pack::json::parse_bytes(&out_bytes).unwrap();
    assert_eq!(out.get("wall").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("wall"));
}
