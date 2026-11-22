# G1 bun test bare-filter guard

## Finding (empirical, scratchpad synthetic tree, bun 1.3.14; the pinned runtime is 1.4.2)
- A `[test] preload` does NOT run before discovery: with `bun test somefilter` matching nothing the preload never ran (log absent, exit 0); with a match it ran once per test file with `argv = [bun, <test file>]`, after the walk. A preload guard on positional filters is therefore impossible.
- Bun skips dot-directories and node_modules by default, but walks `🗑️generated`, `target*`, `dist`, `bazel-out`, `generated` etc. In the synthetic tree 8613 entries were searched.
- `[test] pathIgnorePatterns` prunes directories during the walk: with glob patterns (`**/name/**`) the count fell 8613 -> 605 and wall time 80 -> 28 ms. Bare names (no glob) pruned only partly (3609), so globs are required. Explicit file paths under an ignored dir are also ignored (no tracked test lives in such a dir outside dot-dirs: checked with git ls-files).

## Change
- `/Users/ueli/Documents/semio/bunfig.toml`: `pathIgnorePatterns = ["**/🗑️generated/**", "**/generated/**", "**/target*/**", "**/dist/**", "**/bazel-out/**", "**/.cache/**", "**/.nx/**"]` under `[test]`, zero-touch for every `bun test` invocation, cross-platform.
- New law test `🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🚏️discovery/🧪️tests/🟦️.ts` with independent Python oracle `🐍️oracle/🐍️.py`: the real bunfig lists the patterns; on a synthetic tree the searched-entries drop equals the Python count of ignored entries (210).
- Residual: a bare filter in the checkout root still walks non-ignored non-dot dirs (about the tracked tree); the open-fd hazard is bounded, the supported entry point stays `repo test-file`. No EN+DE refusal exists because nothing can intercept before the walk.

## Tests
- discovery 2/0; `repo test-file` on orphans 11/0 (existing file, runs normally); ceiling 5/2 (DE-language assertions, no language resolved in the lane env; unrelated). Never ran an unbounded scan on the checkout.
