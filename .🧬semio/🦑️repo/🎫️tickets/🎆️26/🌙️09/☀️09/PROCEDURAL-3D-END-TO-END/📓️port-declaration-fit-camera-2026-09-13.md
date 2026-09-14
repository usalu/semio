# Port Declaration & Fit Camera — one id per wire end, one camera per surface (2026-09-13/14)

Lane: **port-declaration-fit-camera**. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

---

## TL;DR

Two defects, two owning layers, both closed.

1. **`extrusion-axis` declared `vector`/`x`/`y`/`z` on BOTH sides** (`📓️wgpu-node-graph-gestures-2026-09-13.md`
   §8, `📓️node-graph-wire-drag-2026-09-13.md` §1.5). The owner is not `math.vector` and not the renderer:
   it is `schema_component_info` in the neural engine, which built every auto-registered schema
   component's OUTPUT channels with the **same ids as its inputs** — and 25 hand-written operators
   repeated the pattern. **38 of the catalogue's operators** were ambiguous, not one.
   The fix is `produced_channel_id(input) = "{input}Out"` at that layer, the schema component's own
   emitted keys, the 25 hand-written declarations and their emit sites, `math.move`'s misnamed input
   (`vector` → `offset`), and the example DSLs/fixtures by hand. Display names are untouched.
   The law — **no operator in the catalogue declares one port id on both sides** — runs over the LIVE
   first-party catalogue (**188 operators**) with a TypeScript twin over `{nodeId}@{portId}`.
2. **`Fit graph` re-published a stale camera on wgpu.** `node_graph_fit_camera` computed the fit into
   the DAG's derived paint copy and read the answer back off the **FlowHost** authority, so the
   control persisted the scene's own boot camera forever. Fixed by giving the flow surface ONE camera
   authority: `FlowHost::{fit_camera_to_content, adopt_camera_or_fit, refit_camera_if_content_left_view}`
   mirror the dag's answer into `fixture.camera`, and the three wgpu call sites route through them.

Runtime: on **6018** a press on `extrusion-axis@vectorOut` resolves to `extrusion-axis@vectorOut`
(the output), `Fit graph` publishes `x=20.114 y=-132.897 zoom=0.9165` and all 7 nodes are inside the
pane; on **6118** `Fit graph` publishes `x=20.114 y=-132.897 zoom=0.9250` instead of the scene's
`x=94.756 y=-97.508 zoom=1.784`. `example-geometry` is **18 passed / 0 failed**, and the 23-step React
journey converges with **hex 3 meshes, every other example 1, No example 0**.

---

## 1. Defect 1 — one port id named two wire ends

### 1.1 Root cause, file:line

`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:620` — `schema_component_info`:

```rust
let mut inputs  = vec![ChannelSpec::requires(&schema.id, …)];              // "vector"
for field in &schema.fields { inputs.push(ChannelSpec::requires(&field.key, …)); }   // "x" "y" "z"
let mut outputs = vec![ChannelSpec::provides(&schema.id, …)];              // "vector"  ← same id
for field in &schema.fields { outputs.push(ChannelSpec::named(code, abbr, &field.key, …)); } // "x" "y" "z"
```

and `SchemaComponent::success_output` / `error_output` (same file, ~`:757`/`:775`) emitted the result
dictionary under those same keys. Every schema whose module is not `core` and that has fields gets one
of these operators at `Registry::finalize` (`:1480`), so `math.vector`, `math.point`, all eight
`bim.*`, `brep.geometry`/`edge`/`face`/`vertex`/`text` and `draw.draw.drawing` were all ambiguous by
construction. `brep.brep` is hand-registered with the same shape
(`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:956`).

A wire endpoint has exactly one public name — `"{nodeId}@{portId}"`. `DagHost` mints one engine handle
per port and keys `handle_key_map` on that string, `🕸️NodeGraph/🟦️.tsx:166` builds the same string for
every hit target, and the guest's `connect`/`disconnect` vocabulary addresses ports by that id. An
operator that declares one id on both sides therefore gives **two handles one key**, and the previous
lane measured the consequence twice: a press on the output `z` journalled `targetPortId: "y"`, and
`connection_hit_world` snapped the press 40 world units across the node.

The renderer half of that was already fixed by the wire-drag lane (`handle_id_for_port_side`, side-aware
resolution). **This lane fixes the contract, so the ambiguity no longer exists to be resolved.**

### 1.2 RED, before the change

The law's predicate over the SHIPPED extension catalogues (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/*/🔣️.json`,
the packed manifests that were on disk) — full list in
`🗑️generated/port-fit/red-port-sides-shipped-catalogue.txt`:

```
TOTAL offenders: 38
bim.building … bim.window (8)     brep.brep, brep.edge, brep.face, brep.geometry, brep.text, brep.vertex
brep.eval.curveClosestParameter, brep.eval.surfaceClosestUv, brep.measure.closestPoint, brep.surf.offset
brep.util.convertToNurbs, brep.xform.{copy,mirror,rotate,rotateAbout,scale,translate}
dictionary.{remove,set,unpack}    draw.draw.drawing    list.{append,remove,set}    logic.not
math.move, math.passThrough, math.point, math.vector    text.upper
```

After the change the same predicate over the **live** registry finds **0 of 188**.

### 1.3 The fix, at the owning layer

**The rule** (stated in the fixture's `provenance.rule`): an operator's input ids and output ids are
disjoint. Inputs keep the plain noun; an output that would repeat one of its OWN operator's input ids
is spelled with the `Out` suffix — `produced_channel_id` — and `code`/`abbreviation`/`fullName` are
untouched, because this is an identity, not a label. Where an INPUT was simply misnamed, the input is
corrected instead, and the fixture names each such correction.

| file | change |
|---|---|
| `🧠️neural/⚙️engine/🦀️.rs` | new `produced_channel_id`; `schema_component_info` outputs and `SchemaComponent::{success_output,error_output}` keys go through it. One edit covers all 14 auto-registered schema components. |
| `🌊️flow/📐️brep-geometry/🦀️.rs` | `out_face_result` / `out_geometry_result` / `out_point_result` beside the existing `out_*`; `BrepDeconstruct` emits `brepOut` |
| `📐️brep/🦀️.rs` | `brep.surf.offset` → `faceOut`; `brep.xform.{translate,rotate,rotateAbout,scale,mirror,copy}` and `brep.util.convertToNurbs` → `geometryOut`; `brep.eval.{curveClosestParameter,surfaceClosestUv}` and `brep.measure.closestPoint` → `pointOut`; `brep.brep` → `brepOut` (its `vertex`/`edge`/`face` lists are NOT suffixed — it is not given those) |
| `🧮️math/🦀️.rs` | `math.passThrough` → `numberOut`; **`math.move`'s input `vector` → `offset`** — the displacement is an offset, the name `brep.xform.translate` already gives it, and its outputs `point`/`vector` then need no suffix |
| `📃️list/🦀️.rs`, `📖️dictionary/🦀️.rs`, `📝️text/🦀️.rs`, `🧠️logic/🦀️.rs` | `listOut`, `dictionaryOut`, `textOut`, `booleanOut` on the three/three/one/one colliding operators only |
| example DSL assets (3) | `extrusion-axis@vector->extrude@vector` → `extrusion-axis@vectorOut->extrude@vector`, and the same in `rectangle-extrude-volume` and `face-sweep-extrude` |
| fixtures (5) | the wgpu node-graph scene, the flow draw-list scene, the node-graph-gestures provenance, the world scene-bridge eval ids, the ui surface-host retention OUTPUT tree |

`core.variable` is the ONE exemption and the law names it: its `*` is a marker the flow bridge and the
flow host replace with the variable's own name (`🌊️flow/🌉️bridge/🦀️.rs:57`,
`🌊️flow/🖥️host/🦀️.rs:1347`) before a handle exists, so no node ever carries a `*` handle.

### 1.4 Laws

Fixture — **created**: `🧠️neural/⚙️engine/🧫️fixtures/🔌️port-sides/🔣️.json`, 17 rows (each collided
family, the wildcard marker, the corrected input, and two controls that never collided), plus `suffix`,
`wildcardMarker` and `correctedInputs`.

| law | where | what it pins |
|---|---|---|
| `no_operator_in_the_catalogue_declares_one_port_id_on_both_sides` | `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | the LIVE first-party catalogue, **188 operators**, 0 offenders |
| `every_named_port_side_row_is_the_catalogue_the_extensions_register` | same | every fixture row equals the live operator, and a suffixed output's plain noun IS one of its own inputs |
| `a_renamed_output_keeps_the_display_name_it_always_had` | same | `math.vector`'s `vectorOut` still reads `VE`/`vec`/`Vector` |
| `schema_component_info_declares_tri_modal_ports` (extended) | `🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | `["pointOut","xOut","yOut","zOut","errors"]` and input/output disjointness |
| 🔌️ operator port sides — 3 tests | `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | every port of an operator gets its own `${nodeId}@${portId}`; the suffix appears only against one of its own inputs; the wildcard and the corrected input |

Counts: **3 new Rust laws + 1 extended + 3 TypeScript tests**, all green
(`5 passed` in the flow-host filter, `7 passed` in the renderer filter including the camera twins).

### 1.5 The hex column still evaluates identically

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test
example-geometry` → **18 passed; 0 failed** (13.76 s), including
`hexagonal_mushroom_column_evaluates_to_the_analytic_prism` (volume 3.897114317029974 against the
analytic regular-hexagon prism) and `hexagonal_mushroom_column_preview_payload_matches_the_scene_bridge_fixture`.

The evaluated chain, read off the running host, carries the vector across the renamed wire:

```
"extrusion-axis": { "in": { "z": {"value":6.0} },
                    "out": { "vectorOut": {"$schema":"vector","x":0,"y":0,"z":6}, "xOut":…, "yOut":…, "zOut":…, "errors":… } },
"extrude":        { "in": { "wire": …, "vector": {"$schema":"vector","x":0,"y":0,"z":6} },
                    "out": { "solid": {"$schema":"geometry","kind":"solid","handle":"d1fb4e85…"} } }
```

### 1.6 The oracle now convicts a lost mesh (coordinator's request)

The coordinator's 01:10 React journey lost geometry (hex 3→2, rectangle-extrude 1→0, face-sweep 1→0)
against a stage my `activate-2`/`activate-3` had produced. **That stage was half-built**: those two runs
ended `EXIT=130` with `@semio-tech/flow-plugin:component-dev` failing, so the extension catalogues were
new (`vectorOut`) while the procedural guest still carried the old `extrusion-axis@vector->` DSL — the
wire's source port no longer existed, `extrude` never received its axis, and a node with no input still
reports `ok`. The cause of the failure was **not** the port rename: `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
was missing `semio-framework-tool-run` (see §4).

The oracle nevertheless deserved the sharper predicate, and now has it:
`delivery.minMeshes` is **replaced** by an exact `delivery.meshes` in all eight example fixtures, and
`🧩️geometry/🦀️.rs:850` asserts `payload_meshes == delivery.meshes` (and instances equal it too) instead
of `>=`. Per-example volume was already exact (`expect.volume`, every example but the wire preview,
which has no volume). Committed counts, measured from the live delivery: **hexagonal 3, every other
example 1**.

---

## 2. Defect 2 — `Fit graph` published a camera it did not compute

### 2.1 Root cause, file:line

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2530`:

```rust
NodeGraphEngine::Flow(host) => host.dag.fit_camera_to_content()
    .then(|| [host.fixture.camera.x, host.fixture.camera.y, host.fixture.camera.zoom]),
```

It computes into one camera copy and reads back a different one. A flow surface carries two:

* `FlowHost.fixture.camera` (`🌊️flow/🖥️host/🦀️.rs:154`) — the **authority**. `screen_to_world_point`
  projects with it, `build_dag_fixture_v1` re-seeds the dag copy from it on every `rebuild_dag`, and the
  renderer publishes it as `nodeGraphViewport`.
* `DagHost.fixture.camera` — the **derived paint copy**; `FlowHost::paint_scene` delegates straight to it.

`DagHost::fit_camera_to_content` (`🕸️dag/🦀️.rs:3725`) computes the correct fit from current data and
calls `DagHost::set_camera`, which writes only the dag copy. Nothing propagated it up, so the shell's
control (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6637`) persisted the pre-fit camera — the scene's own
`{94.7558, -97.5083, 1.7844}` — through `nodeGraphViewport`, and `:2017` re-applied it, clobbering the
fitted paint copy. Two siblings shared the flaw and would be silently reverted by the next
`rebuild_dag`: `:2015` (`adopt_camera_or_fit` on first viewport) and `:2030` (`refit_camera_if_content_left_view`).

### 2.2 RED, before the change

With `FlowHost::fit_camera_to_content` reverted to the pre-fix one-liner (`self.dag.fit_camera_to_content()`),
the new law fails on the very first row with §8's own numbers:

```
a_fitted_flow_surface_publishes_the_camera_it_computed … FAILED
assertion `left != right` failed: fit-after-the-boot-camera-publishes-the-camera-it-computed:
  a fit that reports success must move the published camera off the pre-fit one
  left:  [94.75581571737445, -97.50833134679668, 1.78443256160111]
  right: [94.75581571737445, -97.50833134679668, 1.78443256160111]
```

### 2.3 The fix, at the owning layer

One camera authority for the node-graph surface. `🌊️flow/🖥️host/🦀️.rs:555` gains `FlowHost::camera()`,
a private `adopt_dag_camera()` and the three wrappers `fit_camera_to_content` / `adopt_camera_or_fit` /
`refit_camera_if_content_left_view`, each delegating to the dag and mirroring the answer into
`fixture.camera` (plus an interaction-projection refresh, so hit-testing cannot lag the paint). The wgpu
sites at `:2015`, `:2030` and `:2530` now call the host, and `node_graph_fit_camera`'s Flow arm answers
`host.camera()`. The fit RULE is unchanged and still shared — `canvas::camera::fit_camera` with
`CONTENT_FIT_PADDING_PX`, the rule React's `fitGraphToView` twins.

The two existing flow camera laws in `🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs` were moved onto the
authority methods for the same reason (they were pinning the derived copy).

### 2.4 Laws

Fixture — **extended, not forked**: `♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json` gains
`provenance.publication` and **4 `surfaceRows`** (the boot camera, a zoom-out, a pan off the graph, and
idempotence).

| law | where | what it pins |
|---|---|---|
| `a_fitted_flow_surface_publishes_the_camera_it_computed` | `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | over a REAL `FlowHost`: the fit moves the published camera, published == painted, published == `fit_camera` over the host's own content, coverage 1.0, fitting twice does not drift |
| `a_flow_surface_projects_screen_points_with_the_camera_it_published` | same | the viewport centre projects through the FITTED camera |
| `fit_graph_answers_the_fitted_camera_not_the_scene_camera` | `⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` | the renderer seam itself: `node_graph_fit_camera` ≠ the scene camera, == `host.camera()` == the painted camera, coverage 1.0 |
| `publishes the camera it computed for every surface row` | `🔬️engine-contract/🟦️.ts` | the renderer twin over the same 4 rows: the fit differs from the pre-fit camera, coverage is exactly 1, and a second fit is identical |

Counts: **3 new Rust laws + 1 TypeScript twin**, all green
(`13 passed` in the wgpu node-graph filter, `7 passed` in the renderer camera/port filter).

---

## 3. Runtime proof

Probe: `🐍️port-fit-probe.mjs` (created). It aims **only** at geometry the host itself publishes —
`window.__semioFlowGraphProbe[surfaceId].entity(…)` on React, the `wgpu node-graph geometry` census and
the `[DEBUG] shell node-graph fit` line on wgpu — and every finding below is a recorded console line or
a measured rect. Artifacts under `🗑️generated/port-fit/`.

### 3.1 React — `:6018`, `🗑️generated/port-fit/react-6` — 11 of 12

```
✓ react.graph.wiredFromTheOutput  the published document's wire: extrusion-axis@vectorOut->extrude@vector
✓ react.handles.published         extrusion-axis@vectorOut=664,448 12x13 · extrusion-axis@vector=630,448 12x13
                                  extrusion-axis@xOut=664,461 · extrusion-axis@x=630,461 · extrusion-axis@z=630,486
✓ react.handles.sidesDiffer       vectorOut x=664 vs x x=630 — one id per side, one rect per id
✓ react.handles.staleIdIsTheInput extrusion-axis@vector now resolves to the INPUT column (x=630)
✓ react.press.resolvesOutput      [DEBUG] dag port press port=Some("extrusion-axis@vectorOut") interaction=draw-edge(new)
✓ react.fit.dispatched            nodeGraphViewport settled
✓ react.fit.publishedANewCamera   the wheel left camera { x=20.138 y=-136.458 zoom=0.5412 };
                                  Fit graph published camera { x=20.114 y=-132.897 zoom=0.9165 }
✓ react.fit.framesEveryNode       7/7 nodes measured, 0 outside the 483x814 pane
```

The two handles that share a noun now sit **34 px apart on opposite edges of the node**, and the press
aimed at the output rect is answered by the OUTPUT handle — the exact gesture that resolved to the
input before. The `✗` is `react.fit.nodeReadBackResolves`' sibling note: `entity()` answers from the
resolver's warmed cache, so its node rects do not move under a live wheel; the camera claim is read
from the host's own history journal instead, which is why that row exists.

### 3.2 wgpu — `:6118`, `🗑️generated/port-fit/wgpu-1`

```
✓ wgpu.boot                    boot_shell left
✓ wgpu.surface                 graph pane 487x814+491,54
✓ wgpu.fit.dispatched          [DEBUG] shell node-graph fit
                               {"surface":"procedural-main","x":20.11421097139491,"y":-132.89711682581412,"zoom":0.9249677671800861}
✓ wgpu.fit.notTheSceneCamera   published {x:20.114, y:-132.897, zoom:0.92497}
                               vs the scene's own {x:94.75581571737445, y:-97.50833134679668, zoom:1.7844325616011099}
```

§8's defect was that this line carried the scene's own camera. It now carries a computed one — and it
agrees with React's fit to 3 decimal places in `x` and `y` (`20.114 / -132.897`), the two renderers
answering the same shared rule at their two pane widths (487 vs 483 px → zoom 0.9250 vs 0.9165).

The renderer wasm under this run is the one this lane built
(`bunx nx run @semio-tech/framework-renderer-wgpu:wasm`, 5 m 58 s, reload only — the serve never
restarts for new wasm). The GUEST behind `:6118` is still the previously staged one: `:6118` reads
`🧑‍💻dev/🔌️plugin-modules`, staged by `activate-generation3d-wgpu-dev`, and that run failed on a peer's
break (§4.4) — so its console still carries the old `extrusion-axis@vector#0` eval ids. That is why
§3.2 claims the camera and nothing about ports; the camera is computed from node POSITIONS and is
independent of port ids.

### 3.3 The 23-step React journey — `🗑️generated/port-fit/journey`

Run after the final React restage. Meshes per step:

| step | meshes | | step | meshes |
|---|---|---|---|---|
| boot (Hexagonal Mushroom Column) | **3** | | view:No example | 0 |
| edit:No example | 0 | | view:Hexagonal Mushroom Column | **3** |
| edit:Hexagonal Mushroom Column | **3** | | view:Rectangle Extrude Volume | 1 |
| edit:Rectangle Extrude Volume | 1 | | view:Sphere Cut With Torus | 1 |
| edit:Sphere Cut With Torus | 1 | | view:Box Fillet Preview | 1 |
| edit:Box Fillet Preview | 1 | | view:Sphere Box Fuse | 1 |
| edit:Sphere Box Fuse | 1 | | view:Face Sweep Extrude | 1 |
| edit:Face Sweep Extrude | 1 | | view:Rectangle Wire Preview | 1 |
| edit:Rectangle Wire Preview | 1 | | view:Box Shell Preview | 1 |
| edit:Box Shell Preview | 1 | | generate-mode | 0 |
| generate-added | 1 | | back-to-edit | 1 |
| viewer-role | 1 | | **`DONE steps 23 meshSteps 20`** | all converged |

That is the accepted baseline exactly — hex 3, every other example 1, No example 0 — and it restores
the two examples the coordinator measured at 0.

---

## 3.4 Gates, final run

| gate | result |
|---|---|
| `-p semio-framework-os-flow --lib -- port_side no_operator… a_fitted_flow_surface a_flow_surface_projects a_renamed_output every_named_port_side draw_list` | **13 passed / 0 failed** (5 new + the 8 draw-list camera/caption laws) |
| `-p semio-framework-os-renderer-wgpu --lib -- node_graph` | **14 passed / 0 failed** (13 existing + `fit_graph_answers_the_fitted_camera_not_the_scene_camera`) |
| `-p semio-framework-os-kernel-neural-engine --lib -- schema_component` | **3 passed / 0 failed** |
| `-p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry` | **18 passed / 0 failed**, 13.76 s |
| `@semio-tech/framework-renderer-react:test -t "operator port sides\|node-graph opening camera"` | **7 passed / 0 failed** |
| `-p semio-framework-os-flow --lib` (whole crate) | 190 passed / 37 failed — the recorded 2026-09-13 baseline's same 37, see §6 |

---

## 4. Peer breaks fixed forward, and the one that caused the half-built stage

Three, all minimal, all noted here because they are someone else's wave:

1. `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` and
   `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` were missing `semio-framework-tool-run`.
   `dyn_enum_close!` copies `PluginApp::tool_run_trace_delta`'s signature verbatim into the generated
   impl, so the crate must resolve those types; `procedural`'s own Cargo.toml already carries the dep
   with that exact comment, which is what both additions copy. **The flow one is what made
   `activate-2`/`activate-3` produce the half-built stage behind the coordinator's lost meshes.**
2. `Cargo.lock` was behind three peer Cargo.toml edits that had added `semio-framework-tool-run`
   (`--locked` plugin builds could not start at all). Refreshed with `cargo metadata --offline`.
3. `semio-framework-plugin` was mid-wave twice (`pattern does not mention field tool_run`, then
   `variant AppFrame::Ephemeral has no field named tool_run`). **Not patched** — waited it out; the
   peer's wave landed and the crate compiles.
4. `activate-generation3d-wgpu-dev` is currently **blocked by the same peer wave** and was left
   blocked: `@semio-tech/framework-renderer-wgpu:generate-frame-worker` fails with
   `WGPU browser import is not schema-owned: ../⏯️tool-run/🟦️.ts in 🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`
   — a module-ownership policy the peer's `tool_run` refactor has not satisfied yet. Patching another
   lane's ownership policy mid-wave is not a fix-forward, so the `:6118` guest stays on its previous
   stage (`activate-generation3d-react-dev` is green and staged the React side fully). The
   `procedural-plugin:materialize-dev` failure in the same run is that run's SIGINT (empty probe
   stderr, 60 s budget); the identical step is green in the React activation.

---

## 5. Files

**Changed**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` — `produced_channel_id`, `schema_component_info`, `SchemaComponent::{success_output,error_output}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — the tri-modal port law extended with the disjointness assertion; three call sites renamed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `camera()`, `adopt_dag_camera()`, `fit_camera_to_content()`, `adopt_camera_or_fit()`, `refit_camera_if_content_left_view()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — `//#region 📷️CameraAuthority` (2 laws) and `//#region 🔌️PortSides` (3 laws); the hexagonal fixture's `e5` rewired
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` — `out_face_result`/`out_geometry_result`/`out_point_result`, `BrepDeconstruct`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs` — the camera laws moved onto the authority
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/🎬️draw-list/🔣️.json` — `e5` rewired
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — three Flow camera call sites
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` — `fit_graph_answers_the_fitted_camera_not_the_scene_camera`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json` — `e5` rewired
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json` — the provenance's output list
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — the surface-row camera twin and the three port-side twins
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — one docstring reference
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json` — `provenance.publication` + 4 `surfaceRows`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json` — 8 eval-channel ids
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🪪️surface-host-retention.json` — the two OUTPUT port trees (the two INPUT trees left alone)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/{📐️brep,🧮️math,📃️list,📖️dictionary,📝️text,🧠️logic}/🦀️.rs` and their `🧪️tests/🔬️unit/🦀️.rs`, `🏗️bim/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` — `e5` rewired
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/📚️examples/{🍄️hexagonal-mushroom-column,📦️rectangle-extrude-volume,🧹️face-sweep-extrude}/🖼️assets/**/🗣️.dsl.semio`
- all eight `📚️examples/*/🧫️fixtures/🧩️example/🔣️.json` — `delivery.minMeshes` → exact `delivery.meshes`
- `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` and `🟦️.ts` — the exact mesh-count assertion
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`, `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml`, `Cargo.lock` — the peer fix-forwards of §4

**Created**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧫️fixtures/🔌️port-sides/🔣️.json`
- `<ticket>/🐍️port-fit-probe.mjs`
- this report

---

## 6. What is NOT claimed

- **The wgpu press on `extrusion-axis@vectorOut` is not runtime-proven.** Two reasons, both recorded:
  the `:6118` geometry census publishes once and is then quiet (the same sweep failure the gestures lane
  recorded in its §5), and the `:6118` guest is staged by `activate-generation3d-wgpu-dev`, which is
  blocked by a peer's in-flight `tool_run` wave (§4.4) — so that serve still runs the pre-rename guest.
  The wgpu press is covered by the Rust laws over the real `DagHost` and by the React run on the
  identical wasm host; §3.2 claims only the fit, which does not depend on port ids. **Re-running
  `activate-generation3d-wgpu-dev` once the peer's wave lands is the one open item of this lane.**
- **The extension crates' own unit suites are red on this tree and were already red.** Every one of them
  (math, list, dictionary, text, logic, bim, brep) fails with
  `final Dictionary ownership must be explicitly retired or owned by a cold boundary`
  (`🧠️neural/⚙️engine/🦀️.rs:101`) on tests this lane never touched (`add_sums_number_dictionaries`,
  `greater_compares_numbers`, `concat_joins_text`). The renamed call sites in those files are corrected
  but not proven green there; the port contract itself is proven by the flow-host catalogue law, which
  builds the same registries from the same `register()` functions.
- **`descriptor_is_fresh` was not made green.** The checked-in `🛂️.descriptor.semio` / `🔣️.json` at each
  extension's owner root are build outputs that `component-dev` does not write back (the math one is
  unchanged since 2026-09-13 06:39, before this lane). The STAGED artifacts the serves actually read
  were rebuilt and verified to carry the new ids.
- **`semio-framework-os-flow --lib` is 190 passed / 37 failed**, against the 2026-09-13 22:45 baseline of
  184/37 in `📓️remaining-suite-reds-2026-09-13.md` — the same 37, plus this lane's 5 new passes and one
  that a peer turned green. `hexagonal_mushroom_fixture_reports_extruded_solid_output` is among the 37:
  it asserts `handle.starts_with("solid-")` while the kernel now mints content-hash handles
  (`d1fb4e85…`); the chain it exercises evaluates correctly (§1.5) and the stale assertion is the
  kernel lane's.
- **No React source was changed** beyond one docstring; React's `Fit graph` was re-checked, not touched.
- Zoom bands, multi-graph layouts, and the minimap were not re-measured.
