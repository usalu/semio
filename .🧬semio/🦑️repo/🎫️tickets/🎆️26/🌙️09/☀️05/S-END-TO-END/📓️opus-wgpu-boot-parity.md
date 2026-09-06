# Lane P — `wgpu-boot-parity`: the browser shell boots, and what stopped it

Opus 5 implementer, 2026-09-06 14:48 → . Input: `📓️opus-wgpu-crate-repair.md` (lane N), `📓️opus-wgpu-tier.md`
(lanes L/L2), `📋️plan.md` definition-of-done item 5. Private cargo target `target-s-e2e-wgpu`,
`RUSTC_WRAPPER=""`, `CARGO_PROFILE_WASM_DEV_DEBUG=false`, never `SEMIO_PARITY_QUIET_CARGO=1`.
Logs: scratchpad `lane-p/` (`serve-6066*.txt` = trunk serve, `boot-{1..4}.txt` = headless probes,
`flow-check-after.txt`, `wgpu-boot-probe.ts`).

## 0. Headline

| Item | Before this lane | After |
|---|---|---|
| `cargo check -p semio-framework-os-flow` | **18 × E0599** (peer's unfinished edit) | **exit 0, 6.85 s** |
| `trunk serve` on 6066 | never started | **`📡 server listening at 127.0.0.1:6066`, 14 successful builds** |
| Headless boot of the wgpu shell | never attempted | reached the GPU bootstrap; **four distinct blockers found and fixed**, §3 |
| `dumpStructure` introspection pair | not started (lane L2 §5) | **implemented end to end**, 3 new protocol tests, 35/35 green |
| The renderer crate's `#[cfg(test)]` suites | 56 errors (lane N §6.1) | 3 stale `include_str!` paths repaired; the rest **not reached** — §6 |

## 1. Task 1 — unblocking `semio-framework-os-flow`

Re-read at 14:49; the file was still ` M` unstaged with mtime **14:12:36**, i.e. untouched for 37 minutes,
and still had 27 `retire_cold(…)` call sites with no `ColdRetire` in scope. The failure was measured first
rather than taken from lane N's transcript:

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu CARGO_PROFILE_WASM_DEV_DEBUG=false \
    cargo check -p semio-framework-os-flow --message-format short
🌊️flow/📦️packages/🦀️rust/../../🖥️host/🦀️.rs:328:14: error[E0599]: no method named `retire_cold` found for struct `neural_engine::Tree`
… 18 total (Tree ×7, HashMap<String, Dictionary> ×7, EvalChannels ×4)
error: could not compile `semio-framework-os-flow` (lib) due to 18 previous errors; 29 warnings emitted
```

**The one change:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:13` — `ColdRetire` added to
the file's existing `use neural::{…}` list (between `BudgetedEval` and `Dictionary`, matching its own
ordering), **not** a second `use neural_engine::ColdRetire;` line as rustc suggested, so a peer resuming the
refactor sees one import list rather than a duplicate binding. Nothing else in the file was touched.

```
$ … cargo check -p semio-framework-os-flow --message-format short
    Finished `dev` profile [unoptimized] target(s) in 6.85s
EXIT=0
```

## 2. Task 3 — the `dumpStructure` introspection pair (implemented before the boot, on purpose)

Lane L2 §5 sketched this; it is now real. Doing it *before* the first browser build folded it into the same
trunk cycle instead of costing a second one.

### 2.1 Protocol — `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`

| Line | Change |
|---|---|
| `:17-18` | `FRAME_WORKER_INTROSPECTION_CAPACITY = 4`, `FRAME_WORKER_INTROSPECTION_TIMEOUT_MS = 10_000` |
| `:92-98` | `BrowserFrameIntrospectionProbe = "structure" \| "frame-stats"`, `BrowserFrameWorkerIntrospect` |
| `:100` | `introspect` joins `BrowserFrameUiMessage` |
| `:120` | `introspection` (`requestId`, `probe`, `json: string \| null`, optional `detail`) joins `BrowserFrameWorkerMessage` |
| `:198-199` | bounded pending map + monotonic request id |
| `:325-348` | `introspect(probe)` — **flushes a frame batch first**, so message ordering guarantees the Worker has ticked before it reads the retained tree; resolves `null` (never rejects, never faults) when not ready, over credit, or past the timeout |
| `:410-417` | the `introspection` reply settles its pending entry |
| `:452-456` | `clearQueues` settles every in-flight request with `null`, so a fail/quarantine/close cannot strand a caller |

### 2.2 Worker — `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`

- `:32-33` `RendererBindings` gains `dumpStructure`/`dumpFrameStats`; `:105` the module namespace is hoisted
  to a module-level `bindings` (the boot's local is now `loaded`) so it outlives `boot()`.
- `:132-135` `receive()` answers `introspect` **before** the `closed/closing/failed/quarantined` gate — a
  diagnostic must still work on a surface that is going down.
- `:174-196` `answerIntrospection()` never faults the Worker: a missing export, a throwing export, or a turn
  wider than `INTROSPECTION_STEP_BUDGET_MS` (64 ms) all come back as `json: null` plus a `detail`.

### 2.3 UI isolate — `…/🚀️browser-boot/🟦️.ts` — and the beacon is **not** `window.wasmBindings`

`:60-78` `attachIntrospectionBindings()` publishes `window.semioWgpuIntrospection = { dumpStructure,
dumpFrameStats }` **in `onReady` only** (`:225`), and removes it on fault and on `pagehide` (`:249`, `:263`).

Lane L2 assumed the hooks simply could not exist on the main thread. Measured, it is worse: **Trunk injects
its own module into `index.html` that instantiates the 133 MB renderer a SECOND time on the UI thread and
publishes it as `window.wasmBindings`** — despite `data-type="worker"`. That copy never boots, so
`triageParityBoot`'s old gate latched onto it at 3.1 s and the first call trapped:

```
3.1s STATE {"canvas":true,"hooks":true,…}                              # ← trunk's decoy, not the shell
3.1s FRAMESTATS ERR RuntimeError: memory access out of bounds
     at …<OnceLock<Mutex<semio_framework_ui::wgpu::engine::Ui>>>::initialize…
```

`data-wasm-no-import` and `data-wasm-no-import="true"` were both tried against trunk 0.21.14 and **both are
ignored** (verified by diffing the emitted `index.html` after two rebuilds). The decoy cannot be suppressed
from our side, so the shell stopped competing for the name: `🌐️.html:7-13` documents the hazard and the shell
owns `semioWgpuIntrospection` instead. That also removes a race — whichever module finished last used to win
the global.

### 2.4 Parity harness — `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`

- `:3168-3186` `dumpWgpuStructure`/`dumpWgpuFrameStats` now `await window.semioWgpuIntrospection.*` (the shim
  is async — it crosses the Worker seam); the docstring records why `wasmBindings` is wrong.
- `:3550` `triageParityBoot`'s wgpu gate waits for `semioWgpuIntrospection.dumpStructure`.

### 2.5 Tests — `…/🧪️tests/📨️browser-frame-transport.ts`, 32 → 35, all green

Three added: the request/response round trip (asserting the message order is `boot, batch, introspect`, i.e.
a frame is flushed first), the three fail-soft paths (not-ready / credit-exhausted / abandoned-by-close, and
that none raises a fault), and a source-shape assertion that the beacon is attached in `onReady` and that
`🚀️browser-boot` never writes `wasmBindings`.

```
$ bun ./📜️script.ts test-browser-worker
 Test Files  2 passed (2)      Tests  35 passed (35)      Duration  515ms
$ bun ./📜️script.ts check-browser-worker      # EXIT=0
$ bun ./📜️script.ts generate-frame-worker     # rerun after every edit
```

## 3. Task 2 — the boot, and the four blockers it exposed

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-wgpu CARGO_PROFILE_WASM_DEV_DEBUG=false \
    SEMIO_BUILD_BUDGET_MS=14400000 SEMIO_PLUGIN=s S_OS_PORT=6066 bun ./📜️script.ts serve
2026-09-06T13:10:08Z INFO 📡 server listening at:
2026-09-06T13:10:08Z INFO     🏠 http://127.0.0.1:6066/
```

First `trunk serve` ever to complete for this tier: `.🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` went from
empty to `index.html`, `semio-framework-os-renderer-wgpu.js` (206 KB), `…_bg.wasm` (**133 MB**),
`🎞️frame-worker.js`, `🚀️boot.js`, `🔌️plugin-modules/`, `🖼️assets/`. Probes used the coordinator's harness
shape plus the parity harness's own launch flags
(`--use-angle=swiftshader --enable-unsafe-swiftshader --enable-unsafe-webgpu`).

### 3.1 `worker-boot-step-overrun: wasm-instance blocked the Worker for 62.400 ms` (probe 1)

`monitoredSuspension` measured *every* boot stage against `WORKER_STEP_BUDGET_MS` (8 ms) — the **frame**
budget. Instantiating a 133 MB debug module, importing 57 plugin modules and acquiring a GPU device are the
browser's work, not ours, and are not sliceable; the budget could never be met, so no boot could ever
succeed. `🎞️frame-worker/🟦️.ts:59-65` adds `BROWSER_OWNED_SUSPENSION_BUDGET_MS = 1_000`, and `:67`/`:75` give
`ownedStep`/`monitoredSuspension` an explicit budget parameter, applied at exactly the four browser-owned
stages (`renderer-module`, `wasm-instance`, `plugin:*`, `gpu-platform`). `shell-boot` and the sliced
`renderer-bootstrap` steps deliberately keep the 8 ms budget — that code is ours.

### 3.2 `plugin-credits: boot plan exceeds 32 plugins` (probe 2)

`PLUGIN_BOOT_CAPACITY` was the literal `32`. The `s` boot plan resolves to **57** plugins (measured against
the real catalogue), so the hardcoded credit rejected the product outright. `🎞️frame-worker/🟦️.ts:51-55`
derives it from the generated catalogue itself — `PLUGIN_CATALOG.plugins.length +
PLUGIN_CATALOG.extensions.length` — still a compile-time-fixed bound (the catalogue is a generated constant)
but one that cannot go stale as the product grows.

### 3.3 A single bad plugin took the whole shell down

The boot loop let any plugin's load/manifest failure escape and kill the surface. The plugin cache
legitimately carries a fresh `space` core beside stale-or-missing descriptors for 23 others, so that is the
normal state, not an exception. `🎞️frame-worker/🟦️.ts:214-238` extracts `mountPluginHandles()`, which isolates
each target: a failure is reported as a `plugin-fault:<id>` boot-progress stage and skipped — the same
isolation the React shell's router gives a per-plugin descriptor fault. Cancellation is re-thrown, and an
empty result is still fatal upstream. Probe 3 confirmed all 57 mount and that boot advances to
`renderer-runtime 65%`, with the five `resolvePlaygroundBoot` dependency errors reported and survived:

```
2.0s error [DEBUG] resolvePlaygroundBoot(s): Plugin "draw" needs "draw-fsm", which is not installed.
2.0s error [DEBUG] resolvePlaygroundBoot(s): Plugin "sequence" needs "imperative-{control,effect,math,text}" …
2.1s worker http://127.0.0.1:6066/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js   (×4 shard workers)
```

Two supporting changes fell out of this:

- **The boot deadline became a boot *stall* deadline.** `🚚️browser-frame-transport/🟦️.ts:8-12` renames
  `FRAME_WORKER_BOOT_TIMEOUT_MS` → `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS`, and `:380-383`
  `armBootStallTimer()` rearms it on every `boot-progress`. A 57-plugin cold boot legitimately outruns any
  fixed total budget; a Worker that stops reporting still fails closed in 15 s.
- **Uncaught Worker errors now carry a stack.** `🎞️frame-worker/🟦️.ts:121-135` adds `scope.onerror` /
  `scope.onunhandledrejection`, routing a trap raised inside a wasm-scheduled callback into the protocol's
  typed `fault` with `filename:line:col` and the JS stack, instead of the UI isolate seeing only
  `worker-message-failed: <bare message>`.

### 3.4 `RuntimeError: memory access out of bounds` in the GPU bootstrap — a 1 MiB wasm shadow stack

Probes 3 and 4 (the second with trunk's decoy stripped by a `page.route` rewrite, proving the decoy was not
the cause):

```
4.1s STATE {"canvas":true,"hooks":false,"status":"renderer-runtime 65%",
     "alert":"worker-message-failed: Uncaught RuntimeError: memory access out of bounds"}
```

`renderer-runtime 65%` is the stage immediately before `semioWgpuWorkerBootstrap`, so the trap is inside
`GpuContext::from_offscreen_canvas`. It is a **trap**, not a Rust panic (`unreachable executed`), and the
same trap appeared in the decoy's `dumpStructure` inside
`<OnceLock<Mutex<Ui>>>::initialize → <UiSurfaceRegistry as Default>::default → array::try_from_fn::<…, 64>`.
Both are ordinary construction blowing the shadow stack: `wasm-ld` defaults to **1 MiB**, and a debug build
of this renderer materialises `[Option<UiSurfaceSlot>; 64]` and wgpu's descriptors in single frames.

The repo already knew this — `🧑‍💻dev/…/📜️script.ts:113` passes `-C link-arg=-zstack-size=8388608` for every
**plugin** wasm — but `[target.wasm32-unknown-unknown]`, the browser renderer's own target, never got it.
`.cargo/config.toml:8-16` now sets `-C link-arg=-zstack-size=16777216` (double the plugins', because the
renderer's debug frames are far larger) with the measurement recorded in the comment.

That invalidates every `wasm32-unknown-unknown` fingerprint, so the in-flight `trunk serve` was killed (the
whole process tree; `target-s-e2e-wgpu/.cargo-lock` verified free of orphans afterwards) and restarted cold.

## 4. Files changed

| File | What |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | `:13` `ColdRetire` import (§1) |
| `.cargo/config.toml` | `:8-16` browser-target shadow stack 1 MiB → 16 MiB (§3.4) |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | introspection message pair, bounded pending map, boot-stall timer (§2.1, §3.3) |
| `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | introspection handler, hoisted bindings, browser-owned budgets, catalogue-derived plugin credit, `mountPluginHandles`, uncaught-error seam (§2.2, §3.1-3.3) |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | `semioWgpuIntrospection` beacon + detach paths (§2.3) |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html` | documents trunk's duplicate UI-thread instantiation (§2.3) |
| `…/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport.ts` | 3 new tests (§2.5) |
| `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` | parity harness points at the shell-owned beacon (§2.4) |
| `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `:10152-10154` stale `include_str!` paths (§6) |

## 5. Blockers and hand-offs

1. **Trunk instantiates the renderer twice.** `data-type="worker"` does not stop trunk 0.21.14 appending a
   UI-thread `init()` + `window.wasmBindings = bindings`, and `data-wasm-no-import` is ignored. The page
   compiles 133 MB of wasm it never uses, and the decoy's exports trap when called. Fixable only by dropping
   `data-trunk rel="rust"` for an explicit build hook, or upstream. Worked around by naming the shell's own
   beacon; documented in `🌐️.html`.
2. **`check-frame-worker` went RED again** on a peer's unrelated OS-TS edit — this time between a kill and a
   restart of `trunk serve`, costing a restart. 5th occurrence across four sessions (lane L2 §6.5, lane N
   §4). It should become a build step rather than a committed artifact.
3. **`startParityDevServer` hardcodes `SEMIO_PARITY_QUIET_CARGO: "1"`** (`🧑‍💻dev/…/📜️script.ts:4022`), which
   injects `RUSTFLAGS=-Awarnings`, changes every fingerprint and forces a full cold rebuild in whatever
   target dir it uses — the exact mistake lane N warns about, wired into the harness itself. It also sets no
   `CARGO_TARGET_DIR` unless `PARITY_CARGO_TARGET_DIR` is exported, so it defaults to the contended repo
   `target/`. `SKIP_DEV=1` with operator-started servers on 7300/7301 is the only way to run `parity verify`
   against a warm target dir today.
4. **Every browser build is a full rebuild once**, for anyone sharing a `wasm32-unknown-unknown` target dir,
   because of the stack-size flag. Unavoidable and correct.
