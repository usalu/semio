# Terra Fresh P3mn Independent Source/Static Audit

Date: 2026-08-25  
Auditor: `/root/p5b_external_caller_propagation/p3mn_fresh_terra_audit`  
Scope: live source, root verifier mutations, formatting/parse/diff/census only. No production or verifier edits; no Cargo, Nx, Wasm, browser, build, or runtime execution.

## Verdict

**RED — P3mn source/static contract is not accepted.**

The fixed CPU/GPU arrays, checked nonzero generation, native/browser retained host close, and P5e one-slot invalidation handoff are real. The root verifier nevertheless accepts source that violates the contract's ordinary replacement and exact retirement guarantees. A second normal EngineCanvas replacement wedges the slot and then faults the presenter without scheduling its retained `Aborted` close. The packet authority can also destruct an additional over-capacity producer, while three claimed child disposers use an outer `ManuallyDrop::drop` as their terminal proof.

This is a substantive source rejection, not a deferred-runtime-only finding.

## Inputs and live route audited

Read completely: the P3mn repair contract and implementation report, P3m/P3n prerequisite reports, accepted P5d source audit/implementation report, and P5e safe-scope audit/implementation report.

The live route is present:

```text
FrameTransaction EngineCanvas packet
  -> AppFrameBuild fixed `FrameEnginePackets`
  -> AppPresenter::present_step / EngineCanvasPresenter::realize_step
  -> fixed GPU slot Reserve -> Texture -> View -> Renderer -> Render
  -> ReplacementTexture -> ReplacementView -> Stage -> Publish

native CloseRequested / browser closeStep
  -> OsHost::try_into_retirement
  -> PairedEngineSurfaceClose Scan -> BeginCpu -> BeginGpu -> Cpu -> Gpu -> Witness
  -> fixed 64-slot OsHost abandonment recovery
```

The mounted close uses nonblocking CPU probes: `ENGINE_SURFACES` is a `Mutex` guarded by `try_lock` in `EngineCanvas/component.rs:1258-1287`; scan, begin, and witness return a contention result rather than blocking (`:1643-1663`). `PairedEngineSurfaceClose` retains an exact token/phase and couples CPU then GPU in `os_host.rs:237-370`. Native refusal restores the host and `about_to_wait` pumps retirement (`winit_app.rs:751-829`); browser follows the same `try_into_retirement`/retained-host route (`browser_worker.rs:315-401`). These parts are source-positive.

## Blocking counterexamples

### 1. Ordinary replacement permanently blocks its own GPU slot

`EngineGpuSlot::publish_candidate` replaces `live` and stores the displaced valid surface into the only `retirement` cursor (`EngineCanvas/component.rs:930-933`). The normal presentation route has no retirement-progress phase: `close_active_candidate_step` only scans `candidate` (`:1107-1115`), and `EngineGpuRetirement::close_step` is reachable only from terminal `close_surface_step` (`:1135-1157`).

Therefore, after the first ordinary replacement:

1. The old live texture/view/renderer remains in `slot.retirement`.
2. The next EngineCanvas packet reaches `realize_step`.
3. `realize_step` refuses it solely because `slot.retirement.is_some()` (`:990-1002`).

The prior live surface is initially preserved, but the exact displaced retirement never advances during ordinary operation. This violates all-or-nothing replacement with exact displaced retirement and prevents subsequent normal presentation.

### 2. The replacement fault is retained but not mounted for progress

On that refusal, `AppPresenter::present_step` sets the cursor phase to `Aborted` and returns `Err` (`glue.rs:12304-12316`). The native caller only records `present_fault` on `Err`; unlike `Pending`, it does not invalidate the scheduler (`winit_app.rs:193-216`). `has_pending_presentation` remains true for this retained cursor (`glue.rs:12124-12125`), preventing a new frame from being admitted (`:12223-12230`).

Absent an unrelated future event/redraw, the next `Aborted` turn that would call `close_active_candidate_step` is never scheduled. Thus failure after a valid first replacement can leave a candidate/faulted packet retained indefinitely, contrary to mounted failure recovery and one-opportunity resumability.

### 3. Engine packet overflow silently destructs a second rejected producer

`EngineCanvasBuildContext` has 256 fixed packet slots and only one `rejected` owner. On full capacity it retains the incoming packet only when `rejected` is empty; when already occupied, `packet` falls out of `enqueue` and is destructed (`EngineCanvas/component.rs:643-693`, especially `:683-689`). The API returns no rejected producer. This contradicts the contract's requirement that a full terminal/output authority return the exact producer or leave it discoverable.

The current surface cap may make this rare in the usual render topology, but the source authority is not total: the private `enqueue` accepts every caller-provided snapshot/scene and has no preflight proof that a second overflow is unreachable. The hostile law/verifier does not exercise this state.

### 4. Claimed exhaustive child retirement still ends in ordinary whole-host drop

`EditorHostRetirement::close_step` pops selected collections then calls `ManuallyDrop::drop(&mut self.host)`; its terminal witness is only `released` (`editor/component.rs:217-281`, especially `:254-275`). `MapHostRetirement` does the same (`tiled-map/component.rs:1755-1845`, especially `:1808-1838`). `DagHostRetirement` directly drops its remaining outer `DagHost` after clearing only selected fields (`dag/component.rs:2209-2285`, especially `:2270-2279`).

The outer `EngineSurfaceRetirement` in turn accepts only child boolean witnesses and finally whole-drops `EngineSurface` (`EngineCanvas/component.rs:347-601`, especially `:570-595`). These are not exhaustive nonopaque field witnesses and permit uncatalogued fields/backings to be deep-dropped in one grant. The contract explicitly forbids this form of populated child/outer terminal proof. Existing populated Graph/Flow/Map/Editor fixtures establish that selected defaults terminate; they do not discriminate an added non-scalar child field or prohibit these direct drops.

### 5. Per-surface metrics freshness is self-comparison, not a live CPU authority check

CPU surface metrics advance in `ensure_surface` (`EngineCanvas/component.rs:1609-1640`), and a packet captures that value. During realization, the candidate only compares its captured `metrics_generation` to the same packet (`:749-778`, `:1017-1019`); it never consults the live CPU slot/snapshot. P5e's global primary metrics invalidation does scan one fixed GPU slot per opportunity (`:966-988`), but it is not a check of a packet's current per-surface CPU metrics generation.

Consequently a surface can resize after frame preparation but before realization while the packet/candidate comparison remains true. Document/scene/raster freshness has an upstream raster witness, but the required independent per-surface metrics freshness is absent.

## Static gate results

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p3mn` | **GREEN** — baseline and all **17/17** faithful mutations were rejected. This does not cover the live counterexamples above. |
| `bun ./📜️script.ts verify interactivity p5d` | **GREEN** — baseline and its 39 hostile mutations. |
| Direct P5e predicate + `interactivityMountedSurfaceLaneSelfTests` | **GREEN** — baseline and all **16/16** P5e mutations were rejected. |
| Direct P5e preservation self-tests | **GREEN** — P5b live reconcile, P5c layout/text, P5a frame transaction, and P5d prepared render. |
| `rustfmt --edition 2021 --check` on the 12 exclusive P3mn Rust files | **GREEN**. |
| `rustfmt --edition 2021 --emit stdout` on shared `glue.rs` | **GREEN parse**. Whole-file `--check` is **RED** on pre-existing/shared formatting drift outside the P3mn region; no formatting was changed. |
| Scoped `git diff --check` over the 13 audited P3mn files | **GREEN**. |
| Static census | **GREEN** for prohibited dynamic Engine GPU registry, `realize_one`, generation wrapping, ordinary slot `forget`, and blocking `ENGINE_SURFACES` close lock. |

The attempted aggregate P5e command exceeded the isolated 30-second command window before output. Its exact 16-mutation predicate and all four preservation self-tests were then invoked directly and completed green as recorded above.

## Verifier gap demonstrated

The P3mn verifier requires the strings `self.retirement = Some(EngineGpuRetirement::new(displaced))` and `retirement.close_step()` but does not require a normal-operation path from `realize_step`/`present_step` to advance the displaced retirement. Its `non-atomic-publication` mutation therefore passes while the real normal replacement lifecycle remains stuck. It likewise checks child disposer names and `close_step` signatures but not the `ManuallyDrop::drop` terminal paths or a full/rejected-full packet fixture.

## Deferred gates

No Cargo/Nx/Wasm/browser/native/device-loss/timing gate was run. Those remain mandatory after the above source blockers are repaired. In particular, real repeated EngineCanvas replacement, stale per-surface resize between prepare and realization, stage/device-loss failure recovery, and populated Graph/Flow/Map/Editor/Board close must run on native and browser-Wasm before Phase 3 can be accepted.

