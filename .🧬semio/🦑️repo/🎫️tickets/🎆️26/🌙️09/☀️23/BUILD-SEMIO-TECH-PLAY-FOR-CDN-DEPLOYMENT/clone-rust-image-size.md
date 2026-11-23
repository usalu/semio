# Snapshot rust image size (play clone)

Outcome: NOT grown. Snapshot `/Users/ueli/.sinc/play/.cache/b/r` is unchanged (22 GiB, mounted, 10.41 GiB used, `repo cache status` healthy). Nothing was unmounted, recreated or edited.

## Why the supported path cannot reach 40 GiB here
- Sizing lives in `.../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📐️elastic/🟦️.ts` with rules in `📦️budget/🔣️policy.json`.
- There is no in-place resize (no hdiutil resize). The only supported grow is: record growth (`elastic-growth.json` via `repositoryPartitionGrowthRecordV1`) then reset/regrow the idle image (`📥️opening/♻️reset`), after which the next opener births the image at `effectiveCapacityV1`.
- `effectiveCapacityV1` caps every new or grown rust image at `floor(hostFree * (100 - hostFreeReservePercent) / 100)`, i.e. 34 percent of host free bytes (rust `hostFreeReservePercent` 66), aligned down to 1 GiB. `elasticMinimumV1` also lowers the 32 GiB declared minimum to that share.
- Host free now ~51 GiB, plus the old image's own ~13 GiB allocation that a reset returns: 0.34 * 64 = ~21.8 GiB, aligned to 21 GiB. A reset would SHRINK the image (22 -> 21 GiB). 40 GiB needs ~118 GiB free.
- The live checkout got 40 GiB because the host had much more free space when it was born. The snapshot was born at ~69 GiB free (0.34 * 69 = 23.6 GB), matching its receipt.
- No env override exists: `SEMIO_CACHE_BUDGET_BYTES` only drives the machine budget plan, not image size; `pinHostFreeBytesForSizingV1` is a test-only pin (private-repository harness).

## Is it a bug?
It is the documented design, not an accident: shares of all partitions are asserted to add up to at most 90 percent (test in `📦️budget/🧪️tests/🟦️.ts`, "no absolute minimum exceeds what the host can create", with an independent Python model). Current shares sum to exactly 90 (rust 34, deps 20, products 12, tools 10, graph 8, nx 3, diag 2, leases 1). Raising rust requires lowering others or relaxing that law, plus the Python model fixtures. I did not change code: that is a policy decision, and the invariant protects the host from over-promising sparse images.

## Options
1. Policy change (needs a decision): give rust a larger share (for example reserve 40) and shrink dependencies/products/tools shares, update the law test and its model. A 51 GiB host would still only give 0.6*64 = ~38 GiB, so even that is below 40 GiB.
2. Free host disk to ~118 GiB, then reset the snapshot rust image (cargo home 1.5 GiB would be copied out/in around it).
3. Reduce the need instead: 8.2 GiB of `m/u` (unit cache) is counted as needed by the build; the 11.7 GiB remaining vs 19.8 GiB needed gap is ~8 GiB.

## Update: snapshot policy edit and reset attempt
- Snapshot-only `policy.json` edit: rust `hostFreeReservePercent` 66 -> 44 (share 34 -> 56), graph 92 -> 96 (8 -> 4), products 88 -> 94 (12 -> 6), dependencies `hostSharePercent` 20 -> 10, nx 3 -> 1. Sum stays 90. A reborn rust image at ~67 GiB host free would be ~37 GiB. Live repo untouched.
- `repo run repo-cache-reset:cache-reset-rust` (supported reset) declined: journal verdict `foreign-charge`. `.cache/b/r/m/storage-events.jsonl` holds two committed 4 GiB `compiler-workspace` charges (`repo:compiler:prototype` and `repo:compiler:release`) left by the stopped build chain, never released. Whole-image reset admits exactly one recomputable charge. `repo cache gc` does not release them.
- Not done: editing the journal by hand or deleting the image directory (both bypass the product's protocol). The image remains 22 GiB, mounted.
