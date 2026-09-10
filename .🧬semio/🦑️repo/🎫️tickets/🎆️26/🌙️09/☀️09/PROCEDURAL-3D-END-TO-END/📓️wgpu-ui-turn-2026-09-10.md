# wgpu UI-turn budget — `ui-turn-overrun` is a diagnostic, never a shell verdict

Lane: wgpu shell UI-turn / frame budget verdicts.
Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`.

Live finding this lane answers (`📓️runtime-verification-2026-09-09.md`, wgpu boot #2, 09:05):

```
shell-construct 85% — wgpu renderer fault: ui-turn-overrun: progress-hook UI turn took 5.000 ms
                       — No UI-thread frame fallback was attempted.
```

`shell-construct` is the frame worker's step 5, posted as `progress("shell-construct", 0.65 + 0.65 * 0.3)`
= 0.845 (`🎞️frame-worker/🟦️.ts:332`, `🌐️browser-worker/🦀️.rs:615`). The boot did not fail at the renderer:
the *UI isolate's own progress hook* — one `status.textContent = …` assignment — measured 5 ms of WALL time
in a hidden pane on a box at load ~20, and a single wall sample over a 2 ms ceiling was a FATAL verdict
during boot.

## 1. Verdict inventory — every UI-turn / frame budget in the wgpu shell path (before this lane)

| # | site | budget declaration | clock | measured span | verdict on breach |
|---|---|---|---|---|---|
| 1 | `🚚️browser-frame-transport/🟦️.ts:374` `observeUiTurn` | `FRAME_UI_TURN_BUDGET_MS = 2` (`:20`) | `options.now ?? performance.now()` — **wall** | one synchronous callback | `status === "ready"` → `quarantine("ui-turn-overrun")`; otherwise → **`fail("ui-turn-overrun")`** → transport `faulted`, worker closed, queues cleared, `onFault` → full-screen banner |
| 2 | `🚚️browser-frame-transport/🟦️.ts:526` `runUiHook` | same | same | `ready-hook`, **`progress-hook`**, `directive-hook`, `fault-hook` | as #1 — *and* a hook that THREW was reported under the same `ui-turn-overrun` code |
| 3 | `🚚️browser-frame-transport/🟦️.ts:326` `flush` `frame-transfer` | same | same | one `worker.postMessage` structured clone | as #1; `flush` returns `false` and the batch is lost |
| 4 | `🚀️browser-boot/🟦️.ts:106,163,230` | same, via `transport.observeUiTurn` | `performance.now()` — wall | pointer/wheel/key/paste/resize DOM handlers | as #1 (fatal only while `booting`; quarantine once ready) |
| 5 | `🔌️browser-interactive-job-port/🟦️.ts:85` `observeConsumerTurn` | `INTERACTIVE_JOB_UI_BUDGET_MS = 2` (`:7`) | transport's `now` — wall | one consumer callback | `quarantine(...)` → port `quarantined`, every slot force-closed, `quarantineConsumer` → transport quarantine → banner |
| 6 | `🔌️browser-interactive-job-port/🟦️.ts:278` `observe` | same | same | `input consumer`, `output consumer`, `terminal consumer`, `consumer close`, `status observer` | as #5 |
| 7 | `🎞️frame-worker/🟦️.ts:84` `ownedStep` | `WORKER_STEP_BUDGET_MS = 8` | `performance.now()` — wall | one owned worker step | `throw worker-boot-step-overrun` → `fault` message → transport `worker-boot-failed` |
| 8 | `🎞️frame-worker/🟦️.ts:92` `monitoredSuspension` | `BROWSER_OWNED_SUSPENSION_BUDGET_MS = 30_000` for browser-owned suspensions | `performance.now()` + 2 ms heartbeat | module load / `WebAssembly.instantiate` / `requestDevice` | as #7 |
| 9 | `🚚️browser-frame-transport/🟦️.ts:487` worker frame reply | `workerDurationMs >= 8` | worker-side wall | one worker frame step | `quarantine("worker-step-overrun")` |
| 10 | `🧵️frame-job/🦀️.rs:250,289,310,333` | `INTERACTIVE_STEP_CEILING_US` | `semio_framework_trace::Watchdog` — wall | one frame-job step | `StepOverrunLedger::admit(...).is_terminal()` → quarantine **only after 4 consecutive breaches** — already correct |
| 11 | `⏱️trace/🦀️.rs:601` `StepQuarantine::is_terminal` | `SUSTAINED_OVERRUN_QUARANTINE_STEPS = 4` | — | — | the repo's ratified law, see §2 |

Rows 1–6 are the defect: **single-sample wall verdicts that are fatal**. Rows 10–11 show the framework had
already ratified the opposite law on the Rust side and the TypeScript UI isolate never learned it.

## 2. Root cause

Two defects, both in the UI isolate's TypeScript, neither in the renderer or the boot plan.

### 2.1 A single WALL sample was a verdict

`observeUiTurn` compared one `performance.now()` delta against a 2 ms ceiling and acted on it. On this
target that reading cannot distinguish the turn's own work from the isolate not running: a browser
publishes **no per-thread CPU clock** (`performance.now()` is wall time), a hidden pane is additionally
background-throttled, and the box was scheduling ~20 other WASM workers. `status.textContent = …` is
microseconds of work; it read 5.000 ms.

The framework had already settled this exact question on the Rust side and written the reasoning down —
`⏱️trace/🦀️.rs`'s `SUSTAINED_OVERRUN_QUARANTINE_STEPS`:

> a single over-ceiling wall reading is as often the machine descheduling the thread as it is the step's
> own work (measured: a 19 µs unit read 14 571 µs on a load-average-90 box, and a browser worker is
> descheduled the same way)

and `CallbackVerdict::is_fault` is documented as "A measurement, never on its own a quarantine authority".
`🧵️frame-job/🦀️.rs` obeys that law (row 10). The TypeScript UI isolate never learned it.

### 2.2 The verdict was fatal, and the banner lied about the consequence

`runUiHook`'s failure ladder ran `fail(...)` while `status === "booting"`, which closes the Worker, clears
every queue and paints the full-screen banner — so the *measurement instrument* aborted shell
construction at 85 %. Worse, the same code was raised for a hook that **threw**, so a genuine defect and a
scheduling spike were indistinguishable. The banner then asserted a static
`No UI-thread frame fallback was attempted`, which is doubly wrong: nothing chose not to attempt one, and
after `transferControlToOffscreen()` there IS no UI-thread frame path to attempt.

Why boot #1 of the wgpu shell got past this point and boot #2 did not, with no change to the progress
path between them: a wall-clock kill on a contended box is load-dependent. The boot was a coin flip.

## 3. Fix

| # | file | change |
|---|---|---|
| 1 | **new** `🎯️targets/🧊️wgpu/⏱️ui-turn-budget/🟦️.ts` | the law, in one place. `UI_TURN_BUDGET_MS = 2` (unchanged meaning); `UiTurnClock` — an executing-span accumulator, the browser twin of the guest reactor's `TURN_EXECUTION`, so a turn that yields accrues nothing across the gap; `UiTurnLedger` — the twin of `StepOverrunLedger`, `SUSTAINED_UI_TURN_OVERRUN_TURNS = 4`, fixed 64-sample ring, no allocation on the measured path; `uiTurnDiagnosticsEnabled()` behind `SEMIO_RUNTIME_DIAGNOSTICS`. **The module owns no fault path at all** — it is structurally incapable of failing a surface |
| 2 | `🚚️browser-frame-transport/🟦️.ts` | `observeUiTurn` records instead of judging; `runUiHook` runs inside the executing clock and fails ONLY on a throw, under the new, separate `ui-hook-failed` code; `ui-turn-overrun` is **deleted** from `BrowserFrameWorkerFaultCode`; `flush`'s `frame-transfer` overrun no longer discards the batch it already posted; a degraded transport defers the progress hook through `deferToNextTurn` (yield-and-continue) instead of running it inline; new `degraded()`, `fallbackState()`, `onUiTurn` |
| 3 | `🚚️browser-frame-transport/🟦️.ts` | the interactive-job port's quarantine callback now raises `interactive-job-violation`, so a credit/protocol breach is no longer reported as a budget overrun |
| 4 | `🔌️browser-interactive-job-port/🟦️.ts` | every consumer turn (`input`/`output`/`terminal`/`close`/`status observer`) is priced by the same ledger and clock; a breach never quarantines the port, and the input pull is now still answered instead of being dropped mid-flight; new `reportConsumerFault` is the ONLY consumer-side path that may quarantine; new `uiTurnSnapshot()` |
| 5 | `🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts` | `InteractiveJobPort` gains `reportConsumerFault`; the observer walk stops using `observeConsumerTurn(site, Number.POSITIVE_INFINITY)` as a "threw" sentinel, and no longer abandons the remaining observers on a slow one |
| 6 | `🚀️browser-boot/🟦️.ts` | `renderFault` takes the real `BrowserFrameFallbackState` and renders it, in **en and de**, replacing the static no-fallback sentence: surface state, why UI-thread frames are unavailable (`OffscreenCanvas transferred`), whether the Worker was terminated, whether input is still admitted, and the UI-turn ledger (recorded/sustained, p99, worst site); `onUiTurn` stamps `canvas.dataset.uiTurn` so a probe can read the last recorded overrun |
| 7 | `🎞️frame-worker/🟦️.ts` | docstring corrected — the `boot-progress`-every-second overrun it cites is no longer fatal |

### What "degrades gracefully" means concretely

* An **isolated** breach: recorded, traced under `SEMIO_RUNTIME_DIAGNOSTICS`, work continues inline. Nothing else.
* A **sustained** run (4 in a row): the transport latches deferred cadence — subsequent `boot-progress`
  hooks are handed to `deferToNextTurn` (a zero-delay macrotask) instead of running inline, so the isolate
  paints and pumps input between the pieces. The boot still completes. The latch clears itself on the first
  turn that fits the ceiling again, so a surface recovers without intervention.
* A **clock fault** (absent/backward/non-finite reading): recorded as such and ignored, because a broken
  clock is not evidence about the work.
* A hook that **throws**: still fatal, under `ui-hook-failed`. That is a defect, not a budget.

## 4. Verdict inventory — after

| # | site | budget | clock | verdict on breach |
|---|---|---|---|---|
| 1 | `observeUiTurn` | `UI_TURN_BUDGET_MS = 2` | `UiTurnClock` — **executing** spans | record → `onUiTurn`; **never** quarantine/fail/close |
| 2 | `runUiHook` (`ready`/`progress`/`directive`/`fault`/`deferred` hooks) | same | same | record; a THROW → `fail("ui-hook-failed")` |
| 3 | `flush` `frame-transfer` | same | same | record; the posted batch is kept |
| 4 | `🚀️browser-boot` input/resize handlers | same, via `observeUiTurn` | wall delta fed to the ledger | record |
| 5 | `observeConsumerTurn` | same | caller's delta | record; `false` = "please yield", only after 4 in a row |
| 6 | port `observe` (5 consumer sites) | same | `UiTurnClock` | record; a THROW → `quarantine` |
| 7–9 | frame worker `ownedStep` / `monitoredSuspension` / `workerDurationMs >= 8` | unchanged (worker-owned, not this lane) | | unchanged |
| 10–11 | `🧵️frame-job/🦀️.rs` + `StepOverrunLedger` | unchanged | | unchanged — and now the law both sides share |

## 5. Tests

New file `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts`
(registered in `📦️packages/🦀️rust/vitest.config.ts` and in `BrowserWorkerTestScript`):

| test | drives |
|---|---|
| pins the ceiling and the attribution law to their Rust twins | reads `⏱️trace/🦀️.rs` and asserts `SUSTAINED_UI_TURN_OVERRUN_TURNS === SUSTAINED_OVERRUN_QUARANTINE_STEPS`; pins `UI_TURN_DIAGNOSTICS_KEY` against `🏛️ShellHost/🟦️.tsx`'s `RUNTIME_DIAGNOSTICS_KEY` — the cross-language twins cannot drift silently |
| charges executing time only — a turn parked across a real 50 ms suspension measures nothing | REAL `setTimeout(50)` between `suspend()`/`resume()`: wall ≥ 45 ms, executing < 2 ms |
| charges a real 50 ms spin and reaches the sustained verdict, never a fatal one | REAL busy-spin ×4 → `recorded`, `recorded`, `recorded`, `sustained`; then one cheap turn clears the latch |
| treats an unusable clock reading as a clock fault, never as an overrun | `undefined` / `NaN` / `Infinity` / `-1` |
| **completes shell construction when a progress-hook turn sleeps 50 ms of wall time** | the required scenario: the isolate's clock jumps 50 ms across the `shell-construct` progress hook — the hook is delivered, the transport stays `booting`, no fault is raised, and the following `booted` reaches `ready` |
| **degrades to deferred cadence and keeps reporting progress when the hook spins past the ceiling** | the required scenario: 4 consecutive REAL 50 ms spinning progress hooks → `degraded() === true`, still `booting`, no fault; the 5th report is deferred to a macrotask (not run inline) and is delivered when it drains; `booted` then reaches `ready` |
| reports the real fallback state instead of a static no-fallback claim | `fallbackState()` before/after a worker fault |
| keeps recorded-overrun traces behind the runtime diagnostics switch | nothing printed disarmed; `[DEBUG] ui-turn recorded-overrun site=…` armed |

Rewritten in `🧪️tests/📨️browser-frame-transport/🟦️.ts`:
* "measures external UI hooks centrally and **stops immediately on hook overrun**" → "**keeps the surface alive** on hook overrun" (asserts `ready`, no fault, one `recorded-overrun:ready-hook`, and that the next batch still flushes)
* new "fails the surface only when a UI hook throws, under its own fault code" (`ui-hook-failed`)
* "**fails closed** when the bounded structured clone consumes the UI turn budget" → "**records rather than fails**" (`flush()` now `true`, surface `ready`, ledger incremented)

Rewritten in `🧪️tests/🎮️browser-interactive-job-port/🟦️.ts`:
* "quarantines an exact consumer overrun…" → "records a consumer overrun without quarantining **and still answers the pull**" (the old code dropped the `job-input-page` on an overrun — a lost pull, not just a false verdict)
* new "quarantines a consumer that throws, and cursor-drains close" (keeps the drain coverage the old test carried)
* new "asks the caller to yield only after a sustained run of consumer overruns"

## 6. Runs

```
bun ./📜️script.ts test-browser-worker                      # wgpu package TS/JS
  Test Files  3 passed (3)
       Tests  46 passed (46)

bun x vitest run … 🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts --reporter=verbose
  ✓ pins the ceiling and the attribution law to their Rust twins                                  3ms
  ✓ charges executing time only — a turn parked across a real 50 ms suspension measures nothing  54ms
  ✓ charges a real 50 ms spin and reaches the sustained verdict, never a fatal one              201ms
  ✓ treats an unusable clock reading as a clock fault, never as an overrun                        0ms
  ✓ completes shell construction when a progress-hook turn sleeps 50 ms of wall time              1ms
  ✓ degrades to deferred cadence and keeps reporting progress when the hook spins past the …    251ms
  ✓ reports the real fallback state instead of a static no-fallback claim                         1ms
  ✓ keeps recorded-overrun traces behind the runtime diagnostics switch                           0ms
  Tests  8 passed (8)

CARGO_TARGET_DIR=$S/target-wgpu RUSTC_WRAPPER="" RUST_MIN_STACK=536870912 \
  cargo check -p semio-framework-os-renderer-wgpu --keep-going
  warning: `semio-framework-os-renderer-wgpu` (lib) generated 18 warnings
      Finished `dev` profile [unoptimized] target(s) in 1m 17s          → errors 0, warnings 122

… --target wasm32-unknown-unknown --keep-going
  warning: `semio-framework-os-renderer-wgpu` (lib) generated 14 warnings
      Finished `dev` profile [unoptimized] target(s) in 1m 22s          → errors 0, warnings 118

bun x vitest run … 🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx     # the InteractiveJobPort interface change
  Test Files  1 passed (1)
       Tests  42 passed (42)

bun x tsc --noEmit -p tsconfig.json
  0 errors in any file this lane touched (the repo-wide count is generated `.d.ts` /
  `storybook-static/` noise owned by other lanes)
bun x tsc -p tsconfig.json --listFiles | grep …
  all six changed/new sources ARE in the program — a silent zero over an empty file set
  would have been worthless
```

Both cargo runs emitted per-crate **warnings**, so the crate genuinely type-checked rather than aborting
early (`📓️require-warnings-as-proof-of-typecheck`). Nothing in this lane is Rust; the checks are the
regression gate on the package the bundle is built from.

The wgpu package's `test` target also runs the crate's native cargo tests; four
`async_boundary_tests::*` cases (one a `SIGABRT` in a `store` `Drop` witness) fail there and are
**pre-existing and unrelated** — this lane changed no Rust.

## 7. Bundle proof

`bun ./📜️script.ts build` with `CARGO_TARGET_DIR=$S/target-wgpu-boot RUSTC_WRAPPER="" SEMIO_RENDERER=wgpu`
after regenerating both generated entrypoints (`generate-browser-boot`, `generate-frame-worker` — both are
freshness-checked by `checkBrowserBoot`/`checkFrameWorker` inside the build itself):

```
2026-09-10T07:17:15Z  INFO applying new distribution
2026-09-10T07:17:15Z  INFO ✅ success
trunk built wgpu renderer -> …/.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu
```

Markers in the deployed `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` bundle:

| marker | `🚀️boot.js` | `🎞️frame-worker.js` |
|---|---|---|
| `ui-turn-overrun` | **0** | **0** |
| `No UI-thread frame fallback` | **0** | — |
| `unavailable-offscreen-transferred` | 1 | — |
| `SUSTAINED_UI_TURN_OVERRUN_TURNS` | 3 | 3 |
| `ui-hook-failed` | 2 | — |
| `deferred-hook` | 1 | — |
| `localStorage` | (UI isolate only) | **0** |

The coordinator only needs to restart `trunk serve` on 6118.

⚠️ The FIRST `build` attempt failed with `asset pipeline: No such file or directory` and left a truncated
dist (18 of 63 `🔌️plugin-modules`, 6 of 19 `🖼️assets`) — the live `trunk serve` on the SAME `dist`
(`Trunk.toml` points both at the shared renderer-module cache) was rebuilding into it concurrently. The
retry succeeded and restored the full dist. Anyone rebuilding this bundle while a serve is live should
check the dist child counts, not just the exit code.

### One deliberate constraint discovered while wiring the diagnostics gate

`⏱️ui-turn-budget/🟦️.ts` is bundled into `🎞️frame-worker.js`, whose carrier census
(`📜️script.ts` `checkFrameWorkerCarrierCensus`) forbids `localstorage` outright — the first
`generate-frame-worker` failed on exactly that. The module therefore reads **no** web storage: it resolves
only an explicit override and `VITE_SEMIO_RUNTIME_DIAGNOSTICS`, and `🚀️browser-boot/🟦️.ts`'s
`armUiTurnDiagnostics()` reads the stored preference in the UI isolate and hands the answer to
`setUiTurnDiagnostics`. The census is intact (0 hits in the built worker).

## 8. Files changed

| file | |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⏱️ui-turn-budget/🟦️.ts` | **new** — the law |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | non-fatal pricing, `ui-hook-failed`, `deferToNextTurn`, `degraded()`, `fallbackState()`, `onUiTurn` |
| `…/🎯️targets/🧊️wgpu/🔌️browser-interactive-job-port/🟦️.ts` | same law for consumer turns, `reportConsumerFault`, `uiTurnSnapshot()` |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | real bilingual fallback banner, `armUiTurnDiagnostics`, `canvas.dataset.uiTurn` |
| `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | docstring correction |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts` | `reportConsumerFault` on the port interface; no `Infinity` sentinel; observers no longer abandoned |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx` | three port stubs updated |
| `…/🧪️tests/⏱️wgpu-ui-turn-budget/🟦️.ts` | **new** — 8 tests |
| `…/🧪️tests/📨️browser-frame-transport/🟦️.ts` | 3 tests rewritten/added |
| `…/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts` | 3 tests rewritten/added |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts`, `📜️script.ts` | new suite registered |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js`, `🎞️frame-worker.js` | regenerated |

Raw logs: `🗑️generated/uiturn-cargo-native.txt`, `uiturn-cargo-wasm.txt`, `uiturn-bundle.txt`,
`uiturn-vitest.txt`, `uiturn-diagram.txt`, `uiturn-tsc-owned-files.txt`.
