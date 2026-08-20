# Codebase Map — Baseline for the Interactive Job Runtime Refactor

Baseline commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`. Produced by three read-only exploration fleets (async/actor, UI/host, simulations/dependencies). Line numbers are approximate anchors from the baseline commit — re-verify before editing.

## 1. Async runtime

Crate `semio-framework-async` — `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/🦀️component.rs` (~929 lines).

| Symbol | Anchor | Note |
| --- | --- | --- |
| `OperationContext` | ~55–77 | trace, lane, deadline_ms, cancel, capability token |
| `CancelToken` / `CancelState` | ~87–177 | tri-state Live/Park/Cancelled, parent-chain hierarchy |
| `ScopeOwner` / `ScopeHandle` / `ScopeDrainReport` | ~207–251 | structured scopes; no detached spawn |
| `ChannelPolicy` | ~261–266 | LatestWins, Coalesced, LosslessBounded, ByteCredit |
| `ThreadPlan` | ~275–281 | kernel, shards, io_workers, compute, epoch_ticker — **replaced in Phase 1** |
| `thread_plan()` | ~301–308 | invariant: cores ≥ 4 ⇒ shards + compute + 1 ≤ cores |
| `ThreadRole` / `ThreadBudget` | ~313–365 | atomic subtraction, **debug-only assertion — can wrap in release** |
| `HostAsyncRuntime` | ~378–443 | open_scope, spawn_scoped, `run_blocking`, sleep_until, cancel_scope, now_ms |
| `block_on` | ~466–500 | thread::park native, busy-loop on wasm32 |
| `ManualRuntime` | ~510–696 | testkit double, no tokio |

Macros: `🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/🦀️component.rs` — `#[async_test]` expands to sync `#[test]` with inline `block_on` + ThreadWaker.

## 2. Actor layer

Crate `semio-framework-actor` — `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/🦀️component.rs` (~3569 lines). **Keep the semantics, re-host the execution.**

| Symbol | Anchor | Note |
| --- | --- | --- |
| `PackageId`, `ActorId`, `ActorKind` | ~217–372 | bit-packed identity (plugin_ordinal/kind/ordinal/generation) |
| `Lane` | ~380–433 | Interactive, UserVisible, Background, Maintenance + priority_rank/weight |
| `Budget` | ~440–487 | fuel, wall_ms, memory_bytes, ui_nodes, mailbox_len, max_effects, max_patch_bytes |
| `lane_defaults::budget_for` | ~497–504 | 4 / 16 / 50 / 200 ms — **become grants of many bounded steps** |
| `Origin`, `Payload`, `CoalesceKey`, `Envelope` | ~529–696 | `Payload::{Event,Suspend,Resume,Cancel,JobStep}` — job protocol seeds |
| `TurnStatus`, `Usage`, `TurnResult`, `Backpressure` | ~705–836 | Idle/MoreWork/CheckpointReady/Faulted |
| `Mailbox` | ~836–948 | coalescing, backpressure, deadline tracking |
| `FailureSignal/Stage/Escalation`, `FailureState` | ~981–1270 | Healthy → Throttled → Disabled → Quarantine |
| `ShardId`, `ShardKind`, `ShardTable`, `ShardTransport` | ~1366–1607, ~2202–2294 | Thread/WebWorker/Process; `ThreadTransport` is native-only mpsc |
| `Decision`, `TurnGrant`, `Scheduler` | ~1608–1826 | two-level DRR (plugin, then actor); `TURN_ENVELOPE_BATCH = 8` |
| `SceneSnapshot`, `SceneStore` | ~1834–1918 | immutable snapshots; `apply_patch` + `commit_frame` — **extended with preview overlays in Phase 2** |
| `ActorMetrics` etc. | ~1940–2188 | wall_us ring, p95 saturation detection |
| `Kernel` | ~2331–2570+ | activate/submit/tick/complete, scenes per window, failure ladder |

## 3. Thread creation census (baseline)

1. `TokioHostRuntime` — `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/🦀️component.rs` ~238–326; builds tokio runtime from `ThreadPlan`; `ScopeTable` ~88–236; timer wheel `WheelCore`/`TimerWheel` ~328–415.
2. Shard executor threads — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`, `ShardExecutor::spawn()`, one OS thread per shard, runs wasmtime guests.
3. Shard forwarder threads — `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` ~87–126, named `semio-os-host-kernel-shard-forward-{i}`, each running `block_on(recv_deadline(FORWARD_POLL))` at 250 ms.
4. Epoch ticker thread (per `ThreadPlan`).

## 4. `block_on` / `run_blocking` call sites

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` ~254 — CLI root (**stays**, approved entry point).
- `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` ~114 — forwarder poll loop (**deleted in Phase 1**).
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` ~1956–1985 — extension pack/install (**moves to I/O lane job**).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/…/🦀️component.rs` ~605 — `ComputeScheduler::run_blocking` (**becomes job submission**).
- db storage `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs` — disk I/O via run_blocking.

## 5. UI runtime

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs`

- `UiRuntime` struct ~133–150: store, tracking, inbox, gateway, presence, surfaces, pending_first_present, pending_intents, custom_handlers, pending_wakes.
- **`transact(now_ms)` ~246–261** — the run-to-completion transaction: `drain_and_apply_deltas` → `route_intents` → `flush_effects_to_fixpoint` → `present_dirty_surfaces` → `reconcile_trees` → collect gateway output / presence / next_wake.
- Limits: `PROJECTION_DRAIN_LIMIT = 256` (~113), `EFFECT_STORM_BUDGET = 64` (~109), `DEFAULT_REVISION_TOLERANCE` (~280), `is_stale_intent` revision guard.
- `SurfaceSlot` ~60–89: fn-pointer vtable erasure (ruling U3, not `dyn`), holds `SurfaceReconciler`.
- Reconciliation: `🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs` ~57–100 `SurfaceReconciler::reconcile()`.

## 6. Host / window layer

`🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs`

- **`WindowDelegate` ~252–274**: `scheduler_mut`, `handle_event(DispatchEvent)`, `handle_metrics(WindowMetrics)`, `redraw(InvalidationReason) -> RedrawOutcome`, `close_requested`. All synchronous — **becomes enqueue/present-only in Phase 3**.
- `NativeHost` (winit) ~347–533: `normalize()`, `resumed()`, `window_event()`; `NativeRuntime::new()` builds `EventLoop<WakeMessage>`; `run_native()` blocks in `EventLoop::run_app`; `WakeProxy` cross-thread wake.
- `CanvasHost` (browser) ~588–690: `ResizeObserver`, `visibilitychange`, RAF dedup via `raf_pending`, `on_animation_frame()` → `delegate.redraw()` (~687).
- `BrowserClipboard::read_text_async` ~203–211 — only sanctioned async outside the event loop, result re-enters as `DispatchEvent::Paste`.
- Event normalization: `🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️event.rs` — winit/web_sys → platform-neutral `DispatchEvent`, `PointerRegistry`.
- OS renderer seam: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs` ~38–114 (`OsHost`'s WindowDelegate impl), `🦀️os_host.rs` ~105–138 scheduler+kernel seam.

## 7. Render pipeline

- `FrameScheduler` — `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️schedule.rs` ~82–148; `should_render(now)` gates idle/hidden and coalesces N invalidations into one frame; `InvalidationReason` bitflags ~26–35 (STRUCTURE, LAYOUT, PAINT, ANIMATION, THEME, VIEWPORT, RESOURCE_READY, INPUT_STATE, SURFACE, ACCESSIBILITY).
- `SceneBuilder`/`Scene` — `🖼️render/📦️packages/🦀️rust/🦀️scene.rs` (frame-local).
- Hit testing — `🖼️render/📦️packages/🦀️rust/🦀️dispatch.rs` ~140–200+: `hit_test(tree, root, x, y)`, reverse-paint DFS, `DispatchFlags` ~48–103 (CLIPS_CHILDREN, HIT_TRANSPARENT, OVERLAY, DRAG_SOURCE, DROP_TARGET, SCROLLABLE, FOCUSABLE, LAYOUT_CONTAINER, OVERLAY_TRIGGER, EDITABLE); capture/target/bubble routing.
- wgpu target — `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`: `🦀️dispatch.rs`, `🦀️layout.rs`, `🦀️text.rs`, `🦀️tessellate.rs`, `🦀️gpu.rs`, `🦀️backend.rs`.

## 8. TypeScript / browser side

- `FrameworkOsShell` React component — `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🎯️targets/⚛️react/`.
- Dev multi-shell harness — `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🧩️multi.tsx` ~18–66; `resolvePlaygroundBoot(PLUGIN_CATALOG, variant)`; variants cad, gis2d.
- Plugin registry/catalog — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/` (`🟦️catalog.ts`, `📜️script.ts`).
- Backbone protocol — `🧰️framework/🛍️products/💻️os/🟦️component.ts`; worker `🟦️backbone-worker.ts` runs the WASM OS kernel off the main thread.

## 9. Simulations

### Puzzle 3D — `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/`
- `🪣️fill/🦀️component.rs`: `FillBuilder` ~24–37 (base, fixture, applied_count, sequence, appended objects/attractions, placed collision entries, candidate cache, RNG state, stalled flag, max_count), `PlacedCollisionEntry` ~18–22, `FILL_COUNT_MAX = 1000`.
- `🦀️component.rs`: `Puzzle3dCollision` two lanes (brush candidate cache, fill planning); **`PUZZLE3D_PRECOMPUTE_STEP_BUDGET_MS = 12.0` ~58**; `precompute_step_lane` ~389; `precompute_step` ~460; `fill_progress_summary` ~467; `fill_progress`, `apply_fill_count`, `compose_fill_display`.
- `📐️geometry/🦀️component.rs`: **`solid_overlap_volume`** (indivisible — becomes sample-batched), `CollisionBody`, `Pose3d`, `pose_isometry`, `world_bounds`, `world_volumes_contain_aabb`.
- `🖌️brush/🦀️component.rs`: `brush_compatible_candidates`, `brush_candidate_suggestion_weight`, `enumerate_brush_fill_vortex_targets`, `brush_preview_from_candidate`.

### Puzzle 2D — `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🦀️component.rs`
`Puzzle2dCamera`, `Puzzle2dHandle`, `Puzzle2dNode`, `Puzzle2dNodeAnchor`, `PUZZLE_2D_SCHEMA = "puzzle.2d.fixture"`.

### WFC — `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/`
- `💡️inferences/🦀️component.rs`: **`compile_and_solve(snapshot, seed)` ~19** (monolithic), `AssemblySolve` ~76 (`store::InferredField`), `AssemblySolveResult` ~68. Determinism already explicit: seed is an argument, no ambient randomness.
- `💡️inferences/🧩️wfc-engine/`: `🌐️domain/`, `🎼️motif/`, `🐾️trail/`, `⛓️constraint/`, `⚠️error/` — **reuse the math, restructure the control flow**.
- `📸️snapshot/🦀️component.rs`: `AssemblySnapshot` holds the problem only; solution is derived.
- Types: `ModelBuilder::add_pattern`, `add_relation`, `allow_mirrored`, `GraphTopologyBuilder`, `GraphSolverBuilder::fix`, `solver.solve(seed)`.

### FEM — engine `✏️s/🔨️modules/🏗️fem/⚙️engine/`, plugin `✏️s/🔌️plugins/🏗️fem/`
- `🏗️model/🦀️component.rs` model assembly; `📏️elements2d/`, `🧊️3d/`; `➕️algebra/🦀️component.rs` `VecD`/`MatD` ~18–202 (**`async fn` that never suspend** — `zeros`, `identity`, `get/set/add_at`, `transpose`, `matmul`, `mul_vec`, `add_triple_product`); `🔢️sparse/`; `🧮️analyses/`; `🧊️3d/🎵️modal-buckling/`; app surface `⚙️engine/🖥️app-surface/🦀️component.rs`; artifacts `🗿️artifacts/◻2d/`, `🗿️artifacts/🧊️3d/`.
- Deps: `spade` (Delaunay), serde, serde_json, thiserror, wasm-bindgen.

### Energy — `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs`
`Engine` ~19; **`Engine::run(model, config)` ~23–100**: validate ~24 → `resolve_weather` ~27 → `PrecomputedModel::build` ~28 → `SimulationKernel::initialize` ~31 → `warmup` ~32 → hourly timestep loop ~41–74 (`advance_timestep`) → post-process (sizing, summaries, environmental, resilience, LCCA). Accumulators `MeterTable`, `TimeSeriesTable`; `EndUse`, `FuelType`, `EmissionFactors`, `SourceEnergyFactors`, `LccaParameters`, `compute_lcca`. Loop is resumable at hour boundaries.

### Other plugins
`📐️cad`, `🖍️draw`, `📏️layout`, `🧱️block`, `🏭️process`, `🪵️sourcing`, `🌿️vcs`, `🎞️animate`, `🌀️procedural` (procedural2d/3d flow graphs). Command routing: `🧰️framework/🔨️modules/🔀️dispatch/🦀️component.rs`, `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs`; per-plugin `.editor_mutation_roster::<EditorType>()`.

## 10. Dependency surface (to be eliminated)

Workspace pins — `Cargo.toml` ~157–164: `serde 1.0.228`, `serde_json 1.0.149`, `flate2 =1.1.9`, `libz-sys =1.1.29`, `wasm-bindgen 0.2.106`, `thiserror 2.0.18`, `tokio 1`, `ts-rs 10`.

By reference count: ports (58, internal), serde/serde_json (54/42), wasm-bindgen (20), web-sys (13), tokio (12), thiserror (8), ts-rs (7), syn (6), image (6), bytemuck (6), wit-bindgen (5), wgpu (4), wasmtime (4), zip (3), winit (3), vello (3), swash (3), rusqlite (3), parley (3), getrandom (3), objc2 family (4), gltf (2), fontdb (2), axum (2), sqlx (2), uuid (2), miniz_oxide (2), jsonschema (2), nalgebra (1), parry3d (1), reqwest (1), rayon (1), neo4rs (1), naga (1), notify (1), prost (1), spade (FEM), criterion (bench).

JS (`package.json`): nx 21.6.11 + @nx/js + @nx/devkit + @nxlv/python, esbuild 0.27.2, vite-plugin-singlefile, binaryen, @playwright/test + playwright 1.57.0, vitest 4.0.17 + coverage-v8, storybook 10.4.0 (+ react-vite, addon-docs, addon-vitest, eslint-plugin), eslint 10.0.1 + @eslint/js + typescript-eslint, dependency-cruiser, lint-staged, @mdx-js/rollup, remark-gfm/mdx-frontmatter/frontmatter, rehype-slug/autolink-headings, react + react-dom 19.2.3, typescript 5.9.3, chevrotain 11.0.3, globals, @types/node, @types/react.

Existing owned replacements to build on: `🧰️framework/🔨️modules/🎒️pack/` (record/pack codec, `.spr`), `🧰️framework/🔨️modules/🧬️schema/` (schema generation), `🧰️framework/🔨️modules/📚️compiler/` (DSL compilation).

## 11. Test infrastructure

- Levels (`📜️script.ts` ~37–41, ~1220–1222): `fundamental` (`test`), `quick` (`test-quick`), `long` (`test-long`), `exhaustive` (`test-exhaustive`).
- Rust via `cargo nextest` (installed ~327–328); Nx via `bun nx run-many -t <level> --all` (~832, ~1020); Storybook via `@storybook/addon-vitest`; E2E via Playwright (`test:storybook` ~147).
- Coverage: LCOV merge/parse/render (~34), Go coverage args (~28), threshold enforcement (~21).
- Determinism today: Puzzle 3D `async_test` + `puzzle3d_now_ms()`; WFC explicit seed + `InferredField` DepHash caching; Energy deterministic by model/config/weather.
