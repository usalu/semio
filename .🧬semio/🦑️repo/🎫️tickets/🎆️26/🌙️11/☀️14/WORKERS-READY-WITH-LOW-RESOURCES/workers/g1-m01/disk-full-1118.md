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

## 03:39Z: 0.2 GiB Free

- `df`: 8.86 GiB free at 02:38Z (the other session's build had ended), 6.23 GiB at 03:09Z, 0.20 GiB at 03:38:58Z, 0.15 GiB a minute later. APFS container: `Capacity In Use By Volumes: 244.9 GB (99.9% used)`, `Capacity Not Allocated: 156.3 MB`; no local snapshots (`tmutil listlocalsnapshots /` lists none).
- Rust image 91.1 GiB occupied on the host (89.66 GiB at 02:29Z); the other images unchanged. VM volume (swap) 7.8 GB; swap 6784 MB of 7424 MB used.
- Writer: the other agent session's `native-lanes.sh` build is running again (one `rustc`, 3.5 GiB resident). Not touched by this session.
- This session removed only its own files: 68 run logs `~/native-bootstrap-round-*.log` and `~/tools-setup-round-*.log` (117 MiB; their tails are in the round reports) and its scratch files. Free afterwards: 0.24 GiB.
- Replica at 03:40Z: `health.json` 51 s old, `store: ok`.
- The account was notified (decision asked: stop that build, or recreate the rust image).

## 06:12Z: Space Is Back (79.9 GiB Free)

- `df`: 3.65 GiB free at 05:11Z, 79.88 GiB at 06:12Z. Nothing was done by this session in between.
- Rust image: 15.78 GiB occupied on the host (91.1 GiB at 03:39Z), 14 GiB used inside (28.76 GiB before); image file modified 06:12Z. So blocks were both collected inside the image and given back to the host: the thing finding 2 of "For The Coordinator" asked for now happens here. Which landing did it was not traced from this machine (`result-R184.md`, `result-R185.md` and `result-R188.md` of the coordinator name g1-m01's `disk-short` state).
- Other images on the host: d 11.48, g 12.0, t 3.13, p 0.52, n 0.71 GiB.
- Replica: `health.json` 3 s old, `store: ok`.
- `ready.json`: stage `workspace` running (attempt 69); the graph read prints `waiting for the fleet to infer @semio-tech/s-dev-composition (beyond the expected 1800 s) … 2557 of 1800`.
