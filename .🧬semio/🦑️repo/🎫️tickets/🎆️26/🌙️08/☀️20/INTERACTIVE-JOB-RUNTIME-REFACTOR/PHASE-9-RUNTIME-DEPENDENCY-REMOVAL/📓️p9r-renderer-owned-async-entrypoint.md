# Renderer Owned Async Entrypoint

## Scope

Phase 3/5/9 closure for `semio-framework-os-renderer-wgpu`'s final direct executor dependency. The source census follows the WGPU glue's include tree, so the owned native ProgramBridge and Shell test-entrypoint sites are included. Stdio, FEM, generated/typegen, UI-WGPU implementation modules, Flow, Puzzle, and OS host source are excluded.

## Exact census before

- Direct manifest edges: `pollster = "0.4.0"`; dev-only `wasm-bindgen-test = "0.3.56"`; no direct `naga` edge existed.
- Exact expanded call census: 21 `pollster::block_on` calls: ten native scale-bench calls, one native smoke wrapper, one production `ProgramBridge::read_history` bridge, and nine test-only Shell calls.
- Interactive calls: the one ProgramBridge production bridge was UI-reachable through Shell history refresh. The kernel request state machine was already mounted through `KernelPoolFuture::spawn(renderer_worker_pool(), Lane::Interactive, ...)`; app continuations use `spawn_app_task`/`poll_tasks`; prepared frames use the prepared packet/gate seam.
- Test-only thread census: `render_snapshot.rs` contains a scoped test thread; it is not a runtime executor bridge and is unchanged.

## Implementation

- Converted the complete scale-bench `Env` graph (`new`, activation, submit, pump, tracked pump, unregister) and budgets 2–7 to genuine async functions.
- Converted the scale process `run` and headless smoke `run_smoke` to async.
- Converted `ProgramBridge::read_history` and Shell's history snapshot chain to genuine async calls; no production executor bridge remains in the expanded renderer include tree.
- Replaced the nine test-only calls with the owned `semio_framework_async` test-entrypoint driver. A later media-dialog async parity test uses that same sanctioned driver.
- Added one private `drive_entrypoint` in the native binary, backed by `semio_framework_async::block_on`; only `--scale` and `--smoke` call it. The interactive `run_native` event loop remains direct.
- Enabled the existing `semio-framework-async` dependency's `entrypoint` feature in the native target section; no dependency was added.
- Removed direct `pollster` and unused `wasm-bindgen-test` rows. `naga` remains transitively owned by WGPU only, preserving shader validation without a WGPU-target dev edge.
- Added focused static boundary tests asserting zero executor bridge in library product logic, exactly one binary driver with exactly two process call sites, zero retired direct manifest edges, and presence of worker/app-task seams. The interactivity allowlist records that binary line as the permanent native process entrypoint.

## Interactivity invariants

- No UI callback waits for GPU/plugin work; product library bridge census is zero.
- Kernel work remains on the injected renderer worker pool and the interactive lane.
- Prepared-frame freshness and generation gates are untouched.
- Cancellation and stale-result behavior are untouched; production history refresh now yields normally through the already-mounted app task.
- Scale-bench blocking shard-outcome waits remain confined to the one-shot `--scale` process path and never enter the interactive renderer.

## Evidence

- `📝️p9r-wgpu-static-ratchet.txt`: library bridge census zero; binary bridge census exactly one; retired direct manifest edges zero; worker/app-task seams present.
- `📝️p9r-wgpu-direct-dependency-tree.txt`: depth-one dependency tree has no `pollster`, `wasm-bindgen-test`, or `naga`.
- `📝️p9r-wgpu-dependency-tree-census.txt`: no `pollster`/`wasm-bindgen-test`; three `naga` occurrences are transitive below WGPU.
- `📝️p9r-wgpu-native-check-1.txt`: isolated native check reached the pre-renderer Flow dependency and stopped on 14 unrelated Flow diagnostics (removed BRep bridge import, catalogue channel shape, and async host call shapes). No renderer-owned diagnostic was emitted before that boundary.
- `📝️p9r-wgpu-native-check-2.txt`: the second isolated native check again stopped before renderer compilation: 14 Flow errors and 2,740 Puzzle async-migration errors. These are explicitly out of this packet's ownership.
- `📝️p9r-wgpu-owned-diff.txt`: exact owned diff snapshot.

## Gate status

- Formatting: green via package `cargo fmt -- --check`; evidence `📝️p9r-wgpu-fmt-check.txt`.
- Static forbidden/dependency ratchets: green.
- Native debug/release, focused WGPU unit tests, and native binary: not compiler-reachable because Cargo stops in the external Flow/Puzzle walls above. The UI-host WASM boundary is green (`📝️p9t-ui-host-wasm-check.txt`); full renderer WASM remains behind the same product dependency migration wall. Rerun in the warm isolated target after Flow and Puzzle are green.
