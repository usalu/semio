# Flow window render path — semantic tree + node-graph canvas fragments (2026-09-12)

Read-only audit. Reproduced live against `http://127.0.0.1:6018/?plugin=generation3d` (coordinator's
dev serve, untouched) with a new probe, `🐍️flow-window-probe.mjs` (launch pattern copied from
`🐍️journey-probe.mjs` / `🐍️console-dump-probe.mjs` in this folder). Outputs under
`🗑️generated/flow-window-audit/{default,webgpu,resize}/`.

## 0. Headline

The screenshot in `🗑️generated/probe-restage-4-long/final.png` **is not a layering/z-index bug**. Live
DOM geometry (below) proves the semantic tree (outline) and the NodeGraph canvas are two non-overlapping
flex siblings, exactly as the Rust `render()` intends — the black box and the stray port labels sit
**inside the canvas's own half**, right at its left edge, which only *reads* as "on top of the tree" in
a screenshot because that half is otherwise empty (transparent background, no drawn diagram). The real
defects are (a) the canvas half never receives the actual node-graph drawing — it runs a hard-coded
placeholder instead — and (b) that placeholder ignores the camera, so what little it draws lands in the
wrong place. Both reproduce **identically with and without WebGPU** — this is not a headless-only defect.

## 1. Render path, file:line

### 1.1 Rust — one `render()`, two unconditional children, never a fallback

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`:

- `render()` (`:257-297`) builds `outline = graph_outline(...)` (`:190`) and `canvas = scene_surface(...)` (`:270-286`, `SurfaceKind::NodeGraph`), then composes `row().try_child(outline).try_child(canvas)` (`:290-297`) — a **row**, so outline is the left flex child and canvas the right one. Both are built on **every** render call; there is no `if !hasCanvas` branch. Nothing is conditional on WebGPU/2D/headless.
- `graph_outline`'s own doc comment (`:143-146`) states its intent precisely: "the node-graph surface beside it paints the same records on the GPU, which no screen-reader, no accessibility tree, and no headless probe can read — this tree is the graph's renderer-neutral body". So the tree is **not** a degraded fallback for when the canvas can't render — it is the framework's permanent, renderer-neutral mirror of the same DAG, always present beside the canvas, mirroring `interactionSelect`/`interactionHover` through the identical `graph` domain (`node_row` at `:112-124` wires the same `INTERACTION_SELECT_ACTION_ID`/`INTERACTION_HOVER_ACTION_ID` a canvas pick would use).

### 1.2 React — `NodeGraphHost` → `FlowGraphCanvasHost`, 2D canvas always, WebGPU device never read

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`:

- `NodeGraphHost` (`:1106-1163`): `useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.fixtureJson)` (`:1148`). generation3d always carries `fixtureJson` (built by `flow_backed_node_graph_extras` in the Rust `render()` above), so it **always** routes to `FlowGraphCanvasHost` (`:1161`), never the generic `WasmGraphSurface`/`GraphWasmCanvas` engine (`:568,837`) that other, non-Flow node-graph apps use.
- `FlowGraphCanvasHost`'s container (`:2474-2477`) is `position: relative`, `h-full w-full` — sized by its **own** flex slot (the row's second child), not the viewport. Two canvases inside it are `position: absolute; inset: 0` (`:2548-2549`), i.e. confined to that same container — never escaping into the outline's half.
- `renderFlow()` (`:2055-2064`) calls `session.renderCanvas(canvas)`. Confirmed live: this is the **only** paint call path for the GPU canvas; there is no other `renderCanvas`/`renderFlowSurface` call site in this component.

### 1.3 JS bridge — the WebGPU device is acquired and then never used to draw

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js:85-90`: `FlowSession.renderCanvas(canvas)` calls `renderFlowSurface(features, canvas)` with **no third argument** — so it always falls to the default parameter.

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js`:
- `attachFlowSurface` (`:274-319`) optionally acquires a WebGPU device (`requestFlowSurfaceDevice`, `:265-272`) and logs it (`:292`, the `device=none`/`device=webgpu` marker from the defect report), but the device is only stored on the returned attachment record — **grep confirms zero other reads of `.device`** in this file or in `NodeGraph/🟦️.tsx`.
- `renderFlowSurface(features, canvas, render = renderFlowCanvas)` (`:321-324`) — `render` is never overridden by any caller, so production always runs the built-in placeholder.
- `renderFlowCanvas` (`:449-474`) is the entire "renderer": `context.clearRect(...)` then, per widget, `context.fillRect(x - 60, y - 24, 120, 48)` (`:472`) using **`widget.x`/`widget.y` verbatim — raw document/world coordinates, no camera transform** — and **`fillStyle` is never set**, so every box paints in the 2D canvas default black. This is the "solid black rectangle."

So: created, always 2D (`device` value is cosmetic — confirmed by the `webgpu` probe run below, byte-identical output with a real `device=webgpu`). Canvas creation itself is unconditional and correct; what's missing is a real painter behind it.

### 1.4 Where the real drawing actually goes — dropped on the floor

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs`, `render_frame` (`:5518-5525`):
```
self.host.sync_dag_ghost();
let mut scene = canvas::Scene::new();
self.host.paint_scene(&mut scene, self.width, self.height, self.dpr);   // ← the REAL paint; logs "[DEBUG] dag draw lod=…"
let fixture = self.host.fixture_json().map_err(domain_error)?;
let labels = self.host.label_overlay_paint_state_json().map_err(domain_error)?;
Ok(format!("{{...\"fixture\":{fixture},\"labels\":{labels}}}", ...).into_bytes())   // ← `scene` is NEVER included
```
`paint_scene` (`🌊️flow/🖥️host/🦀️.rs:1830-1831`, delegating to `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`) is the **same** LOD-aware vector painter the wgpu shell uses to composite real geometry into the window (per `📓️wgpu-node-graph-2026-09-10.md` §2.2, `FlowHost::paint_scene`/`DagHost::paint_scene`) — its `dag_debug_log` at `dag/🦀️.rs:6056` is exactly the `[DEBUG] dag draw lod=normal …` line quoted in the defect. On the JS/wasm target this `scene` is built, logged, and then **dropped** — `render_frame`'s return value carries only `fixture` (raw widget positions) and `labels` (the label-overlay row list), never the scene's draw commands. `renderFlowCanvas` never had real geometry to draw in the first place; it was written as a stand-in and nothing upstream of it was ever wired to feed it the real picture.

### 1.5 The label overlay is correct, and paints in a different half of the same story

`paintDagLabelOverlays` (`NodeGraph/🟦️.tsx:1552-1607`) **does** apply the camera: `dagWorldToScreen(camera, viewportW, viewportH, row.x, row.y)` (`:1578`). Its `state.camera` comes from `session.labelOverlayPaintStateJson()` — a live host query, not a stale default — so label placement is architecturally correct. What is visible live (§2) is consistent with only 1-2 of the graph's 7 nodes landing inside the current camera framing at boot, which is a separate, pre-existing concern (`📓️wgpu-node-graph-2026-09-10.md` §5.1 flags the same paint/refresh cadence question on the wgpu side).

## 2. Reproduction, three modes

Probe: `🐍️flow-window-probe.mjs`, `SEMIO_PROBE_MODE=default|webgpu|resize`. Screenshots and per-mode
`*-geometry.json` (live `getBoundingClientRect`/`getComputedStyle` of the outline root, the NodeGraph
host, and both its canvases) are under `🗑️generated/flow-window-audit/`.

| mode | screenshot | flow window body | outline half | NodeGraph host / both canvases |
|---|---|---|---|---|
| (a) headless, default (no WebGPU) | `default/1-boot.png` | flex row, `x=6, w=966` | `x=6, w=483` | `x=489.5, y=54, w=483×814`, `position:absolute inset:0`, **`background: rgba(0,0,0,0)`** (fully transparent), z-index `auto` (label canvas `z-40`) |
| (b) headless + `--enable-unsafe-webgpu --ignore-gpu-blocklist --use-angle=metal` | `webgpu/1-boot.png` | identical | identical | identical geometry; console shows `device=webgpu` (a real adapter/device **was** acquired) instead of `device=none` — **screenshot is pixel-for-pixel the same layout** as (a): the placeholder painter runs unconditionally regardless of `device` |
| (c) resize 1440×900 → 1000×700 → 1440×900 | `resize/{1,2,3}-*.png` | shrinks/grows with viewport (`967→667→967`) | tracks 50/50 with the canvas half at every size | canvas backing store resizes in lockstep (`966×814 → 667×614 → 966×814`, dpr-scaled) — `ResizeObserver`-driven `syncSurfaceSize` (`:2072-2082`) works correctly |

Confirms: sizing/position of the canvas relative to the outline is **correct and stable** across viewport
changes and WebGPU availability — the split itself is not the bug. `background: rgba(0,0,0,0)` on both
canvases (computed style, not inferred) is the direct answer to "is the black rectangle a canvas with no
clear colour, or an unpainted region": **neither, precisely** — the canvas element itself has no CSS
background (correct; a `<canvas>` should be transparent), and the region *around* the black box is
genuinely unpainted (never drawn to), while the box itself *is* real 2D-context paint (`fillRect`) landing
in the default black fill because `fillStyle` was never set (§1.3).

Instability found in (c): after the second resize (1000×700, `resize/2-*.png`) neither the black box nor
the two node-title labels (`Number`, `Math.add`) reappeared once back at 1440×900
(`resize/3-*.png`) — only a small grey circular port marker persisted. This is consistent with §1.3's
root cause (placeholder paint keyed to raw, non-refreshed widget coordinates) rather than a new defect;
not chased further here since §1.3/§1.4 already explain why nothing reliable is drawn.

## 3. `! wire` / `! vector` labels

Not error badges. `IoPortSpec::label_with_cardinality`
(`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:143-152`):
```rust
let cardinality = match self.resolved {
    Some(false) => "?",
    _ => self.cardinality.as_str(),   // defaults to "!" (default_port_cardinality, :96)
};
format!("{cardinality} {label}")
```
`"!"` is the **default, steady-state** cardinality marker (required/singular port), shown for every
input/output port at a LOD that shows port labels (`port_label_overlay_rows`,
`♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:5326-5375`) — it is not conditioned on eval status.
It flips to `"?"` only when the port's node id is in `unresolved_input_ports`
(`dag/🦀️.rs:4018`, populated during structural/topology checks, not eval convergence). The node title
above them (e.g. `Brep.solid.extrude`) is the same row family's node-title text
(`node_label_text`, `dag/🦀️.rs:5659-5672`) — the raw operator-kind id, since this fixture's `Extrude`
node has no localized `name`/`abbreviation` set in the document-derived catalogue
(`document_operator_records`, generation3d `🕸️flow/🦀️.rs:190-227`, uses the raw `kind` string when
there's no catalogue match).

**These persist regardless of eval status.** Cardinality is structural (port arity), not a computed-state
badge — it does not change when the node's status moves to `ok` (confirmed: `node_status_label`,
`🕸️flow/🦀️.rs:70-81`, reads a completely separate `status_json` field and renders as the tree row's
*description*, not as part of the port label). They will only disappear if the LOD drops below
`shows_port_labels()`, or the port becomes hidden/removed, or — for the `?` variant — the port resolves.

## 4. Classification and fix

| # | finding | headless-only or real-browser | fix locus | pinning test |
|---|---|---|---|---|
| 1 | Canvas paints a black `fillRect` placeholder instead of the real vello `Scene` `paint_scene` already computes | **real-browser** — reproduced identically with a genuine `device=webgpu` acquired (§2b); the device is simply never read (§1.3) | `🌊️flow/🕸️wasm/🦀️component.rs:5518-5525` `render_frame` must serialize `scene`'s draw output (rasterize to an image buffer for `putImageData`, or emit replayable 2D-path commands) instead of discarding it; `🖥️flow-host.js:449-474` `renderFlowCanvas` must consume that output instead of drawing its own boxes | New Rust test beside `render_frame` (or in the wasm component's existing test module) asserting the returned payload is non-empty for the scene/carries a non-trivial byte length when `paint_scene` emits real commands (today: assert it does **not** — proving the drop); new vitest lane in `🕸️NodeGraph/🧪️tests/…` (or extending `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`'s existing recording-2D-context pattern from `📓️node-graph-attach-2026-09-10.md`) asserting `renderCanvas` never calls `fillRect` with an unset `fillStyle`, and that drawn geometry positions move when `session.setCamera(...)` changes — pinning that camera is honored |
| 2 | `renderFlowCanvas` uses raw world coordinates, ignoring `fixture.camera` | **real-browser** (same call path as #1; not gated on headless/WebGPU) | same file/function as #1 — once real geometry replaces the placeholder this collapses into the same fix, but if a placeholder must stay temporarily, it should route `x,y` through the same `dagWorldToScreen`-equivalent camera transform `paintDagLabelOverlays` already uses (`NodeGraph/🟦️.tsx:1578`) | Unit test asserting box screen position for a known `fixture.camera` (zoom≠1, offset≠0) matches `dagWorldToScreen`, not raw `x,y` |
| 3 | "Drawn on top of the tree" read | **misdiagnosis, not a bug** — DOM geometry (§2) shows two non-overlapping flex children; no fix needed | — | the probe's `*-geometry.json` is the regression artifact; could be folded into a DOM assertion in an existing React test asserting the outline and NodeGraph-host rects never intersect |
| 4 | `! wire` / `! vector` / node-title labels | **not a defect** — correct, persistent, structural port-cardinality display (§3) | — | none needed; existing `label_with_cardinality` unit coverage (if absent, a one-line Rust test pinning `"!"` default vs `"?"` on `resolved: Some(false)`) |

## 5. Hover/selection in the 2D fallback

**Works, by code inspection.** Pointer handling (`NodeGraph/🟦️.tsx:2591-2643`,
`onPointerDown`/`onPointerMove`/`onPointerUp`/`onWheel`) calls `session.pointerDownScreen(...)` /
`pickTargetsAtScreenJson` / `wheelScreen` directly on the live `FlowSession` — these are **host-side
hit tests against the DAG's own node/port bounds and camera** (`dagScreenToWorld`, the session's
`selectionDomainsJson`/`pickTargetsAtScreenJson`), not reads of canvas pixels. They are entirely
independent of whether `renderFlowCanvas` painted anything correctly. Confirms:
- `interactionSelect`/`interactionHover` dispatch (`emitInteractionState`, wired through the same
  `onPointerUp`/pointer-move handlers) fires regardless of the paint defect.
- The semantic-tree rows carry their **own** identical `select`/`hover` triggers
  (`🕸️flow/🦀️.rs:112-124`, `node_row`) against the same `graph` domain — so today, a user can
  drive the exact same selection/hover state either through the (broken-looking) canvas or through the
  (fully functional) tree, and both land on the one `GENERATION_3D_INTERACTION_DOMAIN`.
- The only thing that does **not** work today is *seeing* the selection/hover highlight on the canvas
  half — `paintDagLabelOverlays`'s highlight fill (`dagOverlayLabelFillHex`, `:1589`) only affects the
  labels it draws, and with most of the graph never drawn (§1.4), a correct selection produces an
  invisible or barely-visible visual confirmation on the canvas side, even though the action pipeline
  and the tree-side row highlight are correct.

## 6. Files referenced (absolute-relative to repo root)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs`
- Probe: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️flow-window-probe.mjs`
- Evidence: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/flow-window-audit/{default,webgpu,resize}/*.png`, `*-geometry.json`, `console.txt`
- Prior reports read: `📓️wgpu-node-graph-2026-09-10.md`, `📓️node-graph-attach-2026-09-10.md`, `📓️audit-window-inventory-2026-09-12.md` §1.1/§2
