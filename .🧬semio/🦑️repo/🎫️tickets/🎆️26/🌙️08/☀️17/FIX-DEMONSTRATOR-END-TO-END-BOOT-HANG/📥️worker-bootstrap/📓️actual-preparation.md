# Actual Worker Metadata Preparation — Contract Before Runtime Edits

Date: 2026-08-28. Scope approved by runtime coordinator: only actual Shard retained-cell preparation; original eager constructor/fake Worker remains unadmitted setup. No worker, handler, SAB, receiver, request or factory cutover.

## Canonical Source And Methods

Source declaration/neutral vectors: actor/🏘️composition/🏗️bootstrap/{🧬️schema.json,🧪️fixture.json}. Actual implementation and in-source tests remain in the existing ShardClient file.

```typescript
prepareWorkerBootstrap(grant: ResidentGrant): ResidentStep
closeWorkerBootstrapStep(grant: ResidentGrant): ResidentStep
```

The second method is the **original worker-metadata preparation close driver**, not worker/platform close, UI close, a boolean retirement authority or a new terminal witness. Its original client identity is the method receiver; there is no index/current-route lookup or caller-supplied cell/record.

## Ownership And Close Phases

The existing shared controller208 becomes224 and remains under its original record/cell (784 total). It owns the shared purpose word before UI/worker-specific use. Its original UI-named fields may not let UI `retired` erase shared-root ownership. A UI-close-then-worker-first call uses the live shared record. A never-started UI close may leave no root; later worker preparation can initialize the shared prefix without reopening the UI pool.

The eight worker-only words are separately declared128 with their original record/cell688. Their future child-admission fields remain empty. No extra close flag, outcome object, witness, closure, map or neutral capability is introduced; existing phase words encode forward/closing/observing/known-refusal/ambiguous-fault frontiers. Any additional actual field must be priced before source release.

Close stops only this worker preparation. During the shared prefix it leaves the shared original root/cell owned by the original client for a later UI continuation. It does not invoke `closeUiResidentPoolStep`. Before a worker record exists, a healthy unused worker cell may release its pending association and then retire with its genuine private cell witness, each on a separate grant. Only then may the original worker cell slot be cleared.

Once a worker record has been admitted, **even before installation**, this bounded slice retains it and its cell. It funds real worker-controller fields needed by subsequent observations. Close enters a stopped record-held phase and returns blocked; it never refunds that record, drives its intrinsic close or claims whole-controller terminal. Installed record refund remains impossible without the later original-client terminal join. This deliberately avoids an early refund while the controller still owns its own observation/cell/fault fields.

A first arbitrary fault remains exact and unread. Same-fault replay is stable; a different original fault is refused to its actual caller. Known canonical pending-release followed by a separately granted original observation can free serialization while the fault cell remains charged/nonterminal. Null after a wrapper throw alone cannot do so. If that observation is unavailable the exact purpose, cell and original fault stay retained; no close success is claimed.

Cancelled/record-held worker preparation does not restart. UI may still prepare after worker-specific cancellation when the shared root is live and the exact global pending handoff has cleared. Thus the required `worker-partial-cancel-then-ui` cases distinguish a retired unused worker cell from a stopped, still-charged worker record.

## Before-Implementation Tests

The first actual source gate must reproduce both the absent preparation method and current recovery-only `matchesShell` behavior at shared prefixes7/8 under ledger/record/cell/fault closing. It must capture the original neutral record through actual reserve calls and execute the actual public UI method, not only a text oracle or an inert neutral shell.

Subsequent tests will cover all22 gate endings with original retained observations and separate grants, shared UI/worker interleaving, UI-close-then-worker-first, worker-partial-cancel-then-UI, exact first/distinct faults and wrapper calls which execute the real primitive before throwing. New worker/post/handler/SAB counters stay zero relative to existing fake-worker setup. The original actor fixture/UI1608 baseline will change only at coherent source handoff, coordinated with UI.

No production implementation or test pass is claimed by this declaration. The earlier refinement oracle remains historical scoped evidence.

## Actual First Source RED — Before Production Edits

Executed existing Actor target with selector `ShardWorkerBootstrap`: **1PASS/8FAIL/180skip189**, 9 collected files,711ms,start05:20:54,Nx exit1. Six selected pre/post hashes matched. Production method/root changes had not begun. The missing-method case failed; six closing cases returned pending instead of rejected, and the prefix8 closed-ledger case called prepareAdmission despite the closed shared root. Prefix8 with an already faulted shared cell was the single passing law.

Raw output is preserved at [actual-preparation-red-1.log](./🧪️actual-preparation-red-1.log). Exact source before new tests was98710401ee3d18c95536fa64a8e7cfabd09e9ba06adf8d745792d5d452376a73; test-only source wasf144c0e111bbfd30bed145554f0846ce6457c14d2f576acce05c6aa7c8e704fc. This is an actual source/runtime RED, not a new bootstrap implementation or whole-suite regression claim.

```text
❯ |@semio-tech/framework-actor| 🧵️shard-client.ts (132 tests | 8 failed | 123 skipped) 66ms
   × ShardWorkerBootstrap declares only original metadata preparation and close methods 57ms
   × ShardWorkerBootstrap shared closing prefix 7 ledger cannot admit a UI descendant 3ms
   × ShardWorkerBootstrap shared closing prefix 7 record cannot admit a UI descendant 1ms
   × ShardWorkerBootstrap shared closing prefix 7 cell cannot admit a UI descendant 0ms
   × ShardWorkerBootstrap shared closing prefix 7 fault cannot admit a UI descendant 0ms
   × ShardWorkerBootstrap shared closing prefix 8 ledger cannot admit a UI descendant 2ms
   × ShardWorkerBootstrap shared closing prefix 8 record cannot admit a UI descendant 0ms
   × ShardWorkerBootstrap shared closing prefix 8 cell cannot admit a UI descendant 0ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 8 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap declares only original metadata preparation and close methods
AssertionError: expected 'undefined' to be 'function' // Object.is equality

Expected: "function"
Received: "undefined"

 ❯ ð§µï¸shard-client.ts:1921:65

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/8]⎯

 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 7 ledger cannot admit a UI descendant
 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 7 record cannot admit a UI descendant
 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 7 cell cannot admit a UI descendant
 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 7 fault cannot admit a UI descendant
 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 8 record cannot admit a UI descendant
 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 8 cell cannot admit a UI descendant
AssertionError: expected 'pending' to be 'rejected' // Object.is equality

Expected: "rejected"
Received: "pending"

 ❯ ð§µï¸shard-client.ts:1938:113

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/8]⎯

 FAIL  |@semio-tech/framework-actor| 🧵️shard-client.ts > ShardWorkerBootstrap shared closing prefix 8 ledger cannot admit a UI descendant
AssertionError: expected "prepareAdmission" to not be called at all, but actually been called 1 times

Received:

  1st prepareAdmission call:

    Array [
      ShardClient {
        "activationGeneration": 0n,
        "actorActivations": Map {},
        "actorShard": Map {},
        "createWorker": [Function createWorker],
        "effectReplySeq": 0,
        "exclusiveIndices": Set {},
        "heartbeatSabView": null,
        "heartbeatTimeoutMs": 5000,
        "instanceLifecycles": Map {},
        "instanceTurns": WeakMap {},
        "maxOutstandingEffectsPerActor": 64,
        "nextRoundRobin": 0,
        "now": [Function now],
        "onActorTrap": undefined,
        "onHostEffect": undefined,
        "onShardLost": undefined,
        "outstandingEffectsByActor": Map {},
        "pending": Map {},
        "requestSeq": 0,
        "shards": Array [
          Object {
            "actorIds": Set {},
            "available": true,
            "heartbeat": Object {
              "lastHeartbeatAtMs": -Infinity,
              "lastHeartbeatTurnSeq": 0,
              "lastMissCountedAtMs": 0,
              "missedCount": 0,
              "oldestPendingStartedAtMs": null,
            },
            "index": 0,
            "pendingRequestIds": Set {},
            "worker": FakeShardWorker {
              "index": 0,
              "onerror": [Function anonymous],
              "onmessage": [Function anonymous],
              "sent": Array [],
              "terminated": false,
            },
          },
        ],
        "watchdogHandle": null,
        "watchdogIntervalMs": 5000,
      },
      "data",
      Object {
        "maxBytes": 296,
        "maxItems": 1,
      },
    ]


Number of calls: 1

 ❯ ð§µï¸shard-client.ts:1938:151

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[3/8]⎯


 Test Files  1 failed | 8 skipped (9)
      Tests  8 failed | 1 passed | 180 skipped (189)
   Start at  05:20:54
   Duration  711ms (transform 868ms, setup 0ms, import 1.24s, tests 66ms, environment 1ms)

Warning: command "bun ./📜️script.ts test --testNamePattern=ShardWorkerBootstrap" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-actor failed

Failed tasks:

- @semio-tech/framework-actor:test

Hint: run the command with --verbose for more details.
```

Stable selected manifest:

```text
f144c0e111bbfd30bed145554f0846ce6457c14d2f576acce05c6aa7c8e704fc  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
e5f5ca88b9ae4fc3884a8151e1548ad571a82a5d4f15d19765b0414a4ff14d88  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧬️schema.json
7bce8e78a5b70f64b6f6fee7da3bd938dcde5b2d1247c473690529da918772e6  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🏗️bootstrap/🧪️fixture.json
72b0e0ba9bab57f7b95c988c40171e7237b0e5ca00e84b063df03e2c3edd6530  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts
```

