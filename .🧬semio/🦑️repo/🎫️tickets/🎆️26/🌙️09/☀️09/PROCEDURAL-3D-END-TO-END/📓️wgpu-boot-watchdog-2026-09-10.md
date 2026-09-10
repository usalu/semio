# wgpu browser boot — a watchdog may only kill an UNRESPONSIVE Worker, never a busy one

Lane: wgpu browser-boot watchdog / progress protocol.
Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`.

Live finding this lane answers (wgpu boot #5, 11:45, `http://localhost:6118/?plugin=generation3d`,
hidden pane, box load 55–70):

```
renderer-runtime 65% — wgpu renderer fault: worker-boot-timeout:
                       Worker reported no boot progress for 60000 ms
                       — Worker terminated: yes · input accepted: no
                       UI turns over budget (recorded/sustained): 1/0 (worst 2.1 ms)
                       Worker steps over budget (recorded/sustained): 0/0
```

`0/0` worker steps and `1/0` UI turns in that very banner are the proof that nothing measurable ran long.
This is the same verdict family the three previous lanes removed from UI turns
(`📓️wgpu-ui-turn-2026-09-10.md`), worker steps (`📓️wgpu-worker-boot-2026-09-10.md`) and the guest first
turn (`📓️first-step-deadline-2026-09-10.md` / `📓️shard-termination-2026-09-10.md`) — and the one place it
survived, because it was the last clock in the boot path that could still terminate anything.

## 1. Inventory — every boot watchdog / timeout in the wgpu browser boot

All line numbers as of this lane's start. **Verdict** is what a breach actually does.

### 1.1 UI isolate — `🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`

| # | site | value | clock | what counts as progress | verdict |
|---|---|---|---|---|---|
| 1 | `:17,:484` `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS` boot-stall timer | **60 000 ms** | `window.setTimeout`, wall | a `boot-progress` **or** `boot-liveness` message re-arming the timer | **`fail("worker-boot-timeout")` → `requestWorkerClose()` + `worker.terminate()`; the transferred `OffscreenCanvas`, the surface's only frame path, dies with it** — *the live fault* |
| 2 | `:26,:396` `FRAME_WORKER_INTROSPECTION_TIMEOUT_MS` | 10 000 ms | `setTimer`, wall | the matching `introspection` reply | resolves `null` — a diagnostic, never a verdict |
| 3 | `:262` `deferToNextTurn` | 0 ms | macrotask | — | none |
| 4 | `:5` UI turn ledger (`UI_TURN_BUDGET_MS` 2 ms) | 2 ms | executing spans | — | recorded / deferred cadence only (previous lane) |
| 5 | `:559` worker frame reply verdict | 8 ms, worker-side | executing spans | — | `quarantine`, only on a sustained run (previous lane) |

### 1.2 Frame Worker — `🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`

| # | site | value | clock | what counts as progress | verdict |
|---|---|---|---|---|---|
| 6 | `:61` `BOOT_HEARTBEAT_MS` | 2 ms | `setInterval` on the Worker's own loop | samples the gap into `maximumBlockMs` | observation only |
| 7 | `:70` `BOOT_LIVENESS_INTERVAL_MS` | 1 000 ms | the same `setInterval` | posts `boot-liveness`, re-arming #1 | **cannot fire while the Worker's event loop is blocked** — the defect |
| 8 | `:96` `stepLedger` (`WORKER_STEP_BUDGET_MS` 8 ms) | 8 ms | executing spans | — | recorded / deferred cadence (previous lane) |
| 9 | `:100` `suspensionLedger` (`BROWSER_OWNED_SUSPENSION_BUDGET_MS`) | 30 000 ms | wall | — | recorded only; explicitly cannot end a boot (previous lane) |
| 10 | `:82` `INTROSPECTION_STEP_BUDGET_MS` | 64 ms | wall | — | reported on the answer |
| 11 | `:145` frame step / `:404` close step | 8 ms | executing spans | — | quarantine on a sustained run |

### 1.3 Reached from the boot, owned elsewhere

| # | site | value | verdict |
|---|---|---|---|
| 12 | `🎭️actor/📮️shard-client/🟦️.ts:413` `SHARD_LIVENESS_POLICY` — `heartbeatTimeoutMs` 5 000 × `missedLimit` 3, `firstTurnTimeoutMs` 30 000, `pluginLoadIdleTimeoutMs` 30 000, `pluginLoadCeilingMs` 300 000 | terminates one shard worker, with `describeShardSilence` naming the phase — **the precedent this lane follows** |
| 13 | `🌐️browser-worker/🦀️.rs` `BrowserRendererBootstrap::step` | none | the Rust bootstrap phases are unbounded and, being synchronous, invisible to #7 |

**Row 1 was the only clock in the whole boot path that could still terminate anything** — and its progress
predicate (row 7) is structurally unable to fire during exactly the phases that take longest.

## 2. Root cause

`boot-liveness` is a `setInterval` on the frame Worker's **own** event loop. Three of the boot's phases
block that loop by construction, and therefore cannot post while they run:

- `wasm-instance` — `loaded.default(url)`, wasm-bindgen's `instantiateStreaming` of **76 048 601 B**;
- `gpu-platform` — `semioWgpuWorkerBootstrap`'s synchronous prologue; the last `boot-progress` before it is
  `progress("renderer-runtime", 0.65)`, which is exactly the stage the live banner froze on;
- `renderer-bootstrap` — one synchronous Rust phase per `bootstrap.step()` (`font-atlas` measured at
  **12 002 ms** in `📓️wgpu-worker-boot-2026-09-10.md`).

So the watchdog's premise — *"a message proves life, therefore silence proves death"* — is false precisely
where it is load-bearing. A perfectly healthy Worker doing the single most expensive thing the boot asks of
it is indistinguishable from a wedged one, and on a box at load 55–70 in a background tab it crosses 60 s.
This is verbatim the same false premise `evaluateShardLiveness` had to abandon for guest first turns
(`📮️shard-client/🟦️.ts:482` — a compute-bound JSPI turn runs with the ticker firing **0** times inside it).

## 3. The fix — declare-phase + heartbeat, with schema-owned per-phase ceilings

New shared law: **`🎯️targets/🧊️wgpu/🫀️boot-liveness/🟦️.ts`**, imported by BOTH browser isolates, so a
ceiling can never drift between the watchdog and the Worker that is measured by it.

```
FRAME_WORKER_BOOT_LIVENESS_POLICY = {
  silenceTimeoutMs: 60 000,          // bound on a Worker with NOTHING declared
  livenessIntervalMs: 1 000,         // the Worker's own ticker cadence
  defaultPhaseCeilingMs: 300 000,    // an undeclared phase is still bounded
  phaseCeilingMs: {
    renderer-module 300 000 · wasm-artifact 60 000 · wasm-cache-read 120 000 ·
    wasm-compile 900 000 · wasm-instantiate 300 000 · plugin 300 000 ·
    gpu-platform 900 000 · shell-boot 900 000 · renderer-bootstrap 900 000
  },
}
```

**Protocol.** A new worker→UI message `{ kind: "boot-phase", phase, state: "enter" | "leave", elapsedMs }`.
The Worker posts `enter` **before** it blocks — while its loop still runs, so the declaration always
arrives — and `leave` in a `finally`, carrying the phase's real cost. Two sites declare:

- `monitoredSuspension(stage, …)` — every browser-owned suspension (`renderer-module`, `wasm-*`,
  `plugin:<id>`, `gpu-platform`, `shell-boot`);
- new `declaredStep(stage, …)` — the synchronous blockers `renderer-bootstrap`, `renderer-finish`,
  `interactive-job-registry`.

**Rule** (`evaluateBrowserBootLiveness`, pure and replayable from a JSON timeline, no transport in the
picture — the shape `evaluateShardLiveness` established):

- a phase is declared and inside its own ceiling → **BUSY**: re-arm for `min(silenceTimeout, remaining)`,
  never terminate;
- a phase is declared and past its ceiling → terminate, naming the phase, its elapsed and that ceiling;
- no phase declared and silent past `silenceTimeoutMs` → terminate; this is the one state that really is a
  wedged event loop, and the fault says so in those words.

The watchdog is now **one self-rescheduling timer for the whole boot** instead of one re-armed per
`boot-progress`: a message only stamps `lastLivenessAtMs`, and the wake re-reads the window and schedules
itself for the exact remaining time — fewer timers, and the deadline is exact rather than a window late.

**Diagnosis.** `describeBrowserBootSilence(report, "en" | "de")` composes the fault detail; the fallback
panel gains two lines from `describeBrowserBootPhase` and `BrowserFrameFallbackState`'s new
`bootPhase` / `bootPhaseElapsedMs` / `bootStage` / `bootSilentForMs`:

```
Boot stage: renderer-runtime · silent for 901000 ms
Long boot phase: "gpu-platform" for 901000 ms (ceiling 900000 ms)

Boot-Stufe: renderer-runtime · still seit 901000 ms
Lange Boot-Phase: „gpu-platform“ seit 901000 ms (Obergrenze 900000 ms)
```

## 4. Long steps now emit progress themselves

`loaded.default(url)` was one call that fetched, compiled, instantiated and linked 76 MB behind a single
stage name and reported nothing while it ran. It is replaced by four declared phases
(`instantiateRendererWasm`):

| phase | work | what it reports |
|---|---|---|
| `wasm-artifact` | one `HEAD` for the artifact's validator | declared; ~1 round trip |
| `wasm-cache-read` | IndexedDB lookup of the compiled module | declared |
| `wasm-compile` | `WebAssembly.compileStreaming` over a **counting `TransformStream`** | declared **and** one `boot-progress` every 2 % of the body — real streamed progress, `wasm-compile 38 %` |
| `wasm-instantiate` | `bindings.default(module)` on the already-compiled module | declared |

Streaming compilation is preserved — the bytes are never buffered into one 76 MB array; the transform only
counts them on the way past. Any failure (a browser that will not `compileStreaming` a constructed
`Response`) answers `undefined` and hands the boot straight back to wasm-bindgen's own URL path, still
inside a declared `wasm-instantiate`, so the fix cannot make the boot worse than it was.

## 5. Compiled-module cache across reloads

`🎯️targets/🧊️wgpu/🗄️wasm-module-cache/🟦️.ts` keeps the compiled `WebAssembly.Module` in IndexedDB, so a
second boot on an unchanged artifact skips the compile entirely.

**Identity.** The row is validated against the artifact's HTTP validator — `ETag` when offered, else
`Last-Modified` + `Content-Length` — read by one `HEAD` that precedes the 76 MB body. The live serve on
6118 answers `last-modified: Thu, 10 Sep 2026 09:29:09 GMT` + `content-length: 76048601`, so the key on
this surface is `mtime:Thu, 10 Sep 2026 09:29:09 GMT:76048601`.

**Deviation from the brief, stated plainly:** the brief asked for the artifact *sha*. Web Crypto has no
streaming digest, so hashing would mean retaining the entire 76 MB body purely to hash it — spending more,
in both time and resident memory on an already-thrashing box, than the compile the cache saves, and
defeating `compileStreaming` in the process. The server's own validator is the identity the server asserts,
a changed artifact never matches it, and the tag is stored on the row so a reader can see what matched. If
a content digest is later required, the honest place for it is a build-time manifest the boot can read in a
constant number of bytes — not a client-side rehash of every boot.

**Fail-soft, everywhere.** A browser that refuses to structured-clone a `WebAssembly.Module` (WebKit), a
private window with no IndexedDB, a quota rejection, a blocked upgrade from another tab and a corrupt row
all read as *no cache*, and the boot compiles exactly as it did before. Every IndexedDB operation is capped
at 5 s, because a cache lookup must never be the thing that stalls a boot. A server that stops offering a
validator evicts the row rather than risking a stale module.

## 6. Measurements

### 6.1 What the compile actually costs — and what it does *not* explain

`$T/🐍️wasm-compile-timing-probe.mjs`, run natively (Bun/JavaScriptCore) against the **live artifact served
on 6118**, `mtime:Thu, 10 Sep 2026 09:53:20 GMT:76048601`, 76 048 601 B, 706 descriptor rows, three runs:

| run | `compileStreaming` (cold boot pays) | buffered `WebAssembly.compile` | reuse of an already-compiled `Module` (cached boot pays) |
|---|---|---|---|
| 1 | 995.7 ms | 932.0 ms | 0.366 ms |
| 2 | 622.1 ms | 478.9 ms | 1.316 ms |
| 3 | 570.6 ms | 319.4 ms | 0.687 ms |

So the module cache removes **≈0.6–1.0 s** per boot on this box — worth having, and it makes the second
boot's `wasm-*` phases effectively free. JavaScriptCore compiles wasm lazily, so V8's eager baseline pass
over 76 MB in a throttled background tab is expected to be materially larger; this is a floor, not a
ceiling.

**But it settles the attribution, and it corrects the brief's premise.** The 60 s stall was NOT the wasm
compile. The compile happens at `wasm-instance`, **15 %**, long before the banner froze; `renderer-runtime`
at **65 %** is the `progress()` call issued immediately *before*
`monitoredSuspension("gpu-platform", () => semioWgpuWorkerBootstrap(...))`, so the phase that actually went
silent for 60 s is **`gpu-platform`** — the Rust engine bring-up, whose synchronous prologue blocks the
Worker's event loop and therefore its liveness ticker. That is exactly the phase this lane now declares,
and it is why the declaration — not the cache — is the fix.

### 6.2 Live boot times: not measured by this lane

Browser tools were out of scope here, so there is no boot #6 wall time in this report. What the lane leaves
behind so the next boot yields those numbers directly:

- every declared phase posts its real cost on withdrawal (`boot-phase … state:"leave", elapsedMs`), logged
  as `[DEBUG] boot-phase <phase> <ms> ms` for any phase over 1 s;
- the compile reports itself every 2 % (`wasm-compile 38 %`), so the status line moves through the
  transfer instead of freezing;
- a cached boot renders `wasm-instantiate:cached 72.5 MB` as its stage, which is a one-glance cache hit/miss
  readout.

## 7. Tests

`bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker` — **4 files, 61 tests, all passing**
(was 51). Ten new cases, all in `🧪️tests/📨️browser-frame-transport/🟦️.ts`, driven by a virtual clock so a
120 s boot is decided in microseconds:

| case | asserts |
|---|---|
| re-arms for the exact remaining window… | `evaluateBrowserBootLiveness` as a pure function: exact re-arm, terminate past the silence ceiling, `∞` for a Worker that never spoke, BUSY inside a phase ceiling, terminate at it |
| prices a phase family by the segment before its colon | `plugin:generation3d` inherits `plugin`; an unnamed phase still gets a bounded default |
| **keeps a Worker that declared a long phase alive across 120 s of total wall silence** | no fault, `status === "booting"`, `worker.terminated === false`, `bootPhaseElapsedMs === 120 000`; then `leave` + `booted` → `ready` |
| keeps a heartbeating Worker alive indefinitely… | 240 × 1 s liveness beats (240 s wall) with no fault, then 60 s of silence → `worker-boot-timeout` |
| **…and still terminates one that goes silent with no phase declared** | fault detail contains `declared no long phase` **and** the last stage `plugin:generation3d`; the close message is posted |
| terminates a declared phase only once it blows its OWN ceiling | alive at `ceiling − 1`, faulted at `ceiling + 1`; detail names `"gpu-platform"`, `900000 ms ceiling` and `last reported stage "renderer-runtime"`; the fallback state carries the phase |
| names a Worker that never sent a single message, and says the same thing in German | `hat nie eine einzige Nachricht gesendet`, `Ereignisschleife hängt`; exact en/de strings from `describeBrowserBootSilence` and `describeBrowserBootPhase` |
| declares every browser-owned phase BEFORE it blocks | source contract: `declarePhase(stage,"enter")` precedes `stepClock.suspend()` inside `monitoredSuspension`; `declaredStep("renderer-bootstrap")`; `compileStreaming` + cache reads present; the opaque `monitoredSuspension("wasm-instance")` is gone; the panel renders the phase line |
| **replayed on a third-party clock** | the same 120 s timeline decided by vitest's own fake timers (`@sinonjs/fake-timers`) driving real `setTimeout`/`clearTimeout` — an INDEPENDENT clock implementation reaching the same two verdicts, so a bug in this file's hand-rolled queue cannot pass for a passing law |
| renderer module cache: identity and fail-soft | `ETag` → `Last-Modified`+`Content-Length` → `""`; a 404, a validator-less 200 and a throwing `fetch` all disable the cache; with no IndexedDB every read is `undefined`, every write `false`, and the evict resolves |

Other gates:

| gate | result |
|---|---|
| `test-preview-generated` (`🧪️tests/🧩️package-integration`, the deterministic in-memory frame-worker owner contract, renders both browser bundles) | **1 file, 20 tests, passing** |
| `📨️browser-frame-transport` + `📥️wgpu-intake-budget` under the **react** engine config at `SEMIO_TEST_LEVEL=long` (different aliases, different root) | **2 files, 35 tests, passing** |
| `check-browser-worker` (bundles both isolates, runs the frame-worker carrier census) | exit 0 — no credential carrier, no `localStorage`/`sessionStorage` in the Worker bytes; the cache uses IndexedDB, which the census permits |
| `cargo check -p semio-framework-os-renderer-wgpu` | **not run — this lane changed no Rust.** Every edit is TypeScript in `🎯️targets/🧊️wgpu`; `🌐️browser-worker/🦀️.rs` and the shell are untouched |

Raw logs: `🗑️generated/wdog-browser-worker.txt`, `🗑️generated/wdog-preview-generated.txt`,
`🗑️generated/wdog-react-transport.txt`, `🗑️generated/wdog-wasm-timing.txt`.

## 8. Bundle

The dist `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` has a **live `trunk serve` on 6118**
(pid 45306) as its single writer, so the bundle was rebuilt **through** that writer, never against it:
`generate-browser-boot` and `generate-frame-worker` refreshed the two generated inputs, the running serve
noticed them under `[watch]`'s `🟦️typescript`, rebuilt, staged into `.stage/` and promoted. No competing
`trunk build` was ever started, and 6018 was never touched.

### 8.1 Source-input and watch contracts for the two new modules

The boot bundle's Nx cache key is derived from the **import graph**, not a hand-kept list
(`📚️library/🟨️.mjs` `declaredSourceInputs` → `relativeSourceInputs`), and
`🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts` asserts that every esbuild `metafile` input of
`🚀️browser-boot/🟦️.ts` is declared. Measured directly (`$T/🐍️browser-boot-source-graph-probe.ts`):

```
declaredCount 13 · graphCount 7 · untracked []
🫀️boot-liveness declared: true · in the boot graph: true
🗄️wasm-module-cache in the boot graph: false   ← worker-only, correctly absent
```

`Trunk.toml`'s `[watch]` list *is* hand-kept, and it named `⏱️turn-budget` but could not name modules that
did not exist yet, so `../../🫀️boot-liveness` and `../../🗄️wasm-module-cache` were added to it. Without
that, an edit to the liveness law alone would not have rebuilt the served bundle.

**Pre-existing failure, not this lane's:** the full `🧊️wgpu-browser-boot-cache-inputs` contract aborts on
its very first assertion —

```
project.targets["check-browser-worker"].cache === false
  "Generated-file freshness checks must execute against the current output"
  true !== false
```

`📋️project.json` carries `"cache": true` for `check-browser-worker` **at `HEAD` (9b605a4550, 2026-09-09
12:10)**, before this lane started, and this lane did not touch that file. Left for its owner; the boot
graph half of the same contract was therefore verified directly, above.

### 8.2 Promoted — proof

Promotion completed **12:14:13**.

| artifact | before | after | proof |
|---|---|---|---|
| `🚀️boot.js` | 52 268 B @ 11:52 | **58 106 B @ 12:04** | `evaluateBrowserBootLiveness` ×2, `describeBrowserBootPhase` ×2, `Long boot phase` ×2, `Lange Boot-Phase` ×2, `declared no long phase` ×1, `hat nie eine einzige Nachricht gesendet` ×1 |
| `🎞️frame-worker.js` | 1 131 119 B @ 11:52 | **1 137 994 B @ 12:04** | `"boot-phase"` ×1, `declarePhase` ×5, `bootDeclarationsOpen` ×3, `WebAssembly.compileStreaming` ×1, `readCachedRendererModule` ×2, `semio-wgpu-renderer-modules` ×1; retired `wasm-instance` ×**0** |
| `semio-framework-os-renderer-wgpu.js` / `_bg.wasm` / `index.html` | @ 11:53 | @ 12:13–12:14, 174 887 B / **76 048 601 B** / 6 208 B | trunk-rebuilt; byte-identical size — this lane changed no Rust, as expected |
| **served by 6118** | — | `GET /🚀️boot.js` → **200, 58 106 B**; `GET /🎞️frame-worker.js` → **200, 1 137 994 B**; `GET /semio-framework-os-renderer-wgpu_bg.wasm` → **200, 76 048 601 B**; `GET /semio-framework-os-renderer-wgpu.js` → **200, 174 887 B** | the running surface is serving this lane's bundle, and the markers above were grepped out of the **downloaded** bytes, not the staged ones |

6018 was not touched.

**Not verified by this lane:** a live boot #6 in the browser. Browser tools were out of scope, so the
evidence is bundle-level (the served bytes carry the fix), the thirteen unit cases, and the native compile
timings — not a rendered `runtime-ready`. What the next boot should show: the status line moving through
`wasm-compile 2 % … 100 %`, `renderer-runtime 65 %` no longer being able to end the boot, `[DEBUG]
boot-phase gpu-platform <n> ms` naming the real cost of the phase that was being killed, and — on the
second load of an unchanged artifact — `wasm-instantiate:cached 72.5 MB`.

## 9. Files changed

| file | change |
|---|---|
| `🎯️targets/🧊️wgpu/🫀️boot-liveness/🟦️.ts` | **new** — the shared boot liveness law: `FRAME_WORKER_BOOT_LIVENESS_POLICY`, `bootPhaseCeilingMs`, `evaluateBrowserBootLiveness`, `describeBrowserBootSilence`, `describeBrowserBootPhase` (en/de) |
| `🎯️targets/🧊️wgpu/🗄️wasm-module-cache/🟦️.ts` | **new** — IndexedDB cache of the compiled renderer `WebAssembly.Module`: `rendererArtifactTag`, `readCachedRendererModule`, `writeCachedRendererModule`, `evictCachedRendererModule`; fail-soft and 5 s-bounded throughout |
| `🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `boot-phase` message kind; `lastLivenessAtMs` / `bootStage` / `bootPhase` / `locale` state; `armBootStallTimer` → self-rescheduling `armBootWatchdog` + `judgeBootLiveness` + `witnessWorker`; `FRAME_WORKER_BOOT_STALL_TIMEOUT_MS` now mirrors the policy; `BrowserFrameFallbackState` gains `bootPhase`/`bootPhaseElapsedMs`/`bootStage`/`bootSilentForMs` |
| `🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | `declarePhase` + `declaredStep`; `monitoredSuspension` declares before it blocks and withdraws with the real elapsed; `bootDeclarationsOpen` closes declarations at `booted` so the asset pump posts none; `wasm-instance` replaced by `instantiateRendererWasm` (`wasm-artifact` → `wasm-cache-read` → `wasm-compile` → `wasm-instantiate`) and `compileRendererModule` (`compileStreaming` over a counting `TransformStream`, 2 % progress buckets); `renderer-bootstrap`/`renderer-finish`/`interactive-job-registry` now declared steps; liveness cadence read from the shared policy |
| `🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | the fallback panel renders `Boot stage: … · silent for … ms` and the `describeBrowserBootPhase` line, en and de |
| `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml` | `[watch]` now names `../../🫀️boot-liveness` and `../../🗄️wasm-module-cache` |
| `🧪️tests/📨️browser-frame-transport/🟦️.ts` | **+13 cases** — the liveness law, the phase-ceiling table, the 120 s declared-phase survival, the silent-Worker termination with the phase named, en/de diagnosis, the source contract, the vitest-fake-timer replay, and the module cache's identity + fail-soft behaviour |
| `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js` | regenerated bundle input (52 268 → 58 106 B) |
| `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` | regenerated bundle input (1 131 119 → 1 137 994 B) |
| `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu/*` | promoted by the live `trunk serve` on 6118 (pid 45306) |
| `$T/🐍️wasm-compile-timing-probe.mjs` | **new** ticket-folder probe: times `compileStreaming` / buffered `compile` / cached-module reuse against the live artifact |
| `$T/🐍️browser-boot-source-graph-probe.ts` | **new** ticket-folder probe: the boot bundle's esbuild input graph vs. the declared Nx source inputs |

No Rust changed. No `git` history was rewritten, no worktree was created, and 6018/6118 were never killed.
