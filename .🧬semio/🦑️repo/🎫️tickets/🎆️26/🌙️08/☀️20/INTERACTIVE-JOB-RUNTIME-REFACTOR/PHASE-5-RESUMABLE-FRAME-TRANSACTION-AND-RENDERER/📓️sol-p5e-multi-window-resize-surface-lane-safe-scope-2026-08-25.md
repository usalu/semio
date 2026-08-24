# P5e Multi-Window Resize and Surface Lane Safe-Scope Implementation

Date: 2026-08-25

Owner: `/root/p5b_external_caller_propagation`

Source verdict: **AUDIT-READY FOR THE P3mn-INDEPENDENT SCOPE; COMPLETE P5e REMAINS DEPENDENCY-BLOCKED**

## Boundary and prerequisites

P2a1, P5c, and P5d were independently GREEN before this packet began. Their source/static preservation self-tests remain GREEN on the final P5e source. P3m/P3n is still pending: the mounted EngineCanvas presenter retains `HashMap<String, EngineGpuSurface>`, and the paired CPU/GPU surface replacement plus populated terminal disposer has not been accepted. This packet therefore does not edit or claim the P3mn-owned EngineCanvas surface registry, paired publication, or paired close.

The implemented independent boundary is the fixed UI scheduling metadata plus the primary native/browser swapchain callback-to-worker handoff. It removes platform resize work from the metrics callback, preserves latest generation and exact close ownership, and exposes the retained capability point that the final P3mn replacement authority must consume.

## Changed-file inventory

| File | P5e change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️engine.rs` | Replaced bare surface tokens with fixed reason/epoch entries; added retained Validate→Apply→Publish theme propagation that advances one registry slot per opportunity; preserved weighted lane scheduling and mounted layout sessions. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️surface_lane.rs` | New fixed 64-slot generation-qualified resize authority, one-scalar worker job, exact rejection/close, latest metrics publication, zero-size suspension, and AtomicPtr abandonment rediscovery. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs` | Mounted the resize authority and placed it before presenter teardown in incremental `OsHostRetirement`; interrupted retirement publishes the lane into its fixed recovery slot. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs` | Metrics callbacks now enqueue fixed scalar owners only; redraw drives worker/admission/capability progress; native close transfers `OsHost` into one-unit retirement before event-loop exit. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Declared the new module and added retained primary presenter Apply→Retire→Complete capability state with exact candidate refusal and close. |
| `📜️script.ts` | Added the isolated P5e source/static gate, 16 direct live-source mutations, and direct P5b/P5c/P5a/P5d preservation execution; updated the P5c fixed queue predicate to the reason/epoch entry schema. |

The shared tree already contained accepted P5a–P5d and peer changes in several listed files. The inventory names P5e-owned regions, not authorship of the complete working-tree diff.

## Mounted route

```text
native Winit / browser worker Resize
  → OsHost::handle_metrics
      → EventQueue::enqueue_metrics
      → SurfaceResizeAuthority::enqueue
      → RuntimeApply::Resize
  → OsHost::redraw_core
      → close_abandoned_step
      → MountedSurfaceResizeLane::drive_one
      → shared renderer WorkerPool, Interactive lane, fuel 1, deadline 1 ms
      → PreparedSurfaceResize freshness check
      → AppPresenter retained capability cursor
```

The callback has no production reachability to `AppPresenter::resize`, `GpuContext::resize`, `surface.configure`, texture allocation, or view allocation. The browser worker reaches the same `OsHost::handle_metrics` implementation.

## Ownership and close table

| Owner | Admission/publication | Refusal, supersede, stale, cancel, close, Drop |
|---|---|---|
| Surface lane permit | Fixed 64 process slots; checked nonzero slot generation | Released only after pending/session/ready owners are terminal-empty; exhaustion never wraps |
| Metrics request | Fixed `Copy` scalar envelope with checked metrics generation | Non-finite scale returns the exact request; latest replacement returns the superseded request; stale candidates cannot be taken or restored |
| Resize worker | `MountedWorkerJobSession` on the shared pool, Interactive lane | Admission rejection retains the exact job plus pre-admitted fault backing; cancel/deadline/supersede enter incremental session close |
| Prepared candidate | Published only when slot generation and newest metrics generation match | Zero extent becomes suspended and preserves the last live surface; stale or refused presenter admission is restored to the exact lane |
| Presenter cursor | One fixed candidate, Apply→Retire→Complete | `close_surface_resize_step` detaches one candidate owner per call |
| Lost lane handle | Fixed per-slot `AtomicPtr<MountedSurfaceResizeLane>` | `close_abandoned_step` rediscovers one exact owner and advances one close unit; permit releases only at terminal empty |
| Native host | `WinitApp.host: Option<OsHost>` | `CloseRequested` moves the exact host into `OsHostRetirement`; `about_to_wait` advances one unit and exits only after terminal empty |
| UI theme propagation | One current fixed cursor and one latest pending `Theme` | Validates and applies one registry slot per call; checked revision exhaustion parks the exact cursor without alias or fallback |
| Lane queue entry | Fixed `SurfaceLaneEntry { token, reason, epoch }` | Stale epochs requeue the current generation; invalid generations cannot dirty or publish a replacement slot |

## Hostile laws

- `resize_job_consumes_one_scalar_per_grant`
- `million_resize_samples_retain_only_the_latest_exact_request`
- `zero_size_suspends_and_invalid_scale_returns_exact_producer`
- `interrupted_lane_drop_is_rediscovered_and_incrementally_closed`
- `changed_theme_propagates_one_fixed_surface_slot_per_opportunity`
- Existing P5c resize storm, weighted fairness, MAX + 1, stale generation, atomic layout publication, cancellation, deadline, and under-eight-millisecond laws remain bound by the preservation gate.

The P5e verifier performs 16 faithful mutations over the exact production files: dynamic slot/queue/theme storage, wrapping generation, bulk fuel, wide deadline, missing finite and freshness gates, missing Drop/drain, callback-local platform resize, ordinary native host drop, cursorless presenter handoff, unqualified queue entry, whole theme propagation, and missing Drop law.

## Static gates

| Gate | Result |
|---|---|
| `bun ./📜️script.ts verify interactivity p5e` | GREEN; P5b live reconcile, P5c layout/text, P5a frame transaction, P5d prepared render, and P5e 16-mutation self-tests all passed |
| `bun ./📜️script.ts verify interactivity p5d` | GREEN |
| Scoped Rust edition-2021 rustfmt | GREEN |
| Aggregate `bun ./📜️script.ts verify interactivity` | RED only at the pre-existing unrelated Puzzle FillBuilder whole-preview/result-envelope predicate; execution stops before aggregate P5 calls, which are therefore invoked directly by the P5e gate |
| Cargo/Nx/Wasm/browser/runtime/timing | Not run, per packet instruction and overlapping shared-tree work |

## Exact residual and dependency

Complete P5e is not claimed. P3mn must first replace `EngineCanvasPresenter { surfaces: HashMap<String, EngineGpuSurface> }` with its accepted generation-qualified paired CPU/GPU registry, expose resize replacement admission/publication/retirement, and provide a populated terminal disposer rediscoverable after handle loss. The current primary presenter capability cursor still invokes the existing `GpuContext::resize` as one isolated UI-capability call; that callee performs in-place `surface.configure`, old scene-target invalidation, and depth recreation rather than the contract's all-or-nothing candidate publication. No runtime ≤8 ms claim or paired-surface atomicity claim is made for that opaque call.

After P3mn is independently GREEN, P5e must directly replace this capability seam with the final paired replacement token and add native/browser device-loss, surface-loss, candidate-phase interruption, multi-window churn, allocation counter, and timing tests. No adapter or second registry was introduced here.
