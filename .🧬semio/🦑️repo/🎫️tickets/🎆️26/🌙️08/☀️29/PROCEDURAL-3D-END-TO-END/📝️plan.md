# 🧊️ Procedural 3D End to End — Plan

## 🎯️ Objective
`procedural3d` works end to end in the `s` OS: **every output channel of every preview-enabled
widget renders its own preview in the 3D world**, **hover is bidirectional** between the node-graph
window and the 3D world window, and **all flow extensions** contribute working neuron kinds whose
geometry-bearing channels preview.

## 🔍️ Baseline (verified, not assumed)
- Preview pipeline: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  - `geometry_handles_for_widget` (:800) reads `widget_eval.get("out").or_else(|| get("in"))` and
    recursively flattens **every** handle into one unordered `Vec<String>` — **channel identity is
    lost**. Instances get ids `"{widgetId}"` / `"{widgetId}#{index}"`, label `"{widgetId}"`.
  - `preview_payload_from_eval_with_session` (:940) hardcodes `let selected = false; let hovered = false;`
    with the comment "`render` carries no `InteractionView` … until a future wave threads interaction in".
  - `render` (:434) and `context_menu` (:474) carry no `InteractionView`; `preview_selection_json`
    (:716) therefore always emits an empty `graph` selection/hover.
  - `interaction_topology` (:397) declares only `node` (widgets, nested cluster neurons) and `edge`
    (synapses) targets — no port/channel/instance targets.
- Framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
  - `InteractionView<'a>` (:9409) exposes `selection(domain)`, `hover(domain, channel)`,
    `active_granularity`, `active_mode`, `peers_selecting`, `peers_hovering`.
  - Runtime `VcsArtifactApp::render` (:24450) already materializes `interaction_state` (:24469) and
    owns `interaction_hover` + `peer_presence`, then calls
    `A::render_with_request_context(owner, body_key, doc, cfg, transient)` (:24480, :24501)
    — the view is **available at the call site and simply not passed**.
  - Only **3** files outside the framework override the `render_with_*` chain
    (`🌊️flow` editor, `🖍️draw` editor, `💠️lowpoly` editor); 313 files implement plain `render`.
- Renderer already supports both directions:
  - `World3dHost` instances carry `hovered`/`selected`; `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts:434`
    maps the mesh window's hover onto the framework verb `interactionHover`.
  - `NodeGraph` tracks `hoveredNodeId()` / `hoveredChannelJson()` and has
    `setHover(widgetId)` / `setHoverChannel(widgetId, port)`.
- Extensions: 9 flow extensions, ~146 neuron kinds; only `brep` (61) is brep-handle bearing,
  `draw` emits 2D handles, `bim` emits element handles, `math` emits points/vectors.

## 🧱️ Locked API contract (so waves can run in parallel)
```rust
// semio_framework_plugin — sync trait (~:26014) and its async twin (~:11234)
fn render_with_request_context(
    owner: &ArtifactInstanceOperationOwnerHandle,
    body_key: &str,
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
    transient: &TransientView<'_, Self::Transient>,
    interaction: &InteractionView<'_>,   // ← NEW, appended last
) -> UiAssemblyResult<ComponentTree>;
```
`render` (3-arg) and `render_with_instance_operation_owner` are **unchanged** — an app that wants
interaction overrides `render_with_request_context`. This is the trait's own existing layering
idiom (`render` → `+owner` → `+transient` → `+interaction`), not a compatibility shim.

### Channel-qualified preview identity (locked)
| target | id | granularity | parent |
|---|---|---|---|
| widget / neuron | `{widgetId}` | `node` | cluster widget id or none |
| output channel | `{widgetId}@{channel}` | `port` | `{widgetId}` |
| preview instance | `{widgetId}@{channel}#{index}` | `instance` | `{widgetId}@{channel}` |
| synapse | `{synapseId}` | `edge` | none |

`{widgetId}@{channel}` is exactly the node-graph surface's own port id form (`"nodeId@portId"`),
so graph-port hover and 3D-instance hover resolve to the same interaction target. Transitive hover
(already `HoverSpec { transitive: true }`) then makes "hover the node" light up every channel and
every instance below it for free.

## 🌊️ Waves
1. **Framework** — thread `InteractionView` into `render_with_request_context` (trait defs,
   forwarding impls, runtime call sites, the 3 external overrides, framework test fixtures).
2. **Per-channel previews** — replace handle flattening with channel-aware enumeration; one
   preview instance per (channel, index); channel-qualified ids + labels; cover point/vector
   channels, not just brep handles.
3. **Bidirectional hover** — procedural3d overrides `render_with_request_context`, reads
   `interaction.hover("graph", "pointer")` / `interaction.selection("graph")`, marks instances and
   graph nodes hovered/selected (own id or any ancestor id); node-graph hover routed into the
   `graph` domain; `interaction_topology` extended with `port` + `instance` granularities.
4. **Extensions** — every flow extension builds for `wasm32-wasip2` and registers; every
   geometry-bearing channel of every extension neuron kind previews.
5. **Verify** — native tests, wasm build, and the real app at the
   `🛠️dev🔧️procedural🏙️3d⚛️react` playground (port 6018).

## ⚠️ Stated assumption
"Every single channel is showing a preview in 3d" is implemented as: **every output channel that
carries geometry-bearing values** (brep handles, points, vectors, draw/bim handles) gets its own
preview instance. Pure-data channels (scalars, text, booleans, dictionaries) render no 3D marker —
they remain readable in the inspection panel. Flagged here rather than silently fabricating
placeholder geometry for numbers.
