# Graph maintenance tests: final (2026-11-22)

Follows `graph-maintenance-remaining-2026-11-22.md`. Per-file runs (`NATIVE_EXEC_TARGET=repo-graph-maintenance:test`, one `bun test` per file through `native-exec.ts`).

## 1. Root cause of the host-dependent sizing (fixed)
Every partition image that is not born yet is sized from the live free bytes of the host volume (`effectiveCapacityV1` and `pinnedCapacityV1` in `budget/📐️elastic`). `repositoryCacheBudgetV1` and `repositoryStrictStorageV1` each read the host separately for the not yet born rust image, so on a nearly full, busy host the two reads can differ and the production check `Rust cache envelope disagrees with repository total budget` fires (the first-case failure seen earlier). The same non-determinism sized the graph image of the ledger test (8 percent of host free, aligned down to GiB: below 1 GiB headroom on a small host), which the former skip gate estimated wrongly (it still failed on an 18 GiB host with `computations` 2 instead of 1).

Fix (production check untouched): a sizing port `pinHostFreeBytesForSizingV1` in `budget/📐️elastic/🟦️.ts` (re-exported by `budget/🟦️.ts`); `undefined` keeps the live volume. The private test repository (`🧪️testing/🧫️private-repository`) pins `privateRepositoryHostFreeBytesV1` = 40 GiB when it opens (graph image 3 GiB funds the production optional headroom, rust 13 GiB; images are sparse). Tests no longer read `statfsSync` for expectations: the budget test uses the exported constant (three places).

- Ledger test `distinct graph datasets`: skip gate and `budgetPolicy` import removed; it always runs and passes (ledger file 7 pass, 0 skip).
- Budget test file `budget/🧪️tests`: 17 pass, 2 fail (`the nx partition provisions its own backing...` and an unnamed one, both `Actual Native platform image required` in a platform-image registry check, unrelated to sizing; this file is not part of `repo-graph-maintenance:test`, not investigated further).

## 2. Remaining closure-maintenance flakiness: external macOS xattr writes into the fixture volume (not fixed)
With sizing deterministic the closure file still fails 1 to 5 of 26 cases per run, a different set each run (earlier: generation, leaf-replaced, reference-replaced, optional-new-reference; later: inference, publication-fallback, corrupt, unknown, optional-release, referenced, ...); each passes alone. Evidence from a temporary `[DEBUG]` probe in the closure `recheck` (reverted): in the unexpected `Closure reference directory changed` case the empty directory `<fixture image>/m/tmp` changed only its ctime (mtime, size, nlink, inode equal) 2 to 6 s after the proof was captured, and the directory then carries the extended attributes `com.apple.macl` and `com.apple.provenance`. `com.apple.macl` is attached by macOS when another application with file access (the desktop app or an indexer watching the checkout, in which `.cache/test-sandbox` lives) first touches the directory. The same mechanism explains the 4096-byte `Authenticated graph cache cannot fund required physical headroom` (available 32784384 vs minimum 32788480): the xattr consumes one 4 KiB block of the 32 MiB fixture volume between the test's `before` free-space read and the maintenance check, so the `floor` is missed by one block; the negative cases (`unknown`, `corrupt`, ...) then fail their declared-primary-rejection assertion because the refusal differs. The three expected `directory changed` events seen in the probe belong to the deliberately mutating negative cases (leaf-replaced, reference-replaced, new-reference) and are correct.

Not changed: dropping ctime from directory stamps would weaken the production proof (a forged mtime would pass), and the private repository parent cannot be moved out of the checkout (`SEMIO_PRIVATE_REPOSITORY_PARENT=/private/tmp/...` is refused with `Exact Original workspace required`, tried). Expected green on a host where nothing touches the checkout during the run (CI, no desktop file watcher); on this machine it stays flaky.

## Final per-file counts (this host)
- storage tests: 24 pass, 0 fail
- ledger tests: 7 pass, 0 fail, 0 skip
- headroom tests: 26 pass, 0 fail
- closure maintenance tests: 21 to 25 pass, 1 to 5 fail per run (xattr noise above)
- (outside the project script) budget tests: 17 pass, 2 fail (unrelated)

## Changed files (relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`)
- `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📐️elastic/🟦️.ts` (sizing port)
- `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts` (re-export)
- `🧪️testing/🧫️private-repository/🟦️.ts` (pin and exported constant)
- `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts` (no host statfs)
- `🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts` (skip gate removed)
