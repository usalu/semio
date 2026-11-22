# Rust volume zero-avail recovery

## Root cause
The APFS volume at `.cache/b/r/m` was latched: the kernel's spaceman sampled host free space 0 when the host disk was full (image had grown to 42 GiB allocated vs 14 GiB used) and never refreshed it. Kernel log: `spaceman_alloc: disk15 can't allocate metadata 1 host free 0`, then ENOSPC on every transaction and `apfs_vfsop_sync: failed to finish all transactions`. `df` showed avail 0/ifree 0 although the container had 88.9 GB free and the host 27 GiB. Because the dirty transactions could never flush, non-forced `hdiutil detach`/`diskutil unmount` answered "Resource busy" with no user-space holder (no lease on the rust image, no cargo). The repo already knew the stale figure (`nativeImageEffectiveAvailableBytesV1`) but only as advice; the reclaim path had no way out of the latch.

## Recovery performed
No holder existed (leases_status, process scan). `diskutil unmount force` then `hdiutil detach disk14` (APFS checkpoint intact, nothing deleted), then `diskutil image attach --mountPoint .cache/b/r/m fixed.dmg` (the repo's UNMAP-capable attach). The attach trimmed the image: 42 GiB -> 13 GiB on the host, host free 27 -> 59 GiB. Verified `touch` OK, `df` avail 29 GB.

## Why `repo cache gc` hung
Its graph phase waited for exclusive `repository-graph-facts:shard:56` held by pid 96518 (graph inference, reading g image, progressing, not stalled on ENOSPC: it has no handle on the r image), and the rust reclaim ran only after the ledger phase. Cancelled my gc; 96518 left alone.

## Changes
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json`: `availability.latchedHostFreeBytes`, `forceDetachArguments`.
- `.../🍎️image/⚖️availability/🟦️.ts`: `nativeImageVolumeLatchedV1` (pure), `nativeImageForceDetachArgumentsV1`; re-exported in the packages entry.
- `.../🍎️image/♻️reclaim/🟦️.ts`: Darwin detach retries with `hdiutil detach -force` only after a busy refusal on a latched volume.
- `.../♻️reclaim/🧪️tests/🟦️.ts`: new test (pure latch table + force only when latched); 20/20 pass.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts`: whole-gc reclaims the rust image before the ledger/graph phase, so it no longer waits behind graph inference.

## Not done
The `reclaim-wait`/`reclaim` targets report `not-due` for a detached image; the latch is only auto-detected while mounted. Re-running the full gc end to end was not repeated.
