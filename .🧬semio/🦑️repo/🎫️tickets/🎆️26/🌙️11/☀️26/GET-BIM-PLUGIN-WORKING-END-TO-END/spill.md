# Spill: why the `w` shard filled and how it is bounded

## Findings

- **Writer.** `publishUnitV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts`) writes a unit to `<u>/w/spill/<sha256>` (plus `entries/<key>.json`) when the host CAS refuses it and a fleet client exists (`script.ts` line ~351). The spill is only meant to live until the fleet upload has read it.
- **Never evicted.** Only the uploader's `finish()` drops a spill entry, after a successful upload or after the job expires (`maximumJobAgeMilliseconds` = 24 h). No gc stage knew the folder, and no bound existed. At the failure: 2265 objects, 441 crates (all bim-model test binaries, 400-790 MB each), 29 GiB, oldest 17 h, none orphaned, 2255 files in an image of 30 GiB.
- **Why it hurts.** `w` is `layout.temporary`, a shard (`s/w/fixed.dmg`, 32212254720 bytes) shared by staging and scratch files, so a full spill leaves 97 MiB: linkers fail with ENOSPC. The host had 47 GB free.
- **Why the shard cannot grow.** `native-build/💾️storage/🍎️image/🪜️growth` (schema, measured on macOS 27): an attached image cannot be resized (`hdiutil` status 35, `diskutil` busy), growth only runs detached, and the darwin cap is 32 GiB (`maximumImageBytes` 34359738368; `w` is already 30 GiB). Growth is also only driven for lane images. So growing is not available; bounding is the fix. The machine budget (60 % of the volume) already caps the total.
- **Why binaries are 600+ MB (not debuginfo).** `prototype-native` is already `debug: 1`, `opt-level 0` (profile schema `🎚️mode/🦀️profile/🧬️schema`). A sample binary: `__TEXT` 202 MB, `__LINKEDIT` 402 MB (4.0 M symbols), only 2.8 M of them stabs. `strip -S` removes just 9 % (638 to 582 MB), and on this nightly `debuginfo=line-tables-only` links to the same size as `1` (rustc probe, 8000 functions: 69.3 MB vs 69.3 MB; `0`: 59.0 MB). `split-debuginfo` is already `unpacked` on macOS. A profile change would buy at most 10 % and cost line numbers in backtraces, so none was made. The real multiplier is that each bim-model test file links the whole model into its own binary (hundreds of `tests_N` / `mut_any_N` binaries); consolidating them into fewer test targets is the lever, left open.

## Fix

- New schema policy `spill` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json`: `maximumBytes` 4 GiB, `sharePercent` 15, `keepFreePercent` 10, `protectMilliseconds` 15 min, `temporaryMilliseconds` 1 h.
- New module `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🧽️spill-bound/🟦️.ts`: pure `spillBoundBytesV1` (least of maximum, share of the volume, what keeps the volume free) and `planSpillEvictionV1` (unreferenced objects, empty entries, then least recently used entries, objects only when no kept entry names them; anything used within the protect window stays); `pruneSpillV1` applies it with statfs of the folder and drops stale `.tmp` files.
- `💾️store/🟦️.ts`: `pruneSpillObjectsV1`; `publishUnitV1` with `spill` prunes first and refuses (returns `undefined`, the unit is just not shared) when the unit alone exceeds the bound; `readSpillObjectV1` touches the object so a running upload keeps it.
- `🧹️gc/🟦️.ts`: `collectGarbageSingleFlightV1` and `reclaimUnitStorageV1` prune the spill first.

## Tests

- New `💾️store/🧽️spill-bound/🧪️tests/🟦️.ts` + independent oracle `🐍️.py`: bound, hand-checked plan, 500 seeded random states equal between code and Python, folder pruning (LRU, protected entries, read touch, absent folder). 4 pass.
- Existing `💾️store/🧪️tests/💧️spill/🟦️.ts` and `🧹️gc/🧪️tests/🟦️.ts` pass.

## Space freed

Ran the fixed code once on `.cache/b/r/m/u` (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️26/GET-BIM-PLUGIN-WORKING-END-TO-END/spill-prune.ts`): removed 1276 entries / 2162 objects, 27.5 GB; kept 4.0 GiB (entries used within 15 min or inside the bound). `w` went from 100 % to 14 % used (26 GiB free).

## Changed files

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🧽️spill-bound/🟦️.ts (new)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🧽️spill-bound/🧪️tests/🟦️.ts (new)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🧽️spill-bound/🧪️tests/🐍️.py (new)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️26/GET-BIM-PLUGIN-WORKING-END-TO-END/spill-prune.ts (ticket input script)

## Open

- Bim-model integration tests: one binary per file; consolidate.
- Spilling a 600 MB binary to the same volume as scratch is questionable; with the bound it is capped at 4 GiB.
