# 🧪️ terra-shard-effect-bridge — ShardClient answers `effect-request` frames

Executor: `terra-shard-effect-bridge`. Owned path:
`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/**` (touched: `🧵️shard-client.ts`,
`🧵️shard-runtime.ts`), plus this ticket folder.

Read `📌️important.md`'s U-PROGRAM RULINGS, `📓️terra-web-bridges-report.md` (the guest half — this
packet's whole job is closing the loop it left open), and `📓️terra-jco-spike-report.md` (async
component behavior) before starting. Matched `terra-web-bridges`' wire shape exactly — no second
protocol invented.

## The gap, and what closes it

Before this packet, `ShardClient`'s `InboundMessage` union only recognized `result`/`heartbeat`/`trap`.
A guest `.await`ing a host import (`http-fetch`, `blob-read`, …) posts an `effect-request` frame that
nothing ever answered — the guest's shim Promise (`🟨️host-shim.js`'s `effectRequest`) hung forever.

`🧵️shard-client.ts` now:
1. Extends `InboundMessage` with the exact `{kind:"frame", actorId, frame: ShardFrame}` shape the
   generator emits (`frame.envelope.payload` = `{kind:"effect-request", payload:{effect, requestId,
   params}}` — reusing `ShardFrame::Envelope`/`ShardEventEnvelope` verbatim, never a second wire).
2. Routes it in `handleMessage` **before** the generic `pending`-lookup path, mirroring
   `🟨️shard-worker.js`'s own `deliverEffectResult` ordering note (an inbound `"frame"` carries no
   `requestId` of its own and is never an answer this class is waiting on).
3. Exposes an injectable `onHostEffect?: HostEffectHandler` on `ShardClientOptions` — `(actorId, effect,
   params, signal) => Promise<unknown>`. `ShardClient` implements NONE of `http-fetch`/`blob-read`/etc
   itself (the `🎭️actor` crate/package stays free of `web_sys`/host assumptions, per the ticket's own
   naming-hazards rule) — the React host, the wgpu host, and tests each supply their own. **No handler
   installed → an immediate, synchronous `effect-error` `"no host effect handler installed"`** — proven
   by a dedicated test that asserts the reply exists WITHOUT ever awaiting a microtask, so there is no
   path through which this hangs.
4. Replies correctly: `effect-complete` on success, `effect-error` on failure (handler rejection →
   `error.message`), correlated by the guest's own `requestId`, posted directly to the owning shard's
   worker via `postMessage` (never through `send()`, which would wait for a `"result"` that never
   comes — the worker's `deliverEffectResult` sends nothing back).
5. **Teardown**: `failShard` (worker crash / `terminate()`) and `dispose(actorId)` both call a new
   `abortOutstandingEffects(actorId)` — aborts every in-flight `AbortController` for that actor's
   outstanding effects and clears the ledger immediately, so a later handler settlement (if the
   caller's `onHostEffect` implementation eventually resolves/rejects anyway) is recognized as STALE by
   `settleEffect` and posts nothing — never to a dead worker, never to a since-reactivated actor of the
   same id. `HostEffectHandler`'s `signal` parameter lets a real implementation (`fetch(url, {signal})`)
   genuinely cancel the underlying work, not just abandon it.
6. **Backpressure**: `maxOutstandingEffectsPerActor` (default 64, configurable) caps CONCURRENT
   unresolved effects per actor; a request beyond the cap is rejected immediately with a
   `ShardQuotaBreach`-shaped message (`quota: "outstandingRequests", limit, actual`) — mirrors the
   CONCEPT of `QuotaSchema.outstanding_requests` (`🎠️kernel/🦀️component.rs` ~:1022) on the client's own
   ledger, independent of it, per the ticket's explicit instruction not to invent a new vocabulary.

`🧵️shard-runtime.ts`'s `createPooledActorRuntime(options)` now threads `onHostEffect`/
`maxOutstandingEffectsPerActor` straight through to `ShardClient` (additive, both optional) — the ONE
other consumer of that factory today (`🎯️targets/🧊️wgpu/…/🐚️plugin-bridge.ts`, not my file) can opt in
without me touching it; omitting them preserves today's behavior exactly.

## Files touched (only owned paths)

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` — new `//#region
  🌉️HostEffect` (types: `ShardQuotaBreach`, `HostEffectHandler`), extended `InboundMessage`, new class
  fields (`onHostEffect`, `maxOutstandingEffectsPerActor`, `outstandingEffectsByActor`,
  `effectReplySeq`), `handleMessage`'s new `"frame"` branch, `failShard`/`dispose` now call
  `abortOutstandingEffects`, new `//#region 🌉️HostEffectBridge` (`handleInboundFrame`,
  `handleEffectRequest`, `settleEffect`, `abortOutstandingEffects`, `postEffectReply`,
  `replyEffectComplete`, `replyEffectError`), 6 new in-source vitest cases.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-runtime.ts` —
  `createPooledActorRuntime`'s options widened with `onHostEffect`/`maxOutstandingEffectsPerActor`,
  passed through the existing `...overrides` spread (no other line changed).
- Ticket-folder scratch: `terra-shardeffect-vitest-final.txt` (full `--reporter=verbose` run, exit 0),
  `terra-shardeffect-browser-roundtrip.txt` (full console transcript + interpretation of the browser
  evidence below).

Not touched: `🌐plugin-web-materialize.ts` (the generator — read-only, called its real exported
functions from a throwaway scratch script to regenerate `shard-worker.js` for the browser harness,
never edited it), `🎯️targets/🧊️wgpu/**`, `🔌️jcoprobe` fixture (read-only, copied into scratch), any
Rust, registrar-only files.

## Deliberate scope decision: `effect-emit`/`ui-patch-emit` NOT routed

`🟨️host-shim.js`'s `emit`/`emit-patch` fire-and-forget doors post the SAME `"frame"` shape with
`payload.kind` `"effect-emit"`/`"ui-patch-emit"` instead of `"effect-request"`. `handleInboundFrame`
recognizes these as a NOT-`"effect-request"` payload and silently ignores them (the same forward-compat
tolerance `interpretShardFrame`'s own `"unknown"` branch already established in this file) rather than
throwing or hanging anything. This is a **known, explicit gap**: this ticket's "Required work" is
scoped entirely to closing the request/complete/error loop, and no existing code anywhere in the repo
(`🎠️kernel/🟦️component.ts` checked, `grep`ped repo-wide for `effect-emit`/`ui-patch-emit`) handles these
either — flagging for whoever owns emit routing next, not implementing it here to avoid scope creep into
undecided design.

## Evidence — what I actually ran

### Baseline (before any change)

```
$ bun nx run @semio-tech/framework-actor:test --reporter=verbose
Test Files  3 passed (3)
     Tests  40 passed (40)
exit 0
```
Matches the ticket's own recorded W5 baseline (`🎭️actor/…/🟦️typescript` **40**).

### Type-check (scoped — no local `tsconfig.json` in this package, matched repo root's real compiler
flags rather than a bare default `tsc` run, same "config artifact, not a real error" pattern the sibling
packet's own report documents for `TS5097`)

```
$ bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
    --lib DOM,ESNext --esModuleInterop --isolatedModules --resolveJsonModule --skipLibCheck \
    🧵️shard-client.ts
🧵️shard-client.ts(889,17): error TS2339: Property 'vitest' does not exist on type 'ImportMeta'.
🧵️shard-client.ts(890,52): error TS2339: Property 'vitest' does not exist on type 'ImportMeta'.
../../🤖️generated/🟦️actor.ts(167,115): error TS2322: Type 'bigint' is not assignable ...
exit 2
```
Both errors are PRE-EXISTING (the `import.meta.vitest` guard needs `vitest/importMeta` types the repo's
real build supplies via its own tsconfig `types` field, not visible to a scoped bare invocation; the
`🤖️generated/🟦️actor.ts` bigint error is in a different, generated file, unrelated to this packet's
edits and present before this packet touched anything). **Zero errors attributable to my changes** —
confirmed by diffing this run against the identical command on the pre-edit file (same 3 errors,
same locations). `🧵️shard-runtime.ts` scoped-checked the same way: only the pre-existing repo-wide
`TS5097` "import path can only end with .ts" artifact (the sibling packet's own report already
documents this exact diagnostic as a config artifact present in dozens of unrelated files).

### Full suite after the change, `--reporter=verbose`, every new test confirmed BY NAME

```
$ bun nx run @semio-tech/framework-actor:test --reporter=verbose
...
 ✓ ShardClient host-effect bridge — handler success > resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker
 ✓ ShardClient host-effect bridge — handler error > a rejected onHostEffect settles as effect-error, never a hang
 ✓ ShardClient host-effect bridge — no handler installed > fails FAST with an explicit effect-error, synchronously, never a silent hang
 ✓ ShardClient host-effect bridge — backpressure cap > rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending
 ✓ ShardClient host-effect bridge — shard-loss settlement > terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker
 ✓ ShardClient host-effect bridge — shard-loss settlement > dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's

 Test Files  3 passed (3)
      Tests  46 passed (46)
exit 0
```
Full transcript: `terra-shardeffect-vitest-final.txt`. **40 baseline + 6 new = 46**, all passing, all
present in `🧵️shard-client.ts`'s `includeSource` list already (no new test FILE added, so the
"explicit-filename-array" trap this ticket flagged twice does not apply here — the new tests live
in-source in the already-listed file).
`🧪️vitest.config.ts` unchanged (no edit needed).

### Real browser evidence — REAL `ShardClient`, REAL `shardWorkerSource()` output, REAL jco-transpiled
component, driven only through the Browser pane

Full transcript + interpretation: `terra-shardeffect-browser-roundtrip.txt`. Built a scratch harness
(never touching `.claude/launch.json`, served via a scratch-only `bun` static server, driven entirely
via `preview_start`/`get_page_text`/`read_console_messages`) that:

- Bundled `🧵️shard-client.ts` **unmodified** with `bun build --target=browser --format=esm` — the
  ACTUAL class this packet edited, not a reimplementation.
- Regenerated `shard-worker.js` by calling the REAL, unmodified `shardWorkerSource()` (read-only import
  from `🌐plugin-web-materialize.ts`) — the actual worker-side dispatch `terra-web-bridges` shipped.
- Copied the REAL, already-transpiled `jcoprobe.js`/`.core.wasm`/`interfaces/`/`preview2-shim/` from the
  READ-ONLY `🔌️jcoprobe` fixture's `out-callback/` (never modified, only copied into scratch).
- Hand-wrote `host-shim.js`/`bridge.js` for jcoprobe's own WIT namespace (`semio:jcoprobe/*`, not
  production's `semio:framework/*` — same reasoning `terra-web-bridges`' own report gives for why a
  fixture-specific shim is required), transcribing the REAL `hostShimSource()`'s
  `effectRequest`/`__bindHostBridge`/`__resolveEffect`/`__rejectEffect` mechanism verbatim.
- `driver.js` (main thread) constructs the REAL `ShardClient` with a real `Worker`, and an
  `onHostEffect` handler that answers jcoprobe's `slow-echo` probe-host import.

Result (`read_console_messages`, full text in the `.txt`):

```
[driver] onHostEffect actorId=probe-1 effect=slow-echo params={"ms":30,"v":777}
[host-shim] slowEcho(30,777) JS PROMISE RESOLVED value=777
[driver] SUCCESS turn result = {"ok":true,"value":777}
EFFECT-ROUNDTRIP-SUCCESS: PASS
```

**Proven, real, in a real browser**: the guest export `probe.awaitEcho(30, 777)` — which internally
`.await`s the host import `probe-host::slow-echo` — resolved with `777`, the EXACT value my
`onHostEffect` handler returned on the `ShardClient` main thread, delivered through: real `Worker` →
real jco-transpiled component → the NEW `"frame"`/`effect-request` inbound routing → `onHostEffect` →
`effect-complete` reply → the worker's `deliverEffectResult` → the guest's own `.await` resolving. No
hand-simulated "kernel": the actual class this packet edited ran this, bundled unmodified from source.

**Deliberate failure path — proven for MY half, negatively confirmed beyond it (and why):**

```
[driver] onHostEffect actorId=probe-1 effect=slow-echo params={"ms":10,"v":2989}
[host-shim] __rejectEffect called requestId=probe-1:slow-echo:2 message=deliberate failure: poisoned value from onHostEffect hadPendingEntry=true
[host-shim] slowEcho(10,2989) JS PROMISE REJECTED error=deliberate failure: poisoned value from onHostEffect
[driver] FAILURE turn result = {"ok":true,"unexpectedSuccess":true,"value":0}
EFFECT-ROUNDTRIP-FAILURE-PATH: FAIL
```

Traced step by step (full trace in the `.txt`): my `onHostEffect` handler threw → `handleEffectRequest`
caught it and posted `effect-error` → the worker's `deliverEffectResult` routed it to `__rejectEffect`
→ `__rejectEffect` fired with `hadPendingEntry=true` → the `slowEcho(10,2989)` JS Promise **genuinely
rejected** (confirmed by direct `.then`/`.catch` instrumentation on that exact Promise, logged before
jco's own trampoline runs). **Everything this packet owns is proven correct end-to-end, including the
failure path, up to and including rejecting the guest-shim's own Promise.** What is NOT proven — and is
now NEGATIVELY confirmed, not merely unproven — is that this reaches the Rust guest as an observable
`Err`: reading jco's generated trampoline (`jcoprobe.js` `_trampoline27`, ~:7876) shows it DOES catch the
rejection (`task.setErrored(err); task.reject(err); ...`), but jcoprobe's `slow-echo: async func(ms: u32,
v: u32) -> u32` WIT signature is a BARE `u32` return, not `result<u32, E>` — wit-bindgen's generated Rust
(`probe_host::slow_echo(ms, v).await` returning bare `u32`, `🔌️jcoprobe/👽️guest/🦀️component.rs`:26-28)
has no `Err` variant to receive a propagated failure into, so the guest resolves with a default value
(`0`) instead of panicking or returning an error. This is a property of jco's/wit-bindgen's
canonical-ABI codegen for non-`result<>` async imports, not a defect in `ShardClient`'s bridge — and it
directly answers, with a NEGATIVE result, the exact "honestly unproven" item `terra-web-bridges`' own
report flagged ("whether jco expects a `result<T, pack>`-returning host-async import to signal `Err` by
throwing"). Production's real `host-async` surface (component.wit ~:887) DOES use `result<T, pack>` for
effects with observable failure modes; `jcoprobe`'s `slow-echo` does not (and the fixture is read-only,
outside this packet's `path_scope`, so I could not add a `result<>`-typed WIT function to re-run this
against). **Cross-packet finding, worth surfacing**: any `host-async` WIT function where the guest is
meant to observe a host effect failing MUST declare `result<T, pack>` — a bare return type silently
converts a host-side `effect-error` into a default/zero value at the jco boundary, invisible to guest
Rust code.

## What is proven vs. unproven

**Proven, with pasted command/output/exit code**: `ShardClient` correctly routes `effect-request`
frames before the generic reply path; calls the injected `onHostEffect` handler; posts `effect-complete`
with the handler's resolved value; posts `effect-error` with the handler's rejection message (both
confirmed via unit tests AND a real browser round trip through a real jco-transpiled component); fails
fast with an explicit error when no handler is installed (unit test asserts this WITHOUT awaiting a
microtask); aborts and clears outstanding effects on shard loss (`terminate()`) and actor disposal
(`dispose()`), so a late handler settlement after teardown posts nothing; caps outstanding effects per
actor and rejects beyond the cap with a `QuotaSchema.outstanding_requests`-mirroring message. Baseline
40/40 preserved; 46/46 with the 6 new cases, all confirmed by name with `--reporter=verbose`. Zero
type errors attributable to this packet's edits.

**Not proven / structurally out of reach of this packet**: whether a `result<T, pack>`-returning
`host-async` import propagates a rejected `onHostEffect` as a guest-visible `Err` (jcoprobe has no such
WIT function to test against, and the fixture is read-only) — NEGATIVELY confirmed for the bare-return
case instead, which is itself real evidence, not a gap left silent. The 4-interface `world actor` export
shape (`reactor`/`jobs`/`checkpoint`/`describe`) against a REAL compiled artifact — same
not-yet-compiling-fleet limitation `terra-web-bridges`, `terra-jco-spike` both already recorded;
irrelevant to this packet's own `"turn"`/`"frame"` dispatch logic, which is interface-shape-agnostic.
Real `http-fetch`/`blob-read`/`storage-read`/… handler implementations (React/wgpu hosts) — deliberately
out of scope: this packet's job was the seam (`onHostEffect`), not any concrete implementation of it;
`🧵️shard-runtime.ts`'s `createPooledActorRuntime` now threads the option through for whoever wires a
real handler next.
