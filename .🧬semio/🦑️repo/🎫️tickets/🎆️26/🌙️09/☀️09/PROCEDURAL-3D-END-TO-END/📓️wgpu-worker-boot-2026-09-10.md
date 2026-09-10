# wgpu frame-Worker boot steps — `worker-boot-step-overrun` is a diagnostic, never a verdict

Lane: wgpu frame-Worker boot-step budgets.
Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`.

Live finding this lane answers (wgpu boot #3, 09:35, `http://localhost:6118/?plugin=generation3d`,
hidden pane 1440×900, box load ~20):

```
plugin-graph 25% — wgpu renderer fault: worker-boot-failed: worker-boot-step-overrun:
                   plugin-graph took 8.300 ms against a 8 ms budget
                   — Surface: faulted; UI-thread frames: unavailable — the canvas belongs to the frame
                   Worker (OffscreenCanvas transferred); Worker terminated: yes · input accepted: no;
                   UI turns over budget (recorded/sustained): 0/0
```

The previous lane (`📓️wgpu-ui-turn-2026-09-10.md`) taught the **UI isolate** that a single wall sample is
not a verdict. The fault then moved one step earlier, into the **frame Worker**, which had never learned it:
`ownedStep` still compared one `performance.now()` delta against 8 ms and **threw**, and every throw inside
`boot()` is `worker-boot-failed` — Worker terminated, canvas gone, input dead.

`UI turns over budget (recorded/sustained): 0/0` in that very banner is the proof: the UI isolate's own
ledger, measuring executing spans, recorded **zero** overruns during the same boot. Only the Worker's raw
wall clock saw 8.3 ms.

## 1. Verdict inventory — every worker step / frame budget in the wgpu path (before this lane)

All line numbers as of this lane's start.

| # | site | budget declaration | clock | measured span | verdict on breach |
|---|---|---|---|---|---|
| 1 | `🎞️frame-worker/🟦️.ts:86` `ownedStep` | `WORKER_STEP_BUDGET_MS = 8` (`:49`) | `performance.now()` — **wall** | one synchronous owned step | **`throw worker-boot-step-overrun`** |
| 2 | `🎞️frame-worker/🟦️.ts:322` `plugin-graph` | default 8 ms via #1 | wall | `resolvePlaygroundBoot(PLUGIN_CATALOG, variant)` | as #1 → `fault("worker-boot-failed")` → **Worker closed** — *the live fault* |
| 3 | `🎞️frame-worker/🟦️.ts:293` `plugin-handle:<id>` | default 8 ms via #1 | wall | `pluginHandleForBridge(module)`, ×11 | as #1, but caught per plugin → `plugin-fault:` progress row |
| 4 | `🎞️frame-worker/🟦️.ts:267,399` `asset-abort` | default 8 ms via #1 | wall | `runtime.abortAssetResponse()` | as #1 → `asset-abort-fault` / swallowed |
| 5 | `🎞️frame-worker/🟦️.ts:367,375,378,379,382,389,392,394` `asset-*` | default 8 ms via #1 | wall | one asset pump step each | as #1 → `fault("asset-stream-fault")` |
| 6 | `🎞️frame-worker/🟦️.ts:317,332,341,342` `runtime-environment` / `renderer-bootstrap` / `renderer-finish` / `interactive-job-registry` | `BROWSER_OWNED_SUSPENSION_BUDGET_MS = 30_000` (`:84`) | wall | one Rust bootstrap phase | as #1 (30 s ceiling — a wedge tripwire, not an interactive budget) |
| 7 | `🎞️frame-worker/🟦️.ts:94` `monitoredSuspension` `:start` | caller's `blockBudgetMs` | wall | the synchronous prologue of an async op | as #1 |
| 8 | `🎞️frame-worker/🟦️.ts:111` `monitoredSuspension` block watch | same | 2 ms heartbeat gap — wall | longest gap the Worker did not run | `throw worker-boot-step-overrun: … blocked the Worker for …` |
| 9 | `🎞️frame-worker/🟦️.ts:185` interactive-job admission | `WORKER_STEP_BUDGET_MS` | wall | `interactiveJobs.receive(message)` | `fault("interactive-job-overrun")` → **Worker closed** |
| 10 | `🎞️frame-worker/🟦️.ts:198` frame step | `WORKER_STEP_BUDGET_MS` | wall | `enqueueBatch` + `tick` | `quarantined = …` → `requestFault(...)` → **Worker closed** |
| 11 | `🎞️frame-worker/🟦️.ts:245` close turn | `WORKER_STEP_BUDGET_MS` | wall | one `closeStep()` pair | `pendingFault = worker-close-overrun` |
| 12 | `🎞️frame-worker/🟦️.ts:227` introspection | `INTROSPECTION_STEP_BUDGET_MS = 64` (`:71`) | wall | `dumpStructure`/`dumpFrameStats` | reported on the answer — **already correct** |
| 13 | `🚚️browser-frame-transport/🟦️.ts:559` worker frame reply | hardcoded `8` | worker-side wall | one worker frame step | `quarantine("worker-step-overrun")` — surface degraded, input still admitted |
| 14 | `🌐️browser-worker/🦀️.rs:585` `BrowserRendererBootstrap::step` | none | none | one Rust bootstrap phase (`font-atlas` … `runtime-ready`) | none — but the phase is unmeasured, so a long one is invisible |
| 15 | `🧵️frame-job/🦀️.rs:250,289,310,333` | `INTERACTIVE_STEP_CEILING_US` | `Watchdog` — wall | one frame-job step | `StepOverrunLedger::admit(...)` — quarantine only after 4 consecutive — **already correct** |
| 16 | `⏱️trace/🦀️.rs:601` `StepQuarantine::is_terminal` | `SUSTAINED_OVERRUN_QUARANTINE_STEPS = 4` | — | — | the ratified law |

Rows 1–11 and 13 are the defect: **single wall samples that are verdicts**, nine of which terminate the
Worker. Rows 15–16 are the law the Rust side already ratified and the Worker isolate never learned —
exactly the shape the UI isolate had before the previous lane.

## 2. Root cause

Two defects, both in the frame Worker's TypeScript. Neither is in the renderer, the boot plan, or the
plugin graph.

### 2.1 A single WALL sample was a verdict — and `plugin-graph` is 187 µs of work

`ownedStep` read `performance.now()` twice around a synchronous callback and threw when the delta reached
the ceiling. A Worker has no per-thread CPU clock (`performance.now()` is wall time), a hidden pane is
background-throttled, and the box was scheduling ~20 other WASM workers — so that delta cannot separate
the step's own work from the isolate not running.

Measured natively (`🧪️wboot-measure.ts`, variant `generation3d`, 59 catalog rows → 11-plugin closure,
median of 9, this same machine):

| step | median | min | max |
|---|---|---|---|
| `plugin-graph` — whole `resolvePlaygroundBoot` | **187 µs** | 107 µs | 878 µs |
| ↳ catalog-row projection (`moduleUrl` ×59) | 46 µs | 39 µs | 63 µs |
| ↳ dependency closure (`expandPluginRegistry`) | 10 µs | 8 µs | 33 µs |
| ↳ activation order (`orderPluginRegistryEntries`) | 18 µs | 15 µs | 40 µs |

The live reading was **8.300 ms** — a **44×** scheduling artefact over the median. No plausible chunking
would have saved that boot, because the step was never the cost; the throw was. That is exactly the
attribution `⏱️trace/🦀️.rs` had already settled:

> a single over-ceiling wall reading is as often the machine descheduling the thread as it is the step's
> own work

and the banner printed the proof next to the fault: `UI turns over budget (recorded/sustained): 0/0` —
the UI isolate's executing-span ledger recorded **nothing** across the same boot.

### 2.2 Every budget breach in the Worker was terminal

`boot()` wraps everything in one `try/catch` whose `catch` is `fault("worker-boot-failed", …)`, and
`fault` → `requestFault` → `beginClose` → `scope.close()`. So a budget throw anywhere in the boot did not
degrade the surface — it destroyed it, together with the transferred `OffscreenCanvas`, which is the
surface's ONLY frame path after `transferControlToOffscreen()`. The same shape applied to
`interactive-job-overrun` (a slow admission killed the Worker), the frame step (`duration >= 8` →
`requestFault`), and `worker-close-overrun`. `monitoredSuspension` went further and threw on
`maximumBlockMs` — a measurement whose entire content is "this Worker did not run", i.e. the descheduling
signal itself, promoted to a fatal verdict.

## 3. Fix

| # | file | change |
|---|---|---|
| 1 | `🎯️targets/🧊️wgpu/⏱️ui-turn-budget/` → **`⏱️turn-budget/`** | the module was already the browser's turn law; it now says so. Both isolates are priced here: `UI_TURN_BUDGET_MS = 2` (UI thread) and the **new** `WORKER_STEP_BUDGET_MS = 8` (frame Worker), with ONE `TurnClock` (executing spans) and ONE `TurnLedger` (sustained-run attribution) serving both. `TurnLedger` gained a `budgetMs` ceiling and a `scope` trace label; the identifiers lost their `Ui` prefix (`UiTurnClock`→`TurnClock`, `UiTurnLedger`→`TurnLedger`, `SUSTAINED_UI_TURN_OVERRUN_TURNS`→`SUSTAINED_TURN_OVERRUN_TURNS`, `UI_TURN_DIAGNOSTICS_KEY`→`TURN_DIAGNOSTICS_KEY`, `setUiTurnDiagnostics`→`setTurnDiagnostics`, `uiTurnDiagnosticsEnabled`→`turnDiagnosticsEnabled`). **The module still owns no fault path at all.** |
| 2 | `🎞️frame-worker/🟦️.ts` | `ownedStep` runs inside `stepClock` and admits to `stepLedger`; it **cannot throw on a budget** — only the callback's own throw propagates. `WORKER_STEP_BUDGET_MS` is imported, not redeclared. `worker-boot-step-overrun`, `interactive-job-overrun` and `worker-close-overrun` are **deleted** as codes. |
| 3 | `🎞️frame-worker/🟦️.ts` | `monitoredSuspension` takes a `TurnLedger` instead of a raw ceiling and admits `maximumBlockMs` as a `<stage>:block` observation on the 30 s `suspensionLedger` — the "this Worker did not run" reading is recorded, never thrown. `stepClock.suspend()/resume()` bracket every await, so a boot step parked across an `await` accrues nothing. |
| 4 | `🎞️frame-worker/🟦️.ts` | new `driveChunks(unit, base, span)`: drives any `ResumableBootUnit` one chunk per owned step, reports each chunk as boot progress, and **yields to a macrotask between chunks whenever `stepLedger.degraded()`** — the yield-and-continue cadence, latched by a sustained run and cleared by the first admitted step. |
| 5 | `🎠️kernel/🟦️.ts` | new `PlaygroundBootPlanner` — the plugin graph as a **resumable** unit: `PLUGIN_GRAPH_CHUNK_ROWS = 16` catalog rows per chunk, then `closure`, then `order`. `resolvePlaygroundBoot` is now that planner driven to completion in one turn, so there is exactly **one** implementation of the graph and no drift is possible. |
| 6 | `🎞️frame-worker/🟦️.ts` | the frame step is an owned step too. `duration >= 8 → requestFault` is gone: a **sustained** run (4 consecutive executing overruns) reports `quarantined: true` with `worker-step-overrun`, which degrades the surface while the Worker keeps running and input stays admitted; only a renderer-declared quarantine still raises a fault. |
| 7 | `🎞️frame-worker/🟦️.ts` + `🚚️browser-frame-transport/🟦️.ts` | `boot-progress` now carries the Worker's own `BrowserFrameWorkerStepReport` (degraded, recorded/sustained, worst step + site), and the frame reply carries `workerExecutingMs` + `workerStepVerdict`. The transport's hardcoded `message.workerDurationMs >= 8` verdict is **deleted**; a degraded Worker report also puts the progress hook on deferred cadence. |
| 8 | `🚀️browser-boot/🟦️.ts` | the boot status line appends `· deferred cadence` / `· verzögerte Taktung` and stamps `status.dataset.workerDegraded` / `workerStepOverruns`; the fault banner gained a `Worker steps over budget (recorded/sustained)` line in **en and de**, so a reader can tell WHICH isolate ran long. |
| 9 | `🌐️browser-worker/🦀️.rs` | every Rust bootstrap phase now reports `elapsedUs` — measured with `performance.now()` inside the Worker (`worker_now_ms`, new `WorkerGlobalScope` web-sys feature) because `js_sys::Date::now()` is millisecond-resolution wall time. The phase arms build a `BootPhase` and the driver stamps the cost in one place. The JS driver logs `[DEBUG] renderer-bootstrap stage=… executing=…ms phaseUs=…`. |

### Why `shell-construct` is not chunked further

`shell-construct` is Rust phase 5 — `ShellState::new` (one struct literal) plus `load_persisted_panel_layout()`.
It is **already** an isolated chunk: the JS driver awaits a macrotask before every phase, so each phase is
one turn of the Worker's event loop. It is priced against the 30 s browser-owned `suspensionLedger`, not the
8 ms interactive one, and after this lane no ceiling can end it. Splitting the struct literal itself would
mean restructuring a 165-field construction used by 20+ native call sites for no measured gain; the honest
fix is the `elapsedUs` reading that makes its real cost visible for the first time.

### What "degrades gracefully" means concretely, in the Worker

* An **isolated** breach: recorded, traced under `SEMIO_RUNTIME_DIAGNOSTICS`, work continues inline.
* A **sustained** run (4 in a row): `driveChunks` hands the isolate back to its event loop between chunks,
  `boot-progress` reports `degraded: true`, and the UI isolate defers the progress hook. The boot completes.
  The latch clears on the first step that fits the ceiling, so the Worker recovers on its own.
* A **clock fault**: recorded as such and ignored.
* A step that **throws**: still fatal. That is a defect, not a budget.

## 4. Verdict inventory — after

| # | site | budget | clock | verdict on breach |
|---|---|---|---|---|
| 1 | `ownedStep` | `WORKER_STEP_BUDGET_MS = 8` (imported) | `TurnClock` — **executing** spans | record → `stepLedger`; **never** throws, closes, or faults |
| 2 | `plugin-graph` | same | same | now **6 chunks** (`rows ×4`, `closure`, `order`) via `driveChunks` + `PlaygroundBootPlanner`; a sustained run yields between chunks |
| 3 | `plugin-handle:<id>` ×11 | same | same | record; a throw is still isolated per plugin as a `plugin-fault:` progress row |
| 4–5 | `asset-*` steps, `asset-abort` | same | same | record; only a throw is `asset-stream-fault` |
| 6 | `runtime-environment` / `renderer-bootstrap` / `renderer-finish` / `interactive-job-registry` | `BROWSER_OWNED_SUSPENSION_BUDGET_MS = 30_000` | same, `suspensionLedger` | record; never degrades the interactive cadence |
| 7 | `monitoredSuspension` `<stage>:start` | caller's ledger | same | record |
| 8 | `monitoredSuspension` `<stage>:block` | `suspensionLedger` | 2 ms heartbeat gap | **record only** — this reading is the descheduling signal itself |
| 9 | `interactive-job:<kind>` admission | `WORKER_STEP_BUDGET_MS` | `TurnClock` | record; the code `interactive-job-overrun` is deleted |
| 10 | `frame-step` | same | same | record; **sustained** (4 consecutive) → `quarantined: true` `worker-step-overrun` on the reply, Worker **stays alive**, input stays admitted; only a renderer-declared quarantine faults |
| 11 | `close-step` | same | same | record; the code `worker-close-overrun` is deleted |
| 12 | introspection | `INTROSPECTION_STEP_BUDGET_MS = 64` | wall | reported on the answer — unchanged, already correct |
| 13 | transport frame reply | — | — | the hardcoded `workerDurationMs >= 8` verdict is **deleted**; the transport reads `workerStepVerdict` |
| 14 | `🌐️browser-worker/🦀️.rs` phases | — | `performance.now()` in the Worker | reports `elapsedUs`; each phase is one macrotask-separated chunk |
| 15–16 | `🧵️frame-job/🦀️.rs` + `StepOverrunLedger` | unchanged | | unchanged — the law all three isolates now share |

The only remaining way to end a boot is a **throw**, plus the UI isolate's `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS`
(60 s of SILENCE, re-armed by `boot-liveness` and by every `boot-progress` chunk — and the chunked
`plugin-graph` now re-arms it six times where it used to re-arm it once).

## 5. Measurements — µs per step, before and after

`🧪️wboot-measure.ts` (in this ticket folder), variant `generation3d`, 59 catalog rows → 11-plugin closure,
this machine. Raw: `🗑️generated/wboot-measure-after.txt`.

**Before — one step, one wall sample, fatal at 8 ms**

| step | median | min | max |
|---|---|---|---|
| `plugin-graph` (whole) | 187 µs | 107 µs | 878 µs |
| live reading in a hidden pane at load ~20 | — | — | **8 300 µs → Worker terminated** |

**After — six chunks, executing-time priced, non-fatal**

| chunk | worst of 15 |
|---|---|
| `plugin-graph:rows 16/59` | 37 µs |
| `plugin-graph:rows 32/59` | 103 µs |
| `plugin-graph:rows 48/59` | 45 µs |
| `plugin-graph:rows 59/59` | 39 µs |
| `plugin-graph:closure` | 95 µs |
| `plugin-graph:order` | 71 µs |
| **worst single chunk** | **103 µs — 78× under the 8 ms ceiling** |
| whole-plan equivalence | `planner == resolvePlaygroundBoot` → identical 11 plugin ids, defaultAppId, dependencyErrors |

The second run's numbers are *worse* than the first (128 µs median vs 187 µs, 103 µs worst chunk vs 42 µs)
because the box load moved between runs — which is the whole point: on this target the reading is dominated
by the machine, not by the step, and no ceiling read off one wall sample can tell the two apart.

Rust bootstrap phases now report their own `elapsedUs`; the JS driver logs
`[DEBUG] renderer-bootstrap stage=<stage> executing=<ms> phaseUs=<µs>` per phase, so the next live boot
publishes per-phase costs for `font-atlas` … `runtime-ready` for the first time.

## 6. Tests

New file `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts`,
registered in `📦️packages/🦀️rust/vitest.config.ts` and in `BrowserWorkerTestScript` (`📜️script.ts`).

| test | drives |
|---|---|
| pins the Worker ceiling and the shared attribution law | `WORKER_STEP_BUDGET_MS === 8`, `SUSTAINED_TURN_OVERRUN_TURNS === 4`, and asserts the frame Worker source contains **no** `worker-boot-step-overrun`, `interactive-job-overrun`, `worker-close-overrun`, and no `>= WORKER_STEP_BUDGET_MS` comparison at all |
| **completes a boot step that spins 50 ms of EXECUTING time** | the required scenario: 5 REAL 50 ms busy-spins driven exactly as `driveChunks` drives one — every step **completes**, `recordedOverruns === 5`, at least one `sustained-overrun`, the driver **yields** ≥2 times, and one cheap step afterwards clears the latch (`degraded() === false`) |
| **completes a boot step suspended 500 ms of WALL time with nothing recorded** | the required scenario: REAL `setTimeout(500)` between `suspend()`/`resume()` — wall ≥ 450 ms, verdict `admitted`, executing < 8 ms, `recordedOverruns === 0` |
| chunks the plugin graph so no chunk needs the ceiling | drives `PlaygroundBootPlanner` chunk by chunk: ≥3 chunks, the `rows`/`closure`/`order` stages all present, **every** chunk under `WORKER_STEP_BUDGET_MS`, the rows chunk count equals `ceil(catalogRows / PLUGIN_GRAPH_CHUNK_ROWS)`, and the plan equals `resolvePlaygroundBoot`'s (ids, defaultAppId, dependencyErrors) — the third-party-free equivalence oracle for the resumable rewrite |
| reports what every Rust bootstrap phase executed for | pins `elapsed_us` + `worker_now_ms` in `🌐️browser-worker/🦀️.rs`, all 8 phase stage ids, and the JS driver's `phaseUs=` trace |

Updated: `🧪️tests/📨️browser-frame-transport/🟦️.ts` (frame replies carry `workerExecutingMs`/`workerStepVerdict`;
the quarantine case is now a `sustained-overrun` reply, not a raw `workerDurationMs >= 8`; the worker-source
pin follows `resolvePlaygroundBoot` → `new PlaygroundBootPlanner`) and `🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts`
(`boot-progress` carries the Worker's step report; identifiers follow the `⏱️turn-budget` rename).

## 7. Verification

| gate | result |
|---|---|
| `bun ./📜️script.ts test` (vitest part) | **5 files, 71 tests passed** (`📨️browser-frame-transport`, `🎮️browser-interactive-job-port`, `⏱️wgpu-ui-turn-budget`, **`⏱️wgpu-worker-step-budget`**, `🧩️package-integration`) |
| `bun ./📜️script.ts test-browser-worker` | **4 files, 51 tests passed**, exit 0 |
| `bun ./📜️script.ts check-frame-worker` | `🎞️frame-worker.js is fresh` |
| `bun ./📜️script.ts check-browser-worker` | exit 0 (carrier census + generated-bytes identity) |
| `tsc -p tsconfig.json --noEmit` | **0 errors** in every file this lane touched (`🎞️frame-worker`, `🚀️browser-boot`, `🚚️browser-frame-transport`, `🔌️browser-interactive-job-port`, `⏱️turn-budget`, `🎠️kernel`, both test suites) |
| `cargo check -p semio-framework-os-renderer-wgpu --keep-going` (native) | **0 errors**; `semio-framework-os-renderer-wgpu (lib) generated 18 warnings`, `Finished dev profile in 45.99s` — raw `🗑️generated/wboot-cargo-native.txt` |
| `cargo check … --target wasm32-unknown-unknown --keep-going` | **0 errors**; `… generated 14 warnings`, `Finished dev profile in 1m 23s` — raw `🗑️generated/wboot-cargo-wasm.txt`. This is the authoritative gate for the Rust change: `mod browser_worker` is `#[cfg(target_arch = "wasm32")]` and never compiles natively |
| `bun ./📜️script.ts test` (cargo-nextest part) | 4 pre-existing failures in `async_boundary_tests` (`native_binary_owns_exactly_one_entrypoint_driver`, `presenter_ack_retirement_source_mutations_are_denied`, `raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete`, `renderer_asset_probe_…` SIGABRT). All four are **source-contract assertions** over `BINARY_SOURCE` / `GPU_SOURCE` / `DRAW_SOURCE` / `♾️infinite/🌍️world/🦀️.rs` — files this lane never touched, all of them dirty in `git status` from a concurrent lane. Not caused by, and not fixable from, this lane |

### Peer state seen during this lane

The first native `cargo check` failed on `semio-framework-plugin-host` with
`E0004: non-exhaustive patterns: &GuestInstanceState::Owned(_) not covered` — the **sibling guest-memory
lane** mid-refactor, which blocks the wgpu crate natively (it depends on the host off-wasm only). The
re-run 40 minutes later was clean, so that lane landed its variant in between. The wasm gate was clean
throughout and is the one that actually compiles this lane's Rust.

## 8. Bundle

The dist `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` has a **live `trunk serve` on 6118** (pid 45306)
as its single writer, and the previous lane recorded a concurrent `trunk build` truncating it. So the bundle
was rebuilt through that single writer rather than against it: the generated inputs were regenerated, the
live serve rebuilt, and only the one file the serve had not yet promoted was written into the dist once,
atomically (temp + `rename`).

| artifact | before | after | proof |
|---|---|---|---|
| `🎞️frame-worker.js` | 1 125 050 B @ 09:54 | **1 130 598 B @ 10:29** | `PlaygroundBootPlanner` ×2, `worker-step`, `browser-owned-suspension`; `worker-boot-step-overrun` ×**0**, `interactive-job-overrun` ×**0** |
| `🚀️boot.js` | 51 073 B @ 09:14 | **52 268 B @ 10:14** | `Worker steps over budget` ×1, `Worker-Schritte über dem Budget` ×1, `workerDegraded` ×1 |
| `semio-framework-os-renderer-wgpu_bg.wasm` | @ 09:56 | **@ 10:15** | `strings | grep elapsedUs` → 1 (the new Rust boot-phase field) |
| `index.html`, `semio-framework-os-renderer-wgpu.js` | @ 09:56 | @ 10:15 | trunk-rebuilt |
| served by 6118 | — | `GET /🎞️frame-worker.js` → **200, 1 130 598 B**; `GET /🚀️boot.js` → **200, 52 268 B** | the running surface is serving this lane's bundle |

Generated inputs refreshed: `bun ./📜️script.ts generate-frame-worker` and
`bun ../../⚙️browser-build/📜️script.ts generate-browser-boot`. `🚀️boot.js` had **never** been regenerated
for the previous lane's `browser-boot` changes either — the wgpu `[watch]` list did not name
`⏱️ui-turn-budget`, so trunk never saw that module change at all.

### Trunk `[watch]` gaps closed

`📦️packages/🦀️rust/Trunk.toml` watched `../../🧊️renderer` but none of the sibling directories that crate
`#[path]`-includes, and did not watch the turn-budget module. Added: `../../⏱️turn-budget`,
`../../🌐️browser-worker`, `../../🧵️frame-job`, `../../🎠️runtime`, `../../🏠️os-host`, `../../📐️surface-lane`,
`../../📮️runtime-mailbox-core`, `../../📸️render-snapshot`, `../../🪢️kernel-seam`.

Still unwatched, deliberately: `../../🧱️elements` (Shell, Interpreter, Dock, ProgramBridge, Scenes,
EngineCanvas targets). Editing any of them today gives the dev no rebuild, but watching that tree would make
every concurrent lane's element edit churn a 76 MB wasm rebuild under a shared cargo lock. That trade needs a
decision this lane should not take alone.

## 9. What this lane did NOT change

* `INTROSPECTION_STEP_BUDGET_MS` (64 ms, already reported-not-fatal) and `🧵️frame-job/🦀️.rs`'s
  `INTERACTIVE_STEP_CEILING_US` ledger (already the ratified law).
* `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS` (60 s of silence) — the one remaining outer bound, and correctly so:
  it fires on SILENCE, never on slowness, and the chunked `plugin-graph` re-arms it more often than before.
* `ShellState::new` — see §3. `shell-construct` is already one macrotask-separated chunk under the 30 s
  browser-owned ceiling; it now reports its own `elapsedUs` so the next live boot says what it really costs.

## 10. Observed flake (recorded, not caused here)

While the box was building the wgpu bundle, `🧪️tests/🧩️package-integration` intermittently failed
`composes one nonempty WGPU surface through exact patch acknowledgement and terminal owner retirement`
(and once `activates only the current no-follow package through both independent TypeScript compilers`).
Both drive `🐚️plugin-bridge.ts` — a file a concurrent lane rewrote at 10:08 — and both are async and
time-sensitive. Isolated re-runs of that file were green 3/3, and the full 5-file suite was green **3/3 at
load average 99.7** once the trunk build finished. Recorded so the next lane recognises it as a load
artefact of exactly the kind this whole ticket is about, rather than a regression: a wall-clock-sensitive
assertion on a contended box is a coin flip.

Raw artefacts: `🗑️generated/wboot-measure-after.txt`, `🗑️generated/wboot-cargo-native.txt`,
`🗑️generated/wboot-cargo-wasm.txt`. Harness (kept): `🧪️wboot-measure.ts`.

## 11. Files changed

| file | kind |
|---|---|
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | new `PlaygroundBootPlanner` + `PLUGIN_GRAPH_CHUNK_ROWS`; `resolvePlaygroundBoot` is now the planner driven to completion |
| `…/🎯️targets/🧊️wgpu/⏱️ui-turn-budget/` → `…/⏱️turn-budget/🟦️.ts` | renamed; `WORKER_STEP_BUDGET_MS`, ledger ceiling + scope, `Ui`-prefixes dropped |
| `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | the whole step authority: executing clock, two ledgers, no budget throw, chunked `plugin-graph`, degraded cadence, worker step report |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `BrowserFrameWorkerStepReport`, `workerExecutingMs`/`workerStepVerdict`, `workerDurationMs >= 8` deleted, fallback state carries the Worker ledger |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | degraded cadence in the boot status line + dataset, Worker-step ledger in the fault banner (en/de) |
| `…/🎯️targets/🧊️wgpu/🔌️browser-interactive-job-port/🟦️.ts` | follows the `⏱️turn-budget` rename |
| `…/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | `BootPhase` split, `elapsed_us` per phase, `worker_now_ms` |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | web-sys `WorkerGlobalScope` feature |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml` | `[watch]` gaps closed |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts`, `📜️script.ts` | new suite registered |
| `…/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts` | **new** |
| `…/🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts`, `…/🧪️tests/📨️browser-frame-transport/🟦️.ts` | updated for the new protocol and rename |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js`, `🚀️boot.js` | regenerated |
