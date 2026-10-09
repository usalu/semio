# Board Snapshot Field Input Audit Current

Read-only exact existing semantic receiver field lookups. Layout and fixture syntax remain live-changing; keys are admissions to validate, not proposed blind allowance.

## Normal Snapshot Projection
- `obj` direct keys: `handles, height, hidden, iconKind, id, locked, nodeKind, node_kind, radius, root, scale, shape, text, visible, width, x, y`.
- `ho` direct keys: `angle, color, handleKind, hidden, iconKind, id, locked, radius, scale, visible`.
- `e` edge direct keys: `edgeKind, edge_kind, hidden, id, locked, source, sourceTip, source_tip, target, targetTip, target_tip, visible`.
- `o` region direct keys: `id, label, hidden, locked, selected`; numeric x/y/width/height are read through the keyed finite-number closure.
- `entry` direct keys: ``.

```rust
fn board_snapshot_scene_descriptor(f: BoardSnapshot, has_ports: bool) -> Option<SceneDescriptor> {
            let mut desc = SceneDescriptor::default();
            for entry in f.nodes {
                let obj = &entry;
                let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
                    return None;
                };
                let Some(x) = obj.get("x").and_then(|v| v.as_f64()) else {
                    return None;
                };
                let Some(y) = obj.get("y").and_then(|v| v.as_f64()) else {
                    return None;
                };
                if !x.is_finite() || !y.is_finite() {
                    return None;
                }
                let text = obj.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                if has_ports {
                    let Some(handles_arr) = obj.get("handles").and_then(|v| v.as_array()) else {
                        return None;
                    };
                    let mut handles: Vec<HandleDescriptor> = Vec::new();
                    for h in handles_arr {
                        if h.as_object().is_none() { return None; }
                        let ho=h;
                        let Some(hid) = ho.get("id").and_then(|v| v.as_str()) else {
                            return None;
                        };
                        let Some(angle) = ho.get("angle").and_then(|v| v.as_f64()) else {
                            return None;
                        };
                        if !angle.is_finite() {
                            return None;
                        }
                        let handle_kind = ho.get("handleKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map_or_else(|| "port".into(), String::from);
                        let handle_color = ho.get("color").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                        let handle_icon_kind = ho.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                        let handle_scale = ho.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                        let handle_radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                        handles.push(HandleDescriptor {
                            id: hid.into(),
                            node_id: id.into(),
                            angle,
                            radius: handle_radius,
                            scale: handle_scale,
                            selected: None,
                            style: None,
                            handle_kind: Some(handle_kind),
                            color: handle_color,
                            icon_kind: handle_icon_kind,
                            user_data: None,
                            visible: board_visible_option(&crate::infinite::board::BoardVisibility {hidden:ho.get("hidden").and_then(|v|v.as_bool()),visible:ho.get("visible").and_then(|v|v.as_bool()),locked:ho.get("locked").and_then(|v|v.as_bool())}),
                            locked: board_locked_option(&crate::infinite::board::BoardVisibility {hidden:ho.get("hidden").and_then(|v|v.as_bool()),visible:ho.get("visible").and_then(|v|v.as_bool()),locked:ho.get("locked").and_then(|v|v.as_bool())}),
                        });
                    }
                    desc.handles.extend(handles);
                } else if obj.get("handles").is_some() {
                    return None;
                }
                let shape_str = obj.get("shape").and_then(|v| v.as_str());
                let snapshot_node_kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let snapshot_node_scale = obj.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                if shape_str == Some("rectangle") {
                    let Some(width) = obj.get("width").and_then(|v| v.as_f64()) else {
                        return None;
                    };
                    let Some(height) = obj.get("height").and_then(|v| v.as_f64()) else {
                        return None;
                    };
                    if width <= 0.0 || height <= 0.0 {
                        return None;
                    }
                    let root = obj.get("root").and_then(|v| v.as_bool());
                    let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    desc.nodes.push(NodeDescriptor {
                        id: id.into(),
                        x,
                        y,
                        draggable: None,
                        selected: None,
                        style: None,
                        text,
                        icon_kind,
                        node_kind: snapshot_node_kind.clone(),
                        user_data: None,
                        visible: board_visible_option(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}),
                        locked: board_locked_option(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}),
                        root,
                        shape: Some("rectangle".into()),
                        radius: None,
                        width: Some(width),
                        height: Some(height),
                        scale: snapshot_node_scale,
                    });
                } else {
                    let Some(radius) = obj.get("radius").and_then(|v| v.as_f64()) else {
                        return None;
                    };
                    if radius <= 0.0 {
                        return None;
                    }
                    let root = obj.get("root").and_then(|v| v.as_bool());
                    let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    desc.nodes.push(NodeDescriptor {
                        id: id.into(),
                        x,
                        y,
                        draggable: None,
                        selected: None,
                        style: None,
                        text,
                        icon_kind,
                        node_kind: snapshot_node_kind.clone(),
                        user_data: None,
                        visible: board_visible_option(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}),
                        locked: board_locked_option(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}),
                        root,
                        shape: Some("circle".into()),
                        radius: Some(radius),
                        width: None,
                        height: None,
                        scale: snapshot_node_scale,
                    });
                }
            }
            for entry in f.edges {
                let e = &entry;
                let Some(id) = e.get("id").and_then(|v| v.as_str()) else {
                    return None;
                };
                let Some((source, target)) = board_edge_handle_ids(e.get("source").and_then(|v|v.as_str()),e.get("target").and_then(|v|v.as_str())) else {
                    return None;
                };
                if !has_ports {
                    let node_ids: BTreeSet<&str> = desc.nodes.iter().map(|n| n.id.as_str()).collect();
                    if !node_ids.contains(source) || !node_ids.contains(target) {
                        return None;
                    }
                }
                let edge_kind = e.get("edgeKind").or_else(|| e.get("edge_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let source_tip = e.get("sourceTip").or_else(|| e.get("source_tip")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let target_tip = e.get("targetTip").or_else(|| e.get("target_tip")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                desc.edges.push(EdgeDescriptor {
                    id: id.into(),
                    source: source.into(),
                    target: target.into(),
                    edge_kind,
                    source_tip,
                    target_tip,
                    selected: None,
                    style: None,
                    user_data: None,
                    visible: board_visible_option(&crate::infinite::board::BoardVisibility {hidden:e.get("hidden").and_then(|v|v.as_bool()),visible:e.get("visible").and_then(|v|v.as_bool()),locked:e.get("locked").and_then(|v|v.as_bool())}),
                    locked: board_locked_option(&crate::infinite::board::BoardVisibility {hidden:e.get("hidden").and_then(|v|v.as_bool()),visible:e.get("visible").and_then(|v|v.as_bool()),locked:e.get("locked").and_then(|v|v.as_bool())}),
                });
            }
            for entry in f.target_regions {
                let o = &entry;
                let Some(id) = o.get("id").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) else {
                    continue;
                };
                let read = |key: &str| o.get(key).and_then(|v| v.as_f64()).filter(|v| v.is_finite()).unwrap_or(0.0);
                desc.regions.push(RegionDescriptor {
                    id: id.into(),
                    x: read("x"),
                    y: read("y"),
                    width: read("width"),
                    height: read("height"),
                    label: o.get("label").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string()),
                    hidden: o.get("hidden").and_then(|v| v.as_bool()),
                    locked: o.get("locked").and_then(|v| v.as_bool()),
                    selected: o.get("selected").and_then(|v| v.as_bool()),
                });
            }
            Some(desc)
        }
```

## Retained Fixture And Puzzle Command Evidence

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:24` — `source, target` — `"edges": [{ "id": "e1", "source": "a", "target": "b" }]`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:54` — `source, target` — `"edges": [{ "id": "e1", "source": "a", "target": "b" }]`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:374` — `shape` — `{ "id": node_id, "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0 },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:375` — `shape` — `{ "id": "node-b", "x": 20.0, "y": 0.0, "shape": "circle", "radius": 10.0 }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:377` — `source, target` — `"edges": [{ "id": "edge-a-b", "source": node_id, "target": "node-b" }]`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:771` — `handleKind` — `let handles: Vec<serde_json::Value> = (0..handles_per_node).map(|handle| serde_json::json!({ "id": format!("node-{index}:v{handle}"), "handleKind": "b-l", "angle": handle as f64 * 0.5, "radius": 3.0 })).collect();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:772` — `handles, shape` — `serde_json::json!({ "id": format!("node-{index}"), "x": index as f64 * 60.0, "y": 0.0, "shape": "circle", "radius": 24.0, "handles": handles })`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:792` — `handleKind` — `let handles: Vec<serde_json::Value> = (0..11).map(|handle| serde_json::json!({ "id": format!("node-{index}:v{handle}"), "handleKind": "b-l", "angle": handle as f64 * 0.5, "radius": 3.0 })).collect();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:793` — `handles, shape` — `serde_json::json!({ "id": format!("node-{index}"), "x": index as f64 * 60.0, "y": 0.0, "shape": "circle", "radius": 24.0, "handles": handles })`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:796` — `source, target` — `let edges: Vec<serde_json::Value> = (0..nodes.saturating_sub(1)).map(|index| serde_json::json!({ "id": format!("edge-{index}"), "source": format!("node-{index}:v0"), "target": format!("node-{}:v1", index + 1) })).collect();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:840` — `handleKind, handles, shape` — `"nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:867` — `handleKind, handles, shape` — `"nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:874` — `shape` — `serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "circle" }], "edges": [] }),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:875` — `color, handleKind, handles, shape` — `serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-z:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0, "color": "not-a-color" }] }], "edges": [] }),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:876` — `shape` — `serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-z", "x": 1.0, "y": 1.0, "shape": "rectangle", "width": 0.0, "height": 4.0 }], "edges": [] }),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:894` — `handleKind, handles, shape` — `.map(|index| serde_json::json!({ "id": format!("node-{index}"), "x": index as f64 * 60.0, "y": 0.0, "shape": "circle", "radius": 24.0, "handles": [{ "id": format!("node-{index}:a"), "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }, { "id": format!("node-{index}:b"), "handleKind": "b-l", "angle": 3.0, "radius": 3.0 }] }))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:896` — `source, target` — `let edge_rows: Vec<serde_json::Value> = (0..edges).map(|index| serde_json::json!({ "id": format!("edge-{index}"), "source": format!("node-{index}:b"), "target": format!("node-{}:a", index + 1) })).collect();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:914` — `handleKind, handles, shape` — `serde_json::json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "locked": locked, "handles": [{ "id": format!("{id}:v0"), "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] })`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1104` — `handleKind, handles, shape` — `{ "id": "node-a", "x": -40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1105` — `handleKind, handles, shape` — `{ "id": "node-b", "x": 40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-b:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1106` — `handleKind, handles, shape` — `{ "id": "node-under", "x": pivot.x + radius, "y": pivot.y, "shape": "circle", "radius": 24.0, "handles": [{ "id": "node-under:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1225` — `handleKind, handles, shape` — `"nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] }],`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1227` — `targetRegions` — `"targetRegions": [`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1230` — `hidden` — `{ "id": "region-hidden", "x": -300.0, "y": -40.0, "width": 80.0, "height": 80.0, "hidden": true }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1456` — `handles, shape, targetRegions` — `let overflowing = serde_json::json!({ "schema": "board.ports.directed.v1", "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 }, "nodes": [{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [] }], "edges": [], "targetRegions": rows }).to_string();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1501` — `handles, iconKind, shape` — `{ "id": "node-a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 36.0, "iconKind": icon, "handles": [] },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1502` — `handles, iconKind, shape` — `{ "id": "node-b", "x": 400.0, "y": 0.0, "shape": "circle", "radius": 36.0, "iconKind": icon, "handles": [] }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1546` — `handleKind, handles, shape` — `{ "id": "a", "x": 0.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [{ "id": "a:h", "handleKind": "door", "angle": 0.0, "radius": 3.0 }] },`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1547` — `handleKind, handles, shape` — `{ "id": "b", "x": 200.0, "y": 0.0, "shape": "circle", "radius": 20.0, "handles": [{ "id": "b:h", "handleKind": "door", "angle": 3.141592653589793, "radius": 3.0 }] }`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1549` — `source, target` — `"edges": [{ "id": "e", "source": "a:h", "target": "b:h" }]`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1567` — `handles, shape` — `let node = |id: &str, x: f64, locked: bool| serde_json::json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "locked": locked, "handles": [] });`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1713` — `handles, shape` — `let node = |id: &str, x: f64| serde_json::json!({ "id": id, "x": x, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [] });`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:9` — `hidden, label, shape, source, target, targetRegions, visible` — `r#"{"schema":"puzzle.2d","camera":{"x":0,"y":0,"zoom":1},"nodes":[{"id":"a","shape":"circle","x":0,"y":0,"radius":20,"text":"root"},{"id":"b","shape":"rectangle","x":120,"y":40,"width":60,"height":30},{"id":"ghost","x":500,"y":500,"visible":false}],"edges":[{"id":"e","source":"a","target":"b"}],"targetRegions":[{"id":"t","x":-40,"y":-40,"width":240,"height":120,"label":"goal","hidden":false,"locked":false}],"meta":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:12` — `handles` — `let before = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n1", "anchor": "fixed", "x": 0.0, "y": 0.0, "handles": [] }, { "id": "n2", "anchor": "fixed", "x": 10.0, "y": 0.0, "handles": [] }], "edges": [] });`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:13` — `handles` — `let after = json!({ "schema": PUZZLE_2D_SCHEMA, "nodes": [{ "id": "n2", "anchor": "fixed", "x": 99.0, "y": 0.0, "handles": [] }, { "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }], "edges": [] });`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:16` — `handles` — `let created: crate::Puzzle2dNode=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&json!({ "id": "n3", "anchor": "fixed", "x": 1.0, "y": 0.0, "handles": [] }))).expect("node admits");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:35` — `handles, nodeKind, shape` — `let node: crate::Puzzle2dNode=semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(&json!({ "id": "n1", "nodeKind": "seed", "shape": "circle", "x": 0.0, "y": 0.0, "text": "n1", "handles": [], "radius": 24.0 }))).expect("sparse node admits");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-handles/↩️inverse/📑️ordered-restoration/🧪️tests/🦀️.rs:12` — `handles` — `let node: Puzzle2dNode = serde_json::from_value(json!({"id":"node","anchor":"fixed","x":3.0,"y":-4.0,"handles":[{"id":"retained-handle","angle":1.0},{"id":"handle","angle":2.0}],"text":"😀\u{0}"})).expect("native node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-handles/↩️inverse/📑️ordered-restoration/🧪️tests/🦀️.rs:46` — `source, target` — `base.edges.push(serde_json::from_value(json!({"id":id,"source":"outside","target":"outside"})).expect("native retained edge"));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️connect-handles/↩️inverse/📑️ordered-restoration/🧪️tests/🦀️.rs:48` — `edgeKind, source, target` — `let mutation: Puzzle2dMutation = serde_json::from_value(json!({"mutation":"connectHandles","id":"new","source":"handle","target":"outside","edgeKind":null,"gap":0.0,"shift":0.0,"rise":0.0,"rotation":0.0,"turn":0.0,"tilt":0.0,"x":0.0,"y":0.0,"sourceTip":null,"targetTip":null,"index":row["index"]})).expect("native insertion payload");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:19` — `source, target` — `"meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:21` — `handleKind, handles, nodeKind, shape` — `{ "id": "a", "nodeKind": "k", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "A", "handles": [{ "id": "a:v0", "handleKind": "a", "angle": 0.0 }, { "id": "a:v1", "handleKind": "a", "angle": 3.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:22` — `handleKind, handles, nodeKind, shape` — `{ "id": "b", "nodeKind": "k", "shape": "circle", "x": 100.0, "y": 0.0, "radius": 24.0, "text": "B", "handles": [{ "id": "b:v0", "handleKind": "a", "angle": 0.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:23` — `handles, nodeKind, shape` — `{ "id": "c", "nodeKind": "k", "shape": "circle", "x": 200.0, "y": 0.0, "radius": 24.0, "text": "C", "handles": [] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:24` — `handleKind, handles, nodeKind, shape` — `{ "id": "d", "nodeKind": "k", "shape": "circle", "x": 300.0, "y": 0.0, "radius": 24.0, "text": "D", "handles": [{ "id": "d:v0", "handleKind": "a", "angle": 0.0 }, { "id": "d:v1", "handleKind": "a", "angle": 1.0 }] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:26` — `source, target` — `"edges": [{ "id": "e1", "source": "a:v1", "target": "b:v0" }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:27` — `hidden, targetRegions` — `"targetRegions": [{ "id": "r1", "x": 0.0, "y": 0.0, "width": 10.0, "height": 10.0, "hidden": false, "locked": false }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:206` — `handleKind, handles, nodeKind` — `let payload = json!({ "nodeId": "placed", "edgeId": "placed-link", "nodeKind": "k", "x": 40.0, "y": 50.0, "sourceHandleId": "d:v0", "targetHandleIndex": 0, "handles": [{ "handleKind": "a", "angle": 0.0 }] });`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs:220` — `source, target` — `("edgeCreate", json!({ "id": "e2", "source": "a:v0", "target": "d:v0" }), |row| matches!(row, Puzzle2dMutation::ConnectHandles(_))),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:57` — `nodeKind` — `let node_values: Vec<Value> = (0..nodes).map(|index| semio_framework_pack_json::json!({ "id": format!("node-{index}"), "text": format!("Node {index}"), "nodeKind": "capsule", "x": index as f64, "y": 0.0 })).collect();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:58` — `edgeKind, source, target` — `let edge_values: Vec<Value> = (0..edges).map(|index| semio_framework_pack_json::json!({ "id": format!("edge-{index}"), "source": format!("node-{index}"), "target": format!("node-{}", index + 1), "edgeKind": "link" })).collect();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:27` — `nodeKind` — `let nodes: Vec<Value> = (0..ids).map(|index| semio_framework_pack_json::json!({ "id": format!("node-{index}"), "text": format!("Node {index}"), "nodeKind": "capsule" })).collect();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs:30` — `handles` — `board_snapshot: json!({ "schema": crate::editor::puzzle2d::PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": [], "edges": [], "meta": { "kindCatalogs": { "nodes": rows, "handles": [], "edges": [] } } }),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️history-edit-runtime/🦀️.rs:181` — `target` — `let reported: Vec<Value> = rows.iter().zip(&log).flat_map(|(row, (index, _))| row.mutations.iter().flat_map(|mutation| &mutation.messages).map(|message| json!({ "index": index, "code": message.code, "target": message.target })).collect::<Vec<_>>()).collect();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:76` — `kind` — `("drag", json!({ "gestureId": "gesture-1", "kind": "drag", "targets": ["SEED"], "dx": 120.0, "dy": 240.0, "proximity": [] })),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:77` — `kind` — `("rotate", json!({ "gestureId": "gesture-2", "kind": "rotate", "targets": ["SEED"], "pivotX": 0.0, "pivotY": 0.0, "angle": 0.75, "proximity": [] })),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:93` — `kind` — `let rows: Vec<Value> = (0..3).map(|step| json!({ "name": "gesture", "payload": { "gestureId": format!("gesture-{step}"), "kind": "drag", "targets": [id], "dx": 10.0, "dy": 0.0, "proximity": [] } })).collect();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:122` — `handles` — `{ "id": "free", "x": 0.0, "y": 0.0, "handles": [{ "id": "free:link", "angle": 0.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:123` — `handles` — `{ "id": "bound", "x": 0.0, "y": 0.0, "locked": true, "handles": [] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:124` — `handles` — `{ "id": "host", "x": 0.0, "y": 0.0, "handles": [{ "id": "host:link", "angle": 0.0, "locked": true }] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:126` — `source, target` — `"edges": [{ "id": "wire", "source": "free:link", "target": "host:link", "locked": true }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️locks/🦀️.rs:127` — `targetRegions` — `"targetRegions": [{ "id": "zone", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0, "locked": true }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:460` — `kind` — `let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:529` — `kind` — `dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:543` — `kind` — `dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:570` — `kind` — `match dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None) {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:589` — `kind` — `dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:826` — `kind` — `let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:953` — `kind` — `dispatch(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None).expect("a adds node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:954` — `kind` — `dispatch(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None).expect("b adds node");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:973` — `kind` — `dispatch(&mut sender, "addNode", Some(&json!({ "kind": "seed" })), None).expect("add");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1108` — `kind` — `let events = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": [id], "dx": 8.0, "dy": 4.0, "proximity": [] } }]).to_string();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1230` — `nodeKind` — `brush_candidates: vec![semio_framework_value::ToValue::to_value(&(&json!({ "nodeKind": "alpha", "targetHandleIndex": 0 }))), semio_framework_value::ToValue::to_value(&(&json!({ "nodeKind": "beta", "targetHandleIndex": 2 })))],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1363` — `nodeKind` — `Some(label) => json!({ "id": id, "nodeKind": kind, "text": label, "x": 0.0, "y": 0.0 }),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1364` — `nodeKind` — `None => json!({ "id": id, "nodeKind": kind, "x": 0.0, "y": 0.0 }),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1367` — `label` — `json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": nodes, "edges": [], "meta": { "kindCatalogs": { "nodes": [{ "id": "capsule", "name": "Capsule", "label": "Capsule", "description": "", "icon": "", "image": "", "unit": "" }] } } })`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1442` — `handles` — `let mut recorder = recorder_of(json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": [{ "id": "a", "x": 1.0, "y": 0.0, "handles": [{ "id": "a:v0", "angle": 0.0 }, { "id": "a:v1", "angle": 1.0 }] }], "edges": [] }));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1475` — `kind` — `let result = dispatch(&mut app, "setSelectableKind", Some(&json!({ "kind": kind })), Some(overview::WINDOW_KIND_ID)).unwrap_or_else(|error| panic!("setSelectableKind {kind} must not fault: {error:?}"));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1598` — `kind` — `let record = |angle: f64, targets: Value| json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "rotate", "targets": targets, "pivotX": x0, "pivotY": y0 + 10.0, "angle": angle, "proximity": [] } }]).to_string();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1620` — `handleKind, handles, shape` — `{ "id": "node-a", "x": -40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-a:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1621` — `handleKind, handles, shape` — `{ "id": "node-b", "x": 40.0, "y": 0.0, "shape": "circle", "radius": 10.0, "handles": [{ "id": "node-b:v0", "handleKind": "b-l", "angle": 0.0, "radius": 3.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1622` — `handles, shape` — `{ "id": "node-locked", "x": 0.0, "y": 0.0, "locked": true, "shape": "circle", "radius": 10.0, "handles": [] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1665` — `kind` — `let moved = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": [id.clone()], "dx": 121.0, "dy": 211.0, "proximity": [] } }]).to_string();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1678` — `kind` — `let unknown = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-2", "kind": "drag", "targets": ["never-painted"], "dx": 1.5, "dy": 1.5, "proximity": [] } }]).to_string();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-history/🦀️.rs:402` — `handleKind, handles` — `{ "id": "a", "x": 0.0, "y": 0.0, "text": "Alpha", "handles": [{ "id": "a:h", "handleKind": "door" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-history/🦀️.rs:403` — `handleKind, handles` — `{ "id": "b", "x": 0.0, "y": 0.0, "handles": [{ "id": "b:h", "handleKind": "door" }] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-history/🦀️.rs:405` — `source, target` — `"edges": [{ "id": "e", "source": "a:h", "target": "b:h" }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-history/🦀️.rs:406` — `label, targetRegions` — `"targetRegions": [{ "id": "r", "label": "Courtyard" }, { "id": "s" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️board-tools/🦀️.rs:113` — `hidden` — `board["targetRegions"] = json!([{ "id": "region-1", "x": 0.5, "y": 0.5, "width": 10.5, "height": 10.5, "hidden": false, "locked": false }]);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️board-tools/🦀️.rs:114` — `source, target` — `board["edges"] = json!([{ "id": "edge-left-mid", "source": "left:v0", "target": "mid:v0" }]);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:18` — `source, target` — `"meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:20` — `handleKind, handles, shape` — `{ "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "left:v0", "handleKind": "a", "angle": 0.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:21` — `handleKind, handles, shape` — `{ "id": "right", "shape": "circle", "x": 1000.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "right:v0", "handleKind": "a", "angle": std::f64::consts::PI }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:22` — `handles, shape` — `{ "id": "locked", "shape": "circle", "x": 500.0, "y": 500.0, "radius": 24.0, "locked": true, "handles": [] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:25` — `hidden, targetRegions` — `"targetRegions": [{ "id": "region-1", "x": 10.0, "y": 10.0, "width": 20.0, "height": 20.0, "hidden": false, "locked": false }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:48` — `kind, source, target` — `let drag = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "gesture-1", "kind": "drag", "targets": ["left"], "dx": 8.0, "dy": -4.0, "proximity": [{ "source": "left:v0", "target": "right:v0" }] })).expect("drag decodes");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:50` — `kind` — `let rotate = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "gesture-2", "kind": "rotate", "targets": ["left"], "pivotX": 1.0, "pivotY": 2.0, "angle": 0.5, "proximity": [] })).expect("rotate decodes");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:52` — `kind` — `let scale = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "gesture-3", "kind": "scale", "targets": ["left"], "pivotX": 1.0, "pivotY": 2.0, "factor": 2.0, "proximity": [] })).expect("scale decodes");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:54` — `kind` — `for malformed in [json!({ "kind": "drag", "targets": ["left"], "dx": 1.0 }), json!({ "kind": "spin", "targets": ["left"] }), json!({ "kind": "drag", "dx": 1.0, "dy": 1.0 }), json!({ "kind": "drag", "targets": ["left"], "dx": "far", "dy": 1.0 })] {`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:231` — `kind` — `let decoded = Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "g", "kind": "drag", "targets": ["left", "left"], "dx": 1.0, "dy": 0.0, "proximity": [] })).expect("decodes");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:238` — `kind` — `assert_eq!(Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "g", "kind": "drag", "targets": [], "dx": 1.0, "dy": 0.0, "proximity": [] })), None, "a target-less record is malformed");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs:240` — `kind` — `assert_eq!(Puzzle2dSelectionRecord::from_gesture(&json!({ "gestureId": "g", "kind": "scale", "targets": ["left"], "pivotX": 0.0, "pivotY": 0.0, "factor": factor, "proximity": [] })), None, "factor {factor} is malformed");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs:362` — `kind` — `let result = dispatch(&mut app, "hostEvent", Some(&json!({ "windowId": overview::WINDOW_KIND_ID, "kind": kind })), Some(overview::WINDOW_KIND_ID)).expect("the forwarded host event");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs:424` — `handles, shape` — `"nodes": [{ "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [] }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs:450` — `handles, shape` — `"nodes": [{ "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [] }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs:455` — `kind` — `{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": ["left"], "dx": 3.0, "dy": 4.0, "proximity": [] } }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:262` — `hidden` — `document["targetRegions"] = json!([{ "id": "region-1", "x": bounds[0], "y": bounds[1], "width": half * 2.0, "height": half * 2.0, "hidden": hidden, "locked": false }]);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:465` — `handles, shape` — `head["nodes"].as_array_mut().expect("nodes").push(json!({ "id": "intruder", "shape": "circle", "x": create.node.x, "y": create.node.y, "radius": 1.0, "handles": [] }));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:24` — `nodeKind` — `let brush_runtime = Puzzle2dPlayRuntime { brush_candidates: vec![json!({ "nodeKind": "node" }).into()], ..Puzzle2dPlayRuntime::default() };`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs:41` — `source, target` — `let result = dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v0") })), None).expect("createEdge");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs:56` — `source, target` — `let refused = dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v7") })), None).expect("createEdge");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs:67` — `source, target` — `dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v0") })), None).expect("first connect");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs:69` — `source, target` — `let refused = dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v2") })), None).expect("second connect");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs:80` — `source, target` — `dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v0") })), None).expect("connect");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:15` — `source, target` — `"meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:17` — `handleKind, handles, shape` — `{ "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "left:v0", "handleKind": "a", "angle": 0.0 }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:18` — `handleKind, handles, shape` — `{ "id": "right", "shape": "circle", "x": gap, "y": 0.0, "radius": 24.0, "handles": [{ "id": "right:v0", "handleKind": "a", "angle": std::f64::consts::PI }] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:58` — `source, target` — `occupied["edges"] = json!([{ "id": "e0", "source": "left:v0", "target": "right:v0" }]);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:69` — `handleKind, handles, shape` — `nodes.push(json!({ "id": format!("left{index}"), "shape": "circle", "x": 0.0, "y": y, "radius": 24.0, "handles": [{ "id": format!("left{index}:v0"), "handleKind": "a", "angle": 0.0 }] }));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:70` — `handleKind, handles, shape` — `nodes.push(json!({ "id": format!("right{index}"), "shape": "circle", "x": 52.0, "y": y, "radius": 24.0, "handles": [{ "id": format!("right{index}:v0"), "handleKind": "a", "angle": std::f64::consts::PI }] }));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:72` — `source, target` — `let mut snapshot = recorder_of(json!({ "schema": "board.ports.directed.v1", "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] }, "nodes": nodes, "edges": [] }));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs:86` — `kind` — `let events = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": ["right"], "dx": -948.0, "dy": 0.0, "proximity": [] } }]).to_string();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:17` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:24` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:27` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:56` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:63` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:66` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:98` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:105` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:108` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:139` — `source, target` — `"edges": [{ "id": "e1", "source": "a", "target": "b" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:166` — `handles` — `{ "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0, "handles": [] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:167` — `handles` — `{ "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0, "handles": [] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:169` — `source, target` — `"edges": [{ "id": "e1", "source": "a", "target": "b" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:204` — `handleKind, handles` — `"handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:210` — `source` — `"source": format!("{prev}:h0"),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:211` — `target` — `"target": format!("{id}:h0")`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:256` — `handleKind, handles` — `"handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:262` — `source` — `"source": format!("{prev}:h0"),`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:263` — `target` — `"target": format!("{id}:h0")`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:296` — `handleKind, handles` — `{ "id": "a", "x": 0.0, "y": 0.0, "radius": 30.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:297` — `handleKind, handles` — `{ "id": "b", "x": 3.0, "y": 1.0, "radius": 30.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:298` — `handleKind, handles` — `{ "id": "c", "x": -2.0, "y": 4.0, "radius": 28.0, "handles": [{ "id": "c:h0", "angle": 1.0, "handleKind": "port" }] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:301` — `source, target` — `{ "id": "e1", "source": "a:h0", "target": "b:h0" },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:302` — `source, target` — `{ "id": "e2", "source": "b:h0", "target": "c:h0" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:327` — `handleKind, handles` — `{ "id": "a", "x": 0.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:328` — `handleKind, handles` — `{ "id": "b", "x": 5.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:329` — `handleKind, handles` — `{ "id": "c", "x": 2.0, "y": 8.0, "radius": 18.0, "handles": [{ "id": "c:h0", "angle": 0.0, "handleKind": "port" }] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:332` — `source, target` — `{ "id": "e1", "source": "a:h0", "target": "b:h0" },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:333` — `source, target` — `{ "id": "e2", "source": "b:h0", "target": "c:h0" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:365` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:372` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:375` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:407` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:414` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:417` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:439` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:446` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:449` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:494` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:499` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:502` — `source, target` — `"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:525` — `handles, root` — `{ "id": "r", "root": true, "radius": 18.0, "handles": [] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:526` — `handles` — `{ "id": "c1", "radius": 18.0, "handles": [] },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:527` — `handles` — `{ "id": "c2", "radius": 18.0, "handles": [] }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:530` — `source, target` — `{ "id": "e1", "source": "r", "target": "c1" },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:531` — `source, target` — `{ "id": "e2", "source": "r", "target": "c2" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:563` — `root` — `"root": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:565` — `handleKind, handles` — `"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:570` — `handleKind, handles` — `"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:575` — `handleKind, handles` — `"handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:579` — `source, target` — `{ "id": "e1", "source": "r:h", "target": "c1:h" },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:580` — `source, target` — `{ "id": "e2", "source": "r:h", "target": "c2:h" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:614` — `root` — `"root": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:616` — `handleKind, handles` — `"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:623` — `handleKind, handles` — `"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:630` — `handleKind, handles` — `"handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:634` — `source, target` — `{ "id": "e1", "source": "r:h", "target": "c1:h" },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:635` — `source, target` — `{ "id": "e2", "source": "r:h", "target": "c2:h" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:670` — `root` — `"root": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:672` — `handleKind, handles` — `"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:679` — `handleKind, handles` — `"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:682` — `source, target` — `"edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:714` — `root` — `"root": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:716` — `handleKind, handles` — `"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:721` — `handleKind, handles` — `"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:724` — `source, target` — `"edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:752` — `root` — `"root": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:754` — `handleKind, handles` — `"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:759` — `handleKind, handles` — `"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:762` — `source, target` — `"edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:790` — `root` — `"root": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:792` — `handleKind, handles` — `"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:332` — `color` — `manifest["portKinds"].as_array().unwrap().iter().map(|row| json!({ "id": row["id"], "name": row["name"], "color": row["presentation"]["color"], "defaultWireKind": row["presentation"]["defaultWireKind"] })).collect();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:15` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:27` — `source, target` — `assert_eq!(record["payload"]["proximity"], json!([{ "source": "a:h0", "target": "b:h0" }]), "the record carries the previewed proximity pair: {record}");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:35` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:82` — `color` — `{"id":"core.rect.bottom","name":"B","color":"#112233","defaultWireKind":"link.w"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:83` — `color` — `{"id":"core.rect.top","name":"T","color":"#112233","defaultWireKind":"link.w"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:90` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:221` — `root, shape` — `{ "id": "a", "x": 0.0, "y": 0.0, "width": 48.0, "height": 48.0, "shape": "rectangle", "root": true },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:222` — `shape` — `{ "id": "b", "x": 120.0, "y": 0.0, "width": 40.0, "height": 40.0, "shape": "rectangle" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:225` — `edgeKind, source, target` — `{ "id": "e1", "source": "a", "target": "b", "edgeKind": "wires.owns" }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:245` — `color` — `{"id":"wires.owns","name":"Owns","color":"#ff0000","stroke":"3","pattern":"dashed","targetTip":"filled-diamond","directed":false},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:246` — `color` — `{"id":"wires.is","name":"Is","color":"#00ff00","pattern":"dotted","targetTip":"filled-arrow","directed":false}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:249` — `color, shape` — `{"id":"capsule","name":"Capsule","shape":"circle","color":"#aabbcc"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:348` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:355` — `handleKind, handles, hidden` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "hidden": true }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:381` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:391` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "parent" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:398` — `hidden` — `"hidden": true,`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:399` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "child" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:445` — `handleKind, handles` — `"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:452` — `handleKind, handles` — `"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "locked": true }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:489` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:511` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:545` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:560` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:587` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:616` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:635` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:649` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:668` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:729` — `color` — `"handleKinds": [{"id":"slot-a","name":"Slot A","color":"#112233","scale":2.0}],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:753` — `color` — `"handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:759` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:783` — `color` — `{"id":"space","name":"S","color":"hsl(206 52% 48%)"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:784` — `color` — `{"id":"comma","name":"C","color":"hsl(206, 52%, 48%)"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:785` — `color` — `{"id":"slash","name":"Sl","color":"hsl(206 52% 48% / 0.5)"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:801` — `color, label` — `let err = h.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({"handleKinds":[{"id":"h","label":"legacy","color":"#112233"}]}).to_string()).unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:812` — `color` — `"handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:820` — `source, target` — `{"source":"flow.wire","target":"nope","specificity":"wire"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:821` — `source, target` — `{"source":"parent","target":"child","specificity":"general","important":true}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:847` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:884` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:289` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:290` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:295` — `handles` — `"handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:296` — `handleKind` — `{ "handleKind": "child", "angle": 0.0 },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:297` — `handleKind` — `{ "handleKind": "child", "angle": 3.141592653589793 }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:316` — `color` — `"handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:320` — `handleKind, handles` — `"handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:394` — `color` — `"handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:395` — `handleKind, handles` — `"nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }] }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:402` — `handles, nodeKind, shape` — `{ "id": "a", "nodeKind": "a.kind", "shape": "circle", "radius": 40.0, "x": 0.0, "y": 0.0, "handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:403` — `handleKind` — `{ "id": "a:h0", "handleKind": "port", "angle": 0.0 },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:404` — `handleKind` — `{ "id": "a:h1", "handleKind": "port", "angle": 3.141592653589793 }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:406` — `handles, nodeKind, shape` — `{ "id": "b", "nodeKind": "a.kind", "shape": "circle", "radius": 40.0, "x": 200.0, "y": 0.0, "handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:407` — `handleKind` — `{ "id": "b:h0", "handleKind": "port", "angle": 3.141592653589793 }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:410` — `edgeKind, source, target` — `"edges": [{ "id": "e0", "edgeKind": "link", "source": "a:h0", "target": "b:h0" }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:436` — `color` — `"handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:440` — `handleKind, handles` — `"handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:514` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:515` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:520` — `handleKind, handles` — `"handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:548` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:549` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:554` — `handleKind, handles` — `"handles": [{"handleKind": "child", "angle": 3.141592653589793}]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:579` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:580` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:585` — `handles` — `"handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:586` — `handleKind` — `{ "handleKind": "child", "angle": 0.0 },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:587` — `handleKind` — `{ "handleKind": "child", "angle": 3.141592653589793 }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:618` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:619` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:624` — `handles` — `"handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:625` — `handleKind` — `{ "handleKind": "child", "angle": 0.0 },`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:626` — `handleKind` — `{ "handleKind": "child", "angle": 3.141592653589793 }`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:667` — `color, handleKind, handles` — `h.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({ "handleKinds": [{ "id": "child", "name": "Child", "color": "#888888" }], "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "child", "angle": 0.0 }] }] }).to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1080` — `iconKind, nodeKind, shape` — `h.set_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1100` — `shape` — `"shape": "circle",`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1102` — `handleKind, handles` — `"handles": [{"handleKind": "door", "angle": 0.0}]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1108` — `iconKind, nodeKind, shape` — `h.set_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1124` — `color` — `"handleKinds": [{"id": "parent", "name": "Parent", "color": "#888888"}],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1128` — `handleKind, handles` — `"handles": [{"handleKind": "parent", "angle": 3.141592653589793}]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1142` — `nodeKind` — `"nodeKind": "brush.kind",`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1145` — `shape` — `"shape": "circle",`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1147` — `handleKind, handles` — `"handles": [{"handleKind": "parent", "angle": 3.141592653589793}]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1166` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1170` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1171` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1177` — `handles` — `"handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1178` — `handleKind` — `{"handleKind": "child", "angle": 0.0},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1179` — `handleKind` — `{"handleKind": "child", "angle": 3.141592653589793}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1185` — `handleKind, handles` — `"handles": [{"handleKind": "child", "angle": 3.141592653589793}]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1214` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1218` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1219` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1224` — `handles` — `"handles": [`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1225` — `handleKind` — `{"handleKind": "child", "angle": 0.0},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1226` — `handleKind` — `{"handleKind": "child", "angle": 3.141592653589793}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1424` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1425` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1430` — `handleKind, handles` — `"handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1453` — `source, target` — `h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1457` — `color` — `{"id": "parent", "name": "Parent", "color": "#888888"},`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1458` — `color` — `{"id": "child", "name": "Child", "color": "#888888"}`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1463` — `handleKind, handles` — `"handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1493` — `label` — `"label": "Brush Kind",`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1503` — `handleKind, handles, label` — `"handles": [{ "id": "t0", "name": "t0", "label": "T0", "description": "", "icon": "", "handleKind": "port", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1505` — `color, handles, label` — `"handles": [{ "id": "port", "label": "Port", "compatibleWith": [], "description": "", "icon": "", "color": "#888888", "defaultWireKind": "wire.link" }],`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1584` — `label` — `"label": "Brush Kind",`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1594` — `handleKind, handles, label` — `"handles": [{ "id": "t0", "name": "t0", "label": "T0", "description": "", "icon": "", "handleKind": "port", "angle": 3.141592653589793 }]`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1596` — `color, handles, label` — `"handles": [{ "id": "port", "label": "Port", "compatibleWith": [], "description": "", "icon": "", "color": "#888888", "defaultWireKind": "wire.link" }],`

## Admission Semantics Established By Current Receiver

Nodes require string id and finite x/y. Directed snapshots require handles array; each handle requires object/string id/finite angle. Normal snapshots do not require handles. Rectangle variants require finite dimensions; default circle radius and optional scale follow the actual branch rules in the source excerpt. Edge id/source/target strings are mandatory; Normal endpoints must resolve to actual node identities. Directed endpoints are handle identities. Source/target object-shaped endpoints are not accepted by this receiver. Region rows with missing/empty id are explicitly skipped; non-finite or absent region coordinates/dimensions default to0; label trims empty strings. Metadata userData/style/data/kind values present in fixtures must not be silently admitted as node visual facts merely because the older open map ignored them. The schema-first owner should distinguish open semantic userdata from declared graph facts and handcraft fixture fields consistently. Current snake/camel edge_kind/source_tip/target_tip and node_kind aliases are actual existing receiving, not a reason to retain compatibility names in greenfield typed APIs.

The line evidence is lexical fixture input, not a complete AST object-shape inventory. Multi-line raw string bodies and new concurrent fixtures need the execution peer’s own final admission witnesses.
