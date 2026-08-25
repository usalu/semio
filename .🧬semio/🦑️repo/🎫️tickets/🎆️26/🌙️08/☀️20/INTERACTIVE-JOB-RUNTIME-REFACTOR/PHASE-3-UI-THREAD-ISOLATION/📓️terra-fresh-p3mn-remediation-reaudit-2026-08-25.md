# Terra Fresh P3mn Remediation Re-Audit

Date: 2026-08-25  
Auditor: `/root/p5b_external_caller_propagation/p3mn_fresh_terra_audit`  
Scope: current production source, verifier mutations, source-only formatting/parse/diff/census gates. No production/verifier edits; no Cargo, Nx, Wasm, browser, build, or device-runtime execution.

## Verdict

**GREEN — the P3mn source/static contract is accepted.**

This is a new audit and does not replace the prior RED report, `📓️terra-fresh-p3mn-independent-source-static-audit-2026-08-25.md`. I re-read that report and the updated Sol repair report, then traced the live FrameTransaction-to-presenter, CPU/GPU, native, and browser callees. All five prior blockers are closed in the current source. Runtime/device matrices remain deferred, not waived.

## Live ownership route

```text
FrameTransaction
  -> EngineCanvasBuildContext (256 ready + 256 rejection slots)
  -> AppFrameBuild::FrameEnginePackets (fixed 256 slots)
  -> AppPresenter::present_step / EngineCanvasPresenter::realize_step
  -> fixed token-aligned GPU slot:
       Reserve -> Texture -> View -> Renderer -> Render
       -> ReplacementTexture -> ReplacementView -> Stage -> Publish

native CloseRequested / browser closeStep / interrupted Drop
  -> OsHost::try_into_retirement
  -> PairedEngineSurfaceClose:
       Scan -> BeginCpu -> BeginGpu -> Cpu -> Gpu -> Witness
  -> fixed 64-slot generation-qualified abandonment recovery
```

The CPU registry has 256 fixed slots, an inline 256-byte surface ID, nonzero checked generations, and permanent exhaustion rather than wraparound (`EngineCanvas/🧊️component.rs:95-108`, `:160-175`, `:213-245`, `:283-307`). The GPU uses the matching fixed slot index and rejects an identity/generation mismatch before allocation (`:1160-1195`).

## Prior RED blockers: independent closure evidence

### 1. Consecutive ordinary replacement

The publication path atomically replaces `live` and retains the exact displaced GPU surface. The current normal `realize_step`, before considering a candidate, advances that retained cursor and returns `Pending`; `EngineGpuRetirement` releases renderer, view, and texture on separate calls before terminal (`EngineCanvas/🧊️component.rs:1007-1053`, `:1173-1178`). Therefore the next replacement starts only after the preceding displaced owner is fully retired, and later replacements are not wedged. The mounted hostile law is present at `:1631-1639`.

### 2. Realization fault, exact aborted cursor, and rescheduling

On an engine realization error, `AppPresenter::present_step` preserves the formatted fault, changes the exact retained cursor to `Aborted`, and returns `Pending`; the subsequent `Aborted` phase drives the active candidate, prepared-GPU cursor, raster witness, and completed frame through retained retirement before later returning the saved fault (`📦️glue.rs:12324-12357`, `:12384-12392`). Native converts every `Pending` to `RESOURCE_READY` and also invalidates after a returned error when the presenter remains pending (`🦀️winit_app.rs:193-220`). Browser tick requests another frame while `has_pending_presentation()` is true (`🦀️browser_worker.rs:278-310`). This removes the former unscheduled-Aborted counterexample.

### 3. Packet saturation, generation/sequence accounting, and close

Reservation chooses one of two fixed 256-slot destinations before scene construction; the 513th reservation returns the exact snapshot. Sequence and outstanding-reservation increments are checked, and terminality requires both arrays empty, no outstanding reservation, and published-sequence equality (`EngineCanvas/🧊️component.rs:807-854`). The 512-admission/MAX+1 hostile law asserts exact destinations and unchanged overflow snapshot (`:1593-1613`). Transfer saturation moves the exact packet into `FrameBuildCursor::engine_rejected`; close calls `EngineCanvasPacket::close_step` and drains fixed packet slots one at a time (`📦️glue.rs:10670-10750`, `:11970-11980`). I found no ordinary `Drop` handoff on this mounted saturation path.

### 4. Field-wise child and outer surface disposal

`EngineSurfaceRetirement::new` destructures `EngineSurface`, owns every nonopaque child/cache/scalar separately, and its phase/witness checks every retained source and child cursor before `Released` (`EngineCanvas/🧊️component.rs:357-425`, `:508-688`). Editor, Map, and DAG now likewise destructure their hosts, consume a single owned item/scalar per close opportunity, and assert a full terminal witness before ordinary Rust field destruction (`editor/🦀️component.rs:244-321`; `tiled-map/🦀️component.rs:1798-1887`; `dag/🦀️component.rs:2198-2404`). A direct census found no `ManuallyDrop::drop` in these three child disposer bodies and no outer `ManuallyDrop::drop(&mut self.surface)`.

The Graph/Flow chain remains domain-owned: outer retirement creates `GraphHostRetirement` or `FlowHostRetirement` and demands its terminal witness before advancing (`EngineCanvas/🧊️component.rs:533-552`); the P3mn predicate requires Flow DAG/neural/store close bodies, not merely their type names (`📜️script.ts:9985-9986`). Board is passed into its established `BoardHostRetirement` via `ManuallyDrop::into_inner` only when the outer cursor owns it; outer witness requires that cursor terminal before release.

### 5. Live CPU publication freshness and P5e metrics invalidation

At packet reservation, the current CPU slot validates token/id and metric equality, rejects document/scene regressions, then records the monotonic document and scene values. GPU `Publish` performs a nonblocking live CPU lookup and requires exact identity, metrics, document generation, and scene revision; disappearance or mismatch begins candidate close, while CPU contention returns `Ok(false)` without losing the candidate (`EngineCanvas/🧊️component.rs:1909-1960`, `:1002-1004`, `:1264-1277`). The focused hostile law rejects each live metric/document/scene divergence (`:1617-1627`). This is a live CPU authority check, not packet self-comparison.

For P5e, a newer primary metrics generation starts a fixed scan at slot zero; each invalidation call touches at most the indexed candidate then advances one index. `AppSurfaceResizePhase::InvalidateEngine` waits for the complete fixed scan before applying GPU resize (`EngineCanvas/🧊️component.rs:1138-1157`; `📦️glue.rs:12126-12165`). The P5e gate remains green below.

## Close, contention, refusal, and Drop

`PairedEngineSurfaceClose` retains a single token and progresses Scan/BeginCpu/BeginGpu/Cpu/Gpu/Witness. CPU lookup/begin/witness return contention as `false`, rather than treating it as absence, and CPU/GPU identity disagreement faults without erasing either owner (`🦀️os_host.rs:237-380`). Its state is driven from the retained host close sequence before presenter/world release (`:453-554`).

The 64-entry host-abandonment registry uses `AtomicPtr`, checked nonzero generations, exhaustion, a reservation marker, and bounded rediscovery. `OsHostRetirement::Drop` publishes the exact incomplete state to that registry; native and browser both pump it (`🦀️os_host.rs:188-235`, `:585-660`; `🦀️winit_app.rs:193-220`; `🦀️browser_worker.rs:278-315`). These live bodies close the prior native/browser refusal and interruption counterexamples.

The only `.lock()` found in the census is `WorkerCell::borrow` for unrelated worker-side map-tile state. The mounted CPU surface probe exports use `try_borrow_mut`/`try_lock`, and paired close handles contention explicitly; no blocking mounted close lock was found.

## Gate results

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p3mn` | **GREEN** — baseline plus **23/23** faithful current-callee mutations. The additions directly remove ordinary retirement progress, return realization fault before close, collapse rejected capacity, restore packet-self freshness, and weaken outer/Map/Editor/DAG witnesses (`📜️script.ts:10020-10052`). |
| `bun ./📜️script.ts verify interactivity p5d` | **GREEN**. |
| Direct P5e predicate + `interactivityMountedSurfaceLaneSelfTests` | **GREEN** — baseline plus **16/16** P5e mutations. |
| Direct preservation: P5b/P5c/P5a/P5d self-tests | **GREEN**. |
| Edition-2021 `rustfmt --check` on 11 exclusive P3mn/P5e Rust files | **GREEN**. |
| Edition-2021 `rustfmt --emit stdout` shared `📦️glue.rs` | **GREEN parse**. The shared file was not reformatted. |
| Scoped `git diff --check` on all audited source files | **GREEN**. |
| Prohibition census | **GREEN** — no dynamic CPU/GPU surface registry, `realize_one`, generation `wrapping_add`, `std::mem::forget(slots)`, single rejection slot, or forbidden child/outer whole-owner drop. |

The aggregate `bun ./📜️script.ts verify interactivity p5e` did not produce completion within the environment's 30-second command cap on two attempts. I therefore ran the exact constituent P5e predicate/mutations and each preservation self-test directly; all completed GREEN. No result is inferred from the timed-out wrapper itself.

## Residual deferred runtime gates

This GREEN verdict is source/static only. The following are still required once broad runtime gates are permitted:

- Native and browser/Wasm repeated replacement beyond several renderer/view/texture retirement cycles.
- GPU allocation/render/stage/device-loss faults, including the retained Aborted cursor through real scheduler wakeups.
- Contended CPU registry close, native refusal, browser close, and interrupted `Drop` recovery under scheduling pressure.
- 256/512 packet saturation with real opaque scene retirement, and P5e resize racing prepare versus GPU Publish.
- Measured one-grant timing/fairness with populated Graph/Flow/Map/Editor/Board/DAG and live device backends.

No source/static counterexample remains from this audit.
