# Lane G — shard liveness (busy vs dead), boot resilience, per-plugin load deadline

Opus implementer, ticket `26/09/05/S-END-TO-END`. All paths absolute from the repo root.

## Headline

The fatal boot was **two** faults stacked, and only one of them was the watchdog:

1. **`SHARD_WORKER_URL` pointed at a route that does not exist.** `🔌️PluginRuntime/🟦️.tsx` declared its own
   ASCII transliteration of the distribution route. Vite answered it with the SPA fallback, a module
   `Worker` refuses an HTML body, and that reaches the page as a parent-side `error` event **carrying no
   message at all** — which is why the boot log had four anonymous `shard N worker error Event` lines and
   ~60 unexplained `timeout loading <plugin>` faults descending from them. Measured against the live
   served shell:

   | URL | status | content-type |
   |---|---|---|
   | `/plugin-modules/_shard/🟨️shard-worker.js` (old) | 200 | `text/html` |
   | `/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js` (canonical) | 200 | `text/javascript` |

2. **The watchdog could not tell busy from dead.** `checkHeartbeats` compared `lastHeartbeatAtMs >=
   oldestPendingStartedAtMs` — an absolute timestamp against a *different* request's start — so a worker
   that beat once and then wedged forever looked healthy, while a worker legitimately parked inside one
   multi-second `await import()` of a multi-MB wasm component was killed the moment an unrelated newer
   request became the oldest pending.

Both are fixed, plus in-worker fault reporting, a boot-time JSPI probe, a single-retry boot path, and a
progress-proportional per-plugin load deadline.

## File:line changes

### Liveness rule and policy — `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`
- `:409-415` `SHARD_LIVENESS_POLICY` — the ONE schema-owned record (`heartbeatTimeoutMs 5000`,
  `missedLimit 3`, `progressIntervalMs 1000`, `pluginLoadIdleTimeoutMs 30000`, `pluginLoadCeilingMs
  300000`). Replaces the former `DEFAULT_HEARTBEAT_TIMEOUT_MS`/`HEARTBEAT_MISSED_LIMIT` literals.
- `:424`, `:434` `ShardHeartbeatState.lastLivenessAtMs` — new field, `-Infinity` on a fresh shard.
- `:440-483` `ShardLivenessWindow` / `ShardLivenessDecision` / `evaluateShardLiveness` — the whole
  decision as one pure function over a JSON-replayable window. Silence is measured from
  `max(lastLivenessAtMs, oldestPendingStartedAtMs)`; `missedLimit` consecutive silent windows terminate.
- `:485-489` `isShardLostError` — names the one failure the boot may retry.
- `:495-508` `describeShardWorkerError` — turns a bare `ErrorEvent` into `message at filename:line:col`,
  with an explicit branch for the redacted, message-less kind.
- `:511-517` `formatShardWorkerFault` — one readable line out of a `worker-fault` payload.
- `:520-560` `SHARD_JSPI_FAULT_CODE` (`plugin.runtime.jspi-unavailable`), bilingual
  `SHARD_JSPI_FAULT_TEXT` (`{en, de}`, no default language), `shardJspiAvailable`,
  `ShardJspiUnavailableError`, `assertShardJspiAvailable`.
- `:391-397` `InboundMessage` heartbeat variant gains optional `phase`; new `worker-fault` variant.
- `:997-1003` `handleMessage` dispatches `worker-fault` before the generic pending lookup.
- `:990` `handleMessage` calls `noteLiveness` for EVERY inbound message.
- `:2098-2112` `noteLiveness` / `recordHeartbeat`.
- `:2120-2129` `pollHeartbeatSab` now folds only a **monotonic advance** (`>` not `!==`) — the old `!==`
  accepted a regression, which a worker that never processed `attachHeartbeatSab` produces on every tick,
  and under the new rule that would have refreshed a dead shard's clock forever.
- `:2135-2158` `checkHeartbeats` is now a thin driver over `evaluateShardLiveness`.
- `:1180-1185` `spawnShard`'s `worker.onerror` logs and fails with `describeShardWorkerError(event)`.

### Schema + fixture (language-agnostic owner)
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️schema.json` — new,
  `semio.actor.shard-liveness.v1`: the policy record, the worker's boundary phases, the JSPI vectors and
  the watchdog timeline scenarios.
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️fixture/🔣️.json` — new: `policy`, `worker`
  (`activationPhases`, `progressPhase`), `jspi` (5 vectors), `scenarios` (10 timelines).

### Generated shard worker — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`
- `:35` `SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS = 1000` — interpolated into the worker; held equal to
  `policy.progressIntervalMs` by the shard client's own suite. Declared rather than read from the fixture
  at generation time because this module is in `⚙️vite.config.ts`'s import graph and is `Bun.build`-bundled
  for the bench harness, where neither a relative `import.meta.url` read nor an import of the shard client
  survives.
- `:132-168` (generated) `faultPhase`/`faultActorId`/`faultModuleUrl` breadcrumbs, `describeFault`,
  `reportWorkerFault`, and `self.addEventListener("error"/"unhandledrejection")` — registered BEFORE the
  `"message"` listener so a permissive test stub still keeps the dispatcher.
- `:181-183` (generated) `PROGRESS_HEARTBEAT_INTERVAL_MS`, `progressHandle`, `inFlightRequests`.
- `:213-247` (generated) `heartbeat(phase)` — one beat door for every liveness signal; `beginRequest` /
  `endRequest` — a `setInterval` beats `"progress"` while ANY request is outstanding. This is the entire
  busy-vs-dead discriminator: a worker parked on an `await` still runs its event loop and keeps beating; a
  worker wedged in synchronous guest code cannot run the callback and still dies after `missedLimit`
  windows. Guarded on `typeof setInterval === "function"` for bare VM contexts.
- `:293-301` (generated) `loadActor` beats `module-fetch` → `module-ready` → `actor-ready` and sets
  `load-bridge`/`instantiate`/`actor-ready` fault phases.
- `:371-374`, `:404-406`, `:446-454` (generated) request lifecycle: beat, `beginRequest`, phase tagging
  (`first-step` vs `turn`), `reportWorkerFault` in the catch, `endRequest` in `finally`.
- `:322-331` (generated) `replyError` cross-realm fix: an `Error` thrown in another realm is not
  `instanceof Error` and `JSON.stringify`s to `"{}"` — exactly what the host was told for the faults it
  most needs. Duck-typed on `stack`/`message`. **This made a pre-existing red assertion green** (verified
  identical behaviour at HEAD with `🔨️shard-liveness-fault-probe.ts`, so the red was not mine).
- `:295` (generated) `attachHeartbeatSab` tolerates a null `sab`.

### Shell — `🔌️PluginRuntime/🟦️.tsx`
- `:87` imports the canonical `SHARD_WORKER_URL` from `🎭️actor/🧵️shard-runtime/🟦️.ts`; `:205-216` the
  local dead literal is gone (see Headline #1).
- `:280-287` `PLUGIN_BOOT_SHARD_LOST_FAULT = "plugin.boot.shard-lost"` + `PluginBootShardLostError`
  (carries `code`, so `windowFaultFromError` names it).
- `:290-300` `notePluginLoadProgress` / `pluginLoadProgressAt` — per-plugin proof-of-progress clock.
- `:309` `getShardClient()` calls `assertShardJspiAvailable()` ONCE on the main thread before the first
  worker exists.
- `:1110-1114` `loadPluginModule` reports progress at each of its own await boundaries.

### Shell — `🛠️ShellHelpers/🟦️.tsx`
- `:1394-1405` `PLUGIN_LOAD_IDLE_TIMEOUT_MS` / `PLUGIN_LOAD_CEILING_MS` from the policy (no literals) and
  the pure `pluginLoadRemainingMs(startedAtMs, lastProgressAtMs, nowMs, …)`.
- `:1420-1445` `loadPluginModuleResilient` re-arms its deadline instead of firing once at a fixed offset:
  an **idle** budget that moves forward with observed progress, bounded by an absolute ceiling.

### Shell — `🏛️ShellHost/🟦️.tsx`
- `:2258-2283` `establishPrimaryWithShardRetry` — one watchdog kill is not a fatal boot; the session is
  established once more against the rebuilt shard, and only a SECOND loss fails, as
  `PluginBootShardLostError`. Every other boot error propagates untouched and unretried.
- `:2315`, `:2333` wired into `installPlugin`.

### Fixture/test upkeep caused by the above
- `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️fixture/🔣️.json` +
  `🧬️schema.json` — `ShardHeartbeatState` gains `lastLivenessAtMs` (worker shell 304 → 320 bytes,
  recomputed from the fixture's own `recordBytes`/`fieldBytes` model), inbound gains `phase` and the
  `worker-fault` variant, `workerReplyKinds` gains `worker-fault`, `currentFaultTraces` updated.
- `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts:244-252, 281-286` — the inventory now reads the
  EMITTED worker bytes (the return is a template *expression* now that it interpolates the policy); the
  AST check still fences the shape. `addEventListener` stubs filter to `"message"`.

## Tests

Language-agnostic JSON fixture → real `ShardClient` → independent oracle, three-way agreement.
`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:4600-4760`:

| Test | What it proves |
|---|---|
| mirrors the schema-owned policy record in every consumer of it | `SHARD_LIVENESS_POLICY` ≡ fixture `policy` ≡ `SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS` |
| serves the shard worker from the schema-owned distribution route | `SHARD_WORKER_URL` ≡ `MODULE_PLUGIN_ROUTE`/`MODULE_SHARD_DIRECTORY`/`SHARD_WORKER_FILE`; no spawner may carry a private path |
| classifies JSPI availability exactly as the fixture's vectors say | 5 vectors, against an independent restatement of the rule; typed bilingual fault |
| names a worker onerror instead of logging a bare Event | `describeShardWorkerError` on real/redacted/nested events |
| agrees with the independent oracle and the fixture on every watchdog timeline | 10 timelines replayed against the real client (fake time, fake worker), a flat simulation written from the prose rule, and the fixture's recorded expectations |
| beats at every generated-worker activation boundary and tickers on through a stalled import | drives the REAL generated worker in a `vm` with a held-open import |

```
$ cd 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript && bun ./📜️script.ts test exhaustive --reporter=verbose -t "liveness watchdog"

 ✓ ../../📮️shard-client/🟦️.ts > ShardClient liveness watchdog (schema-owned timelines) > mirrors the schema-owned policy record in every consumer of it 7673ms
 ✓ ../../📮️shard-client/🟦️.ts > ShardClient liveness watchdog (schema-owned timelines) > serves the shard worker from the schema-owned distribution route, not a transliteration of it 1100ms
[DEBUG] ShardClient JSPI vectors=jspi-present:true,suspending-missing:false,promising-missing:false,both-missing:false,no-webassembly:false
 ✓ ../../📮️shard-client/🟦️.ts > ShardClient liveness watchdog (schema-owned timelines) > classifies JSPI availability exactly as the fixture's vectors say 163ms
[DEBUG] shard 0 worker error: top-level throw at worker.js:18:9
 ✓ ../../📮️shard-client/🟦️.ts > ShardClient liveness watchdog (schema-owned timelines) > names a worker onerror instead of logging a bare Event 162ms
[DEBUG] ShardClient liveness timelines replayed against client+oracle=10
 ✓ ../../📮️shard-client/🟦️.ts > ShardClient liveness watchdog (schema-owned timelines) > agrees with the independent oracle and the fixture on every watchdog timeline 447ms
[DEBUG] ShardWorkerLiveness activation beats=request,module-fetch,progress,progress,module-ready,actor-ready
 ✓ ../../📮️shard-client/🟦️.ts > ShardClient liveness watchdog (schema-owned timelines) > beats at every generated-worker activation boundary and tickers on through a stalled import 1800ms

 Test Files  1 passed | 8 skipped (9)
      Tests  6 passed | 199 skipped (205)
```

Full package suites:

```
$ cd 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript && bun ./📜️script.ts test exhaustive
 Test Files  9 passed (9)
      Tests  205 passed (205)
   Duration  44.11s

$ cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript && bun ./📜️script.ts test quick
 Test Files  3 passed (3)
      Tests  267 passed (267)
   Duration  25.02s
```

Bundle check (`bun build --target=browser` on `🏛️ShellHost/🟦️.tsx`): 3 errors, all pre-existing
`?raw` Vite-only suffixes in `📃️UiDocumentStore`, none from this lane's imports.

## Boot proof — served shell, port 6076

```
$ S_OS_PORT=6076 bun ./📜️script.ts dev s served
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages)
VITE v7.3.6  ready in 10577 ms
➜  Local: http://127.0.0.1:6076/
```

Probe: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/🔨️lane-g-boot-probe.ts` (Playwright,
`--enable-features=WebAssemblyJavaScriptPromiseIntegration`, waits for
`document.documentElement.dataset.semioOsReady === "s"`).

**Before (coordinator's 14:05 boot)** → **after (this boot)**, from 125 captured console/page events:

| Symptom | before | after |
|---|---|---|
| `shard N worker error Event` | 4 | **0** |
| `shard N terminated` / watchdog kill | fatal | **0** |
| `timeout loading <plugin>` | 4+ | **0** |
| React mounted | no (empty body) | **yes** |

Body text now renders real shell chrome:
`"Fullscreen / No plugins loaded / Display / Remote: detached / No one else is here / Settings /
Marketplace / Command / Agent disconnected"` — before it was empty.

The shard worker loaded from the canonical route, `s#1` activated, its jco bridge instantiated, and the
**first real guest turn ran**, failing inside the plugin's own Rust — named exactly by the new in-worker
reporting:

```
thread '<unnamed>' (1) panicked at …/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:5739:64:
app-definition.interactive-job-classification: unclassified interactive command
  's.space.studio@1/*#editor:setAppRegistrations'
[DEBUG] shard 0 worker fault [handler/first-step] actor=s#1 module=/🔌️plugin-modules/🪐️s/🌉️bridge.js: unreachable RuntimeError: unreachable
[DEBUG] PluginRuntime: actor s#1 trapped: shard 0 worker fault [handler/first-step] actor=s#1 …
[DEBUG] program worker s#1 error type=RuntimeError framesBytes=n/a
[DEBUG] PluginRuntime: turn failed for actor s#1 Error: unreachable
Framework OS boot failed Error: unreachable
```

The readiness beacon is therefore still `null` — correctly, and for a reason that is now **named**: the
cached `semio_s_plugin_space` core traps against today's host. That is a rebuild, not a liveness fault,
and the single-retry path deliberately does NOT retry it (it is not a shard loss).

`vite` pid 52566 (parent 41266) killed afterwards; 6076 free, 6070 untouched throughout.

## Remaining console noise (other lanes)

- ~20 `plugin.descriptor-unavailable` (HTTP 404) + 4 `plugin.descriptor-identity-mismatch` — the Wave 2
  catalog rebuild / lane B census.
- `AppRouter excluded plugin "demonstrator": surface.contribution-not-permitted …` — repeated but no longer
  fatal (the router now excludes rather than dying); lane H owns the manifest.

## Blockers / notes

- **Machine load.** Peers' `rustc` builds (`semio_s_plugin_stdio`, `semio_framework_plugin`, …) pushed
  load average from 20 to 70-115 mid-run; `/` on BOTH 6076 and 6070 failed to answer within 60 s during
  that window (`curl --max-time 540` returned nothing at load ~90). The successful probe ran when it
  eased. Vite itself was idle (2.7 % CPU) — starvation, not a server fault.
- A repo-wide `tsc --noEmit` over the touched files could not be completed: the ad-hoc program did not
  pick up `allowImportingTsExtensions`, so its 157 diagnostics were all `TS5097` noise. Covered instead by
  the browser bundle check and the two green vitest packages.
- Scratch probe kept as an input file: `🔨️shard-liveness-fault-probe.ts` (HEAD-vs-now worker fault
  comparison) and `🔨️lane-g-boot-probe.ts` (the Playwright boot probe).
