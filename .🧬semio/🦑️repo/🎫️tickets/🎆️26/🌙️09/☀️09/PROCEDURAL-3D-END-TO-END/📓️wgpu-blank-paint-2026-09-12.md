# 🖌️ wgpu BLANK-PAINT — five defects between the intake and the paint pipeline, and the one that is left

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu blank paint + effects", 2026-09-12.
Resumes `📓️wgpu-playground-boot-2026-09-12.md` §6. Repo MCP was down all session
(`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no ticket was opened, closed or
reopened. Evidence under `🗑️generated/wgpu-paint/run-1 … run-10` and `🗑️generated/wgpu-boot/run-10`;
nothing under any `🗑️generated` folder was swept. The react serve on 6018 was not touched and the
procedural guest was not rebuilt or restaged.

---

## 1. TL;DR

| question | answer |
|---|---|
| Was the frame loop even running? | **Yes, and it never stopped.** 4 372 frames in 70 s, `requestFrame: true` on every one — the renderer asked for a frame 62×/s for 300 s and never finished one (§2). |
| Where was the gap? | **Five separate defects on the same path**, each one hidden behind the previous: an unretired document-lease alias, a per-step frame-build driver, a transient ingress refusal read as a terminal fault, a storm budget charged per step, and a superseded frame treated as a renderer fault (§3). |
| Are they fixed? | **Yes — all five, each verified on 6118 by the probe that exposed it.** The shell now boots in 4.3 s, builds and presents frames continuously at ~60 Hz with no fault and no quarantine, and drives its retained documents from page 0 to `finish_document` in ~15 ms instead of never (§4). |
| Does the canvas paint? | **No — and the reason is now exact, not a symptom.** `UiTree::publish_document` stores the published `UiDocumentTree` in a field that NOTHING ever reads back: the wgpu engine has no reconcile from a retained document's `UiNodeRecord`s into the paintable `UiTree` arena, so `tree.root` is `None` forever and `frame_into_step` answers `Missing` (§5). That is the remaining blocker and it is an engine feature, not a bug. |
| Defect B (no effects)? | **Narrowed to the guest, not the door.** The shell's `setContributions` crossing carries the right roster (`windowInstances` with `procedural-preview`) and the wgpu bridge already re-arms whatever comes back; the guest answers `effects: 0` because `set_contributions::apply` returns early or `rearm_attached_previews` finds nothing owed. The one wgpu site that discards turn effects wholesale is named in §6. |

---

## 2. What the baseline actually measured

`🐍️wgpu-probe.mjs` reads the Worker's introspection beacon; it cannot see whether the UI isolate is
driving frames at all. `🐍️wgpu-paint-probe.mjs` (new, this ticket folder) patches
`Worker.prototype.postMessage` and the worker's `message` handler **before boot** and counts every
crossing. `run-1`, against the build `📓️wgpu-playground-boot-2026-09-12.md` §6 reported on:

```
sent      { boot: 1, shard-port: 4, activate: 1, turn: 89, batch: 4372, introspect: 142 }
received  { booted: 1, wake: 2, frame: 4372, … }
requestFrameTrue 4372   requestFrameFalse 0
firstBatchAt 4833 ms    lastBatchAt 75406 ms
dumpFrameStats { windowId: "procedural-main", drawCalls: 0, quadCount: 0, glyphCount: 0 }
```

So the loop was healthy and the renderer never settled: `BLANK-PAINT` was never "the shell stopped",
it was "no frame the shell started ever finished". Two temporary `[DEBUG]` censuses (a chrome-step
census in `ShellState::render_chrome_step`, a retained-document census in `render_ui_document_step`)
turned that into the sequence below. Both censuses have been removed again; §7 says where to re-add
them.

---

## 3. The five defects

### 3.1 A per-frame document read leaked an arena alias — the surface went dark after SEVEN steps

`run-2`, the first instrumented boot, printed this and then nothing:

```
steps=31 phase=main-window child=4   ui-doc steps=1 vacant gen=1 nodes=30
steps=32 child=4                     ui-doc steps=2 pending page=0/30
…
steps=37 child=4                     ui-doc steps=7 pending page=5/30
steps=38 phase=main-window child=4   ← no ui-doc line at all
steps=39 phase=main-window child=5   ← the chrome walked PAST the window body
```

Exactly seven reads, then the body vanished from the walk. `render_main_window_step` reached its
document through `self.window_ui.get(window).and_then(|lease| lease.try_alias().ok())`, and
`UiDocumentLease::try_alias` mints an arena alias: `UiDocumentArena::alias` admits
`UI_DOCUMENT_LEASE_ALIASES` (8) per slot, a published document already holds one, and **only**
`close_read_step_with_grant` gives one back — `UiDocumentArena::release` has no other caller in the
tree. The walk dropped its alias every step, so the eighth read answered `AliasCapacity`, the
`.ok()` turned that into `None`, and the arm read it as "this window has no document": the chrome
advanced, the page ingress froze at 6 of 30, and every later frame failed on its first read. The
panel walk had the identical shape.

**Fix** — the chrome step LENDS the one owner instead of aliasing it:
`ShellState::take_window_document` / `restore_window_document` (and `panel_documents.remove` /
`.insert` for a tab) move the exact `UiDocumentLease` out for the step and put it back. An owner move
mints no credit, so it cannot run out. `try_alias` no longer appears in the shell outside its own
post-mortem docstring.

### 3.2 The browser frame build advanced ONE step per two frames

Same census: consecutive chrome steps were **33 ms apart** while frames arrived every 16 ms. Two
causes, both in the browser driver:

* `ActiveFrameBuild::step` ran exactly one `advance()` and returned `Yield`, ignoring the whole
  `StepBudget` `batch_params` mints for it (`INTERACTIVE_LANE_FUEL` fuel, `INTERACTIVE_LANE_WALL_US`
  wall).
* `FrameBuildHandle::poll_runtime_and_resubmit` (wasm32) ran ONE session phase per call, and a job
  step is three phases — `Idle → try_step_on_caller`, `Outcome → take/resume`, then `Idle` again.

A retained window body ingests one node page per `advance`, and a boot chrome pass is hundreds of
them across seven surfaces, so the first frame could not converge at all.

**Fix** — `ActiveFrameBuild::step` loops `advance()` until its own `cx.should_yield()`, and the wasm
driver loops the session's phases until the build finishes or `BROWSER_FRAME_BUILD_DRIVE_US` is
spent. That constant is `semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2` — derived from the one
ratified ceiling, with the other half left for the present half, rather than chosen. Every `advance`
still carries its own `os_renderer.frame.*` watchdog and quarantine admission, so the loop cannot
hide an over-ceiling phase.

Measured immediately after (`run-4`): **512 job steps per 128 driver calls** (was 128), and the
retained-document census went from 24 steps in 60 s to **3 432 448**.

### 3.3 A transient ingress refusal was a TERMINAL fault, and the chrome then blocked on it forever

With the throughput fixed, `run-4` faulted on the very first document step and never moved again:

```
[DEBUG] ui-doc-census steps=1 window=procedural-main phase=fault begin-failed gen=1 fault=Deadline
[DEBUG] ui-doc-census steps=3432448 window=procedural-main phase=fault
```

`begin_document`, `apply_document_page` and `finish_document` all refuse a step whose `StepContext`
is cancelled or out of budget, and `render_ui_document_step` mapped **every** refusal to
`UiDocumentFramePhase::Fault`. One `Deadline` — an ordinary budget yield — froze the surface's page
ingress for the life of the shell. Worse, `Fault` is a terminal phase that does nothing and returns
`false`, and the chrome arm read `false` as "come back next opportunity", which for a terminal cursor
is never: the whole frame transaction spun on one unreadable body.

**Fix**, two halves:

* the budget is checked ONCE at the top of the Ingress arm, so an opportunity that cannot pay is
  simply not spent (`UiDocumentFramePhase::Ingress if step.is_cancelled() || step.should_yield()`),
  and `Cancelled` / `Deadline` / `InterruptedClose` / `ValidationPending` are named retries in both
  `begin_document` and `finish_document`. Only `StaleGeneration`, `Invalid(_)` and a failed page read
  stay terminal.
* a cursor that DOES reach its terminal fault releases the chrome step:
  `cursor.document.terminal_is_fault()` (an accessor that already existed and had no caller) lets the
  walk advance and records the surface's own fault card through `record_document_paint_fault`. A
  successful paint clears it again.

### 3.4 The effect-storm budget was charged per STEP, not per round

`run-6`, the first boot to reach the post-intent tail:

```
worker-step-overrun: frame effect storm budget exhausted   ·   Surface: quarantined
```

`FrameTransaction::step` charged `effect_opportunities` on **every** step taken while the stage was
`FlushEffects` — but that stage covers the whole tail (`FrameDeferred`, `BoardAuthority`,
`World3dSnapshot`, the prepared packet), which legitimately needs far more than the 64 one-cursor
steps `EFFECT_STORM_BUDGET` allows. It stayed invisible only while the wgpu frame never reached the
stage at all.

**Fix** — the charge moved to the ONE transition into the stage, so the budget bounds effect-flush
*rounds*, which is what its name and its doc say.

### 3.5 A superseded frame opportunity quarantined the surface instead of rebuilding

`run-7`, the next boot:

```
worker-step-overrun: frame opportunity base revision was superseded   ·   Surface: quarantined
```

`FrameTransaction::step`'s freshness guard answered `AppFrameTransactionStep::Fault` **and**
`runtime.record_frame_fault(...)` for six conditions — a rebound operation or generation, a
cancelled context, an exceeded deadline, a superseded input generation, a superseded base revision.
None of those is a defect. The scene revision is bumped by **every** runtime completion
(`enqueue_runtime_completion`, `RuntimeApply::finish`), so a frame build long enough to ingest a
retained document is superseded as a matter of course — and the browser worker turns a recorded frame
fault into a surface quarantine.

**Fix** — a new `AppFrameTransactionStep::Superseded` ends that build with no frame and no fault; the
frame job retires it to `ActiveFrameStep::Complete(None)` and the caller's next opportunity admits a
fresh build against the current witness.

### 3.6 (same family) The browser never admitted a SECOND frame build, and one presentation cost seconds

Two further single-line gates, both measured by a temporary redraw/drive census:

* `poll_runtime_and_resubmit` admitted a build only when `generation` differed from the last admitted
  one. On the browser `OsHost::frame_generation` is **overwritten by the transport generation** on
  every `enqueue_batch` (`🌐️browser-worker/🦀️.rs:310`), so it is pinned for the life of an idle shell
  — measured constant at `2`. One build was admitted, ever. `self.session` is the authority that gate
  was standing in for, so the gate is gone.
* `present_step()` ran one phase of the present cursor per redraw; a single presentation took ~2.3 s,
  and `admit_next_frame` refuses to build while one is pending. `build_and_publish_snapshot` now
  drives the present cursor for the remaining half of the interactive ceiling, with every prepared GPU
  opportunity still priced by its own 2 ms ceiling (`admit_prepared_gpu_opportunity`, the §4.4 law of
  `📓️wgpu-playground-boot-2026-09-12.md`).

---

## 4. What the shell does now (run-9, run-10)

`run-9` — the retained document completes, for the first time on this target:

```
ui-doc steps=3   pending page=0/30      (t = 4 631 ms)
ui-doc steps=33  pending page=30/30
ui-doc steps=94  phase=viewport          ← finish_document published
ui-doc steps=95  phase=layout
ui-doc steps=96  phase=paint  layout=Idle
ui-doc steps=97  phase=complete paint=Missing
```

Thirty pages, header validation and the whole record walk in **~15 ms**, then Viewport → Layout →
Paint, and from there a steady 3-step re-paint cycle per surface per frame.

`run-10` — the final build, censuses removed:

```
sent      { boot: 1, shard-port: 4, activate: 1, turn: 89, batch: 3465, introspect: 112 }
received  { booted: 1, wake: 296, frame: 3465, … }
requestFrameTrue 3465   alert null            (no fault banner, no quarantine, 60 s)
dumpFrameStats { windowId: "procedural-main", drawCalls: 0, quadCount: 0, glyphCount: 0 }
```

`wake: 296` (was `2`) is the runtime actually completing work behind the frames. Boot is unchanged at
**4.3 s** to `runtime-ready` and the seven surfaces still render in 3.0 s.

**`drawCalls` is still 0 — see §5. This lane does NOT claim a painted canvas.**

---

## 5. The remaining blocker — the published document is never reconciled into the paintable tree

`run-9` step 97 names it exactly: `paint=Missing`. `Ui::frame_into_step` answers `Missing` for only
two reasons, and the window exists, so:

```rust
// 🌲️tree.rs:390
pub fn publish_document(&mut self, document: UiDocumentTree) -> Option<UiDocumentTree> {
    self.document.replace(document)
}
```

`UiTree.document` is a **write-only field**. Its only readers in the whole tree are
`Ui::document_status` (a generation comparison) and `UiTree::take_document` (retirement). Nothing
ever walks the published `UiDocumentTree`'s `UiNodeRecord`s into the arena, so `UiTree::root` stays
`None`, `frame_into_step` returns `Missing` on the first line it checks, the draw list stays empty and
`dumpStructure` reports 0 nodes.

The only writer of the arena is `Ui::apply_tree` → `UiTree::apply_tree(&UiNode)` — and it is
`#[cfg(any(test, feature = "testkit"))]`. **In production the wgpu engine has no path at all from a
retained document to a paintable tree**, which is why this target has never painted a window body.

What is missing, precisely:

| piece | state |
|---|---|
| `UiNodeRecord { key, component, layout, style, activity, disabled, accessibility, bindings, menu, children }` → `Node { key: NodeKey, spec: WidgetSpec(UiNode), … }` | **absent** — there is no `Component` → `UiNode` projection anywhere under `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/` |
| a stepped reconcile that diffs the published document against the live arena (the document half of `🔁️reconcile.rs`'s `apply_tree`/`diff_and_update`/`reconcile_children`) | **absent** |
| the call site: `Ui::finish_document` after `publish_document`, or a new `UiDocumentFramePhase` between `Ingress` and `Viewport` | **absent** — `finish_document` already marks layout dirty `if let Some(root) = window.tree.root`, i.e. it is written expecting a root that nothing sets |

That is a real engine feature (a per-record, budget-stepped reconcile with its own retirement ladder),
not a fix, and it is the whole of what stands between this target and a painted shell. Everything
upstream of it is now demonstrably healthy on 6118.

---

## 6. Defect B — no effects on wgpu

The shell's crossing is correct. A temporary `[DEBUG] contributions slim view` line (removed) shows
what the guest is actually handed:

```
keys: ["activeModeId","activeWindowKindId","locale","terminology","focusedWindowId","windowInstances"]
windowInstances: [ {id:"procedural-main", windowKindId:"procedural-main"},
                   {id:"procedural-preview", windowKindId:"procedural-preview"} ]
```

`GENERATION_3D_PLAY_WINDOW_PREVIEW` **is** `"procedural-preview"`, so
`generation3d_preview_windows` finds the attached preview, and the wgpu bridge's
`pushScopedContributions` already re-arms every `dispatchAction` the answer carries (its
`contributions rearm` branch). The guest still answers `effects: 0`, so the re-arm is refused inside
the guest:

```rust
// set_contributions::apply
if !session.invalidate_for_flow_extension_registry(generation) { return Ok(Emit::default()); }
Ok(Emit { effects: crate::preview_eval::rearm_attached_previews(session, preview_windows), .. })
```

— either the registry generation was already observed, or `FlowEvalSession::arm_owed_window_tick`
believes a tick is still outstanding for that window. The second is the likely one and has a named
wgpu cause: **`settleInstanceLifecycle` in `🐚️plugin-bridge.ts` discards every lifecycle turn's
`effects`** (it calls `route.accept(current, execute)` and re-reads `current` without ever collecting
`current.effects`). A `flowEvalTick` armed while the instance opens is therefore dropped by the host
while the guest's latch stays armed, and nothing can ever re-arm it. That is the next thing to
instrument — a `[DEBUG]` of `current.effects` inside `settleInstanceLifecycle` answers it in one boot.
Not taken on here: without §5 there is no painted preview for a mesh to land in.

---

## 7. Files

**New**
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📃️document-lease-owner-move/🔣️.json` — the neutral oracle:
  alias capacity and the read at which a dropped alias is refused, the owner-move read count, the
  retryable-vs-terminal ingress table, the six superseded conditions, the storm budget's charge unit,
  and which cursor phases release the chrome step.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📃️document-lease-owner-move/🦀️.rs` — the Rust law, against the
  **live** `UI_DOCUMENT_ARENA`: seven aliased reads are admitted and the eighth answers
  `AliasCapacity`; 3 600 owner-move reads never refuse; a retired alias returns its credit without
  retiring the published owner. Registered in `🧬️contract/📦️packages/🦀️rust/📃️document.rs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖌️wgpu-document-owner-move/{🟦️.ts,laws.json}`
  — the TypeScript twin on the SAME fixture: the chrome walk moves the owner and never names
  `try_alias`, a terminal cursor releases the step and every other phase holds, every refusable
  ingress fault is classified explicitly, no freshness condition records a renderer fault, the storm
  budget is charged on the stage transition, and both drive loops name the ratified ceiling.
  Registered in the wgpu `vitest.config.ts`.
- `<ticket>/🐍️wgpu-paint-probe.mjs` — the crossing-counting probe (§2).

**Changed**
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `take_window_document`/`restore_window_document`, the
  panel owner move, the terminal-cursor release, `record_document_paint_fault`/`clear_…`/
  `document_body_key`, and `record_surface_fault` now logs through `debug_log` (`eprintln!` is a
  no-op in a `wasm32-unknown-unknown` Worker, so a recorded surface fault left no trace at all).
- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — the ingress budget check and the
  retryable-refusal classification.
- `🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` — `ActiveFrameBuild::step`'s budget loop, the wasm driver loop,
  `BROWSER_FRAME_BUILD_DRIVE_US`, unconditional admission, the `Superseded` arm.
- `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `AppFrameTransactionStep::Superseded`, the freshness guard,
  the per-round effect-storm charge.
- `🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` — the present drive loop.
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` — declares `ui_styling`, which a peer's
  `⚙️EngineCanvas` edit (`ui_styling::metrics::label::LEGIBLE_MIN_PX`) had already started using;
  without it the crate did not compile at all. Their change completed, not reverted.
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts` — registers the new twin suite.

No `launch.json` entry was added (`procedural3d-wgpu` already exists and is the row this lane ran).

---

## 8. Tests actually run, verbatim

```
$ cargo test -p semio-framework-ui-contract --lib -- document_lease_owner_move
running 1 test
test document::document_lease_owner_move_tests::a_per_frame_read_moves_the_exact_owner_because_an_alias_credit_runs_out ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 168 filtered out

$ bunx vitest run --config vitest.config.ts 🧪️tests/🔢️wgpu-u64-seam/🟦️.ts 🧪️tests/🧩️wgpu-module-routes/🟦️.ts \
      🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts 🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts
 Test Files  4 passed (4)
      Tests  26 passed (26)

$ cargo test -p semio-framework-ui --features wgpu-engine --lib -- prepared_present
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 383 filtered out

$ cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --offline
warning: `semio-framework-os-renderer-wgpu` (lib) generated 24 warnings
    Finished `dev` profile [unoptimized] target(s)
```

### 8.1 Observed, not mine

* `cargo test -p semio-framework-ui-contract --lib` (the WHOLE suite) aborts with
  `panic in a destructor during cleanup` inside
  `document_component_compare_tests::…cancel_and_contention_keep_live_document_and_incoming_root`.
  It aborts identically with `--skip document_lease_owner_move`, i.e. with this lane's test not run at
  all, and that suite passes on its own with `--test-threads=1`. Pre-existing arena-contention
  flakiness of that suite, not this change.
* Peers broke the shared tree twice during the session — `wgpuActivatedModulesRoot` removed from
  `🧑‍💻dev/♻️activation/🟦️.ts` while `🧊️wgpu/…/📜️script.ts` still imported it (they fixed it within a
  minute), and a `semio-s-artifact-puzzle-{3d,5d}` refactor that failed `cargo check` for ~8 minutes.
  Both waited out, neither touched.

---

## 9. Rebuilds and the serve

The renderer wasm was rebuilt **six** times (each Rust change), through the ticket's own
`📜️serve-generation3d-wgpu.sh` under `screen -dmS g3dwgpu` — kill the trunk pid, restart the screen.
`🎞️frame-worker.js` was regenerated **once**, for the `[DEBUG] contributions slim view` line in
`🐚️plugin-bridge.ts` (`bun ./📜️script.ts generate-frame-worker`, which writes the artifact and only
then throws on the peer's `localstorage` census — the route-around
`📓️wgpu-playground-boot-2026-09-12.md` §5 documents). The final serve is
`trunk` on 6118, artifact `semio-framework-os-renderer-wgpu_bg.wasm` 77 401 188 B.

Shared cargo build dir throughout, `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`. No
git-state-modifying command, no worktree. The react serve on 6018 was not touched, the procedural
plugin wasm was not restaged, and no `🗑️generated` folder was swept.

## 10. Re-adding the censuses

Both temporary censuses were thread-local counters that printed the first N events and then one line
every 4096: `ShellState::debug_chrome_census(cursor)` at the top of `render_chrome_step` (phase, child
phase, item, document phase) and `debug_document_census(window_id, phase, note)` at the end of
`render_ui_document_step` (with a `note` set in each ingress branch). A third,
`debug_drive_census(event, generation)` in `FrameBuildHandle::poll_runtime_and_resubmit`, counted
calls / stale generations / admits / phases, and a fourth printed
`redraws/generation/frameReady/pendingPresentation` from `redraw_core`. Between them they localise any
future frame-path stall to one phase in one boot; they are cheap to re-add and must not ship.
