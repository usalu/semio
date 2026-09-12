# Preview Mesh Delivery — why 5 of 8 examples showed no mesh (2026-09-12)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "preview mesh delivery". Repo MCP was down for the
whole run (`invalid initialize params` / `Connection closed`); ticket bookkeeping is on disk and no
ticket was opened, closed or reopened.

Probe: `🐍️mesh-delivery-probe.mjs`. Runtime outputs: `🗑️generated/mesh-delivery/run-{A..E}/`.
Native law: `…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` region `🔖️MeshDelivery` (`[[test]] example-geometry`).

---

## 1. Verdict

There was **no lost mesh, no dropped stream, no size ceiling discarding a pack and no missing
re-arm.** Every one of the eight bundled examples tessellates, transfers, decodes and publishes
correctly — measured end to end, natively, through the real extension envelope and the real session
fold. What was broken is **how many interactive round trips the delivery costs**, plus one stale
artifact and one probe artefact:

| # | cause | examples | layer |
|---|---|---|---|
| **A** | the mesh body crossed **4 KiB of base64 per whole `flowEvalTick`** — the transfer unit was pinned to the 8 KiB *gesture* quota the chain shared with 24 unrelated routes | Sphere Cut With Torus (10 round trips), Sphere Box Fuse (5), Box Fillet Preview (8) | app route contract + `brep_geometry` wire |
| **B** | the tessellate step was budgeted in **units, not time** — one measured step costs 29 µs, another 5.9 s, so cheap steps each burned a whole round trip | Box Fillet Preview (5 of its 8), Box Shell Preview (2 of its 3) | `brep_geometry` step envelope |
| **C** | the **staged `flow-extension-brep` wasm is stale**: its shell operator returns an inward-oriented outer shell, so the validate gate refuses before tessellation | Box Shell Preview | staged artifact → **restage** |
| **D** | not broken at all — the wire *does* render as a polyline; the journey probe's convergence predicate gave up first, believing `phase: idle, ratio: 1` | Rectangle Wire Preview | probe + an honest `ratio` |

A fifth, cross-cutting finding sits underneath A and D: **`ratio` lied.** `PreviewTessellateStatus::ratio`
counted only the kernel's units, so during the whole of a ten-chunk transfer the surface published
`ratio: 1` — which is exactly what made `🐍️journey-probe.mjs` call a still-streaming preview
"converged" and screenshot a blank viewport.

---

## 2. Evidence

### 2.1 The runtime, chunk by chunk

`🗑️generated/journey-5/console.txt`, correlated against `results.json`'s step windows. Every
`tessellate` answer, with its size:

| step | tessellate answers (bytes) | meshes |
|---|---|---|
| Hexagonal Mushroom Column | `361`, `2234` | **3** |
| Rectangle Extrude Volume | `116`, `1532` | **1** |
| Face Sweep Extrude | `116`, `1532` | **1** |
| Sphere Cut With Torus | `4262`, `4260`, `4260`, … | 0 |
| Box Fillet Preview | `119`, `119`, `119`, `119`, `120`, `4264`, `4258` | 0 |
| Sphere Box Fuse | `378`, `116`, `4261`, `4259`, `4259` | 0 |
| Rectangle Wire Preview | `297` (arrived 2 s before the probe moved on) | 0 |
| Box Shell Preview | `232` | 0, `phase: "invalid"` |

The `~4 260 B` answers are all the same size across *different geometry*: they are one full
`MESH_PACK_CHUNK_BASE64_CHARS = 4 * 1024` chunk plus the accounting header. The `116–120 B` answers
are `Working` progress envelopes. The answers land ~5–11 s apart, because each one is a separate
`flowEvalTick` → `InvokeExtension` → answer → full `refreshUi` cycle:

```
578988 tessellate answered (chunk 0)      →  588222 refreshUi  →  588418 flowEvalTick
590072 tessellate answered (chunk 1)      →  591654 refreshUi  →  591839 flowEvalTick
594457 tessellate answered (chunk 2)      …
```

Ten chunks at that cadence is ~100 s, far past both the probe's 60 s window and any user's patience.

### 2.2 The wire itself

A temporary `[DEBUG] tessellate wire` log in `🔌️PluginRuntime/🟦️.tsx` (request head + answer head,
`meshPack` elided) captured the envelopes directly. It has been removed again; re-add it beside the
existing `[DEBUG] extension request answered` to re-arm the probe.

`🗑️generated/mesh-delivery/run-C/tessellate-wire.txt`:

```
request={"handle":"98a97d…","tolerance":0.05,"nodeHash":4862305879059431961,"budget":24,"chunk":0,…}
answer={"done":true,"cancellable":false,"phase":"complete","unitsDone":12,"unitsTotal":12,
        "facesDone":0,"facesTotal":0,"chunk":0,"chunks":1,"packBytes":150,"meshPack":"<…
```

### 2.3 Box Shell Preview — the stale staged wasm (`🗑️generated/mesh-delivery/run-D/`)

The browser's answer is an `invalid` envelope:

```
handle 3a915ebc79a4aaed2234feee75eb4e1eb2012ad5c4101f983247895f29e6f3e2
issues [{ code: "shell-orientation-inward",
          entity: "solid-2",
          message: "outer shell's signed volume is negative (-4.096000000003073);
                    face normals appear to point inward" }]
```

Today's kernel, same DSL, same tolerance, natively:

```
[STATS] box-shell-preview handle=759ea2c50da71ab32b7fa3f14c4f3955b6bf9cc6038d8d2ecb3c177087223037
        triangles=24 closed=true boundary=0 nonManifold=0 orientationDefects=0
        volume=3.904000389 signedVolume=3.904000389 parryVolume=3.903999567
```

Two different handles for the same authored document — handles are content-hashed, so the staged
guest is computing a **different shell**. `−4.096` is exactly `−1.6³`, the INNER box's volume with an
inward normal; the current kernel produces `+3.904` (the hollow difference) and the validate gate
passes. This example needs the restage, nothing else.

### 2.4 Rectangle Wire Preview renders (`🗑️generated/mesh-delivery/run-E/`)

```
t=145340  meshes=1  data-meshes-json=203 B
          shapes=[{id: "eval-rect@wire#0", pos: 0, idx: 0, edge: 24}]  phase=idle ratio=1
```

`edge: 24` floats = four polyline segments — the authored rectangle. `mesh_has_preview_geometry`
already admits an indices-empty mesh on `edge_positions` alone, and `apply_show_mode_mesh` keeps
edges in every mode except `points`. Nothing was owed here; the journey probe simply declared
convergence 2 s before the single round trip answered.

### 2.5 The native oracle for the whole chain

`assert_delivery` (new, region `🔖️MeshDelivery`) evaluates each example's real
`🗣️.dsl.semio`, takes the preview handle, evicts the mesh cache, and then drives the **real wire** —
`brep_geometry::tessellate_step_envelope_json` answered into
`FlowEvalSession::resolve_preview_tessellate` at the LOD `preview_tolerance` gives the live preview —
counting round trips; finally it builds the published payload through
`editor::generation3d::preview_payload` with that same session. The pre-existing
`example-geometry` laws use `tessellate_geometry`, which is a `tessellate_step(usize::MAX)` loop: it
never chunks and never re-enters, which is precisely why 9/9 passed while the browser showed nothing.

**Before the fix** (`cargo test … --test example-geometry -- delivery_ --nocapture`):

| example | roundTrips | chunks | packBase64 | triangles | edgeSegments | payloadMeshes |
|---|---|---|---|---|---|---|
| rectangle-wire-preview | 1 | 1 | 136 | 0 | 4 | 1 |
| rectangle-extrude-volume | 2 | 1 | 1 368 | 12 | 12 | 1 |
| face-sweep-extrude | 2 | 1 | 1 368 | 12 | 12 | 1 |
| hexagonal-mushroom-column | 2 | 1 | 2 072 | 20 | 18 | 3 |
| box-shell-preview | 3 | 1 | 2 712 | 24 | 24 | 1 |
| box-fillet-preview | **8** | 3 | 8 408 | 92 | 72 | 1 |
| sphere-box-fuse | **5** | 4 | 15 896 | 440 | 25 | 1 |
| sphere-cut-with-torus | **10** | 10 | 38 584 | 1 114 | 45 | 1 |

Exactly the three examples with > 2 round trips are three of the five that never painted. Per-step
wall times from the same run make cause **B** explicit:

```
box-fillet-preview   stepMicros=[147895, 29, 728, 111, 30, 489, 199, 100]
sphere-cut-with-torus stepMicros=[5884783, 488, 436, 427, 806, 634, 694, 683, 432, 400]
```

A 24-unit budget let one step run **5.9 s** and made another cost **29 µs** — and each of those 29 µs
bought a whole interactive round trip. Units are not time.

---

## 3. The fix

### 3.1 The chain owns its own route (cause A)

`flowTessellateResolve` shared `Generation3dBoundedCommandJobFactory` with 24 gestures, and a factory
registers ONE `ToolExecutionContract` for all its keys — so the transfer unit was pinned to 8 KiB
minus the header. Widening it there would have widened 24 unrelated interactive routes.

The chain now has its own route, in exactly the shape the sibling viewer surface already declared
(`GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS`) and the contributions route before it:

- `GENERATION3D_FLOW_EVAL_TOOL_IDS` = `flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`,
  `cancelPreviewEval`, `flowTessellateCancelResolve` — moved out of `GENERATION3D_RETAINED_TOOL_IDS`
  (24 gestures remain), with their five publication contracts moved with them.
- `Generation3dFlowEvalJobFactory` + `generation3d_flow_eval_contract()` +
  `GENERATION3D_FLOW_EVAL_RAW_BYTES = 65_536` + `Generation3dFlowEvalJobFactoryProofs`, registered
  and aggregated into `bounded_first_step_tool_proofs`.
- `GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES` raised to the same 64 KiB (it was already its own constant).
- `MESH_PACK_CHUNK_BASE64_CHARS`: **4 KiB → 48 KiB** — one guest-contiguous page minus the envelope
  header, since the producing guest must hold the whole envelope as one contiguous `String` and
  dlmalloc never returns it (`project-guest-contiguous-request-ceiling`).

The gesture quota `GENERATION3D_RETAINED_RAW_BYTES` is untouched at 8 KiB, asserted.

### 3.2 The round trip is bounded in time, not units (cause B)

`tessellate_step_envelope_json(handle, tolerance, budget, wall_micros, chunk)` now keeps stepping
while the job is `Working` **and** the wall deadline has not passed (`semio_framework_job::default_now_us`,
the framework's own guest-safe clock). `budget` stays the 24-unit *cancellation granularity*;
`TESSELLATE_STEP_WALL_MICROS = 6_000` is the interactivity bound, carried on the wire as `wallMicros`
so the app owns it (`PREVIEW_TESSELLATE_STEP_WALL_MICROS`) and the extension merely honours it.

### 3.3 Progress stopped lying

`PreviewTessellateStatus` gained `chunks_done` / `chunks_total`, and `ratio()` is now
`(units_done + chunks_done) / (units_total + chunks_total)`. A body still crossing can no longer
publish `ratio: 1`. (The `phase: "transferring"` half of this landed concurrently in the
`preview-eval-cancellation` lane's working tree; this change is the ratio it pairs with.)

### 3.4 Measured after

Same command, same machine:

| example | roundTrips | chunks | before |
|---|---|---|---|
| rectangle-wire-preview | 1 | 1 | 1 |
| rectangle-extrude-volume | 1 | 1 | 2 |
| face-sweep-extrude | 1 | 1 | 2 |
| hexagonal-mushroom-column | 1 | 1 | 2 |
| box-shell-preview | 1 | 1 | 3 |
| box-fillet-preview | **2** | **1** | 8 / 3 |
| sphere-box-fuse | **2** | **1** | 5 / 4 |
| sphere-cut-with-torus | **1** | **1** | 10 / 10 |

Every bundled example now delivers in **at most two** tessellate round trips and **exactly one**
mesh-body chunk. Triangle/edge/mesh counts are byte-for-byte unchanged.

---

## 4. Tests

### 4.1 Language-agnostic fixture rows

Each example's committed `🧪️tests/🧩️example/🔣️.json` gained a `delivery` block — the one source
both lanes read:

```json
"delivery": {
  "lodMode": "",
  "minMeshes": 1,
  "minTriangles": 92,
  "minEdgeSegments": 72,
  "maxRoundTrips": 2,
  "maxChunks": 1
}
```

The wire example states `minTriangles: 0, minEdgeSegments: 4` — a preview that paints on edges alone.

### 4.2 Rust law (`example-geometry`, region `🔖️MeshDelivery`)

Eight `delivery_*` laws. Each asserts: no validate-gate diagnostic, terminal phase `complete`,
delivered triangles/edge segments ≥ the row, `triangles > 0 || edgeSegments > 0`, published
`payloadMeshes ≥ minMeshes` with one instance per mesh, published triangles/edges ≥ the row, and the
two budgets `chunks ≤ maxChunks`, `roundTrips ≤ maxRoundTrips`. Every envelope is additionally held
to `tessellate_envelope_maximum_bytes()`.

### 4.3 TypeScript twin (`🧪️tests/🧩️geometry/🟦️.ts`)

`assertDeliveryContract`, called from `assertFixtureContract`, restates the same row independently:
`minMeshes ≥ 1`, `minTriangles + minEdgeSegments > 0`, `maxChunks === 1`,
`1 ≤ maxRoundTrips ≤ EXAMPLE_DELIVERY_ROUND_TRIP_CEILING (2)`, a `wire` preview must declare
`minTriangles === 0` and `minEdgeSegments ≥ 1`, and a solid's delivered triangles may never be fewer
than the geometry lane's own `expect.minTriangles`.

### 4.4 Route laws

- `tessellate_transfer_unit_fits_the_declared_response_wire_bound` (editor) now pins the transfer
  unit to `GENERATION3D_FLOW_EVAL_RAW_BYTES`, asserts it does **not** fit the gesture quota (a unit
  that fits needs no route of its own), and asserts the gesture quota is unchanged.
- `view_tessellate_envelope_fits_the_declared_wire_bound` (viewer, NEW) — the same law for the
  viewer's own chain route, so an LOD the editor paints is never one the viewer silently cannot.
- `retained_route_dispositions_are_exact_and_exhaustive` — three factories, 24 + 5 + 1 tool ids,
  30 aggregated proofs, and a tool id owned by exactly one factory.

### 4.5 Runs (foreground, quoted)

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly \
  --test example-geometry -- --test-threads=1
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 62.80s

cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly \
  --test io-round-trip -- --test-threads=1
test result: ok. 34 passed; 0 failed; …

cargo test -p semio-framework-os-flow --test flow_mesh_pack_wire
test result: ok. 7 passed; 0 failed; …

bun test ./<each example>/🧪️tests/🧩️example/🟦️.ts
32 pass, 0 fail, 309 expect() calls, 8 files

cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly -- --test-threads=1
test result: FAILED. 371 passed; 5 failed
```

Wasm reachability (native `cargo check` never compiles the guest):

```
cargo check -p semio-s-plugin-flow-extension-brep            --target wasm32-wasip2   → Finished
cargo check -p semio-s-artifact-procedural-generation3d \
            --features component-app-assembly                --target wasm32-wasip2   → Finished
```

**The five lib failures are peer-lane, pre-existing, and unrelated** — all five are recorded in this
ticket's own notes and all sit in framework files a peer is editing right now
(`🔌️plugin/🦀️.rs` mtime 07:17, `🧠️neural/⚙️engine/🦀️.rs` mtime 07:20):
`add_generation_records_an_undoable_generation_operation`,
`undo_redo_round_trips_flow_graph_edits`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`,
`two_instances_converge_disjoint_widget_moves` ("remote snapshot merge is fail-closed…", a documented
VCS stub) and `generation_preview_is_one_app_transient_shared_by_two_generation_windows`
(`📓️gap-inventory-2026-09-10.md` §Lane J). The same peer wave currently reds most of
`semio-framework-os-flow --lib` ("ordered-map root must be explicitly retired before drop") and
`semio-s-plugin-flow-extension-brep --lib` ("final Dictionary ownership must be explicitly retired…");
both signatures predate this lane and appear in earlier ticket notes.

---

## 5. Restage

Everything in §3 is **guest-side Rust** — `semio-framework-os-flow`'s `brep_geometry` and
`FlowEvalSession`, the `flow-extension-brep` handler and the `generation3d` app. The react dev serve
on `:6018` serves host TypeScript live, so a re-run there still exercises the OLD staged wasm; this
lane therefore stops at native proof, as instructed.

**A restage of `flow-extension-brep` and `semio-s-artifact-procedural-generation3d` is required** to
see §3.4 in the browser — and it is *also* the fix for cause **C** (Box Shell Preview), whose staged
shell operator is stale on its own. After the restage, re-run:

```
cd <ticket> && SEMIO_PROBE_OUT=mesh-delivery/run-F SEMIO_PROBE_BOOT_WAIT=200 \
  SEMIO_PROBE_PICK_WAIT=240 SEMIO_PROBE_QUIET=45 \
  SEMIO_PROBE_PICK="Sphere Cut With Torus,Box Fillet Preview,Sphere Box Fuse,Rectangle Wire Preview,Box Shell Preview" \
  bun 🐍️mesh-delivery-probe.mjs
```

and expect every pick to reach `meshCount ≥ 1` within two tessellate answers.

---

## 6. Files

Host (TypeScript) — temporary instrumentation only, added and removed:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

Changed:

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` | `MESH_PACK_CHUNK_BASE64_CHARS` 4 KiB → 48 KiB; new `TESSELLATE_STEP_WALL_MICROS`; `tessellate_step_envelope_json` gains `wall_micros` and loops to its deadline |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | `PreviewTessellateStatus::{chunks_done, chunks_total}`; `ratio()` counts the transfer stage |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` | `tessellate` handler reads `wallMicros`; `TESSELLATE_STEP_BUDGET` redocumented as the cancellation granularity |
| `…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` | `PREVIEW_TESSELLATE_STEP_WALL_MICROS`; the invocation carries `wallMicros` |
| `…/🧊️generation3d/…/✏️editor/🦀️.rs` | flow-eval route: tool-id split, payload schema, `GENERATION3D_FLOW_EVAL_RAW_BYTES`, contract, factory, publication contracts, proofs, registration, `build_tool_job` |
| `…/🧊️generation3d/…/👁️viewer/🦀️.rs` | `GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES` → 64 KiB |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | three route laws updated for the split |
| `…/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs` | extent law covers both gesture and chain routes |
| `…/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | NEW `view_tessellate_envelope_fits_the_declared_wire_bound` |
| `…/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs` | harness honours `wallMicros` |
| `…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` | NEW region `🔖️MeshDelivery` — `DeliveryExpectation`, `run_delivery`, `assert_delivery`, 8 laws |
| `…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts` | `ExampleDeliveryExpectation`, `EXAMPLE_DELIVERY_ROUND_TRIP_CEILING`, `assertDeliveryContract` |
| `…/📚️examples/<8 examples>/🧪️tests/🧩️example/🔣️.json` | NEW `delivery` row |
| `<ticket>/🐍️mesh-delivery-probe.mjs` | NEW runtime probe |
