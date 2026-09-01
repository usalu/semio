# Goal Fixdrift Report

## Defect

`🧹️normalization/🟦️.ts:7451` `requireExactKeys(value, ["schemaVersion", "virtualPathPolicyCases", "symlinkFlavorCases"], "transaction disposition fixture")`
enforced exactly 3 keys against
`🧫️fixtures/🧪️transaction-dispositions/🔣️.json`, which actually carried 13 keys (`attemptLayout`,
`affectedStateCases`, `boundaries`, `expectedDispositionOperations`, `failureStages`, `journalStates`,
`negativeDispositionCases`, `schemaVersion`, `symlinkFlavorCases`, `transactionLedgers`,
`virtualPathPolicyCases`, `virtualPreimageNodes`, `workspaceLedgers`) — blocking every plan that
reaches the repo library, framework-wide included.

## Consumer census (git grep on the fixture path + every key name)

Four independent readers of the one blob, with disjoint-to-partially-overlapping key needs:

1. **`🧹️normalization/🟦️.ts` `serializedSentinelCases`** (production, evidence-removal disposition) —
   reads exactly `schemaVersion`, `virtualPathPolicyCases`, `symlinkFlavorCases`.
2. **`📦️index.test.ts` describe("taxonomy transaction dispositions v2")** — reads those 3 plus
   `expectedDispositionOperations`, `affectedStateCases`, `negativeDispositionCases`, `failureStages`,
   `journalStates`, `virtualPreimageNodes` (asserts the engine's own behaviour end-to-end).
3. **`🧪️tests/🧪️transaction-v2/🟦️.test.ts`** — reads `boundaries`, `transactionLedgers`,
   `workspaceLedgers` (used pervasively via a module-level `transactionGolden`) plus `failureStages`,
   `journalStates`, `virtualPreimageNodes` (one large recovery-proof test).
4. **`📦️packages/🟦️typescript/📜️script.ts` `transaction-v2` command** — reads only `boundaries`, for
   shard/coverage auditing, plus whole-file byte hashing for run-provenance tracking.
5. **`attemptLayout`** — zero consumers anywhere in the repo (`git grep` confirmed). Dead data, added
   the same day as the other new keys but never wired to anything.

Git archaeology confirmed the drift mechanism: the fixture already had 10 keys (consumers 1+2) when
the validator was authored — so the validator was stale from day one, reflecting only consumer 1's
own reads. `a8d1caf41f` then added `boundaries`/`transactionLedgers`/`workspaceLedgers` for consumer 3
without anyone touching the validator, because nothing connected the two.

## Decision: (b) — split, not widen

Multiple independent test suites and one production code path each read their own slice of one file.
Widening `requireExactKeys` to the full 13-key list (option a) would keep coupling consumer 1's
production authority check to three other suites' unrelated bookkeeping — the next new consumer would
reproduce the exact same class of bug. Split into four kind-only-leaf fixtures, one per semantic
domain, each exactly matching what actually reads it:

- `📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-sentinel-cases/🔣️.json` —
  `schemaVersion`, `virtualPathPolicyCases`, `symlinkFlavorCases` (production + its test).
- `📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-disposition-outcomes/🔣️.json` —
  `schemaVersion`, `expectedDispositionOperations`, `affectedStateCases`, `negativeDispositionCases`
  (test-only bookkeeping, no production reader).
- `📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-protocol/🔣️.json` —
  `schemaVersion`, `failureStages`, `journalStates`, `virtualPreimageNodes` (shared commit-protocol
  vocabulary genuinely read by both consumer 2 and consumer 3 from different angles).
- `📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-ledger-boundaries/🔣️.json` —
  `schemaVersion`, `boundaries`, `transactionLedgers`, `workspaceLedgers` (consumer 3 + consumer 4).

`attemptLayout` was dropped entirely — no compatibility carry-forward, per the greenfield rule.
The old combined `🧫️fixtures/🧪️transaction-dispositions/` directory was deleted.

## Changes

- `🧹️normalization/🟦️.ts`: `TRANSACTION_DISPOSITIONS_FIXTURE_PATH` → `TRANSACTION_SENTINEL_CASES_FIXTURE_PATH`,
  now pointing at the sentinel-cases fixture; `requireExactKeys` list is unchanged (already exactly
  matches the new file) and error/message text renamed to "transaction sentinel cases fixture" for
  accuracy (no test asserted on the old wording).
- `📦️index.test.ts`: the one test now reads three fixtures (`sentinelCases`, `dispositionOutcomes`,
  `protocol`) instead of one combined blob; added `Object.keys(...).sort()` exact-key assertions and
  `ownedFilePaths` single-leaf checks for all three directories it touches.
- `🧪️tests/🧪️transaction-v2/🟦️.test.ts`: split `TRANSACTION_GOLDEN` into
  `TRANSACTION_LEDGER_BOUNDARIES_GOLDEN` (module-level `transactionGolden`, now non-optional/exact
  typed) and `TRANSACTION_PROTOCOL_GOLDEN` (local `protocolGolden`); added exact-key assertions and
  `ownedFilePaths` checks for both directories.
- `📦️packages/🟦️typescript/📜️script.ts`: `identityPaths.golden` → `identityPaths.ledgerBoundaries`,
  pointing at the ledger-boundaries fixture.
- New fixtures created (see above); old `🧪️transaction-dispositions/` directory removed.

## Test-driven verification (both directions)

New file `🧪️tests/🧪️transaction-fixture-key-exactness/🟦️.ts` (registered as
`test-transaction-fixture-key-exactness` in `📋️project.json`, `bun ./📜️script.ts test
transaction-fixture-key-exactness`, and in both `.vscode/launch.json` and
`.vscode/🧩️launch.seed.jsonc` under `4_gate`). It asserts each of the four split fixtures exists and
carries **exactly** its expected key set, that the old combined directory is gone, that
`attemptLayout` was not smuggled into any split file, and that sentinel-case rows keep the exact shape
the engine requires.

Verified in both directions by temporarily moving the four new fixture files aside with plain `mv`
(no git) and back:

- **Before** (fixtures absent, simulating pre-fix state): `1 pass / 6 fail` — ENOENT / missing-file
  assertions, exactly the shape of failure this fixes.
- **After** (fixtures restored): `7 pass / 0 fail`, run both directly (`bun test ./🧪️...`) and through
  the registered nx target (`bun ./📜️script.ts test transaction-fixture-key-exactness`).

Also re-ran the two real existing suites that read the split data:

- `bun test ./🧪️index.test.ts -t "taxonomy transaction dispositions v2"` → **6 pass, 0 fail** (91
  expect() calls).
- `bun ./📜️script.ts test transaction-v2` (repo library's own crash/recovery suite, ~1100
  assertions) → **0 fail, 1107 expect() calls**, both times run. Both runs hit the suite's own
  internal 14-second aggregate wall-clock budget under the concurrent sibling load this ticket
  warned about (three other workers actively running framework-module tests) — a timing budget trip,
  not an assertion failure; every actual check passed both times.

## VERIFY — framework-wide plan (first time this scope has ever completed)

```
B=bb06c41f73f0122fbed315b7487428b976f99921
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework" --baseline "$B" --plan "$T/🗑️temp/🔣️fixdrift.json" --workers 4
```

Result — **plans successfully instead of throwing the fixture-shape error**:

```
[clean taxonomy plan] moves=1918 roots=0 relocations=0 symlinks=0 removals=1 edits=4327
regenerations=13 unresolved=4721 digest=b676c2bdecc3c074259a6525a18649e6b508847b16a9ffdd2dd4b1b140f1fbf8
```

The CLI then exits non-zero with `blocked by 4721 unresolved decision(s)` — that is the normal,
separate "plan produced but not fully resolvable yet" exit, not the original defect; the scope no
longer fails to plan at all. 4721 unresolved is consistent with `🖱️ui` (809/563) and `🎭️actor`
(55/97) still being untouched large scopes per `📓️goal-session-status.md` §13, plus whatever the two
other concurrent sibling workers' in-flight module changes contributed — I did not diff against a
clean baseline to attribute the exact split, so the unresolved count may shift for reasons outside
this slice. The 12MB plan file was deleted from `🗑️temp/` after recording these numbers (tool
output, not evidence).

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️transaction-v2/🟦️.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️transaction-fixture-key-exactness/🟦️.ts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-sentinel-cases/🔣️.json` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-disposition-outcomes/🔣️.json` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-protocol/🔣️.json` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-ledger-boundaries/🔣️.json` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-dispositions/` (removed)
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (new gate entry)
