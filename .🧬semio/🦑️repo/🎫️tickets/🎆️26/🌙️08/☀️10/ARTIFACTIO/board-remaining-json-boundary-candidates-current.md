# Board Remaining JSON Boundary Candidates

This current-source inventory is an audit handoff. These candidates are unfinished ownership work, not exemptions. Public layout helpers, host catalog/session inputs, DAG projection/status setters and browser bridge endpoints must be classified against semantic/physical ownership. Browser bridge functions and semantic host functions are retained as separate candidates. Duplicate method names make the lexical caller sets conservative; the following audit must resolve receiver types.

Full exact signatures and bodies, source coordinates, SHA-256 body digests, codec symbols and direct call-site text are preserved in `🗑️generated/board-remaining-json-bodies.json`. This inventory is source evidence only; no runtime/ownership conclusion is inferred.

## apply_force_graph_layout_to_board_snapshot_value

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:932` through line 940.
- Exact body SHA-256: `3107823242009c0fdf2a7f4b23e346d71e6b89455e0ed4f23c4803fdc8596928`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn apply_force_graph_layout_to_board_snapshot_value(snapshot: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), String> {
        let nodes = snapshot.as_object().and_then(|root| root.get("nodes")).and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let handle_to_node = build_handle_to_node(&nodes);
        infinite::board::normal::undirected::apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot, opts, |endpoint, id_to_index| {
            let node_id = handle_to_node.get(endpoint).cloned().unwrap_or_else(|| endpoint.to_string());
            id_to_index.contains_key(&node_id).then_some(node_id)
        })
        .map_err(|e| e.to_string())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:946:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1713:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:343:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:398:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`

## apply_force_graph_layout_to_board_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:943` through line 948.
- Exact body SHA-256: `cfcc8cadb0558d865a8dfb61e303364af62f8c867ec24e9c20ef203414c350c4`.
- Explicit codec calls: `serde_json::from_str`, `serde_json::to_string`.

```rust
pub fn apply_force_graph_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, String> {
        let mut snapshot: Value = serde_json::from_str(snapshot_json).map_err(|e| e.to_string())?;
        let opts: ForceGraphLayoutOptions = if options_json.trim().is_empty() { ForceGraphLayoutOptions::default() } else { serde_json::from_str(options_json).map_err(|e| e.to_string())? };
        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;
        serde_json::to_string(&snapshot).map_err(|e| e.to_string())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:37:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:77:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:179:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:189:    let err = apply_force_graph_layout_to_board_snapshot_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:231:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:285:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:286:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:316:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:317:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:346:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚛️force-layout/🦀️.rs:8:    let Ok(layout_json) = crate::editor::puzzle2d::engine::apply_force_graph_layout_to_board_snapshot_json(&ctx.scene.board_snapshot.to_string(), r#"{"mode":"force-graph"}"#) else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:36:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:64:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`

## apply_hierarchical_tree_layout_to_board_snapshot_value

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1311` through line 1514.
- Exact body SHA-256: `fcc94ceaef1cf7a8a42fc1a0b67e6536d619cadb20b64c1a7b9748bf03ad0f80`.
- Explicit codec calls: `serde_json::json`.

```rust
pub fn apply_hierarchical_tree_layout_to_board_snapshot_value(snapshot: &mut Value, opts: &HierarchicalTreeLayoutOptions) -> Result<(), String> {
        let dir = TreeDirection::parse(&opts.direction)?;
        let Some(root) = snapshot.as_object_mut() else {
            return Err("snapshot root must be object".into());
        };
        if root.get("schema").and_then(|v| v.as_str()) != Some("board.ports.directed.v1") {
            return Err("schema must be board.ports.directed.v1".into());
        }
        let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err("nodes array missing".into());
        };
        if nodes.is_empty() {
            return Ok(());
        }
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        let mut id_to_node: HashMap<String, Value> = HashMap::new();
        for node in nodes.iter() {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}) {
                continue;
            }
            let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            id_to_node.insert(nid.to_string(), node.clone());
            let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for h in handles {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:ho.get("hidden").and_then(|v|v.as_bool()),visible:ho.get("visible").and_then(|v|v.as_bool()),locked:ho.get("locked").and_then(|v|v.as_bool())}) {
                    continue;
                }
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
        if id_to_node.is_empty() {
            return Ok(());
        }
        let mut directed: Vec<(String, String)> = Vec::new();
        let mut seen_dir: HashSet<(String, String)> = HashSet::new();
        for e in &edges_json {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:eo.get("hidden").and_then(|v|v.as_bool()),visible:eo.get("visible").and_then(|v|v.as_bool()),locked:eo.get("locked").and_then(|v|v.as_bool())}) {
                continue;
            }
            let Some((src_h, tgt_h)) = board_edge_handle_ids(eo.get("source").and_then(|v|v.as_str()),eo.get("target").and_then(|v|v.as_str())) else {
                continue;
            };
            let source_node_id = super::resolve_endpoint_node_id(src_h, &handle_to_node);
            let target_node_id = super::resolve_endpoint_node_id(tgt_h, &handle_to_node);
            if source_node_id == target_node_id {
                continue;
            }
            if !id_to_node.contains_key(&source_node_id) || !id_to_node.contains_key(&target_node_id) {
                continue;
            }
            if seen_dir.insert((source_node_id.clone(), target_node_id.clone())) {
                directed.push((source_node_id, target_node_id));
            }
        }
        let mut incoming_edge_count_by_node: HashMap<String, u32> = HashMap::new();
        for id in id_to_node.keys() {
            incoming_edge_count_by_node.insert(id.clone(), 0);
        }
        for (_source_nid, target_nid) in &directed {
            *incoming_edge_count_by_node.entry(target_nid.clone()).or_insert(0) += 1;
        }
        let mut roots: Vec<String> = Vec::new();
        for node in nodes.iter() {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}) {
                continue;
            }
            if obj.get("root").and_then(|v| v.as_bool()) == Some(true) {
                if let Some(nid) = obj.get("id").and_then(|v| v.as_str()) {
                    roots.push(nid.to_string());
                }
            }
        }
        roots.sort();
        roots.dedup();
        if roots.is_empty() {
            for (id, &d) in &incoming_edge_count_by_node {
                if d == 0 {
                    roots.push(id.clone());
                }
            }
            roots.sort();
        }
        if roots.is_empty() {
            roots = id_to_node.keys().cloned().collect();
            roots.sort();
        }
        let mut depth: HashMap<String, i32> = HashMap::new();
        for r in &roots {
            depth.insert(r.clone(), 0);
        }
        let cap = directed.len().saturating_mul(3).saturating_add(nodes.len()).saturating_add(8);
        for _ in 0..cap {
            let mut changed = false;
            for (source_nid, target_nid) in &directed {
                let Some(&dp) = depth.get(source_nid) else {
                    continue;
                };
                let nd = dp + 1;
                let cur = *depth.get(target_nid).unwrap_or(&-1);
                if nd > cur {
                    depth.insert(target_nid.clone(), nd);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let max_depth = depth.values().copied().max().unwrap_or(0);
        for id in id_to_node.keys() {
            depth.entry(id.clone()).or_insert(max_depth + 1);
        }
        let raw = run_buchheim_layout(&id_to_node, &roots, &directed, &depth)?;
        let mean_half: f64 = id_to_node.values().map(half_extent).sum::<f64>() / id_to_node.len().max(1) as f64;
        let along_scale = (opts.sibling_gap + 2.0 * mean_half).max(8.0);
        let mut pos: HashMap<String, (f64, f64)> = HashMap::new();
        for (id, (bx, by)) in raw {
            let along = bx * along_scale;
            let orth = by * opts.layer_spacing;
            let (lx, ly) = match dir {
                TreeDirection::Downwards => (along, orth),
                TreeDirection::Upwards => (along, -orth),
                TreeDirection::Right => (orth, along),
                TreeDirection::Left => (-orth, along),
            };
            pos.insert(id, (lx, ly));
        }
        let mut minx = f64::INFINITY;
        let mut maxx = f64::NEG_INFINITY;
        let mut miny = f64::INFINITY;
        let mut maxy = f64::NEG_INFINITY;
        for (id, (x, y)) in &pos {
            let h = half_extent(id_to_node.get(id).unwrap());
            minx = minx.min(x - h);
            maxx = maxx.max(x + h);
            miny = miny.min(y - h);
            maxy = maxy.max(y + h);
        }
        if !minx.is_finite() {
            minx = 0.0;
            maxx = 1.0;
            miny = 0.0;
            maxy = 1.0;
        }
        let cx = (minx + maxx) * 0.5;
        let cy = (miny + maxy) * 0.5;
        let gx = opts.center_x.unwrap_or(0.0);
        let gy = opts.center_y.unwrap_or(0.0);
        let dx = gx - cx;
        let dy = gy - cy;
        let locked_set: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut pinned_world: HashMap<String, (f64, f64)> = HashMap::new();
        if !locked_set.is_empty() {
            for node in nodes.iter() {
                let Some(obj) = node.as_object() else {
                    continue;
                };
                if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}) {
                    continue;
                }
                let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                    continue;
                };
                if !locked_set.contains(nid) {
                    continue;
                }
                if !id_to_node.contains_key(nid) {
                    continue;
                }
                let px = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let py = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                pinned_world.insert(nid.to_string(), (px, py));
            }
        }
        for (id, (x, y)) in pos {
            let (fx, fy) = if let Some(&(px, py)) = pinned_world.get(&id) { (px, py) } else { (x + dx, y + dy) };
            let idx = nodes.iter().position(|n| n.get("id").and_then(|v| v.as_str()) == Some(id.as_str())).ok_or_else(|| format!("node index {id}"))?;
            let Some(obj) = nodes[idx].as_object_mut() else {
                continue;
            };
            obj.insert("x".into(), serde_json::json!(fx));
            obj.insert("y".into(), serde_json::json!(fy));
        }
        Ok(())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1728:                apply_hierarchical_tree_layout_to_board_snapshot_value(&mut snapshot, &hierarchical_opts)?;`

## apply_edge_handle_snap_to_board_snapshot_value

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1570` through line 1657.
- Exact body SHA-256: `49ce431d561b6ae752a2d5b6be8bdcecea0b17645783651e6217e1be5d94777e`.
- Explicit codec calls: `serde_json::json`.

```rust
pub fn apply_edge_handle_snap_to_board_snapshot_value(snapshot: &mut Value) -> Result<(), String> {
        let Some(root) = snapshot.as_object_mut() else {
            return Err("snapshot root must be object".into());
        };
        if root.get("schema").and_then(|v| v.as_str()) != Some("board.ports.directed.v1") {
            return Err("schema must be board.ports.directed.v1".into());
        }
        let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err("nodes array missing".into());
        };
        let mut shapes: Vec<Option<NodeShapeSnap>> = Vec::with_capacity(nodes.len());
        let mut handle_loc: HashMap<String, (usize, usize)> = HashMap::new();
        for (ni, node_val) in nodes.iter().enumerate() {
            let Some(no) = node_val.as_object() else {
                shapes.push(None);
                continue;
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:no.get("hidden").and_then(|v|v.as_bool()),visible:no.get("visible").and_then(|v|v.as_bool()),locked:no.get("locked").and_then(|v|v.as_bool())}) {
                shapes.push(None);
                continue;
            }
            shapes.push(parse_node_shape_snap(no));
            let Some(hs) = no.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for (hi, h) in hs.iter().enumerate() {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:ho.get("hidden").and_then(|v|v.as_bool()),visible:ho.get("visible").and_then(|v|v.as_bool()),locked:ho.get("locked").and_then(|v|v.as_bool())}) {
                    continue;
                }
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_loc.insert(hid.to_string(), (ni, hi));
                }
            }
        }
        let mut angle_by_loc: HashMap<(usize, usize), f64> = HashMap::new();
        for e in &edges_json {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:eo.get("hidden").and_then(|v|v.as_bool()),visible:eo.get("visible").and_then(|v|v.as_bool()),locked:eo.get("locked").and_then(|v|v.as_bool())}) {
                continue;
            }
            let Some((src_h, tgt_h)) = board_edge_handle_ids(eo.get("source").and_then(|v|v.as_str()),eo.get("target").and_then(|v|v.as_str())) else {
                continue;
            };
            let Some(&(ni_a, hi_a)) = handle_loc.get(src_h) else {
                continue;
            };
            let Some(&(ni_b, hi_b)) = handle_loc.get(tgt_h) else {
                continue;
            };
            let Some(sa) = shapes.get(ni_a).copied().flatten() else {
                continue;
            };
            let Some(sb) = shapes.get(ni_b).copied().flatten() else {
                continue;
            };
            if let Some(ang_a) = sa.handle_angle_toward(sb.center()) {
                angle_by_loc.insert((ni_a, hi_a), ang_a);
            }
            if let Some(ang_b) = sb.handle_angle_toward(sa.center()) {
                angle_by_loc.insert((ni_b, hi_b), ang_b);
            }
        }
        for ((ni, hi), ang) in angle_by_loc {
            let Some(node_val) = nodes.get_mut(ni) else {
                continue;
            };
            let Some(no) = node_val.as_object_mut() else {
                continue;
            };
            let Some(hs) = no.get_mut("handles").and_then(|v| v.as_array_mut()) else {
                continue;
            };
            let Some(h) = hs.get_mut(hi) else {
                continue;
            };
            let Some(ho) = h.as_object_mut() else {
                continue;
            };
            ho.insert("angle".into(), serde_json::json!(ang));
        }
        Ok(())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1661:        apply_edge_handle_snap_to_board_snapshot_value(&mut snapshot)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1733:            apply_edge_handle_snap_to_board_snapshot_value(&mut snapshot)?;`

## apply_edge_handle_snap_to_board_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1659` through line 1663.
- Exact body SHA-256: `26a97cf4130a7ef222e2e6974e7ec4f6a86df1601288b4cb7dc562f39e59057e`.
- Explicit codec calls: `serde_json::from_str`, `serde_json::to_string`.

```rust
pub fn apply_edge_handle_snap_to_board_snapshot_json(snapshot_json: &str) -> Result<String, String> {
        let mut snapshot: Value = serde_json::from_str(snapshot_json).map_err(|e| e.to_string())?;
        apply_edge_handle_snap_to_board_snapshot_value(&mut snapshot)?;
        serde_json::to_string(&snapshot).map_err(|e| e.to_string())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:419:    let out = apply_edge_handle_snap_to_board_snapshot_json(&snapshot.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:71:    apply_edge_handle_snap_to_board_snapshot_json(snapshot_json).map_err(|e| JsValue::from_str(&e))`

## apply_redraw_layout_to_board_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1693` through line 1736.
- Exact body SHA-256: `f9e541990dc275d8dac15efe1b888c0ac9a4c12c82bf463df7bc22b6be959dac`.
- Explicit codec calls: `serde_json::from_str`, `serde_json::to_string`.

```rust
pub fn apply_redraw_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, String> {
        let opts: RedrawSnapshotOptions = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
        let mut snapshot: Value = serde_json::from_str(snapshot_json).map_err(|e| e.to_string())?;
        match opts.mode.as_str() {
            "force-graph" => {
                let mut fo = opts.force_graph.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    fo.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    fo.center_y = opts.center_y;
                }
                if let Some(s) = opts.random_seed {
                    fo.random_seed = s;
                }
                for id in &opts.locked_node_ids {
                    if !fo.locked_node_ids.contains(id) {
                        fo.locked_node_ids.push(id.clone());
                    }
                }
                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;
            }
            "hierarchical-tree" => {
                let mut hierarchical_opts = opts.hierarchical_tree.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    hierarchical_opts.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    hierarchical_opts.center_y = opts.center_y;
                }
                for id in &opts.locked_node_ids {
                    if !hierarchical_opts.locked_node_ids.contains(id) {
                        hierarchical_opts.locked_node_ids.push(id.clone());
                    }
                }
                apply_hierarchical_tree_layout_to_board_snapshot_value(&mut snapshot, &hierarchical_opts)?;
            }
            other => return Err(format!("unknown redraw mode: {other}")),
        }
        if opts.redraw_handles_after {
            apply_edge_handle_snap_to_board_snapshot_value(&mut snapshot)?;
        }
        serde_json::to_string(&snapshot).map_err(|e| e.to_string())
    }
```

Direct lexical call sites (Rust/TypeScript):

None found in the current first-party source census; exported callers or dispatch may still exist.

## puzzle_2d_lod_scale_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:197` through line 210.
- Exact body SHA-256: `8d154e73d938a7ef62b15c9673b23ba3fdd5ea541ff57b8b633169c2e1943202`.
- Explicit codec calls: `serde_json::json`, `serde_json::to_string`.

```rust
pub fn puzzle_2d_lod_scale_json() -> String {
        let rows: Vec<serde_json::Value> = PUZZLE_2D_LODS
            .iter()
            .map(|lod| {
                serde_json::json!({
                    "id": lod.id,
                    "name": lod.name,
                    "description": lod.description,
                    "maxZoom": lod.max_zoom,
                })
            })
            .collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📊️lod-scale-json/🦀️.rs:9:    let _ = crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🔭️lod/🦀️.rs:11:    semio_framework_pack_json::from_json_str::<Vec<Value>>(&crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:327:        puzzle_2d_lod_scale_json()`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs:12:    semio_framework_pack_json::from_json_str::<Vec<Value>>(&semio_framework_os_infinite::puzzle_2d_lod_scale_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned LOD options admit").into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()`

## payload_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:1869` through line 1871.
- Exact body SHA-256: `aad1214393ffe95f4ac4f2a521686a56c345e9c4c8d85056306e857a81cfa75c`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn payload_json(&self) -> &str {
            std::str::from_utf8(&self.payload[..usize::from(self.payload_len)]).expect("board event payload is UTF-8")
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs:161:    let document = payload_json(semio_framework_pack_json::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5405:    serde_json::from_str::<Value>(event.payload_json()).ok()?.get("gestureId")?.as_str().map(str::to_string)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5456:    let camera = queue.iter().filter(|event| event.kind() == BoardEventKind::Camera).filter_map(|event| engine_camera_from_json(event.payload_json())).last().map(|(x, y, zoom)| [x, y, zoom]);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:1885:            output.push_str(self.payload_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3587:            self.output_raw(event.payload_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3601:                self.output_raw(event.payload_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:315:            assert!(event.payload_json().contains(&format!("node-{index}")));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:335:        let payload: serde_json::Value = serde_json::from_str(event.payload_json()).unwrap();`

## write_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:1881` through line 1887.
- Exact body SHA-256: `e26f799a90e412f45a30bfac720ddba8471b0e3fe0678b88dd4b22c4547801e9`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn write_json(&self, output: &mut String) {
            output.push_str("{\"name\":\"");
            output.push_str(self.kind.name());
            output.push_str("\",\"payload\":");
            output.push_str(self.payload_json());
            output.push('}');
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1239:        event.write_json(&mut output);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:329:            event.write_json(&mut row);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5438:    event.write_json(output);`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️.rs:1875:        let text = write_json(&self.to_json().await).await;`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs:97:    let written = write_json(&value).await;`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs:160:    let written = write_json(&value).await;`
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs:163:    assert_eq!(write_json(&JsonValue::Num(3.0)).await, "3");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8174:                event.write_json(&mut out);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8180:                event.write_json(&mut out);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:230:                    Ok(crate::io_schema::IoPayload::Text(write_json(value.get(), &mut c)?))`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:1234:                    self.write_json(&edge, Process3dTimelinePhase::Edges(index + 1));`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs:1248:                    self.write_json(&node, Process3dTimelinePhase::Nodes { index: index + 1, tool: tool + usize::from(has_tool) });`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:108:    write_json(&read_json(bytes)?)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:206:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:218:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:232:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:245:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:255:            write_json(&root)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs:257:        "restore-snapshot" => write_json(&library_from_wire(params.get("snapshot").and_then(|snapshot| snapshot.get("value")).unwrap_or(&Json::Null))?),`

## events_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3361` through line 3363.
- Exact body SHA-256: `70a4ab83543c8dcc45a3696a092e435c9030f94f832ce45ca6102d11674eca9f`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn events_json(&self) -> &str {
            std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("pointer publication is encoded from UTF-8 schema tokens")
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5707:    let dispatch = board_page_dispatch_rows(plan.events_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5836:        let Some(events_json) = board_page_dispatch_rows(publication.events_json())? else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:170:        let value: serde_json::Value = serde_json::from_str(escaped.expect("escaped release plan").events_json()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:208:        let rows: serde_json::Value = serde_json::from_str(publication.events_json()).expect("publication rows");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:236:        assert!(preview.events_json().contains("preselect"));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:249:        assert!(commit.events_json().contains("select"));`

## events_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3607` through line 3609.
- Exact body SHA-256: `ffc1d5dfa24b608494085a64ee82f2e13cf4158b66194b22800f5509eb578b3e`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn events_json(&self) -> &str {
            std::str::from_utf8(&self.output[..usize::from(self.output_len)]).expect("board event page is encoded from UTF-8 schema tokens")
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5707:    let dispatch = board_page_dispatch_rows(plan.events_json())?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5836:        let Some(events_json) = board_page_dispatch_rows(publication.events_json())? else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:170:        let value: serde_json::Value = serde_json::from_str(escaped.expect("escaped release plan").events_json()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:208:        let rows: serde_json::Value = serde_json::from_str(publication.events_json()).expect("publication rows");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:236:        assert!(preview.events_json().contains("preselect"));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:249:        assert!(commit.events_json().contains("select"));`

## overlay_paint_state_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:3757` through line 3765.
- Exact body SHA-256: `a4c3f99fde2efcd783236a40bfbed4b4a9098a4dc8086b943effbb714c671dc9`.
- Explicit codec calls: `serde_json::json`.

```rust
pub fn overlay_paint_state_json(&self) -> String {
            let nodes: Vec<serde_json::Value> = self.nodes.values().filter(|n| n.visible).map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y })).collect();
            serde_json::json!({
                "camera": { "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom },
                "lod": Self::board_draw_lod_label(self.draw_lod_for_frame()),
                "nodes": nodes,
            })
            .to_string()
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:407:    let raw: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&h.overlay_paint_state_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("overlay paint state json");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs:241:    let overlay: Value = semio_framework_pack_json::from_json_str(&host.overlay_paint_state_json(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(Value::Null);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:307:        self.state.borrow().host.overlay_paint_state_json()`

## set_external_link_preview_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4335` through line 4353.
- Exact body SHA-256: `928b88c174e2ff6c40da93704ee2361c9ab8fb16dac40c8694b9ff65a2af7700`.
- Explicit codec calls: `serde_json::from_str`.

```rust
pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::ExternalLinkPreviewJson)?;
            let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
            if source.is_empty() {
                if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                }
                return Ok(());
            }
            let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
            let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
            let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
            let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
            self.interaction = Interaction::ExternalLinkPreview { source_id: source, end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
            self.sync_link_gesture_events();
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:451:        self.state.borrow_mut().host.set_external_link_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`

## set_handle_link_compat_from_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4423` through line 4439.
- Exact body SHA-256: `25fe493dedb06b58980a76ae1ead774b898273887d6f70102fb96548bb898061`.
- Explicit codec calls: `serde_json::from_str`.

```rust
pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            let v: serde_json::Value = serde_json::from_str(json)?;
            let arr = v.as_array().ok_or(NormalPortError::CompatNotArray)?;
            let mut next = Vec::new();
            for row in arr {
                let o = row.as_object().ok_or(NormalPortError::RowNotObject("compat"))?;
                let source = o.get("source").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatSourceMissing)?.trim().to_string();
                let target = o.get("target").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatTargetMissing)?.trim().to_string();
                let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
                let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
                let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
                let specificity = Self::parse_compat_specificity(spec_s)?;
                next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
            }
            self.link_compat_rules = next;
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1166:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1214:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1260:        h.set_handle_link_compat_from_json(&compat_str).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1342:        h.set_handle_link_compat_from_json(&compat_str).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1453:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:15:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:35:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:90:        h.set_handle_link_compat_from_json(r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:381:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:489:        h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:511:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:545:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:560:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:587:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:616:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:635:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:649:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:668:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:759:        h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:818:        h.set_handle_link_compat_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:847:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:884:        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1152:            let _ = host.set_handle_link_compat_from_json(&json);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:317:        self.state.borrow_mut().host.set_handle_link_compat_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3129:        let _ = host.set_handle_link_compat_from_json(&board.placement_compatibility_json);`

## set_board_kind_catalogs_from_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4460` through line 4581.
- Exact body SHA-256: `464dfb2fdd55875ab11848f36b40b4c1ac2e9fda62e84b7d5477d94812bc4de3`.
- Explicit codec calls: `serde_json::from_str`.

```rust
pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            if json.len() > BOARD_EVENT_BYTE_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            let v: serde_json::Value = serde_json::from_str(json)?;
            let o = v.as_object().ok_or(NormalPortError::KindCatalogsRootNotObject)?;
            for key in ["handleKinds", "wireKinds", "nodeKinds", "edgeTips", "edgeKinds"] {
                if o.get(key).and_then(serde_json::Value::as_array).is_some_and(|rows| rows.len() > BOARD_POINTER_ITEM_CAPACITY) {
                    return Err(NormalPortError::EventCredits);
                }
            }
            let template_count = o
                .get("nodeKinds")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_object)
                .filter_map(|row| row.get("handles"))
                .filter_map(serde_json::Value::as_array)
                .try_fold(0usize, |count, handles| count.checked_add(handles.len()))
                .ok_or(NormalPortError::EventCredits)?;
            if template_count > BOARD_POINTER_ITEM_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let ho = row.as_object().ok_or(NormalPortError::RowNotObject("handle kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(ho, "handle")?;
                    let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("handle kind"))?;
                    let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::HandleKindColorMissing)?;
                    let color = Self::parse_css_color(color_s).ok_or_else(|| NormalPortError::InvalidHandleKindColor(color_s.to_string()))?;
                    let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                    next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
                }
                self.handle_kinds = next;
            }
            if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let wo = row.as_object().ok_or(NormalPortError::RowNotObject("wire kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(wo, "wire")?;
                    let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("wire kind"))?;
                    let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
                }
                self.wire_kinds = next;
            }
            if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let no = row.as_object().ok_or(NormalPortError::RowNotObject("node kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(no, "node")?;
                    let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("node kind"))?;
                    let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                    let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                        Some("rectangle") => NodeShape::Rectangle,
                        _ => NodeShape::Circle,
                    };
                    let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                    if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                        for row in arr {
                            let ho = row.as_object().ok_or(NormalPortError::RowNotObject("node kind handle"))?;
                            let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::NodeKindHandleKindMissing)?;
                            let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or(NormalPortError::NodeKindHandleAngleMissing)?;
                            let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                            handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                        }
                    }
                    let icon = no.get("icon").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    let color_fill = no.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                    next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
                }
                self.node_kinds = next;
            }
            if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
                let mut tips = builtin_edge_tips();
                for row in arr {
                    let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge tip"))?;
                    let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge tip"))?;
                    let geometry=match eo.get("geometry") {
                        None=>None,
                        Some(value)=>Some(match value.as_str().map(str::trim){Some("arrow")=>EdgeTipGeometry::Arrow,Some("fine-arrow")=>EdgeTipGeometry::FineArrow,Some("diamond")=>EdgeTipGeometry::Diamond,Some("circle")=>EdgeTipGeometry::Circle,Some("bar")=>EdgeTipGeometry::Bar,_=>return Err(NormalPortError::EdgeTipRowInvalid(id.to_string()))})
                    };
                    let entry=crate::infinite::board::ports::directed::EdgeTipCatalogEntry{id:id.to_string(),geometry,filled:eo.get("filled").and_then(|v|v.as_bool()),scale:eo.get("scale").and_then(|v|v.as_f64())};
                    let def = EdgeTipDef::from_catalog_entry(&entry).ok_or_else(|| NormalPortError::EdgeTipRowInvalid(id.to_string()))?;
                    tips.insert(id.to_string(), def);
                }
                self.edge_tips = tips;
            }
            if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(eo, "edge")?;
                    let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge kind"))?;
                    let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let color = eo.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                    let stroke_width = eo
                        .get("stroke")
                        .and_then(|x| x.as_f64())
                        .filter(|v| v.is_finite() && *v > 0.0)
                        .or_else(|| eo.get("stroke").and_then(|x| x.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).filter(|v| v.is_finite() && *v > 0.0))
                        .unwrap_or(2.0);
                    let pattern = match eo.get("pattern").and_then(|x| x.as_str()).map(str::trim) {
                        Some("dashed") => EdgeStrokePattern::Dashed,
                        Some("dotted") => EdgeStrokePattern::Dotted,
                        _ => EdgeStrokePattern::Solid,
                    };
                    let source_tip = Self::parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                    let target_tip = Self::parse_catalog_tip_slot(eo.get("targetTip").or_else(|| eo.get("target_tip")).and_then(|x| x.as_str()).or_else(|| eo.get("marker").and_then(|x| x.as_str())));
                    let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                    next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
                }
                self.edge_kinds = next;
            }
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:286:        host.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:323:        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:397:        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:443:        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:511:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:545:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:576:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:615:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:667:        h.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({ "handleKinds": [{ "id": "child", "name": "Child", "color": "#888888" }], "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "child", "angle": 0.0 }] }] }).to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1094:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1122:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1167:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1215:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1261:        h.set_board_kind_catalogs_from_json(&catalogs_json_from_manifest_id("nakagin")).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1353:        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1421:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1454:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1530:        host.set_board_kind_catalogs_from_json(catalogs_json).expect("catalog push must be accepted by the engine");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1558:        BoardHost::new().set_board_kind_catalogs_from_json(&catalogs).expect("manifest catalogs must satisfy the engine contract");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1607:        catalog_host.set_board_kind_catalogs_from_json(&catalogs).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:79:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:242:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:727:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:751:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:780:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:801:        let err = h.set_board_kind_catalogs_from_json(&semio_framework_pack_json::json!({"handleKinds":[{"id":"h","label":"legacy","color":"#112233"}]}).to_string()).unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:810:        h.set_board_kind_catalogs_from_json(`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:338:    host.set_board_kind_catalogs_from_json(&catalogs_json).expect("catalog json derived from the manifest must be valid");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1148:        let _ = host.set_board_kind_catalogs_from_json(&json);`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:206:        self.state.borrow_mut().host.set_board_kind_catalogs_from_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3124:        let _ = host.set_board_kind_catalogs_from_json(&board.glyph_catalogs_json);`

## set_brush_session_mirror_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7525` through line 7588.
- Exact body SHA-256: `f4c2cb00629133186ec96dab282be4e9fe9100e14d87887e2f9b97cb050f4f76`.
- Explicit codec calls: `serde_json::from_str`.

```rust
pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            if json.trim().is_empty() {
                self.brush_slot_suggestions_active = false;
                self.brush_slot_source_id = None;
                self.brush_candidates.clear();
                self.brush_candidate_index = 0;
                self.brush_preview = None;
                self.brush_preview_emit_key = None;
                self.brush_candidates_emit_key = None;
                self.bump_content_scene_generation();
                return Ok(());
            }
            if json.len() > BOARD_EVENT_BYTE_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::BrushSessionJson)?;
            let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            self.brush_slot_source_id = source.clone();
            let mut candidates = BrushCandidatePage::default();
            if let Some(rows) = v.get("candidates").and_then(|x| x.as_array()) {
                if rows.len() > BOARD_POINTER_ITEM_CAPACITY {
                    return Err(NormalPortError::EventCredits);
                }
                for row in rows {
                    let (node_kind_id, target_handle_index) = if let Some(node_kind_id) = row.as_str().map(str::trim).filter(|value| !value.is_empty()) {
                        (node_kind_id, 0)
                    } else {
                        let node_kind_id = row.get("nodeKind").or_else(|| row.get("nodeKindId")).and_then(|value| value.as_str()).map(str::trim).filter(|value| !value.is_empty()).ok_or(NormalPortError::EventCredits)?;
                        let target_handle_index = row.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0);
                        let target_handle_index = usize::try_from(target_handle_index).map_err(|_| NormalPortError::EventCredits)?;
                        (node_kind_id, target_handle_index)
                    };
                    candidates.push(node_kind_id, target_handle_index, 0.0).map_err(|_| NormalPortError::EventCredits)?;
                }
            }
            self.brush_candidates = candidates;
            self.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map_or(0, |i| i as usize);
            if self.brush_candidates.is_empty() {
                self.brush_candidate_index = 0;
            } else {
                self.brush_candidate_index %= self.brush_candidates.len();
            }
            self.brush_preview = match (source.as_deref(), v.get("preview")) {
                (Some(source_id), Some(preview)) if !preview.is_null() => {
                    let node = preview.get("node").filter(|n| !n.is_null());
                    let edge = preview.get("edge").filter(|e| !e.is_null());
                    match (node, edge) {
                        (Some(node), Some(edge)) => Self::brush_preview_snapshot_from_session_json(node, edge, source_id),
                        _ => None,
                    }
                }
                _ => None,
            };
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.brush_slot_suggestions_active = v.get("suggestionsActive").and_then(|x| x.as_bool()).unwrap_or(false);
            if self.brush_preview.is_none() && !self.brush_candidates.is_empty() {
                self.brush_rebuild_preview();
            } else {
                self.brush_sync_preview_events();
            }
            self.bump_content_scene_generation();
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1152:        h.set_brush_session_mirror_json(&session.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:431:        self.state.borrow_mut().host.set_brush_session_mirror_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:436:        let _ = self.state.borrow_mut().host.set_brush_session_mirror_json("");`

## set_drop_preview_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:7906` through line 7919.
- Exact body SHA-256: `45258b80e5be5a8e4e09902b49a7a41e97683d04c20c368a18a9fea2a62e3110`.
- Explicit codec calls: `serde_json::from_str`.

```rust
pub fn set_drop_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            if json.trim().is_empty() {
                self.drop_preview = None;
                self.bump_content_scene_generation();
                return Ok(());
            }
            let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::DropPreviewJson)?;
            self.drop_preview = self.drop_preview_from_json(&v);
            if self.drop_preview.is_none() {
                return Err(NormalPortError::DropPreviewInvalid);
            }
            self.bump_content_scene_generation();
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1080:        h.set_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1084:        h.set_drop_preview_json("").unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1108:        h.set_drop_preview_json(r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1111:        h.set_drop_preview_json("").unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:441:        self.state.borrow_mut().host.set_drop_preview_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:446:        let _ = self.state.borrow_mut().host.set_drop_preview_json("");`

## drain_events_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8166` through line 8184.
- Exact body SHA-256: `87bb87b3f603da67e1e65bca628a40559dd53ded1d04b19bcaeb4607f577b12d`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn drain_events_json(&mut self) -> String {
            let mut out = String::from("[");
            let mut first = true;
            while let Some(event) = self.events.pop() {
                if !first {
                    out.push(',');
                }
                first = false;
                event.write_json(&mut out);
            }
            if let Some(event) = self.event_overflow.take() {
                if !first {
                    out.push(',');
                }
                event.write_json(&mut out);
            }
            out.push(']');
            out
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:370:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:373:        let ev2 = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:413:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:416:        let fastened = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:419:        assert!(!h.drain_events_json().contains("brushPlace"), "an empty page must place nothing");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:422:        let free = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:487:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:495:        let ev_commit = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:498:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:500:        let ev_cancel = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:527:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:530:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:534:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:561:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:564:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:566:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1081:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1135:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1153:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1193:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1196:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1234:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1237:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1396:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1401:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1437:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1440:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1470:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1476:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1532:        let _ = host.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:1536:        let events: semio_framework_pack_json::Value = semio_framework_pack_json::from_json_str(&host.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("board events json");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:17:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:24:        let rows: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:37:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:44:        let rows: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:55:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:65:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:172:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:184:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:195:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:199:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:209:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:361:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:371:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:405:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:412:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:458:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:468:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:492:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:502:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:514:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:524:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:547:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:562:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:575:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:589:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:604:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:637:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:670:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:676:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:684:        let ev_end = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:762:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:772:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:827:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:837:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:849:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:859:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:871:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:876:        assert!(!h.drain_events_json().contains("edgeCreate"));`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:886:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:900:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:903:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:915:        let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🧪️tests/🔬️unit/🦀️.rs:921:        let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:347:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:352:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:362:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:368:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:372:    let end: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::from_json_str(&h.drain_events_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("release rows");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:458:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:461:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:465:    let ev2 = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:490:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:492:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:497:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:500:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:654:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:660:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:693:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:700:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:736:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:743:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:786:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:793:    let ev = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:857:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:934:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:982:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:988:    let mid = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:996:    let fin = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1063:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1104:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1106:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1115:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1120:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1127:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1130:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1165:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1172:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1178:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1227:    let _ = h.drain_events_json();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:1231:    let _ = h.drain_events_json();`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs:4077:        self.state.borrow_mut().host.drain_events_json()`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs:563:    let events: Vec<serde_json::Value> = serde_json::from_str(&host.drain_events_json()).expect("events");`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs:1090:    assert_eq!(direct.drain_events_json(), planned.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:900:        let events = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:952:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:958:    let during = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:962:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:982:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:989:        frames += board_event_names(&host.drain_events_json()).len();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:994:    assert_eq!(board_event_names(&host.drain_events_json()), vec!["gesture".to_string()], "and the whole gesture commits once");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1005:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1054:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1062:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1075:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1084:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1113:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1119:    let names = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1130:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1142:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1152:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1162:    let names = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1296:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1300:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a paint frame announces nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1303:    let released = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1320:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1323:    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1331:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a rectangle under the extent floor commits nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1341:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1343:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "the press stages its selection and publishes nothing yet");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1345:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "a region drag frame announces nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1347:    let moved = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1357:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1362:    let resized = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1371:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1374:    assert_eq!(board_event_names(&host.drain_events_json()), vec!["select".to_string()], "a release that moved nothing publishes only the selection");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1378:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1383:    assert_eq!(board_event_names(&host.drain_events_json()), Vec::<String>::new(), "and publishes nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1395:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1399:    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1406:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1410:    let payload = region_event_payloads(&host.drain_events_json(), "regionCreate").remove(0);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1420:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1424:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1427:    let names = board_event_names(&host.drain_events_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1462:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1466:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1574:    let _ = host.drain_events_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1581:    serde_json::from_str::<Vec<serde_json::Value>>(&host.drain_events_json()).expect("events parse").into_iter().filter(|row| row["name"] != "hover").collect()`

## interaction_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8966` through line 9014.
- Exact body SHA-256: `01efa66d101da21f7c9698c6957159f7f94078cdd4742ba8803ea8c302055121`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn interaction_json(&self) -> String {
            let mode = if self.region_paint.is_some() {
                "regionPaint"
            } else if self.region_drag.is_some() {
                "regionDrag"
            } else if self.transform_drag.is_some() {
                "transformRotate"
            } else {
                match self.interaction {
                    Interaction::None => "none",
                    Interaction::Pan { .. } => "pan",
                    Interaction::DragNodes { .. } => "dragNodes",
                    Interaction::SelectionPending { .. } => "selectionPending",
                    Interaction::Selection { .. } => "selection",
                    Interaction::LinkAtSourceHandle { .. } => "linkAtSourceHandle",
                    Interaction::LinkDragSnap { .. } => "linkDragSnap",
                    Interaction::LinkTargetNode { .. } => "linkTargetNode",
                    Interaction::ExternalLinkPreview { .. } => "externalLinkPreview",
                }
            };
            let utility = match self.active_utility {
                ActiveUtility::Select => "select",
                ActiveUtility::Brush => "brush",
                ActiveUtility::AreaBrush => "areaBrush",
            };
            let mut out = String::from("{\"mode\":\"");
            out.push_str(mode);
            out.push_str("\",\"utility\":\"");
            out.push_str(utility);
            out.push_str("\",\"hoveredId\":");
            match self.hovered_id.as_deref() {
                Some(id) => {
                    out.push('"');
                    out.push_str(&id.replace('\\', "\\\\").replace('"', "\\\""));
                    out.push('"');
                }
                None => out.push_str("null"),
            }
            out.push_str(",\"selectionCount\":");
            out.push_str(&self.selection.len().to_string());
            out.push_str(",\"preselectCount\":");
            out.push_str(&self.preselect.len().to_string());
            out.push_str(",\"revision\":");
            out.push_str(&self.interaction_revision.to_string());
            out.push_str(",\"deferringDescriptorSync\":");
            out.push_str(if self.defers_descriptor_sync_from_js() { "true" } else { "false" });
            out.push('}');
            out
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:379:        self.state.borrow().host.interaction_json()`

## handle_positions_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:9025` through line 9074.
- Exact body SHA-256: `dd83afb4c0ad58ffc7d67cecdd6326ed8e2344f04dffd7edffad871dadc2ae2d`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn handle_positions_json(&self) -> String {
            let (w, h) = (f64::from(self.width), f64::from(self.height));
            let mut rows: Vec<(&str, Point, &str, &str, bool)> = Vec::new();
            let mut on_screen = 0usize;
            for (id, handle) in &self.handles {
                let Some(world) = self.handle_world_pos(handle) else { continue };
                let screen = self.world_to_screen(world);
                if screen.x < 0.0 || screen.y < 0.0 || screen.x > w || screen.y > h || !self.handle_effectively_visible(id.as_str()) {
                    continue;
                }
                on_screen += 1;
                if rows.len() < BOARD_HANDLE_VITALS_CAP {
                    rows.push((id.as_str(), world, handle.node_id.as_str(), handle.handle_kind.as_str(), !self.handle_has_incident_edge(id.as_str())));
                }
            }
            let quote = |value: &str, out: &mut String| {
                out.push('"');
                out.push_str(&value.replace('\\', "\\\\").replace('"', "\\\""));
                out.push('"');
            };
            let mut out = String::from("{\"total\":");
            out.push_str(&self.handles.len().to_string());
            out.push_str(",\"onScreen\":");
            out.push_str(&on_screen.to_string());
            out.push_str(",\"published\":");
            out.push_str(&rows.len().to_string());
            out.push_str(",\"capped\":");
            out.push_str(if on_screen > rows.len() { "true" } else { "false" });
            out.push_str(",\"rows\":[");
            for (index, (id, world, node_id, handle_kind, open)) in rows.iter().copied().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push('[');
                quote(id, &mut out);
                out.push(',');
                out.push_str(&world.x.to_string());
                out.push(',');
                out.push_str(&world.y.to_string());
                out.push(',');
                quote(node_id, &mut out);
                out.push(',');
                quote(handle_kind, &mut out);
                out.push(',');
                out.push_str(if open { "true" } else { "false" });
                out.push(']');
            }
            out.push_str("]}");
            out
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:386:        self.state.borrow().host.handle_positions_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1201:    let json = host.handle_positions_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1208:    let row = host.handle_positions_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1212:    let away = host.handle_positions_json();`

## pick_targets_at_screen_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:9863` through line 9866.
- Exact body SHA-256: `f4e845c21f3db777cfc693f53b2d3b5511fd5f67c116c846036fb0314c641ace`.
- Explicit codec calls: `serde_json::to_string`.

```rust
pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
            let world = self.screen_to_world(Point::new(sx, sy));
            serde_json::to_string(&self.resolve_pick_targets_world(world)).unwrap_or_else(|_| "[]".into())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:251:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🦀️.rs:1423:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:668:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:681:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs:145:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| host.pick_targets_at_screen_json(0.0, 0.0))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4199:        NodeGraphEngine::Flow(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4200:        NodeGraphEngine::Dag(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5369:        let json = host.pick_targets_at_screen_json(sx, sy);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6191:        let hits: Vec<Value> = serde_json::from_str(&host.pick_targets_at_screen_json(sx, sy)).ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6236:                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6237:                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6255:    let hits = with_board_host(surface_id, |host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6267:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6294:            Some((pick_target_hits(&host.pick_targets_at_screen_json(sx, sy)), Some(text)))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:544:        self.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:865:            self.state.borrow().host.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:564:    let json = host.pick_targets_at_screen_json(200.0, 200.0);`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs:1267:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:116:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(300.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:134:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(200.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:142:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(0.0, 0.0)).expect("json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1442:    let targets = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-260.0, 0.0)).x, host.world_to_screen(Point::new(-260.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1444:    let visible = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-50.0, 0.0)).x, host.world_to_screen(Point::new(-50.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4527:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.pick_targets_at_screen_json(number(args, "sx")?, number(args, "sy")?).into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1797:        self.dag.pick_targets_at_screen_json(sx, sy)`

## set_node_positions_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:10259` through line 10270.
- Exact body SHA-256: `8d349fb403e725eae3912b483285f5756984ac4a90cf7aacd2f0c2c8a1618dbc`.
- Explicit codec calls: `serde_json::from_str`.

```rust
pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            #[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
            struct NodePositionMoveJson {
                id: String,
                x: f64,
                y: f64,
            }
            let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json)?;
            let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
            self.set_node_positions(&moves);
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs:391:    h.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:201:        self.state.borrow_mut().host.set_node_positions_json(json).map_err(|e| JsValue::from_str(&e.to_string()))`

## highlighted_ids_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:11201` through line 11203.
- Exact body SHA-256: `238cacf8af5902748adf33976263e2c9b684b9f9c5630634ac07da289801004f`.
- Explicit codec calls: `serde_json::to_string`.

```rust
pub fn highlighted_ids_json(&self) -> Result<String, NormalPortError> {
            Ok(serde_json::to_string(&self.highlighted_ids.iter().cloned().collect::<Vec<_>>())?)
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:111:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:115:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["left","mid"]"#, "a new preview keeps what the draft references");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs:118:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), "[]", "a closed draft highlights nothing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1716:    assert_eq!(host.highlighted_ids_json().expect("highlighted ids"), r#"["node-a","node-b","node-locked"]"#, "a preview repaint keeps what the draft references");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1719:    assert_eq!((style(&host, "node-a"), host.highlighted_ids_json().expect("highlighted ids")), (BoardElementStyleKind::Neutral, "[]".to_string()), "the empty set clears it");`

## transform_gumball_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:13180` through line 13202.
- Exact body SHA-256: `6da46939d221faa3dfc5405f8d62502c73e9ccb5c2c68d96f1e72db8f87270c1`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn transform_gumball_json(&self) -> String {
            let geometry = self.transform_drag.as_ref().map(|drag| (drag.pivot, drag.radius_world)).or_else(|| self.transform_gumball_geometry());
            let mut out = String::from("{\"move\":");
            out.push_str(if self.transform_flags.move_enabled { "true" } else { "false" });
            out.push_str(",\"rotate\":");
            out.push_str(if self.transform_flags.rotate_enabled { "true" } else { "false" });
            out.push_str(",\"ringVisible\":");
            out.push_str(if geometry.is_some() { "true" } else { "false" });
            out.push_str(",\"dragging\":");
            out.push_str(if self.transform_drag.is_some() { "true" } else { "false" });
            out.push_str(",\"radians\":");
            out.push_str(&self.transform_drag.as_ref().map_or(0.0, |drag| drag.radians).to_string());
            if let Some((pivot, radius)) = geometry {
                out.push_str(",\"pivot\":{\"x\":");
                out.push_str(&pivot.x.to_string());
                out.push_str(",\"y\":");
                out.push_str(&pivot.y.to_string());
                out.push_str("},\"radius\":");
                out.push_str(&radius.to_string());
            }
            out.push('}');
            out
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs:208:    let gumball: Value = semio_framework_pack_json::from_json_str(&host.transform_gumball_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("gumball vitals parse");`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:360:        self.state.borrow().host.transform_gumball_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1176:    assert!(host.transform_gumball_json().contains("\"ringVisible\":false"), "and the vitals say so: {}", host.transform_gumball_json());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1188:    assert!(host.transform_gumball_json().contains("\"rotate\":true"), "the vitals name the composed handles: {}", host.transform_gumball_json());`

## target_regions_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:13372` through line 13404.
- Exact body SHA-256: `1bb291d9a55415adc6f5cb87068998d5c831cf7d094752d08df1ca5402d680ca`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn target_regions_json(&self) -> String {
            let mut out = String::from("[");
            for (index, region) in self.regions.values().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                let [min_x, min_y, max_x, max_y] = self.region_live_bounds(region);
                out.push_str("{\"id\":\"");
                out.push_str(&region.id.replace('\\', "\\\\").replace('"', "\\\""));
                out.push_str("\",\"x\":");
                out.push_str(&min_x.to_string());
                out.push_str(",\"y\":");
                out.push_str(&min_y.to_string());
                out.push_str(",\"width\":");
                out.push_str(&(max_x - min_x).to_string());
                out.push_str(",\"height\":");
                out.push_str(&(max_y - min_y).to_string());
                if let Some(label) = region.label.as_deref() {
                    out.push_str(",\"label\":\"");
                    out.push_str(&label.replace('\\', "\\\\").replace('"', "\\\""));
                    out.push('"');
                }
                out.push_str(",\"hidden\":");
                out.push_str(if region.hidden { "true" } else { "false" });
                out.push_str(",\"locked\":");
                out.push_str(if region.locked { "true" } else { "false" });
                out.push_str(",\"selected\":");
                out.push_str(if region.selected { "true" } else { "false" });
                out.push('}');
            }
            out.push(']');
            out
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:373:        self.state.borrow().host.target_regions_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1277:    let published = host.target_regions_json();`

## apply_dag_layout_to_host_snapshot_v1_value

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1175` through line 1289.
- Exact body SHA-256: `542e0328ead91019e0a40284bfc04926102877ae39e5718d39bec51557a90ee8`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn apply_dag_layout_to_host_snapshot_v1_value(snapshot: &mut Value, opts: &DagLayoutOptions) -> Result<(), DagError> {
    let Some(root) = snapshot.as_object_mut() else {
        return Err(DagError::SnapshotRootNotObject);
    };
    if root.get("schema").and_then(|v| v.as_str()) != Some("dag.hostDocument") {
        return Err(DagError::SchemaMismatch);
    }
    let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
        return Err(DagError::NodesMissing);
    };
    if nodes.is_empty() {
        return Ok(());
    }
    let mut handle_to_node: HashMap<String, String> = HashMap::new();
    let mut node_ids: HashSet<String> = HashSet::new();
    for node in nodes.iter() {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        node_ids.insert(nid.to_string());
        if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
            for h in handles {
                if let Some(hid) = h.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
    }
    let mut directed: Vec<(String, String)> = Vec::new();
    for e in &edges_json {
        let Some(eo) = e.as_object() else {
            continue;
        };
        let src = eo.get("source").and_then(|v| v.as_str()).or_else(|| eo.get("sourceHandle").and_then(|v| v.as_str()));
        let tgt = eo.get("target").and_then(|v| v.as_str()).or_else(|| eo.get("targetHandle").and_then(|v| v.as_str()));
        let (Some(src_h), Some(tgt_h)) = (src, tgt) else {
            continue;
        };
        let u = resolve_layout_node_id(&handle_to_node, src_h, &node_ids);
        let v = resolve_layout_node_id(&handle_to_node, tgt_h, &node_ids);
        if u != v && node_ids.contains(&u) && node_ids.contains(&v) {
            directed.push((u, v));
        }
    }
    let mut incoming: HashMap<String, u32> = HashMap::new();
    for id in &node_ids {
        incoming.insert(id.clone(), 0);
    }
    for (_, v) in &directed {
        *incoming.entry(v.clone()).or_insert(0) += 1;
    }
    let roots: Vec<String> = node_ids.iter().filter(|id| incoming.get(*id).copied().unwrap_or(0) == 0).cloned().collect();
    let roots = if roots.is_empty() { node_ids.iter().cloned().collect() } else { roots };
    let mut depth: HashMap<String, i32> = HashMap::new();
    for r in &roots {
        depth.insert(r.clone(), 0);
    }
    for _ in 0..directed.len().saturating_add(node_ids.len()).saturating_add(4) {
        let mut changed = false;
        for (u, v) in &directed {
            let Some(&du) = depth.get(u) else {
                continue;
            };
            let nd = du + 1;
            if depth.get(v).copied().unwrap_or(-1) < nd {
                depth.insert(v.clone(), nd);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let pos = buchheim_positions(&roots, &directed, &depth);
    let mut minx = f64::INFINITY;
    let mut maxx = f64::NEG_INFINITY;
    let mut miny = f64::INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    for (x, y) in pos.values() {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    let cx = (minx + maxx) * 0.5;
    let cy = (miny + maxy) * 0.5;
    let gx = opts.center_x.unwrap_or(0.0);
    let gy = opts.center_y.unwrap_or(0.0);
    let (dx, dy) = match opts.orientation {
        DagLayoutOrientation::LeftRight => (gx - cy * opts.layer_spacing, gy - cx * opts.sibling_gap),
        DagLayoutOrientation::TopBottom => (gx - cx * opts.sibling_gap, gy - cy * opts.layer_spacing),
    };
    for node in nodes.iter_mut() {
        let Some(obj) = node.as_object_mut() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some((bx, by)) = pos.get(nid) else {
            continue;
        };
        let (nx, ny) = match opts.orientation {
            DagLayoutOrientation::LeftRight => (by * opts.layer_spacing + dx, bx * opts.sibling_gap + dy),
            DagLayoutOrientation::TopBottom => (bx * opts.sibling_gap + dx, by * opts.layer_spacing + dy),
        };
        obj.insert("x", Value::from(nx));
        obj.insert("y", Value::from(ny));
    }
    Ok(())
}
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4837:        apply_dag_layout_to_host_snapshot_v1_value(&mut snapshot_value, opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4861:            let _ = apply_dag_layout_to_host_snapshot_v1_value(&mut snapshot_value, &DagLayoutOptions::default());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:165:    apply_dag_layout_to_host_snapshot_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:175:    apply_dag_layout_to_host_snapshot_v1_value(&mut fixture, &opts).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:184:    apply_dag_layout_to_host_snapshot_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:187:    apply_dag_layout_to_host_snapshot_v1_value(&mut wide, &DagLayoutOptions { layer_spacing: 240.0, sibling_gap: 80.0, ..DagLayoutOptions::default() }).unwrap();`

## dag_lod_scale_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1505` through line 1514.
- Exact body SHA-256: `9d392fc17398f70e0025d3af5b1f0ecb4fd7889e753ad012f120a2b4457113e2`.
- Explicit codec calls: `semio_framework_pack_json::object`, `semio_framework_pack_json::to_string`.

```rust
pub fn dag_lod_scale_json() -> String {
    let rows: Vec<Value> = DAG_LODS
        .iter()
        .map(|lod| {
            let max_zoom = if lod.max_zoom.is_finite() { lod.max_zoom + DAG_LOD_ZOOM_SHIFT } else { lod.max_zoom };
            semio_framework_pack_json::object([("id".to_string(), Value::from(lod.id)), ("name".to_string(), Value::from(lod.name)), ("description".to_string(), Value::from(lod.description)), ("maxZoom".to_string(), lod_max_zoom_json(max_zoom))])
        })
        .collect();
    semio_framework_pack_json::to_string(&Value::Array(rows))
}
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:78:            SEQUENCE_OPERATION_LOD_SCALE => Ok(infinite_board_port_directed_dag::board::ports::directed_dag::dag_lod_scale_json().into_bytes()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:829:            dag::dag_lod_scale_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7028:            dag_lod_scale_json()`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🦀️.rs:14:    items.extend(serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:3907:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(dag::dag_lod_scale_json().into_bytes()) };`

## dag_graph_edit_rows_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2207` through line 2214.
- Exact body SHA-256: `0c8befbb47b1ddceb92ca2d68021ecd310bae3fca4abe33fb1cc526c10c9a1e8`.
- Explicit codec calls: `semio_framework_pack_json::object`, `semio_framework_pack_json::to_string`, `semio_framework_pack_json::array`.

```rust
pub fn dag_graph_edit_rows_json(edits: &[DagGraphEdit], refused: Option<DagJournalRefusal>) -> String {
    let mut rows = DagGraphEditJsonRows::default();
    if let Err(never) = write_dag_graph_edit_rows(edits, &mut rows) {
        match never {}
    }
    let refused = refused.map(|refusal| ("refused".to_string(), semio_framework_pack_json::object([("rows".to_string(), Value::from(refusal.rows)), ("limit".to_string(), Value::from(refusal.limit))])));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object(std::iter::once(("operations".to_string(), semio_framework_pack_json::array(rows.rows))).chain(refused)))
}
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts:169:    expect(answerBody).toContain("dag::dag_graph_edit_rows_json(&edits, refusal)");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:513:        dag::dag_graph_edit_rows_json(&edits, refusal)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:68:    let rows: Value = serde_json::from_str(&dag_graph_edit_rows_json(edits, None)).expect("journal rows json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:45:    let encoded: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:47:    assert_eq!(dag_graph_edit_rows_json(&[], None), r#"{"operations":[]}"#, "an empty journal is no row");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:116:    let answer: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs:127:    let refused: Value = serde_json::from_str(&dag_graph_edit_rows_json(&[], Some(refusal))).expect("refusal json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1328:        dag::dag_graph_edit_rows_json(&edits, refusal)`

## selection_domains_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3617` through line 3626.
- Exact body SHA-256: `31cb70df65219d28c218472e23a4a9711738d7355a1b037319d4bc4e325b5579`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn selection_domains_json(&self) -> String {
        #[derive(ToValue, FromValue)]
        struct Domains {
            nodes: Vec<String>,
            edges: Vec<String>,
            handles: Vec<String>,
        }
        let handles: Vec<String> = self.selected_channels().into_iter().map(|channel| format!("{}@{}", channel.widget_id, channel.port)).collect();
        semio_framework_pack_json::to_json_string(&Domains { nodes: self.selected_node_ids(), edges: self.selected_edge_ids(), handles })
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6236:                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6237:                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1867:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_domains_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1787:        self.dag.selection_domains_json()`

## set_selection_domains_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3655` through line 3668.
- Exact body SHA-256: `6cd811dc8279f077d5f66bd51672b4f8c54671ffbc0c850762a74d700f422246`.
- Explicit codec calls: `semio_framework_pack_json::from_json_str`.

```rust
pub fn set_selection_domains_json(&mut self, json: &str) {
        #[derive(Default, ToValue, FromValue)]
        struct Domains {
            nodes: Vec<String>,
            edges: Vec<String>,
            handles: Vec<String>,
        }
        if let Ok(domains) = semio_framework_pack_json::from_json_str::<Domains>(json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
            self.apply_selection_domains(&domains.nodes, &domains.edges, &domains.handles);
            return;
        }
        let ids: Vec<String> = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
        self.set_selection(&ids);
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:43:    host.set_selection_domains_json(&format!("{{\"nodes\":[],\"edges\":[{edge:?}],\"handles\":[]}}"));`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2618:    host.dag.set_selection_domains_json(&json.to_string());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1818:        self.dag.set_selection_domains_json(json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1828:        self.dag.set_selection_domains_json(&json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:2179:    host.dag.set_selection_domains_json(r#"{"nodes":[],"edges":["s1"],"handles":[]}"#);`

## selected_channels_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3722` through line 3724.
- Exact body SHA-256: `1cef0608e058c46b5c8719e2243238e1e61022b2e925299dce6948a54f52a0f8`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn selected_channels_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.selected_channels())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1984:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selected_channels_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1813:        self.dag.selected_channels_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1964:    let selected: Vec<dag::DagChannelRef> = semio_framework_pack_json::from_json_str(&host.selected_channels_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`

## wire_type_refusal_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3732` through line 3747.
- Exact body SHA-256: `6b9b52e7a4862a2d8f28ffb46400924808d43e7fe2af282a838b5c53b365257a`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn wire_type_refusal_json(&self) -> String {
        let Some((source_hid, target_hid)) = self.engine.wire_drag_type_refusal() else {
            return "null".into();
        };
        let (Some(source_key), Some(target_key)) = (self.handle_key_map.get(&source_hid), self.handle_key_map.get(&target_hid)) else {
            return "null".into();
        };
        let types = |handle_id: u64| self.engine.handles.get(&handle_id).map(|handle| handle.value_types.clone()).unwrap_or_default();
        format!(
            "{{\"source\":{},\"sourceTypes\":{},\"target\":{},\"targetTypes\":{}}}",
            semio_framework_pack_json::to_json_string(source_key),
            semio_framework_pack_json::to_json_string(&types(source_hid)),
            semio_framework_pack_json::to_json_string(target_key),
            semio_framework_pack_json::to_json_string(&types(target_hid))
        )
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3753:        let refusal = self.wire_type_refusal_json();`

## hovered_channel_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3752` through line 3763.
- Exact body SHA-256: `11512d1d101d7ba18667a3a07371658e7ed306fa5256fdaa2cda0f37c83c0595`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn hovered_channel_json(&self) -> String {
        let refusal = self.wire_type_refusal_json();
        match (self.hovered_channel(), refusal.as_str()) {
            (None, "null") => "null".into(),
            (None, _) => format!("{{\"refusal\":{refusal}}}"),
            (Some(channel), "null") => semio_framework_pack_json::to_json_string(&channel),
            (Some(channel), _) => {
                let encoded = semio_framework_pack_json::to_json_string(&channel);
                format!("{}{}{}", &encoded[..encoded.len().saturating_sub(1)], format_args!(",\"refusal\":{refusal}"), "}")
            }
        }
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:390:        self.dag.hovered_channel_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:814:            self.state.borrow().host.hovered_channel_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:319:    assert_eq!(host.hovered_channel_json(), "null");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1945:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.hovered_channel_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1808:        self.dag.hovered_channel_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1959:    let hovered: dag::DagChannelRef = semio_framework_pack_json::from_json_str(&host.hovered_channel_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`

## screen_geometry_census_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3773` through line 3791.
- Exact body SHA-256: `16eb851e963c8152e170ec96b9b8ae26f86f508785aaaeabc2f315c5647654b0`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn screen_geometry_census_json(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        for node in &self.host_snapshot.nodes {
            let body = match self.entity_screen_rect("node", &node.id) {
                Some(rect) => self.first_screen_point_in(rect, |hit| hit.is_draggable_body() && hit.node_id.as_deref() == Some(node.id.as_str())),
                None => None,
            };
            let body_json = body.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"));
            rows.push(format!("{{\"kind\":\"node\",\"id\":{},\"body\":{body_json},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&node.id), self.entity_screen_json("node", &node.id)));
            for (port, direction) in node.inputs().iter().map(|port| (port, "in")).chain(node.outputs().iter().map(|port| (port, "out"))) {
                let channel = format!("{}@{}", node.id, port.id);
                rows.push(format!("{{\"kind\":\"handle\",\"direction\":\"{direction}\",\"id\":{},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&channel), self.entity_screen_json("handle", &channel)));
            }
        }
        let panel = [0.0, 0.0, self.width as f64, self.height as f64];
        let navigate = self.first_screen_point_in(panel, |hit| hit.minimap && !hit.minimap_viewport);
        rows.push(format!("{{\"kind\":\"minimap\",\"navigate\":{}}}", navigate.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"))));
        format!("[{}]", rows.join(","))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4657:            NodeGraphEngine::Flow(host) => host.dag.screen_geometry_census_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4658:            NodeGraphEngine::Dag(host) => host.dag.screen_geometry_census_json(),`

## pick_targets_at_screen_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3829` through line 3858.
- Exact body SHA-256: `e0c00b6723e31d2978c298fb1ba0c6aa01675f0a6ba636247db0a880c939ca93`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        #[derive(ToValue, FromValue)]
        struct Row {
            domain: String,
            id: String,
            generality: u32,
            #[value(default, skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        }
        let world = self.screen_to_world_point(sx, sy);
        let targets = self.engine.hit_test_pick_targets(world);
        let rows: Vec<Row> = targets
            .into_iter()
            .filter_map(|target| {
                let row = match target.domain.as_str() {
                    "node" => self.widget_id_for_node_id(target.id).map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "edge" => self.edge_id_map.get(&target.id).cloned().map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "handle" => self.decode_channel_ref(target.id).map(|channel| Row {
                        domain: "handle".into(),
                        id: format!("{}@{}", channel.widget_id, channel.port),
                        generality: target.generality,
                        label: Some(format!("{} · {}", channel.widget_id, channel.port)),
                    }),
                    _ => None,
                };
                row
            })
            .collect();
        semio_framework_pack_json::to_json_string(&rows)
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs:251:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🦀️.rs:1423:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:668:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🔨️modules/✍️editor/🧪️tests/🔬️unit/🦀️.rs:681:    let json = host.pick_targets_at_screen_json(screen["x"].as_f64().unwrap(), screen["y"].as_f64().unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs:145:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| host.pick_targets_at_screen_json(0.0, 0.0))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4199:        NodeGraphEngine::Flow(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4200:        NodeGraphEngine::Dag(host) => host.pick_targets_at_screen_json(sx, sy),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5369:        let json = host.pick_targets_at_screen_json(sx, sy);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6191:        let hits: Vec<Value> = serde_json::from_str(&host.pick_targets_at_screen_json(sx, sy)).ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6236:                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6237:                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains_json()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6255:    let hits = with_board_host(surface_id, |host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6267:        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6294:            Some((pick_target_hits(&host.pick_targets_at_screen_json(sx, sy)), Some(text)))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:544:        self.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:865:            self.state.borrow().host.dag.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:564:    let json = host.pick_targets_at_screen_json(200.0, 200.0);`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs:1267:        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:116:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(300.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:134:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(200.0, 200.0)).expect("json");`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:142:    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(0.0, 0.0)).expect("json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1442:    let targets = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-260.0, 0.0)).x, host.world_to_screen(Point::new(-260.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs:1444:    let visible = host.pick_targets_at_screen_json(host.world_to_screen(Point::new(-50.0, 0.0)).x, host.world_to_screen(Point::new(-50.0, 0.0)).y);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4527:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.pick_targets_at_screen_json(number(args, "sx")?, number(args, "sy")?).into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1797:        self.dag.pick_targets_at_screen_json(sx, sy)`

## selection_preview_points_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3878` through line 3881.
- Exact body SHA-256: `f8f82cfa9ebc714a3e016be6538b80ab5df2775d290b5464a2074ef4a24303aa`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn selection_preview_points_json(&self) -> String {
        let points: Vec<[f64; 2]> = self.engine.selection_preview_points().iter().map(|p| [p.x, p.y]).collect();
        semio_framework_pack_json::to_json_string(&points)
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:103:            SEQUENCE_OPERATION_SELECTION_PREVIEW_POINTS => Ok(self.host.dag.selection_preview_points_json().into_bytes()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4908:                preview_points_json: host.selection_preview_points_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4914:                preview_points_json: host.dag.selection_preview_points_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:789:            self.state.borrow().host.dag.selection_preview_points_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:806:    let preview_points: Vec<[f64; 2]> = semio_framework_pack_json::from_json_str(&host.selection_preview_points_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4735:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_preview_points_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1336:        self.dag.selection_preview_points_json()`

## selection_union_bounds_screen_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4110` through line 4138.
- Exact body SHA-256: `04705c374216bbb6f2b78ca83b13df6a51ed2daf48fb9498ae50432cf26d3b3c`.
- Explicit codec calls: `semio_framework_pack_json::to_string`, `semio_framework_pack_json::object`.

```rust
pub fn selection_union_bounds_screen_json(&self) -> String {
        let selected = self.selected_snapshot_nodes();
        if selected.is_empty() {
            return "null".into();
        }
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let pad_world = 4.0 / self.host_snapshot.camera.zoom.max(0.05);
        let mut corners = Vec::new();
        for (_, node) in &selected {
            let hw = node.width * 0.5 + pad_world;
            let hh = node.height * 0.5 + pad_world;
            corners.push(Point::new(node.x - hw, node.y - hh));
            corners.push(Point::new(node.x + hw, node.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return "null".into();
        };
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let tl = world_to_screen(&cam, &viewport, Point::new(bounds.min_x, bounds.min_y));
        let br = world_to_screen(&cam, &viewport, Point::new(bounds.max_x, bounds.max_y));
        semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("x".to_string(), Value::from(tl.x)),
            ("y".to_string(), Value::from(tl.y)),
            ("width".to_string(), Value::from((br.x - tl.x).max(1.0))),
            ("height".to_string(), Value::from((br.y - tl.y).max(1.0))),
        ]))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:106:            SEQUENCE_OPERATION_SELECTION_BOUNDS => Ok(self.host.dag.selection_union_bounds_screen_json().into_bytes()),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4911:                selection_bounds_json: host.selection_union_bounds_screen_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4917:                selection_bounds_json: host.dag.selection_union_bounds_screen_json(),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:784:            self.state.borrow().host.dag.selection_union_bounds_screen_json()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:845:    let json = host.selection_union_bounds_screen_json();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4852:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.selection_union_bounds_screen_json().into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1833:        self.dag.selection_union_bounds_screen_json()`

## entity_screen_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4150` through line 4254.
- Exact body SHA-256: `94e4d960d4fe0dd6a987704144af40926e1145244377d0c222061aa95074c97d`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        #[derive(ToValue, FromValue)]
        struct EntityGeometry {
            visible: bool,
            #[value(default, skip_serializing_if = "Option::is_none")]
            x: Option<f64>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            y: Option<f64>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            rect: Option<[f64; 4]>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            polyline: Option<Vec<[f64; 2]>>,
        }
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let unresolved = EntityGeometry { visible: false, x: None, y: None, rect: None, polyline: None };
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let viewport_center = (self.width as f64 * 0.5, self.height as f64 * 0.5);

        let world_rect_to_screen = |min_x: f64, min_y: f64, max_x: f64, max_y: f64| -> ([f64; 4], (f64, f64)) {
            let tl = world_to_screen(&cam, &viewport, Point::new(min_x, min_y));
            let br = world_to_screen(&cam, &viewport, Point::new(max_x, max_y));
            ([tl.x, tl.y, (br.x - tl.x).max(1.0), (br.y - tl.y).max(1.0)], ((tl.x + br.x) * 0.5, (tl.y + br.y) * 0.5))
        };
        let handle_world_bounds = |widget_id: &str, port: &str| -> Option<(f64, f64, f64, f64)> {
            let node = self.host_snapshot.nodes.iter().find(|node| node.id == widget_id)?;
            if let Some(index) = node.inputs().iter().position(|candidate| candidate.id == port) {
                return input_port_connector_bounds(node, index);
            }
            let index = node.outputs().iter().position(|candidate| candidate.id == port)?;
            output_port_connector_bounds(node, index)
        };
        let nearest_center_screen = |candidates: &[(f64, f64, f64, f64)]| -> Option<(f64, f64, f64, f64)> {
            let mut best: Option<((f64, f64, f64, f64), f64)> = None;
            for &bounds in candidates {
                let (_, screen_center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
                let distance = (screen_center.0 - viewport_center.0).hypot(screen_center.1 - viewport_center.1);
                if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                    best = Some((bounds, distance));
                }
            }
            best.map(|(bounds, _)| bounds)
        };

        let bounds_result: Option<(f64, f64, f64, f64)> = match domain {
            "node" => {
                if id == "*" {
                    let all: Vec<(f64, f64, f64, f64)> = self
                        .host_snapshot
                        .nodes
                        .iter()
                        .map(|node| {
                            let b = Self::dag_node_world_bounds(node);
                            (b.min_x, b.min_y, b.max_x, b.max_y)
                        })
                        .collect();
                    nearest_center_screen(&all)
                } else {
                    self.host_snapshot.nodes.iter().find(|node| node.id == id).map(|node| {
                        let b = Self::dag_node_world_bounds(node);
                        (b.min_x, b.min_y, b.max_x, b.max_y)
                    })
                }
            }
            "handle" => {
                if id == "*" {
                    let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
                    for node in &self.host_snapshot.nodes {
                        for port in node.inputs().iter().chain(node.outputs().iter()) {
                            if let Some(bounds) = handle_world_bounds(&node.id, &port.id) {
                                all.push(bounds);
                            }
                        }
                    }
                    nearest_center_screen(&all)
                } else {
                    id.split_once('@').and_then(|(widget_id, port)| handle_world_bounds(widget_id, port))
                }
            }
            "edge" => {
                let edge = if id == "*" { self.host_snapshot.edges.first() } else { self.host_snapshot.edges.iter().find(|edge| edge.id == id) };
                let Some(edge) = edge else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some((source_widget, source_port)) = edge.source.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some((target_widget, target_port)) = edge.target.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some(source_bounds) = handle_world_bounds(source_widget, source_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some(target_bounds) = handle_world_bounds(target_widget, target_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let (_, source_center) = world_rect_to_screen(source_bounds.0, source_bounds.1, source_bounds.2, source_bounds.3);
                let (_, target_center) = world_rect_to_screen(target_bounds.0, target_bounds.1, target_bounds.2, target_bounds.3);
                let midpoint = ((source_center.0 + target_center.0) * 0.5, (source_center.1 + target_center.1) * 0.5);
                if !self.screen_point_is_on_surface(midpoint) {
                    return semio_framework_pack_json::to_json_string(&unresolved);
                }
                return semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(midpoint.0), y: Some(midpoint.1), rect: None, polyline: Some(vec![[source_center.0, source_center.1], [target_center.0, target_center.1]]) });
            }
            _ => None,
        };

        let Some(bounds) = bounds_result else { return semio_framework_pack_json::to_json_string(&unresolved) };
        let (rect, center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
        if !self.screen_point_is_on_surface(center) {
            return semio_framework_pack_json::to_json_string(&unresolved);
        }
        semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(center.0), y: Some(center.1), rect: Some(rect), polyline: None })
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:99:            serde_json::from_str::<Value>(&host.entity_screen_json(domain, id)).ok()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2311:        NodeGraphEngine::Dag(host) => host.entity_screen_json(domain, entity),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2312:        NodeGraphEngine::Flow(host) => host.entity_screen_json(domain, entity),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:550:        self.dag.entity_screen_json(domain, id)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:870:            self.state.borrow().host.dag.entity_screen_json(domain, id)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:574:    let json = host.entity_screen_json("node", "a");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:581:    let json = host.entity_screen_json("node", "missing");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3781:            rows.push(format!("{{\"kind\":\"node\",\"id\":{},\"body\":{body_json},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&node.id), self.entity_screen_json("node", &node.id)));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3784:                rows.push(format!("{{\"kind\":\"handle\",\"direction\":\"{direction}\",\"id\":{},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&channel), self.entity_screen_json("handle", &channel)));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3796:        let value = semio_framework_pack_json::parse(&self.entity_screen_json(domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:73:/// 'entity_screen_json("handle", …)' hands a script, a demonstration or an assistive caller. Its`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:76:    let geometry: Value = serde_json::from_str(&host.entity_screen_json("handle", endpoint)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:237:/// The defect this pins: 'entity_screen_json("handle", …)' published the port ROW rect while`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:357:    let geometry: Value = serde_json::from_str(&host.entity_screen_json("handle", &endpoint)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs:362:    let back: Value = serde_json::from_str(&host.entity_screen_json("handle", &endpoint)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:856:    let json = host.entity_screen_json("node", "scale");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:862:    let wildcard: Value = semio_framework_pack_json::parse(&host.entity_screen_json("node", "*"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:872:    let input_json: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "scale@in"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:874:    let output_json: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "combine@b"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:876:    let malformed: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "scale"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:878:    let missing_port: Value = semio_framework_pack_json::parse(&host.entity_screen_json("handle", "scale@nope"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:886:    let json: Value = semio_framework_pack_json::parse(&host.entity_screen_json("edge", "e1"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:898:        let json: Value = semio_framework_pack_json::parse(&host.entity_screen_json(domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:905:    let empty: Value = semio_framework_pack_json::parse(&DagHost::from_host_snapshot(empty_fixture).entity_screen_json("node", "*"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1670:/// at every zoom ('entity_screen_json("handle", …)'), so every tier but the silhouette must accept`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:558:    /// rect through 'entity_screen_json("handle", …)' at every zoom — so a press on one wires at every`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4566:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { Ok(domain.host.entity_screen_json(text(args, "domain")?, text(args, "id")?).into_bytes()) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:206:        let geometry: Value = serde_json::from_str(&adapter.host.entity_screen_json("node", id)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:261:        let geometry: Value = serde_json::from_str(&host.entity_screen_json("node", &id)).expect("entity screen json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1803:        self.dag.entity_screen_json(domain, id)`

## set_selected_channels_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4453` through line 4469.
- Exact body SHA-256: `bc35f5f07ac9ca1815b8cb893e524adc902a354db6e965d5723d3a4868404ebd`.
- Explicit codec calls: `semio_framework_pack_json::from_json_str`.

```rust
pub fn set_selected_channels_json(&mut self, json: &str) {
        let channels: Vec<DagChannelRef> = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            let mut selection = Selection::default();
            for channel in channels {
                if let Some(hid) = self.handle_id_for_port(&channel.widget_id, &channel.port) {
                    selection.handle_ids.insert(hid);
                }
            }
            self.engine.selection = selection;
            self.engine.preselect = Selection::default();
            self.engine.preselect_removed = Selection::default();
            return;
        }
        let widget_ids: Vec<String> = channels.into_iter().map(|channel| channel.widget_id).collect();
        self.set_selection(&widget_ids);
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:2208:                        domain.host.set_selected_channels_json(text(args, "json")?);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1863:        self.dag.set_selected_channels_json(json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1963:    host.set_selected_channels_json(r#"[{"widgetId":"add","port":"a","direction":"in"}]"#);`

## set_node_statuses_from_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4503` through line 4544.
- Exact body SHA-256: `db3a4191617b650eb7f49da898baff017ede4e5b094bf786f22b265ceb78e622`.
- Explicit codec calls: `semio_framework_pack_json::parse`.

```rust
pub fn set_node_statuses_from_json(&mut self, json: &str) {
        self.computing_active = None;
        self.computing_stale.clear();
        self.node_eval_status.clear();
        self.unresolved_input_ports.clear();
        let Ok(value) = semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return;
        };
        let Some(map) = value.as_object() else {
            return;
        };
        for (widget_id, entry) in map {
            let Some(nid) = self.node_id_for_widget_id(widget_id) else {
                continue;
            };
            let status = entry.get("status").and_then(|value| value.as_str()).unwrap_or("ok");
            match status {
                "computing" => {
                    self.computing_active = Some(nid);
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Computing);
                }
                "queued" => {
                    self.computing_stale.insert(nid);
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Queued);
                }
                "error" => {
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Error);
                }
                "blocked" => {
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Blocked);
                    if let Some(ports) = entry.get("ports").and_then(|value| value.as_array()) {
                        for port in ports.iter().filter_map(|value| value.as_str()) {
                            self.unresolved_input_ports.insert((nid, port.to_string()));
                        }
                    }
                }
                _ => {
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Ok);
                }
            }
        }
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2869:                NodeGraphEngine::Flow(host) => host.set_node_statuses_from_json(json),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2870:                NodeGraphEngine::Dag(host) => host.dag.set_node_statuses_from_json(json),`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:331:            self.dag.set_node_statuses_from_json(status_json);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:1638:                        domain.host.set_node_statuses_from_json(text(args, "json")?);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:667:        self.dag.set_node_statuses_from_json(json);`

## load_host_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4822` through line 4828.
- Exact body SHA-256: `c536f9173434d9b32582fc2a82c8ab6b89087e52a66cdffd8bea3a47428926a9`.
- Explicit codec calls: `semio_framework_pack_json::from_json_str`.

```rust
pub fn load_host_snapshot_json(json: &str) -> Result<Self, DagError> {
        let host_snapshot: DagHostSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        if host_snapshot.schema != "dag.hostDocument" {
            return Err(DagError::SchemaMismatch);
        }
        Ok(Self::from_host_snapshot(host_snapshot))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6958:            let host = DagHost::load_host_snapshot_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:74:    let mut host = DagHost::load_host_snapshot_json(&semio_framework_pack_json::to_json_string(&fixture)).map_err(|error| layout_fault("dag.layout-run.layered-load", error.to_string()))?;`

## host_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:4830` through line 4832.
- Exact body SHA-256: `d452cceb345786e275a83b7c2b5769ac8ef81197e70582d23fe4412f823282f1`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn host_snapshot_json(&self) -> Result<String, DagError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:567:        Ok(self.dag.host_snapshot_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:906:            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:611:    let json = host.host_snapshot_json().expect("fixture json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6965:            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5468:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.host_snapshot_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5712:        let fixture = self.host.host_snapshot_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1080:    let json = host.host_snapshot_json().unwrap();`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️rule-application/🦀️.rs:81:    let snapshot_json = g.host_snapshot_json().unwrap();`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:242:    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: graph.host_snapshot_json()?, query: result }))`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs:245:    let json = g.host_snapshot_json().expect("fixture json");`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:76:    let layered: DagHostSnapshot = host.host_snapshot_json().ok().and_then(|json| semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).ok_or_else(|| layout_fault("dag.layout-run.layered-result", "the layered host returned no decodable fixture".into()))?;`

## screen_hit_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5186` through line 5198.
- Exact body SHA-256: `e8574604e519058cd5b4369efaaa5b17ac62e359c6cefae872f84d4d1c9184fc`.
- Explicit codec calls: `semio_framework_pack_json::to_json_string`.

```rust
pub fn screen_hit_json(&self, sx: f64, sy: f64) -> String {
        let hit = self.screen_hit(sx, sy);
        semio_framework_pack_json::to_json_string(&DagScreenHitJson {
            node: hit.node_id.clone(),
            draggable: hit.is_draggable_body(),
            handle: hit.channel.as_ref().map(|channel| format!("{}@{}", channel.widget_id, channel.port)),
            direction: hit.channel.as_ref().map(|channel| channel.direction.clone()),
            widget: hit.widget,
            minimap: hit.minimap,
            minimap_viewport: hit.minimap_viewport,
            port_insert: hit.port_insert,
        })
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4297:        let trace = dag.screen_hit_json(sx, sy);`

## node_overlays_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5872` through line 5908.
- Exact body SHA-256: `b9b776b2859d6ef5d3482c538b076ca57a60e7045ee101d07743db3ae3a5bf78`.
- Explicit codec calls: `semio_framework_pack_json::object`, `semio_framework_pack_json::to_string`.

```rust
pub fn node_overlays_json(&self) -> Result<String, DagError> {
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let mut overlays = Vec::new();
        for node in &self.host_snapshot.nodes {
            let DagNodeKind::Screen { media: Some(media), .. } = &node.kind else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let inset = 8.0 / cam.zoom.max(0.05);
            let top = node.y - hh + hh * 0.35;
            let bottom = node.y + hh - inset;
            let left = node.x - hw + inset;
            let right = node.x + hw - inset;
            let tl = world_to_screen(&cam, &viewport, Point::new(left, top));
            let br = world_to_screen(&cam, &viewport, Point::new(right, bottom));
            let media_kind = match media.kind {
                DagMediaKind::Image => "image",
                DagMediaKind::Svg => "svg",
                DagMediaKind::Pdf => "pdf",
                DagMediaKind::Video => "video",
            };
            overlays.push(semio_framework_pack_json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("mediaKind".to_string(), Value::from(media_kind)),
                ("src".to_string(), Value::from(media.src.clone())),
                (
                    "rect".to_string(),
                    semio_framework_pack_json::object([("x".to_string(), Value::from(tl.x)), ("y".to_string(), Value::from(tl.y)), ("w".to_string(), Value::from((br.x - tl.x).max(1.0))), ("h".to_string(), Value::from((br.y - tl.y).max(1.0)))]),
                ),
            ]));
        }
        Ok(semio_framework_pack_json::to_string(&Value::Array(overlays)))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6970:            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:776:    let json = host.node_overlays_json().unwrap();`

## label_overlay_rows_for_node_spec

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5989` through line 5995.
- Exact body SHA-256: `867c960b3038612448f50f5a147579670ad97ac5a360676a0dfe46271d12ed7e`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn label_overlay_rows_for_node_spec(&self, node: &DagNodeSpec, ghost: bool) -> Vec<Value> {
        let lod = self.draw_lod_for_frame();
        let zoom = self.host_snapshot.camera.zoom;
        let lod_index = dag_lod_index(zoom);
        let engine_nid = self.node_id_for_widget_id(&node.id);
        Self::label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost, engine_nid, &self.unresolved_input_ports)
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1872:    let ghost_overlay_rows = host.dag.label_overlay_rows_for_node_spec(ghost, true);`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1889:    let placed_rows = host.dag.label_overlay_rows_for_node_spec(&placed_node, false);`

## slider_overlay_state_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6136` through line 6166.
- Exact body SHA-256: `64951bc422ee7a5f05e44cf320ec174c1cb7fc4e324c920deef645bea479907d`.
- Explicit codec calls: `semio_framework_pack_json::object`, `semio_framework_pack_json::to_string`.

```rust
pub fn slider_overlay_state_json(&self) -> Result<String, DagError> {
        let cam = &self.host_snapshot.camera;
        let mut sliders: Vec<Value> = Vec::new();
        for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, snapshot_node);
            let DagNodeKind::Slider { min, max, step, value, .. } = &node.kind else {
                continue;
            };
            let (x0, y0, x1, y1) = slider_track_bounds(&node);
            sliders.push(semio_framework_pack_json::object([
                ("widgetId".to_string(), Value::from(snapshot_node.id.clone())),
                ("label".to_string(), Value::from(node.name.clone())),
                ("value".to_string(), Value::from(*value)),
                ("min".to_string(), Value::from(*min)),
                ("max".to_string(), Value::from(*max)),
                ("step".to_string(), Value::from(*step)),
                ("x".to_string(), Value::from((x0 + x1) * 0.5)),
                ("y".to_string(), Value::from((y0 + y1) * 0.5)),
                ("w".to_string(), Value::from((x1 - x0).max(1.0))),
                ("h".to_string(), Value::from((y1 - y0).max(1.0))),
                ("fontScreenPx".to_string(), Value::from(DAG_LABEL_SCREEN_PX * DAG_SLIDER_VALUE_LABEL_RATIO)),
                ("gapScreenPx".to_string(), Value::from(DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_RATIO)),
            ]));
        }
        Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("width".to_string(), Value::from(self.width)),
            ("height".to_string(), Value::from(self.height)),
            ("sliders".to_string(), Value::Array(sliders)),
        ])))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:201:        let state: Value = serde_json::from_str(&host.slider_overlay_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2295:            NodeGraphEngine::Dag(host) => host.dag.slider_overlay_state_json().ok()?,`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2296:            NodeGraphEngine::Flow(host) => host.slider_overlay_state_json().ok()?,`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:779:            self.state.borrow().host.dag.slider_overlay_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:622:        let json = host.slider_overlay_state_json().unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:656:    let raw: Value = semio_framework_pack_json::parse(&host.slider_overlay_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:2577:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.slider_overlay_state_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1998:        Ok(self.dag.slider_overlay_state_json()?)`

## label_overlay_paint_state_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6169` through line 6191.
- Exact body SHA-256: `8550688d1893a8cf8ca2f5c92343ba968a967c47719cafd3fd63652bdfaa0680`.
- Explicit codec calls: `semio_framework_pack_json::to_string`, `semio_framework_pack_json::object`.

```rust
pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {
        let lod = self.draw_lod_for_frame();
        let cam = &self.host_snapshot.camera;
        let lod_index = dag_lod_index(cam.zoom);
        let mut labels = Vec::new();
        for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, snapshot_node);
            let engine_nid = self.engine_node_id_for_index(idx);
            labels.extend(Self::label_overlay_rows_for_node(node.as_ref(), lod, cam.zoom, lod_index, false, engine_nid, &self.unresolved_input_ports));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            labels.extend(Self::label_overlay_rows_for_node(ghost, lod, cam.zoom, lod_index, true, None, &self.unresolved_input_ports));
        }
        let minimap_widget = self.minimap_widget_json();
        Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("lod".to_string(), Value::from(lod.label())),
            ("width".to_string(), Value::from(self.width)),
            ("height".to_string(), Value::from(self.height)),
            ("labels".to_string(), Value::Array(labels)),
            ("minimapWidget".to_string(), Value::from(minimap_widget)),
        ])))
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:95:            SEQUENCE_OPERATION_LABEL_OVERLAY => self.host.dag.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error),`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:182:        let labels = self.host.dag.label_overlay_paint_state_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:365:            serde_json::from_str(&host.label_overlay_paint_state_json().ok()?).ok()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4869:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4873:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:394:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:774:            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:620:    let json = host.label_overlay_paint_state_json().expect("labels");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:636:    let state: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label state")).expect("independent JSON oracle");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6975:            self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:563:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:592:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:693:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:744:        let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1723:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1734:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1824:    let state: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:2017:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4117:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5713:        let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:329:        let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:361:    let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2173:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1145:    let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1874:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1922:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue/🦀️.rs:220:        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`

## load_host_snapshot_json / loadHostSnapshotJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6957` through line 6961.
- Exact body SHA-256: `a9db502ba671f78bcad57886c76bfb8ad1872b82e5c87ab96b58ebf0a51ee490`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn load_host_snapshot_json(&self, json: &str) -> Result<(), JsValue> {
            let host = DagHost::load_host_snapshot_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:12:    loadHostSnapshotJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:12:    loadHostSnapshotJson(json: string): void;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:26:    loadHostSnapshotJson(json: string): void;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:74:    let mut host = DagHost::load_host_snapshot_json(&semio_framework_pack_json::to_json_string(&fixture)).map_err(|error| layout_fault("dag.layout-run.layered-load", error.to_string()))?;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:26:    loadHostSnapshotJson(json: string): void;`

## host_snapshot_json / hostSnapshotJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6964` through line 6966.
- Exact body SHA-256: `6674ed3dea9c4b344037369b97580e2562895a8a86436c32208358a2e0539908`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn host_snapshot_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:567:        Ok(self.dag.host_snapshot_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:906:            self.state.borrow().host.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:611:    let json = host.host_snapshot_json().expect("fixture json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:10:    hostSnapshotJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5468:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.host_snapshot_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5712:        let fixture = self.host.host_snapshot_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1080:    let json = host.host_snapshot_json().unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:10:    hostSnapshotJson(): string;`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️rule-application/🦀️.rs:81:    let snapshot_json = g.host_snapshot_json().unwrap();`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs:242:    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: graph.host_snapshot_json()?, query: result }))`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/🔬️unit/🦀️.rs:245:    let json = g.host_snapshot_json().expect("fixture json");`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:24:    hostSnapshotJson(): string;`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs:76:    let layered: DagHostSnapshot = host.host_snapshot_json().ok().and_then(|json| semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).ok_or_else(|| layout_fault("dag.layout-run.layered-result", "the layered host returned no decodable fixture".into()))?;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:24:    hostSnapshotJson(): string;`

## node_overlays_json / nodeOverlaysJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6969` through line 6971.
- Exact body SHA-256: `b7ab231585030058db3a72325b046dab4b1afbd1d21b4dd921486632e19d606a`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn node_overlays_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:776:    let json = host.node_overlays_json().unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:15:    nodeOverlaysJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:15:    nodeOverlaysJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:29:    nodeOverlaysJson(): string;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:29:    nodeOverlaysJson(): string;`

## label_overlay_paint_state_json / labelOverlayPaintStateJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6974` through line 6976.
- Exact body SHA-256: `a9a0556d977d16d3d3e621f06b7dcda01d0e5876d6c9688f7687160f18851edf`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:136:export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:95:            SEQUENCE_OPERATION_LABEL_OVERLAY => self.host.dag.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error),`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs:182:        let labels = self.host.dag.label_overlay_paint_state_json().map_err(domain_error)?;`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:69:  labelOverlayPaintStateJson(): SequenceTask<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:365:            serde_json::from_str(&host.label_overlay_paint_state_json().ok()?).ok()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4869:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4873:                let state_json = host.label_overlay_paint_state_json().ok()?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:106:  labelOverlayPaintStateJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:844:      paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr, {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1974:      const cameraJson = read("labels", session.labelOverlayPaintStateJson());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1991:        const cameraJson = read("labels", session.labelOverlayPaintStateJson());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:394:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🦀️.rs:774:            self.state.borrow().host.dag.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:620:    let json = host.label_overlay_paint_state_json().expect("labels");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🔬️unit/🦀️.rs:636:    let state: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label state")).expect("independent JSON oracle");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:563:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:592:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:693:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:744:        let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1723:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1734:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1824:    let state: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:2017:    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:11:    labelOverlayPaintStateJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:4117:                let result: Result<Vec<u8>, FlowFailure> = flow_result! { domain.host.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error) };`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs:5713:        let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:329:        let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs:361:    let state: Value = serde_json::from_str(&host.label_overlay_paint_state_json().expect("label overlay state")).expect("label json");`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2173:        Ok(self.dag.label_overlay_paint_state_json()?)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1145:    let raw: serde_json::Value = serde_json::from_str(&host.dag.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1874:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:1922:    let overlay: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:11:    labelOverlayPaintStateJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:25:    labelOverlayPaintStateJson(): string;`
- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue/🦀️.rs:220:        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:25:    labelOverlayPaintStateJson(): string;`

## lod_scale_json / lodScaleJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7027` through line 7029.
- Exact body SHA-256: `f9bcb430f33ac63d149cc112c57d6412310c59ab75876bee733eed76f8fec3be`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn lod_scale_json(&self) -> String {
            dag_lod_scale_json()
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2577:            "lodScaleJson" => lod_scale_json::lod_scale_json(ctx),`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:136:export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:138:export class DagSession { lodScaleJson() { return dagLodScaleJson(); } }`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:139:export class BoardSession { lodScaleJson() { return dagLodScaleJson(); } }`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:59:  lodScaleJson(): SequenceTask<string>;`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:23:    lodScaleJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:13:    lodScaleJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:13:    lodScaleJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:27:    lodScaleJson(): string;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:27:    lodScaleJson(): string;`

## set_canvas_theme_json / setCanvasThemeJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7082` through line 7086.
- Exact body SHA-256: `9af99d36d93defd9b7581c7a9e74566d00ed9085568eab6d55e12f91b75d23f0`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn set_canvas_theme_json(&mut self, json: &str) {
            let mut accepted=|_|true;
            let mut control=semio_framework_value::NativeDecodeControl::new(64*1024,&mut accepted);
            if let Ok(overlay)=crate::infinite::board::io::text::palette::decode_board_palette_overlay_json(json,&mut control){self.state.borrow_mut().host.set_canvas_palette(&overlay);}
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts:573:      setCanvasThemeJson(json: string) {`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts:232:        setCanvasThemeJson(json: string) {`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts:570:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts:578:    session.setCanvasThemeJson(serializeCanvasThemeJson());`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:63:  setCanvasThemeJson(json: string): SequenceTask<Uint8Array>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:2671:  observeFlowTask(session, "setCanvasThemeJson", session.setCanvasThemeJson(serializeCanvasThemeJson()));`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:94:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:150:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:237:  setCanvasThemeJson(json: string): void;`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:83:    setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:24:    setCanvasThemeJson(json: string): void;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:24:    setCanvasThemeJson(json: string): void;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:38:    setCanvasThemeJson(json: string): void;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:38:    setCanvasThemeJson(json: string): void;`

## snapshot_json / snapshotJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7154` through line 7157.
- Exact body SHA-256: `872e9b7afc310b80f93931653fbcfe3a3a7975adcb006c1e9c9690e7aefd6feb`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub async fn snapshot_json(&self) -> Result<String, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:76:    let json = snapshot_json(node);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:106:    let json = snapshot_json(tree);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:124:    let json = snapshot_json(tree);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:238:    let json = snapshot_json(tree);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:283:    let json = snapshot_json(opened);`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:311:    let json = snapshot_json(node);`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧪️tests/🙈️ignore-integration/🦀️.rs:36:            .snapshot_json(VECTORS)?`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧪️tests/🪪️artifact-id-builders/🦀️.rs:76:            .snapshot_json(VECTORS)?`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/🧪️tests/🧬️definition-kind-derivation/🦀️.rs:18:            .snapshot_json(VECTORS)?`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:135:export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts:19:  snapshotJson(): SequenceTask<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8522:    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8534:    assert_eq!(store.snapshot_json().expect("snapshot json"), serde_json::to_string(&DemoSnapshot { n: Some(7) }).unwrap());`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:43:    snapshotJson(): Promise<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:79:    snapshotJson(): string;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:43:    snapshotJson(): Promise<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:79:    snapshotJson(): string;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:57:    snapshotJson(): Promise<string>;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:93:    snapshotJson(): string;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:57:    snapshotJson(): Promise<string>;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:93:    snapshotJson(): string;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:345:        let projection = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:358:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:365:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:379:        let applied = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:386:        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:402:            ("document".to_string(), snapshot_json(&derived)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧪️tests/🔢️mutate-semio-value/🦀️.rs:449:            ("graph".to_string(), snapshot_json(&graph)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:181:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:220:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:232:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:239:        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:252:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧪️tests/🎥️mutate-semio-video/🦀️.rs:277:            ("document".to_string(), snapshot_json(&twice)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:401:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:411:        let projection = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:423:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:430:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:445:        let applied = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:452:        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs:494:            ("building".to_string(), snapshot_json(&building)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:427:            format!("masters=[{masters}] layouts=[{layouts}] slides=[{slides}] digest={}", digest(snapshot_json(deck).to_string().as_bytes()))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:437:        Ok(Outcome::projection(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:448:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:455:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:473:        Ok(Outcome::projection(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧪️tests/📽️mutate-semio-presentation/🦀️.rs:511:                    ("document".to_string(), snapshot_json(&parsed)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:157:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:190:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:202:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:209:        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:225:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧪️tests/🔊️mutate-semio-audio/🦀️.rs:250:            ("document".to_string(), snapshot_json(&twice)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:989:function snapshotJson(ctx: AdapterContext, uri: string): unknown {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1007:  const mutation = snapshotJson(ctx, stepUris(ctx, "shared://🔺️mutate-semio-mesh/").find((uri) => uri.endsWith("/🦠️mutation/🔣️.json"))!) as Mutation;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1014:  const mutation = snapshotJson(ctx, stepUris(ctx, "shared://🔺️mutate-semio-mesh/").find((uri) => uri.endsWith("/🦠️mutation/🔣️.json"))!) as Mutation;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1025:  const before = snapshotJson(ctx, uris[0]!) as Snapshot;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1026:  const mutation = snapshotJson(ctx, uris[1]!) as Mutation;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts:1027:  const expected = snapshotJson(ctx, uris[2]!) as Snapshot;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:212:        let document = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:225:        let mutated = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:232:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current)?)])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:245:        Ok(Outcome::projection(snapshot_json(&current)?))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🦀️.rs:280:        let document = snapshot_json(&parsed)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:231:        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:267:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:279:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:286:        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:299:        Ok(outcome(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧪️tests/🎞️mutate-semio-animation/🦀️.rs:327:            ("document".to_string(), snapshot_json(&twice)),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:174:        let document = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:187:        let mutated = snapshot_json(&current)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:194:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current)?)])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:207:        Ok(Outcome::projection(snapshot_json(&current)?))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🦀️.rs:242:        let document = snapshot_json(&parsed)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:250:        Ok(Outcome::projection(Json::Object(vec![("document".to_string(), snapshot_json(&current)), ("raster".to_string(), raster_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:261:        let mutated = snapshot_json(&current);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:268:        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:287:        Ok(Outcome::projection(snapshot_json(&current)))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧪️tests/🖼️mutate-semio-image/🦀️.rs:323:            ("document".to_string(), snapshot_json(&parsed)),`

## envelope_json / envelopeJson

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:7160` through line 7163.
- Exact body SHA-256: `5b3f9de12b265115fdac807328b5e05aa5547eb2586932e0957f4060385eaeb9`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub async fn envelope_json(&self) -> Result<String, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🕸️bindings/flow_core.d.ts:41:    envelopeJson(): Promise<string>;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts:41:    envelopeJson(): Promise<string>;`
- `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/🕸️bindings/cad_geometry.d.ts:55:    envelopeJson(): Promise<string>;`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session.d.ts:55:    envelopeJson(): Promise<string>;`

## apply_force_graph_layout_to_board_snapshot_value

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:217` through line 219.
- Exact body SHA-256: `5309f4e4c83544a0e3b42a1eae901a6bf7f08b43f0fc676c394f0f8824eb0716`.
- Explicit codec calls: none in this body; inspect delegates.

```rust
pub fn apply_force_graph_layout_to_board_snapshot_value(snapshot: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), UndirectedGraphError> {
        apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot, opts, resolve_node_id_endpoint)
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:946:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:1713:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:343:        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:398:                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;`

## apply_force_graph_layout_to_board_snapshot_value_resolved

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:222` through line 337.
- Exact body SHA-256: `c88dee91437a633e5ce686e71eae93544ac17ac48d3005d5c964088f31dfc89e`.
- Explicit codec calls: `serde_json::json`.

```rust
pub fn apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot: &mut Value, opts: &ForceGraphLayoutOptions, resolve_node_id: impl Fn(&str, &HashMap<String, usize>) -> Option<String>) -> Result<(), UndirectedGraphError> {
        let Some(root) = snapshot.as_object_mut() else {
            return Err(UndirectedGraphError::SnapshotRootNotObject);
        };
        if !snapshot_schema_ok(root.get("schema").and_then(|v| v.as_str())) {
            return Err(snapshot_schema_error());
        }
        let edges = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err(UndirectedGraphError::NodesMissing);
        };
        if nodes.is_empty() {
            return Ok(());
        }
        let locked_ids: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut id_to_index: HashMap<String, usize> = HashMap::new();
        let mut visible_node_indices: Vec<usize> = Vec::new();
        let mut optional_xy: Vec<Option<(f64, f64)>> = Vec::new();
        let mut is_locked: Vec<bool> = Vec::new();
        let mut positions: Vec<Vec2> = Vec::new();
        let mut radii: Vec<f64> = Vec::new();
        for (raw_idx, node) in nodes.iter().enumerate() {
            let Some(obj) = node.as_object() else {
                return Err(UndirectedGraphError::NodeNotObject);
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:obj.get("hidden").and_then(|v|v.as_bool()),visible:obj.get("visible").and_then(|v|v.as_bool()),locked:obj.get("locked").and_then(|v|v.as_bool())}) {
                continue;
            }
            let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                return Err(UndirectedGraphError::NodeIdMissing);
            };
            let x_opt = obj.get("x").and_then(|v| v.as_f64());
            let y_opt = obj.get("y").and_then(|v| v.as_f64());
            let xy = match (x_opt, y_opt) {
                (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
                _ => None,
            };
            id_to_index.insert(nid.to_string(), positions.len());
            visible_node_indices.push(raw_idx);
            optional_xy.push(xy);
            is_locked.push(locked_ids.contains(nid));
            positions.push(Vec2::ZERO);
            radii.push(node_repulsion_radius(node));
        }
        let n = positions.len();
        if n == 0 {
            return Ok(());
        }
        let mut sum = Vec2::ZERO;
        let mut finite_ct: u32 = 0;
        for (x, y) in optional_xy.iter().flatten() {
            sum += Vec2::new(*x, *y);
            finite_ct += 1;
        }
        let anchor = if finite_ct > 0 { sum / (finite_ct as f64) } else { Vec2::new(opts.center_x.unwrap_or(0.0), opts.center_y.unwrap_or(0.0)) };
        for i in 0..n {
            positions[i] = if let Some((x, y)) = optional_xy[i] { Vec2::new(x, y) } else { Vec2::ZERO };
        }
        let pin: Vec<Option<Vec2>> = (0..n).map(|i| if is_locked[i] { Some(positions[i]) } else { None }).collect();
        force::seed_positions(&mut positions, &pin, anchor, opts.random_seed);
        let mut edge_pairs: Vec<(usize, usize)> = Vec::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for e in &edges {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_visible_or_true(&crate::infinite::board::BoardVisibility {hidden:eo.get("hidden").and_then(|v|v.as_bool()),visible:eo.get("visible").and_then(|v|v.as_bool()),locked:eo.get("locked").and_then(|v|v.as_bool())}) {
                continue;
            }
            let Some((src, tgt)) = snapshot_edge_node_ids(eo) else {
                continue;
            };
            let Some(a) = resolve_node_id(src, &id_to_index) else {
                continue;
            };
            let Some(b) = resolve_node_id(tgt, &id_to_index) else {
                continue;
            };
            if a == b {
                continue;
            }
            let Some(&ia) = id_to_index.get(&a) else {
                continue;
            };
            let Some(&ib) = id_to_index.get(&b) else {
                continue;
            };
            let lo = ia.min(ib);
            let hi = ia.max(ib);
            if seen.insert((lo, hi)) {
                edge_pairs.push((lo, hi));
            }
        }
        let mut cx = 0.0f64;
        let mut cy = 0.0f64;
        for p in &positions {
            cx += p.x;
            cy += p.y;
        }
        cx /= n as f64;
        cy /= n as f64;
        let gx = opts.center_x.unwrap_or(cx);
        let gy = opts.center_y.unwrap_or(cy);
        force::run_force_layout(&mut positions, &radii, &edge_pairs, &pin, &core_opts(opts, gx, gy));
        for (idx, raw_idx) in visible_node_indices.into_iter().enumerate() {
            let Some(node) = nodes.get_mut(raw_idx) else {
                continue;
            };
            let Some(obj) = node.as_object_mut() else {
                continue;
            };
            obj.insert("x".into(), serde_json::json!(positions[idx].x));
            obj.insert("y".into(), serde_json::json!(positions[idx].y));
        }
        Ok(())
    }
```

Direct lexical call sites (Rust/TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:935:        infinite::board::normal::undirected::apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot, opts, |endpoint, id_to_index| {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:218:        apply_force_graph_layout_to_board_snapshot_value_resolved(snapshot, opts, resolve_node_id_endpoint)`

## apply_force_graph_layout_to_board_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:340` through line 345.
- Exact body SHA-256: `b37264fcf423e64b8d71e139e33e8fa12a059cff6272b89291d427c47fd992f7`.
- Explicit codec calls: `serde_json::from_str`, `serde_json::to_string`.

```rust
pub fn apply_force_graph_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, UndirectedGraphError> {
        let mut snapshot: Value = serde_json::from_str(snapshot_json)?;
        let opts: ForceGraphLayoutOptions = if options_json.trim().is_empty() { ForceGraphLayoutOptions::default() } else { serde_json::from_str(options_json)? };
        apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &opts)?;
        Ok(serde_json::to_string(&snapshot)?)
    }
```

Direct lexical call sites (Rust/TypeScript):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:37:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:77:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:179:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:189:    let err = apply_force_graph_layout_to_board_snapshot_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:231:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:285:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:286:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:316:    let out_a = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:317:    let out_b = apply_force_graph_layout_to_board_snapshot_json(&s, &o).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🧪️tests/🔬️unit/🦀️.rs:346:    let out = apply_force_graph_layout_to_board_snapshot_json(&snapshot.to_string(), &opts.to_string()).unwrap();`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚛️force-layout/🦀️.rs:8:    let Ok(layout_json) = crate::editor::puzzle2d::engine::apply_force_graph_layout_to_board_snapshot_json(&ctx.scene.board_snapshot.to_string(), r#"{"mode":"force-graph"}"#) else {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:36:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs:64:    let out = apply_force_graph_layout_to_board_snapshot_json(&fixture.to_string(), &opts.to_string()).unwrap();`

## apply_redraw_layout_to_board_snapshot_json

- Source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔️undirected/🦀️.rs:375` through line 403.
- Exact body SHA-256: `d75f780ba89d64adbf18a891b117269494adee3eaf8b802dae284040f89528a1`.
- Explicit codec calls: `serde_json::from_str`, `serde_json::to_string`.

```rust
pub fn apply_redraw_layout_to_board_snapshot_json(snapshot_json: &str, options_json: &str) -> Result<String, UndirectedGraphError> {
        let opts: RedrawSnapshotOptions = serde_json::from_str(options_json)?;
        if opts.redraw_handles_after {
            return Err(UndirectedGraphError::RedrawHandlesAfterUnsupported);
        }
        let mut snapshot: Value = serde_json::from_str(snapshot_json)?;
        match opts.mode.as_str() {
            "force-graph" => {
                let mut fo = opts.force_graph.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    fo.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    fo.center_y = opts.center_y;
                }
                if let Some(s) = opts.random_seed {
                    fo.random_seed = s;
                }
                for id in &opts.locked_node_ids {
                    if !fo.locked_node_ids.contains(id) {
                        fo.locked_node_ids.push(id.clone());
                    }
                }
                apply_force_graph_layout_to_board_snapshot_value(&mut snapshot, &fo)?;
            }
            other => return Err(UndirectedGraphError::UnsupportedRedrawMode(other.to_string())),
        }
        Ok(serde_json::to_string(&snapshot)?)
    }
```

Direct lexical call sites (Rust/TypeScript):

None found in the current first-party source census; exported callers or dispatch may still exist.

