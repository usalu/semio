# engine-attach-tests-2 port to the production API (2026-11-22)

## Status
Edits applied and verified by source inspection; NOT yet compiled green. Three `repo cargo check --tests -p semio-renderer-wgpu-engine-attach-tests-2` runs (store shared, prototype, parallelism 2) failed before reaching the crate: (1) a peer-edited `semio-os-store-operation-wire` (`replacement/🦀️.rs:131` E0061/E0282) plus the shared snapshot moving, (2) and (3) `wgpu 29.0.4` fails with `couldn't read .../wgpu-29.0.4/src/util/blit.wgsl` although the file exists on disk (registry src race / served-snapshot omission of `.wgsl`; run 3 took 16 min). `fit_graph_action_persists_the_fitted_camera_like_a_wheel_zoom` has therefore NOT run. No test numbers exist.

## Edits (verified against production source)
Test files (`🧱️elements/⚙️EngineCanvas/🧪️tests/`):
- `🕸️wgpu-node-graph/🦀️.rs`: `host.dag.host_snapshot` -> `host.dag.graph.host_snapshot` (lines 124, 125, 340); `semio_os_infinite_board_dag_graph::hovered_node_id` -> `flow::dag::hovered_node_id` (460, 491; no new dep); `NodeGraphHover` literal gains `port_ids: Vec::new()` (527).
- `🧊️wgpu-standalone/🦀️.rs`: `host.dag.graph.host_snapshot.camera` (351, 359); `selected_node_ids_json()` -> `selected_node_ids()` (365, 371). `ui_gpu::RasterTextureWitness` (148) resolved by the Cargo alias.
- `🫳️node-graph-gestures/🦀️.rs`: `host.dag.graph.host_snapshot` (266, 290); return type `infinite_canvas::board::schema::dag_input::DagScreenHit` (re-exported; the audit's schema-crate Cargo row is unnecessary); `host.dag.screen_hit(..)` -> `semio_os_infinite_board_dag_host::screen_hit(&host.dag.viewport, &host.dag.graph, &host.dag.minimap, sx, sy)`.
- `🪜️engine-attach-tests-2/🦀️.rs`: `paint2d_pointer_cancel_into` added to the `paint2d::{..}` import.
Cargo (`🪜️engine-attach-tests-2/📦️packages/🦀️rust/Cargo.toml`): added `semio-os-infinite-board-dag-host` and `ui_gpu` (alias of `semio-ui-wgpu-render`); paths verified to exist. 2 Cargo rows, not 3.

## Marquee call

`advance_paint2d_pixel_selection_into` production is `(scene, window_id: &str, input)`. The cancel-path call in `marquee-and-navigator-laws` now passes `"selection-fixture-window"`, the same id as the two calls above it. The other engine-attach-tests-2 mounts (`wgpu-node-graph`, `wgpu-standalone`, `node-graph-gestures`, `metrics-invalidation-drain`, `wgpu-catalogue-workflow-drop`) were scanned for the previously audited field/import drift (`host.dag.graph.host_snapshot`, `port_ids`, `RasterTextureWitness`, `screen_hit`); those edits are already in the files. Not compiled this pass (`repo cargo` withheld).

## Trace `shell node-graph fit`

`shell_node_graph_fit_trace` formats `[DEBUG] shell node-graph fit {"x":N,"y":N,"zoom":N}`. `framework_fit_node_graph` logs it when `node_graph_fit_action` returns a viewport and when it returns `None`. The `None` arm reads `node_graph_current_camera` (no second fit). `port-fit-probe.mjs` `JSON.parse`s from the first `{` and requires numeric `x`, `y`, `zoom`. The gesture probe and `wgpu-battery.mjs` only count the substring `shell node-graph fit`. A missing camera logs `0, 0, 1` so the line still contains an object. `semio-renderer-wgpu-shell-actions` was already about 15KB and 224 active lines under the 262144 / 32 / 4000 ceiling; this helper is a few lines. Not compiled.
