# Disk: host free space and cache bounds

Host `df -h /Users/ueli`: 25 GiB free before, 42 GiB free after (228 GiB volume, 80 % used). Free space fluctuated between 11 and 46 GiB during the run because other sessions build and the collector rebuilt a shard image.

## What was freed and how

- `repo cache gc --reclaim=20GiB` (no timeout, background, log `🗑️generated/gc-disk-1.log`) ran about 11 minutes and completed.
  - Evicted idle build units and compacted shard `p803402` into a fresh image `q90be4b` (21 GiB on host before, 10 GiB after). The host images shrank: `.cache/b/r/fixed.dmg` 34 GiB to 13 GiB, `.cache/b/d/fixed.dmg` 31 GiB to 12 GiB, `.cache/b/r/s` 68 GiB (peak while the new shard was seeded) to 47 GiB.
  - Image `rust` (`.cache/b/r/fixed.dmg`) could not be compacted in place: it is 34.36 GB, macOS only unmaps containers up to 32 GiB. Its `reclaim-failed` message stays until the image is split or recreated at at most 32 GiB.
  - Images `graph` 2.47 to 2.23 GiB, `rust` 23.41 to 23.21 GiB on the host.
- `repo cache cas gc` (log `🗑️generated/gc-disk-cas.log`): content-addressed store 36.2 GiB to 25.6 GiB (the holder, the rust unit cache, evicted its own entries during the collection). Everything left is held by the unit cache.
- Manual: `~/Library/Caches/Homebrew` and `go-build` (about 180 MiB). No `🗑️generated` folder of a closed ticket existed (only this open ticket has one); `~/t` does not exist. Idle shard `pb777d4` (3.6 GiB) is not retired in `shards.jsonl` (only a `born` row), so the tool keeps it.
- Nothing mounted or in use was touched; the only processes killed: none.

## Root cause

- The machine budget total (`defaultTotalBytes` 194 GiB) was independent of the volume: 85 % of a 228 GiB disk. The planner only reacted when host free space fell below `hostFreeFloor` (min(15 GiB, 6 % of the volume) = 13.7 GiB). With 25 GiB free `cache gc` therefore found "nothing to collect" while the sparse images kept their high-water size.
- The declared CAS ceiling (256 GiB) is larger than the whole disk. The cap in force was already derived from the partition (1 GiB here, `casEffectiveCapBytesV1`), but `cas status` printed the declared ceiling as "of 256 GiB cap".
- The `rust` image is 34.36 GB and cannot return blocks to the host.

## Fix (schema-first)

- `hostSharePercent` (60) added to the machine policy and schema: the effective machine total is `min(declared or configured total, 60 % of the host volume)` (`machineHostCapV1`), combined with the existing free-space pressure rule; `overBytes` is measured against that ceiling.
- `cache cas status` prints the cap in force instead of the declared ceiling.
- The free-space floor was left at min(15 GiB, 6 %) because the CAS capacity vectors and the 72 GiB role reserve are derived from it.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🔣️policy.json`
- `.../🎚️machine/🧬️schema/🔣️.json`
- `.../🎚️machine/🟦️.ts`
- `.../🎚️machine/🧪️tests/🟦️.ts` (new test "the effective total is bounded by a share of the host volume")
- `.../🎚️machine/🧪️tests/🐍️.py` (independent oracle applies the same ceiling to the 400 seeded machines)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/⌨️command/🟦️.ts`

Tests (`repo test-file`): machine budget 12 pass, CAS 72 pass, cache-gc budget 7 pass, fleet-status 32 pass; logs `🗑️generated/disk-test-*.log`.

## Open

- Recreate the `rust` image at at most 32 GiB so it can return blocks.
- Enforce the share ceiling against the image footprint (allocated bytes), not only content, so a high-water image triggers recreation.
