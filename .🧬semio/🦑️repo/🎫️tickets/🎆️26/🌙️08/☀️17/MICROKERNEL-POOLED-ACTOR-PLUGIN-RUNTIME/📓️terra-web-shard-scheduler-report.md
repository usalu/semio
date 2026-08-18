# terra-web-shard-scheduler — report

Executor: `terra-web-shard-scheduler`. Coordinator: sol. Packet: web-shard-scheduler ("the web shard stops being FIFO").

## delivered

1. **`🧵️turn-scheduler.ts`** (new file, `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts`) — `TurnScheduler<TPayload, TBudget>`:
   - one `createBoundedMailbox` (from the already-landed `📬️mailbox.ts`) per actor, reused as-is — no changes to `mailbox.ts`.
   - `enqueue(actorId, { lane, coalesce?, payload })` never dispatches inline; it schedules a microtask pump so a whole synchronous batch of `enqueue` calls lands before the first pick is made — otherwise lane priority could never out-rank an arrival that merely happened first. Returns the mailbox's own `Backpressure` verbatim (`accept | coalesced | dropped | rejected`).
   - cross-actor pick: for each lane in priority order (`Interactive > UserVisible > Background > Maintenance`), the first not-busy actor with a pending envelope in that lane wins (`Map` insertion order as the FIFO tie-break). Tracked via a per-actor `Record<Lane, number>` pending-count, kept in sync purely from each `Backpressure` result — no `peek()` needed on `BoundedMailbox` (which deliberately doesn't expose one).
   - one turn in flight per actor at a time: an actor is marked busy the instant its envelope is popped and freed only when `runTurn`'s promise settles; other actors dispatch concurrently in the same synchronous pump pass.
   - `cancelQueued(actorId)` drains and discards every still-queued (not yet dispatched) turn for that actor, returns the count; `teardownActor(actorId)` does that plus forgets the actor's mailbox/bookkeeping entirely (for suspend/dispose). An in-flight turn is untouched by either — it settles on its own, matching `ShardClient.terminate`'s existing in-flight-rejection contract.
   - `onTurnError` reports a rejected `runTurn` instead of throwing out of the pump loop, and the actor is still freed for its next turn.
   - `isBusy`/`pendingCount` introspection.

2. **`🧵️shard-client.ts` — watchdog self-tick.** Added `startWatchdog(intervalMs = watchdogIntervalMs)` / `stopWatchdog()`. Verified first (grep, see below) that **nothing in production called `checkHeartbeats`/`pollHeartbeatSab` at all** — only this file's own tests did. `startWatchdog` ticks both on a real `setInterval`, idempotent, `stopWatchdog` idempotent and also called from `disposeAll()`. Followed this repo's own existing convention for this exact kind of loop (`ActivationRegistry.startRuntimeMetricsPublisher` in `🎠️kernel/🟦️component.ts`) — real timer + `vi.useFakeTimers()`/`vi.advanceTimersByTime` in tests, no injectable interval-function parameter needed since `now` is already injectable and fake timers cover the rest.

3. **`failShard` now clears routing.** Verified by reading the pre-change source: `failShard` rejected in-flight requests and reset heartbeat bookkeeping but left `this.actorShard`/`slot.actorIds` completely untouched. Concretely: `worker.onerror` calls `failShard` directly (it does **not** call `terminate()`/`rebuild()`), so before this fix a crashed-but-not-yet-3-strikes shard kept every one of its actors routed to it — a later `activate()`/`turn()` for the same actor would `postMessage` into the dead worker and hang until the heartbeat watchdog eventually caught it. Fixed by having `failShard` also delete each `slot.actorIds` entry from `this.actorShard` and clear `slot.actorIds`. `terminate()`/`rebuild()` still work unchanged (they capture/iterate a snapshot or an already-empty set — harmless either way).

## public signature changes + call-site counts

**None of `ShardClient`'s existing public method/constructor signatures changed.** Everything added is additive:
- `ShardClientOptions.watchdogIntervalMs?: number` (new optional field)
- `ShardClient.startWatchdog(intervalMs?): void` (new method)
- `ShardClient.stopWatchdog(): void` (new method)

Call-site counts gathered before touching anything (`grep -rn` across the whole repo, excluding `node_modules`):
- `new ShardClient(` — **3 sites**: this file's own test harness, `🎠️kernel/🟦️component.ts:1780` (production), `…renderer/…/TaskManager/🧪️component.test.tsx:125` (test). Unaffected — constructor is additive-only.
- `.turn(` on a `ShardClient` — **0 production sites outside this file's own tests** (the one repo hit outside is an unrelated `.turn()` on a different `c` object in an `.old.tsx` file from a different ticket).
- `checkHeartbeats(` / `pollHeartbeatSab(` — **0 call sites anywhere outside this file's own tests**, confirming the ticket's claim verbatim: the watchdog was wired but nothing ever turned the crank in production.

The one *behavioral* (not signature) change is `failShard` now also clearing `actorShard`/`slot.actorIds`. This is exercised by the real production consumer (`🎠️kernel/🟦️component.ts`'s `ActivationRegistry`, which owns the one production `new ShardClient(...)`) — see verification below; its own 17 inline tests still pass unchanged.

## budget seam

`TurnSchedulerOptions.budgetFor: (actorId: string) => TBudget` is called **fresh on every dispatch**, immediately before `runTurn`, never cached or computed once per actor. This is deliberately the seam: once the native `ShardFrame::Grant{actor, budget, envelopes}` DRR wire lands, a caller swaps `budgetFor` for a provider that reads the latest grant for that actor (e.g. from a small map the transport keeps updated) — no change to `TurnScheduler` itself. Today a caller can simply return the same constant `ShardBudget` every time. `TurnScheduler` has zero dependency on `ShardClient`/`ShardBudget` (no import), by design, so this seam works whether the eventual caller wires `runTurn` to the current interim `ShardClient.turn()` (opaque JSON envelope) or to whatever native-parity transport lands later.

## commands + exit codes (verbatim)

Baseline, before any of my changes:
```
$ cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript" && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript

 Test Files  4 passed (4)
      Tests  38 passed (38)
   Start at  19:09:18
   Duration  193ms (transform 204ms, setup 0ms, import 231ms, tests 64ms, environment 1ms)

EXIT:0
```
Matches the packet brief's stated baseline (38 passed / 0 failed).

After all changes, exact acceptance command:
```
$ cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript" && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript

 Test Files  6 passed (6)
      Tests  58 passed (58)
   Start at  19:18:54
   Duration  138ms (transform 194ms, setup 0ms, import 226ms, tests 79ms, environment 1ms)

EXIT:0
```

Cross-check with a second, independent consumer — the production `ActivationRegistry` in `🎠️kernel/🟦️component.ts` (which is byte-frozen territory I did **not** edit, but reading + running its own inline tests is not an edit) still passes unchanged after the `failShard` fix, using the diagnostic vitest config a sibling packet (terra-t1) already left in this ticket folder for exactly this file:
```
$ bunx vitest run --config ".🧬semio/…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-t1-kernel-vitest.config.ts"
 RUN  v4.1.10 /Users/ueli/Documents/semio

 Test Files  1 passed (1)
      Tests  17 passed (17)
   Start at  19:20:59
   Duration  614ms (transform 80ms, setup 0ms, import 93ms, tests 8ms, environment 378ms)

EXIT:0
```

## baseline vs after + proof new tests ran

Baseline: **38 passed / 0 failed** (4 test files — `📬️mailbox.ts` + `🧵️shard-client.ts`, each counted twice by this repo's vitest setup; that doubling is pre-existing and unrelated to this packet).

After: **58 passed / 0 failed** (6 test files — the same two plus the new `🧵️turn-scheduler.ts`, still each counted twice) = **20 net new passing test instances = 10 new unique tests × 2**.

Ran with `--reporter=verbose` to prove every new test executed **by name**, not silently skipped (the explicit-filename `include`/`includeSource`/`coverage.include` arrays in `🧪️vitest.config.ts` make a new sibling file invisible until added — I added `🧵️turn-scheduler.ts` to all three arrays first, then confirmed with verbose output). New test names that appeared in the run:
```
🧵️turn-scheduler.ts > TurnScheduler lane priority > dispatches by lane priority, not arrival order, when a batch lands before the first pick
🧵️turn-scheduler.ts > TurnScheduler per-actor ordering under interleaving > never starts an actor's next turn before its current one settles, even while other actors interleave
🧵️turn-scheduler.ts > TurnScheduler coalescing > collapses a burst of same-key envelopes to one queued turn, never 200 deep
🧵️turn-scheduler.ts > TurnScheduler backpressure at the cap > rejected surfaces synchronously at the cap instead of the queue growing past it
🧵️turn-scheduler.ts > TurnScheduler cancellation > cancels only queued turns, leaving an in-flight one to settle on its own
🧵️turn-scheduler.ts > TurnScheduler cancellation > teardownActor cancels queued work and forgets the actor so a later enqueue starts fresh
🧵️turn-scheduler.ts > TurnScheduler onTurnError > reports a rejected runTurn instead of throwing out of the pump loop, and keeps draining
🧵️shard-client.ts > ShardClient.startWatchdog / stopWatchdog > self-ticks checkHeartbeats + pollHeartbeatSab with no external caller, detects a missed heartbeat, and rebuilds
🧵️shard-client.ts > ShardClient.startWatchdog / stopWatchdog > is idempotent to call twice, and stopWatchdog before ever starting is a no-op
🧵️shard-client.ts > ShardClient failShard clears routing > clears actorShard + slot.actorIds immediately on a worker crash (onerror), before any terminate()/rebuild()
```
All 10 unique new tests visible above with a `✓`, each executed by its full describe/it name — none silently skipped. No real sleeps anywhere: async ordering uses `queueMicrotask`-based flushing and manually-controlled deferred promises; the watchdog self-tick tests use `vi.useFakeTimers()` + `vi.advanceTimersByTime` (this repo's own existing convention, see `ActivationRegistry.startRuntimeMetricsPublisher`'s test), wrapped in `try { … } finally { vi.useRealTimers(); }` so real timers are restored even on assertion failure.

## lease-requests

None. Everything landed inside the two owned paths (`🧵️shard-client.ts`, new `🧵️turn-scheduler.ts`) plus the pre-authorized `🧪️vitest.config.ts` edit (adding the new file's name to the three explicit include arrays — required, or the new file's tests silently would not run).

## honest gaps

- **`onerror` still doesn't self-heal the worker.** The fix makes `failShard` clear routing immediately (so no *new* work gets sent into a crashed worker), but a crash via `worker.onerror` alone (outside the 3-missed-heartbeat ladder) does **not** call `terminate()`+`rebuild()` — the dead `ShardWorkerLike` object is still sitting at `this.shards[index]`. If a caller round-robins a brand-new actor onto that same index before the heartbeat watchdog's 3-strike ladder gets there, it will assign to a dead worker again (though now at least any *previously* routed actor is safely unrouted and can be reassigned/re-activated elsewhere by the caller). Making `onerror` auto-`terminate()`+`rebuild()` felt like a larger, more opinionated behavior change than "verify whether `failShard` clears routing; fix if not," so I left it as scoped and am flagging it here rather than silently expanding scope.
- **`TurnScheduler` is not yet wired to `ShardClient`/`ActivationRegistry`.** By design (transport-agnostic, no `ShardClient` import) and because the actual wiring point (`🎠️kernel/🟦️component.ts`'s `ActivationRegistry`) is explicitly byte-frozen peer territory I must not touch. The class is fully additive, tested standalone, and ready for whichever packet owns that wiring.
- **Empty per-actor mailboxes/lane-count entries are never garbage-collected** once an actor drains to zero without an explicit `teardownActor` call — a minor, bounded memory footprint (one small object per ever-seen `actorId`), not a correctness issue, but worth a follow-up if actor churn turns out to be high.
- I did not run the `TaskManager` component test suite or a broader renderer-level vitest project (no dedicated vitest config exists next to `…/TaskManager/🧪️component.test.tsx` — it's presumably part of a larger renderer-level project I didn't chase down) — the one `new ShardClient(...)` there is an additive-surface, low-risk call site, and I verified the real production consumer (`ActivationRegistry`, 17/17) instead.
