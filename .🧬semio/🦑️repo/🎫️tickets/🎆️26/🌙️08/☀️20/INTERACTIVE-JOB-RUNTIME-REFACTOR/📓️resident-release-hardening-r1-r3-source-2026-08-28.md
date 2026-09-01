# Resident Release Hardening and Current-API Baseline

## Outcome

Ticket-only hardening complete. Actual source R1 reproduced duplicate/missing frontier acceptance; source R2 and final-source R3 passed strict Ajv plus the Immer reference: **7 cases, 9 frontiers, 58 transitions, 17 hostile inputs**. These are not native tests or allocator observations. All five selected R3 inputs were unchanged across the run.

The seven proposed native laws now require exact bytes/slots/owners refund and original Data/Control partition conservation. A separate smallest **current-public-API** semantic baseline is staged without any proposed Release or Layout fields. Neither baseline nor future seven has been mounted or compiled. Canonical resident authority and existing17 tests remain `508b7872…` / `ebde45c9…`; no production/API/dependency edit or native command occurred.

This report supersedes only the initial packet's unexecuted-reference status and initial four-file hashes. The original declaration and source receipt remain historical at `📓️resident-original-root-free-refund-packet-2026-08-28.md`. Original resident R8/R9 and Opening evidence are untouched.

## Frontier Desired-Law RED and Repair

The prior schema checked array length nine and broad scalar ranges but admitted nine copies of one frontier. R1 first added twelve actual desired-law negatives to the controller:

- Nine replacements: remove each original `(kind, calls)` in turn, duplicate another pair but alter its destruction counter. This prevents `uniqueItems` alone from masquerading as tuple uniqueness.
- Nine identical copies of the first row.
- An eight-row missing case.
- A nine-row inventory with unsupported `(record, 3)` replacing `(record, 2)`.

R1's actual accepted vector was `[true,true,true,true,true,true,true,true,true,true,false,true]`: eleven malformed inventories passed; only the eight-row length violation was already rejected. The assertion failed before the phase/reference model ran. No phase-model PASS is attributed to R1.

The repair adds nine schema `contains` requirements, one for each exact required `(kind,calls)` pair, alongside exact length nine. Each pair must therefore appear exactly once regardless of its output counters. The independent Immer page-initialization model still checks the destruction/free output values. No native code or fixture expectation was relaxed.

## Exact All-Axis Native Changes

The future record law executes the same original-root construction in both directions:

1. Consumer in Control; admission and record in Data.
2. Consumer in Data; admission and record in Control.

The total test capacity is unchanged: exactly the same three original page Layouts and three slots/owners. Only the subdivision changes. This is not a quota increase or a new live composition policy. The unrelated partition has a real nonzero consumer charge, not a fabricated sentinel.

Snapshots now retain both full `ResidentResources` triples. For a record with no additional envelope, exact expected refund is `Layout<RecordNode<Shell>>.size / 1 slot / 1 owner` in its original partition. Destruction and actual free must leave both triples unchanged. Refund must subtract exactly that triple from the original partition and zero from the other; Clear must leave both triples unchanged. The original consumer charge in the other partition must remain exact. Final cleanup requires both full triples and actual allocated bytes to be zero.

Short-grant comparisons now include both full triples. The concurrent law distinguishes the exact still-charged Refund state from the already-refunded Clear state on all axes. The poison law requires both full triples zero after its permitted pointerless cleanup. All seven use the same strengthened final conservation check where healthy public reads are available; the poison test observes its exact original state under exclusive test access without clearing poison.

No work grant exceeds 4096. Native Layout values and these actual state transitions remain unexecuted.

## Current-API Semantic Baseline

Staged file: `🧪️resident-release/🧪️baseline/🦀️.rs`.

Exactly one test: `resident_current_api_charge_remains_after_allocator_return`. It uses only existing public `ResidentLedgerRoot` APIs, existing `ResidentNativeLayout` fields, and the existing test fixture capacity. It does **not** import any proposed Release type, inspect private LedgerState fields, or require the future seven's Layout additions.

For each schema-declared Data/Control case, two actual `prepare_consumer::<u64>` calls produce one reserved raw allocation. No C payload, record, admission, alias, extra Box, or private parent receiver is created. The test keeps the real root outside the observation closure, calls existing close steps until the existing allocator actually returns from deallocation, then captures public usage and allocated bytes immediately after that call. It finishes actual root cleanup and verifies both partitions/allocated bytes zero **before** intended assertions/logging.

The desired assertion is that the original bytes/slots/owners are still charged immediately after the physical free, with the unrelated partition unchanged. Current source refunds in the same call before freeing; the expected semantic RED is that this triple is already zero. That is a prediction from source, not an executed result. On the first failing Data assertion, Control would remain unexecuted; do not credit both rows from a partial failure. A later passing run must execute both.

The hook observes `System::dealloc` **return**, not the old pre-call counter. Its fixed thread-local count/size/alignment is enabled only around the actual root close call. It never stores/dereferences freed pointers, calls the root, allocates, locks, asserts, formats, or panics inside the allocator. The diagnostic is not refund authority. No allocator-unwind injection is used. Scope teardown disables observation even if ordinary outer code unwinds.

The cleanup bound is the declared one-owner close phase ceiling plus ceiling-divided current public root/consumer/descriptor layouts at 4096. Exhaustion is an explicit incomplete cleanup failure, not a claim of general liveness. No strict live Store payload is present in this baseline.

## Minimal Future Include/Hook Delta — Not Applied

Only `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs` would change for the baseline capture. The exact repository-relative file path below was read-only checked to resolve to the staged baseline:

```rust
#[path = "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🧪️baseline/🦀️.rs"]
mod release_baseline;
```

Inside the **existing** `ObservedAllocator::dealloc`, preserve all existing pre-call diagnostics and change only its tail from the current single System call to:

```rust
unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, ptr, layout); }
release_baseline::observe_system_dealloc_returned(layout);
```

The baseline module owns its fixed observer; there is no second global allocator or production hook. The existing allocator-owning parent module supplies only its current capacity helper/imports. The future seven remain unmounted and their missing types must not enter this semantic baseline graph.

After root/OS6 coordination and an explicit source mount/release, the sole executor can use the existing no-argument exhaustive `@semio-tech/value-resident-rs:test` route. The package controller was read: it rejects arguments and invokes the existing shared budget with `--lib`. Expected roster is existing17 plus this one baseline, not future seven. Do not add a selector flag, new route, compiler, or Wasm run for a deliberate RED. This packet does not authorize dispatch by itself.

## Executed Source Commands and Complete Output

All three runs used precisely the existing explicit one-project route, from `/Users/ueli/Documents/semio`:

```sh
bun x nx exec --projects=@semio-tech/framework-plugin -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/📜️script.ts'
```

No native process was launched. R1 session73558 exited1. R2 session84175 exited0. R3 session29209 exited0. R3 followed a ticket-only Rust correction from `Result != Ok(true)` to `matches!(..., Ok(true))`, avoiding an unnecessary `AccessError: PartialEq` requirement in both staged observers; the reference controller does not compile that Rust. It is not native evidence.

### R1 Actual RED — Complete Captured Output

```text
[DEBUG] frontier desired-law accepted=[true,true,true,true,true,true,true,true,true,true,false,true] expected=12false
17 | const duplicated = structuredClone(fixture); duplicated.frontiers = Array.from({ length: 9 }, () => ({ ...fixture.frontiers[0] })); frontierNegatives.push(duplicated);
18 | const missing = structuredClone(fixture); missing.frontiers.pop(); frontierNegatives.push(missing);
19 | const foreignTuple = structuredClone(fixture); foreignTuple.frontiers[8].calls = 3; frontierNegatives.push(foreignTuple);
20 | const frontierAccepted = frontierNegatives.map((value: unknown) => validate(value));
21 | console.log(`[DEBUG] frontier desired-law accepted=${JSON.stringify(frontierAccepted)} expected=12false`);
22 | assert.deepEqual(frontierAccepted, Array(12).fill(false), "every declared kind/call must occur exactly once, regardless of output counters");
            ^
AssertionError: every declared kind/call must occur exactly once, regardless of output counters
+ actual - expected

  [
+   true,
+   true,
+   true,
+   true,
+   true,
+   true,
+   true,
+   true,
+   true,
+   true,
    false,
+   true
-   false,
-   false,
-   false,
-   false,
-   false,
-   false,
-   false,
-   false,
-   false,
-   false,
-   false
  ]

 generatedMessage: false,
     actual: [
  true, true, true, true, true, true, true, true, true, true, false, true
],
   expected: [
  false, false, false, false, false, false, false, false, false, false, false, false
],
   operator: "deepStrictEqual",
       code: "ERR_ASSERTION"

      at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/📜️script.ts:22:8

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/📜️script.ts"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: 1,
  signal: null,
  output: [ null, null, null ],
  pid: 77012,
  stdout: null,
  stderr: null
}
```

### R2 Actual GREEN — Complete Captured Output

```text
[DEBUG] frontier desired-law accepted=[false,false,false,false,false,false,false,false,false,false,false,false] expected=12false
[DEBUG] resident release reference=immer cases=7 frontiers=9 transitions=58 nativeRoster=7 hostile=17 nativeExecuted=0 productionMounted=0
```

### R3 Final-Source GREEN — Complete Captured Output

```text
[DEBUG] frontier desired-law accepted=[false,false,false,false,false,false,false,false,false,false,false,false] expected=12false
[DEBUG] resident release reference=immer cases=7 frontiers=9 transitions=58 nativeRoster=7 hostile=17 nativeExecuted=0 productionMounted=0
```

## Selected SHA-256 Receipt

R1 used original schema `2e9c7d9e…`, original fixture `076d975b…`, unchanged initial future Rust `fc755c9b…`, and the new desired-law controller `807f0e83…`. R2's five pre/post hashes were identical; only subsequent Rust observer spelling changes distinguish R3. R3's final five pre/post hashes were identical:

| File under `🧪️resident-release` | Final R3 SHA-256 |
| --- | --- |
| `📜️script.ts` | `807f0e83920b23013d27d5d4f02e2fbc03accabc93b2f195a31f1312ae9d7354` |
| `🔣️.json` | `2c82d7ad51115a6c5d2dc85bec5d0b2c31818275dcd4f68d7995d6556dcf828c` |
| `🧬️schema/🔣️.json` | `49dea5839829f00466753e75be20ff618e62a74669749b9572adf181061119c4` |
| `🦀️.rs` | `8949cb8507c798758108a5b77d01221d8c87ff9d5feb2cd4f8522cca67d55019` |
| `🧪️baseline/🦀️.rs` | `2e73f918e3100c7a232edf22032edfe87ab225ba2d40fa232c3814d5c4420c6f` |

R2 Rust/baseline hashes, preserved rather than relabeled as final: `1edaa078b532e370958e4201d6dfa479159559defde7a6b48b3be4e8ec179a75` / `facfed40dbfb0a0ab58970bfa3230b6b8661888f73135d6c807a5e15203b5401`. The other three R2 hashes equal R3.

Canonical resident authority remained `508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f`; existing17 tests remained `ebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e` at final observation. Opening7 was not edited. No native lease is held; no test process remains active from these three source runs.
