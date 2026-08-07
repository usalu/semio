      "neuronKind": "brep.measure.volume",
      "params": {},
      "input_ports": ["geometry"],
      "preview": false
    }
  ],
  "synapses": [
    { "id": "e1", "from": "width", "to": "rect", "fromPort": "number", "toPort": "width" },
    { "id": "e2", "from": "height", "to": "rect", "fromPort": "number", "toPort": "height" },
    { "id": "e3", "from": "rect", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e4", "from": "distance", "to": "vector", "fromPort": "number", "toPort": "z" },
    { "id": "e5", "from": "vector", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "volume", "fromPort": "solid", "toPort": "geometry" }
  ],
  "layout": {
    "rect": { "x": 120, "y": -40 },
    "vector": { "x": 200, "y": 20 },
    "extrude": { "x": 280, "y": -40 },
    "volume": { "x": 360, "y": -40 },
    "width": { "x": 40, "y": -60 },
    "height": { "x": 40, "y": -20 },
    "distance": { "x": 120, "y": 20 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
        assert_eq!(solid.get("$schema").and_then(|v| v.as_str()), Some("geometry"));
        assert_eq!(solid.get("kind").and_then(|v| v.as_str()), Some("solid"));
    }

    #[test]
    fn hexagonal_mushroom_fixture_reports_extruded_solid_output() {
        // 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
        // handcrafted DSL (`crate::os_store::DocumentDsl`) — inlined the same flow-fixture JSON this test actually
        // parses (`FlowHost::parse_fixture_json`), decoupled from procedural's document format.
        let json = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
        let fixture = FlowHost::parse_fixture_json(json).expect("fixture json");
        let mut host = FlowHost::from_fixture(fixture);
        host.set_neuron_kind_infos_json(&fixture_kind_infos_json());
        let eval_json = host.evaluate().expect("evaluate");
        let parsed: serde_json::Value = serde_json::from_str(&eval_json).expect("eval json");
        let solid = parsed.get("extrude").and_then(|entry| entry.get("out")).and_then(|out| out.get("solid").or_else(|| out.get("S"))).expect("extrude solid output");
        assert_eq!(solid.get("$schema").and_then(serde_json::Value::as_str), Some("geometry"));
        assert_eq!(solid.get("kind").and_then(serde_json::Value::as_str), Some("solid"));
        let handle = solid.get("handle").and_then(serde_json::Value::as_str).expect("solid handle");
        assert!(handle.starts_with("solid-"));
        let mesh = flow_extension_brep::tessellate_geometry(handle, 0.05).expect("solid mesh");
        assert!(!mesh.positions.is_empty());
        assert!(mesh.indices.len() >= 3);
    }

    #[test]
    fn compiled_wire_literal_includes_operator_kinds() {
        let host = host_with_test_bridge();
        let text = host.compiled_wire_literal();
        assert!(text.contains("core.number"));
        assert!(text.contains("math.add"));
    }

    #[test]
    fn flow_fixture_to_form_spec_maps_input_widgets() {
        use self::forms_bridge::flow_fixture_to_form_spec;
        let fixture = FlowFixture::default();
        let spec = flow_fixture_to_form_spec(&fixture);
        let kinds: Vec<&str> = spec.steps[0].blocks.iter().map(|question| question.kind.as_str()).collect();
        assert!(kinds.contains(&"slider"));
    }

    #[test]
    fn apply_generation_values_to_fixture_patches_slider_value() {
        use self::forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec};
        let fixture = FlowFixture::default();
        let spec = flow_fixture_to_form_spec(&fixture);
        let slider_id = spec.steps[0].blocks.iter().find(|question| question.kind == "slider").map(|question| question.id.clone()).expect("slider question");
        let fixture_json = serde_json::to_string(&fixture).expect("fixture json");
        let mut values = serde_json::Map::new();
        values.insert(slider_id.clone(), serde_json::json!(8.0));
        let patched = apply_generation_values_to_fixture(&fixture_json, &values);
        let reparsed: serde_json::Value = serde_json::from_str(&patched).expect("patched json");
        let slider = reparsed.get("widgets").and_then(|widgets| widgets.as_array()).and_then(|widgets| widgets.iter().find(|widget| widget.get("id").and_then(|id| id.as_str()) == Some(slider_id.as_str()))).expect("slider widget");
        assert_eq!(slider.get("value").and_then(|value| value.as_f64()), Some(8.0));
    }
}
// #endregion 🔖️Tests
