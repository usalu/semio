# closure-missing on build-lock-closures/1b4538cd... (dsl-record-emission-rs)

## Findings
- Raised: `🔨️modules/📚️library/🏗️build/🕸️graph/🩹️closure-repair/🟨️.mjs` `closureMissingErrorV1` ("leaf is not on this machine"), thrown through `readClosureLeafV1` from `🕸️graph/📁️workspace/🟦️.ts:87` (`portableInputs` expansion). Leaves live in `.cache/b/g/m/build-{input,lock}-closures/<sha256>.json`.
- Leaf `1b4538cd...` is absent (238 lock leaves present). It is named only by the owner fragment `generations/o/306547a5...json.br` of generation 221, which was `restored from the remote cache` (another machine's generation; `.cache` is not replicated, leaves only travel through the fleet exchange). No inference or derived-node record names it (dry-run purge: 0).
- Root cause: a restored generation fragment names a leaf this machine never produced, and the service that failed (04:32Z) ran the code before the typed repair existed (its log line `named by <project>` and untyped failure do not match current source).
- The design fix is already in the tree and already live in the running service (log now shows `closure-missing <key> named by N projects: re-derived | asked of the fleet`): `obtainClosureLeafV1` (fleet fetch, sha256 verified) then `settleClosureLeafV1` (purge records, re-derive) from `settleRestoredClosuresV1` (restore) and the expansion catch in `🕰️generations/🔌️load/🟨️.mjs:~388-396` (up to 3 rounds, `engine.invalidate` forces a cold plan that re-infers the owner and republishes the leaf; typed `closure-missing` after the rounds, never ENOENT or a silent fallback). `missingClosureOfV1` also recognises the `ENOENT: closure-missing:<key>` text that crossed a boundary.
- Tests: `🩹️closure-repair/🧪️tests/🟦️.ts` (6 tests) and `🕰️generations/🔌️load/🧪️tests/🟦️.ts` lines ~420-490 (rounds, typed failure, key extraction). Not re-run by me (the service must stay untouched).

## Local store repair
None needed or performed. Nothing deleted.
