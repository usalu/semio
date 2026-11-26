# F86: other `withCommandSourceControlV1` users, label audit

Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. Grep for `withCommandSourceControlV1` and `build:command-source-imports` (the two strings from F85's section 2).

## Who uses what

| File | Calls `withCommandSourceControlV1` | Arms `build:command-source-imports` |
|---|---|---|
| `🏗️build/🧪️tests/🕸️graph/🔭️observed-facts/🟦️.ts` | yes (line 79) | yes (line 81), the only remaining stale arm |
| `🏗️build/🧪️tests/🕸️graph/🛂️admission/🟦️.ts` | yes | already fixed by F85 (arms `build:closure-graph` on the fresh cache) |
| `🏗️build/🧪️tests/🕸️graph/🌐️source-discovery/🟦️.ts` | no, only imports it (line 12) | no |
| `🏗️build/🕸️graph/📥️inference/🧪️tests/⏳️yield-budget/🟦️.ts` | yes (line 14) | no, it arms `build:selected-yield-budget` |

`🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts` is the helper that defines `withCommandSourceControlV1`, not a test.

## Counts (`repo test-file`, one file at a time)

| File | Before | After | Notes |
|---|---|---|---|
| observed-facts | 6 pass / 1 fail | 6 pass / 1 fail (unchanged) | see below |
| source-discovery | 4 pass / 1 fail | not rerun (no change made) | failure is not a label problem |
| yield-budget | 1 pass / 0 fail | not rerun (no change made) | passes; no label armed |

Scratch outputs: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F86/` (`before-observed-facts.txt`, `after-observed-facts.txt`, `source-discovery.txt`, `yield-budget.txt`, `typecheck-observed-facts.txt`).

### observed-facts: "same receiver observed replay awaits actual command closure cancellation in production frames"

Failure, verbatim: `error: expect(received).rejects.toThrow(expected)` / `Expected promise that rejects` / `Received promise that resolved: Promise { <resolved> }`, at line 81.

This is the "label never yields" shape. The armed call is a replay on the same cache, which F85 showed no longer walks.

Tried: the armed call on a fresh cache from `port.createScriptInputCache` with `control.arm("build:closure-graph")` (the F85 pattern), and the assertion at line 82 moved to `build:closure-graph`. It still resolved, so the label swap alone does not fix it. The fresh cache apparently does not yield `build:closure-graph` inside this observation scope (`captureObservedSourceInputsV1` / `memoizedObservedAsyncComputationV1`). I did not dig further under the seal rule.

Both edits were reverted. The file now holds the original text: `build:command-source-imports` appears twice (lines 81 and 82), and `repo typecheck` on it exits 0. No landing lock was taken, because the net change is zero.

Open: the cause needs a look at why the fresh-cache walk avoids the yield in this scope. Left as-is.

### source-discovery: "relative import resources retain physical bytes and source-only closure with bundler suffixes"

Failure, verbatim: `error: expect(received).toBe(expected)` / `Expected: true` / `Received: false`, at the line-55 assertion on `captured.observations` for a `kind==="file"` record whose `fact.sha256` equals the leaf's bytes hash.

This is not the armed label: the file never arms `build:command-source-imports`. It is an observation-fact assertion. Not guessed at, not changed.

### yield-budget

Passes (1/0). It arms `build:selected-yield-budget`, which is unaffected by F85's commit.

## Summary

- No file in scope needed the label fix that F85 applied to the admission test, other than observed-facts, and that fix alone did not make observed-facts pass.
- Product code: untouched.
- Test code: observed-facts edited and reverted (net no change). Typecheck exit 0.
