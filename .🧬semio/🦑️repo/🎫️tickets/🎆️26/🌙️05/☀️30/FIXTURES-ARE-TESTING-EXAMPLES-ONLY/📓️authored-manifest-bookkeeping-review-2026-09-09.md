# Authored Manifest Bookkeeping Review — 2026-09-09

## Completion Document Review

The final independent document review found that recorded counts and passing/failing runtime claims match their linked evidence. All links resolve except the authored manifest and lifecycle completion records, which are created by the final closure steps. The legacy fixture-directory name census is separate from the still-running full case-placement scan; the completion table labels that boundary explicitly. The final scan paragraph must be replaced with its confirmed outcome before closure.

## Candidate hygiene

The reviewed `🗑️generated/coordinator/authored-manifest-candidate.json` has 31,884 entries. It contains no `AGENTS.md`, no path below `🗑️generated`, and no entry whose filesystem status is `directory`.

`retained(ticket)` intentionally contributes 74 ticket records, including reports and retained coordination scripts. Those are ticket-closure records rather than production-source attribution and should remain distinguishable from the source-path manifest.

## Findings

1. `undefined/geometry-policy.rs` and `undefined/overlay.json` are intentional removed-path ledger entries from `📓️stray-test-overlay-classification-2026-09-09.md`. They identify redundant stray test-overlay output removed by this task. The collector’s explicit `undefined/` allowance is therefore required for this ledger and these two `removed` records must remain distinguishable from live source paths.

2. The pre-regeneration candidate omitted 37 of the 41 records in standalone `📋️plugin-followup-authored-paths-2026-09-09.json`; only four appeared through unrelated report entries or ticket-file recursion. The collector parses only top-level JSON arrays and does not traverse that object’s `updated`, `moves.from`/`moves.to`, or `retainedRecords` fields.

   This is resolved in the report input: `📓️plugin-followup-fixture-fixes-2026-09-09.md` now appends a top-level `## Authored Paths` JSON string array. Its 41 paths exactly equal the standalone flattening of `updated`, both sides of every move, and `retainedRecords`. Every entry passes the collector’s accepted-path rule; none is generated output or an `AGENTS.md` path. Generic recursive parsing of unrelated report objects remains unnecessary.

## Manifest comparison

The current `📓️final-framework-fixture-audit-2026-09-09.md` declares nine exact authored paths; all nine are already in the candidate. Its collector input adds six unique paths because the remaining three were collected by earlier reports.

`📓️framework-computed-reader-fixes-2026-09-09.md` now declares the eight reader repairs plus the OS Dev Vitest configuration in a recognised top-level `## Authored Paths` array. Its nine unique paths all exist, including the corrected `📇️directory` coordinate. The array contains neither generated output nor an `AGENTS.md` path.

The large plugin manifest contributes 29,848 candidate paths and the framework-separation report contributes 1,139. This review found no directory attribution in either contribution. The next candidate regeneration will ingest both verified flat arrays.

## Final Collector Rejection Review

The regenerated candidate contains 32,008 paths: 17,344 present files, 14,664 removed-path ledger entries, and no directory entries. It retains no generated path and no `AGENTS.md`.

The final allowlist correctly admits the legitimate path families previously omitted by prefix-only collection:

- 18 `♻️mit-bestand/` Netz move and reader records, covering both removed and canonical destinations.
- Two root `🧪️tests/🦀️rust-warnings` records and the root `🧫️fixtures/🦀️rust-warnings/🔣️.json` canonical fixture.
- Root `🔒️dependencies.json`.

The 11 remaining rejected values are not authored file paths: one empty string; four trailing-slash Hub directory expressions; two quoted Hub source-code replacement fragments; and four literal `join(...)` source-code expressions. No legitimate manifest file is rejected by this final set.
