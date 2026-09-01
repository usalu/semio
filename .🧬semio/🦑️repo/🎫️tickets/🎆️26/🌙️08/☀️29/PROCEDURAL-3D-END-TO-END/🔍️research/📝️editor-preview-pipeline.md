# Procedural3d Editor Preview Pipeline: End-to-End Analysis

## 1. Editor Window Kinds

### Edit Mode Windows (🎭️modes/✏️edit/🪟️windows)

| Window ID | Window KIND_ID | File Path | Renders | Surface Kind |
|-----------|---|---|---|---|
| `procedural-preview` | `PROCEDURAL_3D_PLAY_WINDOW_PREVIEW` | `👁️preview/🦀️component.rs:11` | Tessellated 3D geometry (meshes+instances) | `World3d` |
| `procedural-main` | `PROCEDURAL_3D_PLAY_WINDOW_MAIN` | `🕸️flow/🦀️component.rs:11` | Flow graph (nodes, edges, ports) | `NodeGraph` |

**Key File References:**
- Preview window definition: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs:17-34`
- Flow graph definition: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs:17-34`

### Generate Mode Windows (🎭️modes/🧬️generate/🪟️windows)

| Window ID | Window KIND_ID | Renders | Surface Kind |
|-----------|---|---|---|
| `procedural-preview` | `PROCEDURAL_3D_PLAY_WINDOW_PREVIEW` | Tessellated generation preview geometry | `World3d` |
| `📝️form` | Form inputs for generation parameters | HTML Form |
| `🗂️generations` | List of previous generations | Tree/List |

---

## 2. The 3D Preview Pipeline

### 2.1 High-Level Flow

```
Flow Evaluation (FlowEvalSession::tick)
  ↓
collect_geometry_handles_from_eval (find "handle" fields in widget outputs)
  ↓
pending_preview_tessellate_handles (filter handles needing tessellation)
  ↓
InvokeExtension → brep.tessellate (async)
  ↓
session.resolve_preview_tessellate (cache tessellated mesh)
  ↓
preview_payload_from_eval_with_session (build JSON meshes+instances)
```

### 2.2 Key Functions

#### `geometry_handles_for_widget(eval: &Value, widget_id: &str) -> Vec<String>`
**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:800-811`

```rust
pub fn geometry_handles_for_widget(eval: &Value, widget_id: &str) -> Vec<String> {
    let Some(widget_eval) = eval.get(widget_id) else {
        return Vec::new();
    };
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"));  // LINE 804: Channel enumeration
    let Some(channels) = channels else {
        return Vec::new();
    };
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles
}
```

**Channel Enumeration:** 
- Looks for `widget_eval["out"]` first, falls back to `widget_eval["in"]` (line 804)
- This is where the editor distinguishes output vs input geometry channels per widget

#### `collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>)`
**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:779-798`

```rust
pub fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for entry in map.values() {
                collect_geometry_handles_from_eval(entry, handles);  // Recursive traversal
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}
```

**Geometry Handle Location:** Recursively searches channel JSON for fields with key `"handle"` containing BRep handles.

#### `mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<MeshData>`
**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:862-878`

```rust
fn mesh_data_for_preview_handle(handle: &str, tolerance: f64, session: Option<&FlowEvalSession>) -> Option<MeshData> {
    if let Some(session) = session {
        if let Some(json) = session.preview_mesh_json(handle) {
            if let Ok(value) = serde_json::from_str::<Value>(json) {
                if value.get("error").is_none() {
                    if let Ok(data) = serde_json::from_value::<MeshData>(value) {
                        if mesh_has_preview_geometry(&data) {
                            return Some(data);
                        }
                    }
                }
            }
        }
    }
    let data = flow::tessellate_geometry(handle, tolerance).ok()?;  // LINE 876: Tessellation call
    mesh_has_preview_geometry(&data).then_some(data)
}
```

**Tessellation:** Calls `flow::tessellate_geometry(handle, tolerance)` at **line 876**. Uses cached mesh if available in session (`session.preview_mesh_json(handle)`), otherwise tessellates synchronously.

#### `preview_payload_from_eval_with_session(eval_json: &str, fixture: &FlowFixture, cfg: &Procedural3dConfig, session: Option<&FlowEvalSession>) -> (String, String)`
**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:942-995`

```rust
pub fn preview_payload_from_eval_with_session(eval_json: &str, fixture: &flow::FlowFixture, cfg: &Procedural3dConfig, session: Option<&FlowEvalSession>) -> (String, String) {
    // ... validation ...
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {  // LINE 956: Iterate widgets
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        let preview = matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let handles = geometry_handles_for_widget(&eval, &id);
        if handles.is_empty() {
            continue;
        }
        let selected = false;
        let hovered = false;
        for (index, handle) in handles.iter().enumerate() {  // LINE 970: Loop over handles per widget
            let mesh_id = if handles.len() == 1 { format!("eval-{id}") } else { format!("eval-{id}#{index}") };
            let instance_id = if handles.len() == 1 { id.clone() } else { format!("{id}#{index}") };
            if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
                if let Some(data) = mesh_data_for_preview_handle(handle, tolerance, session) {
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
                    "position": [0.0, 0.0, 0.0],  // LINE 985: Origin placement
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0],
                    "label": id,
                    "selected": selected,
                    "hovered": hovered}));
            }
        }
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}
```

**Critical Loop (line 956-992):**
- Iterates every widget in the fixture
- Filters to only preview widgets (line 958)
- For each preview widget, gets all geometry handles from the channel eval
- **For each handle** (line 970), creates a separate mesh+instance entry
- Mesh IDs include index suffix if multiple handles: `"eval-{id}#{index}"`

### 2.3 Tessellation Triggering

#### `preview_tessellate_effects(session: &mut FlowEvalSession, eval_json: &str, fixture: &flow::FlowFixture, cfg: &Procedural3dConfig) -> Vec<Effect>`
**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:911-936`

```rust
pub fn preview_tessellate_effects(session: &mut FlowEvalSession, eval_json: &str, fixture: &flow::FlowFixture, cfg: &Procedural3dConfig) -> Vec<Effect> {
    let tolerance = preview_tolerance(&cfg.lod_mode);
    let tolerance_bits = tolerance.to_bits();
    let mut live = std::collections::HashSet::new();
    let eval: Value = serde_json::from_str(eval_json).unwrap_or(json!({}));
    for widget in &fixture.widgets {  // LINE 916: Collect all live handles
        let id = crate::artifacts::procedural3d::widget_id(widget).to_string();
        for handle in geometry_handles_for_widget(&eval, &id) {
            live.insert(handle);
        }
    }
    session.retain_preview_meshes(&live);
    let mut effects = Vec::new();
    for handle in pending_preview_tessellate_handles(eval_json, fixture, session) {  // LINE 924: Get pending handles
        let node_hash = flow::preview_tessellate_node_hash(&handle, tolerance_bits);
        if session.note_pending_tessellate(node_hash, handle.clone()) {
            effects.push(Effect::InvokeExtension {
                req: semio_framework_plugin::RequestId(105),
                extension_id: "brep".into(),
                capability: "tessellate".into(),
                request_json: json!({ "handle": handle, "tolerance": tolerance, "nodeHash": node_hash }).to_string(),
            });
        }
    }
    effects
}
```

**Tessellation Commands:**

| Command | File | Triggered By | Purpose |
|---------|------|---|---|
| `flow-eval-tick` | `🎮️commands/🧮️flow-eval-tick/🦀️component.rs:14-31` | Line 28: Calls `preview_tessellate_effects` after eval completes | Schedules pending tessellations after evaluation |
| `flow-eval-resolve` | `🎮️commands/🧮️flow-eval-resolve/🦀️component.rs:17-20` | Flow engine (extension) | Seeds node cache with evaluated output |
| `flow-tessellate-resolve` | `🎮️commands/🧮️flow-tessellate-resolve/🦀️component.rs:17-20` | Brep engine (extension) | Caches tessellated mesh via `session.resolve_preview_tessellate` |

---

## 3. Preview Geometry: Per-Channel or Per-Widget?

**Answer: Per-HANDLE, which typically equals per-channel, but can be multiple per widget.**

**Evidence:**
- Line 970 in `preview_payload_from_eval_with_session`: `for (index, handle) in handles.iter().enumerate()`
- Multiple meshes per widget if `handles.len() > 1`
- Each handle → one unique mesh ID and instance ID
- Handles are extracted from channel JSON (.get("out") or .get("in"))

**Implication:** If a single widget's output channel contains multiple geometry handles (e.g., an array or nested structure), each handle generates its own preview mesh+instance. Typically one handle per channel, but the structure allows for multi-geometry outputs.

---

## 4. Preview Widgets Definition

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:958`

```rust
let preview = matches!(widget, flow::Widget::Neuron { preview: true, .. } | flow::Widget::OutputPreview { .. });
```

**Preview Widget Kinds:**
1. `Widget::Neuron { preview: true, ... }` — Neuron neurons with `preview` flag set to true
2. `Widget::OutputPreview { ... }` — Explicit preview output widget

Only these two kinds emit preview geometry to the 3D scene. All other widget types are filtered out (line 959: `if !preview { continue; }`).

---

## 5. Preview Geometry Placement & Layout

**All preview instances are placed at the origin:**
```rust
"position": [0.0, 0.0, 0.0],  // LINE 985
"rotation": [0.0, 0.0, 0.0, 1.0],
"scale": [1.0, 1.0, 1.0],
```

**Placement Strategy:** No per-widget or per-channel offset. All geometry renders at world origin. Spatial arrangement (if any) must be encoded in the geometry coordinates themselves (via transform nodes in the flow).

---

## 6. Node-Graph Window: Node & Port ID Structure

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:361-387`

```rust
pub fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),  // NODE ID: Direct from fixture
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { 
                id: format!("{}@{}", node.id, port.id),  // LINE 372: PORT ID FORMAT
                label: Some(port.label.clone()), 
                ..Default::default() 
            }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { 
                id: format!("{}@{}", node.id, port.id),  // LINE 373: PORT ID FORMAT
                label: Some(port.label.clone()), 
                ..Default::default() 
            }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { 
                id: edge.id.clone(), 
                source_node_id, 
                source_port_id, 
                target_node_id, 
                target_port_id, 
                label: None 
            }
        })
        .collect();
}
```

**Node-Graph ID Scheme:**

| Entity | ID Format | Source | Example |
|--------|-----------|--------|---------|
| Node | `node.id` | `DagFixture::nodes[].id` | `"node_42"` |
| Input Port | `"{node_id}@{port_id}"` | Line 372 | `"node_42@geometry"` |
| Output Port | `"{node_id}@{port_id}"` | Line 373 | `"node_42@geometry"` |
| Edge | `edge.id` | `DagFixture::edges[].id` | `"edge_1"` |

**Channels in Flow Graph:** Channels are NOT separate node-graph entities. They are data flowing through ports. Ports are identified by the `@` syntax. The fixture contains no explicit "channel" nodes.

---

## 7. Hover Handling in the Editor

**Current State (Ticket 26/08/14):** Hover has been moved to the framework's interaction domain.

**Key References:**

1. **Flow Graph Hover (NodeGraph surface):**
   - File: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs:54-82`
   - Lines 61-66: Comment notes that `selection`/`hover` are currently empty/none at the framework level — no live interaction UI until a future wave threads interaction into scene rendering.

2. **Transitive Hover on Cluster Groups:**
   - File: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:~ lines with "transitive hover"`
   - DAG parent-links enable transitive hover: hovering a Cluster group node highlights all child nodes
   - Configured with `hover: HoverSpec { transitive: true, ..HoverSpec::default() }`

3. **Preview Window Hover (World3d surface):**
   - File: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:969`
   - Hard-coded to `false`: Preview instances never mark themselves as hovered

4. **Utility-Switch & Hover:**
   - File: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-active-utility/🦀️component.rs`
   - Set-active-utility no longer clears hover — framework owns `graph` hover exclusively
   - No doc comment but flagged as framework behavior change

**Missing Interaction Threading:** Comment at line 966 in `preview_payload_from_eval_with_session`:
```rust
// 🕹️ `render` carries no `InteractionView` ... — no preview instance is ever marked 
// selected/hovered until a future wave threads interaction in.
```

This is identified as a **framework gap**: interaction selection/hover not yet threaded into the preview render path.

---

## Summary Table: Windows & Outputs

| Mode | Window | Surface | Data Source | Outputs |
|------|--------|---------|-------------|---------|
| **Edit** | Preview | World3d | `preview_payload_from_eval_with_session` | Meshes (JSON) + Instances (JSON) |
| **Edit** | Flow | NodeGraph | `fixture_to_workflow` | Nodes (id, ports, coords) + Edges (source→target) |
| **Generate** | Preview | World3d | Generation-preview eval | Meshes + Instances |
| **Generate** | Form | HTML | Generation schema | Parameter inputs |
| **Generate** | Generations | Tree/List | History | Previous results |

---

## Files & Key Line References

**Core Preview Pipeline:**
- `/✏️s/.../🏅️standards/🔖️1/.../✏️editor/🦀️component.rs`
  - Line 779: `collect_geometry_handles_from_eval`
  - Line 800: `geometry_handles_for_widget` (channel enumeration)
  - Line 862: `mesh_data_for_preview_handle` (tessellation)
  - Line 911: `preview_tessellate_effects` (tessellation scheduling)
  - Line 942: `preview_payload_from_eval_with_session` (main preview loop, **956-992**)
  - Line 958: Preview widget filter
  - Line 970: Handle enumeration loop
  - Line 985: Instance placement (origin)

**Windows:**
- `/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs:62` (preview window render)
- `/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs:54` (flow graph render)

**Commands:**
- `/🎮️commands/🧮️flow-eval-tick/🦀️component.rs:14-31`
- `/🎮️commands/🧮️flow-eval-resolve/🦀️component.rs:17-20`
- `/🎮️commands/🧮️flow-tessellate-resolve/🦀️component.rs:17-20`

**Schema & Node-Graph:**
- `/🧬️schema/🦀️component.rs:361` (`fixture_to_workflow`, node/port IDs)

