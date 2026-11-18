# g1-m01 Disk Full (2026-11-18, 01:07Z to 01:25Z)

Read-only measurements; nothing was deleted or changed by this session.

## Symptom

- `workspace.log` of the `ready` child, attempt 66 (exit 1), verbatim: `Partition / has only 6.77 GiB free (97% used, limit 97%), so no build or cache write starts before space is freed. Free space with \`repo cache cas gc\`, then run the command again.`
- Replica `health.json` at 01:07Z: `store: stalled` (it was `ok` at 00:07Z).
- `df -h /System/Volumes/Data`: 228 GiB, 197 GiB used, 4.8 GiB free at 01:10Z, 3.8 GiB free at 01:18Z (98 to 99 %). On 15 November 17:45Z the same volume had 60 GiB free (`round-30.pre.md`).

## Where The Space Is

`du -sh`: `.cache/b` 173 GiB, `.git` 8.5 GiB, `~/Library/Caches/semio` 2.1 GiB, `/private/tmp` 2.2 GiB, `.cache/f` 784 MiB.

`.cache/b` holds one `fixed.dmg` per cache partition. Size of the image file, blocks it occupies on the host volume, and what is used inside the mounted image (`df` of `<partition>/m`, `repo cache status`):

| Partition | Declared | Image file | Occupied on the host | Used inside |
|---|---|---|---|---|
| rust (`r`) | 96 GiB | 96.0 GiB | 85.8 GiB | 28.76 GiB (30 %) |
| dependencies (`d`) | 36 GiB | 36.0 GiB | 12.0 GiB | 11.22 GiB (31 %) |
| graph (`g`) | 6 GiB | 12.0 GiB | 12.0 GiB | 6.66 GiB (111 % of the declared 6 GiB; `recreate pending`) |
| tools (`t`) | 8 GiB | 8.0 GiB | 3.1 GiB | 2.85 GiB |
| products (`p`) | 24 GiB | 24.0 GiB | 0.5 GiB | 0.32 GiB |
| nx (`n`) | 1.5 GiB | 1.5 GiB | 0.9 GiB | not mounted |

The rust image occupies 85.8 GiB of the host volume while 28.76 GiB are in use inside it: about 57 GiB are blocks that were written once, freed inside the image and never returned to the host. That difference alone is the free space this machine lost since 15 November.

The declared partitions add up to about 172 GiB on a machine whose data volume is 228 GiB with a 97 % write limit (about 221 GiB usable): once every image has been written through, the images, `.git` and the worktree do not fit.

## What The Collectors Give

- `repo cache cas gc --dry-run` (the command the refusal names): `Cache store collected: 35361 entries, 180.4 MiB freed, 0 GiB kept, 0 MiB protected. Partition / was 98.3% used (high watermark 90%, low watermark 80%). The partition stays 42665 MiB above its low watermark because the rest is pinned, leased or too young.` So the suggested command cannot lift the refusal: 180 MiB against 42.7 GiB.
- `repo cache gc --dry-run`: started 01:18Z, gave no result in 50 minutes and was interrupted by this session at 02:08Z (`Cancelled (SIGINT); leases released.`). While it ran, `repo leases status` showed it holding the EXCLUSIVE lease `repository-graph-optional-collector:/Users/ueli/Documents/semio` for 3324 s (`progressing`) besides shared `native-fixed-image` leases on `.cache/b/o` and `.cache/b/g`. Whether that lease held back the `ready` child's attempts 67 and 68: not measured. A dry run that takes an exclusive lease for an hour is a finding of its own.
- Free space meanwhile: 3.8 GiB at 01:18Z, 5.5 GiB at 01:37Z, 8.2 GiB at 02:08Z (nothing deleted by this session; whether the dry runs removed anything was not measured: the `cas gc` line above says `collected … 180.4 MiB freed`).

## For The Coordinator

1. The refusal sends the reader to a command that frees 0.2 GiB; the space is in the unreturned blocks of the rust image (and the graph image of twice its declared size), which no collector touches.
2. A 256 GB worker needs either smaller declared partitions (rust 96 GiB is 42 % of the disk) or images that give freed blocks back (hole punching or a recreate of the image when occupied minus used passes a threshold, as `recreate pending` already does for graph).
3. Until then every stage of the `ready` child on this machine ends at the 97 % refusal, and the replica reports its store as stalled.

## 02:29Z: Free Space Falls Again (2.4 GiB)

- `df`: 8.2 GiB free at 02:08Z, 2.87 GiB at 02:29:05Z, 2.44 GiB one minute later.
- The rust image grew on the host from 85.8 GiB to 89.66 GiB occupied (of 96.0 GiB) in 70 minutes; the other images did not change (d 12.0, g 12.0, t 3.13, p 0.48, n 0.9 GiB). Swap: 5164 MB of 6144 MB used.
- What writes into it: a release build of another agent session on this machine (`CLAUDE_CODE_HOST_SESSION_ID=local_c8359bff-…`, `SEMIO_BUILD_MODE=release`): `bash ./native-lanes.sh component 🔋️energy` → `cargo build --locked --manifest-path …/🌎️hub/Cargo.toml -p semio-hub-energy-component --lib --target wasm…` → `semio-build-scheduler rustc --crate-name semio_hub_energy_component` (rustc 2.0 GiB resident). It holds the shared leases on `.cache/b/r`, `d`, `t` seen at 02:08Z (process 78434). Not started by this session and not touched.
- The image can grow by another 6.3 GiB (to its 96 GiB); 2.4 GiB are free. If the build keeps writing fresh blocks, the host volume fills before the image does.
