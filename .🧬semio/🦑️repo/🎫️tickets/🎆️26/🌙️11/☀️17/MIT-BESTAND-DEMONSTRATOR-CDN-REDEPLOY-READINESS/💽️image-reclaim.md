# Image reclaim: host space tracks actual use

## Root cause

1. Freed blocks never reached the host because every image was attached through the legacy stack: `nativeImageAttachArgumentsV1` built `hdiutil attach` (`native-build/💾️storage/🍎️image/👤️owners/🟦️.ts:5`, schema `ImageProtocol.attachArguments`). That stack drops the APFS trim of the mounted volume. Measured on this Mac (4 GiB scratch image, macOS 27.2): the same raw file keeps 1.5 GB after write and delete under `hdiutil attach` (also after detach), but falls to 2 MB after one `diskutil image attach` cycle (diskimagesd forwards the trim as hole punching). Raw UDRW, the file name and the file size stay identical, so every receipt and identity check stays valid. No format migration is needed.
2. Images were created far above their working set: `elastic.rust.needBytes` in `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json:33` equalled the cap (96 GiB), so `elasticCapacityV1` always wanted 120 GiB and the host-free half or the cap decided; pressure growth (`elasticGrowthV1`, 1.5x) only ever grows.
3. The only bloat answer was `repositoryPartitionRegrowDueV1` (old `budget/🟦️.ts:38`): destroy and recreate the image, losing the cache.

## Design

- Attach: `diskutil image attach --nobrowse --mountOptions owners --mountPoint <m> <image>` (`attachProgram`, `attachArguments` in the image schema; `nativeImageAttachProgramV1`; `nativeImageHdiutilCommandV1` routes the `image` verb to diskutil, detach and info stay on hdiutil). New tool registry entry `diskutil`. Every future attach now trims on mount.
- Sizing: `elastic.rust.needBytes` 96 GiB to 32 GiB (headroom 25 percent gives 40 GiB, minimum 32 GiB, cap 96 GiB; pressure growth still raises it).
- Reclaim: new module `🍎️image/♻️reclaim` (public entry under `📦️packages`). `nativeImageReclaimDueV1` is the pure bloat rule (`elasticBloatedV1` delegates to it). `reclaimNativeImageV1` runs one cycle on an idle image: detach when attached (busy refuses, nothing changes), attach through diskutil at a private scratch mount, poll the host allocation until stable (bounded), always detach; progress in 4 steps, cancellation between steps and polls. `createNativeImageDarwinReclaimPortV1` is the macOS port.
- Orchestration: `reclaimImageIfQuiescentV1` (`♻️reset/🟦️.ts`) takes a non-waiting exclusive `native-fixed-image:<dir>` lease, skips pending retirements and images that are not bloated. `repo cache gc` (`runCacheLedgerV1` in `🏗️build/🧹️cache-gc/⌨️command/🟦️.ts`, not the older `CachePruneScript`) calls it after the ledger collection, without `--area`, for rust, dependencies, graph, products, tools, diagnostics, nx (not leases), with `--dry-run` printing what would be reclaimed. Bloat no longer triggers recreation. New policy key `reclaim` is the common rule for partitions without an elastic rule (`repositoryPartitionReclaimRuleV1`, `repositoryPartitionReclaimDueV1`).
- Per OS (schema `ImageProtocol.reclaim.mechanisms`): darwin unmap-on-attach; linux sparse loop file with `discard`/`fstrim` (punch hole); windows dynamic VHDX with `Optimize-VHD -Mode Full`. Only darwin runs images here; other platforms return null.

## When it runs

- New attaches use diskutil from the next image open or creation; live images `r`, `d`, `g` were not touched.
- A live image shrinks on the host at the next `repo cache gc` that finds it idle (exclusive lease free, nothing mounted busy inside, for example no shell inside `.cache/b/r/m`), because the cycle leaves it detached; the next opener attaches it as after a restart. Not yet exercised on a live image.
- A new rust image is born at 40 GiB; existing images keep their receipt size until a reset.

## Tests

- `🍎️image/♻️reclaim/🧪️tests`: language-agnostic corpus `🔣️.json` checked by the TypeScript rule and the Python oracle `🐍️.py` (also the host allocation via `os.stat`); fake-port cycle, cancellation, bound; real macOS image written by hdiutil and reclaimed (size and inode unchanged). 8 pass.
- Updated attach expectations (owners, silence, sparse, fold) and the sparse allocation assertion (allocation now shrinks at mount); owners 3, sparse 7 pass; silence 6 of 7 pass (one needs the runner artifact directory).
- Budget suite: new policy and sizing test, rust fixture rows recomputed with the Python model; 15 pass, 3 fail only for the missing script invocation envelope outside `repo test`.
- Open: run the suites through `repo test`, and one end-to-end gc on a scratch workspace.

## 2026-11-17 21:20 incident

The first gc did not reclaim because (a) the reclaim was wired into the older prune script, not the ledger gc that `repo cache gc` runs, and (b) a `repo-cache-reset:cache-regrow-rust` run held a shared lease on `r`. With `r` unheld and no open files on its mount, `r` was detached, attached through `diskutil image attach` at its canonical mount and left mounted: 99.8 GB to 19.5 GB allocated (80 GB freed, host 3.7 GiB to 80 GiB free). `d` got the same cycle and holds 12 GB of real data. `g` is held by another agent's gc and was not touched.

## 2026-11-18 regression: volume reports 4.5 GiB free on a 96 GiB image with 82 GB unallocated

Measured: after the manual cycle `r` shows size 96G, used 20G, avail 2.3 to 4.5 GiB, while `diskutil apfs list` says 81.8 GB not allocated and the host has 64 GiB free. A fresh image attached through the same stack reports its full size, so the volume itself is fine. The figure matches the host free space (3.7 GiB) at the moment `r` was attached during the incident: the volume's available space is sampled when it is mounted and is not refreshed after the trim returned 80 GB to the host. The incident cycle attached `r` with the host disk full and left it mounted; the code path in `reclaimNativeImageV1` always detaches after the trim, so the next attach samples the real host free space.

Fix: re-attach `r` once it is idle (`remount-image.sh r` in this folder: refuses while any file is open on the mount, detaches, attaches through diskutil). Not done by me because cargo and rustc of another session hold files on `r` right now.

## 2026-11-18 Linux and Windows ports, stale-statfs admission

- Schema `ImageProtocol.reclaim`: per-OS `imageName` (fixed.dmg, fixed.img, fixed.vhdx), command data `linux` (losetup, `mount -o discard,noatime`, findmnt, `fstrim --verbose`, umount) and `windows` (Get-VHD, `Mount-VHD -ReadOnly -NoDriveLetter`, `Optimize-VHD -Mode Full`, Dismount-VHD), and `availability` (tolerance, correction window, `APFSContainerFree`).
- `♻️reclaim/🟦️.ts`: the port gained an optional `trim` step (called after attach, before settling); `createNativeImageLinuxReclaimPortV1`, `createNativeImageWindowsReclaimPortV1`, factory `createNativeImageReclaimPortV1(platform, ...)`. Same cycle, progress and cancellation; always detaches (a failed mount releases the loop device). `reclaimImageIfQuiescentV1` now picks the port by platform and skips when the image file is absent (only fixed.dmg exists today).
- New `🍎️image/⚖️availability`: `nativeImageEffectiveAvailableBytesV1` (volume figure, unless the APFS container free bytes exceed it by more than max(256 MiB, 1/16): then container figure capped by host free, never lowered), plist reader and `diskutil info -plist` sync probe. `component-pass` consults it only when admission would refuse (and again after the collector ran), caches the correction 30 s and logs it; no remount needed.
- Tests: corpus `🧪️tests/🔣️.json` gained 4 OS cycles (command order per platform, quote escaping) and 10 availability cases; Python oracle decides availability too. 16 pass via `repo tools exec bun`. Component-pass suite needs the `repo test` envelope (not run); linux/windows commands are mocked, never executed.
