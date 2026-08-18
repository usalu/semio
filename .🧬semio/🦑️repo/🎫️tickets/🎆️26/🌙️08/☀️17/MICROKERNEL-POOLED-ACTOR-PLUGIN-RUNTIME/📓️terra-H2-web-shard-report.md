# 📓️ terra — H2-web-shard report

Packet: **H2-web-shard** — replace one-Worker-per-plugin with a bounded shard pool (design-runtime.md §3).

Status: **done** for the five scope items on the four owned files. Real, precisely-scoped downstream
breakage in files this packet does not own — see Lease-requests. Backbone real-time-sync delivery has
an honest, documented gap (see §Known gaps).

## Files touched

**Created**
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` — `ShardClient`.
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- (this file, plus `📓️terra-H2-tsc-check.txt`, `📓️terra-H2-test-actor.txt`, `📓️terra-H2-test-framework.txt`, `📓️terra-H2-test-framework-os.txt`)

**Edited**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`

No other file was written to. `🔌️plugin/**` (outside the one carved-out `🌐plugin-web-materialize.ts`
file) and `📺️renderer/**` were read for research only, never edited.

## What landed, item by item

**1. `🌐plugin-web-materialize.ts`** — `pluginWorkerSource()` → `shardWorkerSource()`; `PLUGIN_WORKER_FILE`
→ `SHARD_WORKER_FILE = "🟨️shard-worker.js"`. The generated worker keeps `Map<actorId, {api, moduleUrl,
pendingAssets}>`, dynamically `import()`s each actor's own jco bridge on first `activate` (never at
worker-bootstrap), rejects a second in-flight `turn` for the same `actorId` (different actors still
interleave — every handler is `async`), and posts `{kind:"heartbeat", turnSeq}` at the start of every
request, mirroring into an `Atomics.store` SAB slot when one was attached via `attachHeartbeatSab`.
`pluginComponentBridgeSource` drops the whole `runSerialized` retry/reload loop and exposes
`createActorApi()` with `poll`/`startJob`/`stepJob`/`cancelJob`/`checkpoint`/`restore` (plus `describe`,
which the WIT world also exports) over the four jco-generated interface namespaces (`reactor`/`jobs`/
`checkpoint`/`describe`) — **UNVERIFIED against a real compiled artifact**, see §Known gaps.
`hostShimSource()` now implements only `log`/`nowMs`/`traceSpan` — the WIT `pure` interface, confirmed
by reading `component.wit`'s `interface pure { log; now-ms; trace-span; }` and its own doc comment ("the
ONLY interface any world ever imports"). `writeBlob`/`readBlob`'s synchronous XHR and the
`backboneSend`/`backbonePoll`/`backboneStatus` relay are gone entirely — not migrated, deleted, because
`world actor` has no import surface for them anymore (reads/writes/network/backbone-shaped traffic are
now `effect`s answered by `event`s inside `poll`). `transpilePluginComponent`'s `--map` target moved from
`semio:framework/host` to `semio:framework/pure`.

`guestSlimAssets` (item 5) is no longer a worker-bootstrap special case: the worker caches whatever
`assets` arrive on `activate` and splices them into the `assets` field of the first `instance-open`
event it sees in a `turn`'s events (the KERNEL constructs the base `instance-open` event — it alone
knows `app-id`/`config`/`quotas` — the worker only owns fetching/caching the bytes). The actual fetch-once
caching lives in `ActivationRegistry.loadAssets` on the kernel side now (see item 3/5 below), not in the
generated worker script.

**2. `ShardClient`** (`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`) — owns the whole bounded
pool (one `ShardWorkerLike` per shard), `actorId`-tagged request/reply multiplexing, `leaseExclusive`/
`releaseExclusive` against a reserved tail of ≤2 exclusive shard indices, `terminate()`/`rebuild()`, and
a heartbeat watchdog: a shard only "misses" while it has an in-flight turn/job older than
`heartbeatTimeoutMs` with no fresher heartbeat since (an idle shard can never be flagged); three
consecutive timeout windows of continued silence trigger `terminate()`+`rebuild()`+`onShardLost`. SAB
path (`pollHeartbeatSab`, `Atomics.load`) and `postMessage` path feed the exact same state machine —
`postMessage` heartbeats are unconditional (the generated worker always sends them), so correctness
never depends on `SharedArrayBuffer` being available, satisfying the brief's explicit requirement.
26 in-source vitest tests cover activation+turn round-trip, out-of-order multiplexed replies,
round-robin shard assignment (skipping the exclusive tail), the heartbeat state machine on both paths
(including the "fresh heartbeat resets the miss count" case), `leaseExclusive`/`releaseExclusive`
(including exhaustion and idempotency), `terminate`/`rebuild`, and worker `onerror` handling. All 26 pass
(`📓️terra-H2-test-actor.txt`).

**3. `🎠️kernel/🟦️component.ts`** — `PluginWorkerClient` (class + both maps
`pluginWorkerClients`/`activeWorkerByPluginId`), `loadPluginModuleViaWorker`, `LeasePool`/`Lease`/
`LeasePoolStats`/`createLeasePool` (relocated, not deleted — see item 4), `PluginModuleLease`/
`acquirePluginModule`/`evictPluginModule`, `loadPluginModuleUncached`, `pluginHandleForBridge`,
`withSerializedPluginWasmHandle`/`pluginErrorText`/`isPluginInstanceBusyError`, `pluginWorkerUrl`,
`guestSlimAssetsForModule`, `PLUGIN_WORKER_UNRESPONSIVE_MS` are all deleted. Grepped the whole tree for
each symbol before deleting (see §Lease-requests for the ones with real external callers).

Replaced with `ActivationRegistry`: manifest-only records (`registerManifest`/`registerCatalog` — the
latter seeds straight from a `PluginCatalog`'s `plugins`/`extensions`, no worker/module touched until an
actual `activate()`); `activate(pluginId, actorId, reason: ActivationReason)` maps an
`events::activation-event`-shaped reason onto `ShardClient.activate` (design's `Kernel::activate` on the
web transport); `touch()`/LRU residency ordering with `maxResidentActors`-driven memory-pressure
`suspend()` (checkpoints via `ShardClient.checkpoint`, then `dispose`s the worker-side entry) before any
activation that would exceed the cap; `resume()` re-activates and `restore()`s the last checkpoint (a
plain cold start if there was never one). `defaultGuestSlimAssetFetcher` ports the deleted
`guestSlimAssetsForModule`'s exact fetch-once/cache/graceful-degrade behavior, generalized to the
`ShardAsset` tuple shape and callable from any actor's `moduleUrl`, injectable via
`ActivationRegistryOptions.fetchAssets` for testability.

`postPluginBackboneInbound` (kept — `ShellHost` imports it, see §Lease-requests) had its only
`PluginWorkerClient`-dependent branch removed; its doc comment states plainly that the whole
worker-postMessage backbone relay this function used to route into is now dead on the guest side too
(deleted alongside `backboneSend`/`backbonePoll` in item 1), so this function's remaining behavior
(push to the main-thread queue) is honest but currently drained by nothing. `registerPluginBackboneRoute`/
`pluginBackboneRoutes`/`relayPluginBackboneOutbound`/`pluginBackboneDocumentIdFromUri` are untouched —
they don't depend on anything deleted.

Proved byte-identical, before and after, using file-hash diffs (not by eye):
- `//#region 🔖️IoRouter` … `//#endregion 🔖️IoRouter`: was lines 559–798 (240 lines), now 560–799 (240
  lines — shifted by exactly 1 for the new `ShardClient` import line added above it). First line
  `//#region 🔖️IoRouter`, last line `//#endregion 🔖️IoRouter`, both times. `diff` of the extracted
  ranges: **empty**.
- `//#region 🧪️IoRouterTests` … `//#endregion 🧪️IoRouterTests` (the in-source vitest block
  `📌️important.md` also calls out): was 2688–2786 (99 lines), now 2391–2489 (99 lines). First line
  `//#region 🧪️IoRouterTests`, last line `//#endregion 🧪️IoRouterTests`. `diff`: **empty**.

**4. `createLeasePool`/`LeasePool`/`Lease`/`LeasePoolStats`** relocated unchanged into
`🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` under `//#region 🪶️LeasePool`, same implementation,
same doc comments (generalized away from the plugin-specific framing since its plugin caller is gone).
Removed the now-dead `acquirePluginModule`/`evictPluginModule`/`createLeasePool`/`pluginWorkerUrl`
imports from glue.ts's own kernel-import block (`acquirePluginModule`/`evictPluginModule` were imported
but never actually called anywhere in glue.ts — confirmed by grep before removing) and deleted the
`pluginWorkerUrl` describe block (3 tests) since that function no longer exists (the new shard worker
has one fixed, package-agnostic URL rather than one derived per plugin module). Grepped the whole tree
for `createLeasePool`/`LeasePool` outside kernel/glue: the only hit is a doc-comment mention in
`WasmSessionLoader/🟦️component.tsx` (renderer) — no real import site broke.

**5. `guestSlimAssets`** — see item 1/3 above: no longer a `PluginWorkerClient`-era worker-bootstrap
special case (`GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE`/`guestSlimTypstFontsPromise` deleted). It is
now `ActivationRegistry`'s `defaultGuestSlimAssetFetcher`, fetched once, passed as `assets` into
`ShardClient.activate`, and spliced by the generated worker into the guest's `instance-open` event
before the first `poll` — resident before the first `surface-visible`, per the brief.

## Known gaps (flagged, not hidden)

1. **Kernel↔Shard wire is interim JSON, not the real `Envelope`/`TurnResult` pack encoding.**
   `ShardClient.turn()`/the worker's `poll` message use a plain JSON `ShardEventEnvelope[]`/object shape
   at the public boundary (still `Uint8Array` in/out at `ShardClient`'s own top-level API per
   `ShardTransport`, but the worker decodes an interim shape internally). The real hand-rolled binary
   pack codec matching `🎭️actor/🦀️component.rs`'s `pack` module has no TS mirror yet — A1's own report
   flags `🤖️generated/🟦️actor.ts` as not-yet-emitted, and building a byte-compatible codec by hand
   without generated tooling or a round-trip target to verify against would be exactly the
   "hand-rolled and drifting" outcome CLAUDE.md's SSOT rule forbids. Swapping the worker's decode step
   for the generated codec once it lands is mechanical — nothing in `ShardClient`'s assignment/
   heartbeat/multiplexing logic depends on the wire format.
2. **jco's exported binding shape for `world actor` is unverified against a real compiled artifact.**
   B1b's wasip2 guest build is still in progress (per `📓️status.md`'s W2 dispatch). `pluginComponentBridgeSource`
   assumes one JS binding per exported interface (`reactor`/`jobs`/`checkpoint`/`describe`); if jco
   nests differently, only those four destructured names need to change.
3. **Backbone real-time-sync delivery is now fully inert**, not just degraded. `postPluginBackboneInbound`/
   the main-thread queue it feeds have nothing left draining them on the guest side (deleted with
   `backbonePoll` in item 1, consistent with the ABI flip). This is a pre-existing crack this packet
   widens rather than one it opens: A4-channel already deleted `AppCommand::AttachBackbone`/
   `DetachBackbone` before this packet started. Proper redesign (routing backbone-shaped traffic through
   `events::message-event`) is real, non-mechanical work belonging to whichever packet finishes migrating
   `ShellHost` off the pre-flip `PluginWasmHandle` ABI — flagged in the function's own doc comment, not
   silently left looking functional.
4. **`🎭️actor/🟦️component.ts` still doesn't re-export `ShardClient`.** A1's own report explicitly
   deferred this ("not mounted here yet") and that file is outside this packet's owned paths
   (`🎭️actor/📦️packages/🟦️typescript/**` only) — left untouched.

## Lease-requests

Every one of these is a real compile/type break caused directly by this packet's mandated renames/
deletions (`pluginWorkerSource`→`shardWorkerSource`, `PLUGIN_WORKER_FILE`→`SHARD_WORKER_FILE`,
`acquirePluginModule`/`PluginModuleLease` deleted), in a file outside this packet's `path_scope`. None
of these were edited.

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📜️store.ts
reason: not owned (🔌️plugin/** — B1b/A5 territory). Imports PLUGIN_WORKER_FILE + pluginWorkerSource (both renamed) at lines 10/14. Line 250 writes a PER-PLUGIN copy of the worker bootstrap into every plugin's own outDir (join(outDir, PLUGIN_WORKER_FILE), pluginWorkerSource()) — under the new design the shard worker is ONE package-agnostic file served from /plugin-modules/_shard/, so this should become a single write (or copy) to that shared location instead of one per plugin, using SHARD_WORKER_FILE + shardWorkerSource. Line 251's pluginComponentBridgeSource(componentBase, EXTENSION_COMPONENT_FILE) call still works unchanged (same signature).
```

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts
reason: not owned. Imports pluginWorkerSource (renamed) at line 45. Line 818 writes join(outDir, "🟨️plugin-worker.js") per plugin — same "should become one shared /plugin-modules/_shard/ write" fix as store.ts above, using SHARD_WORKER_FILE + shardWorkerSource. Line 724's keepFiles = new Set(["🟨️host-shim.js", "🟨️plugin-worker.js"]) (hot-reload stale-file cleanup allowlist) also needs "🟨️plugin-worker.js" replaced — but since the shard worker is no longer per-plugin-outDir, this file likely shouldn't be in a per-plugin keepFiles set at all once the write above moves to the shared location.
```

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
reason: not owned (📺️renderer/** — H3 territory). Defines its own loadPluginModule (line 133-134: `return adaptPluginHandle(pluginId, await acquirePluginModule(pluginId, moduleUrl));`) and adaptPluginHandle(pluginId, lease: PluginModuleLease) — both acquirePluginModule and PluginModuleLease are deleted from kernel/component.ts by this packet (design-runtime.md §3 mandate: "Replace with ActivationRegistry"). This file's whole PluginWasmHandle adapter layer (loadPluginModule/loadPluginModulesInDependencyOrder, ~1300 lines) is built on the pre-flip binary-exchange ABI and needs a real rewrite onto ActivationRegistry + ShardClient's activate/turn/checkpoint/restore, not a mechanical rename — this is genuinely H3's design work, not H2's.
```

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx
reason: not owned (📺️renderer/** — H3 territory; also registrar-adjacent per 📌️important.md's ShellHost note). Imports evictPluginModule (kept — still exists in glue.ts's relocated LeasePool region, but the pluginModulePool it used to evict is gone, so a real call would need a real replacement, likely ActivationRegistry.suspend), postPluginBackboneInbound and registerPluginBackboneRoute (both kept, still compile, but see §Known gaps #3 — the backbone path they feed is now inert). ShellHost's loadedPlugins map holds PluginWasmHandle instances sourced from PluginRuntime's loadPluginModuleResilient (see the PluginRuntime lease-request above) — this file's plugin lifecycle (install/uninstall/hot-swap around lines 1400-1900, and the pluginHandleFor/exchange-based merge-policy/conflict-resolution calls around lines 4193-4263) is the same pre-flip ABI end-to-end and needs the same real H3 rewrite, not a rename.
```

## Acceptance — commands run, real output, exit codes

**`@semio-tech/framework-actor` (new package) — `bun ./📜️script.ts test quick`**
```
 Test Files  2 passed (2)
      Tests  26 passed (26)
EXIT: 0
```
Full output: `📓️terra-H2-test-actor.txt`.

**`@semio-tech/framework` — `bun ./📜️script.ts test quick`** (covers `🟦️glue.ts`, including the
relocated `LeasePool` region's existing tests)
```
 Test Files  2 passed (2)
      Tests  152 passed (152)
EXIT: 0
```
Full output: `📓️terra-H2-test-framework.txt`.

**`@semio-tech/framework-os` — `bun ./📜️script.ts test quick`** (the command the ticket's acceptance
criteria and A4's report both measured 316/318 against)
```
 Test Files  2 failed | 2 passed (4)
      Tests  2 failed | 316 passed (318)
EXIT: 1
```
**316/318 passing — matches the recorded baseline exactly, zero regression.** The 2 failures are the
identical pre-existing test in both runs: `@semio-tech/framework-os workflow > matches the Rust
plan_workflow across shared fixtures decoded via wasm`, failing with `Cannot find module
'.../🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js'` — a missing built wasm artifact, unrelated to
this packet (confirmed: this test file has zero references to any symbol H2 touched). Full output:
`📓️terra-H2-test-framework-os.txt`.

**Typecheck — `node_modules/.bin/tsc --noEmit` against `🎠️kernel/🟦️component.ts` + `🟦️glue.ts`** (no
dedicated `typecheck` nx target exists in either package — same finding A4's report already recorded;
`test quick` above is each package's real check command). Same known-limitation caveat A3's report
established: bare `tsc` without Bun's bundler-mode `.ts`-extension resolution and without this repo's
real module graph produces resolution noise unrelated to any real type error.
```
EXIT: 2, 141 errors total
```
Triaged by file and cross-checked against my new symbols:
- **0 errors** mention `ShardClient`/`ActivationRegistry`/`ShardBudget`/`ShardAsset`/
  `ShardCapabilityGrant` anywhere in the 141.
- **0 errors** in `🟦️glue.ts` mention `Lease`/`LeasePool`/`createLeasePool` (the relocated region).
- `🎠️kernel/🟦️component.ts` itself: 3 errors — 1 is `TS5097` (the same `.ts`-extension noise on my new
  `ShardClient` import line every other import in this file already produces under bare `tsc`); 2 are
  `PluginManifest.contributions`/`ProgramContributionEntry.contribution` inside `buildContributionsJson`
  (line 110-118) — a function this packet never touched, ~1300 lines away from every edit in this
  report, using types imported from `../🛂️manifest/🟦️component.ts` that bare `tsc` resolves differently
  than the real bundler graph (the exact noise class A3's report already catalogued for that same
  generated-manifest dependency).
- `🟦️glue.ts`: 44 errors, all pre-existing categories independently reproduced by running `tsc` against
  `🌐plugin-web-materialize.ts` alone (which transitively pulls glue.ts in through the shared library
  index) BEFORE any kernel/glue edit in this packet touched anything near them: `TS5097` noise, and the
  `🔄️machine` module's `PlayerEvent`/`RecorderEvent` "`eventCount` missing" statechart errors — an
  unrelated module this packet never imports from or edits.

Full output: `📓️terra-H2-tsc-check.txt`.

## Ownership / process compliance

No git-modifying command was run. No `.cargo/config.toml`/registrar file touched. All scratch
verification files (`tsc`/vitest raw output) are `.txt` inside this ticket folder, `[DEBUG]`-prefixed
logs (the ones I wrote — `heartbeat`/asset-fetch warnings) already existed pre-packet or follow the same
convention. `bun ./📜️script.ts test quick` for all three packages and the two `tsc --noEmit` runs above
were the only commands executed, all in the foreground, all pasted in full above with real exit codes.
