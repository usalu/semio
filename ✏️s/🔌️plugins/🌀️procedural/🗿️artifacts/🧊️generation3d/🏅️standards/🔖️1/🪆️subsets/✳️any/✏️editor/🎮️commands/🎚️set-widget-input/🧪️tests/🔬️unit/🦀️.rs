use super::*;
use crate::central_apply::{apply_generation3d_mutation};

#[test]
fn widget_input_language_neutral_mesh_source_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for group in ["meshSource", "meshAssets"] {
    let original = fixture[group]["mesh"].to_string();
    for entry in fixture[group]["cases"].as_array().unwrap() {
        let mut command = entry["command"].clone(); command["widgetId"] = "construct".into(); command["channel"] = "data".into(); command["facet"] = "meshSource".into();
        let payload: SetWidgetInput = semio_framework_pack_json::from_json_str(&command.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = edit_mesh_source(&original, &payload);
        if entry["error"] == true { assert!(result.is_err(), "{}", entry["id"]); continue; }
        let mut expected = entry.get("expectedMesh").cloned().unwrap_or_else(|| fixture[group]["mesh"].clone());
        if let Some(edit) = entry.get("expected") { let mut target = &mut expected; for part in edit["path"].as_array().unwrap() { let key = part.as_str().unwrap(); target = if target.is_array() { &mut target[key.parse::<usize>().unwrap()] } else { &mut target[key] }; } *target = edit["value"].clone(); }
        let actual: serde_json::Value = serde_json::from_str(&result.unwrap().unwrap()).unwrap();
        assert_eq!(actual, expected, "{}", entry["id"]);
    }
    }
}

#[test]
fn widget_input_mesh_source_preserves_connected_owner_and_inverse() {
    use crate::standards::v1::subsets::any::schema::mutations::{inverse_generation3d_mutation};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut base = crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    base.host_snapshot.widgets.push(Widget::InputNote { id: "source".into(), text: fixture["meshSource"]["mesh"].to_string() });
    base.host_snapshot.widgets.push(semio_framework_pack_json::from_json_str(r#"{"kind":"neuron","id":"construct","neuronKind":"brep.mesh.construct"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    base.host_snapshot.synapses.push(semio_framework_pack_json::from_json_str(r#"{"id":"source-link","from":"source","fromPort":"text","to":"construct","toPort":"data","index":0}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let payload = SetWidgetInput { widget_id: "construct".into(), channel: "data".into(), facet: Some("meshSource".into()), path: Some(vec!["vertices".into(), "1".into(), "0".into()]), value: "2".into(), ..Default::default() };
    let leaf = input_leaf(&base.host_snapshot, &payload).unwrap().unwrap();
    let Generation3dMutation::ChangeWidgetInput(input) = &leaf else { panic!("one absolute input leaf"); };
    assert_eq!(input.id, "source"); assert_eq!(input.channel, "text");
    let inverse = inverse_generation3d_mutation(&base, &leaf).expect("valid retained mutation inverse fixture"); assert_eq!(inverse.len(), 1);
    let mut applied = base.clone(); apply_generation3d_mutation(&mut applied, &leaf).unwrap();
    assert_eq!(applied.host_snapshot.synapses, base.host_snapshot.synapses); assert_eq!(applied.host_snapshot.widgets[1], base.host_snapshot.widgets[1]);
    for inverse in inverse { apply_generation3d_mutation(&mut applied, &inverse).unwrap(); }
    assert_eq!(applied.host_snapshot, base.host_snapshot);
    applied.retire_cold(); base.retire_cold();
}

/// 📇️ Supplies typed input metadata for this document-editing law without a geometry evaluator.
fn input_metadata() {
    fn install(registry: &mut semio_framework_os_flow::neural::Registry) {
        use semio_framework_os_flow::neural::{ChannelSpec, OperatorInfo};
        registry.register_operator(OperatorInfo { id: "brep.mesh.box".into(), extension: "widget-input-law".into(), name: "Box".into(), inputs: vec![ChannelSpec::number_default("width", 1.0, &[] as &[&str])], ..Default::default() }, Vec::new(), &[]);
    }
    semio_framework_os_flow::install_flow_extension(semio_framework_os_flow::FlowExtensionSpec { id: "widget-input-law".into(), name: "Widget Input Law".into(), version: "1".into(), install }).expect("typed input metadata admission");
}

#[test]
fn widget_input_language_neutral_collection_cases() {
    use semio_framework_os_flow::neural::Cardinality;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["collections"].as_array().unwrap() {
        let types: Vec<String> = serde_json::from_value(entry["types"].clone()).unwrap();
        let current = (!entry["current"].is_null()).then(|| semio_framework_value::DslValue::from(entry["current"].clone()));
        let mut command = entry["command"].clone();
        command["widgetId"] = serde_json::json!("shape");
        command["channel"] = serde_json::json!("items");
        let payload: SetWidgetInput = semio_framework_pack_json::from_json_str(&command.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = edit_collection_value(&types, current.as_ref(), &payload, &Cardinality::from_symbol(entry["cardinality"].as_str().unwrap()).unwrap());
        if entry["error"] == true { assert!(result.is_err(), "{}", entry["id"]); }
        else {
            let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.unwrap())).unwrap();
            assert_eq!(actual, entry["expected"], "{}", entry["id"]);
        }
    }
}

#[test]
fn widget_input_language_neutral_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["cases"].as_array().unwrap() {
        let types: Vec<String> = entry["types"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_string()).collect();
        let current = if entry["current"].is_null() { None } else { Some(semio_framework_value::DslValue::from(entry["current"].clone())) };
        let result = edit_input_value(&types, current.as_ref(), entry["value"].as_str().unwrap(), entry.get("component").and_then(serde_json::Value::as_str));
        if entry["error"] == true { assert!(result.is_err(), "{}", entry["id"]); }
        else { assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&result.unwrap())).unwrap(), entry["expected"], "{}", entry["id"]); }
    }
}

#[test]
fn widget_input_collections_publish_one_absolute_leaf_and_restore_the_base() {
    use crate::standards::v1::subsets::any::schema::mutations::{inverse_generation3d_mutation};

    use semio_framework_os_flow::{FlowHost, FlowExtensionSpec};
    let _serial = crate::test_serial::lock();
    fn install(registry: &mut semio_framework_os_flow::neural::Registry) {
        use semio_framework_os_flow::neural::{Cardinality, ChannelSpec, OperatorInfo};
        for kind in ["number", "text", "boolean", "point", "vector"] {
            registry.register_operator(OperatorInfo { id: format!("collection-input-law.{kind}"), extension: "collection-input-law".into(), name: kind.into(), inputs: vec![ChannelSpec::named("items", "Items", "items", "Items").with_value_types(&["list"]).with_item_types(&[kind]).with_cardinality(Cardinality::ZeroOrMore)], ..Default::default() }, Vec::new(), &[]);
        }
    }
    semio_framework_os_flow::install_flow_extension(FlowExtensionSpec { id: "collection-input-law".into(), name: "Collection Input Law".into(), version: "1".into(), install }).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["collections"].as_array().unwrap().iter().filter(|entry| entry.get("expected").is_some() && entry["cardinality"] == "*") {
        let mut host = FlowHost::default().with_operator_registry(semio_framework_os_flow::flow_operator_registry());
        let kind = entry["types"][0].as_str().unwrap();
        let shape = host.add_widget(&serde_json::json!({"kind":"neuron","id":"shape","neuronKind":format!("collection-input-law.{kind}")}).to_string(), 0.0, 0.0).unwrap();
        if !entry["current"].is_null() { host.set_neuron_params(&shape, &serde_json::json!({"items":entry["current"]}).to_string()).unwrap(); }
        let mut command = entry["command"].clone();
        command["widgetId"] = serde_json::json!(shape);
        command["channel"] = serde_json::json!("items");
        let payload: SetWidgetInput = semio_framework_pack_json::from_json_str(&command.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let leaf = input_leaf(&host.host_snapshot, &payload).unwrap().expect("one changed input leaf");
        assert!(matches!(leaf, Generation3dMutation::ChangeWidgetInput(_)), "{}", entry["id"]);
        let mut base = crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
        std::mem::replace(&mut base.host_snapshot, host.host_snapshot.clone()).retire_cold();
        host.retire_cold();
        let inverse = inverse_generation3d_mutation(&base, &leaf).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        let Generation3dMutation::ChangeWidgetInput(scalar) = change_widget_input(&shape, "items", WidgetInputValue::Number(1.0)) else { unreachable!() };
        assert!(scalar.landing(&base.host_snapshot.widgets[0], false).is_err(), "a collection never admits a scalar leaf");
        let mut applied = base.clone();
        apply_generation3d_mutation(&mut applied, &leaf).unwrap();
        let widget = &applied.host_snapshot.widgets[0];
        let literal = widget.to_value().get("params").unwrap().get("items").unwrap().clone();
        let expected: WidgetInputValue = semio_framework_pack_json::from_json_str(&entry["expected"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(literal, expected.literal(), "{}", entry["id"]);
        let mut restored = applied.clone();
        for inverse in inverse { apply_generation3d_mutation(&mut restored, &inverse).unwrap(); }
        assert_eq!(restored.host_snapshot, base.host_snapshot, "{}", entry["id"]);
        applied.retire_cold();
        restored.retire_cold();
        base.retire_cold();
    }
}

#[test]
fn widget_input_explicit_widget_facets_use_one_existing_widget_leaf() {
    use crate::standards::v1::subsets::any::schema::mutations::{inverse_generation3d_mutation};

    let _serial = crate::test_serial::lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["facets"].as_array().unwrap() {
        let widget: Widget = semio_framework_pack_json::from_json_str(&entry["widget"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut base = crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
        base.host_snapshot.widgets.push(widget);
        for widget in entry["widgets"].as_array().into_iter().flatten() { base.host_snapshot.widgets.push(semio_framework_pack_json::from_json_str(&widget.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()); }
        for synapse in entry["synapses"].as_array().into_iter().flatten() { base.host_snapshot.synapses.push(semio_framework_pack_json::from_json_str(&synapse.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()); }
        let mut command = entry["command"].clone();
        command["widgetId"] = entry["widget"]["id"].clone();
        let payload: SetWidgetInput = semio_framework_pack_json::from_json_str(&command.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = input_leaf(&base.host_snapshot, &payload);
        if entry["error"] == true { assert!(result.is_err(), "{}", entry["id"]); base.retire_cold(); continue; }
        let leaf = result.unwrap().unwrap();
        assert!(matches!(leaf, Generation3dMutation::UpdateWidget(_)), "{}", entry["id"]);
        let inverse = inverse_generation3d_mutation(&base, &leaf).expect("valid retained mutation inverse fixture");
        assert_eq!(inverse.len(), 1);
        let mut applied = base.clone();
        apply_generation3d_mutation(&mut applied, &leaf).unwrap();
        let expected: Widget = semio_framework_pack_json::from_json_str(&entry["expected"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(applied.host_snapshot.widgets[0], expected, "{}", entry["id"]);
        if let Some(connections) = entry.get("expectedSynapses") { assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&applied.host_snapshot.synapses)).unwrap(), *connections, "{}", entry["id"]); }
        assert!(input_leaf(&applied.host_snapshot, &payload).unwrap().is_none());
        for inverse in inverse { apply_generation3d_mutation(&mut applied, &inverse).unwrap(); }
        assert_eq!(applied.host_snapshot, base.host_snapshot);
        applied.retire_cold();
        base.retire_cold();
    }
}

/// ⚖️ LAW (design §19): a committed operator field is ONE absolute `change-widget-input` leaf of the port's literal type;
/// an unchanged value is no edit, a text source's text is a text input, a connected or unknown input is refused before
/// any document mutation.
#[test]
fn widget_input_is_one_typed_leaf_and_rejects_connected_ports() {
    use semio_framework_os_flow::FlowHost;
    let _serial = crate::test_serial::lock();
    input_metadata();
    let mut host = FlowHost::default().with_operator_registry(semio_framework_os_flow::flow_operator_registry());
    let source = host.add_widget(r#"{"kind":"inputSlider","id":"width","label":"Width"}"#, 0.0, 0.0).unwrap();
    let shape = host.add_widget(r#"{"kind":"neuron","id":"shape","neuronKind":"brep.mesh.box"}"#, 200.0, 0.0).unwrap();
    let note = host.add_widget(r#"{"kind":"inputNote","text":"hi"}"#, 0.0, 200.0).unwrap();
    let mut payload = SetWidgetInput { widget_id: shape.clone(), channel: "width".into(), value: "2.5".into(), component: None, ..Default::default() };
    assert_eq!(input_leaf(&host.host_snapshot, &payload).unwrap(), Some(change_widget_input(&shape, "width", WidgetInputValue::Number(2.5))));
    host.set_neuron_params(&shape, r#"{"width":{"$schema":"number","value":2.5}}"#).unwrap();
    assert_eq!(input_leaf(&host.host_snapshot, &payload).unwrap(), None, "an unchanged field is no edit");
    let text = SetWidgetInput { widget_id: note.clone(), channel: "text".into(), value: "hello".into(), component: None, ..Default::default() };
    assert_eq!(input_leaf(&host.host_snapshot, &text).unwrap(), Some(change_widget_input(&note, "text", WidgetInputValue::Text("hello".into()))));
    host.connect_ports(&source, "number", &shape, "width").unwrap();
    payload.value = "3.5".into();
    assert!(input_leaf(&host.host_snapshot, &payload).unwrap_err().contains("connected"));
    payload.channel = "missing".into();
    assert!(input_leaf(&host.host_snapshot, &payload).is_err());
    host.retire_cold();
}

#[test]
fn widget_input_texture_import_preserves_canonical_assets_and_rejects_invalid_file() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap(); let source = fixture["meshAssets"]["mesh"].to_string();
    for key in ["textureImport", "textureJpegImport"] { let case = &fixture[key];
    let edited = import_mesh_texture(&source, case["target"]["textureId"].as_str().unwrap(), case["payload"].as_str().unwrap()).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&edited).unwrap();
    assert_eq!(actual["textures"]["image"], serde_json::json!({"mime":case["mime"],"bytes":case["bytes"]})); assert_eq!(actual["materials"], fixture["meshAssets"]["mesh"]["materials"]);
    assert!(import_mesh_texture(&source, "image", "data:image/png;base64,YQ==").is_err());
    assert!(import_mesh_texture(&source, "", case["payload"].as_str().unwrap()).is_err());
    assert!(import_mesh_texture(&source, "image", "data:image/jpeg;base64,YQ==").is_err());
    }
}
