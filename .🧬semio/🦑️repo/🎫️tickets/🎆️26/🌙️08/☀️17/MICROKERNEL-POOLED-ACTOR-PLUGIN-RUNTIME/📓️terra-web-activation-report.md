# 📓️ terra-web-activation report

Executor `terra-web-activation`, packet: make `ActivationRegistry` (`🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`, region `//#region 🐚️ActivationRegistry`) memory-aware, crash-recoverable, and lane-scheduled.

## delivered

1. **`TurnScheduler` wired into `ActivationRegistry`.** New `enqueueTurn(actorId, lane, events, { coalesce })` routes through a `TurnScheduler<QueuedTurnPayload, ShardBudget>` owned by the registry (constructed in `ActivationRegistry`'s own constructor), instead of a caller reaching `ShardClient.turn` directly with no lane opinion. `budgetFor` is left as a trivial `() => this.defaultBudget` seam, per the packet's explicit instruction not to invent a DRR budget here. `runTurn` (private `runQueuedTurn`) calls `ShardClient.turn` and forwards the result through the new `onTurnResult` option (default no-op, documented honest gap — no effect/`UiPatch` routing exists on this side of the boundary yet).

2. **`onShardLost` actually restores actors.** New `private restoreActor(actorId)`, public `restoreActors(actorIds)`, and a bound `readonly handleShardLost = (shardIndex, actorIds) => void this.restoreActors(actorIds)` convenience field that is a valid `ShardClientOptions.onShardLost` value (`new ShardClient({ …, onShardLost: registry.handleShardLost })`, proven in a test). `restoreActor` drops residency bookkeeping and calls the existing `resume()` (re-activate + restore last checkpoint) — see `## generation/ordering handling` below for how pre-restart queued turns are kept out.

3. **LRU eviction is memory-aware.** New `MemoryProbeReading` (`deviceMemoryGiB?`, `jsHeapSizeLimitBytes?`), injectable `MemoryProbe` type, `defaultMemoryProbe()` (the only production caller of `navigator.deviceMemory`/`performance.memory`, both cast since neither is in the standard DOM lib), and pure `residentActorCapFromMemory(reading, fallback = DEFAULT_MAX_RESIDENT_ACTORS)` (6 actors/GiB of `deviceMemory`, else 1 actor per 64 MiB of `jsHeapSizeLimit`, clamped to `[4, 96]`, else the untouched pre-existing hardcoded `24`). `ActivationRegistryOptions.maxResidentActors` still wins if explicitly passed (unchanged behavior for any existing caller that sets it); otherwise the cap is derived from `options.memoryProbe ?? defaultMemoryProbe`. Fully injectable — tests never touch a real `navigator`/`performance`.

4. **Runtime metrics publisher has a real sink and a real caller.** `ActivationRegistry` now owns a public `readonly metricsBus: EventTarget` and, when constructed with `autoStartMetricsPublisher: true`, the constructor itself calls `startRuntimeMetricsPublisher` with a sink that does `this.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot }))`. New `dispose()` stops that loop. **No topic-subscriber bus exists anywhere in this codebase yet** (native or web — confirmed by grep for `os.runtime.metrics`/`MessageEndpoint`/any `Bus`/`EventTarget`-based pub-sub class repo-wide; `TaskManager/🟦️component.tsx`'s own header doc independently confirms mounting a real window is still registrar-only work). Rather than invent a second bus, this wires the sink to the **platform's own** `EventTarget`/`CustomEvent` pub-sub, which is zero new abstraction and zero new dependency. `autoStartMetricsPublisher` defaults to `false` (opt-in) precisely so every pre-existing/other-file `new ActivationRegistry({...})` call site (this file's own other tests, `TaskManager/🧪️component.test.tsx`) keeps building a plain object with no live `setInterval` — verified those tests still pass unchanged (see `## commands + exit codes`).

## IoRouter region integrity

Baseline (measured before any edit):
```
sed -n '560,799p' 🟦️component.ts | shasum -a 256
ddb2ce7f1f8fb21ca2ebf6cb7934261e34e50fcce605455823c69ea19e8136a7
```
After all edits (region shifted by exactly 1 line — the one new top-of-file import — so the same content is now at 561..800):
```
sed -n '561,800p' 🟦️component.ts | shasum -a 256
ddb2ce7f1f8fb21ca2ebf6cb7934261e34e50fcce605455823c69ea19e8136a7
```
**Byte-identical.** Confirmed by hash equality, and independently by `//#region 🔖️IoRouter` / `//#endregion 🔖️IoRouter` still bounding exactly 240 lines (561–800).

## line ranges edited

All edits are inside `//#region 🐚️ActivationRegistry`, now spanning **1502–2307** (was 1501–1894 pre-edit; grew because of the new memory-probe/turn-dispatch/restore code and 12 new tests). One line was added OUTSIDE the region, at the top-of-file import block (line 16→17, a single new `import { TurnScheduler, type Backpressure, type CoalesceKey, type Lane } from "…/🧵️turn-scheduler.ts";` line immediately after the pre-existing `ShardClient` import, plus adding `type ShardEventEnvelope` to that same existing import) — this is the same "extend the shared top-of-file import list" pattern packet H2 already used to add the `ShardClient` import itself; nothing else outside the region was touched. Total file: 2721 → 3134 lines.

Specific edits within the region:
- `ActivationRegistryOptions`: added `memoryProbe?`, `turnMailboxCapacity?`, `onTurnResult?`, `onTurnError?`, `autoStartMetricsPublisher?`.
- New `//#region 🧮️MemoryPressureCap` (types, constants, `defaultMemoryProbe`, `residentActorCapFromMemory`) and `//#region 🧵️QueuedTurn` (`QueuedTurnPayload`), both between `defaultGuestSlimAssetFetcher` and `ActivationResidentEntry`.
- `ActivationRegistry` class: new fields `actorGeneration`, `turnScheduler`, `onTurnResult`, `stopMetricsPublisher`, `metricsBus`; constructor now derives `maxResidentActors` from the probe, builds the `TurnScheduler`, and conditionally auto-starts the metrics publisher.
- New `//#region 🧵️TurnDispatch` (`enqueueTurn`, `runQueuedTurn`) inserted between `//#endregion ▶️Activate` and `//#region 🚑️SuspendResume`.
- `suspend()`: now calls `this.turnScheduler.cancelQueued(actorId)` as its first statement.
- New `restoreActor` / `restoreActors` / `handleShardLost` inserted between `resume()` and `cancel()`.
- `cancel()`: now calls `this.turnScheduler.teardownActor(actorId)` and `this.actorGeneration.delete(actorId)`.
- New `dispose()` inserted after `isResident()`.
- `startRuntimeMetricsPublisher`'s doc comment updated from "🚧 GAP: no caller" to "✅ now has a real caller and sink" plus a narrower, still-honest remaining gap (no real consumer subscribes to `metricsBus` yet).
- 12 new tests added inside the pre-existing `if (import.meta.vitest)` block (kept there on purpose, same reasoning the block's own header doc already gives for why tests live here rather than end-of-file), in four new sub-regions: `🧪️TurnDispatchTests`, `🧪️ShardLossRestoreTests`, `🧪️MemoryPressureCapTests`, `🧪️MetricsBusTests`. Also factored the existing `fakeShardClient`'s inline worker into a named, reusable `createAutoReplyWorker()` (used by both the old helper and the new tests that need their own `ShardClient` construction), and added a `flushMicrotasks(n)` helper (bounded-loop `await Promise.resolve()`, no real timer).

## generation/ordering handling

The web `actorId` stays a plain caller-minted string (per this region's own pre-existing header doc — the bit-packed `RuntimeActorId` with its `generation:u14` field lives only in the native `semio-framework-actor` crate). Native's `FailurePolicy` mirrors "Trapped → drop + re-instantiate (**generation++**) + restore last checkpoint"; the web side needed the same *intent* (a turn queued against the pre-restart instance must never run against the post-restart one) without an id-embedded field to carry it.

I added generation **out of band**: a private `actorGeneration: Map<string, number>` on `ActivationRegistry`. `enqueueTurn` snapshots the actor's current generation into the queued payload (`QueuedTurnPayload.generation`); `runQueuedTurn` (the `TurnScheduler`'s `runTurn` seam) compares that snapshot against the actor's *current* generation at dispatch time and silently drops (with a permanent `[DEBUG]` log, same convention this file already uses for `graftWorkerStack`) any turn whose generation is stale, instead of executing it.

Two layers, deliberately redundant:
1. `restoreActor` bumps the generation **first**, then synchronously calls `turnScheduler.cancelQueued(actorId)` — this drains anything already sitting in the mailbox at the moment loss is detected.
2. The generation bump alone additionally protects the *race window* `cancelQueued` cannot cover: a turn some other caller enqueues **during** the `await this.resume(actorId)` that follows. That turn snapshots the *already-bumped* generation, so if it happens to get dispatched before `resume()` finishes it fails loudly (actor not yet activated) rather than silently landing on stale state — and after `resume()` completes, generation-matched, correctly-ordered turns dispatch normally.

Ordinary `suspend()`/`resume()` (LRU eviction, not shard loss) deliberately does **not** bump generation — it only calls `cancelQueued` (the same "cancel synchronously as the very first statement, before the `checkpoint`/`dispose` round trip's first `await`" pattern), because that path is already fully synchronous-cancel-then-checkpoint with no external actor able to enqueue into the gap before the cancel runs. Generation is reserved for the crash/restore path where the actor was torn down by something *other* than this registry's own orderly suspend.

This is proven by two tests: "a suspended actor's queued turns are cancelled, never delivered" (ordinary suspend) and "a restored actor does not receive turns that were queued before the restore, but does receive turns queued after" (shard-loss restore, generation-gated).

## commands + exit codes

```
$ bunx vitest run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-t1-kernel-vitest.config.ts --reporter=verbose
 Test Files  1 passed (1)
      Tests  29 passed (29)
EXIT: 0
(full output: terra-web-activation-kernel-vitest.txt)

$ bun nx run @semio-tech/framework-actor:test
 Test Files  6 passed (6)
      Tests  58 passed (58)
EXIT: 0
(full output: terra-web-activation-actor-vitest.txt; note: nx served this from cache since nothing in that package changed — legitimate, package hash unchanged)

$ node node_modules/vitest/vitest.mjs run --config "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts" --passWithNoTests --root "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements" TaskManager/🧪️component.test.tsx
 Test Files  1 passed (1)
      Tests  12 passed (12)
EXIT: 0
(full output: terra-web-activation-taskmanager-vitest.txt — proves the default-false autoStartMetricsPublisher choice did not disturb this file's existing ActivationRegistry construction sites)

$ bunx tsc --noEmit -p tsconfig.json
19 errors, all in the three files the coordinator already named as pre-existing and unrelated (✏️s/🔌️plugins/🔱️trinity/…/🟦️component.ts ×14, ✏️s/🔌️plugins/🗄️stdio/…schema…/🟦️component.ts ×4, 🧰️framework/…vscode/…/🟦️extension.ts ×1). Zero errors in 🎠️kernel/🟦️component.ts or anywhere touched by this packet.
EXIT: 2
(full output: terra-web-activation-tsc.txt — exit code observed is 2, not the 1 the coordinator's brief quoted; the error SET is identical to the named 19, so this is the same known-and-not-ours gate, just a different tsc-version exit convention)
```

## baseline vs after + proof new tests ran by name

- Kernel package inline tests (`terra-t1-kernel-vitest.config.ts`): **baseline 17/17** (measured fresh before any edit) → **after 29/29** (17 pre-existing + 12 new, zero regressions). New tests confirmed running by name via `--reporter=verbose`:
  - `ActivationRegistry.enqueueTurn lane priority > dispatches turns by lane priority end-to-end through the registry, not enqueue order`
  - `ActivationRegistry.suspend cancels queued turns > a suspended actor's queued turns are cancelled, never delivered`
  - `ActivationRegistry.handleShardLost / restoreActors > is a valid ShardClientOptions.onShardLost value`
  - `ActivationRegistry.handleShardLost / restoreActors > restores exactly the actors that were on the lost shard, leaving an actor on a different shard untouched`
  - `ActivationRegistry restore ordering > a restored actor does not receive turns that were queued before the restore, but does receive turns queued after`
  - `residentActorCapFromMemory > derives the cap from deviceMemoryGiB when present, clamped to [4, 96]`
  - `residentActorCapFromMemory > falls back to jsHeapSizeLimitBytes when deviceMemoryGiB is absent`
  - `residentActorCapFromMemory > falls back to the hardcoded constant when neither signal is present`
  - `ActivationRegistry.maxResidentActors derived from an injected memory probe > a small deviceMemoryGiB reading evicts down to its (small) derived cap`
  - `ActivationRegistry.maxResidentActors derived from an injected memory probe > a large deviceMemoryGiB reading keeps every one of the same 10 activations resident`
  - `ActivationRegistry.metricsBus (autoStartMetricsPublisher) > publishes os.runtime.metrics as a CustomEvent on metricsBus at the 2Hz interval, driven by the injected clock, and dispose() stops it`
  - `ActivationRegistry.metricsBus (autoStartMetricsPublisher) > stays empty (no live interval, no bus traffic) when autoStartMetricsPublisher is left at its default`

  All 12 use fake timers (`vi.useFakeTimers`) or bounded microtask flushing (`flushMicrotasks`) — **no real sleeps anywhere**, matching the packet's requirement.

- Actor package (`@semio-tech/framework-actor`): **baseline 58/58** (measured fresh) → **after 58/58** (unchanged — this packet never touched that package; confirms no regression from consuming its `TurnScheduler`).
- `TaskManager` component test (uses `ActivationRegistry` directly, outside this packet's `path_scope`): 12/12 passing after my edits — confirms the default-`false` `autoStartMetricsPublisher` choice left every pre-existing construction site's behavior unchanged.

## lease-requests

None. All edits stayed inside `🎠️kernel/🟦️component.ts`'s `🐚️ActivationRegistry` region plus the single top-of-file import line already established as fair game by a prior packet (H2's `ShardClient` import). No registrar-only file was touched.

## honest gaps

1. **No real consumer subscribes to `metricsBus` yet, anywhere.** Same underlying gap T1 already recorded for `startRuntimeMetricsPublisher` itself — mounting the task-manager window that would (`TaskManager/🟦️component.tsx`'s own header doc: needs a window-kind registration + a `ShellHost` mount, both registrar-only) is unstarted. This packet makes the publish loop real and gives it a real, generic sink (`EventTarget`/`CustomEvent`, the platform's own bus) rather than a bespoke one — but a UI-visible task manager still needs someone to subscribe. Deliberately opt-in (`autoStartMetricsPublisher: true`) rather than on-by-default so this gap doesn't silently start a live interval for every existing/future caller that doesn't want one.
2. **`onTurnResult`'s default is a documented no-op.** `ShardClient.turn`'s resolved value (the guest's `TurnResult` — effects, UI patches) has no consumer on this side of the boundary; routing it into `UiPatch` application / effect dispatch belongs to the renderer's `ProgramBridge`, not this registry. Nothing currently calls `enqueueTurn` in production code either (same "no caller yet" shape as the metrics publisher before this packet) — this packet's job was to make the *mechanism* real and tested, not to migrate a production call site, since none exists in this file's own path_scope today (`shardClient.turn` had no direct production caller anywhere in the repo, confirmed by grep, before or after this packet).
3. **`ActivationRegistry.cancel`'s pre-existing job-id gap is unchanged** — still documented inline exactly as T1 left it; out of this packet's scope (needs `ShardClient.cancelJob`'s caller-tracked job ids, owned by whoever lands `startJob`/`stepJob` bookkeeping).
4. **The memory-probe heuristic's constants (`6` actors/GiB, `64 MiB`/actor) are my own tuning judgment**, not measured against a real wasm instance's actual footprint — documented as a heuristic in the docstring, not asserted as calibrated. Getting real numbers would need an actual browser memory profiling pass, out of scope for a TypeScript-only, no-browser-available packet.
5. **`tsc --noEmit` exit code observed is `2`, not the `1` the coordinator's brief stated** — the error content (same 19, same three files) is identical, so I read this as a tsc-version/environment difference, not a new problem, but flagging the discrepancy rather than silently "correcting" it to match the brief.
