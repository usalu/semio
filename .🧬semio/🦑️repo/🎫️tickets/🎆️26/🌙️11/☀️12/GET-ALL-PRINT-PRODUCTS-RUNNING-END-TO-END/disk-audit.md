# Disk audit: root partition at 97%

Read-only audit, 2026-11-13 ~21:37 PST. Nothing was deleted, moved, unmounted or modified. Only `repo cache status` and `repo doctor` were run from the repo tooling.

## Free space

| Measure | Value |
|---|---|
| `df -h /` | 228Gi size, 13Gi used, 6.6Gi avail, 67% (this is the sealed system volume, not the problem) |
| `df -h /System/Volumes/Data` | 228Gi size, 193Gi used, 6.6Gi avail, **97%** |
| APFS container disk3 free | 7.0 GB (7,049,809,920 B) |
| Local APFS / Time Machine snapshots | none (`tmutil listlocalsnapshots /` empty). The only snapshot is the sealed OS snapshot, not reclaimable. |
| Purgeable space | not reported by `diskutil info` |

Note: `diskutil info` reports the container as 245.1 GB total. Each `.cache/b/*/fixed.dmg` is a sparse read/write image. `ls -l` shows apparent size (e.g. 103 GB for `r`), but the real on-disk cost is what `du` shows (13 GB).

## Ranked table

Sizes are on-disk (`du`). "Safe" means regenerable and not in use.

| # | Path | Size | What it is | Age | Safety | Reason |
|---|---|---|---|---|---|---|
| 1 | `$TMPDIR/semio-repo/501/plans-89ee733bc283` | 1.3G (12 JSON, up to 260 MB each) | Plan cache from a host session that has exited | newest file 19:05 today | safe (confirm no holder first) | Owning host log last written 19:14, no live pid or socket |
| 2 | `~/.cache/uv` | 3.8G | uv package cache | n/a | safe | Pure cache, re-downloads on demand (`uv cache clean` equivalent) |
| 3 | `~/.bun/install` | 1.6G | Bun package install cache | n/a | safe | Regenerable from registry |
| 4 | `~/Library/Caches/com.google.antigravity` | 369M | app cache | n/a | safe | Cache |
| 5 | `~/Library/Caches/Firefox` | 321M | browser cache | n/a | safe | Cache (close Firefox first) |
| 6 | `~/Library/Caches/semio/staffage-dwg-target` | 409M | build target cache | n/a | safe | Build output, regenerable |
| 7 | `~/.cache/codex-runtimes/codex-runtime-install-TzM1kf` | 322M | leftover install temp dir | n/a | safe (if Codex not installing) | Temp directory from an install |
| 8 | `~/Downloads/googlechrome.dmg` | 276M | Chrome installer (mounted at /Volumes/Google Chrome) | Nov 12 | safe (ask first) | Installer; unmount, then delete |
| 9 | `~/Library/Caches/go-build` + `~/Library/Caches/bun` | 234M | build caches | n/a | safe | Cache |
| | **Safe tier subtotal (rows 1-9)** | **~8.6G** | | | | Would take the data volume from 7 GB to about 15 GB free |
| 10 | `~/Library/Caches/com.anthropic.claudefordesktop.ShipIt` | 626M | Claude desktop update staging | Nov 13 16:08 | safe only when no update is in progress | Update staging |
| 11 | `~/Library/Caches/semio/teaching-python` (+ `-runtimes`) | 1.1G + 55M | runtime caches | n/a | regenerable-but-expensive | Rebuild costs time |
| 12 | `~/.cache/codex-runtimes/codex-primary-runtime` | 1.6G | Codex runtime | n/a | regenerable-but-expensive | Needs reinstall of Codex runtime |
| 13 | `~/.rustup/toolchains` | 869M | Rust toolchains | n/a | regenerable-but-expensive | Reinstall needs network and time |
| 14 | `/private/tmp/semio-indexes` | 330M | index files | n/a | check first | Purpose not confirmed |
| 15 | `~/Library/Caches/puccinialin` | 513M | unknown app cache | n/a | check first | Owner not identified |
| 16 | `~/Library/Caches/semio/ticket-backup` | 63M | ticket backup | n/a | do-not-touch | Backup of tickets |
| 17 | `.cache/b/r/fixed.dmg` | 13G on disk (103G apparent, sparse), mounted at `.cache/b/r/m` | APFS image of build cache | written 21:33 today | do-not-touch | Mounted and actively written (a `tsc` run is active). Deleting breaks builds. |
| 18 | `.cache/b/g/fixed.dmg` | 12G, mounted at `.cache/b/g/m` | APFS image of build cache | written 21:33 today | do-not-touch | Mounted and written today. `repo doctor` says graph is 7.6 GiB over its quota, so use `repo cache gc` instead. |
| 19 | `.cache/b/d/fixed.dmg` | 2.8G (38G apparent), mounted | APFS image | written 21:36 today | do-not-touch | Mounted and written today |
| 20 | `.cache/b/t/fixed.dmg` | 2.7G (8.6G apparent), mounted | APFS image | written 21:33 today | do-not-touch | Mounted and written today |
| 21 | `.cache/b/n/fixed.dmg` (+ nested `n/m/*/native/fixed.dmg`) | 0.9G | APFS image | written 21:00 today | do-not-touch | Nested images also mounted |
| 22 | `.cache/b/o`, `.cache/b/l` | 65M + 63M | APFS images | Nov 10-13 | do-not-touch | Mounted |
| 23 | `$TMPDIR/semio-repo/501/plans-916ec84dcebe` | 1.1G (9 JSON, up to 286 MB) | Plan cache from the live host | written 21:31 today | do-not-touch | Live host (pid 21810, holds leases, pid 21811 running) |
| 24 | `~/Library/Application Support/Claude` (`claude-code` 617M, caches 364M) | 1.1G | app state | n/a | do-not-touch (caches only with app closed) | App state |
| 25 | `~/Library/Application Support/Cursor` | 809M | editor state | n/a | do-not-touch | Settings and history |
| 26 | `~/Library/Application Support/com.apple.wallpaper` | 589M | system wallpaper store | n/a | do-not-touch | System data |
| 27 | `~/.cargo` | 78M | Cargo | n/a | do-not-touch | Small, in use |

## Other findings

- `/Users/Shared/` is only 124K. `/Volumes/semio-store` is NFS (192.168.178.70), so it does not consume local disk. It has 37 GiB free and could take cache offload, but it is not a local reclaim.
- `~/Library/Developer` does not exist (no Xcode DerivedData or CoreSimulator). No Docker or OrbStack is installed.
- `~/.Trash` and `~/Downloads` are not readable (permission denied to this process), so their sizes are unknown. Check them manually in Finder.
- `/private/var/folders/.../T` is 4.2G, of which the plans dirs are 2.4G.
- `/private/tmp` is 695M in total (semio-indexes 330M, semio-repo 281M, claude-501 65M).

## Recommended reclaim path (not executed)

1. Confirm no holder for `plans-89ee733bc283`, then delete it (1.3G).
2. Run `repo cache gc --dry-run` to see the build-cache collector's plan, then run without `--dry-run`. Its own budget says 7.8 GiB is to be evicted (graph 7.6 GiB over quota, nx 162 MiB over). This is the safest way to reclaim from the `.cache/b` images without touching the mounts.
3. Clear the uv cache (3.8G) and the bun install cache (1.6G).
4. Delete `~/Downloads/googlechrome.dmg` after unmounting it (276M).
5. `repo doctor` also reports 88 dead-process lease rows and 5 orphaned lease folders. `repo doctor --fix --only=leases` cleans these. This is not a disk reclaim, but it helps status reads.

Reclaiming the `.cache/b` images themselves (`hdiutil compact` after detach) may recover free blocks inside each sparse image. Do this only when no build is running, and not as part of this audit.

## Totals

- Safe tier, reclaimable now: **about 8.6 GiB** (rows 1-9)
- Plus row 10 (ShipIt, 0.6G) when no update is pending: about 9.2 GiB
- Regenerable-but-expensive: about 15 GiB (rows 11-13, not counted above)
