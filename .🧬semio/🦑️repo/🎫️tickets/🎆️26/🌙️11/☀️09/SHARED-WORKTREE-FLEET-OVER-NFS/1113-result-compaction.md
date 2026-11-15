# Result: snapshots, segment retirement and blob reclaim (2026-11-13)

Module `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Landed 09:38 (lock `1113-landing.lock`, entry in `1113-landings.md`). The repo MCP did not connect, so the ticket is not closed. Nothing was run against the live store except read-only measurements: `🧰️scripts/1113-compact/measure.ts` (journals and blobs, 34 s) and a `status` call. Automatic operation is OFF; `compaction --apply` was never run on the live store (the benchmark below ran on a copy of its journals).

## 1. Measurement of the live store (09:00, read-only)

| Origin | Events | Superseded | Journal bytes | Superseded bytes | Segments |
|---|---|---|---|---|---|
| g1-server | 545,485 | 544,916 (99.9 %) | 199.5 MB | 199.4 MB | 134 |
| g1-m01 | 278,033 | 251,709 (90.5 %) | 100.9 MB | 90.2 MB | 68 |
| g1-m04 | 274,163 | 272,399 (99.4 %) | 99.3 MB | 98.8 MB | 67 |
| g1-m02 | 273,463 | 162,084 (59.3 %) | 99.1 MB | 61.6 MB | 67 |
| g1-m03 | 273,079 | 138,481 (50.7 %) | 99.0 MB | 48.3 MB | 67 |
| g1-m06 | 272,434 | 272,409 (100 %) | 98.8 MB | 98.8 MB | 67 |
| g1-m05 | 6,547 | 5,403 | 2.3 MB | 1.9 MB | 2 |
| **Total** | **1,923,204** | **1,647,401 (85.7 %)** | **698.9 MB** | **599.0 MB (85.7 %)** | 472 |

- Live paths: 275,803 distinct, of which 274,003 live and 1,800 tombstones (1,165 links). A full fold takes 7.4 s in the measuring script.
- "Superseded" = an event whose hlc is not the winning hlc of its path. The 1.92 M figure is higher than the 1.6 M of the brief because the republishes continued after the brief was written.
- Snapshots in the store: six files of about 91 MB (546 MB). The newest is the daily one of 03:30 (cursors: g1-server segment 66, the workers segment 0 or 1), i.e. it is from before the storm, so about 1.6 M events lie behind it.
- Blobs: 239,096 files, 13.75 GB. Referenced by a winning entry: 234,304 hashes, 11.20 GB. **Unreferenced: 4,792 blobs, 2.55 GB**, of which only 3,025 blobs (382 MB) are older than one day; the rest was written during the storm and is younger than the safety age. The full scan took 26 s (no sampling was needed).
- Retirement today: 66 segments (94.4 MiB, all of g1-server) are older than 24 h and covered by the 03:30 snapshot; the storm segments are younger than 24 h.

## 2. What existed

- `snapshotV1`, `writeSnapshotV1` (atomic tmp, fsync, rename), `readNewestSnapshotV1`, `foldStoreV1`: the fold of the newest snapshot plus the journals after its cursors, used by `join`, `state-rebuilt`, `verify`, `bootstrap`.
- `compact` (daily timer on every machine; `--collect-blobs` on the server): per origin; removes its own segments below its own snapshot, only when older than `replica.retentionDays` (14), and with `--collect-blobs` unreferenced blobs older than that. Nothing is old enough yet, so it has removed nothing.
- Recovery from a removed segment existed only from a snapshot written by the same origin (`rejoin-required` otherwise); nothing used a snapshot to catch up; blob reclaim had a race with a publisher that finds its blob already stored (the touch result was ignored).

## 3. Design and rules (code in `M/🗜️compaction/🟦️.ts`, schema `🧬️schema/🔣️.json`)

1. **Snapshot / catch-up.** Any one machine writes a snapshot (several at once are safe: each file is a complete fold at its own cursors in its own directory); `listSnapshotsV1` chooses by reading only the tail of each file. `fleetCompactV1` writes it, then reads it whole (`scanSnapshotV1`) before it justifies a removal. With `automatic` on, a replica at least `catchUpEvents` (20,000) events behind (estimate from the heads, `estimateEventsBehindV1`) that finds a snapshot saving that many reads the snapshot instead (`adoptSnapshot`, progress stage `jump`): entries win by hlc, differing files are queued like events, all cursors that the snapshot has passed are set (own origin excluded). A property test proves snapshot plus tail equals the full fold.
2. **Segment retirement.** A segment of any origin is removed when it is below the snapshot's cursor, below the head segment, below the attested cursor of every holder, at least `safetyHours` (24) old, and part of the prefix of such segments. Holders are origins that attested within `staleHours` (12) with `sync` on; origins that are silent longer, have `sync` off, are retired or never attested are ignored and resume from the snapshot when they return (`adoptSnapshot` with the origin: any snapshot that is ahead, not only the one that origin wrote; this replaces the per-origin recovery in `followSlice` and `foldInto`). Only `wallMs`, `sync` and `cursors` of `attest.json` are read: the live workers still attest without `guards`, so the full-schema reader sees none of them (`fleet` shows them as `missing`), which would have made every origin "unattested"; an unreadable file blocks with no cursors until it is `staleHours` old. g1-m05 (`sync:false`) is ignored as `off`, a silent one as `stale`.
3. **Blob reclaim.** Referenced = entries of the newest snapshot plus every `put` after its cursors (also superseded ones). A candidate older than `safetyHours` is renamed into `v1/trash`, the events appended meanwhile are read, and it is put back if it is named now, if its mtime changed since the scan, or if the reading failed; otherwise deleted. `storeBlobV1` (and the relay for blobs it already holds) now touches an existing blob and stores it again when the touch finds it gone (`touchBlobV1`), so a publish never ends with an event whose blob was removed. A blob written before its event is young and stays. Leftovers of a dead run in `v1/trash` are put back by the next one. A segment that vanishes while reading (another compaction) aborts and restores everything. `.cache/f/lost` is local and holds no store reference (interpretation of the brief); a file that goes back to the content of a reclaimed blob publishes it again (tested end to end).
4. **Commands and visibility.** `compaction --plan` (writes nothing) and `compaction --apply [--no-blobs]` (lock `v1/compaction.lock`, snapshot, retire, reclaim, prune snapshots beyond the newest two older than 24 h, `v1/compaction.json`); `status` shows journal size, newest snapshot and age, reclaimable segments, the last applied result, and with `--deep` the reclaimable blobs. Texts English and German. Topology `replica.compaction` `{automatic:false, snapshotEvents:100000, safetyHours:24, staleHours:12, catchUpEvents:20000}`; the group server (`<group>-server`) runs the compaction in its daemon every ten minutes when due, only with `automatic`.
5. **Crash safety.** One run at a time, lock refreshed every minute, taken over after two hours; segments are removed oldest first, blobs only through the set-aside; the record is tmp, fsync, rename. The unchanged `compact` timer (retention based) stays as it was.

## 4. Tests (all pass against the live tree; 28 files, macOS)

New `M/🗜️compaction/🧪️tests/`: `🟦️.ts` 15 tests (shape and committed defaults; tail versus streamed cursors; **property test**: 24 random histories with puts, deletes, links, equal-millisecond ties, a lagging clock, a retired origin, 1 to 3 rounds of compaction with 18,000-event histories that retire segments, compared with an independent in-memory fold and with the Python oracle `🐍️.py`; the plan against the oracle's retire rule for ten attestation situations; attestation drift; retirement as a prefix; stale/never/fresh replica resumes from the snapshot; far-behind replica jumps (50 events read instead of 9,050), a replica with the setting off does not; reclaim rules; seven race scenarios; lock; command and status in both languages; gating of the automatic run), `💥️killpoints.ts` (a kill before and after each of the 26 mutations of a full compaction, 47 deaths: the store folds to the same manifest, referenced blobs stay, the next run finishes). Other suites unchanged and green: journal 127+18, replica 85, race 14, killpoints 4, resilience 23, chaos 9, store 8, git 20, topology 63, attest 73, boundary 69, tiers 57, git 41, clash 17, state 8, relocate 11, bootstrap 12, adopt 15, access 5, resilience member 16. The 20-seed chaos sweep passed.

## 5. Benchmark on a copy of the live journals (no blobs; `🧰️scripts/1113-compact/bench.ts`, load average about 18)

| Step | Result |
|---|---|
| Fold (join, rebuild) with the 03:30 snapshot plus about 1.6 M events | 8.2 s, 705 MB RSS |
| `compaction --apply` (new snapshot of 275,816 entries, 91 MB JSON) | 8.2 s; retired 66 segments, 94.4 MiB |
| Fold after it (snapshot, no tail) | **3.2 s** (most of it is the 91 MB JSON; follows live paths, not history) |
| Same, once all storm segments are older than the safety age (simulated with 1 h) | journal 699 MB to 5.9 MB: 465 segments, 693 MB removed, 7 head segments stay; manifest digest identical before and after |

The replica-level cost of a 600,000-event catch-up (ingest plus apply) was not benchmarked at 273,000 files; the test shows the structure (events read 50 instead of 9,050).

## 6. Enable and expected effect

1. Check: on the group server `cd M/🔁️replica && repo ./📜️script.ts compaction --plan` (read-only), then once by hand `compaction --apply`.
2. Automatic: in `M/🗺️topology/🔣️.json` set `replica.compaction.automatic` to `true` (landing protocol applies; every daemon restarts onto it by itself and reads the topology then). Do it only after every machine runs the new code (`fleet` shows the code digest): an old daemon cannot resume from a snapshot written by another origin and would report `rejoin-required` if it returns after its segments were retired (fresh holders are protected either way).

What it would reclaim on the live store: **now** about 94.4 MiB of journal (66 segments), 3,025 blobs (382 MB) and the snapshot files older than 24 h beyond the newest two (about 180 MB); **after 05:40 tomorrow** (storm segments older than 24 h) about 693 MB of journal (465 of 472 segments) and up to the 2.55 GB of unreferenced blobs. Snapshot size (91 MB JSON) stays proportional to the live paths.

## Not verified

- Real NFS: the set-aside rename and the mtime comparison rely on the store host seeing the publisher's `utimes`; run the reclaim on the store host (default). Linux and Windows were not run.
- Cross-group relay after retirement: the relay skips segments the source no longer has and copies the newest snapshot of homed origins only; a peer group's replica resuming from a relayed snapshot written by another group's origin is not tested.
- Automatic run in a real daemon (only `autoCompactV1` is tested directly); jump and retirement across mixed code versions on the live fleet.
- Memory of the compaction inside the daemon on the real store (the benchmark process peaked at 800 MB RSS including both folds).
- The replica-level catch-up time at full scale, and the blob reclaim run on the real blob directory (only the read-only scan, 26 s).
