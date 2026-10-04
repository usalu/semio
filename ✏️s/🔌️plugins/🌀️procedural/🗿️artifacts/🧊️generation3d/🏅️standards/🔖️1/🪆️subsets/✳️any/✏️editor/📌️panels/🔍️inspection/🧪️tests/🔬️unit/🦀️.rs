use super::*;

#[test]
fn inspector_selected_slider_has_localized_commit_control() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let case = &fixture["selectionPublication"];
    let host = FlowHostSnapshot { widgets: vec![Widget::InputSlider { id: case["widgetId"].as_str().unwrap().into(), label: String::new(), value: case["value"].as_f64().unwrap(), min: 0.0, max: 10.0, step: 0.1 }], synapses: Vec::new(), ..Default::default() };
    for (locale, labels) in [("en", &Generation3dLabels::NATIVE_EN), ("de", &Generation3dLabels::NATIVE_DE)] {
        let built = render(&host, &[case["widgetId"].as_str().unwrap().into()], labels, &semio_framework_plugin::TreeWindows::unhosted(), if locale == "de" { semio_framework_ui_locale::Locale::De } else { semio_framework_ui_locale::Locale::En }, semio_framework_ui_locale::Terminology::Native, &[], "", "{}", true).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(built)).unwrap();
        let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
        let control = inspector_node(&tree, case["controlKey"].as_str().unwrap()).unwrap();
        for field in ["type", "kind", "commit"] { assert_eq!(control["component"][field], case["control"][field]); }
        assert_eq!(control["accessibility"]["label"], case["control"][locale]);
        let binding = &control["bindings"][0];
        assert_eq!(binding["trigger"], case["control"]["trigger"]);
        assert_eq!(binding["action"]["name"], case["control"]["action"]);
        assert_eq!(binding["args"]["widgetIds"], serde_json::json!([case["widgetId"]]));
        assert_eq!(binding["args"]["field"], "value");
    }
    host.retire_cold();
}

#[test]
fn inspector_mesh_source_projects_structured_fields_and_retained_actions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let source: serde_json::Value = serde_json::from_str(include_str!("../../../../🎮️commands/🎚️set-widget-input/🧫️fixtures/🔣️.json")).unwrap();
    for (locale, labels) in [("en", &Generation3dLabels::NATIVE_EN), ("de", &Generation3dLabels::NATIVE_DE)] {
        let built = mesh_source_input(&semio_framework_plugin::TreeWindows::unhosted(), "construct", "data", &source["meshSource"]["mesh"].to_string(), labels).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(built)).unwrap();
        let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
        for expected in fixture["meshSourceControls"].as_array().unwrap() {
            let node = inspector_node(&tree, expected["key"].as_str().unwrap()).unwrap_or_else(|| panic!("{projection}"));
            assert_eq!(node["component"]["kind"], expected["kind"]); assert_eq!(node["component"]["value"], expected["value"]); assert_eq!(node["component"]["commit"], "blur");
            assert_eq!(node["accessibility"]["label"], expected[locale]);
            let args = &node["bindings"][0]["args"]; assert_eq!(args["facet"], "meshSource"); assert_eq!(args["widgetId"], "construct"); assert_eq!(args["channel"], "data"); assert_eq!(args["path"], expected["path"]);
        }
        for expected in fixture["meshSourceActions"].as_array().unwrap() {
            let node = inspector_node(&tree, expected["key"].as_str().unwrap()).unwrap_or_else(|| panic!("{projection}"));
            assert_eq!(node["bindings"][0]["trigger"], "activate"); assert_eq!(node["bindings"][0]["action"]["name"], "setWidgetInput");
            let args = &node["bindings"][0]["args"]; assert_eq!(args["facet"], "meshSource"); assert_eq!(args["path"], expected["path"]); assert_eq!(args["operation"], expected["operation"]); assert_eq!(args["value"], expected["value"]);
        }
        assert!(semio_framework_plugin::artifact_app_laws::document_input_commit_findings(&tree, &|name| name == "setWidgetInput").is_empty());
    }
}

#[test]
fn inspector_mesh_source_window_reaches_late_vertices() {
    use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};
    use semio_framework_ui_locale::{Locale, Terminology};
    let key = "procedural-play-inspector.mesh.0";
    let source = serde_json::json!({"vertices":(0..1024).map(|index| [index, 0, 0]).collect::<Vec<_>>(),"faces":[[0,1,2]]}).to_string();
    for (locale, labels) in [(Locale::En, &Generation3dLabels::NATIVE_EN), (Locale::De, &Generation3dLabels::NATIVE_DE)] {
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: GENERATION_3D_PLAY_BODY_INSPECTION.into(), node_key: key.into(), open: Some(true), offset: 1000, rows: 2 }], ..ViewModel::new(locale, Terminology::Native) };
        let tree = mesh_source_input(&TreeWindows::for_body(&view, GENERATION_3D_PLAY_BODY_INSPECTION), "construct", "data", &source, labels).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
        for index in [1000, 1001] {
            let node = inspector_node(&tree, &format!("{key}.{index}.field.0.field.input")).unwrap_or_else(|| panic!("{projection}"));
            assert_eq!(node["component"]["value"], index.to_string()); assert_eq!(node["bindings"][0]["args"]["path"], serde_json::json!(["vertices",index.to_string(),"0"]));
        }
        assert!(inspector_node(&tree, &format!("{key}.0")).is_none());
    }
}

fn inspector_node<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if node["key"] == key { return Some(node); }
    node["children"].as_array()?.iter().find_map(|child| inspector_node(child, key))
}

#[test]
fn inspector_language_neutral_ports_project_editable_and_connected_controls() {
    use semio_framework_value::FromValue as _;
    use semio_framework_os_flow::{FlowExtensionSpec, FlowHost};
    use semio_framework_os_flow::neural::{Cardinality, ChannelSpec, OperatorInfo};
    let _serial = crate::test_serial::lock();
    fn install(registry: &mut semio_framework_os_flow::neural::Registry) {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
        let ports = fixture["ports"].as_array().unwrap().iter().map(|port| {
            let name = port["name"].as_str().unwrap();
            let types = port["valueTypes"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
            let mut spec = ChannelSpec::named(name, name, name, name).with_value_types(&types);
            if let Some(value) = port.get("default") { spec = spec.with_default(Value::from_value(semio_framework_value::DslValue::from(value.clone())).unwrap()); }
            if port["collection"] == true {
                let items = port["itemTypes"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
                spec = spec.with_item_types(&items).with_cardinality(Cardinality::ZeroOrMore);
            }
            spec
        }).collect();
        registry.register_operator(OperatorInfo { id: "inspector-input-law.typed".into(), extension: "inspector-input-law".into(), name: "Typed Inputs".into(), inputs: ports, ..Default::default() }, Vec::new(), &[]);
    }
    semio_framework_os_flow::install_flow_extension(FlowExtensionSpec { id: "inspector-input-law".into(), name: "Inspector Input Law".into(), version: "1".into(), install }).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut host = FlowHost::default().with_operator_registry(semio_framework_os_flow::flow_operator_registry());
    let source = host.add_widget(r#"{"kind":"inputSlider","id":"height-source","label":"Height"}"#, 0.0, 0.0).unwrap();
    let shape = host.add_widget(r#"{"kind":"neuron","id":"shape","neuronKind":"inspector-input-law.typed"}"#, 200.0, 0.0).unwrap();
    host.connect_ports(&source, "number", &shape, "height").unwrap();
    let projections = [("en", &Generation3dLabels::NATIVE_EN), ("de", &Generation3dLabels::NATIVE_DE)].map(|(locale, labels)| {
        let tree = render(&host.host_snapshot, std::slice::from_ref(&shape), labels, &semio_framework_plugin::TreeWindows::unhosted(), if locale == "de" { semio_framework_ui_locale::Locale::De } else { semio_framework_ui_locale::Locale::En }, semio_framework_ui_locale::Terminology::Native, &[], "", "{}", true).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        (locale, projection)
    });
    host.retire_cold();
    for (locale, projection) in projections {
        let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
        for expected in fixture["controls"].as_array().unwrap() {
            let port = expected["port"].as_str().unwrap();
            let coordinate = expected["component"].as_str().map(|axis| format!(".{axis}")).unwrap_or_default();
            let key = format!("procedural-play-inspector.input.{port}{coordinate}.input");
            let control = inspector_node(&tree, &key).unwrap_or_else(|| panic!("missing {key}: {projection}"));
            for field in ["type", "kind", "value", "appearance"] {
                if let Some(value) = expected.get(field) { assert_eq!(&control["component"][field], value, "{locale} {key} {field}"); }
            }
            assert_eq!(control["accessibility"]["label"], expected[locale], "{locale} {key} accessible label");
            if expected["type"] == "input" { assert_eq!(control["component"]["commit"], "blur", "{key}"); }
            let binding = &control["bindings"][0];
            assert_eq!(binding["trigger"], expected["trigger"], "{key}");
            assert_eq!(binding["action"]["name"], "setWidgetInput", "{key}");
            assert_eq!(binding["args"]["widgetId"], shape, "{key}");
            assert_eq!(binding["args"]["channel"], port, "{key}");
            assert_eq!(binding["args"]["component"], expected["component"], "{key}");
        }
        for port in fixture["readonly"].as_array().unwrap() {
            let key = format!("procedural-play-inspector.input.{}", port.as_str().unwrap());
            let row = inspector_node(&tree, &key).unwrap();
            assert!(row["children"].as_array().unwrap().is_empty(), "{key} unexpectedly editable");
            if port == "height" { assert!(row["component"]["label"].as_str().unwrap().contains(&format!("{source}.number"))); }
        }
        for expected in fixture["collectionControls"].as_array().unwrap() {
            let port = expected["port"].as_str().unwrap();
            let suffix = expected["component"].as_str().unwrap_or("value");
            let key = format!("procedural-play-inspector.input.{port}.0.{suffix}.input");
            let control = inspector_node(&tree, &key).unwrap_or_else(|| panic!("missing {locale} {key}: {projection}"));
            for field in ["type", "kind", "value", "appearance"] { if let Some(value) = expected.get(field) { assert_eq!(&control["component"][field], value, "{locale} {key} {field}"); } }
            assert_eq!(control["accessibility"]["label"], expected[locale], "{locale} {key}");
            let binding = &control["bindings"][0];
            assert_eq!(binding["trigger"], expected["trigger"]);
            assert_eq!(binding["action"]["name"], "setWidgetInput");
            assert_eq!(binding["args"]["widgetId"], shape);
            assert_eq!(binding["args"]["channel"], port);
            assert_eq!(binding["args"]["index"], expected["index"]);
            assert_eq!(binding["args"]["operation"], "set");
            assert_eq!(binding["args"]["component"], expected["component"]);
        }
        for expected in fixture["collectionActions"].as_array().unwrap() {
            let port = expected["port"].as_str().unwrap();
            let key = format!("procedural-play-inspector.input.{port}.{}", expected["key"].as_str().unwrap());
            let button = inspector_node(&tree, &key).unwrap_or_else(|| panic!("missing {locale} {key}: {projection}"));
            let binding = &button["bindings"][0];
            assert_eq!(binding["trigger"], "activate");
            assert_eq!(binding["action"]["name"], "setWidgetInput");
            assert_eq!(binding["args"]["widgetId"], shape);
            assert_eq!(binding["args"]["channel"], port);
            assert_eq!(binding["args"]["index"], expected["index"]);
            assert_eq!(binding["args"]["operation"], expected["operation"]);
            assert_eq!(binding["args"]["value"], "");
            assert!(!button["accessibility"]["label"].as_str().unwrap().is_empty());
        }
        assert!(semio_framework_plugin::artifact_app_laws::document_input_commit_findings(&tree, &|name| name == "setWidgetInput").is_empty());
    }
}

#[test]
fn inspector_boolean_controls_and_literal_text_have_distinct_accessible_controls() {
    let boolean = editable_input("boolean", "Enabled", "shape", "enabled", None, "true", None).unwrap();
    let text = editable_input("text", "Text", "note", "text", None, "true", Some(InputKind::LongText)).unwrap();
    let boolean = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(boolean)).unwrap();
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(text)).unwrap();
    assert!(boolean.contains("checkbox"));
    assert!(boolean.contains("Enabled"));
    assert!(text.contains("longText"));
    assert!(text.contains("Text"));
}

#[test]
fn inspector_collection_window_reaches_late_items_and_preserves_fixed_cardinality() {
    use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue;
    use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};
    use semio_framework_ui_locale::{Locale, Terminology};
    let key = "procedural-play-inspector.input.rows";
    let items = (0..1024).map(|index| WidgetInputValue::Number(index as f64)).collect::<Vec<_>>();
    for (locale, labels) in [(Locale::En, &Generation3dLabels::NATIVE_EN), (Locale::De, &Generation3dLabels::NATIVE_DE)] {
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: GENERATION_3D_PLAY_BODY_INSPECTION.into(), node_key: key.into(), open: Some(true), offset: 1000, rows: 2 }], ..ViewModel::new(locale, Terminology::Native) };
        let tree = collection_input(&TreeWindows::for_body(&view, GENERATION_3D_PLAY_BODY_INSPECTION), key, labels.input_name_rows.as_str(), "shape", "rows", &items, &semio_framework_os_flow::neural::Cardinality::from_symbol("1024").unwrap(), labels).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
        let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
        assert!(inspector_node(&tree, &format!("{key}.add")).is_none());
        for index in [1000, 1001] {
            let control = inspector_node(&tree, &format!("{key}.{index}.value.input")).unwrap();
            assert_eq!(control["component"]["value"], index.to_string());
            assert_eq!(control["bindings"][0]["args"]["index"], index);
            assert!(inspector_node(&tree, &format!("{key}.{index}.remove")).is_none());
            let up = inspector_node(&tree, &format!("{key}.{index}.up")).unwrap();
            assert_eq!(up["bindings"][0]["args"]["destination"], index - 1);
            let down = inspector_node(&tree, &format!("{key}.{index}.down")).unwrap();
            assert_eq!(down["bindings"][0]["args"]["destination"], index + 1);
        }
        assert!(inspector_node(&tree, &format!("{key}.0")).is_none());
    }
}

#[test]
fn inspector_widget_metadata_controls_bind_explicit_localized_facets() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["widgets"].as_array().unwrap() {
        let widget: Widget = semio_framework_pack_json::from_json_str(&entry["widget"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let id = crate::widget_id(&widget).to_string();
        let snapshot = FlowHostSnapshot { widgets: vec![widget], ..Default::default() };
        for (locale, key, labels) in [(semio_framework_ui_locale::Locale::En, "en", &Generation3dLabels::NATIVE_EN), (semio_framework_ui_locale::Locale::De, "de", &Generation3dLabels::NATIVE_DE)] {
            let evaluation = entry.get("evaluation").map(serde_json::Value::to_string).unwrap_or_default();
            let tree = render(&snapshot, std::slice::from_ref(&id), labels, &semio_framework_plugin::TreeWindows::unhosted(), locale, semio_framework_ui_locale::Terminology::Native, &[], &evaluation, "{}", true).unwrap();
            let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
            let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
            for expected in entry["controls"].as_array().unwrap() {
                let control = inspector_node(&tree, &format!("procedural-play-inspector.{}.input", expected["key"].as_str().unwrap())).unwrap();
                assert_eq!(control["component"]["type"], expected["type"]);
                assert_eq!(control["component"]["value"], expected["value"]);
                assert_eq!(control["accessibility"]["label"], expected[key]);
                let binding = &control["bindings"][0];
                assert_eq!(binding["trigger"], expected["trigger"]);
                assert_eq!(binding["action"]["name"], "setWidgetInput");
                assert_eq!(binding["args"]["facet"], expected["facet"]);
                assert_eq!(binding["args"]["channel"], expected["channel"]);
                assert_eq!(binding["args"]["widgetId"], id);
                if expected["type"] == "select" { assert_eq!(control["component"]["items"].as_array().unwrap().len(), crate::standards::v1::subsets::any::io::document_io::EXPORT_FORMATS.len()); }
            }
            if let Some(value) = entry.get("reflectedValue") {
                let row = inspector_node(&tree, "procedural-play-inspector.variable-value").unwrap();
                assert!(row["component"]["label"].as_str().unwrap().ends_with(value.as_str().unwrap()));
            } else {
                let button = inspector_node(&tree, "procedural-play-inspector.export-connected.button").unwrap();
                assert_eq!(button["bindings"][0]["action"]["name"], "exportDocument");
                assert_eq!(button["bindings"][0]["args"]["widgetId"], id);
                assert_eq!(button["bindings"][0]["args"]["format"], entry["widget"]["format"]);
                assert_eq!(button["accessibility"]["label"], labels.input_export_connected.as_str());
            }
        }
        snapshot.retire_cold();
    }
}

#[test]
fn inspector_connected_export_shows_localized_losses_and_pending_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for entry in fixture["exportStatuses"].as_array().unwrap() {
        let format = entry["format"].as_str().unwrap();
        let snapshot = FlowHostSnapshot { widgets: vec![Widget::OutputExport { id: "download".into(), format: format.into() }], ..Default::default() };
        let meshes = [semio_framework_plugin::MeshData { normals: entry["normals"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect(), ..Default::default() }];
        for (locale, key, labels) in [(semio_framework_ui_locale::Locale::En, "en", &Generation3dLabels::NATIVE_EN), (semio_framework_ui_locale::Locale::De, "de", &Generation3dLabels::NATIVE_DE)] {
            let ready = entry["ready"].as_bool().unwrap();
            let tree = render(&snapshot, &["download".into()], labels, &semio_framework_plugin::TreeWindows::unhosted(), locale, semio_framework_ui_locale::Terminology::Native, &meshes, "", "{}", ready).unwrap();
            let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
            let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
            let button = inspector_node(&tree, "procedural-play-inspector.export-connected.button").unwrap();
            assert_eq!(button["disabled"], !ready);
            let row = if ready { inspector_node(&tree, "procedural-play-inspector.export-loss.normal").unwrap() } else { inspector_node(&tree, "procedural-play-inspector.export-pending").unwrap() };
            assert_eq!(row["component"]["label"], entry[key]);
        }
        snapshot.retire_cold();
    }
}

#[test]
fn inspector_mesh_assets_are_localized_bounded_canonical_controls() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let source: serde_json::Value = serde_json::from_str(include_str!("../../../../🎮️commands/🎚️set-widget-input/🧫️fixtures/🔣️.json")).unwrap();
    for (locale, labels) in [("en", &Generation3dLabels::NATIVE_EN), ("de", &Generation3dLabels::NATIVE_DE)] {
        let view = semio_framework_plugin::ViewModel { tree_windows: fixture["meshAssetsWindows"].as_array().unwrap().iter().map(|row| semio_framework_plugin::TreeWindowRequest { body_key: GENERATION_3D_PLAY_BODY_INSPECTION.into(), node_key: row["nodeKey"].as_str().unwrap().into(), open: row["open"].as_bool(), offset: row["offset"].as_u64().unwrap() as u32, rows: row["rows"].as_u64().unwrap() as u32 }).collect(), ..semio_framework_plugin::ViewModel::new(if locale == "de" { semio_framework_ui_locale::Locale::De } else { semio_framework_ui_locale::Locale::En }, semio_framework_ui_locale::Terminology::Native) };
        let built = mesh_source_input(&semio_framework_plugin::TreeWindows::for_body(&view, GENERATION_3D_PLAY_BODY_INSPECTION), "construct", "data", &source["meshAssets"]["mesh"].to_string(), labels).unwrap();
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(built)).unwrap();
        let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
        fn control<'a>(tree: &'a serde_json::Value, path: &serde_json::Value) -> Option<&'a serde_json::Value> {
            if tree["bindings"][0]["args"]["path"] == *path && tree["bindings"][0]["args"]["operation"].is_null() { return Some(tree); }
            tree.get("children")?.as_array()?.iter().find_map(|child| control(child, path))
        }
        for expected in fixture["meshAssetsControls"].as_array().unwrap() {
            let node = control(&tree, &expected["path"]).unwrap_or_else(|| panic!("Missing {} in {projection}", expected["path"]));
            assert_eq!(node["component"]["type"], expected["type"]);
            assert_eq!(node["accessibility"]["label"], expected[locale]);
            assert_eq!(node["bindings"][0]["action"]["name"], "setWidgetInput");
            assert_eq!(node["bindings"][0]["args"]["facet"], "meshSource");
        }
        fn picker<'a>(tree: &'a serde_json::Value, texture: &str) -> Option<&'a serde_json::Value> {
            if tree["bindings"][0]["action"]["name"] == "importDocumentRequest" && tree["bindings"][0]["args"]["textureId"] == texture { return Some(tree); }
            tree["children"].as_array()?.iter().find_map(|child| picker(child, texture))
        }
        let replace = picker(&tree, "image").unwrap();
        assert_eq!(replace["bindings"][0]["args"]["widgetId"], "construct"); assert_eq!(replace["bindings"][0]["args"]["channel"], "data");
        assert!(picker(&tree, "texture1").is_some());
        assert!(!projection.contains("image: bytes: Item"));
        assert!(projection.contains("image/png"));
        assert!(semio_framework_plugin::artifact_app_laws::document_input_commit_findings(&tree, &|name| name == "setWidgetInput").is_empty());
    }
}

#[test]
fn inspector_selected_analysis_projects_declared_outputs_without_recomputation() {
    use semio_framework_os_flow::neural::{ChannelSpec, OperatorInfo};
    fn install(registry: &mut semio_framework_os_flow::neural::Registry) {
        registry.register_operator(OperatorInfo { id: "inspector-analysis-law".into(), extension: "inspector-analysis-law".into(), name: "Analysis".into(), outputs: [("area", "number"), ("center", "point"), ("topology", "list"), ("report", "text")].map(|(name, schema)| ChannelSpec::named(name, name, name, name).with_value_types(&[schema])).to_vec(), ..Default::default() }, Vec::new(), &[]);
    }
    semio_framework_os_flow::install_flow_extension(semio_framework_os_flow::FlowExtensionSpec { id: "inspector-analysis-law".into(), name: "Inspector Analysis Law".into(), version: "1".into(), install }).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap(); let case = &fixture["selectedOutputs"];
    let host = FlowHostSnapshot { widgets: vec![Widget::Neuron { id: case["id"].as_str().unwrap().into(), neuron_kind: case["kind"].as_str().unwrap().into(), params: Default::default(), input_ports: Vec::new(), output_ports: Vec::new(), preview: false }], ..Default::default() };
    for (locale, labels) in [(semio_framework_ui_locale::Locale::En, &Generation3dLabels::NATIVE_EN), (semio_framework_ui_locale::Locale::De, &Generation3dLabels::NATIVE_DE)] {
        for (available, status, fault) in [(true, "ok", false), (false, "ok", false), (true, "stale", false), (true, "error", true)] {
            let evaluation = if available { serde_json::json!({"analysis":{"out":case["outputs"]}}).to_string() } else { "{}".into() };
            let status_json = serde_json::json!({"analysis":{"status":status}}).to_string();
            let built = render(&host, &["analysis".into()], labels, &semio_framework_plugin::TreeWindows::unhosted(), locale, semio_framework_ui_locale::Terminology::Native, &[], &evaluation, &status_json, true).unwrap();
            let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(built)).unwrap(); let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
            for (port, expected) in case["expectedText"].as_object().unwrap() { let output = inspector_node(&tree, &format!("procedural-play-inspector.output.{port}")).unwrap_or_else(|| panic!("Missing output {port}: {projection}")); let text = output.to_string(); assert!(text.contains(if fault { labels.mesh_output_error.as_str() } else if status == "stale" { labels.mesh_output_stale.as_str() } else if available { expected.as_str().unwrap() } else { labels.input_value_unavailable.as_str() })); assert!(!text.contains("setWidgetInput")); }
        }
    }
    host.retire_cold();
}
