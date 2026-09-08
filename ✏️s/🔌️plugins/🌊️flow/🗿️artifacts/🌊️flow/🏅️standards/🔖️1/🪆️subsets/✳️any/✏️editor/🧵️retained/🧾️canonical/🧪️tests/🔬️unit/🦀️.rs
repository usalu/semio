
use super::*;

fn encode(value: Value<'_>, bytes: &mut Vec<u8>) {
    match value {
        Value::Scalar(value) => match value {
            Node::Null => serde_json::to_writer(bytes, &()).unwrap(),
            Node::Bool(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::I64(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::U64(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::I128(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::U128(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::F32(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::F64(value) => serde_json::to_writer(bytes, &value).unwrap(),
            Node::String(value) => serde_json::to_writer(bytes, value).unwrap(),
            _ => panic!("borrowed scalar cannot be an indexed container"),
        },
        Value::Source(value) => encode(value.canonical_json_borrowed_root().unwrap().unwrap(), bytes),
        Value::Array(values) => {
            bytes.push(b'[');
            for (index, value) in values.enumerate() {
                if index != 0 {
                    bytes.push(b',');
                }
                encode(value, bytes);
            }
            bytes.push(b']');
        }
        Value::Object(values) => {
            bytes.push(b'{');
            for (index, (key, value)) in values.enumerate() {
                if index != 0 {
                    bytes.push(b',');
                }
                serde_json::to_writer(&mut *bytes, key).unwrap();
                bytes.push(b':');
                encode(value, bytes);
            }
            bytes.push(b'}');
        }
    }
}

#[test]
fn every_artifact_variant_matches_serde_bytes_including_nested_chrome() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧾️artifact-canonical.json")).unwrap();
    for row in fixture["widgets"].as_array().unwrap() {
        let value: Widget = dsl::FromValue::from_value(dsl::DslValue::from(row.clone())).unwrap();
        if let Widget::Neuron { input_ports, output_ports, .. } = &value {
            assert_eq!(input_ports, &["in"]);
            assert_eq!(output_ports, &["out"]);
        }
        let mut bytes = Vec::new();
        encode(widget(&value), &mut bytes);
        assert_eq!(bytes, serde_json::to_vec(&serde_json::Value::from(dsl::ToValue::to_value(&value))).unwrap(), "widget {:?}", row["kind"]);
        value.retire_cold();
    }
    for row in fixture["mutations"].as_array().unwrap() {
        let value: FlowMutation = dsl::FromValue::from_value(dsl::DslValue::from(row.clone())).unwrap();
        let mut bytes = Vec::new();
        encode(value.canonical_json_borrowed_root().unwrap().unwrap(), &mut bytes);
        assert_eq!(bytes, serde_json::to_vec(&serde_json::Value::from(dsl::ToValue::to_value(&value))).unwrap(), "mutation {:?}", row["mutation"]);
        crate::retirement::retire_mutation(value).retire_cold();
    }
    eprintln!("[DEBUG] Flow borrowed canonical bytes match typed-DSL serde oracle:9 widgets,10 mutations,nonempty ports");
}

#[test]
fn large_unicode_key_and_label_scene_matches_serde_without_an_ordinal_map_scan() {
    let scene = crate::FlowWorkingScene {
        widgets: vec![Widget::InputSlider { id: "height".into(), label: "🌊".repeat(2048), value: 6.0, min: 0.0, max: 10.0, step: 0.5 }],
        layout: flow::OrderedMap::from([("🌊".repeat(1025), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 })]),
        synapses: Vec::new(),
    };
    let mut bytes = Vec::new();
    encode(scene.canonical_json_borrowed_root().unwrap().unwrap(), &mut bytes);
    assert_eq!(bytes, serde_json::to_vec(&serde_json::Value::from(dsl::ToValue::to_value(&scene))).unwrap());
    assert!(bytes.len() > 4096);
    crate::retirement::retire_scene(scene).retire_cold();
    let absent_tree = neural::Neuron { id: "n".into(), kind: "core.number".into(), params: neural::Dictionary::new(), tree: None };
    let mut bytes = Vec::new();
    encode(neuron(&absent_tree), &mut bytes);
    assert_eq!(bytes, serde_json::to_vec(&serde_json::Value::from(dsl::ToValue::to_value(&absent_tree))).unwrap());
    let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
    retirement.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Neurons(vec![absent_tree]));
    retirement.retire_cold();
    eprintln!("[DEBUG] Flow borrowed canonical bytes preserve large Unicode keys, labels and explicit absent-tree null");
}
