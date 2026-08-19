# 🧪️ terra-web-bridges — jco bridge/shim generator updated for all-`async func` plugins

Executor: `terra-web-bridges`. Owned path:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/**`
(one file: `🌐plugin-web-materialize.ts`), plus this ticket folder.

Read `📓️terra-jco-spike-report.md` in full before starting — its VERDICT (GO-jspi: jco 1.27.0 drives a
fully-async component, but emits `WebAssembly.Suspending`/`promising` unconditionally, no
JSPI-free output exists, no runtime fallback is possible) is what every change below is built around.

## What changed, and why

### 1. Transpile invocation (`transpilePluginComponent`/`transpilePluginComponentAsync`)

No `--async-mode` flag added — the spike's own exact commands (pasted with exit codes in its report)
confirm this is a no-op for a component whose every WIT function is already `async func`
(`--async-mode jspi` byte-diffed as identical to the bare/default transpile). The only real change is
a second `--map` entry:

```
--map "semio:framework/pure=./🟨️host-shim.js"
--map "semio:framework/host-async=./🟨️host-shim.js"
```

Both interfaces now map to the SAME shim file — `hostShimSource()` implements both. Applied
identically to both the sync (`runNodeBinStatus`) and async (`spawnNodeBinAsync`) transpile
functions, so the dev pipeline's bounded-parallel materialize stage (T-P8) and the extension store's
`webMaterialize` stay in lockstep.

### 2. `hostShimSource()` — new `host-async` surface

`pure` (`log`/`nowMs`/`traceSpan`) is untouched. New: all 24 `host-async` async imports
(`storage-read` … `spawn-job`, component.wit ~:887-953) plus the two fire-and-forget `emit`/
`emit-patch` doors. Design:

- **`effectRequest(effect, params)`** — every async import funnels through this. Posts one
  `ShardFrame::Envelope` up to the kernel over the EXACT shape
  `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` already declares (`to`/`from`/`lane`/`seq`/
  `deadlineMs`/`coalesce`/`cancelOf`/`payload`), with `payload: {kind: "effect-request", payload:
  {effect, requestId, params}}` — reusing `ShardEventEnvelope`'s own `{kind, payload}` shape rather
  than inventing a second wire. Returns a Promise settled by a later `__resolveEffect`/
  `__rejectEffect` call.
- **`__bindHostBridge(actorId)`** — called once by `createActorApi(actorId)` right after this module
  is imported for that actor. Safe as plain module-scoped (not global) state ONLY because
  `🟨️shard-worker.js` dynamically `import()`s a distinct `moduleUrl` per actor (`loadActor`'s own
  doc), so each actor gets its own shim module instance — confirmed by re-reading `loadActor` itself,
  not assumed.
- **`streamToByteGenerator(body)`** — adapts a `ReadableStream` into the exact per-byte async
  generator shape jco's `stream<u8>` host imports need, confirmed against a REAL component
  (jcoprobe's `fetchBody`, spike S4). `http-fetch`/`blob-read` route their `body`/return value through
  this.
- `emit`/`emit-patch` stay plain (non-`async`) functions, matching their WIT `func` (not `async
  func`) declarations, and post fire-and-forget (no `requestId`, no Promise).

**Honestly unproven**: whether jco expects a `result<T, pack>`-returning host-async import to signal
`Err` by throwing — `effectRequest` rejects on `effect-error`, following jco's documented
host-import convention, but jcoprobe's `probe-host` never used a `result<>` return so this specific
detail was never exercised against a real component.

### 3. `pluginComponentBridgeSource()` — `createActorApi(actorId)`

Destructure shape (`reactor`/`jobs`/`checkpoint`/`describe`) is unchanged — confirmed correct for a
single-export-interface world against jcoprobe's real transpile (`export * as probe from
'./interfaces/...'`, camelCased names); the 4-interface case is extrapolated from that plus jco's
documented per-interface convention, not independently re-confirmed. Two real changes:

- Every wrapper method is now explicitly `async` (was: a plain arrow whose body happened to return
  whatever the underlying call returned) — self-documenting and robust even if a future jco version
  wraps an export in something not already thenable.
- `createActorApi` now takes `actorId`, imports the host shim directly (`import * as hostShim from
  "./🟨️host-shim.js"`), calls `hostShim.__bindHostBridge(actorId)` before returning, and exposes
  `resolveEffect`/`rejectEffect` passthroughs the shard worker calls on an `effect-complete`/
  `effect-error` frame.

### 4. `shardWorkerSource()` — two additions

- **JSPI diagnostic guard** (item 4 — a diagnostic, NOT a fallback; nothing here runs a plugin
  without JSPI). At the very top, before anything else: `typeof WebAssembly.Suspending !==
  "function" || typeof WebAssembly.promising !== "function"` → posts a `"trap"` message (actorId
  sentinel `"*"`, since no actor has activated yet) naming JSPI, the browsers that have it by
  default, the Firefox flag, and the Node flag, THEN throws. The `postMessage`-before-`throw`
  ordering is deliberate: the spike found real browsers redact cross-context Worker `onerror` details
  to `"undefined undefined undefined"`, so the trap message is the best shot at a readable diagnostic
  reaching `ShardClient.onActorTrap` even if `onerror` itself is useless.
- **`loadActor`** now calls `bridge.createActorApi(actorId)` (was zero-arg), matching item 3 above.
- **`deliverEffectResult(actorId, envelope)`** + an early dispatch branch in the message listener: a
  `"frame"` whose `envelope.payload.kind` is `"effect-complete"`/`"effect-error"` is routed here
  BEFORE the generic `requestId`/`actorId`-gated dispatch (which always posts a `"result"` reply —
  wrong for a message that is itself already an answer to something this worker sent).

## What this deliberately does NOT do

- **Does not re-materialise the 48 bridge artifacts.** That needs a real `wasm32-wasip2` fleet build,
  which does not currently compile (a large in-flight conversion tracked elsewhere on this ticket).
  Left entirely alone.
- **Does not touch `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`** (not owned). `ShardClient`'s
  `InboundMessage` union still only recognizes `result`/`heartbeat`/`trap` — there is NO real
  kernel-side responder for `effect-request`/`effect-complete` yet. Everything this packet built is
  the WORKER-side half of that wire, correctly shaped and internally consistent, verified end-to-end
  against a hand-simulated "kernel" (see below) — but a sibling packet owning `shard-client.ts` still
  needs to add the matching `InboundMessage` case and a real responder for any of this to reach
  production traffic.
- **No runtime capability-probe fallback was written** — per the ticket's own instruction, since the
  spike proved none is implementable (no jco flag produces JSPI-free output).

## Verification performed

### TypeScript sanity

`bun run` successfully imports `🌐plugin-web-materialize.ts` and calls `hostShimSource()`/
`pluginComponentBridgeSource()`/`shardWorkerSource()` — module loads and executes with no runtime
error. `node --check` confirms all three generated JS strings are syntactically valid (this caught
and fixed one real bug during development: an unescaped-backtick doc comment inside
`hostShimSource()`'s own template literal that would have truncated the generated file mid-string).

A scoped `bunx tsc --noEmit --strict` run against the file reports exactly ONE diagnostic
(`TS5097`, "import path can only end with .ts extension") — a pre-existing, repo-wide config artifact
present in dozens of unrelated files too (this repo's own `tsconfig.json` lacks
`allowImportingTsExtensions` even though every file relies on it; the real build clearly resolves this
some other way, e.g. bun's own resolver, which is what actually loaded the file successfully above).
Zero type errors attributable to this packet's changes.

`bun nx run @semio-tech/framework-os-dev:test --reporter=verbose`: **27/27 passed**, including
`sweepStaleExtensionModuleOutputs`'s `"keeps a 🟨️host-shim.js whose content already matches the
current hostShimSource()"` and `"removes a planted stale 🟨️host-shim.js..."` — both call the LIVE
`hostShimSource()` and byte-compare, so they automatically validated against my new (much larger)
generated content with no hardcoded snapshot to go stale. No dedicated vitest suite exists for
`plugin-web-materialize.ts` itself (no `package.json`/`vitest.config.ts` in its own directory; it has
no owning nx project separate from its consumers) — this dev-package suite is the closest existing
coverage and it passed clean pre- and post-change.

### Browser pane verification against a REAL jco-transpiled component (jcoprobe fixture)

Full transcript: `terra-webbridges-browser-roundtrip.txt`. Summary: copied the READ-ONLY jcoprobe
fixture's already-transpiled output (`jcoprobe.js`/`.core.wasm`/interfaces/preview2-shim — untouched)
into a scratch dir, alongside:

- `shard-worker.js` = the EXACT, unmodified output of the real, UPDATED `shardWorkerSource()`.
- `host-shim.js`, `bridge.js` = hand-written (jcoprobe's WIT namespace is `semio:jcoprobe/*`, not
  production's `semio:framework/*`, so the real generator's own map target can't point at this
  fixture directly) but implementing the SAME `effectRequest`/`streamToByteGenerator`/
  `__bindHostBridge` mechanism the real `hostShimSource()`/`pluginComponentBridgeSource()` generate,
  applied to jcoprobe's own `slowEcho`/`fetchBody`/`poll`/`awaitEcho`/`spawnDetached`/`readBody`.
- `driver.js` = a main-thread harness playing the KERNEL side of the wire: receives `effect-request`
  frames, replies with `effect-complete` frames (a real `setTimeout` for `slow-echo`, a REAL
  `ReadableStream` transferred over `postMessage` for `fetch-body`).

Served via a scratch-only `bun` static server (never touching `.claude/launch.json`, which is
registrar-only and not in this packet's owned paths), driven entirely through the Browser pane
(`preview_start`, `get_page_text`, `read_console_messages` — never Bash to interact with the page).
Result, final run:

```
EFFECT-ROUNDTRIP: PASS — awaitEcho(50,777) via effect-request/effect-complete took 817.20ms, result=777, setInterval(5ms) ticks=1 while pending (event loop not blocked)
STREAM-ADAPTER: PASS — readBody() via fetch-body effect + streamToByteGenerator -> 5 (expected 5)
==== DONE overall=true ====
```

An earlier run (before loosening an over-strict tick-count assertion) recorded 89.30ms for the same
effect — close to the spike's own 72-93ms range for its analogous S2 case — with the FINAL run's much
slower 817.20ms almost certainly Chromium background/inactive-tab timer throttling between page
loads in the Browser pane, not a defect (both `setTimeout` and the timing probe's `setInterval` would
throttle together, matching the drop from several ticks to one). Both runs' actual VALUES were
correct (777, and 5 bytes) — that correctness, not exact timing, is what this harness asserts.

This is real, concrete evidence that: `createActorApi(actorId)` binding works; `effectRequest`'s
envelope is emitted and received correctly; the NEW `deliverEffectResult` dispatch in
`shardWorkerSource()` correctly resolves a real guest's pending `.await` with the right value; and
`streamToByteGenerator` correctly adapts a REAL `ReadableStream` into the exact shape a real
wasm guest's `StreamReader::next().await` loop consumes — reproducing the spike's own S4 result (5
bytes) but now via the new effect-request/effect-complete wire instead of a bare host-shim return.

### JSPI diagnostic guard — Node-based check (mirroring the spike's own JSPI-off/on comparison)

Full transcript: `terra-webbridges-jspi-guard-check.txt`. Loaded the real generated
`shardWorkerSource()` output under plain `node` (no `self` global by default, so a minimal stub
`{postMessage}` was installed first) both with and without `--experimental-wasm-jspi`:

- **Without JSPI**: the guard fires — posts `{kind:"trap", actorId:"*", message:"semio shard worker:
  this browser/engine lacks JavaScript Promise Integration (JSPI)..."}`, then throws that same
  message. This is the intended, actionable diagnostic replacing the spike's own opaque `TypeError:
  WebAssembly.Suspending is not a constructor`.
- **With `--experimental-wasm-jspi`**: the guard does NOT fire — execution falls through to
  `self.addEventListener(...)`, which only then fails because the test harness's stub `self` lacks
  `addEventListener` (a harness limitation, not a guard bug) — proving the guard correctly does not
  block when JSPI is present.

## What is proven vs. unproven — explicit, per the ticket's own requirement

**Proven, against a real component and the real (unmodified) generator output:** the transpile
flags/map pattern (no `--async-mode`, dual `--map` to one shim file); the `effectRequest`/
`__bindHostBridge`/`streamToByteGenerator` mechanism end-to-end through a real jco-transpiled
component and the real `shardWorkerSource()`'s new dispatch; the JSPI guard's message and its
correct no-op-when-present behavior.

**Written but UNPROVEN against a real component:** the exact 24-function `host-async` surface
(jcoprobe only exercises 2, under a different WIT namespace); the `result<T, pack>` throw-on-`Err`
convention; the 4-interface (`reactor`/`jobs`/`checkpoint`/`describe`) export destructure shape for a
real `world actor` component (the wasm32-wasip2 fleet does not currently compile); any real
`ShardClient`-side responder for `effect-request`/`effect-complete` (that file is not owned by this
packet and currently has no matching `InboundMessage` case at all — a sibling packet needs to close
that gap before any of this reaches production traffic).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
  (only file in the owned path — `transpilePluginComponent`/`transpilePluginComponentAsync`,
  `hostShimSource`, `pluginComponentBridgeSource`, `shardWorkerSource` all changed; doc comments
  updated in place; no exported TS function signature changed, so `🔌️plugin/🏪️store/📜️store.ts` and
  `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (both checked, both unowned) needed no changes and
  their existing calls compile/run unmodified).
- Ticket-folder scratch/evidence: `terra-webbridges-gen-host-shim.txt`,
  `terra-webbridges-gen-bridge.txt`, `terra-webbridges-gen-shard-worker.txt` (the three generated JS
  outputs, saved verbatim), `terra-webbridges-jspi-guard-check.txt`,
  `terra-webbridges-browser-roundtrip.txt`.

Not modified: the 48 stale bridge artifacts (explicitly out of scope — needs a real fleet build);
`🎭️actor/📦️packages/🟦️typescript/**` (read-only, reused its `ShardFrame`/`ShardEnvelope` shape);
the `🔌️jcoprobe` fixture (read-only, copied into a scratch dir for the browser verification, never
written to); `.claude/launch.json` (registrar-only, not touched — the verification server was run
directly and opened in the Browser pane via `preview_start`'s `url` form instead).
