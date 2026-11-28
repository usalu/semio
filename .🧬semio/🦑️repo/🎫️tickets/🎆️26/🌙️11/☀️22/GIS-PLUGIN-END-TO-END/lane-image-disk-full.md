# Lane image disk full (ENOSPC and EFAULT in per-lane APFS images)

## Root cause
- The lane image was not too small. Lane images are sparse UDRW images, sized at birth as `min(32 GiB cap, 66 % of host free, commitment budget)`: pab149b was 27 GiB, pe3f0f6 26 GiB, p7f01f5 24 GiB, but they held 0.05 to 3 GiB (`lane.size`).
- `statfs` inside a sparse image is clamped by the host volume. The cargo wrapper's own lines show it: "the compiler image ran out of space (0.0 GiB free of 92.0 GiB)" while the image held less than 15 GiB. The ENOSPC is the host volume running dry, not the lane cap.
- Host spikes: 28 GiB free -> 14 GiB free within minutes (172 -> 188 GiB used) while 50+ machines' worth of lanes and builds wrote to sparse images. Slack held in images that cannot return blocks made it worse: `.cache/b/r/fixed.dmg` (96 GiB declared, 55 GiB allocated for about 5 to 15 GiB used) and `.cache/b/d/fixed.dmg` (36 GiB, 40 GiB allocated); macOS only unmaps containers up to 32 GiB, hence "did not give back a 256 MiB test file" and "lease-timeout".
- EFAULT on `rm .../pab149b/v/95956-739bba`: that is `enterLaneV1().release()` removing the live marker at command exit. When the host ran dry the sparse image lost its backing, so the mounted volume answered EFAULT to every call; the unhandled `rmSync` turned the command's own result into "Internal error ... exit 1". Nobody detached the image first (pab149b was retired at 08:41, after the failure); the detach path is polite and never forced on busy images. The mount was dead before release.

## Fix
- Sizing from measured need: `nativeImageShardSizedBytesV1` (pure) = min(cap, want(max(root need, largest recorded lane))). `ensureNativeImageShardV1` births a shard at `min(host/budget ceiling, sized)`, so lanes declare 16 GiB (root floor) instead of 24 to 30 GiB and the commitment budget stops being exhausted by empty lanes.
- Growth: `nativeImageShardGrowTargetV1` (pure) grows when max(used, measured) reaches `growAtPercent` (85 %) of the capacity, to peak plus headroom, bounded by cap, host share and commitment room. `growNativeImageShardV1` detaches the idle image (busy refuses, nothing changes), `hdiutil resize`, re-attaches in a `finally`, logs a new `grown` ledger event. `ensureLaneImageV1` calls it on every acquisition of an existing lane image (the ENOSPC retry re-acquires the lane, so the retry sees a bigger image); a failing grow leaves the lane on its image. Real macOS test grows a scratch shard 1 -> 2 GiB.
- EFAULT: `imageFaultV1` (EFAULT, EIO, ENXIO, ENODEV, ENOTCONN, ESTALE) and `removeInImageV1`; marker removal on release and lane-tree removal on a mounted shard end quietly on an image fault. The dead image's metadata is unreadable, so `retireIdleLaneImagesV1` retires it whole on the next pass instead of an acquisition throwing in `admitNewLaneV1`.
- Stale lanes: no change needed in the planners. `planGcV1` already removes lanes idle over 24 h, sheds oldest idle lanes under pressure and `retireIdleLaneImagesV1` returns whole images (leases and claims respected). Observed churn (pe3f0f6, pab149b retired at 08:41, p7f01f5 at 09:00, pf7055a, p0a66e6, pdb0d09) is that path working.

## Files
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts, 🧬️schema/🔣️.json (growAtPercent, resize args, `grown` event, corpus sizing and growth), 🧪️tests/🟦️.ts, 🐍️.py, 🔣️.json
- 🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts (public re-exports)
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts, 🧪️tests/🟦️.ts, 🏊️pool/🟦️.ts, 🏊️pool/🧩️image/🟦️.ts, 🏊️pool/🧩️image/🧪️tests/🟦️.ts, 🏊️pool/🧪️tests/🚦️admission/🟦️.ts

## Tests (repo test-file)
Shard 25 pass (oracle-checked sizing and growth, fold of `grown`, fake and real macOS grow), lane image 9 pass, lane 4 pass, admission 7, pool-model 6, pool-claims 8, budget shards 5. `repo typecheck`: only an existing error remains at admission test line 27 (corpus typing, not touched).

## Disk
- Before: 28 GiB free at 08:57, 14 GiB at 09:20 (188 GiB used). `repo cache gc`: lanes retired whole, images compacted (r 55 -> 25 GiB allocated, d 40 -> 19 GiB, p 7 -> 3.5 GiB).
- After: 41 to 42 GiB free (160 GiB used). Leased images (products, rust) were not compacted (lease-timeout); nothing leased or in use was deleted.

## Open
- r home image is declared 96 GiB and keeps about 20 GiB of untrimmable slack. Re-creating it at or below 32 GiB (the lanes now live in their own shards) needs a quiet window and a policy change of the rust partition; not done here.
- Declared sparse capacities of all images still add up to far more than the host (about 330 GiB declared vs 228 GiB); only per-root commitment is budgeted.
