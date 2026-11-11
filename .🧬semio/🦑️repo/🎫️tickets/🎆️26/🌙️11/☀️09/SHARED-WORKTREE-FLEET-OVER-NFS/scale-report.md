# Shared worktree fleet: scale report

Target: 50 machines in 5 groups of 10, each group with its own store and relay. Earlier rehearsal: 3 machines in 2 groups.
This round: 50 (later 51) replicas, 5 stores, 5 relays on one host, then one real-tree group of 10. Everything was run from
`🧰️scripts/` next to this file. All data lived under `$SCRATCH/rehearsal*` (APFS clones, `cp -Rc`) and is deleted; the host repo
never got a `.cache/`.

## Summary

- **Works at 50 machines.** With 62 to 64 file operations per second fleet-wide (about 1.3 per machine and second, bursts of 1 to 50 files,
  hot files, directory renames, 8 files of 14 to 48 MB), 50,700 to 65,300 operations in a run of 14 to 17 minutes, all 51 trees ended
  **identical** (path set, bytes, exec bit; 47,251 to 51,500 files), `verify` clean on all 51, no journal event without its blob, in every
  store, quiescent 11 s (best run) after the load stopped.
- **Latency before any fault** (small file, writer to reader, polled every 20 ms, 580+ samples each): in group median 343 / p95 696 / p99 932 / max 1,568 ms,
  across groups median 530 / 957 / 1,205 / 2,706 ms (run A6). Files of 14 to 48 MB: median 0.7 s in group, 1.1 s across groups, max 2.9 s.
- **Faults** (5 daemons `kill -9`, 1 relay `kill -9`, one group's store away for 30 s, 51st machine joins, compaction on a server and a worker, all
  during the load): all recovered by themselves; the price is visible in the tail (see Faults).
- **The host is the limit, not the code.** 50 daemons, 5 relays, the load generator and `fseventsd` use 7 of 12 cores at 64 ops/s. At 90 to 100 ops/s
  the backlog of the single host grows for the whole run (A3, A4: latency tens of seconds). Per daemon the cost is 14 to 16 % of one core at 64 ops/s
  (about 2.1 ms per applied file), so a real machine has about 6x headroom. Memory (ps RSS 180 to 690 MB, macOS footprint 130 MB) is covered below.
- **Defects found and fixed: 10** (two of them corrupted or left holes in a store). Replica suites 48 to 58 tests, journal 122 to 124, all five suites green.
- **Real tree, one group of 10** (270,125 files): idle 1.06 % CPU per daemon, 2 minutes at 20 ops/s: 8.4 % CPU, in group median 340 / p95 561 ms, converged.

## Method

Host: macOS arm64, 16 GB, 12 cores, APFS, Bun 1.4.2, Python 3.14. A second agent ran Linux VMs (about 5 GB) at the same time, load average 12 to 77
during runs. One host stands in for 50 machines, so all latencies below are pessimistic and the CPU figures are the honest part.

Tree (A): `🧰️framework` plus the root `.gitignore`, pruned to the shared files: **37,953 files**, 523 MiB, 5 stores x 0.63 GB after seeding; the code of the
subsystem is part of that tree and runs from it. Fleet: origins `gN-s` (group server) and `gN-w1..9`, groups g1..g5, relays `relay-gN` with the other four stores as
`--peers`. Daemons `run --poll 250 --scan 300 --debounce 150` (scan 300 instead of 900 so that scans happen during a 14-minute run), relays `--poll 250`.

Commands (all in `🧰️scripts/`, env `S`=scratch, `R`=`$S/rehearsal`, `FLEET_GROUPS`, `FLEET_PER_GROUP`):

```
python3 fleet.py full-master | master        # copy the tree, prune to shared files (fleet-shared.ts)
python3 fleet.py build                       # clone 50 trees, seed g1-s, snapshot, relay --once, 49 joins (adopt)
python3 fleet-run.py 840 64 --faults         # start fleet+sampler+prober, load 840 s at 64 ops/s, fault timeline, quiescence, convergence
CONVERGE_SAMPLE=40 python3 fleet-run.py 120 25 --no-start     # part B
python3 fleet-verify.py origins.json 4       # verify --json in every tree, files in lost/
python3 fleet-converge.py origins.json       # path set, size, exec bit, sha256 of every file against the first tree
python3 compare-trees.py A B 8000            # independent check (older script)
python3 fleet-blobs.py                       # every blob referenced by a journal event exists, per store
python3 fleet-summary.py | fleet-timeline.py | fleet-report.py | fleet-load-cpu.py | fleet-logs.py | fleet-idle.py   # tables
python3 fleet-poll-scaling.py 0 25 75 150    # idle cost against origin count (dummy-origins.py)
bun bench-relay.ts <replica .ts> <dir> 40 100 8   # relay round, 40 origins x 100 blobs of 8 KB
```

Latency: `fleet-load.py` writes one probe file per machine every 6 to 10 s (content hash in a log); `fleet-probe.py` picks one reader in the writer's group and one in
another group and polls the reader's tree every 20 ms until the bytes match (3,000+ probes per scope per run). Load per machine thread: 20 % appends to one of 4,000 real
files, edits and creates in `work/<machine>` and `work/shared/s0..11`, deletes, file renames, 1 % directory renames, 9 % rewrites of 20 hot files, 8 large files in total.
Operations are counted only when they succeeded. Sampling: `ps` every 10 s (RSS, CPU time), store sizes and segment counts, load, free memory.

Runs (all with the faults unless noted):

| Run | Code | Offered | Ops | Wall of load | Verdict |
|---|---|---|---|---|---|
| A3 | after defects 1 to 3 | 100 ops/s | 72,527 | 817 s | overload (host), converged identical (51 trees) |
| A4 | + scan jitter, caches | 90 ops/s | 70,758 | 843 s | overload, converged identical, 128 blobs missing in 2 stores (defect 2) |
| A5 | + relay blob guard | 62 ops/s | 50,753 | 840 s | healthy; **2 trees did not converge** (defects 8, 9) |
| **A6** | + defects 8, 9, counts | 64 ops/s | **52,751** | 840 s | **healthy, converged identical, 11.4 s quiescence** |
| A7 | + relay concurrency | 64 ops/s | 65,269 | 1,049 s | host degraded (join 510 s), converged identical, verify 51/51 |

(A1, A2: first rehearsals with the shipped code, aborted after defects 1 and 3.)

## Results A: 50 machines, 38k files

### Latency (ms, small files; "pre-fault" = first 158 s of the load)

| Run | Scope | n | median | p95 | p99 | max |
|---|---|---|---|---|---|---|
| A6 pre-fault | in group | 589 | 343 | 696 | 932 | 1,568 |
| A6 pre-fault | across groups | 589 | 530 | 957 | 1,205 | 2,706 |
| A5 pre-fault | in group | 570 | 368 | 822 | 1,203 | 2,099 |
| A5 pre-fault | across groups | 570 | 524 | 993 | 1,371 | 2,629 |
| A7 pre-fault | in group | 584 | 360 | 1,203 | 1,723 | 2,756 |
| A7 pre-fault | across groups | 584 | 535 | 1,322 | 2,072 | 3,264 |
| A6 whole run, with faults | in group | 3,077 | 537 | 4,534 | 26,469 | 54,751 |
| A6 whole run, with faults | across groups | 3,077 | 1,978 | 23,212 | 37,935 | 59,987 |
| A5 whole run | in group / across | 3,035 | 699 / 1,415 | 13,227 / 29,570 | 34,956 / 44,229 | 90,397 / 99,308 |
| A3 pre-fault (100 ops/s) | in group | 532 | 1,336 | 5,124 | 7,866 | 11,849 |

Files of 14 to 48 MB (8 per run, one in-group and one cross reader each): A6 in group median 678 ms, max 2,494; across groups median 1,124, max 2,884.
A5: 1,282 / 2,950 and 1,454 / 3,613. A7: 1,275 / 2,906 and 1,898 / 2,936.

Reading the whole-run rows: 10 % of probes involve a daemon that is down or catching up, and 20 % involve the group whose store was away.
The 60 s windows of A6 (median in group / across groups, p95): 0 to 120 s 324 to 369 / 511 to 550 (p95 0.7 to 1.0 s); kill -9 at 168 s: p95 0.7 s (120 to 180 s), 5.4 s (180 to 240 s), 3.1 s (240 to 300 s), max 16 s
(the five victims need up to 16 s after their restart); 360 to 480 s (store of g4 away 395 to 425 s) p95 32 s and 26 s, max 51 s: the ten g4 machines
catch up 30 s x 64 events/s; from 420 s the across-groups median rises to 4 to 11 s while in group stays 0.6 to 0.8 s (A6; relays were then sequential, see defect 9).

### CPU, memory, disk (load window 60 to 800 s)

| Measure | A5 | A6 | A7 |
|---|---|---|---|
| Daemon CPU, median / p95 / max (% of one core) | 15.8 / 16.6 / 16.7 | 14.3 / 15.6 / 17.2 | 14.1 / 16.5 / 16.8 |
| 50 daemons together | 7.7 cores | 6.9 cores | 7.0 cores |
| Relay CPU each | 5.8 % | 5.1 % | 4.8 % |
| Daemon RSS (`ps`), mean / max | 194 / 284 MiB | 185 / 270 MiB | 181 / 295 MiB |
| Sum of daemon RSS (means) | 9.5 GiB | 9.0 GiB | 8.8 GiB |
| Relay RSS | 62 MiB | 60 MiB | 66 MiB |
| Files applied, all daemons together | 2.67 M in total | 2.73 M | 3.17 M |
| Store per group, growth over the run | +0.68 GB (journal 12.7 to 27.1 MB) | +0.61 GB (12.9 to 27.5 MB) | +0.76 GB (12.9 to 30.6 MB) |
| Journal segments per store | 60 (51 origins) | 60 | 60 |

Per published event: 258 bytes of journal; per operation 11.3 KB of blobs including the 8 large files (6.7 KB without them). One applied file costs about 2.1 ms CPU
(fleet 7 cores / (64 ops/s x 50 daemons)). Idle with 50 origins: **0.89 % CPU, 98 MiB RSS** per daemon (50 daemons, `--scan 3600`, quiet host), relays 1.6 %; the same measure
for one daemon with one relay at 51 origins gave 2.5 % (noisier host), see poll scaling in C.

The ps sum of RSS passed the 8 GB guideline (peaks 11.6 GiB, host total with Python 10.4 GiB). `footprint` (measured only in part B) says a daemon holds 130 MB dirty
and 274 MB "reclaimable" (JSC pages the OS may take back); the host stayed at 4.5 to 6.5 GB free+inactive and swap flat at 0.7 GB. I did not find an RSS setting that helps
(`BUN_JSC_forceRAMSize` 128 and 256 MB, `--smol`: no change; a forced GC every 5 s: about 15 % lower, kept).

### Faults during the load (A6 times; A5/A7 in brackets)

| Fault | When | Result |
|---|---|---|
| `kill -9` of g1-w7, g2-w8, g3-w7, g4-w7, g5-w8 (recorded PIDs), restart after 8 s | 168 s | all five back, `overwritten` 29 to 32 and `kept` up to 17 each for their offline edits, converged |
| `kill -9` of relay-g3, restart after 10 s | 277 s | caught up, no blob or event lost |
| store of g4 away 30 s (directory renamed) | 395 to 425 s | every g4 daemon: one `store-unreachable`, one `store-reachable`, no error; all relays paused and resumed; no recreated store (defects 1, 2) |
| 51st machine `join` into an empty directory | 504 s | 41.9 s [A5], 165 s [A6], 510 s [A7, host degraded; round 6 quiet host 5.5 to 7.5 s, 15 s with 24 busy loops]; 6.8 s on a quiet host (61,627 paths, 220 MiB peak) |
| `compact --collect-blobs` on g1-s | 640 to 680 s | 9.6 s [A5 17.8, A7 24.8]; quiet host 0.6 s, removed 0 |
| `compact` on a worker | right after | 3.3 s [A5 5.8, A7 10.0]; quiet 0.6 s |
| `compact --collect-blobs --retention-days 0` on the quiet server | after | 11.1 s, 42,808 files (316 MB) removed |

### Errors, retries, `lost/`

| | A5 | A6 | A7 |
|---|---|---|---|
| `error` events (all daemons) | 0 | 1 | 1 |
| `unstable` (file changed while hashed, retried) | 7 | 5 | 7 |
| `blocked`, `blob-missing`, `rejoin-required`, `clock-*` | 0 | 0 | 0 |
| `overwritten` (= files in `lost/`) | 323 (37 max per tree) | 291 | 597 (34 max, 50 trees) |
| `kept` (offline edit newer than the remote one, republished) | 450 | 394 | 1,413 |

Before the fixes the same fault timeline gave 90 to 463 `error` events per run (EINVAL on rename, defect 3), 1,728 to 4,253 `blob-missing` (defect 7) and in A2 corrupt
segments of the g4 origins in the stores of other groups (defect 1). `lost/` holds the losers of hot-file races and the victims' offline edits; every one has a matching `overwritten` event.

### After the load stops

| | A5 | A6 | A7 | B |
|---|---|---|---|---|
| All 51 (10) trees hold every machine's sentinel | 27.6 s | **11.4 s** | 260 s (host degraded) | 0.5 s |
| Stores hold identical heads | same | same | same | same |
| Identical path set and sha256 on all trees (`fleet-converge.py`, first full pass) | 2 victims differ for ever (defects 8, 9) | **identical, 47,251 files** | identical, 49,828 files | identical, 270,125 files (hash of `work/` and 1 in 40 others) |
| `verify --json` | 49 of 51 clean | not run | **51 of 51 clean**, max 14.4 s, 246 MiB peak | 10 of 10 clean, max 32.7 s, 490 MiB |
| Blobs referenced by a journal but missing | 0 | 0 | 0 | |
| `compare-trees.py` 8,000 random files, 3 pairs (A3) | | | | 0 mismatches |

## Results B: one group of 10 on the real tree

Real repo minus `.git`, 67 top-level entries, **270,125 files**, 10 APFS clones (249 s), one store. Seed 66.5 s (store 5.1 GB), 9 joins adopting the clone: 125 s with 6 in parallel.

| | Idle, 5 minutes after a 90 s warm-up | 2 minutes of load (2,430 operations, 20 ops/s) |
|---|---|---|
| CPU per daemon | 1.06 % (max 1.14) | 8.4 % median (max 8.7); 0.83 cores for the group |
| RSS per daemon (`ps`) | 292 MiB (max 300), group 2.9 GiB | 606 MiB mean (max 690), group 6.3 GiB |
| footprint per daemon (idle after the load) | | 130 MB (group 1.3 GB) |
| Latency in group (109 probes) | | median 340, p95 561, p99 815, max 1,509 ms |
| Files of 13 to 41 MB (4) | | median 823, max 1,426 ms |
| Journal | | 95 MB, 75 segments |
| Errors | | 0 (87 `refused` unsafe symlinks per machine, as in the first rehearsal); 19 `overwritten` |

So one 16 GB group of ten replicas of the real tree needs 3 GB of RSS (1.3 GB footprint) idle and up to 6.3 GB RSS under load, which fits; the CPU is under 1 core.
`verify` of the real tree: 32.7 s wall with 10 in parallel (2 at a time), 490 MiB peak.

## Defects found and fixed (all in `🔁️replica/` unless noted; regression test each)

| # | Defect (evidence) | Fix | Test |
|---|---|---|---|
| 1 | **A relay recreated a vanished own store as a local directory** (store renamed away in A1/A2: `mkdir -p` rebuilt `v1/…`, wrote foreign origins into it, and the relays of the other groups then copied journals with garbage: "segment … corrupt at byte …" in other groups). Same hole in every journal write (`writeBlobV1`, `appendEventsV1`, `replaceFile`, `writeSnapshotV1`) and in `ensureHome`. | Journal: `ensureStoreDirectoryV1` creates directories one level at a time and never `v1` again once this process has seen it (`markStoreSeenV1`, set by every successful read of the origins list and by `storeReachableV1`); the first write into a store never seen still creates it (seed, fresh relay target). `relayV1` pauses while its own store is gone and reports `store-unreachable`/`store-reachable` with the own path; `relayOnceV1` takes `create`. **Edits `📒️journal/🟦️.ts`**, not only `🔁️replica`. | journal: "a store root that was seen is never rebuilt…"; replica: "a relay whose own store vanished pauses and reports it…" |
| 2 | **Journal events overtook their blobs through the relay** when a store vanished mid-round: `relayBlob` treated ENOENT as "blob missing", still appended the segment bytes and never came back (A4: 51 events without blob in g3, 77 in g5). | A read failure while either store is unreachable aborts the round (retried next poll); only a blob missing in a reachable peer is reported and passed. | "a peer store that vanishes during a relay round never lets journal events overtake their blobs" (fails 3 of 3 on the old code) |
| 3 | **Apply raced with `prune`** (`rmdir` of the parent): `rename`/`mkdir` returned EINVAL, ENOTDIR or EEXIST on macOS and the file stayed queued with an `error` (90 to 463 per run). | `materialise` retries `mkdir`+`rename` three times with 5 ms backoff for EINVAL, ENOENT, ENOTDIR, EEXIST. | `🧪️tests/🏁️race.ts` (injects EINVAL and ENOENT into `rename`; fails 2 of 2 without the retry) |
| 4 | **A directory renamed away and recreated under the old name never published the deletions of the files that left it** (the watcher reports the directory as changed, not renamed; found in the small run: 172 s until the next scan healed it). | `observe` expands every touched existing directory; entries of the state below an expanded directory that were not found are deletions. To keep that O(1) per event, `ReplicaFoldsV1` counts live paths below every directory (hashed `Int32Array`, 4 MiB; `below()`); the full pass over the state runs only when the count exceeds what the readdir found. | "a directory that was renamed and recreated … even when the watcher reports only a change"; "the folds table counts the live paths below every directory" (the first fails without the fix) |
| 5 | **Thundering herd of scans:** machines started together scan their trees in the same second every `scanSeconds`: 51 x 3 CPU s, host at 100 % for 10 s, latency spikes. | `scanDelayV1`: next scan after 75 to 125 % of `scanSeconds`. | "the safety-net scan interval is spread over 75 to 125 percent…" |
| 6 | **O(origins) work per poll that did not need to be:** `.gitignore` (14 KB) read and hashed on every follow and publish (about 40 % of the profiled time of a loaded daemon, inclusive); `readdir` + sort of the origin list every 250 ms. | `.gitignore` re-read only when `stat` (ino, size, mtime) changes, forced every 2 s; origin list cached by directory `stat`, forced every 5 s (as the head cache already was). | journal: "a cached origin list notices a new origin directory…"; the existing `.gitignore` tests |
| 7 | **`blob-missing` flood while a store is away:** a follow that lost the store mid-round reported every queued blob as missing (4,253 events, thousands of `queued`). | A blob read that fails while the store is unreachable queues silently. | "a store that vanished during a follow queues the work silently…" |
| 8 | **State lost after `kill -9`:** the own journal can be ahead of `state.json`/`state.log` (append done, persist not): a victim never learnt that it had published a file, so a file that left the disk during the outage was never deleted anywhere (A5: 2 of 5 victims stuck for ever, `verify`: `missing-local`). | `replayOwnTail` at open replays the own events after the persisted own cursor into the state with unknown mtime; the next scan rehashes or publishes the delete. | "events the own journal holds but the state never recorded … are replayed" (fails without) |
| 9 | **Relay throughput:** one origin, one blob, one fsync at a time and the peers one after the other; the across-groups median grew to 4 to 11 s in A6 after the g4 outage. | `relayOnceV1` carries origins and the blobs of a delta 8 at a time (`pool`), `relayV1` all peers at once; order blob, segment bytes, head kept per origin. Microbenchmark `bench-relay.ts` 40 origins x 100 blobs of 8 KB on a quiet host: 950 ms before, 255 ms after (x3.7). Not shown end to end (A7's host was degraded). | "a relay that carries twelve origins at once copies identical journals and blobs, also when the origins share blobs" |
| 10 | **Daemon RSS under load:** garbage kept RSS at 300 MB; `Bun.gc` every 5 s in the loop (`heapRounds`) lowers it by about 15 %. | one line in `runV1` | none (tuning) |

Suites at the end (`bun test` on the five globs): journal 124 (was 122), boundary 38, topology 39 (both changed by other agents; 32 and 22 at the start), replica 58 (was 48), bootstrap 11.
Python oracle, fixture and schema: unchanged, the new `store-unreachable` event with only `path` already validates against `ProgressEvent`.

## C: extrapolation to 50 machines x the real tree

Assumptions: each machine holds the 270k-file tree; 50 agents x 2 edits/s = **100 operations/s** fleet-wide at peak; the NFS mounts use `actimeo=1` as in the mount plans;
12 KB of blob and 258 bytes of journal per operation (measured 11.3 KB incl. large files, 6.7 KB without); every group store holds the whole fleet.

**Store polling per daemon as origins grow** (one daemon `--scan 3600` and relay-g1, idle, 60 s windows, `fleet-poll-scaling.py`; the foreign origins are dummies homed at g2..g5):

| Origins in the store | 51 | 151 | 351 | 651 |
|---|---|---|---|---|
| Daemon CPU | 2.5 % | 4.1 % | 7.1 % | 10.9 % |
| Relay CPU | 2.6 % | 5.9 % | 11.6 % | 17.4 % |

Linear: about 1.7 % + 0.0156 % per origin for the daemon, 0.03 % per origin for the relay (no O(origins²); the quiet-host figure at 51 origins was 0.89 % and 1.6 %).
Per poll: 1 `stat` of the store, 1 of the origin directory (the list itself is cached 5 s) and one per origin head, so about 52 GETATTR per 250 ms. With `actimeo=1` NFS answers
most from the client cache: about 50 per second per machine, 500 per second per group server. **50 machines x the real tree per machine: about 1.1 % (B, 10 origins) + 0.6 % for 40 more origins
+ 1.8 % for the scan (16.6 CPU s per 900 s) = 3.5 to 4 % idle; 21 to 25 % at 100 ops/s** (2.1 ms per applied file x 100 + base).
Memory per machine: footprint 130 MB, RSS 290 (idle) to 690 MB (load).

**Relay headroom.** At 64 ops/s a relay moved about 51 events/s for 5 % of a core (the rest is waiting). The microbenchmark does about 15,700 blobs/s on a local disk with concurrency 8;
even at a tenth of that over NFS it is 15x the 80 events/s that one relay must carry at 100 ops/s. Bandwidth: 100 ops/s x 12 KB x 0.8 = about 1 MB/s per relay and a one-off 50 MB x 4 for a large file.
Across groups the relay added about 0.2 s to the median (530 vs 343 ms). Expect on NFS: `actimeo=1` adds up to 1 s per store hop, so in group about 1 to 1.5 s and across groups 2 to 3 s median; the
other agent's NFS rehearsal is the place to measure it.

**Journal growth per day and store**: 258 B x events. Busy 8 h at the full rate (2.88 M events): 0.74 GB journal in 703 segments (4,096 events each, 14 per origin) and 35 GB of blobs;
round the clock (8.64 M events): 2.2 GB in 2,109 segments (42 per origin) and 104 GB of blobs.

**NFS server disk for 14 days of retention**: blobs dominate: **about 0.5 TB (8 h/day) to 1.5 TB (24 h/day)** per group server, plus 10 to 31 GB of journal, plus the current tree (5 GB seed measured) and
snapshots (the two newest per origin: 270k paths = 89 MB each, so 100 x 89 MB = 9 GB if all 50 origins compact). Plan 1 TB, or 2 TB if agents run all night, and the 14-day retention is what bounds it.

**Compaction cadence.** Workers: daily `compact` (snapshot 0.6 s at 51k paths, about 2 to 7 s at 270k, peak RSS 460 MB, runs beside the daemon). Group servers: daily `compact --collect-blobs`; measured 11 s for 42.8k blobs (316 MB)
on a quiet host, so a day of garbage (2.9 to 8.6 M blobs) is a few minutes to 30 minutes of `stat`. The collector keeps the list of every doomed file with its `Stats` in memory (about 200 B each: 0.6 to 1.7 GB for one
day, 4 GB for a weekly run): **run it daily, not weekly** (limit below).

## D: Round 6, reduced end-to-end runs (A7 and relay)

Setup: 20 replicas in 2 groups of 10, the same 37,956-file tree, quiet 12-core host (load 2), `fleet-ab.sh` (rebuild, run 300 s with kills, relay kill, a 15 s store outage, a join of a 21st machine at 60 % and compactions), 90 ops/s offered (4.5 per machine, 3.5x the A6 rate per machine) and 26 ops/s (A6 rate per machine). Scripts: `fleet-ab.sh`, `fleet-relay-catchup.py`, `FLEET_MODSRC`, `FLEET_RELAY_CONC` in `fleet.py`.

| Run | achieved ops/s | join under load | quiescence | pre-fault in-group med / p95 ms | pre-fault across med / p95 | daemon CPU med | RSS mean / max MiB |
|---|---|---|---|---|---|---|---|
| final code, 90 ops/s | 80.2 | 7.5 s | 1.0 s | 328 / 646 | 445 / 754 | 11.6 % | 263 / 313 |
| no forced GC (defect 10 off) | 81.1 | 6.2 s | 1.0 s | 289 / 471 | 453 / 747 | 11.7 % | 280 / 432 |
| relay concurrency 1 (defect 9 off) | 80.7 | 6.6 s | 1.0 s | 301 / 634 | 463 / 759 | 11.7 % | 282 / 459 |
| final code + 24 busy loops | 77.2 | **15.3 s** | 1.0 s | 349 / 628 | 466 / 857 | 12.9 % | 288 / 406 |
| final code, 26 ops/s | 27.4 | 5.7 s | 1.0 s | 335 / 481 | 507 / 694 | 7.5 % | 223 / 274 |
| relay concurrency 1, 26 ops/s | 26.2 | 5.5 s | 0.5 s | 339 / 514 | 486 / 658 | 7.1 % | 232 / 285 |

All six runs: all 21 trees identical by hash, stores equal 1 s after the load. The offered 90 ops/s were carried at 80 (the generator's own limit); the backlog stayed bounded (probe maxima 1.2 to 1.6 s before the first fault, no timeouts). Before round 6 no figure existed for this setup; A7's 64 ops/s of drain and 510 s join are not reproducible without host contention. Whole-run tails are dominated by the outage (p95 across groups 6.9 to 9.6 s, max 24 s) and equal in all variants.

Relay catch-up end to end (`fleet-relay-catchup.py 60 80 1 8 8 1`: daemons run 60 s at 80 ops/s without relays, 4,125 to 4,169 operations, all 20 origins behind in the other store, then the relays start): 1 origin at a time 3.18 s and 3.39 s, 8 at a time 0.98 s and 0.89 s.

## Remaining limits

- **Round 6 correction, A7 and defects 9 and 10.** A reduced rerun (20 replicas, 2 groups, the same 38k-file tree, 300 s, the fault timeline) on a quiet host does **not** reproduce A7: mid-load join 5.5 to 7.5 s, quiescence 1.0 s, trees identical, in every code variant (table in section D). Neither the forced GC (defect 10) nor the 8-way relay (defect 9) costs throughput or latency; with 24 busy loops on 12 cores the same final code joins in 15.3 s (2x) and the in-group p95 over the whole run triples (1.2 to 3.3 s), so A7's 510 s was host contention (the other agent's VMs, load average up to 77), not code. Both changes stay.
- **One host.** CPU saturates at about 65 ops/s with 50 daemons (7 of 12 cores plus `fseventsd` and the kernel at 66 % system time); at 90 to 100 ops/s the backlog never drains during the run. The per-daemon cost says a real
  machine has headroom, but 100 ops/s fleet-wide was not carried by this host. Run A7 shows host variance: same code and load, join 510 s instead of 42 to 165 s, in-group median 3 s over the whole run. Cause not isolated
  (suspects: shared-disk fsyncs, `fseventsd`, the other agent's VMs).
- **Relay concurrency (defect 9) is now shown end to end** (section D): a relay catching up 20 origins after a 60 s load at 80 ops/s takes 0.9 to 1.0 s with 8 origins at a time against 3.2 to 3.4 s with 1 (x3.5). In steady load at 10 origins per group the relay uses 2 to 3 % of a core and the cross-group median is the same (about 450 ms) for 1 and 8, so the speed-up matters only for backlogs.
- **Memory:** ps RSS passed the 8 GB guideline in A5 to A7 (sum 8.8 to 9.5 GiB means, 11.6 GiB peak); footprint was measured only in B. At 38k files RSS under load is 180 to 290 MB, on the real tree 600 to 690 MB, footprint 130 MB; a part of RSS is reclaimable.
- **`compact --collect-blobs` is now streaming** (round 6): at most 2,048 doomed blobs per directory wait in memory, progress per directory, cancellable; memory follows the live tree, so the weekly run is no longer excluded.
- A missed or coalesced watcher event is still healed only by the safety-net scan (900 s default, now 675 to 1,125 s); defect 4 closes the known case (directory rename and recreate), not the class.
- Same-file races are ordered by publish time (documented); `lost/` grew by 290 to 600 files per run from hot files and offline edits, none was lost.
- Not exercised: relay-built bootstrap archive refresh under load, Linux and Windows daemons, real NFS (attribute cache, `actimeo`), inotify limits, clock skew between machines, 4-byte-emoji paths near `PATH_MAX`.
- The journal module was edited for defect 1 (`ensureStoreDirectoryV1`, `markStoreSeenV1`, origin-list cache); the topology folder was not touched.
