# Playwright browsers evicted by disk-pressure reclaim

## Root cause
`planCachePressureV1` (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🟦️.ts) selects the oldest unit of every declared `area` that is neither held nor quiet, ignoring `unusedAgeMs` and budget. The `playwright` area (`tools/ms-playwright`) qualified, so the disk-recovery writer `compile-caches` (`repo cache gc --reclaim=<bytes>`) deleted the browser installs. Only `repo tools setup` / deps-browsers provisions them, nothing recreates them at launch, so Play tests failed with "Executable doesn't exist".

## Fix (schema-first)
- Ledger schema `collect.pressure`: enum `reclaimable` (default) | `pinned`.
- `CacheCollectV1.pressure`, `CacheTargetV1.pinned`; the pressure candidate filter skips pinned targets.
- The gc runner maps `pressure: pinned` to `pinned` and excludes pinned bytes from the machine budget's evictable estimate.
- Policy: `playwright` is `pinned`. `elan` (elan fetches toolchains on demand) and `tectonic` (bundle re-downloaded on demand) stay reclaimable; the `tools/*` retirements and the image-managed `tools` partition are not pressure candidates.
- Ordinary age/budget collection (`planCacheTargetsV1`) is unchanged: a pinned area is still collected by its 90-day unused age and 2 GiB budget.

## Tests
New test in the ledger tests ("pressure collection never selects a pinned tool area..."): an independent Python model (`python3 -I`) plans the same reclaim over fixture targets with a pinned area and must agree for reclaim 0, 1, 200, 1e6; a law over the authored ledger asserts `tools/ms-playwright` is pinned and no pinned area is ever pressure-selected at 1e12 bytes. Existing Python/SQLite oracle tests are untouched.

## Changed files
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts

## Test outcome
Not verified. `repo run repo:test-storage-ledger-oracle-processes` and `repo:test-build-cache-gc` (via MCP) both failed before running the tests with "Undeclared compiler-captured physical Source read refused ... owner-checkpoint/🟦️.ts" (concurrent edits elsewhere, graph contention). A CLI retry never got past graph planning in 590 s. bun is not on PATH, so the test file was not run directly. Re-run `repo run repo:test-storage-ledger-oracle-processes` once the graph settles; the ledger test file is run by that target (confirm; the target name was inferred).
