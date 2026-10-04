use super::*;

#[test]
fn shared_mesh_selection_cases_emit_typed_absolute_inputs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["valid"].as_array().unwrap().iter().chain(fixture["analytic"].as_array().unwrap()) {
        let payload: EditMeshSelection = semio_framework_pack_json::from_json_str(&case["payload"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let ids: Vec<String> = serde_json::from_value(case["ids"].clone()).unwrap();
        assert_eq!(payload.granularity(), case["granularity"].as_str().unwrap());
        let inputs = inputs(&payload, &ids).unwrap().into_iter().map(|(channel, input)| {
            let value = match input { WidgetInputValue::Number(value) => serde_json::json!(value), WidgetInputValue::Text(value) => serde_json::json!(value), WidgetInputValue::Vector(value) => serde_json::json!(value), _ => panic!("mesh edit input") };
            (channel.to_string(), value)
        }).collect::<serde_json::Map<_, _>>();
        assert_eq!(serde_json::Value::Object(inputs), case["inputs"], "{}", case["name"]);
        if case.get("name").and_then(|name| name.as_str()).is_some_and(|name| ["filletEdges", "chamferEdges", "shell"].contains(&name)) { continue; }
        assert!(super::inputs(&payload, &ids.iter().map(|id| id.replace("#0.", "#1.")).collect::<Vec<_>>()).is_err());
    }
    let ids: Vec<String> = serde_json::from_value(fixture["valid"][0]["ids"].clone()).unwrap();
    for case in fixture["invalid"].as_array().unwrap() {
        let payload = semio_framework_pack_json::from_json_str::<EditMeshSelection>(&case["payload"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject);
        assert!(payload.map_or(true, |payload| inputs(&payload, &ids).is_err()), "{}", case["name"]);
    }
}

#[test]
fn selected_brep_edits_insert_one_scoped_branch_and_preserve_consumers() {

    for (id, descriptor) in [("brep", include_str!("../../../../../../../../../../../../🌊️flow/🧩️extensions/📐️brep/🔣️.json")), ("list", include_str!("../../../../../../../../../../../../🌊️flow/🧩️extensions/📃️list/🔣️.json"))] {
        let descriptor: serde_json::Value = serde_json::from_str(descriptor).unwrap();
        let manifest = descriptor["manifest"]["topicContributions"][0]["payload"]["manifestJson"].as_str().unwrap();
        semio_framework_os_flow::install_flow_extension_manifest(id, manifest).unwrap();
    }
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures["analyticGraph"].as_array().unwrap() {
        let snapshot = semio_framework_artifact_flow_flow::FlowHostSnapshot::default();
        let prepared = with_host(&snapshot, |host| {
            let source = case["source"].as_str().unwrap();
            host.add_widget(&serde_json::json!({"kind":"neuron","id":source,"neuronKind":case["kind"]}).to_string(),0.0,0.0).unwrap();
            host.add_widget("{\"kind\":\"neuron\",\"id\":\"consumer\",\"neuronKind\":\"brep.brep\"}",500.0,0.0).unwrap();
            let channel = case["channel"].as_str().unwrap();
            host.connect_ports(source, channel, "consumer", "brep").unwrap();
            let ids = vec![format!("{source}@{channel}#{}.{}.1~{}~9007199254740993~{}",case["index"],case["granularity"].as_str().unwrap(),"a".repeat(64),"b".repeat(64))];
            let payload = EditMeshSelection { operation: case["operation"].as_str().unwrap().into(), ..Default::default() };
            edit_rows(&payload, &host.host_snapshot, &ids).map(|(feature, rows)| (feature, rows, host.host_snapshot.clone()))
        });
        snapshot.retire_cold();
        let (feature, rows, initial) = prepared.unwrap();
            assert!(rows.iter().any(|row| format!("{row:?}").contains(if case["granularity"] == "edge" { "edgeLabels" } else { "faceLabels" })));
            assert!(rows.iter().any(|row| format!("{row:?}").contains("sourceHandle")));
            assert!(feature.contains(case["operation"].as_str().unwrap()));
            let mut before = Generation3dSnapshot::default();
            std::mem::replace(&mut before.host_snapshot, initial).retire_cold();
            let before = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(before);
            let mut actual = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new((*before).clone());
            let mut inverses = Vec::new();
            for row in &rows {
                inverses.push(crate::standards::v1::subsets::any::schema::mutations::inverse_generation3d_mutation(&actual, row).unwrap());
                crate::standards::v1::subsets::any::schema::mutations::apply_generation3d_mutation(&mut actual, row).unwrap();
            }
            let consumer = actual.host_snapshot.synapses.iter().find(|wire| wire.to == "consumer").unwrap();
            if case["collection"] == true {
                assert!(actual.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == &consumer.from && neuron_kind == "list.set")));
            } else { assert_eq!(consumer.from, feature); }
            let selected = actual.host_snapshot.synapses.iter().find(|wire| wire.to == feature && wire.to_port == case["featureInput"].as_str().unwrap()).unwrap();
            assert_eq!(selected.from_port, case["selectedChannel"].as_str().unwrap());
            assert!(actual.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == &selected.from && neuron_kind == "brep.brep")));
            assert!(actual.host_snapshot.synapses.iter().any(|wire| wire.from == selected.from && wire.from_port == "brepOut" && wire.to == feature && wire.to_port == "geometry"));
            let Widget::Neuron { params, .. } = actual.host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == selected.from).unwrap() else { panic!("source selector") };
            assert_eq!(params.get("sourceHandle").and_then(|value| value.as_dictionary()).and_then(|value| value.get("value")).and_then(|value| value.as_atom()).and_then(|value| value.as_str()),Some("a".repeat(64).as_str()));
            if case["collection"] == true {
                assert!(actual.host_snapshot.synapses.iter().any(|wire| wire.from == selected.from && wire.from_port == "sourceIndex" && wire.to == consumer.from && wire.to_port == "index"));
                assert!(actual.host_snapshot.synapses.iter().any(|wire| wire.from == case["source"].as_str().unwrap() && wire.from_port == case["channel"].as_str().unwrap() && wire.to == consumer.from && wire.to_port == "list"));
                assert!(actual.host_snapshot.synapses.iter().any(|wire| wire.from == feature && wire.to == consumer.from && wire.to_port == "value"));
            }
            assert!(!actual.host_snapshot.synapses.iter().any(|wire| wire.from == feature && wire.to == selected.from));
            let committed = crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new((*actual).clone());
            for inverse in inverses.iter().rev() { for row in inverse { crate::standards::v1::subsets::any::schema::mutations::apply_generation3d_mutation(&mut actual, row).unwrap(); } }
            assert_eq!(actual, before);
            for row in &rows { crate::standards::v1::subsets::any::schema::mutations::apply_generation3d_mutation(&mut actual, row).unwrap(); }
            assert_eq!(actual, committed);
            for inverse in inverses { for row in inverse { row.retire_cold(); } }
            for row in rows { row.retire_cold(); }
    }
    println!("[DEBUG] Selected BRep mutation branches scalar=listLeaf labels=uint64 consumers=preserved");
}
