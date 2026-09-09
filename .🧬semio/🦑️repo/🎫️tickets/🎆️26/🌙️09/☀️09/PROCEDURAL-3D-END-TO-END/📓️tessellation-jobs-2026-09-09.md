# Resumable Tessellation Jobs, Pack Mesh Transfer, and the Validate Gate (2026-09-09)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "tessellation jobs". Session ⚪9f5f6952 (Fable 5.1).
Repo MCP was down for the whole run (`invalid initialize params`); ticket bookkeeping is on disk and
no ticket was opened/closed/reopened.

This closes gaps **3**, **4** and **5** of `📓️kernel-and-preview-audit-2026-09-09.md` §6:

> 3. No cancellation for the actual tessellate/kernel call … one synchronous function call with no
>    progress callback and no cancellation token inside it.
> 4. Mesh crosses the host/extension boundary as an uncompacted JSON string … no chunking.
> 5. No automatic `validate()` gate before tessellation.

---

## 1. Design — the job lifecycle

### 1.1 The three layers

```
                     ┌──────────────────────────────────────── generation3d app (wasm guest) ────┐
                     │                                                                            │
  user edits a knob  │  flowEvalTick ──► (graph settles) ──► preview_tessellate_invocations       │
        │            │                                          │                                 │
        ▼            │                                          │ ExtensionInvocation{            │
  UpdateGenerationValues                                        │   extension_id:"brep",          │
                     │                                          │   capability:"tessellate",      │
                     │                                          │   request:{handle,tolerance,    │
                     │                                          │            nodeHash,budget=24,  │
                     │                                          │            chunk},              │
                     │                                          │   response_action:              │
                     │                                          │     "flowTessellateResolve"}    │
                     │                                          ▼                                 │
                     │                            SDK mints req, parks continuation                │
                     └────────────────────────────────────────────┬──────────────────────────────┘
                                                                  │  Effect::InvokeExtension
                     ┌────────────────────────────────────────────▼──────────────────────────────┐
                     │  flow-extension-brep  (ExecutionMode::Linked)                              │
                     │    tessellate_step_envelope_json(handle, tolerance, budget=24, chunk)      │
                     └────────────────────────────────────────────┬──────────────────────────────┘
                                                                  │
                     ┌────────────────────────────────────────────▼──────────────────────────────┐
                     │  semio-framework-os-flow :: brep_geometry                                  │
                     │                                                                            │
                     │  1. cached_mesh_at_or_finer(handle, tolerance) ──hit──► Ready              │
                     │  2. first admission only: Brep::validate_gate_sync ──Err──► Invalid        │
                     │  3. retained TessellationJob registry (32 slots, LRU by last step)         │
                     │        job.step(kernel.tessellation_body(), 24)                            │
                     │  4. Done ──► mesh cache[(handle, tolerance)] + pack body + base64 chunks   │
                     └────────────────────────────────────────────┬──────────────────────────────┘
                                                                  │
                     ┌────────────────────────────────────────────▼──────────────────────────────┐
                     │  semio-s-artifact-stdio-semio :: TessellationJob (the kernel)              │
                     │                                                                            │
                     │  SamplingEdges ──► MeshingFaces ──► PackingEdges ──► Complete              │
                     │   (1 unit/edge)     (1 unit/face)    (1 unit/edge)                          │
                     │        └──────────────── cancel() ────────────────► Cancelled (terminal)   │
                     └────────────────────────────────────────────────────────────────────────────┘
```

The response envelope goes back through `Event::Completed` → `flowTessellateResolve`:

```
Working  {done:false, cancellable:true,  phase, unitsDone/unitsTotal, facesDone/facesTotal}
Complete {done:true,  phase:"complete",  chunk, chunks, packBytes, meshPack:"<base64 chunk>"}
Invalid  {done:true,  phase:"invalid",   diagnostics:[{entity,code,message}]}
Failed   {done:true,  phase:"failed",    error, errorCode}
Cancelled{done:true,  phase:"cancelled"}
```

`FlowEvalSession::resolve_preview_tessellate` folds one envelope and returns a
`PreviewTessellateOutcome`. `Working` (a partial step, or a mesh body whose last chunk has not
arrived) re-arms `flowEvalTick`, which re-emits the invocation with the next `chunk` index. That is
the whole resumption mechanism — no timers, no parked futures (a plugin has nowhere to hold one, see
`📓️extension-round-trip-2026-09-09.md` §1a).

### 1.2 Why a face is the unit

The tessellator is edge-first and crack-free: every edge is discretized exactly once and shared by
its adjacent faces. The smallest granularity that does not force recomputing shared boundary samples
is therefore *one edge* (sampling and packing) or *one face* (triangulation). A pathological single
face still costs one whole unit, which is why the caller picks a **budget in units**, not a wall-clock
deadline — a deadline would have to abandon a face mid-triangulation and throw its work away.

`units_total = edges + faces + edges`, fixed at construction, so `progress()` is monotone and bounded
by construction rather than by convention.

### 1.3 Cancellation

Three distinct retirements, all landing on the same kernel job:

| trigger | path |
|---|---|
| explicit user gesture | `cancelPreviewEval` → `FlowEvalSession::cancel_preview_evaluation` → `brep_geometry::cancel_all_tessellations` → `TessellationJob::cancel` |
| a newer eval of the same node-hash | `retain_preview_meshes(live)` → `retain_tessellation_jobs(live)` drops jobs whose handle is dead |
| handle disposal | `dispose_geometry` drops every job for that handle |

`cancel()` is deliberately a **no-op on a completed job** (`cancelling_a_completed_job_keeps_its_mesh`):
supersession may only retire work that is still in flight, never destroy a result already paid for.

### 1.4 Determinism

The stepped path IS the algorithm. `tessellate_solid`/`tessellate_face`/`tessellate_wire` are now
thin `TessellationJob::run_to_completion` façades, so there is no second implementation to drift
(`stepping_produces_the_same_mesh_as_one_shot_tessellation`). Reaching that required making the edge
order deterministic: the old `tessellate_face_with_report` packed edges by iterating a `HashMap`,
which is not reproducible run to run. Both phases now walk one shared `distinct_face_edges` order.

---

## 2. Mesh transfer

### 2.1 The wire

`MeshData` used to cross as `os_pack::json::to_json_string(&mesh)` — one JSON string of number
arrays, one blob, no chunking. It now crosses as a `pack` **record body**
(`pack::encode_record_body`, container-less: symbol table + fields, no header/manifest/footer, no
chunk table), with every array as one little-endian `Shape::Bytes64` blob:

| field id | key | element |
|---|---|---|
| 1 / 2 / 4 / 5 | positions / normals / colors / uvs | `f32` LE |
| 3 / 6 / 7 | indices / faceIds / vertexIds | `u32` LE |
| 8 / 10 | edgePositions / edgeUvs | `f32` LE |
| 9 | edgeIds | `u32` LE |
| 11 | edgeIsSeam | `u8` |
| 12 | paintTextureBase64 | text |

Because `extension_response_args` lossily UTF-8-decodes the handler's bytes
(`⚛️reactor/🦀️.rs:730`), the body rides the JSON-typed boundary base64-encoded — the exact-JSON
projection rule: binary never enters a JSON number, it enters a JSON *string*. Chunked at
`MESH_PACK_CHUNK_BASE64_CHARS = 48 KiB` of base64 per continuation, so no single round trip can blow
the intake budget however dense the mesh.

The session stores the base64 body per handle (`preview_mesh_pack_by_handle`), and the render path
decodes it to typed arrays through `decode_preview_mesh_pack` — replacing a per-render JSON parse of
a number-per-element array.

### 2.2 LOD-keyed cache

The mesh cache key is `(handle, tolerance.to_bits())` as before, but lookup is now
`cached_mesh_at_or_finer`: a cached mesh at a tolerance **≤** the requested one satisfies the request
(the finest such entry wins). Switching the LOD mode fine → coarse therefore serves the mesh already
computed instead of retessellating; coarse → fine still tessellates, because a coarser mesh must
never silently answer a finer request. Both directions are asserted.

### 2.3 Measured (native, debug, aarch64)

Real tessellated unit sphere through `brep_geometry::tessellate_step`
(`flow_mesh_pack_wire::a_real_tessellated_sphere_halves_the_wire_at_every_lod`, `--nocapture`):

| LOD | deflection | triangles | JSON bytes | pack bytes | base64 bytes | chunks | pack vs JSON |
|---|---|---|---|---|---|---|---|
| coarse | 0.15 | 102 | 7 228 | 2 681 | 3 576 | 1 | **2.70×** smaller |
| default | 0.05 | 284 | 20 448 | 7 146 | 9 528 | 1 | **2.86×** smaller |
| fine | 0.02 | 564 | 40 838 | 14 010 | 18 680 | 1 | **2.91×** smaller |

Even counting the base64 inflation the wire is 2.0–2.2× smaller, and the *decode* side stops being a
JSON tokenizer. The golden triangle vector: json 200 B → pack 136 B → base64 184 B.

Per-LOD tessellation cost (`brep_tessellation_jobs::preview_lod_tolerances_are_ordered_and_bounded`,
`--nocapture`, debug build):

| solid | LOD | deflection | budgeted steps @24 | triangles | vertices | wall time | max chordal |
|---|---|---|---|---|---|---|---|
| box | coarse | 0.15 | 1 | 12 | 24 | 112.8 µs | 0.000000 |
| box | default | 0.05 | 1 | 12 | 24 | 109.3 µs | 0.000000 |
| box | fine | 0.02 | 1 | 12 | 24 | 105.3 µs | 0.000000 |
| sphere | coarse | 0.15 | 0 | 102 | 55 | 775.3 µs | 0.940199 |
| sphere | default | 0.05 | 0 | 284 | 148 | 2.053 ms | 0.604704 |
| sphere | fine | 0.02 | 0 | 564 | 291 | 5.326 ms | 0.386891 |

Reading: a box is 12 edges + 6 faces + 12 packs = 30 units, so a 24-unit budget genuinely splits it
across two calls; a sphere is 2 analytic faces + few seam edges, so it fits one budget but its single
face costs 5.3 ms in a *debug native* build — which is the point of the ceiling. The reactor faults an
instance whose one `maintenance_step` runs past 8 ms, and a wasm debug build is far slower than this;
without the budget a moderately dense solid is a guaranteed instance fault, not merely a stutter.

---

## 3. Validate gate

`Brep::validate_gate_sync(handle)` returns `Result<(), Vec<ValidationIssue>>` — the *typed* twin of
`validate_sync`, which returned a JSON string a caller would have to re-parse. It runs once, on first
admission of a `(handle, tolerance)` pair, and only for solid-like shapes (`Solid`/`Shell`/`Compound`);
a wire or curve preview passes through untouched. Advisory `warning-`-prefixed codes never block; every
other code does — matching `validate_body`'s own documented ERROR/advisory split.

A blocking result never reaches the tessellator. It becomes `phase:"invalid"` with the issues on the
wire, which the session stores under the handle (`preview_diagnostics_by_handle`) and the preview
window's status object republishes as `diagnostics:[{handle, issues:[{entity, code, message}]}]`.
The handle is not retried in a loop (`note_pending_tessellate` refuses a handle carrying a diagnostic).

This directly covers the audit's §6.1 risk: the two boolean examples sit on the known-failing exact
imprint path, and a `shell-not-closed`/`non-manifold-edge` result now surfaces as a typed diagnostic
instead of feeding broken topology to the tessellator.

---

## 4. Progress reaching the UI

`preview_progress_status_json(session)` is merged into the preview window's status object
(alongside the existing `computing` flag, `widgetErrors`, and the `evalLen`/`meshesLen`/`instancesLen`
debug block):

```json
{
  "phase": "meshingFaces",
  "phaseLabel": { "en": "Meshing faces", "de": "Flächen werden vernetzt" },
  "progress": { "unitsDone": 12, "unitsTotal": 30, "facesDone": 4, "facesTotal": 6,
                "inFlight": 1, "ratio": 0.4 },
  "cancellable": true,
  "cancelAction": "cancelPreviewEval",
  "diagnosticCount": 0
}
```

Every phase carries an English **and** German label with no default language
(`PreviewTessellatePhase::labels`). The nine phases share one wire tag vocabulary across the kernel
(`TessellationPhase::tag`), the extension envelope, the session and the UI.

`cancelPreviewEval` is a full app command: declared in `app_commands!`, in
`GENERATION3D_BOUNDED_TOOL_IDS`, published `HostOnly`, proofed by
`bounded_first_step_tool_proofs!`, classified `InteractiveJobClassification::Migrated`, and in the
command palette as *Cancel Preview Computation* / *Vorschauberechnung abbrechen*. (Per
`📓️…/project-interactive-job-classification-gates-dispatch`, anything short of `Migrated` would be
hard-dead at dispatch.)

**Owed:** a dedicated `WindowMeasure` button. `ArtifactEditor::window_measures` receives only
`(doc, cfg, view_state)` — it has no access to the instance's retained `FlowEvalSession`, so a
session-driven measure cannot be built there without a framework signature change. The status object
carries `cancellable` + `cancelAction` so the shell can render the affordance from the scene payload,
and the palette entry is reachable today; a measure-lane session hook is the clean follow-up.

---

## 5. Diff, file:line

### 5.1 `semio-s-artifact-stdio-semio` — the kernel

| file:line | change |
|---|---|
| `…🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:127` | `enum TessellationPhase` + `tag()` |
| `…/🧩tessellation/🦀️.rs:156` | `struct TessellationProgress { units_done, units_total, faces_done, faces_total, phase }` |
| `…/🧩tessellation/🦀️.rs:166` | `enum TessellationStep { Working, Done, Cancelled }` |
| `…/🧩tessellation/🦀️.rs:190` | `struct TessellationJob` — `for_solid`/`for_face`/`for_wire`, `progress`, `cancel`, `is_terminal`, `step(body, budget)`, `into_mesh`, `run_to_completion` |
| `…/🧩tessellation/🦀️.rs:392` | `distinct_face_edges` — the one deterministic edge order both phases walk (the old face path packed edges by `HashMap` iteration order, i.e. not reproducible) |
| `…/🧩tessellation/🦀️.rs:69-101` | `tessellate_solid[_with_report]`/`tessellate_wire`/`tessellate_face[_with_report]` rewritten as `run_to_completion` façades |
| `…/🧩tessellation/🦀️.rs` (removed) | `sample_solid_edge_cache`, `pack_edge_segments_with_info` — superseded by the job's phases |
| `…/⚙️engine/🦀️.rs:1506` | `Brep::tessellate_job_sync` |
| `…/⚙️engine/🦀️.rs:1518` | `Brep::tessellation_body` |
| `…/⚙️engine/🦀️.rs:1527` | `Brep::validate_gate_sync` |
| `…/📦️packages/🦀️rust/Cargo.toml` | `[[test]] name = "brep_tessellation_jobs"` |
| `…🧊️brep/🧪️tests/⏱️tessellation-jobs/🦀️.rs` (NEW, 10 tests) | budget/progress/determinism/cancel/LOD/validate-gate laws |

### 5.2 `semio-framework-os-flow` — the job registry, the wire, the session

| file:line | change |
|---|---|
| `📐️brep-geometry/🦀️.rs:559` | `retain_tessellation_jobs` |
| `📐️brep-geometry/🦀️.rs:571` | `cancel_tessellation(handle, tolerance)` |
| `📐️brep-geometry/🦀️.rs:583` | `cancel_all_tessellations()` |
| `📐️brep-geometry/🦀️.rs:594` | `tessellation_progress(handle, tolerance)` |
| `📐️brep-geometry/🦀️.rs:604` | `cached_mesh_at_or_finer` — the LOD-keyed lookup |
| `📐️brep-geometry/🦀️.rs:632` | `tessellate_step(handle, tolerance, budget)` + `TessellationStepOutcome` + `PreviewDiagnostic`; 32-slot LRU job registry |
| `📐️brep-geometry/🦀️.rs:714` | `tessellate_step_envelope_json` — the extension-boundary envelope |
| `📐️brep-geometry/🦀️.rs:812` | `mesh_pack_spec()` + the twelve `MESH_PACK_FIELD_*` ids |
| `📐️brep-geometry/🦀️.rs:871 / :897` | `encode_mesh_pack` / `decode_mesh_pack` |
| `📐️brep-geometry/🦀️.rs:930` | `chunk_mesh_base64` + `MESH_PACK_CHUNK_BASE64_CHARS` |
| `📐️brep-geometry/🦀️.rs` (tessellate_geometry) | now a `tessellate_step(usize::MAX)` loop — one algorithm |
| `🖥️host/🦀️.rs:2554 / :2559 / :2564` | `preview_mesh_pack`, `preview_diagnostics`, `preview_diagnostic_entries` |
| `🖥️host/🦀️.rs:2597` | `next_tessellate_chunk` |
| `🖥️host/🦀️.rs:2603` | `preview_tessellate_status` |
| `🖥️host/🦀️.rs:2625` | `cancel_preview_evaluation` |
| `🖥️host/🦀️.rs:2642` | `resolve_preview_tessellate` → `PreviewTessellateOutcome` (was `bool`), chunk reassembly, diagnostics |
| `🖥️host/🦀️.rs:2813` | `preview_mesh_pack_has_geometry` (decodes structurally; replaces the JSON-array probe) |
| `🖥️host/🦀️.rs:2826 / :2893 / :2905 / :2932` | `PreviewTessellatePhase` (+`tag`/`from_tag`/`is_cancellable`/`labels`), `PreviewTessellateProgress`, `PreviewTessellateStatus`, `PreviewTessellateOutcome` |
| `🖥️host/🦀️.rs` (state) | `preview_mesh_json_by_handle` → `preview_mesh_pack_by_handle`; new `tessellate_handle_by_hash`, `tessellate_progress_by_hash`, `tessellate_chunks_by_hash`, `preview_diagnostics_by_handle` — all five wired into `set_eval_json`, `close_step` and `terminal_is_empty` |
| `🖥️host/🦀️.rs` (`retire_tessellate_transfer`) | explicit retirement of one tessellation's handle binding + partial body |
| `🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs:40` | field rename follow-through |
| `📦️packages/🦀️rust/Cargo.toml` | `[[test]] name = "flow_mesh_pack_wire"` |
| `🧪️tests/🎒️mesh-pack-wire/🦀️.rs` (NEW, 7 tests) | round trip, cross-language vector, size law, chunking, LOD cache |

### 5.3 `semio-s-plugin-flow-extension-brep`

| file:line | change |
|---|---|
| `🦀️.rs:1944` | `const TESSELLATE_STEP_BUDGET: usize = 24` |
| `🦀️.rs:1956-1963` | `tessellate` handler now budgeted + chunked (`tessellate_step_envelope_json`) |
| `🦀️.rs:1965` | NEW `tessellateCancel` capability — per-handle or whole-registry retirement |

### 5.4 `semio-s-artifact-procedural-generation3d`

| file:line | change |
|---|---|
| `✏️editor/🦀️.rs:1698` | `PREVIEW_TESSELLATE_STEP_BUDGET = 24` |
| `✏️editor/🦀️.rs:1876` | `preview_progress_status_json` (phase, en/de labels, progress, cancellable, diagnostics) |
| `✏️editor/🦀️.rs` (`preview_scene_status_json`) | merges the progress object into the window status |
| `✏️editor/🦀️.rs:2124` | `decode_preview_mesh_pack` |
| `✏️editor/🦀️.rs` (`mesh_data_for_preview_handle`) | reads the pack body, skips a diagnosed handle |
| `✏️editor/🦀️.rs` (`pending_preview_tessellate_handles`) | pack-based readiness probe; a diagnosed handle is not re-requested |
| `✏️editor/🦀️.rs:2185` | invocation carries `budget` + `chunk` |
| `✏️editor/🦀️.rs:91/:276/:652/:1147/:1309/:1505/:1578` | `cancelPreviewEval` declared, listed, published `HostOnly`, proofed, parsed, catalogued, `Migrated` |
| `🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs` | folds the outcome; re-arms `flowEvalTick` while work remains |
| `🎮️commands/🛑️cancel-preview-eval/🦀️.rs` (NEW) | the cancel command |
| `🗿️artifacts/🧊️generation3d/🦀️.rs:690` | `pub mod cancel_preview_eval` |
| `🎮️commands/🔺️flow-tessellate-resolve/🧪️tests/🔬️unit/🦀️.rs` | rewritten for the pack envelope (7 tests, incl. the two liveness-sweep regressions of §7) |
| `🎮️commands/🛑️cancel-preview-eval/🧪️tests/🔬️unit/🦀️.rs` (NEW) | 2 tests |

### 5.5 TypeScript + fixture + launch

| file:line | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🟦️.ts:2273` | `type MeshPack` (typed arrays) |
| `…/🟦️.ts:2336` | `decodeMeshPackBody` — the TS mirror of the record-body wire |
| `…/🟦️.ts:2385` | `decodeMeshPackChunks` |
| `🧰️framework/🛍️products/💻️os/🧪️tests/🧊️mesh-pack-decode/🟦️.ts` (NEW, 5 tests) | decodes the SAME golden bytes Rust encodes |
| `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json` (NEW) | the cross-language golden vector |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | `🧪️test⏱️brep🧊️tessellation-jobs`, `🧪️test🎒️flow🧊️mesh-pack-wire` |

---

## 6. Evidence

Lane: private `CARGO_TARGET_DIR=$S/target-tess` (APFS clone of `target/debug`), `RUSTC_WRAPPER=""`,
`--keep-going`. The shared `target/` was never touched (the boot lane owns it).

### 6.1 Kernel — `brep_tessellation_jobs`, 10/10

```
running 10 tests
test a_budgeted_step_never_outruns_its_budget_and_progress_is_monotone ... ok
test a_zero_budget_step_is_a_pure_progress_probe ... ok
test cancel_retires_a_job_without_panicking_or_producing_a_mesh ... ok
test cancelling_a_completed_job_keeps_its_mesh ... ok
test phase_tags_are_stable_and_distinct ... ok
test preview_lod_tolerances_are_ordered_and_bounded ... ok
test stepping_produces_the_same_mesh_as_one_shot_tessellation ... ok
test the_validate_gate_admits_a_well_formed_solid ... ok
test the_validate_gate_passes_non_solid_previews_through ... ok
test the_validate_gate_rejects_an_unknown_handle_with_a_typed_code ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### 6.2 Wire — `flow_mesh_pack_wire`, 7/7

```
running 7 tests
test a_finer_cached_mesh_answers_a_coarser_lod_request ... ok
test a_real_tessellated_sphere_halves_the_wire_at_every_lod ... ok
test an_oversized_body_is_split_into_several_chunks ... ok
test chunking_a_mesh_body_is_lossless_and_bounded ... ok
test mesh_pack_is_smaller_than_the_json_it_replaces ... ok
test mesh_pack_matches_the_shared_cross_language_vector ... ok
test mesh_pack_round_trips_every_array_exactly ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.81s
```

### 6.3 generation3d commands — 9/9

```
running 9 tests
test editor::generation3d::commands::cancel_preview_eval::tests::cancel_retires_every_in_flight_tessellation_without_panicking ... ok
test editor::generation3d::commands::cancel_preview_eval::tests::cancelling_twice_is_idempotent ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::a_chunked_mesh_body_only_lands_on_its_last_chunk ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::an_invalid_solid_becomes_a_typed_diagnostic_not_a_mesh ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::a_partial_step_re_arms_the_tick_chain_with_progress ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::tessellate_result_resolves_the_pending_handle ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::the_liveness_sweep_drops_a_dead_handles_partial_body ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::the_liveness_sweep_preserves_a_half_transferred_mesh_body ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::unknown_node_hash_resolves_nothing ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 286 filtered out; finished in 0.01s
warning: `semio-s-artifact-procedural-generation3d` (lib test) generated 14 warnings
```

The trailing warning line is the type-check witness required by
`📓️…/feedback-require-warnings-as-proof-of-typecheck`: the crate reached codegen and linked, it did
not abort at module expansion.

### 6.4 TypeScript — the whole `@semio-tech/framework-os` suite, 348/348

```
 Test Files  5 passed (5)
      Tests  348 passed (348)
```

5 of those are the new `mesh pack decode` suite, decoding the same
`🧫️fixtures/🧊️mesh/mesh-pack-body-v1.json` bytes the Rust `flow_mesh_pack_wire` test encodes — the
language-agnostic half of the feature, with the fixture as the shared oracle.

### 6.5 Checks

| crate / target | result | warnings |
|---|---|---|
| `semio-s-plugin-flow-extension-brep` (native) | `Finished dev in 31.37s`, **0 errors** | 1, peer-owned (`semio-framework-os-flow` `SpaceMember` unused import, `🖥️host/🦀️.rs:24`) |
| `semio-framework-os-flow` (native) | `Finished dev in 17.69s`, **0 errors** | 1 (same peer import) |
| `semio-s-plugin-procedural` (native) | `Finished dev in 28.57s`, **0 errors** | 1 (same peer import) |
| `semio-s-plugin-procedural` `--target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | `Finished wasm-dev in 1m 14s`, **0 errors** | 1 (same peer import) |

All four re-run to green after the §7 fix; each log's own `grep -c '^error'` is 0. The wasm lane is
the one that matters for `#[cfg(target_arch = "wasm32")]` code — it compiled
`semio-s-artifact-procedural-generation2d`, `…-generation3d` and `semio-s-plugin-procedural` in
sequence with no diagnostics of their own. Raw logs: `🗑️generated/tessellation-check-*.txt`,
`🗑️generated/tessellation-test-*.txt`.

### 6.6 Peer churn during the run (attributed, not hand-waved)

The three native checks above were green at 20:26. At 20:28 a peer's uncommitted edit to
`✏️s/🔌️plugins/🗄️stdio/…/🧊️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs` (the blend/fillet lane, whose
region this lane was told to stay out of) broke `semio-s-artifact-stdio-semio`:

* `E0603` — `diff::offset::exact_pcurve` is private (`🎨️blend/🦀️.rs:32`)
* `E0282` — type annotations needed (`🎨️blend/🦀️.rs:676`)

and shortly after a second peer broke `semio-framework-plugin` with six
`E0599: no method named live_render_operation`; later, 42 errors appeared in
`semio-s-artifact-procedural-generation3d`'s lib test target (an unqualified-`testkit::` sweep plus a
`Widget::InputSlider { value }` type change), **none** of them in this lane's two command-test
directories. Neither file is touched by this lane's diff (both `git log` provenance and mtime confirm
the blend file was modified at 20:28:53 today, uncommitted, after every check in §6.5 finished). All
gates were re-run to green at 20:5x once the tree recovered; the peer churn is recorded here only so
a future reader does not mistake it for this diff's breakage.

There was also a hard environment constraint worth recording: `/System/Volumes/Data` sat at 96–99%
capacity for the whole run (six concurrent private `CARGO_TARGET_DIR`s of 40–103 GB each), and two
builds died with `No space left on device` mid-`rustc`. `target-tess/debug/incremental` alone was
81 GB; deleting it (cargo regenerates it) plus `CARGO_INCREMENTAL=0` for the remaining runs was what
made the gates finishable. A `rustc-LLVM ERROR: IO failure on output stream` here is a full disk, not
a compiler bug.

Two further pre-existing, peer-owned link failures shaped where the tests live:

* `semio-s-artifact-stdio-semio`'s **lib test** target does not compile (`📦️object` subset:
  `encode_pack().expect()` on a `Vec<u8>`, `FaultCode: !Display` ×2).
* `semio-framework-os-flow`'s **lib test** target does not compile (`flow_fixture_operations`,
  `FlowEnvelope`, `DagCamera`, `IoPortSpec` visibility).

Both are why this lane's kernel and wire laws are declared `[[test]]` integration targets rather than
`#[cfg(test)]` modules — they link against each crate's PUBLIC surface only, so they run green today
and stay runnable independently of whoever owns those test targets. That is a better home for a
cross-implementation contract anyway.

---

## 7. A real bug this lane found and fixed in itself

Worth recording because it is the kind of defect only the *combination* of chunking and the per-tick
liveness sweep produces, and no single-unit test would have caught it.

`preview_tessellate_invocations` calls `retain_preview_meshes(live)` on every `flowEvalTick`, before
re-arming. The first draft pruned the progress and chunk tables by "hashes present in
`pending_tessellate_by_hash`" — but a *partial* answer removes its own pending entry (that is what
lets the continuation re-admit it). So the very first sweep after chunk 0 arrived deleted the
half-received body and the chunk cursor, and the next round trip asked for chunk 0 again: **a
multi-chunk mesh could never land, forever.** Small meshes (every bundled example today) fit one
chunk and were unaffected, which is exactly why it would have shipped.

Fix: a dedicated `tessellate_handle_by_hash` binding (`🖥️host/🦀️.rs`, alongside the pending table),
so liveness is judged by HANDLE for the whole life of a tessellation, and the transfer state is
retired explicitly (`retire_tessellate_transfer`) on every terminal outcome. Locked in by
`the_liveness_sweep_preserves_a_half_transferred_mesh_body` and
`the_liveness_sweep_drops_a_dead_handles_partial_body`.

---

## 8. What remains owed

1. **Descriptor regeneration** for the procedural plugin — `cancelPreviewEval` is a new action, so
   `✏️s/🔌️plugins/🌀️procedural/🛂️.descriptor.semio` / `🔣️.json` are stale until
   `os-plugin-describe-rs:describe -- procedural` runs. Same blocker as
   `📓️extension-round-trip-2026-09-09.md` §5.1 (its `dependsOn: ["build"]` needs the shared `target/`).
3. **Runtime confirmation.** Every hop is unit-tested; no console log from a live `dev` boot has yet
   shown a budgeted `tessellate` step painting progress and a `cancelPreviewEval` retiring it. That is
   the boot lane's gate and it needs (1) and (2).
4. **A `WindowMeasure` cancel button** — needs a session hook on `ArtifactEditor::window_measures`
   (§4).
5. **The renderer's own ingestion.** `World3dScene.meshes_json` still carries JSON to the viewer;
   `decodeMeshPackBody` is in place on the TS side so the World3dHost/wgpu upload path can switch to
   typed arrays without a further wire change, but that switch is not part of this diff.
6. **The stale `OPERATION_QUALITY` rows** (`sphere_prim`/`cylinder_prim`/`cone_prim`/`torus_prim`
   still tagged `MeshDerivedBRep`) — audit §6.6, untouched here, one-line fix owned by the kernel lane.
