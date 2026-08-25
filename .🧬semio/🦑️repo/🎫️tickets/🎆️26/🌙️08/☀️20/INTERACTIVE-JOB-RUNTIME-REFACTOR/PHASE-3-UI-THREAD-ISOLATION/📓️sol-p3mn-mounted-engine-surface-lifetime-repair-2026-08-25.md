# P3mn Mounted Engine Surface Lifetime Repair

Date: 2026-08-25

Owner: `/root/p5b_external_caller_propagation`

Source verdict: **SOURCE/STATIC AUDIT-READY; INDEPENDENT TERRA ACCEPTANCE AND RUNTIME GATES REMAIN DEFERRED**

## Boundary and prerequisites

The implementation follows `📓️p3mn-mounted-engine-surface-lifetime-repair-contract-2026-08-24.md`. The accepted P3 fixed CPU registry, P5a mounted frame transaction, P5b live reconcile, P5c layout/text, P5d prepared render, and P5e safe resize lane were treated as prerequisites rather than compatibility seams. P5e's previously explicit paired-surface dependency is now implemented directly in the mounted presenter and host disposer.

The live route is:

```text
FrameTransaction prepared EngineCanvas packet
  → AppPresenter retained packet cursor
  → EngineCanvasPresenter fixed token-aligned GPU slot
  → Reserve → Texture → View → Renderer → Render
  → ReplacementTexture → ReplacementView → Stage → Publish
  → exact displaced live-surface retirement

native/browser metrics callback
  → P5e fixed resize lane
  → AppPresenter Apply → InvalidateEngine → ApplyGpu → Retire → Complete

native/browser close or interrupted retirement
  → fixed OsHost abandonment registry
  → PairedEngineSurfaceClose Scan → BeginCpu → BeginGpu → Cpu → Gpu → Witness
  → exhaustive CPU child disposers and GPU candidate/live retirement
```

## Changed-file inventory

| File | P3mn change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` | Fixed 256-byte surface identity, checked fixed CPU slots, fixed token-aligned GPU slots, staged realization/publication, ordinary displaced-owner retirement, fixed 256-slot rejected-packet authority, live CPU-slot freshness, exhaustive field-wise CPU retirement, nonblocking CPU close probes, and hostile laws. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Mounted staged presenter work, metrics invalidation, retained realization faults through `Aborted` incremental close, RuntimeMailbox CPU close capability, and AppPresenter paired-close accessors. Peer MountedReplayRecovery/JobReplay regions were preserved. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs` | Added retained paired CPU/GPU close cursor, fixed generation-qualified 64-slot host-retirement handback, terminal ordering, Drop recovery, and hostile abandonment law. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs` | Native `CloseRequested` now transfers the exact host into retained retirement; refusal retains the host; idle pumping recovers abandoned owners; a returned presenter fault reschedules while retained close remains pending. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️browser_worker.rs` | Browser close/tick now use the same exact retained host transfer and abandonment pump. |
| `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️component.rs` | Added the domain-owned `GraphHostRetirement` cursor. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` | Added `FlowHostRetirement` across DAG, fixtures, outputs, neural cache, history, pending state, and store ownership. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` | Added fixed snapshot/mutation retirement factories and exact `ArtifactStore` owner installation for every production FlowHost store. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs` | Added retained `NeuralCacheRetirement`, including shared-owner and one-entry map close. |
| `🧰️framework/🔨️modules/✍️editor/🦀️component.rs` | `EditorHostRetirement` now destructures every host field and witnesses every retained text/cache collection without a terminal whole-host drop. |
| `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` | `MapHostRetirement` now destructures every host field and witnesses feature, tile, event, selection, and string owners without a terminal whole-host drop. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️component.rs` | Added/extended retained BoardHost ownership close. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` | `DagHostRetirement` now destructures every DAG host field and witnesses engine, icon, fixture, map, set, note, ghost, and pending-string owners without a terminal whole-host drop. |
| `📜️script.ts` | Strengthened the isolated P3mn gate to 23 faithful live-callee mutations, including all five independent-audit counterexamples; retained the accepted P5 predicates. |

The shared tree contains accepted P3/P5 and concurrent P2c/P6 edits in several files above. This inventory describes P3mn-owned regions, not ownership of every working-tree line.

## Ownership and close laws

| Owner | Admission/publication | Refusal, stale, close, Drop |
|---|---|---|
| CPU surface identity | Fixed `[u8; 256]` plus length; fixed 256 slots; checked nonzero generation | Oversize, duplicate, saturation, and generation exhaustion refuse before mutation; exhausted slots never wrap or alias |
| CPU surface graph | One token owns canvas, node graph, map, editor, board, events, caches, tile requests, and scalar fields | `EngineSurfaceRetirement` advances one retained child/item/scalar/backing opportunity; populated witnesses precede slot reuse |
| CPU close access | `WorkerCell::try_lock` returns an explicit contention result for scan, begin, and terminal witness | Contention retries the same paired phase; it cannot block the worker or masquerade as an empty slot |
| GPU slot | Fixed slot aligned to the CPU token; candidate retains every texture/view/renderer/render-stage owner | Stale document/scene/metrics/raster identity starts candidate close; one candidate/live/retirement field advances per call |
| Paired publication | Revalidates CPU token, document generation, scene revision, metrics generation, and raster witness before the single live swap | Rejection retains the candidate; displaced live ownership enters the slot's retirement cursor before another publish |
| P5e resize | Fixed metrics candidate enters Apply then scans one engine slot in `InvalidateEngine` before GPU capability and old-target retirement | Zero, stale, superseded, close, and Drop remain in the P5e retained authority; no compatibility registry was added |
| Paired host close | One scan slot and exact CPU/GPU token, operation, sequence, presence witnesses | CPU begin precedes GPU begin; both terminal witnesses must be empty before advancing; mismatch faults without erasing owners |
| Lost host retirement | Fixed 64-slot `AtomicPtr` registry with checked nonzero generation and reservation marker | Native/browser refusal keeps the host; interrupted wrapper Drop publishes the exact retirement state; mounted pumps recover one state per opportunity |

## Independent Terra RED remediation

The first independent audit in `📓️terra-fresh-p3mn-independent-source-static-audit-2026-08-25.md` found five source blockers. All five were remediated in the mounted production path:

| Audit blocker | Production repair | Exact terminal/freshness evidence |
|---|---|---|
| Ordinary replacement stranded the displaced GPU surface | `EngineCanvasPresenter::realize_step` now advances `slot.retirement` before candidate work, releasing exactly renderer, view, then texture on separate grants | The terminal candidate marker stays retained until the displaced cursor is terminal; only then can the packet complete and a later replacement begin |
| Realization `Err` could wedge `Aborted` without another redraw | `AppPresenter` retains the formatted fault, returns `Pending`, advances the exact candidate/frame/raster owners through `Aborted`, then returns the fault only after incremental retirement | Native `Err` also invalidates `RESOURCE_READY` whenever presenter ownership remains pending; close drains the retained fault one scalar per grant |
| A second packet overflow could ordinary-drop its producer | `EngineCanvasBuildContext` owns fixed ready and rejected arrays of 256 slots each; a generation/sequence-qualified reservation selects the exact destination before scene construction | MAX+1 returns the unchanged `EngineSurfaceSnapshot`; terminal requires both arrays empty, zero outstanding reservations, and published sequence equality |
| Editor/Map/DAG/outer EngineSurface used terminal whole-host drops | Every disposer constructor now exhaustively destructures its host, stores each nonopaque owner explicitly, advances one field/item/scalar opportunity, and witnesses every retained field | Production slices contain no `ManuallyDrop::drop(&mut self.host)` or `ManuallyDrop::drop(&mut self.surface)` terminal path |
| GPU publish compared freshness only with its packet | `Publish` performs a nonblocking lookup of the current CPU slot and compares exact token/id, metrics generation, document generation, and scene revision through `matches_live` | Contention retains the candidate and returns `Ok(false)`; disappearance or mismatch begins exact candidate close before returning the retained fault |

## Boundedness and hostile laws

The fixed CPU and GPU registries each contain 256 slots. The host abandonment registry contains 64 slots. GPU realization advances one explicit phase and at most one device/renderer capability call per presenter opportunity. CPU retirement delegates to domain-owned cursors and advances one retained child, item, character, byte page, control, or scalar opportunity. Paired host close scans or advances one token/phase per call. No nested scheduler, dynamic surface registry, whole registry take, recursive host Drop, or generation wrapping remains in the mounted P3mn path.

Bound hostile laws:

- `engine_surface_registry_accepts_exact_id_capacity_and_refuses_generation_exhaustion`
- `populated_graph_map_editor_surface_closes_one_fuel_turn_at_a_time`
- `populated_flow_surface_closes_history_and_cache_before_slot_reuse`
- `engine_packet_capacity_plus_one_returns_the_exact_snapshot_before_scene_transfer`
- `gpu_publish_freshness_uses_the_live_cpu_identity_metrics_document_and_scene`
- `normal_replacement_drains_displaced_renderer_view_texture_before_next_candidate`
- `child_and_outer_surface_retirements_require_explicit_field_witnesses`
- `realize_fault_remains_scheduled_until_the_aborted_cursor_is_terminal`
- `interrupted_host_retirement_is_rediscovered_and_fixed_registry_refuses_max_plus_one`
- Existing P5e million-resize, zero-size, stale, abandonment, and one-slot theme propagation laws remain bound by the preservation gate.

The P3mn verifier performs 23 mutations over the live production callees. In addition to the original CPU generation, nonblocking close, fixed GPU registry, paired close, host abandonment, P5e invalidation, Graph/Flow disposer, and hostile-law mutations, it now faithfully removes the ordinary replacement drain, returns a realization fault before retained close, collapses rejected-packet admission to one slot, restores packet-self freshness, and weakens the outer EngineSurface and each Map/Editor/DAG terminal witness. Every mutation is rejected by a predicate scoped to the actual mounted callee body.

## Static evidence

| Gate | Result |
|---|---|
| `bun ./📜️script.ts verify interactivity p3mn` | GREEN; baseline plus 23 faithful live-callee mutations |
| `bun ./📜️script.ts verify interactivity p5d` | GREEN |
| `bun ./📜️script.ts verify interactivity p5e` | GREEN; directly runs accepted P5b/P5c/P5a/P5d preservation self-tests before P5e |
| Rust edition-2021 rustfmt, 12 exclusive P3mn Rust files | GREEN |
| Shared `📦️glue.rs` edition-2021 rustfmt parse | GREEN; whole-file formatting was not applied across concurrent peer regions |
| Scoped `git diff --check` and staged diff check | GREEN |
| Residual census | Zero dynamic Engine GPU/CPU surface registries, `realize_one`, surface-generation `wrapping_add`, ordinary slot `mem::forget`, single rejected-packet slot, terminal Editor/Map/DAG/EngineSurface whole-owner drop, or old infallible host-retirement transfer in the mounted files |

## Deferred runtime gates and audit request

Cargo, Nx, Wasm, browser, native window/device, and timing gates were not run by instruction while the shared tree has overlapping source packets. Therefore no measured claim is made that opaque backend calls such as texture creation, renderer construction, or render-to-texture individually complete below 8 ms on every device. The source boundary admits only one such capability call in a retained phase; native/browser device-loss, allocation-counter, multi-window churn, and serialized timing matrices remain required once broad gates are permitted.

This report does not self-accept P3mn. A fresh independent Terra audit must trace FrameTransaction through CPU/GPU claim and publication, ordinary displaced-owner retirement, live CPU-slot freshness, fixed multi-slot packet rejection, P5e invalidation, populated field-wise child retirement, native/browser refusal and Drop recovery, reproduce all 23 mutations, and decide GREEN or RED from live source.
