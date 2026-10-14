# Inference Freshness Read-Only Audit

Source inspection only; no tests or runtime processes launched, no source edits, no whole-goal acceptance claim. Existing runtime captures were left untouched.

## Findings

- No confirmed cross-project memo omission found in inspected current callers. In `🏗️build/🕸️graph/📥️inference/🟨️.mjs`, shared script closure/file/command dependency caches (490, 497, 587) and native dependency manifest/directory caches (746, 747, 783) use `memoizedObservedComputationV1`.
- `🏗️build/👁️observation/🟨️.mjs:85–91` captures nested observations and copies them into both receiving inference scope and source capture on a memo hit. It validates each recorded fact before returning the memo value. Async scopes use `AsyncLocalStorage.run`; project compute is awaited (62–74, 430), while the synchronous memo refuses Promise results.
- Raw Rust shortcut returns remain at inference lines 397 and 436. Initial suspicion of cross-project omission is **not confirmed**: the current project caller at 1621 supplies `new Map()` per project, so Rust snapshots/closures remain in that project's original scope. Preserve this containment or move those raw caches behind observed memo replay before sharing them.
- Parser images use source-content-addressed keys and revalidate nested observations (observation 138–150). `observedSourceAnalysisV1` and its async counterpart read current bytes/stamps before lookup (158–171). Parser semantic equality intentionally allows body-only byte changes during receipt validation (173–185); these are semantic observations, not exact byte invariants. Raw file observations retain byte hashes.
- Parent-inference validation memoization (52–59) deliberately shares checks within the original parent. Full outer persistence still calls `current(rows)` at 440 and dependency receipt publication at 475. No new full-graph bypass was found in inspected paths. This source audit does not prove runtime freshness under concurrent mutation.
- No cancellation/deadline checks occur inside synchronous source image lookup/replay or memo validation. Cancellation is checked between inference units through `yieldInferenceV1`, including before async declared-source/project assembly (1619–1620). Bounded parser result retention (16 MiB) does not bound individual parser execution or the number of synchronous nested observation checks. Treat latency as a runtime verification gap, not a demonstrated deadline failure.

## Test Evidence and Gaps

Observation tests at `🏗️build/👁️observation/🧪️tests/🟦️.ts:20` check invocation memo replay into separate captures, content mutation and recomputation; line 37 begins async project admission coverage. Semantic parser replay/error and cache-bound tests exist at 218 and 224. Their presence was inspected; none were run by this audit.

The simple invocation memo case exercises one raw file fact. A stronger source-level coverage target is a different outer cache key sharing an inner native/script cache, asserting every receiving project retains file, directory membership, resolution, and path/type observations after a nested hit. Add cancellation/deadline runtime evidence around large validation/parsing batches if required for acceptance.

The exact command text `test-build-discovery` was not found in the inspected root, product, or library permanent scripts; command registration and the full discovery suite need the execution owner's current source/runtime evidence.
