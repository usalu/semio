//! 🎯️ Verifies directional spatial channels through the public preview geometry API.
use semio_s_artifact_procedural_3d_preview_eval::*;
use semio_os_flow::{FlowEvalSession, neural::ColdRetire};
use semio_plugin::MeshData;
use std::collections::{BTreeMap, HashSet};

fn text(value: &serde_json::Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, found {value}"))
}

#[test]
fn plane_channel_preview_has_an_on_plane_outline_and_anchored_normal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧭️plane-marker.json")).unwrap();
    let half_extent = fixture["halfExtent"].as_f64().unwrap();
    let normal_length = fixture["normalLength"].as_f64().unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let origin: [f64; 3] = serde_json::from_value(row["origin"].clone()).unwrap();
        let normal: [f64; 3] = serde_json::from_value(row["normal"].clone()).unwrap();
        let [x,y,z] = origin;
        let [nx,ny,nz] = normal;
        let text = serde_json::json!({"widget":{"out":{"plane":{
            "$schema":"plane","origin":{"$schema":"point","x":x,"y":y,"z":z},
            "normal":{"$schema":"vector","x":nx,"y":ny,"z":nz}
        }}}}).to_string();
        let eval = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let items = preview_channel_items_for_widget(&eval, "widget");
        assert_eq!(items.len(), 1);
        let item = &items[0];
        assert_eq!(item.channel, "plane");
        assert_eq!(item.inline, Some(PreviewInlineGeometry::Plane { origin, normal }));
        let mesh = inline_preview_mesh(item.inline.as_ref().unwrap());
        assert_eq!(mesh.is_some(), row["valid"].as_bool().unwrap());
        let Some(mesh) = mesh else { continue };
        assert_eq!(preview_mesh_role(item.inline.as_ref(), &mesh), "wire");
        let norm = nx.hypot(ny).hypot(nz);
        let offsets = |point: &[f32]| std::array::from_fn::<_, 3, _>(|axis| f64::from(point[axis]) - origin[axis]);
        for segment in mesh.edge_positions[..24].chunks_exact(6) {
            let start = offsets(&segment[..3]);
            let end = offsets(&segment[3..]);
            let distance = |offset: [f64; 3]| (0..3).map(|axis| offset[axis] * normal[axis] / norm).sum::<f64>();
            assert!(distance(start).abs() < 1e-6);
            assert!(distance(end).abs() < 1e-6);
            let edge_length: f64 = (0..3).map(|axis| (start[axis] - end[axis]).powi(2)).sum::<f64>().sqrt();
            assert!((edge_length - 2.0 * half_extent).abs() < 1e-6);
        }
        let base = offsets(&mesh.edge_positions[24..27]);
        let tip = offsets(&mesh.edge_positions[27..30]);
        assert!(base.into_iter().all(|offset| offset.abs() < 1e-6));
        for axis in 0..3 { assert!((tip[axis] - normal_length * normal[axis] / norm).abs() < 1e-6); }
        assert!(mesh_has_preview_geometry(&apply_show_mode_mesh(mesh, "points")));
    }
}

#[test]
fn inline_spatial_markers_keep_renderer_coordinates_finite() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/marker-float32.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let [x, y, z]: [f64; 3] = serde_json::from_value(row["value"].clone()).unwrap();
        let inline = match text(&row["kind"]) {
            "point" => PreviewInlineGeometry::Point { x, y, z },
            "vector" => PreviewInlineGeometry::Vector { x, y, z },
            "plane" => PreviewInlineGeometry::Plane {
                origin: [x, y, z],
                normal: serde_json::from_value(row["normal"].clone()).unwrap(),
            },
            kind => panic!("unexpected kind: {kind}"),
        };
        let id = text(&row["id"]);
        let mesh = inline_preview_mesh(&inline).filter(mesh_has_preview_geometry).unwrap_or_default();
        assert_eq!(mesh.positions.len() / 3, row["positions"].as_u64().unwrap() as usize, "{id}");
        assert_eq!(mesh.edge_positions.len() / 6, row["segments"].as_u64().unwrap() as usize, "{id}");
        assert!(mesh.positions.iter().chain(&mesh.edge_positions).all(|value| value.is_finite()), "{id}");
    }
}

#[test]
fn preview_bounds_follow_the_emitted_vertex_and_edge_buffers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/painted-bounds.json")).unwrap();
    let coordinates = |lane: &serde_json::Value| -> Vec<f32> {
        lane.as_array().into_iter().flatten().map(|value| value.as_f64().map_or(f32::NAN, |value| value as f32)).collect()
    };
    for row in fixture["cases"].as_array().unwrap() {
        let mut bounds = PreviewMeshBounds::default();
        for mesh in row["meshes"].as_array().unwrap() {
            bounds.include(&MeshData { positions: coordinates(&mesh["positions"]), edge_positions: coordinates(&mesh["edgePositions"]), ..Default::default() });
        }
        let expected = serde_json::from_value::<Option<([f64; 3], [f64; 3])>>(row["bounds"].clone()).unwrap();
        assert_eq!(bounds.finish(), expected, "{}", text(&row["id"]));
    }
}

#[test]
fn preview_projections_share_the_same_evaluated_document() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/preview-evaluation.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let source = text(&row["text"]);
        let evaluation = PreviewEvaluation::from_json(source);
        let id = text(&row["id"]);
        assert!(std::ptr::eq(evaluation.source(), source), "{id}");
        assert_eq!(evaluation.geometry().is_some(), row["valid"].as_bool().unwrap(), "{id}");
        let status = preview_status_json(&evaluation, &Default::default()).and_then(|status| serde_json::from_str::<serde_json::Value>(&status).ok());
        let global = status.as_ref().and_then(|status| status.get("error")).and_then(serde_json::Value::as_str);
        assert_eq!(global, row["global"].as_str(), "{id}");
        let keys: Vec<_> = preview_value_rows(&evaluation).into_iter().map(|row| row.id).collect();
        assert_eq!(serde_json::json!(keys), row["keys"], "{id}");
    }
}

#[test]
fn viewport_channels_cover_both_directions_while_export_keeps_its_source_selection() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, SynapseSpec, Widget};
    use semio_framework_pack_json::{JsonMemberPolicy, parse};

    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/preview-channel-directions.json")).unwrap();
    let evaluation = parse(&law["evaluation"].to_string(), JsonMemberPolicy::Reject).unwrap();
    let mut host = FlowHostSnapshot::default();
    let mut pending = Vec::new();
    for row in law["rows"].as_array().unwrap() {
        let widget = row["widget"].as_str().unwrap();
        host.widgets.push(if let Some(variable) = row.get("variable") {
            Widget::Variable {
                id: widget.into(),
                name: variable["name"].as_str().unwrap().into(),
                schema: variable["schema"].as_str().unwrap().into(),
            }
        } else {
            Widget::Neuron {
                id: widget.into(),
                neuron_kind: "fixture.direction".into(),
                params: Default::default(),
                input_ports: Vec::new(),
                output_ports: Vec::new(),
                preview: true,
            }
        });
        let items = preview_viewport_items_for_widget(&evaluation, widget);
        let viewport: Vec<_> = items.iter().map(|item| {
            let address = item.address(widget);
            serde_json::json!({"port": address.port, "instance": address.instance, "handle": item.handle, "input": item.is_input()})
        }).collect();
        assert_eq!(serde_json::json!(viewport), row["viewport"]);
        let output: Vec<_> = preview_channel_items_for_widget(&evaluation, widget).into_iter().map(|item| item.handle).collect();
        assert_eq!(serde_json::json!(output), row["export"]);
        for mark in row["marks"].as_array().unwrap() {
            let marked_ids: Vec<_> = mark["ids"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect();
            let marked = PreviewGraphTargets::from_ids(&marked_ids);
            let highlighted: Vec<_> = items.iter().map(|item| item.address(widget)).filter(|address| address.marked(&marked)).map(|address| address.instance).collect();
            assert_eq!(serde_json::json!(highlighted), mark["instances"]);
        }
        pending.extend(items.into_iter().map(|item| item.handle));
    }
    let hidden = &law["exportOnly"];
    let hidden_id = hidden["widget"].as_str().unwrap();
    host.widgets.push(Widget::Neuron {
        id: hidden_id.into(), neuron_kind: "fixture.direction".into(), params: Default::default(),
        input_ports: Vec::new(), output_ports: Vec::new(), preview: false,
    });
    host.widgets.push(Widget::OutputExport { id: "export".into(), format: "gltf".into() });
    host.synapses.push(SynapseSpec {
        id: "to-export".into(), from: hidden_id.into(), from_port: hidden["channel"].as_str().unwrap().into(),
        to: "export".into(), to_port: "in".into(),
    });
    let session = FlowEvalSession::new();
    let exported = hidden["pending"].as_str().unwrap().to_owned();
    pending.push(exported);
    assert_eq!(pending_preview_tessellate_handles(&evaluation, &host, &session, 0.05), pending);
    let live = preview_mesh_retention_handles(&evaluation, &host, &session);
    let expected_live: HashSet<_> = pending.into_iter().chain(["face-unselected".into(), "wire-hidden-input".into()]).collect();
    assert_eq!(live, expected_live);
    session.retire_cold();
    host.retire_cold();
}

#[test]
fn viewport_hover_aliases_follow_coincident_retained_meshes() {
    use semio_framework_pack_json::{JsonMemberPolicy, Value, parse, to_string};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/preview-channel-directions.json")).unwrap();
    let placements: Vec<_> = fixture["rows"].as_array().unwrap().iter().flat_map(|row| {
        row["viewport"].as_array().unwrap().iter().map(|item| serde_json::json!({
            "id": item["instance"], "meshId": item["handle"], "interactionId": semio_s_artifact_procedural_3d_model::GraphTargetKind::Handle.target(item["port"].as_str().unwrap())
        }))
    }).collect();
    let mut instances: Vec<_> = placements.iter().map(|instance| parse(&instance.to_string(), JsonMemberPolicy::Reject).unwrap()).collect();
    attach_preview_hover_targets(&mut instances, "handle", &Default::default(), 1);
    let projected: serde_json::Value = serde_json::from_str(&to_string(&Value::Array(instances))).unwrap();
    for instance in projected.as_array().unwrap() {
        assert_eq!(instance["hoverTargets"], fixture["hoverTargetsByHandle"][instance["meshId"].as_str().unwrap()]);
    }
}


#[test]
fn exact_pick_geometry_groups_distinct_retained_meshes_without_replacing_their_ids() {
    use semio_framework_pack_json::{JsonMemberPolicy, Value, parse};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/preview-retained-overlap.json")).unwrap();
    let widget = fixture["widget"].as_str().unwrap();
    let meshes: std::collections::BTreeMap<String, MeshData> = fixture["meshes"].as_object().unwrap().iter().map(|(handle, row)| {
        (handle.clone(), MeshData {
            positions: serde_json::from_value(row["positions"].clone()).unwrap(),
            indices: serde_json::from_value(row["indices"].clone()).unwrap(),
            edge_positions: serde_json::from_value(row["edge_positions"].clone()).unwrap(),
            ..Default::default()
        })
    }).collect();
    let mut ids = PreviewMeshIds::default();
    let mut instances: Vec<_> = fixture["evaluation"][widget]["out"].as_object().unwrap().iter().map(|(channel, row)| {
        let handle = row["handle"].as_str().unwrap();
        ids.track_native_hover(&meshes[handle], handle);
        let address = PreviewChannelAddress::new(widget, channel, 0, false);
        parse(&serde_json::json!({"meshId": handle, "interactionId": address.interaction_id()}).to_string(), JsonMemberPolicy::Reject).unwrap()
    }).collect();
    attach_preview_hover_targets(&mut instances, "handle", &ids.into_hover_aliases(), 1);
    let expected: Vec<String> = serde_json::from_value(fixture["expected"]["group"].clone()).unwrap();
    let distinct: HashSet<_> = instances.iter().map(|instance| instance["meshId"].as_str().unwrap()).collect();
    assert_eq!(distinct.len() as u64, fixture["expected"]["meshes"].as_u64().unwrap());
    for instance in instances {
        let target = instance["interactionId"].as_str().unwrap();
        let addresses = instance["hoverTargets"].as_array().map(|group| group.iter().map(|row| row["id"].as_str().unwrap().to_owned()).collect::<Vec<_>>());
        if expected.iter().any(|address| address == target) {
            assert_eq!(addresses.as_ref(), Some(&expected), "{target}");
        } else {
            assert_eq!(instance["hoverTargets"], Value::Null, "{target}");
        }
    }
}

#[test]
fn inline_preview_meshes_use_exact_shape_and_component_data() {
    use semio_framework_pack_json::{JsonMemberPolicy, Value, parse};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/preview-inline-overlap.json")).unwrap();
    let evaluation = parse(&fixture["evaluation"].to_string(), JsonMemberPolicy::Reject).unwrap();
    let widget = fixture["widget"].as_str().unwrap();
    let mut meshes = PreviewMeshIds::default();
    let mut instances = Vec::new();
    let mut distinct = 0;
    for item in preview_viewport_items_for_widget(&evaluation, widget) {
        let address = item.address(widget);
        let mesh_id = meshes.get(None, item.inline.as_ref()).cloned().unwrap_or_else(|| {
            distinct += 1;
            meshes.insert(None, item.inline.as_ref(), address.instance.clone());
            address.instance.clone()
        });
        let instance = serde_json::json!({"id": address.instance, "meshId": mesh_id, "interactionId": address.interaction_id()});
        instances.push(parse(&instance.to_string(), JsonMemberPolicy::Reject).unwrap());
    }
    assert_eq!(serde_json::json!(distinct), fixture["distinctMeshes"]);
    assert_eq!(serde_json::json!(instances.len()), fixture["instances"]);
    attach_preview_hover_targets(&mut instances, "handle", &Default::default(), 1);
    for instance in instances {
        let id = instance["interactionId"].as_str().unwrap();
        let expected = fixture["groups"].as_array().unwrap().iter().find(|group| group.as_array().unwrap().iter().any(|member| member.as_str() == Some(id)));
        let targets = instance.get("hoverTargets").and_then(Value::as_array).map(|rows| {
            serde_json::json!(rows.iter().map(|row| row["id"].as_str().unwrap()).collect::<Vec<_>>())
        });
        assert_eq!(targets.as_ref(), expected);
    }
}

#[test]
fn graph_wires_keep_delimited_authored_widget_and_channel_ids() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, SynapseSpec};
    use semio_os_infinite_directed_presentation::{DagHostSnapshot, DagHostSnapshotEdge};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🔌️graph-ports/🔣️.json")).unwrap();
    for row in fixture["wireEndpoints"].as_array().unwrap() {
        let text = |field: &str| row[field].as_str().unwrap().to_owned();
        let mut host = FlowHostSnapshot::default();
        host.synapses.push(SynapseSpec {
            id: "wire".into(), from: text("sourceNode"), from_port: text("sourceChannel"),
            to: text("targetNode"), to_port: text("targetChannel"),
        });
        let dag = DagHostSnapshot { edges: vec![DagHostSnapshotEdge {
            id: "wire".into(), source: text("sourceEndpoint"), target: text("targetEndpoint"),
            route_style: Default::default(), properties: Default::default(),
        }], ..Default::default() };
        let (_, edges) = dag_host_snapshot_to_workflow(&dag, &host.synapses);
        let [edge] = edges.as_slice() else { panic!("expected one projected wire"); };
        assert_eq!(edge.source_node_id, text("sourceNode"));
        assert_eq!(edge.target_node_id, text("targetNode"));
        assert_eq!(edge.source_port_id, text("sourceCode"));
        assert_eq!(edge.target_port_id, text("targetCode"));
        host.retire_cold();
    }
}

#[test]
fn equal_authored_node_port_and_wire_ids_keep_separate_hover_closures() {
    use semio_framework_artifact_flow_flow::{FlowHostSnapshot, SynapseSpec, Widget};
    use semio_s_artifact_procedural_3d_model::{graph_port_id, GraphTargetKind::{self, Edge, Handle, Node}};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️schema/🔌️graph-ports/🔣️.json")).unwrap();
    let case = &fixture["interactionCollision"];
    let raw = case["raw"].as_str().unwrap();
    let widget = case["widget"].as_str().unwrap();
    let channel = case["channel"].as_str().unwrap();
    let wire = &case["wire"];
    let string = |key: &str| wire[key].as_str().unwrap().to_owned();
    let mut host = FlowHostSnapshot { widgets: Vec::new(), synapses: Vec::new(), ..Default::default() };
    for id in [raw, widget] {
        host.widgets.push(Widget::Neuron { id: id.into(), neuron_kind: "fixture.box".into(), params: Default::default(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true });
    }
    host.synapses.push(SynapseSpec { id: raw.into(), from: string("source"), from_port: string("sourceChannel"), to: string("target"), to_port: string("targetChannel") });
    let port = graph_port_id(widget, channel, false);
    assert_eq!(port, raw);
    let other_port = graph_port_id(raw, channel, false);
    let topology = graph_interaction_topology(&host, &BTreeMap::from([(widget.to_owned(), vec![port.clone()]), (raw.to_owned(), vec![other_port.clone()])]));
    let graph = topology.domains.get("graph").unwrap();
    let targets = &graph.ordered;
    assert_eq!(targets.len(), 5);
    for (node, child) in [(widget, port.as_str()), (raw, other_port.as_str())] {
        let root = Node.target(node);
        let expected = vec![root.clone(), Handle.target(child)];
        let mut closure = std::pin::pin!(graph.descendant_closure(&root));
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        assert_eq!(std::future::Future::poll(closure.as_mut(), &mut cx), std::task::Poll::Ready(expected));
    }
    for row in case["targets"].as_array().unwrap() {
        let kind = GraphTargetKind::for_granularity(row["granularity"].as_str().unwrap()).unwrap();
        let target = row["target"].as_str().unwrap();
        let topology_target = targets.iter().find(|node| node.id == target).unwrap();
        assert_eq!(topology_target.granularity, kind.granularity());
        assert_eq!(topology_target.parent, (kind == Handle).then(|| Node.target(widget)));

        let ids = [target.to_owned()];
        let marks = PreviewGraphTargets::from_ids(&ids);
        let a = PreviewChannelAddress::new(widget, channel, 0, false);
        let shifted = PreviewChannelAddress::new(raw, channel, 0, false);
        assert_eq!(a.marked(&marks), kind == Handle);
        assert_eq!(shifted.marked(&marks), kind == Node);
        let wire_hover = preview_wire_hover(&host, &marks);
        let wire = &host.synapses[0];
        assert_eq!(wire_hover.matches(&wire.from, &graph_port_id(&wire.from, &wire.from_port, false), false), kind == Edge);
        assert_eq!(wire_hover.matches(&wire.to, &graph_port_id(&wire.to, &wire.to_port, true), true), kind == Edge);
    }
    host.retire_cold();
}

#[test]
fn oversized_viewport_hover_keeps_all_directional_targets() {
    use semio_framework_pack_json::{Object, Value, to_string};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/preview-large-hover-group.json")).unwrap();
    let count = fixture["count"].as_u64().unwrap() as usize;
    let width = fixture["widgetBytes"].as_u64().unwrap() as usize;
    let capacity = fixture["maximumActionBytes"].as_u64().unwrap() as usize;
    let channel = fixture["channel"].as_str().unwrap();
    let mut expected = Vec::new();
    let mut instances: Vec<_> = (0..count).map(|index| {
        let widget = format!("copy-{index:03}-{}", "q".repeat(width - 9));
        let address = PreviewChannelAddress::new(&widget, channel, 0, index % 2 == 0);
        let target = address.interaction_id();
        assert!(target.len() <= 256);
        expected.push(target.clone());
        let mut instance = Object::new();
        instance.insert("id", Value::String(address.instance));
        instance.insert("meshId", Value::String(String::from("coincident")));
        instance.insert("interactionId", Value::String(target));
        Value::Object(instance)
    }).collect();
    expected.sort();
    assert!(expected.iter().map(String::len).sum::<usize>() > capacity);
    let groups = attach_preview_hover_targets(&mut instances, "handle", &Default::default(), 10);
    let wire_targets = &instances[0]["hoverTargets"];
    assert_eq!(wire_targets.as_array().unwrap().len(), 1);
    assert!(to_string(wire_targets).len() < capacity);
    let id = wire_targets[0]["id"].as_str().unwrap().to_owned();
    assert!(instances.iter().all(|row| &row["hoverTargets"] == wire_targets));
    let mut publications = PreviewHoverPublications::default();
    publications.publish("world-preview", groups);
    assert_eq!(publications.targets("world-preview", &id), Some(expected.as_slice()));
    assert!(publications.targets("different-window", &id).is_none());
    publications.retain_windows(|_| false);
    assert!(publications.targets("world-preview", &id).is_none());
}
