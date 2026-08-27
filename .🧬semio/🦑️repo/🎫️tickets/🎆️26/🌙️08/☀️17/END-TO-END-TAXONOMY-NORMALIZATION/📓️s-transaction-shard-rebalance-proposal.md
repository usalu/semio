# Transaction v2 Exact Shard Rebalance Proposal

## Recommendation

Move exactly one existing test from shard 2 to shard 1:

`recovers parent-killed transaction-attempt-canonical-published`

This is a read-only proposal, not an implemented runner change or a passing performance result. All 62 titles, all 98 boundary identities, test bodies, one-wave scheduling, watchdog placement, timeouts, and the existing per-shard concurrency values are preserved. No expensive tests, old bundle execution, global plan, production apply, or actual Compose access occurred during this review.

## Evidence and uncertainty

[The harness retention report](📓️s-transaction-harness-retention.md) records the exploratory uncached run with shard 1 at **5 passes / 9.31 s**, shard 3 at **19 passes / 9.39 s**, shard 4 at **19 passes / 12.53 s**, and shard 2 stopped by the unchanged aggregate watchdog at **14.01 s**. The full target failed, and the three-identical-identity acceptance gate remains **0/3**.

The exact retained boundary registry was re-read, without executing its bundle:

`🧪️transaction-v2-bundle-bViV3C/boundaries-99124-c2c0ee6c-1565-46fe-ba49-54357a8ed2fb.txt`

It is 4,098 bytes, SHA-256 `fb810bc1192cc3a69a8e23f65fd7c802e193b2796afe0fd8a0997112b06ebc14`: 96 lines, 96 unique keys, no unexpected key. Its only missing keys are `killed:transaction-attempt-canonical-published` and `recovered:transaction-attempt-canonical-published`, both owned by the proposed transferred test.

This identifies the missing work, not its cause. Absence of the first boundary does not prove that the case never started, that the phase is defective, or that its isolated duration exceeds the budget. It may have been queued or waiting before its first recorded snapshot. No per-case timing or new contention measurement was collected. The original run occurred during other work, and its complete source/schema identity set is not pinned in the timing report; timing is not an acceptance result for the source snapshot below.

Why this candidate is the smallest useful change:

- Shard 1 completed with about 4.69 seconds of observed watchdog headroom; moving work to it leaves the near-critical 12.53-second shard 4 unchanged.
- Shard 1 currently selects two ordinary tests and three concurrent tests. The proposal selects the same two ordinary tests and four concurrent tests, still below its unchanged concurrency cap of five.
- The moved phase is `KILL_PHASES[11]`, outside `LATE_KILL_PHASES = KILL_PHASES.slice(12)`. It uses the ordinary `referenceFixture`, not the separately prepared active-reference fixture.
- Shard 1 already runs the incomplete-plan test, which creates and plans that same ordinary reference-fixture template. The transferred case can use the existing per-process template/plan cache by its unchanged key; this is a source-level opportunity, not a measured speedup.
- The transfer reduces shard 2 from nineteen selected concurrent tests to eighteen. No elapsed-time prediction is inferred from a simple test-count or worker-wave calculation: callbacks contain synchronous filesystem/transaction work, child waits, and shared-template coordination.

If this exact candidate still fails, it must remain a failed exploratory run. The next decision should use newly observed per-case/timing evidence, not a raised timeout, skipped boundary, smaller assertion set, or an unmeasured second transfer.

## Exact filter replacement

Only the first two strings change. The first adds an exact terminal phrase; the second positively enumerates the two remaining attempt-preparation phases instead of the broader attempt prefix. There is no leading anchor on the added phrase, so both the bare title and Bun's describe-qualified title match. Its trailing anchor prevents an extended sibling title from silently joining this shard.

```ts
const defaultFilterWaves = [[
  "process-tree-killed|language-neutral|incomplete plans|rolls back after-regenerations|rejects stale generator|parent-killed transaction-attempt-canonical-published$",
  "rolls back after-(?:staging|embedded-root-staging|moves|relocations|symlink-retargeting|edits)|rolls back before-verify|parent-killed transaction-(?:attempt-preparation-(?:mkdir|children)|initial)",
  "parent-killed transaction-(?:journal|wal|backup|edit)|rejects forged|rejects unreachable",
  "parent-killed transaction-(?:restore|lease)|keeps double-plan|recovers caught|committed and rolled-back|elects exactly|restores a quarantined|rejects stale (?!generator)|rejects ordinal",
]];
```

Retain the existing `filter.includes("process-tree-killed") ? 5 : 6` rule unchanged. Retain the single wave, fourteen-second aggregate watchdog at its current position after compilation and before spawning, existing fifteen-second case limits, all child deadlines/cancellation, PID cleanup, and exact sorted boundary equality check. No launch, Nx, schema, or test-body change is part of this proposal.

| Shard | Current titles | Proposed titles | Current boundary keys | Proposed boundary keys | Concurrency |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 5 | 6 | 4 | 6 | 5 |
| 2 | 19 | 18 | 31 | 29 | 6 |
| 3 | 19 | 19 | 34 | 34 | 6 |
| 4 | 19 | 19 | 29 | 29 | 6 |

## Non-executing partition verification

The TypeScript parser read the canonical test and router as source text. It extracted the four literal filter strings, fourteen static test titles, and four generated-title loops. The closed literal arrays contain eight failure stages, twenty-nine kill phases, four restore phases, and seven lease phases: forty-eight generated titles, sixty-two total. The failure-case object stages independently equal `FAILURE_STAGES`.

Every bare title and every `transaction plan journal v2 aggregate `-qualified title matches exactly one old and one proposed filter. The only changed assignment is the single title above. The title set remains 62 unique strings. Every generated or static recorded boundary was attributed to its owning test; the combined set is 98 unique keys and equals the unchanged golden's complete boundary-key set. Tests with no appended boundary remain selected and retain their independent assertions.

These are successful static source/JSON checks, not a Bun test execution, actual scheduler validation, or performance proof. The runner's current built-in static guard covers the fourteen literal titles plus a numerical forty-eight assumption; this review additionally expanded and checked the forty-eight generated names explicitly.

## Source snapshot

| Input | SHA-256 | Bytes |
| --- | --- | ---: |
| router: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` | `721315a0e2a2b84bcab91a5b40260cc9f69911814e10bcdf3f0e5789a994fa30` | 15176 |
| test: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️transaction-v2/🟦️.test.ts` | `d5a9314ab92f5300a3159e84ab26c85d9b52383f38456e5444166ed7ddf9cc99` | 82012 |
| golden: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-dispositions/🔣️.json` | `7b700d79e5474417f0c92ddce61f5ffdd24603af56241d0fbdc3cdd5ba560296` | 403163 |
| harness: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-harness-retention/🔣️.json` | `19ac4e062cc2282a87d16c11e78b0d1967ac51d4e292d90e231c7e21c3a45d41` | 976 |

All four source inputs were regular mode `0644`. No shared source was edited. If a captured input changes before implementation, re-check the literal partition against the fresh source; do not treat this report as immutable authority for a changed test universe.

At the final readback, the shared router had changed concurrently to SHA-256 `2bdb09471169fb0f53e97e469df2e99a8a60597d114503d18ceaf3ec659bde79`, 15,270 bytes. A fresh TypeScript parse confirmed that all four current selection filters and the `5 : 6` concurrency rule were unchanged. The other three captured source inputs remained byte-identical. The report's complete table independently rechecked as exactly 62 titles and 98 boundary keys. This lane did not alter or revert the concurrent router edit.

## Complete 62-title and 98-boundary mapping

Order is source registration order. A generated family's line points to its shared `test.concurrent` call. `—` means the test appends no named boundary record; it does not mean the test is optional.

| # | Source line | Exact title | Current shard | Proposed shard | Exact boundary keys |
| ---: | ---: | --- | ---: | ---: | --- |
| 1 | 549 | `keeps the language-neutral golden aligned with owned no-follow enumeration` | 1 | 1 | — |
| 2 | 605 | `rejects incomplete plans and plan-digest drift` | 1 | 1 | — |
| 3 | 631 | `rolls back after-staging with non-empty phase authority` | 2 | 2 | `rolledback:after-staging` |
| 4 | 631 | `rolls back after-embedded-root-staging with non-empty phase authority` | 2 | 2 | `rolledback:after-embedded-root-staging` |
| 5 | 631 | `rolls back after-moves with non-empty phase authority` | 2 | 2 | `rolledback:after-moves` |
| 6 | 631 | `rolls back after-relocations with non-empty phase authority` | 2 | 2 | `rolledback:after-relocations` |
| 7 | 631 | `rolls back after-symlink-retargeting with non-empty phase authority` | 2 | 2 | `rolledback:after-symlink-retargeting` |
| 8 | 631 | `rolls back after-edits with non-empty phase authority` | 2 | 2 | `rolledback:after-edits` |
| 9 | 631 | `rolls back after-regenerations with non-empty phase authority` | 1 | 1 | `rolledback:after-regenerations` |
| 10 | 631 | `rolls back before-verify with non-empty phase authority` | 2 | 2 | `rolledback:before-verify` |
| 11 | 649 | `recovers parent-killed transaction-attempt-preparation-mkdir` | 2 | 2 | `killed:transaction-attempt-preparation-mkdir`<br>`recovered:transaction-attempt-preparation-mkdir` |
| 12 | 649 | `recovers parent-killed transaction-attempt-preparation-children` | 2 | 2 | `killed:transaction-attempt-preparation-children`<br>`recovered:transaction-attempt-preparation-children` |
| 13 | 649 | `recovers parent-killed transaction-initial-lease-json-write-mkdir` | 2 | 2 | `killed:transaction-initial-lease-json-write-mkdir`<br>`recovered:transaction-initial-lease-json-write-mkdir` |
| 14 | 649 | `recovers parent-killed transaction-initial-lease-json-candidate-written` | 2 | 2 | `killed:transaction-initial-lease-json-candidate-written`<br>`recovered:transaction-initial-lease-json-candidate-written` |
| 15 | 649 | `recovers parent-killed transaction-initial-lease-json-canonical-exchanged` | 2 | 2 | `killed:transaction-initial-lease-json-canonical-exchanged`<br>`recovered:transaction-initial-lease-json-canonical-exchanged` |
| 16 | 649 | `recovers parent-killed transaction-initial-lease-prepared` | 2 | 2 | `killed:transaction-initial-lease-prepared`<br>`recovered:transaction-initial-lease-prepared` |
| 17 | 649 | `recovers parent-killed transaction-initial-wal-mkdir` | 2 | 2 | `killed:transaction-initial-wal-mkdir`<br>`recovered:transaction-initial-wal-mkdir` |
| 18 | 649 | `recovers parent-killed transaction-initial-journal-write-mkdir` | 2 | 2 | `killed:transaction-initial-journal-write-mkdir`<br>`recovered:transaction-initial-journal-write-mkdir` |
| 19 | 649 | `recovers parent-killed transaction-initial-journal-candidate-written` | 2 | 2 | `killed:transaction-initial-journal-candidate-written`<br>`recovered:transaction-initial-journal-candidate-written` |
| 20 | 649 | `recovers parent-killed transaction-initial-journal-canonical-exchanged` | 2 | 2 | `killed:transaction-initial-journal-canonical-exchanged`<br>`recovered:transaction-initial-journal-canonical-exchanged` |
| 21 | 649 | `recovers parent-killed transaction-initial-journal-canonical` | 2 | 2 | `killed:transaction-initial-journal-canonical`<br>`recovered:transaction-initial-journal-canonical` |
| 22 | 649 | `recovers parent-killed transaction-attempt-canonical-published` | 2 | 1 | `killed:transaction-attempt-canonical-published`<br>`recovered:transaction-attempt-canonical-published` |
| 23 | 649 | `recovers parent-killed transaction-journal-write-mkdir` | 3 | 3 | `killed:transaction-journal-write-mkdir`<br>`recovered:transaction-journal-write-mkdir` |
| 24 | 649 | `recovers parent-killed transaction-journal-candidate-written` | 3 | 3 | `killed:transaction-journal-candidate-written`<br>`recovered:transaction-journal-candidate-written` |
| 25 | 649 | `recovers parent-killed transaction-journal-previous-exchanged` | 3 | 3 | `killed:transaction-journal-previous-exchanged`<br>`recovered:transaction-journal-previous-exchanged` |
| 26 | 649 | `recovers parent-killed transaction-journal-canonical-exchanged` | 3 | 3 | `killed:transaction-journal-canonical-exchanged`<br>`recovered:transaction-journal-canonical-exchanged` |
| 27 | 649 | `recovers parent-killed transaction-wal-prepared` | 3 | 3 | `killed:transaction-wal-prepared`<br>`recovered:transaction-wal-prepared` |
| 28 | 649 | `recovers parent-killed transaction-backup-write-mkdir` | 3 | 3 | `killed:transaction-backup-write-mkdir`<br>`recovered:transaction-backup-write-mkdir` |
| 29 | 649 | `recovers parent-killed transaction-backup-write-mid` | 3 | 3 | `killed:transaction-backup-write-mid`<br>`recovered:transaction-backup-write-mid` |
| 30 | 649 | `recovers parent-killed transaction-backup-write-prepared` | 3 | 3 | `killed:transaction-backup-write-prepared`<br>`recovered:transaction-backup-write-prepared` |
| 31 | 649 | `recovers parent-killed transaction-backup-inner-exchange` | 3 | 3 | `killed:transaction-backup-inner-exchange`<br>`recovered:transaction-backup-inner-exchange` |
| 32 | 649 | `recovers parent-killed transaction-backup-exchange` | 3 | 3 | `killed:transaction-backup-exchange`<br>`recovered:transaction-backup-exchange` |
| 33 | 649 | `recovers parent-killed transaction-backup-retained` | 3 | 3 | `killed:transaction-backup-retained`<br>`recovered:transaction-backup-retained` |
| 34 | 649 | `recovers parent-killed transaction-edit-write-mkdir` | 3 | 3 | `killed:transaction-edit-write-mkdir`<br>`recovered:transaction-edit-write-mkdir` |
| 35 | 649 | `recovers parent-killed transaction-edit-write-mid` | 3 | 3 | `killed:transaction-edit-write-mid`<br>`recovered:transaction-edit-write-mid` |
| 36 | 649 | `recovers parent-killed transaction-edit-write-prepared` | 3 | 3 | `killed:transaction-edit-write-prepared`<br>`recovered:transaction-edit-write-prepared` |
| 37 | 649 | `recovers parent-killed transaction-edit-inner-exchange` | 3 | 3 | `killed:transaction-edit-inner-exchange`<br>`recovered:transaction-edit-inner-exchange` |
| 38 | 649 | `recovers parent-killed transaction-edit-exchange` | 3 | 3 | `killed:transaction-edit-exchange`<br>`recovered:transaction-edit-exchange` |
| 39 | 649 | `recovers parent-killed transaction-edit-canonical-exchange` | 3 | 3 | `killed:transaction-edit-canonical-exchange`<br>`recovered:transaction-edit-canonical-exchange` |
| 40 | 664 | `recovers parent-killed transaction-restore-mkdir` | 4 | 4 | `killed:transaction-restore-mkdir`<br>`recovered:transaction-restore-mkdir` |
| 41 | 664 | `recovers parent-killed transaction-restore-prepared` | 4 | 4 | `killed:transaction-restore-prepared`<br>`recovered:transaction-restore-prepared` |
| 42 | 664 | `recovers parent-killed transaction-restore-exchange` | 4 | 4 | `killed:transaction-restore-exchange`<br>`recovered:transaction-restore-exchange` |
| 43 | 664 | `recovers parent-killed transaction-restore-canonical-exchange` | 4 | 4 | `killed:transaction-restore-canonical-exchange`<br>`recovered:transaction-restore-canonical-exchange` |
| 44 | 676 | `recovers parent-killed transaction-lease-stale-quarantined` | 4 | 4 | `killed:transaction-lease-stale-quarantined`<br>`recovered:transaction-lease-stale-quarantined` |
| 45 | 676 | `recovers parent-killed transaction-lease-preparation-mkdir` | 4 | 4 | `killed:transaction-lease-preparation-mkdir`<br>`recovered:transaction-lease-preparation-mkdir` |
| 46 | 676 | `recovers parent-killed transaction-lease-json-write-mkdir` | 4 | 4 | `killed:transaction-lease-json-write-mkdir`<br>`recovered:transaction-lease-json-write-mkdir` |
| 47 | 676 | `recovers parent-killed transaction-lease-json-candidate-written` | 4 | 4 | `killed:transaction-lease-json-candidate-written`<br>`recovered:transaction-lease-json-candidate-written` |
| 48 | 676 | `recovers parent-killed transaction-lease-json-canonical-exchanged` | 4 | 4 | `killed:transaction-lease-json-canonical-exchanged`<br>`recovered:transaction-lease-json-canonical-exchanged` |
| 49 | 676 | `recovers parent-killed transaction-lease-prepared` | 4 | 4 | `killed:transaction-lease-prepared`<br>`recovered:transaction-lease-prepared` |
| 50 | 676 | `recovers parent-killed transaction-lease-canonical-published` | 4 | 4 | `killed:transaction-lease-canonical-published`<br>`recovered:transaction-lease-canonical-published` |
| 51 | 689 | `recovers parent-killed committed and rolled-back backup-only terminal cleanup` | 4 | 4 | `killed:transaction-terminal-committed-stage-removed`<br>`recovered:transaction-terminal-committed-stage-removed`<br>`killed:transaction-terminal-rolled-back-stage-removed`<br>`recovered:transaction-terminal-rolled-back-stage-removed` |
| 52 | 707 | `rolls back a process-tree-killed mixed generator and commits ordinal two` | 1 | 1 | `killed:process-tree-mixed-generator`<br>`recovered:process-tree-mixed-generator`<br>`committed:process-tree-mixed-generator` |
| 53 | 729 | `elects exactly one synchronized stale-lease contender and permits committed retry` | 4 | 4 | — |
| 54 | 763 | `restores a quarantined stale lease exactly when acquisition callback throws` | 4 | 4 | — |
| 55 | 774 | `rejects stale baseline, source digest, and counterfeit transaction segment with zero mutation` | 4 | 4 | — |
| 56 | 799 | `rejects stale resume preimage and malformed resume evidence byte-for-byte` | 4 | 4 | — |
| 57 | 821 | `rejects stale generator and embedded-reference authority before mutation` | 1 | 1 | — |
| 58 | 841 | `rejects ordinal collisions and malformed attempt siblings without mutation` | 4 | 4 | — |
| 59 | 872 | `rejects forged backup and restore preparations without mutation` | 3 | 3 | — |
| 60 | 892 | `rejects unreachable duplicate backup and edit publication tuples exactly` | 3 | 3 | — |
| 61 | 923 | `keeps double-plan bytes stable, cancellation exact, and committed second apply immutable` | 4 | 4 | `rolledback:cancellation` |
| 62 | 951 | `recovers caught allocation and journal previous-image callback failures` | 4 | 4 | `rolledback:caught-attempt-canonical-published`<br>`rolledback:caught-journal-previous-exchanged` |

## Handoff

The proposed scope is a two-string router edit only, with an independently checked one-title reassignment. No files other than this Markdown report were written during the review. Runtime lanes remain undisturbed. Implementation and any uncached aggregate run remain with the coordinator after the shared runtime boundary is ready.
