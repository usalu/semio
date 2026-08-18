# terra-web-plugin-runtime report

Owned path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx` (only file touched; before = 1615 lines, after = 2203 lines). No other file was edited.

## delivered

1. **`serializePerActor` / actor turn queue → landed `TurnScheduler`.**
   - Kept `serializePerActor<T>(actorId, run): Promise<T>`'s EXACT public signature/contract (backed by `📦️index.tsx` in the react-target package, which re-exports it, and `🧪️index.test.ts`, which asserts the generic per-actor-mutex contract directly) — but its internals now sit on a dedicated `TurnScheduler<ThunkTurnPayload, undefined>` (`getThunkScheduler`), bounded at `SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY = 256`, rejecting (not growing) once an actor's queue is genuinely full.
   - Added a SECOND, purpose-built `TurnScheduler<PluginTurnPayload, ShardBudget>` (`getPluginTurnScheduler`/`submitPluginTurn`, region `🔖️PluginTurnScheduler`) that is now what `loadPluginModule`'s internal `submitTurn` actually calls. It gives every real turn a `Lane` (`"Interactive"` for `createApp`/`exchange`/`completeExtensionInvoke`; `"UserVisible"` + a `"surface-visible"` coalesce key for `refreshUi`'s pointer-move-driven redraw probe) and a bound (`PLUGIN_TURN_MAILBOX_CAPACITY = 32`, matching `ActivationRegistry`'s own default). Coalesced callers all resolve with the SAME winning turn's result via a `waiters` list on the shared payload, rather than the mailbox's own envelope-replacement silently discarding a superseded caller's callback.
   - `submitTurn` now also calls `registry.touch(actorId)` on every turn — a small, in-scope fix: turns previously bypassed `ActivationRegistry`'s LRU refresh entirely (only `activate`/`resume`/`suspend` touched it), so a busy-but-long-resident actor could get evicted ahead of an idle one under memory pressure.

2. **Dependency boot → level-parallel, bounded.**
   - `computeDependencyLevels` groups `orderPluginRegistryEntries`'s already-topological `order` into levels from `PluginRegistryEntry.dependencies` (walking `order` guarantees every dependency is assigned a level before its dependent is visited).
   - `runBounded` is a plain worker-pool loop (not chunked `Promise.all` batches) so a fast sibling's slot is reused immediately.
   - `loadPluginModulesInDependencyOrder` now loads level-by-level (dependency levels sequential; siblings within one level run concurrently, bounded by `options.concurrency ?? poolConcurrency()`).
   - **Concurrency-bound justification**: `poolConcurrency()` = `min(hardwareConcurrency-1, 4)` — the EXACT same number `getShardClient`'s own worker-pool `shardCount` uses (factored into one shared function so the two can never drift). `activate()`'s real cost is bounded by how many shard workers exist to run it; requesting more concurrent activations than shards would only add `evictForMemoryPressure` LRU thrashing for zero extra real parallelism.
   - Extended the EXISTING fail-soft posture (graph faults never abort the whole boot) to runtime `loadPluginModule` rejections too — new `PluginLoadFailure[]` (kept separate from `PluginGraphError[]`, a different failure class) — and made it CASCADE: a dependent of a failed plugin is skipped (recorded as its own `PluginLoadFailure`), not attempted against a dependency that never loaded.
   - `options.loadModule`/`options.signal` are new, optional, additive parameters (default `loadModule = loadPluginModule`) — added specifically so the level/concurrency/cascade logic is unit-testable without a real Worker; no existing caller (there are none in the repo yet — grepped `loadPluginModulesInDependencyOrder`, zero real call sites, confirmed via `w2-b-report.md`'s own "ready but not yet called" note) is affected.

3. **Watchdog + shard loss + descriptor-fetch abort.**
   - `getShardClient()` now calls `.startWatchdog()` once, on construction, on the shared `ShardClient` — before this, neither `checkHeartbeats` nor `pollHeartbeatSab` had ANY production caller anywhere in the repo (confirmed by `🧵️shard-client.ts`'s own `startWatchdog` doc comment, which names this exact gap).
   - `onShardLost` now calls `handlePluginShardLost`, which delegates to `getActivationRegistry().handleShardLost(shardIndex, actorIds)` — the ALREADY-LANDED, coordinator-verified real restore path (bumps generation, cancels the stale queue, `resume()`s from checkpoint) — instead of only `console.error`-ing.
   - Extracted `buildShardClientOptions(createWorker?)` and `handlePluginShardLost` as separate, testable functions (both module-private) so the wiring can be unit-tested without constructing a real DOM `Worker` (this suite has no `Worker` polyfill).
   - `fetchDescriptorManifest(pluginId, moduleUrl, signal?)` now threads an optional `AbortSignal` into `fetch`. An abort PROPAGATES (throws) rather than falling back to the empty-manifest default — silently continuing to load a plugin the caller explicitly gave up on would be a worse surprise than a rejected promise. A genuine (non-abort) network failure keeps the existing fallback unchanged. `loadPluginModule` gained an optional 3rd `signal?` parameter threading through to it; `loadPluginModulesInDependencyOrder`'s own `options.signal` threads to every `loadModule` call and additionally stops STARTING new loads once aborted (in-flight ones still settle).

4. **Metrics publisher — left OFF.** See `## metrics-publisher ownership decision` below.

## findings confirmed vs not reproduced

All four findings in the brief reproduced exactly as described against the live file (confirmed by re-reading the file in full before editing, not from the earlier audit's line numbers alone — those had drifted slightly but the substance matched):

- `serializePerActor` (was `~:390-399`, actually found at lines 448-457 in the live file): confirmed unbounded `Map<actorId, Promise>` FIFO chain, no lanes, no coalescing, no bound, no cancellation. **Reproduced.**
- Dependency boot (was `~:1122-1129`, actually lines 1349-1356 live): confirmed a strict `for`-loop `await`ing `loadPluginModule` one at a time over `order`. **Reproduced.**
- Watchdog + shard loss (was `~:186-201, ~:547`, actually lines 187-201 / descriptor fetch at 762 live): confirmed `onShardLost` only did `console.error`; confirmed `startWatchdog()` was never called anywhere in this file (grepped the whole repo — zero production callers, matching `🧵️shard-client.ts`'s own doc). **Reproduced.**
- Descriptor fetch had no `AbortSignal` param at all. **Reproduced.**

No finding failed to reproduce; nothing in the brief was stale.

## one-turn-per-actor invariant evidence

The invariant is now structural, not hand-rolled: `TurnScheduler.pump()` (`🧵️turn-scheduler.ts` lines 191-209) marks an actor "busy" the instant it pops that actor's next envelope and only clears it in the dispatched `runTurn` promise's `finally` — `pickNextReadyActor` skips any busy actor, so a second turn for the same actor structurally cannot start before the first settles. This is already covered by `turn-scheduler.ts`'s own landed test ("never starts an actor's next turn before its current one settles, even while other actors interleave").

For THIS file specifically, added `submitPluginTurn (...) > never dispatches a second turn for an actor before the first settles, even while a DIFFERENT actor's turns run concurrently` (new test, passing) — proves the invariant survives on top of the coalescing-waiters logic layered over the raw scheduler (i.e., that my own `PluginTurnPayload`/`pendingCoalescedTurns` bookkeeping didn't accidentally reintroduce a race). Also relevant: `submitPluginTurn (...) > surfaces Rejected once an actor's mailbox is genuinely full...` proves backpressure surfaces as a rejection rather than an unbounded queue, matching the brief's explicit requirement that the shard worker's own "rejects overlapping turns rather than queueing them" guarantee is preserved end-to-end.

## metrics-publisher ownership decision

Left `autoStartMetricsPublisher` at its default (`false`) in `getActivationRegistry()` — **PluginRuntime is not the right owner.** `getActivationRegistry()` is a lazy, shared, module-wide singleton constructed on the FIRST `loadPluginModule` call from anywhere (a plugin boot, not a user action). Turning the publisher on here would start a real 2 Hz `setInterval` the moment the first plugin loads, for the lifetime of the tab, regardless of whether anyone is watching — `ActivationRegistry`'s own header doc says its ONLY real subscriber is the task-manager window (`TaskManager/🟦️component.tsx`, itself still registrar-only/unmounted work), which `ShellHost` (registrar-only, outside this lease) would mount. That is the deliberate-choice construction site the option's own doc asks for, not this file. Turning it on here would silently change a shared default for every other current/future consumer of the singleton — exactly what the opt-in default exists to prevent.

Also considered and rejected reusing `ActivationRegistry.enqueueTurn` for the turn-dispatch fix itself (item 1): its `onTurnResult(actorId, result)` fires once per REGISTRY, not once per caller, so it cannot correlate a specific caller's awaited result once more than one turn can be pending per actor (exactly what coalescing needs) — a dedicated `TurnScheduler` whose `runTurn` seam I control was necessary, mirroring the SAME "one dedicated scheduler per consumer" pattern `ActivationRegistry` already uses internally, not a reinvention.

## commands + exit codes

Baseline (git-committed, pre-edit copy of this file, run via a ticket-folder scratch config with import aliases repointed to the same real dependency files):

```
bunx vitest run --config ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-web-plugin-runtime-scratch/baseline.vitest.config.ts" --reporter=verbose
...
 Test Files  1 passed (1)
      Tests  13 passed (13)
   Start at  22:36:01
   Duration  1.79s
EXIT: 0
```
(full output: `terra-web-plugin-runtime-baseline.txt`)

After (the real, edited file, in place — same command style, pointed at the live path):

```
bunx vitest run --config ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-web-plugin-runtime-scratch/after.vitest.config.ts" --reporter=verbose
...
 Test Files  1 passed (1)
      Tests  26 passed (26)
   Start at  22:36:19
   Duration  534ms
EXIT: 0
```
(full output: `terra-web-plugin-runtime-after1.txt`)

Whole-repo TypeScript check (this file's package has no isolated tsc target; used the root config, which covers `**/*.ts`/`**/*.tsx` repo-wide):

```
bunx tsc --noEmit -p tsconfig.json --skipLibCheck
EXIT: 2
```
Exit 2 with 19 error lines, ALL in three files I never touched (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/🟦️component.ts`, two `stdio` schema files, `repo-coordinator`'s vscode `extension.ts`) — pre-existing syntax errors unrelated to this packet (raw parse errors like "Property or signature expected", not type errors in anything I touched). Zero lines mention `PluginRuntime`. Full output saved at `terra-web-plugin-runtime-tsc.txt` in this ticket folder.

## baseline vs after (named sets)

Baseline named tests (13, all passing) — unchanged, still passing verbatim in "after":
- `PluginRuntime TransactionCoordinator` × 11 (proposal→prepare→commit reverse order; undoGroup fan-out; commit-failed rollback; unknown-target ×2 [missing initiator handle / missing InstanceDirectory entry]; unknown-mutation; contribution-not-permitted; contributed-mutation plan; cycle; depth-exceeded; rejection-code passthrough).
- `PluginRuntime documentPack/transaction wire adapter` × 2 (documentPack/transaction frame round-trip; documentPack cache after loadAppDocumentPack).

New named tests (13, all passing), grouped exactly as required by the brief:
- `submitPluginTurn (...)`: lane priority (Interactive preempts already-queued UserVisible); 200-call coalescing burst collapses to 1 dispatch with every waiter resolved; Rejected surfaces at mailbox capacity; one-turn-at-a-time-per-actor under cross-actor interleaving.
- `PluginRuntime shard-loss wiring (...)`: `handlePluginShardLost` delegates to `ActivationRegistry.handleShardLost` for exactly the affected actorIds; `buildShardClientOptions` wires `onShardLost`/`shardCount` correctly.
- `fetchDescriptorManifest AbortSignal`: propagates an abort; still falls back on a genuine non-abort failure.
- `loadPluginModulesInDependencyOrder — level-parallel boot`: independent siblings parallel within a level / dependent waits for the whole level; concurrency bound respected; runtime-failure cascade skips dependents while unrelated siblings still load; default bound is `poolConcurrency()`; abort mid-boot stops new starts while an in-flight load still settles.

13 + 13 = 26, matching both runs exactly. Counts alone would have been meaningless here (13→26 is just "13 new tests", not a regression signal) — the point is every ORIGINAL name is still present and green.

## lease-requests

None. Everything needed lived inside the owned path; `ActivationRegistry`/`ShardClient`/`TurnScheduler` were consumed as already-landed, unmodified.

## honest gaps

- `BoundedMailbox.enqueue`'s `dropped` backpressure (an actor's mailbox at capacity, evicting the lowest-priority nonempty lane below the incoming one) has NO callback for the evicted envelope — a caller whose turn was silently evicted this way would see its promise never settle. Mitigated, not eliminated: every call site here uses only two lanes (`Interactive`/`UserVisible`), and `UserVisible` traffic is deduplicated to at most one pending envelope per actor by `pendingCoalescedTurns` before it ever reaches the mailbox — so this can only bite if a single actor accumulates more than 32 genuinely distinct `Interactive` turns, which no real call site here produces. Documented in the code (`submitPluginTurn`'s own doc comment) rather than silently risked.
- `getShardClient()`'s own 3-line body (real `new Worker(...)` construction + `.startWatchdog()`) is not exercised by an automated test in this suite — there is no `Worker` polyfill available. What IS tested directly: `buildShardClientOptions` (the options object `getShardClient` passes to `new ShardClient(...)`, including the real `onShardLost` wiring) and `handlePluginShardLost` (the actual restore delegation). `ShardClient.startWatchdog`/`checkHeartbeats` themselves are already tested in `🧵️shard-client.ts`'s own suite, not re-tested here.
- `computeDependencyLevels` derives levels only from `entry.dependencies` on the entries passed to THIS call — a dependency id not present in the caller's own `entries` array (e.g., a cross-boot dependency already loaded in a previous call) contributes no ordering edge, same fail-soft posture `orderPluginRegistryEntries` already has for a missing id, but worth flagging since `loadPluginModulesInDependencyOrder` has no real caller yet to reveal whether that's the right posture for the eventual real boot orchestrator.
- No PR/build-system integration for these new tests — they remain inline `import.meta.vitest` in a file with no real vitest project (same pre-existing gap `📓️w2-b-report.md` already flagged; wiring a real project target is a `project.json`/registrar-scoped change outside this lease). Verified they DO execute by name (`--reporter=verbose`, both runs above) specifically because of the documented "silently not run" trap.
