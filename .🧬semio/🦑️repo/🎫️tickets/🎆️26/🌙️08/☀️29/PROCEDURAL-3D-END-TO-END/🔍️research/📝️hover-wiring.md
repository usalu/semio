# Procedural3D Bidirectional Hover Wiring Analysis

## Executive Summary

**3D World → Plugin:** NO - The 3D world does NOT dispatch `interactionHover`. It dispatches plugin-specific actions: `setHover`, `referenceHover`, `worldVortexHover`.

**Node Graph → Plugin:** YES - The node graph dispatches `interactionHover` with proper domain/channel/targets structure.

**`interactionHover` Args Schema:** 
```json
{
  "domainId": "string",
  "channel": "string (defaults to 'pointer')",
  "targets": "JSON.stringify([{ granularity: string, id: string }, ...])"
}
```

**Port ID String:** `"nodeId@portId"` (split at "@" boundary)

**Graph Domain Granularities:** `"node"`, `"edge"`, `"handle"`

---

## A. 3D World → Plugin

### A.1. Mesh Action Map (🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:434)

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:432-439`

```typescript
export const nodeGraphActions = {
  select: "interactionSelect",
  hover: "interactionHover",
  edit: "nodeGraphEdit",
  parameter: "setGraphParameter",
  viewport: "nodeGraphViewport",
  spotlightCommit: "spotlightCommit",
} as const;
```

**Analysis:** This is a static action map for **node graph only**. There is NO `worldActions` export. The mesh module defines three action maps: `nodeGraphActions`, `textEditorActions`, and `inkCanvasActions` — but no world-specific action map.

### A.2. World3dHost Hover Dispatch (🧰️framework/🛍️products/💻️os/.../World3dHost/🟦️component.tsx)

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx:4345-4346`

```typescript
const dispatchInstanceHover = useMemo(
  () =>
    createCoalescingActionDispatcher<string | null>((id) => {
      if (id == null) dispatch("setHover", {});
      else dispatch("setHover", { objectId: id, mode: "mesh", id: 0 });
    }),
  [dispatch],
);
```

**Analysis:** The 3D world's instance hover is dispatched as `"setHover"` (plugin-specific action), NOT `"interactionHover"`. Additional hover types:
- **Reference hover:** `dispatch("referenceHover", { referenceId: id })` (line 4131)
- **Vortex hover:** `dispatch("worldVortexHover", { fullId })` (line 4355)

Grep search confirmed: NO `"interactionHover"` dispatch in World3dHost.

### A.3. FrameworkInteractionHoverJob Args Schema

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:21000-21006`

```rust
let channel = args.and_then(|value| value.get("channel")).and_then(Value::as_str).unwrap_or("pointer").to_string();
let targets = parse_interaction_targets(args, action).await?;
let hover_input = protocol::HoverInput { channel, targets };
let next = protocol::next_hover(&def.hover, &topology, &hover_input).await;
```

**Args Structure** (from `interaction_target_args` helper at line 34011-34016):
```rust
fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
    let targets = serde_json::to_string(&vec![protocol::InteractionTarget { 
        granularity: "item".into(), 
        id: id.into() 
    }]).expect("targets serialize");
    let mut object = extra;
    object["targets"] = json!(targets);
    object
}
```

**Example from tests** (line 35657):
```rust
app.handle_action(
    semio_framework::INTERACTION_HOVER_ACTION_ID, 
    Some(&interaction_target_args(
        json!({ "domainId": "items", "channel": "pointer" }), 
        "item-1"
    )), 
    &meta()
).await.expect("interactionHover");
```

**JSON Schema:**
```json
{
  "domainId": "string",
  "channel": "string (defaults to 'pointer')",
  "targets": "JSON stringified array: JSON.stringify([{granularity, id}, ...])"
}
```

---

## B. Node Graph → Plugin

### B.1. NodeGraph Action Dispatch (🧰️framework/.../NodeGraph/🟦️component.tsx:714, 763)

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/NodeGraph/🟦️component.tsx:147-150`

```typescript
export function nodeGraphHoverActionArgs(nodeId: string | null | undefined) {
  const targets = nodeId ? [{ granularity: "node", id: nodeId }] : [];
  return { domainId: "graph", channel: "pointer", targets: JSON.stringify(targets) };
}
```

**Dispatch Sites:**
- Line 714: `dispatch(nodeGraphActions.hover, nodeGraphHoverActionArgs(hovered));`
- Line 763: `dispatch(nodeGraphActions.hover, nodeGraphHoverActionArgs(hovered));`
- Line 2024: `dispatch(nodeGraphActions.hover, nodeGraphHoverActionArgs(hovered));`
- Line 2173: `dispatch(nodeGraphActions.hover, nodeGraphHoverActionArgs(...));` — for port hover

**Answer:** YES — node graph dispatches `nodeGraphActions.hover` which resolves to `"interactionHover"` (from line 434 in mesh/.ts). Action args always have shape:
```json
{ "domainId": "graph", "channel": "pointer", "targets": "[{granularity, id}, ...]" }
```

### B.2. Port ID Format (NodeGraphPickChannel)

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/NodeGraph/🟦️component.tsx:115-120`

```typescript
export function nodeGraphPickChannel(target: Pick<CanvasPickTarget, "domain" | "id"> | null): { readonly nodeId: string; readonly portId: string } | null {
  if (target?.domain !== "handle") return null;
  const boundary = target.id.indexOf("@");
  if (boundary <= 0 || boundary === target.id.length - 1) return null;
  return { nodeId: target.id.slice(0, boundary), portId: target.id.slice(boundary + 1) };
}
```

**Format:** `"nodeId@portId"` — split at the first `@` character.

### B.3. Procedural3D Schema Confirmation

**Location:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:372-373`

```rust
inputs: node.inputs().iter().filter(|port| port.visible)
    .map(|port| ui_wgpu::wgpu::NodeGraphPortRecord { 
        id: format!("{}@{}", node.id, port.id), 
        label: Some(port.label.clone()), 
        ..Default::default() 
    })
    .collect(),
outputs: node.outputs().iter().filter(|port| port.visible)
    .map(|port| ui_wgpu::wgpu::NodeGraphPortRecord { 
        id: format!("{}@{}", node.id, port.id), 
        label: Some(port.label.clone()), 
        ..Default::default() 
    })
    .collect(),
```

**Confirmation:** Port ID is exactly `"{}@{}"` format: `nodeId@portId`.

---

## C. Plugin → Renderer (Painting Hover Back Out)

### C.1. World3D Selection JSON

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:36579-36596`

```rust
pub fn world3d_selection_json(method: &str, ids: &[String], hovered_id: Option<&str>) -> String {
    world3d_selection_json_with_granularity(method, ids, hovered_id, None)
}

pub fn world3d_selection_json_with_granularity(
    method: &str, 
    ids: &[String], 
    hovered_id: Option<&str>, 
    granularity: Option<&str>
) -> String {
    let mut value = json!({
        "method": method,
        "mode": "replace",
        "ids": ids,
        "hoveredId": hovered_id,
    });
    if let Some(entry) = granularity {
        if let Some(object) = value.as_object_mut() {
            object.insert("granularity".into(), json!(entry));
        }
    }
    value.to_string()
}
```

**Hover Field:** The 3D world scene carries hover via `hoveredId` (single optional string, one instance at a time).

**JSON Schema:**
```json
{
  "method": "string",
  "mode": "replace",
  "ids": ["...selected instance ids..."],
  "hoveredId": "string|null",
  "granularity": "string (optional)"
}
```

### C.2. Node Graph Scene Hover

**Location:** `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs:298-301`

```rust
pub struct NodeGraphHover {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}
```

**Node Graph Scene** (line 376):
```rust
pub struct NodeGraphScene {
    ...
    pub hover: Option<NodeGraphHover>,
    ...
}
```

**Current State:** NodeGraphHover only carries `node_id`. Port hover is NOT yet implemented in the scene model.

**Note from Flow Component** (line 60-66):
```rust
// 🕹️ `render` carries no `InteractionView` ... so `selection`/`hover` are left
// at `NodeGraphScene::base`'s defaults (empty/none) — the canvas no longer paints a live
// highlight until a future wave threads interaction into scene rendering.
```

The flow window currently sets `hover: None` in the base scene (line 416).

---

## D. Domain Binding

### D.1. InteractionDefinition for "graph" Domain

**Location:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:619-639`

```rust
.interaction(InteractionDefinition {
    id: "graph".into(),
    label: LocalizedLabel::native("Graph", "Graph"),
    granularities: vec![
        GranularityDefinition { 
            id: "node".into(), 
            label: LocalizedLabel::native("Node", "Knoten"), 
            icon_id: "circle".into() 
        },
        GranularityDefinition { 
            id: "edge".into(), 
            label: LocalizedLabel::native("Edge", "Kante"), 
            icon_id: "minus".into() 
        },
        GranularityDefinition { 
            id: "handle".into(), 
            label: LocalizedLabel::native("Handle", "Griff"), 
            icon_id: "move".into() 
        },
    ],
    hierarchy: HierarchyProvider::Topology,
    hover: HoverSpec { transitive: true, ..HoverSpec::default() },
    selection: SelectionSpec {
        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
        methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
        transitive: false,
        broadcast: true,
    },
})
.window_kind_interactions(flow_window::PROCEDURAL_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
.window_kind_interactions(edit_preview::PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
.window_kind_interactions(generate_preview::PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
```

**Granularity IDs:** `"node"`, `"edge"`, `"handle"`

**HoverSpec:** `{ transitive: true, ..default() }` — transitive hover is enabled.

**Windows Bound:** Flow window, edit preview, generate preview (all three share the "graph" interaction domain).

---

## Implementation Gap Analysis

1. **3D World:** Does NOT dispatch `interactionHover` — only plugin-specific `setHover` / `referenceHover` / `worldVortexHover`.
   - To enable bidirectional hover, 3D world must dispatch `interactionHover` with domain/channel/targets.

2. **Node Graph:** Already dispatches `interactionHover` correctly with `domainId: "graph"`, `channel: "pointer"`, and `targets` JSON.

3. **Port Hover in Scene:** NodeGraphHover struct only has `node_id`, no port field.
   - To paint port hover back to renderer, expand NodeGraphHover to include optional `port_id: Option<String>`.

4. **Flow Window Scene:** Currently leaves hover at default `None` (commented as "discovered framework gap").
   - Must thread InteractionView into render to populate hover state in scene.

