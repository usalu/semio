# P5e Fresh Independent Safe-Scope Source/Static Audit

Date: 2026-08-25  
Auditor: `/root/p5b_external_caller_propagation/p5e_safe_scope_fresh_audit`  
Scope: read-only live-source/static audit; no source edits, Cargo, Nx, Wasm, browser, build, or runtime execution.

## Verdict

**GREEN — P3mn-independent safe scope only.** The live primary native and browser-Winit callback route now retains only fixed scalar resize metadata and defers preparation to the shared interactive worker pool. The claimed limited boundary is source/static coherent.

**RED — complete P5e / Phase 5 acceptance.** This is not a substitute for the P3mn prerequisite or the required runtime matrix. The primary resize presenter still calls the opaque `GpuContext::resize` capability, whose implementation synchronously configures the surface and recreates depth resources. No source evidence bounds that call at 8 ms or provides the required paired CPU/GPU candidate publication.

## Governing Inputs Read

- `📋️master.md` — interactive operations are persistent state machines; UI callbacks/worker steps retain the 8 ms hard ceiling.
- `📓️p5e-multi-window-resize-surface-lane-repair-contract-2026-08-24.md` — requires P2a1, P5c, P5d, and P3m/P3n before complete implementation; mandates fixed admission, freshness, exact close, and discriminating mutations.
- `📓️sol-p5e-multi-window-resize-surface-lane-safe-scope-2026-08-25.md` — treated as a claim, not evidence.
- `📓️p3mn-mounted-engine-surface-lifetime-repair-contract-2026-08-24.md`, `📓️p3m-engine-gpu-surface-authority-census-2026-08-23.md`, and `📓️p3n-engine-surface-terminal-retirement-audit-2026-08-23.md` — P3mn remains explicitly RED/pending.

## Independently Traced Production Route

```text
native/browser Winit WindowEvent::{Resized, ScaleFactorChanged}
  -> OsHost::handle_metrics
     -> enqueue_host_metrics(EventQueue fixed latest metrics + VIEWPORT invalidation)
     -> SurfaceResizeAuthority::enqueue (one latest fixed request)
     -> RuntimeApply::Resize (retained runtime mailbox work)
  -> redraw_core
     -> close_abandoned_step
     -> SurfaceResizeAuthority::drive_one
     -> MountedWorkerJobSession::pump_one(renderer_worker_pool, Interactive)
     -> checked token + metrics freshness take/restore
     -> AppPresenter::begin_surface_resize / surface_resize_step
     -> Apply -> Retire -> Complete; redraw remains invalidated while work remains
```

Evidence:

- `🦀️surface_lane.rs:10-16, 194-280` uses 64 static occupancy/generation/abandonment slots, checked generation increments, `MountedWorkerJobSession`, `Lane::Interactive`, fuel `1`, and a `1 ms` job budget. `pending.replace(request)` implements newest-only metrics retention.
- `🦀️surface_lane.rs:68-137, 227-249, 299-365, 442-469` checks cancellation/deadline, retains close owners one at a time, refuses stale ready candidates both on take and restore, and uses the fixed `AtomicPtr` abandonment registry for Drop recovery.
- `🦀️winit_app.rs:104-110` has no presenter/GPU/platform resize call: it only enqueues fixed EventQueue metrics, SurfaceResizeAuthority metrics, and a runtime apply envelope. `WindowEvent::{Resized,ScaleFactorChanged}` call this same method on both native and wasm/browser builds (`:761-771`).
- `🦀️winit_app.rs:135-154` drives the lane only on redraw; `📦️glue.rs:11569-11612` has the retained Apply -> Retire -> Complete cursor and preserves zero-size candidates by not invoking GPU resize.
- `🦀️os_host.rs:221-301` first closes the lane, then the presenter cursor, before remaining host retirement. `🦀️winit_app.rs:751-805` transfers the exact host to `OsHostRetirement`, advances it from `about_to_wait`, and exits only after terminal-empty.
- `🦀️engine.rs:324-397, 695-963, 1663-1783` retains fixed 64-slot lane entries with reason/epoch, deterministic weighted scheduling, and a fixed-slot Validate -> Apply -> Publish theme cursor. Its focused laws cover a 2,000-event layout resize coalesce, background fairness, and one fixed theme slot per opportunity.

## Callback and Coalescing Findings

The metrics callback contains no direct reachability to `AppPresenter::surface_resize_step`, `GpuContext::resize`, `surface.configure`, `create_texture`, or `create_view`. Therefore the callback path performs no GPU work. The shared Winit implementation supplies both native and browser-Wasm event paths, but no browser runtime was executed in this audit.

The lane test `million_resize_samples_retain_only_the_latest_exact_request` performs exactly 1,000,000 `enqueue` calls and asserts the retained payload is generation/width 1,000,000 (`🦀️surface_lane.rs:495-513`). This covers latest-metrics payload retention. It does not claim the final P3mn CPU/GPU replacement semantics.

## P3mn Residual Is Still a Real Blocker

Live source confirms every material P3mn red finding remains:

- `EngineCanvasPresenter` owns `HashMap<String, EngineGpuSurface>` (`EngineCanvas/🧊️component.rs:443-447`).
- CPU registry generations use `wrapping_add(1).max(1)` (`:155,171,210`).
- `begin_engine_surface_close`, `close_engine_surface_step`, and `engine_surface_terminal_nonopaque_is_empty` have zero production callers in the audited framework census; only local definitions/tests remain (`:895-908`).
- The P3n contract records that ordinary populated graph/map/editor owners cannot reach the current terminal witness.

The direct P5e presenter seam is intentionally not credited as a repair: `📦️glue.rs:11585-11588` invokes `GpuContext::resize`, and `🦀️gpu.rs:277-291` synchronously executes `surface.configure`, clears the live scene target, and recreates depth texture/view. This is a single isolated UI-capability call, not evidence of an <=8 ms prepared syscall or of all-or-nothing paired candidate publication. It must remain an explicit P3mn dependency.

## Verifier, Preservation, Formatting, and Diff Evidence

- `bun ./📜️script.ts verify interactivity p5e` exited 0. It executes live reconcile/P5b preservation, P5c, P5a, P5d, and the P5e self-test before its live P5e predicate.
- The 16 faithful P5e mutations in `📜️script.ts:9844-9859` all ran through that gate: dynamic registry, wrapping generation, bulk fuel, wide deadline, removed finite/stale/Drop/abandonment protection, callback resize, ordinary host Drop, cursorless presenter, dynamic/unqualified lane entries, dynamic/whole theme propagation, and removed Drop law.
- Scoped `rustfmt --edition 2021 --check` exited 0 for `surface_lane.rs`, `os_host.rs`, `winit_app.rs`, renderer `📦️glue.rs`, and UI `🦀️engine.rs`.
- Scoped `git diff --check` emitted no diagnostics for tracked P5e files. `surface_lane.rs` is currently untracked, so it was separately checked with `git diff --no-index --check /dev/null surface_lane.rs`, also with no diagnostics.
- No Cargo/Nx/Wasm/browser/build/runtime command was run. The source verifier does not establish final native/Wasm/browser/timing behavior.

## Acceptance Consequence

The P5e lane handoff can be retained as the narrow, P3mn-independent safe scope. Do not represent it as P5e completion, a P3mn repair, paired surface replacement, or an 8 ms proof. Complete acceptance remains blocked on P3mn's fixed paired registry, mounted exact close/recovery, populated terminal disposer, and then the serialized build/browser/device-loss/timing matrix demanded by the governing contract.
