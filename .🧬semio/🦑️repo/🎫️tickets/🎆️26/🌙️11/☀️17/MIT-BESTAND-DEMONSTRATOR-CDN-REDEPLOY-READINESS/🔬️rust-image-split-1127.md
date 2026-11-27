# Rust cache image split (every image at most 32 GiB)

## Finding
- macOS unmaps freed blocks only for APFS containers of at most 2^35 bytes, so `.cache/b/r/fixed.dmg` (96 GiB, about 59 GiB allocated, about 42 GiB used) can never give blocks back.
- `.cache/b/d/fixed.dmg` is 36 GiB (38653698048), also above the cap. It is not "near it": it cannot return blocks either. Its policy is fixed too (see below). Images `g` (12 GiB), `p` (24 GiB), `t`, `o`, `n`, `l` are fine.
- The repo already had the machinery (`🍎️image/🧩️shard`: ledger, lifecycle, per-key images mounted at their logical path, per-image reclaim used by `cache gc`), but it was switched off (`imageShards.enabled: false`) and nothing physically moved files into shards.

## Changes
1. Schema bound (budget schema `$defs.imageCap` = 32 GiB, and the shard schema):
   - `elasticPartition.maximumBytes|minimumBytes|floorBytes|needBytes`, `imageShards.maximumImageBytes`, shard root `needBytes`, fixed `needBytes.<platform>` all at most 34359738368.
   - `partition` has an `anyOf`: an unsplit partition has `capacityBytes` at most 32 GiB; a partition whose `imageShards.enabled` is true may have a larger `capacityBytes`, which is then the budget of all its images.
2. Policy (`budget/🔣️policy.json`):
   - rust: `imageShards.enabled: true` with three disjoint roots: `u/l/<lane>` (one image per Cargo lane), `u/{sealed,w}` (unit cache), `{cargo,components}` (registry/sources and stages/runs). The home image keeps `b`, `published`, `nx`, `configuration`, `timing`, `u` misc. `elastic.rust` max 32 GiB, min 24 GiB, need 24 GiB.
   - dependencies: 36 GiB reduced to 32 GiB (its declared need on every platform is at most 29.5 GiB).
   - elastic products maximum 40 GiB reduced to 32 GiB.
3. Root rule (`🧩️shard/🟦️.ts`): a root below another is allowed while the outer pattern does not match the segment that leads to it (before, `""` and `u` could not coexist with `u/l`).
4. Partition model (`budget/🧩️images/🟦️.ts`, new):
   - `repositoryPartitionImagesV1(workspace, partition)` lists home plus live images with their logical path; `repositoryPartitionImageOfV1` resolves a logical path to its image. Callers never hardcode a mount point.
   - `repositoryRelayoutPlanV1` (pure) and `relayoutRepositoryPartitionV1`: per key, move the home directory aside (`.relayout-<key>`), birth and mount the key image at the same logical path, move files one at a time (copy, unlink), stop cleanly when the host volume would fall below the reserve (`pressureFreeBytes`, 6 GiB), resumable.
5. Locks / scheduling (`📥️opening/♻️reset`): `relayoutRepositoryPartitionIfQuiescentV1` takes the exclusive `native-fixed-image:<dir>` lease with `timeoutMs: 0` (builds hold it shared, so it answers `busy` and touches nothing while any cargo/rustc/unit-cache build runs), serializes births with the shard ledger lease, and never waits.
   - New target `repo-cache-reset:cache-relayout-rust` (`📜️script.ts relayout <partition>`).
   - `cache gc` for a split partition now runs that relayout first (best effort, skipped when busy) and then compacts every image: home plus every ledger image get their own nonwaiting shared lease and unmap kick (`kickRepositoryShardsOnlineV1`), so each compacts independently.

## Tests
- `budget/🧩️images/🧪️tests/🔣️.json` + `🟦️.ts`: language-agnostic cases for the 32 GiB bound (schema vs independent Python model), the relayout plan (implementation vs independent Python model), and a physical relayout with a fake lifecycle (all files keep logical paths, 6 images born, repeat is a no-op, space stop and resume, unmounted home is left alone).
- Updated: `budget/🧪️tests/🧩️shards`, shard `🧪️tests` (nesting roots), budget `🧪️tests` fixtures (`expectedPartitionBytes`, elastic rust capacity rows, image cap growth rows).
- `repo typecheck` on all touched TypeScript: 0 errors.
- Environmental: the existing real-`hdiutil` shard test fails here with `hdiutil: create failed - No child processes` (host sandbox), unrelated to this change.

## Live re-layout: NOT performed
At the time of writing `repo leases status` shows shared `native-fixed-image:/Users/ueli/Documents/semio/.cache/b/r` held by a running build, and 32 cargo/rustc/bun processes use the image. The relayout would answer `busy` and does nothing, and nothing was unmounted, moved or deleted.

### One command, once the build queue (native-queue.sh, lane q53457c) has finished and nothing holds the Rust image
```
repo run repo-cache-reset:cache-relayout-rust
```
(the same step also runs at the start of `repo cache gc` when the image is idle). Then `repo cache gc` compacts each image.

### Expected cost and risk
- The move copies about 42 GiB of live data between images file by file, so it needs host free space of at least the reserve (6 GiB) plus the largest file; with 17 GiB free it stops cleanly with `status: "space"` and resumes on the next run. Free host space first (`repo cache gc`, which returns blocks of the other images) if it stops.
- The old 96 GiB backing keeps its bloat until the home image is recreated. After the move the home holds only about 6 GiB; run `repo run repo-cache-reset:cache-reset-rust` (or `cache-regrow-rust`) once idle to recreate the home image at at most 32 GiB (its first birth is automatically capped by `imageCapBytesV1`).
- The `d` image (36 GiB) needs the same recreation (`repo run repo-cache-reset:cache-reclaim-dependencies` does not shrink it; use a reset/regrow while idle) because its policy now declares 32 GiB, applied only to the next born image.

## Follow-on edits forced by the new numbers
- Ledger machine policy (`🧾️ledger/🎚️machine/🔣️policy.json`): `defaultTotalBytes` 212937474048 to 208642506752 and the basis-point weights recomputed (checked by the independent Python largest-remainder model in `🧾️ledger/🧪️tests/📐️model`).
- `README.md` (`d` is 32 GiB in the EN table and DE sentence) and `📓️build-guide.md` (default 194.3 GiB).
- Budget fixtures: `expectedPartitionBytes` 182067396608, headroom 24082644992.

## Test status (via `repo test-file`, `repo typecheck`)
Pass: budget `🧪️tests/🟦️.ts` (25), `🧩️images` (5), `🧪️tests/🧩️shards` (4), shard `🧪️tests` except the real-hdiutil test, ledger model, law and machine tests.
A wider sweep of 43 policy-reading test files also shows failures that all create real disk images or use the Nx/Bazel graph (`hdiutil: create failed - No child processes`, "Exact Original Graph format owner required", `fixed16GiB` expectation that depends on host free space); they do not read the changed numbers and I could not compare them with a pre-change baseline because git state may not be touched.
