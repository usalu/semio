# shard termination — why `shard 0` died silently at boot #10, and what now says so

Lane: React shard client/worker boot path. Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-10.

Live finding under investigation (`📓️runtime-verification-2026-09-09.md` § "09:55 boot #10"): after the nine
`[DEBUG] hot-swap flow-extension-*` lines the console showed

```
PluginRuntime: shard 0 lost, restoring actors: procedural#1
turn failed for actor procedural#1: Error: shard 0 terminated (ShardClient.terminate…)
Framework OS boot failed: Error: plugin-ui.native-owner-required   (🔌️PluginRuntime/🟦️.tsx:1395)
```

with no panic text and no wasm stack; the page reported `No plugins loaded`.

## 1 Root cause

### 1.1 The worker never died. The host killed a healthy one.

`shard 0 terminated` is produced by exactly one site — `ShardClient.terminate`
(`🎭️actor/📮️shard-client/🟦️.ts`), reached only from `checkHeartbeats` when `evaluateShardLiveness`
returns `terminate: true`: `missedLimit` (3) consecutive `heartbeatTimeoutMs` (5 000 ms) windows in which the
worker sent **nothing**. There was no crash to report because there was no crash.

`evaluateShardLiveness`'s own pre-existing doc states the premise it rests on: liveness "is now proven
CONTINUOUSLY by the worker's progress ticker (which can only fire while its event loop is actually
running)". **That premise is false for a guest turn.** The generated worker's ticker is a
`setInterval(…, 1000)` on the worker's single thread; JSPI suspends the guest only at a host import, so a
compute-bound `reactor.poll` runs to completion with the event loop blocked and the ticker unable to fire.
Silence therefore does not mean death — it means *the guest is running*, and the watchdog cannot tell the
two apart.

**Measured**, headlessly, against the live staged components (`🐍️shard-component-probe.mjs`, node 24 with
`--experimental-wasm-jspi`, a 1 s `setInterval` counter standing in for the worker's progress ticker;
`🗑️generated/shard-probe-ticker.txt`):

| module | `createActorApi` | ticks during it | `poll #1` | ticks during it | `poll #2` |
|---|---|---|---|---|---|
| `🌊️flow` | 1 257 ms | 1 | **2 964 ms** | **0** | 1 ms |
| `🔤️flow-extension-primitive` | 48 ms | 1 | **3 087 ms** | **0** | 2 ms |
| `🌀️procedural` | 1 100 ms | 1 | **3 362 ms** | **0** | 1 ms |

The `await import()` + instantiate phase *does* let the ticker run (1 tick each) — that half of the
pre-existing fix works. The FIRST `poll` does not, at all, in any module: 0 ticks over ~3 s, every time. The
second poll of the same actor costs **1–2 ms**: a ~1 500× difference between an actor's first turn and every
later one, priced by the watchdog as if they were the same class of work.

Native node on an idle machine is the *fast* case. The live boot ran in a hidden browser pane (mobile
emulation + background QoS, `📓️hidden-browser-pane` finding) on a machine under swap pressure, with shard 0
hosting several of the boot closure's actors; one first turn exceeding 3 × 5 000 ms there is entirely
ordinary. That is the whole silent death.

**Not the cause** — checked and cleared:

| hypothesis | verdict |
|---|---|
| the 09:41 component is broken / traps on instantiation | **no** — the exact 09:41 build (`81 214 198` B, from the Nx cache `8331769824652513898`) instantiates and polls cleanly: `createActorApi` 525 ms, `poll #1` 1 232 ms, polls #2/#3 1 ms (`🗑️generated/shard-probe-0941.txt`). So does the currently staged build (`🗑️generated/shard-probe-procedural.txt`). |
| a `WebAssembly.Memory({initial, maximum})` mismatch from the `--max-memory` declaration | **no** — nothing declares a `WebAssembly.Memory`; the core module carries `min 190 / max 8192` pages and jco instantiates that (`📓️guest-memory-2026-09-10.md` §, independently confirmed by the probe running to completion). |
| an OOM inside the worker | **not needed to explain it**, and the probe reaches only ~1.4 GB RSS for a five-module closure in one process (`🗑️generated/shard-probe-closure.txt`) — worth watching, but the shard died from the watchdog, not from an allocator. |

### 1.2 The React boot then replaced that cause with its own teardown's

`🔌️PluginRuntime/🟦️.tsx`'s `createApp` sets `lifecycleByInstance` **before** the `open` turn and
`uiOwnerByInstance` only **after** it returns. A trapped/rejected open therefore leaves a lifecycle with no
UI owner, and the rejection path did:

```ts
return opening.then(value => …, async error => { forget(); await handle.destroyApp(instanceId); throw error; });
```

while `destroyApp` threw `new Error("plugin-ui.native-owner-required")` for exactly that
lifecycle-without-owner state. The `await` let the teardown's rejection escape first, so the original
`shard 0 terminated` never reached the shell. This is the exact twin of the wgpu bug
(`📓️wgpu-shell-boot-2026-09-10.md` §1.2/§2.2), in a file that lane does not own.

It had a second consequence: `🏛️ShellHost/🟦️.tsx:3227` already retries a boot **once** on the rebuilt shard
when `isShardLostError(bootError)` — and `plugin-ui.native-owner-required` does not match that predicate, so
the one retry that was designed for exactly this fault never ran.

## 2 Fixes

### 2.1 `🎭️actor/📮️shard-client/🟦️.ts` + `🧬️schema` + `🧫️fixtures` — price the first turn honestly

- **`SHARD_LIVENESS_POLICY.firstTurnTimeoutMs = 30 000`**, added to the schema-owned policy record
  (`🧬️schema/🔣️.json` `policy`, `🧫️fixtures/🔣️.json` `policy`) and mirrored in the TS constant that the
  in-source suite asserts field-for-field equal to the fixture. Aligned with the already-schema-owned
  `pluginLoadIdleTimeoutMs`, which measures the same one-off cost one layer up.
- **`evaluateShardLiveness`** takes `firstTurnTimeoutMs` + `oldestPendingIsFirstTurn` and measures the
  silence AND the miss spacing against `max(heartbeatTimeoutMs, firstTurnTimeoutMs)` while the oldest
  in-flight request is an actor's first turn. Everything after the first turn is unchanged. A first turn
  that really is wedged still dies, on its own ladder.
- **`ShardClient` first-turn bookkeeping**: `PendingEntry` now carries `kind` + `firstTurn`;
  `actorsPastFirstTurn` is filled when a `turn` settles and cleared on `activate` (a fresh activation is a
  fresh guest with the same one-off cost in front of it).
- **`describeShardSilence` / `ShardSilenceReport`** (new, exported, pure): composes the terminating
  rejection from state the client already holds — how long the worker was silent, the last phase it
  reported, and every outstanding request with its actor, age and first-turn flag. `terminate(index, detail)`
  carries it, `checkHeartbeats` logs it, and it keeps the `shard <n> terminated` prefix `isShardLostError`
  matches, so ShellHost's single retry still triggers.
- **`describeShardMessageError`** (new, exported) + `ShardWorkerLike.onmessageerror` + a
  `worker.onmessageerror` handler in `spawnShard`. A failed structured-clone deserialization was previously
  observed by nothing at all: the request or result it carried simply vanished and the shard went quiet.
- **`settleFailedInstanceOpen`** moved here as its single owner (see 2.3).

`worker.onerror`, the generated worker's `self.addEventListener("error"/"unhandledrejection")` →
`worker-fault` channel, and the handler `catch` that answers an `activate`/`turn` rejection on BOTH channels
(`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`) already existed and already carry instantiation failures —
`WebAssembly.instantiate` rejections and allocation errors included. `messageerror` was the one live gap.

### 2.2 `🔌️PluginRuntime/🟦️.tsx` — stop masking the cause

- `destroyApp`: `if (!lease || !owner)` now joins the existing "nothing owned to retire" branch
  (`registry.cancel(actorId); releaseInstanceMaps(...)`). An instance whose `open` never bound a UI owner
  holds no retained UI; cancelling the actor is the whole correct teardown.
- `createApp`'s rejection path routes through `settleFailedInstanceOpen(error, () => handle.destroyApp(id))`,
  so the cleanup always runs, a cleanup fault is reported on its own `[DEBUG]` line, and the caller always
  receives the ORIGINAL error.

### 2.3 `🐚️plugin-bridge.ts` — one owner for the failed-open settlement

The wgpu lane's local `settleFailedInstanceOpen` is deleted and re-exported from `📮️shard-client/🟦️.ts`
(both bridges import that module already). Both targets had grown the same masking bug independently; the
repair now exists once.

## 3 Tests

| test | file |
|---|---|
| three new schema-owned timelines — a first turn outlives the ordinary ladder; a first turn past its OWN ceiling still terminates; a request after the first turn is back on the ordinary ladder — replayed against the real client, the fixture's recorded expectations and the independent oracle | `📮️shard-client/🧫️fixtures/🔣️.json` + `🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` (`pending-first-turn` timeline event, oracle + driver both extended) |
| `evaluateShardLiveness` unit vectors + a real client: 4 s of silence on an actor's FIRST turn leaves the shard alive, the SAME 4 s on its second turn kills it | same |
| a `messageerror` becomes a named shard loss and rejects the in-flight request (it went unobserved before) | same |
| `describeShardSilence` names the silence, phase and every outstanding request; a live watchdog termination carries it into the turn's rejection; `isShardLostError` still matches it | same |
| `settleFailedInstanceOpen` rejects with the ORIGINAL fault even when the cleanup throws `plugin-ui.native-owner-required`, and still runs the cleanup | same (the wgpu bridge's own test for the re-export still passes) |
| policy mirror (`SHARD_LIVENESS_POLICY` ≡ fixture `policy`) and the ajv validation of fixture-against-schema cover the new field automatically | pre-existing, unchanged |

Language-agnostic pairing: the timelines live in `🧫️fixtures/🔣️.json` under `🧬️schema/🔣️.json` and are
driven three ways — the real `ShardClient`, the fixture's own recorded expectations, and a flat oracle
simulation that shares no code with `evaluateShardLiveness`.

## 4 Checks

```
bunx vitest run --config 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts -t ShardClient
  → 📮️shard-client/🟦️.ts  158 tests | 0 failed | 44 skipped        (113 passed of the ShardClient filter)

bun ./📜️script.ts test-browser-worker   (wgpu package, incl. the settleFailedInstanceOpen re-export test)
  → Test Files 4 passed (4) · Tests 51 passed (51)          [🗑️generated/shard-browser-worker-tests.txt]

bun ./📜️script.ts test long             (react engine, incl. 🔌️PluginRuntime's in-source suite)
  → Test Files 20 passed (20) · Tests 795 passed (795)      [🗑️generated/shard-react-engine-long.txt]

bun x tsc --noEmit -p …/🎯️targets/⚛️react/tsconfig.json
  → 0 errors in 🔌️PluginRuntime/🟦️.tsx, 📮️shard-client/🟦️.ts, 🐚️plugin-bridge.ts and the test file
    (1 021 pre-existing errors elsewhere in the repo-wide program, none from this lane)
```

**Pre-existing failures NOT from this lane** — the full actor-package run reports 12 failures in
`🚪️lifetime/🟦️.ts`, `🚪️lifetime/🩹️patch/🟦️.ts`, `📤️return/🟦️.ts`, `📤️return/📨️response/🟦️.ts` and
`🪪️activation/🚪️instance/📥️output/🟦️.ts`, all of the form
`can't resolve reference https://semio.tech/schema/framework/value/schema.json#/$defs/NonZeroU64` — a peer
lane's schema `$ref` churn. None of those files is touched here and the shard-client file passes clean.

## 5 Reproduction harness

`🐍️shard-component-probe.mjs` (ticket root, kept) is the headless mirror of what `🟨️shard-worker.js` does:
it imports a staged module's generated `🌉️bridge.js`, calls `createActorApi(actorId, activationGeneration)`
and drives `poll` turns, several modules in ONE process the way one pooled shard worker hosts several
actors, with a 1 s ticker counter proving whether the worker *could* have heartbeated in each phase.

```
node --experimental-wasm-jspi 🐍️shard-component-probe.mjs <turns> <staged-module-dir>…
```

To replay the exact 09:41 component, copy it (and the sibling `🪞️vendor` directory it imports) out of the
Nx cache first — a symlink resolves to the cache path and the vendor import then fails:

```
cp -Rc .nx/cache/8331769824652513898/…/dist/dev/🔌️plugin-modules/🌀️procedural  $S/probe0941/🌀️procedural
cp -Rc 🧰️…/🧑‍💻dev/🔌️plugin-modules/🪞️vendor                                   $S/probe0941/🪞️vendor
node --experimental-wasm-jspi 🐍️shard-component-probe.mjs 3 $S/probe0941/🌀️procedural
```

## 6 What still needs the next restage

**Restage required: yes** — every change here is TypeScript that ships inside the browser bundles
(`🔌️PluginRuntime`, the shard client, the wgpu plugin bridge). No wasm rebuild is needed: the plugin
components are unchanged and proven good by §1.1.

After the restage, boot #11 should show, in place of the old two lines:

- if a shard is still terminated: `shard 0 terminated by the host watchdog: the worker was silent for
  N ms, last reported phase "…"; outstanding: turn (first turn) procedural#1 started N ms ago. …` — and
  ShellHost's one-shot retry on the rebuilt shard will now actually run, because that message matches
  `isShardLostError`;
- if the first turn simply took longer than 15 s: nothing at all — it now has its own 30 s window
  (`missedLimit` of them) instead of the ordinary 5 s one;
- if anything else fails during `open`: that fault's own message, never `plugin-ui.native-owner-required`.

Open, and **not** this lane's to fix: an actor's first turn costing 3 s of *blocked* worker event loop is
itself a guest-side cost (one-off initialization inside the component, plus whatever the 09:41 memory
diagnostics add per turn). The host now survives it and names it; making it small, or making it yield, is
guest work. The five-module closure reaching ~1.4 GB RSS in one process is the other number worth carrying
into the memory lane.
