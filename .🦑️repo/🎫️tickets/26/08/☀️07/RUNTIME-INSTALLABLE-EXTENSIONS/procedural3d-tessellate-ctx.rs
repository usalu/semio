        if parsed.get("error").and_then(Value::as_str).is_some() {
            return ("[]".into(), "[]".into());
        }
    }
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let show_mode = if cfg.show_mode.is_empty() { "solid" } else { cfg.show_mode.as_str() };
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let handles = geometry_handles_for_widget(&eval, &id);
        if handles.is_empty() {
            continue;
        }
        let selected = cfg.selected_node_ids.iter().any(|entry| entry == &id);
        let hovered = cfg.hovered_node_id.as_deref() == Some(id.as_str());
        for (index, handle) in handles.iter().enumerate() {
            let mesh_id = if handles.len() == 1 { format!("eval-{id}") } else { format!("eval-{id}#{index}") };
            let instance_id = if handles.len() == 1 { id.clone() } else { format!("{id}#{index}") };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                if let Ok(data) = tessellate_geometry(handle, tolerance) {
                    let data = apply_show_mode_mesh(data, show_mode);
                    if mesh_has_preview_geometry(&data) {
                        meshes.push(json!({ "id": mesh_id, "data": data }));
                    }
                }
            }
            if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                instances.push(json!({
                    "id": instance_id,
                    "meshId": mesh_id,
                    "position": [0.0, 0.0, 0.0],
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered,
                }));
            }
        }
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

pub fn evaluate_generation_preview(fixture: &FlowFixture, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
    host.evaluate().unwrap_or_default()
}

pub fn merge_preview_meshes(meshes: &[semio_framework_plugin::MeshData]) -> semio_framework_plugin::MeshData {
    let mut merged = semio_framework_plugin::MeshData::default();