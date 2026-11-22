# 🔎️ WGPU test-crate drift audit (2026-11-22)

Read-only audit for handoff lanes 3 and 5 (`📓️handoff-2026-11-22.md`). No repository file was edited, no git command and no cargo or repo build was run. Working files for the scan sit in the session scratchpad, not in the ticket.

Base path for crates: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/`.

## Method and evidence limits

- Compiler logs: no `error[E` text exists for these crates under `🗑️generated/` or the ticket. `a2b.txt`, `a2c.txt` and `a2d.txt` hold only `[script]` and `[capacity]` progress lines. The 29-error list for `engine-attach-tests-2` is therefore reconstructed from `📓️activation-build-blockers-2026-11-22.md` and from source inspection, not reproduced.
- Scan 1, imports: every `use` path in each crate's lib and in every file pulled in by `#[path]` mounts or `include!` was resolved against the definitions in `🧰️framework` (489 distinct leaf names). Only 17 leaves lacked a production definition and all 17 are fixtures or enum variants that exist in test or fixture crates, so they are not drift.
- Scan 2, free-function arity: call sites of imported free functions were compared with the definition's parameter count. No real mismatch remained (the hits were false positives from string literals containing commas).
- Scan 3, callees: every called name in the test files was checked against the set of all `fn` names in `🧰️framework` (66k names). Only non-std names missing from it were local closures or macros.
- Scan 4, Ctx methods: `XCtx::of(..).method(..)` calls in the shell crates were compared with the method's parameter count. 67 calls, 0 mismatches.
- Dependency scan: path-qualified crate references (`crate::`-style) were compared with each crate's `[dependencies]` and `[dev-dependencies]`.
- Not verified without a compile: type-level mismatches (for example a return type changed inside a body) and borrow errors.

## 🧪 engine-attach-tests-2 (crate `semio-renderer-wgpu-engine-attach-tests-2`)

Lib: `🪜️engine-attach-tests-2/🦀️.rs`. Mounted: `⚙️EngineCanvas/🧪️tests/{🕸️wgpu-node-graph,🖌️wgpu-paint2d-engine,🫳️node-graph-gestures,🩺️metrics-invalidation-drain,🔬️wgpu-catalogue-workflow-drop}`. Included: `⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` (line 61 of lib, `include!`, missed by the first scan).

| test file:line | symbol used | current production signature / location | edit needed |
|---|---|---|---|
| `🪜️engine-attach-tests-2/🦀️.rs:23` | `paint2d_pointer_cancel_into` used at `🖌️wgpu-paint2d-engine/🧲️marquee-and-navigator-laws/🦀️.rs:26` but not imported | `fn paint2d_pointer_cancel_into(surface_id: &str) -> bool` at `semio_renderer_wgpu_engine_board::paint2d` (file `⚙️EngineCanvas/.../🖌️paint2d/🦀️.rs:617`) | add `paint2d_pointer_cancel_into` to the `paint2d::{...}` import on line 23 (E0425) |
| `🫳️node-graph-gestures/🦀️.rs:42` | return type `flow::dag::DagScreenHit` | `DagScreenHit` is defined in `🎲️board/🧬️schema/🪟️dag-views-traits-and-facts/🦀️.rs:111`. `flow::dag` is `infinite::board::ports::directed_dag`, which re-exports `DagHost` and `hovered_node_id` but not `DagScreenHit` | import `DagScreenHit` from the schema crate (see Cargo rows) and drop the `flow::dag::` prefix (E0433) |
| `🫳️node-graph-gestures/🦀️.rs:46` | `host.dag.screen_hit((x - bounds.x) as f64, (y - bounds.y) as f64)` | no method on `DagHost`. Free fn `pub fn screen_hit(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState, sx: f64, sy: f64) -> DagScreenHit`, re-exported by `semio-os-infinite-board-dag-host` (`🧭️host/🦀️.rs:61`) | `semio_os_infinite_board_dag_host::screen_hit(&host.dag.viewport, &host.dag.graph, &host.dag.minimap, sx, sy)` (E0599) |
| `🫳️node-graph-gestures/🦀️.rs:266` | `host.dag.host_snapshot.edges` | `DagHost` has fields viewport, graph, pointer, paint, minimap, notes. `host_snapshot` now lives on `DagGraphState` (`🕸️graph/🧬️state-records-and-screen-conversion/🦀️.rs:25`) | `host.dag.graph.host_snapshot.edges` (E0609) |
| `🫳️node-graph-gestures/🦀️.rs:290` | `host.dag.host_snapshot.camera` | as above | `host.dag.graph.host_snapshot.camera` (E0609) |
| `🕸️wgpu-node-graph/🦀️.rs:124` | `host.dag.host_snapshot.nodes/.edges` (Dag arm) | as above | `host.dag.graph.host_snapshot...` (E0609) |
| `🕸️wgpu-node-graph/🦀️.rs:125` | same, Dag arm | as above | same (E0609) |
| `🕸️wgpu-node-graph/🦀️.rs:340` | `host.dag.host_snapshot.camera` | as above | same (E0609) |
| `🕸️wgpu-node-graph/🦀️.rs:460` and `:491` | `semio_os_infinite_board_dag_graph::hovered_node_id(&host.dag.viewport, &host.dag.graph)` | crate not a dependency of this test crate. The same fn is re-exported as `flow::dag::hovered_node_id` (`directed_dag`, same signature) | replace with `flow::dag::hovered_node_id(...)`, so no Cargo row is needed (E0433) |
| `🕸️wgpu-node-graph/🦀️.rs:527` | `NodeGraphHover { node_id, port_id, edge_id, hover_source_window_id }` | struct now has `pub port_ids: Vec<String>` (`🖱️node-graph/🦀️.rs:98`), no serde default for struct literals | add `port_ids: Vec::new()` (E0063, this is the `port_ids` error in the note) |
| `🧊️wgpu-standalone/🦀️.rs:148` | `ui_gpu::RasterTextureWitness { scene_revision, preview_generation, operation }` | struct at `🖍️draw/🖼️raster/🔑️texture-keys-ledger-and-registry/🦀️.rs:24`, re-exported by `ui_gpu` (`semio-ui-wgpu-render`). This crate has no `ui_gpu` dependency | add the Cargo alias (see Cargo rows) (E0433) |
| `🧊️wgpu-standalone/🦀️.rs:351` and `:359` | `host.dag.host_snapshot.camera.*` (Dag arm) | as above | `host.dag.graph.host_snapshot.camera.*` (E0609) |
| `🧊️wgpu-standalone/🦀️.rs:365` and `:371` | `host.selected_node_ids_json()` on `GraphHost` | `GraphHost` has `pub fn selected_node_ids(&self) -> Vec<String>` (`🕸️NodeGraph/🦀️.rs:384`). The `_json` variant exists only on the wasm_bindgen wrapper (`🕸️NodeGraph/🦀️.rs:807`, JsValue) | `host.selected_node_ids()` (comparison still works on `Vec<String>`) (E0599) |

Cargo (`📦️packages/🦀️rust/Cargo.toml`), needed edits:

1. Add `ui_gpu = { path = "../../../🖍️draw/📦️packages/🦀️rust", package = "semio-ui-wgpu-render" }`. The `../../../` depth matches the sibling row `semio-renderer-wgpu-engine-attach` (`../../../🔗️engine-attach/...`) in this same file.
2. Add `semio-os-infinite-board-dag-host` for `screen_hit`, with path `../../../../../../../♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/📦️packages/🦀️rust` (the depth matches the `infinite_canvas` row, line 47, which is also 7 levels up).
3. Add `semio-os-infinite-board-schema` for `DagScreenHit`, under `♾️infinite/🎲️board/🧬️schema/`. Its package directory is not confirmed in this audit, so check its `Cargo.toml` location before adding the path.

Count: 14 source edits (1 import, 6 `dag.host_snapshot`, 1 `screen_hit`, 1 `DagScreenHit` path, 2 `hovered_node_id`, 2 `selected_node_ids_json`, 1 `port_ids` literal) plus 3 Cargo rows. Remaining sites (`wgpu-node-graph:306` and `:539`, `:287`) already use the correct paths and need no change. The 29-error count is larger than these direct sites, so the rest are likely cascades from the same API drift. They cannot be confirmed without a compile.

## 🍰 shell-tests-input-3 (`semio-renderer-wgpu-shell-tests-input-3`)

Lib `🍰️shell-tests-input-3/🦀️.rs`, 10 files including mounts (`🧱️elements/🐚️Shell/🧪️tests/...`).

- All 116 distinct `use` paths resolve. Fixtures (`semio_renderer_wgpu_fixtures::shell::{host_test_shell,split_pane_shell,tour_shell,...}`) exist in `🧰️fixtures/🐚️shell/🦀️.rs`.
- `operation_publications_from_json` (listed in the activation note as missing) now resolves through `semio_renderer_wgpu_program_bridge` (`pub use js_calls_and_operation_publications::operation_publications_from_json`, ProgramBridge lib line 73).
- `take_media_actions`, `push_media_action`, `pinned_app_for`, `relay_open_plan` (listed in the note) are not referenced by any file of this crate. Their errors come from production staging code, not from this test crate.
- All `XCtx::of(&mut shell)` constructors exist (bundle pattern, `of(shell: &'a mut ShellState)`), and all 67 Ctx method calls match their arities.
- `FramePumpsCtx` and `ChromeOverlaysCtx` (the split crates of today) resolve and are dependencies.

Count: 0 confirmed edits. Re-run the compile after the peers finish the staging split before editing.

## 🥧 shell-tests-docs-1 (`semio-renderer-wgpu-shell-tests-docs-1`)

Lib `🥧️shell-tests-docs-1/🦀️.rs`, mount `🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel` (recursive, 10 files).

- All `use` paths resolve (169 distinct). `ReplayRefusalReasonV1` variants used here exist.
- `extern crate semio_os_kernel as store_sync` alias is valid (dev-dependency present).
- `TimeTravelVerb`, `band_label_in_catalog`, `history_refusal_of_fault`, `time_travel_band_plan` and the other `shell_prefs` names resolve in `📥️staging/🎛️prefs`.

Count: 0 confirmed edits.

## 🥪 shell-tests-docs-2 (`semio-renderer-wgpu-shell-tests-docs-2`)

Lib `🥪️shell-tests-docs-2/🦀️.rs`, six mounts (`wgpu-identity-directory-presence`, `hub-projection-workspace`, `wgpu-local-folders`, `wgpu-document-relay`, `wgpu-shell-boot-isolation`, `wgpu-directory-door`).

- All `use` paths resolve (187 distinct), including `semio_os_kernel::os_directory::*` and `semio_renderer_wgpu_doors::directory_door::*`.
- `extern crate semio_os_kernel as protocol` and `store_sync` aliases are valid.
- `ReplayRefusalReasonV1::{InvalidRequest, LocalCatalogUnavailable, SignInRequired, SpaceIndexRequired, UnroutedCommand}` exist (`📥️staging/🎛️prefs/🔖️identity-and-types/🦀️.rs:49`).

Count: 0 confirmed edits.

## 🧪 world_host_unit_3 (`semio-os-infinite-world-host`, `[[test]] world_host_unit_3`)

Target: `🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🧪️unit-3/🦀️.rs` (689 lines). Declared in `📦️packages/🦀️rust/Cargo.toml:111` with `required-features = ["testkit"]`.

The GPU-section test is `on_a_real_gpu_a_section_removes_the_far_half_and_the_cap_closes_the_cut_without_leaking_stencil` (line 572, in region `ModellingWgpu` 172–614). It calls `render_on_gpu` (lines 476–547) and `world_gpu_fixture_stock()` (line 549).

Symbol check, all resolve in the current tree:

| used at | symbol | current location / signature |
|---|---|---|
| `:477`, `:479` | `ui_gpu::{RasterTextureCleanupStep, RasterTextureWitness, GpuContext}` | Cargo alias `ui_gpu` = `semio-ui-wgpu-render` (`Cargo.toml:46`); re-exports at `🖍️draw/🦀️.rs:11-12` |
| `:478` | `semio_ui_prepared::{PreparedRasterKeepCursorV1, PreparedRasterKeepStepV1, PreparedRenderGate, PreparedRenderInput, PreparedRenderJob, UiPresentToken}` | all defined in `🎟️prepared` (`semio-ui-prepared`) |
| `:486` | `prepared_world_build_context()` | `🧰️testkit/🦀️.rs:67` |
| `:490` | `append_step(&mut input) -> Result<bool, World3dBuildRejected>` | `🗃️model/🎒️prepared-resources/🦀️.rs:317` |
| `:492` | `retire_world_build_context(resources)` | `🧰️testkit/🦀️.rs:79` |
| `:501` | `PreparedRenderInput::try_new(u64,u64,DrawList,Option<DrawList>,f32) -> Result<Self, PreparedRenderInputRejected>` | `📮️render-input-and-mailbox/🦀️.rs:148` (`.fault()` exists at :32) |
| `:504` | `PreparedRenderJob::try_new(input)` | `🔧️render-job-state-and-close/🦀️.rs:201` |
| `:505` | `semio_ui_prepared::testing::drive_preparation_until_terminal(&mut job) -> Option<JobOutcomeKind>` | `🧪️testing/🦀️.rs:232` |
| `:506`, `:263` | `job.take_packet()`, `job.fault()` | `🔧️render-job-state-and-close/🦀️.rs:263`, `:196` |
| `:509` | `OriginalJobFixtureCaller::admit(OperationId, Generation, RetainedCloneGrant, fn()->Option<u64>, u64, u64)` | `🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs:10`, root re-export at `🧵️job/🦀️.rs:9`. `default_now_us` re-exported via `⏱️context/🦀️.rs:39` |
| `:510` | `close_job_step(&mut job, &mut caller)` | `🧪️testing/🦀️.rs:34` |
| `:511` | `caller.terminal_is_empty()`, `caller.retire_step(RetainedCloneGrant)` | `🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs:123`, `:137` |
| `:512` | `packet.expect(...)`, `close_packet_step(&mut packet)` | `🧪️testing/🦀️.rs:5` |
| `:513` | `GpuContext::headless(1,1)` (async, via block_on), `source.offscreen(w,h)` | `🧠️gpu-context-and-meshes/🦀️.rs:84`, `:87`. The error prefix `"offscreen adapter:"` the test matches is still produced by `platform/🦀️.rs:20` |
| `:514` | `begin_prepared(&UiPresentToken, &PreparedRenderGate, &packet, rev, gen) -> Result<(), String>` | `🧠️gpu-context-and-meshes/🦀️.rs:220` |
| `:515` | `RasterTextureWitness { scene_revision, preview_generation, operation }` | `🔑️texture-keys-ledger-and-registry/🦀️.rs:24` |
| `:516`, `:521-523` | `begin_raster_ownership`, `publish_raster_ownership(w, &str)`, `seal_raster_ownership`, `packet.raster_keep_step(&mut PreparedRasterKeepCursorV1)` | `draw-scalar-encoding-and-raster-delegates/🦀️.rs:267/271/275`; `render-commands-and-packet/🦀️.rs:373` |
| `:529` | `apply_prepared_upload_step(&packet, index, w, w) -> Result<bool,String>`, `packet.uploads().len()` | `prepared-upload-and-present-steps/🦀️.rs:7`; `uploads()` at `render-commands-and-packet/🦀️.rs:349` |
| `:531-533` | `begin_prepared_present`, `prepared_present_step`, `take_missing_world_mesh` | `prepared-upload-and-present-steps/🦀️.rs:91/113`; `gpu-context-and-meshes/🦀️.rs:187` |
| `:535-536`, `:111` | `begin_prepared_readback`, `prepared_readback_step`, `readback.take_rgba()` | `gpu-context-and-meshes/🦀️.rs:102/113`; `📤️readback/🦀️.rs:111` |
| `:537-538` | `cursor.begin_close()`, `cursor.close_step()` | `present-cursor-and-helpers/🦀️.rs:179/188` |
| `:540-544` | `abort_presented_rasters_step`, `close_raster_upload_step` (`RasterTextureCleanupStep::Complete`), `close_mesh_upload_step`, `close_raster_table_step`, `close_mesh_table_step` | `draw-scalar-encoding-and-raster-delegates/🦀️.rs:239/243/247`; `gpu-context-and-meshes/🦀️.rs:177/212` |
| `:549-551` | `semio_framework_value::RetainedCloneGrant { maximum_items, maximum_copy_bytes, maximum_capacity_bytes, maximum_release_bytes, maximum_depth }` | `🔨️modules/🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs:7` (fields match) |

Result: no symbol, field or arity drift in the GPU-section path. The API the test was written against is still the API that exists, and the test compiles on paper. Count of compile-level edits: 0.

Likely failure source, not confirmed (the test has not been run):

1. Job path. `🧵️job` files changed between 13:59 and 14:47 today (`⏱️context/🦀️.rs`, `⏱️context/🛬️owned/🦀️.rs`, `⏱️context/🎟️source/...`, `🧪️testing/🎟️caller/🦀️.rs`, `📬️outcome/⏸️continuation/...`). The test drives `OriginalJobFixtureCaller` to terminal with a loop (`while !caller.terminal_is_empty() { caller.retire_step(...).expect(...) }`). A change in what counts as terminal, or in the grant accounting (`world_gpu_fixture_stock`, `maximum_items: 1 << 20`, `maximum_copy_bytes: 1 << 30`), would show as a hang or as the `"original fixture caller retirement"` panic.
2. Prepared gate. `PreparedRenderGate::validate` (`🛡️presentation-gate/🦀️.rs:75`) rejects while `closing` or while a presentation is `pending`. The test relies on a fresh `PreparedRenderGate::default()`, so this is the second place to check if `begin_prepared(...).expect("packet admitted")` panics.
3. Platform. `prepare_headless` (`📥️platform/🦀️.rs:33`) was rewritten in the platform refactor (mtime 05:01 today). A headless adapter failure that does not start with `"offscreen adapter:"` would panic at `render_on_gpu` line 484 instead of skipping.

Diagnosis step, for whoever runs the test: read the first panic message. `prepared job ended in`, `packet admitted`, `original fixture caller retirement` each point to one of the three causes above.

## 🧾 Summary of edits

| crate | source edits | Cargo rows | status |
|---|---|---|---|
| `semio-renderer-wgpu-engine-attach-tests-2` | 14 | 3 (ui_gpu alias, dag-host, board-schema) | confirmed by source inspection |
| `semio-renderer-wgpu-shell-tests-input-3` | 0 | 0 | no drift found |
| `semio-renderer-wgpu-shell-tests-docs-1` | 0 | 0 | no drift found |
| `semio-renderer-wgpu-shell-tests-docs-2` | 0 | 0 | no drift found |
| `world_host_unit_3` (GPU section) | 0 compile edits | 0 | runtime cause unknown, see above |

`fit_graph_action_persists` (lane 3) lives in `semio-renderer-wgpu-engine-attach-tests-2`, so the 14 source edits above must land before it can be run. The shell crates' dependency on today's split crates is already in place.
