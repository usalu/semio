# 🔬️ Apply, Publish and Store Cost Audit of the Shared-Worktree Replica

Read-only audit of the follow, apply, publish, persistence, relay and compaction path of the replica
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`). Local change detection (watcher, scans,
hint channel, per-round checks in `runV1`, attestation view cadence) is excluded, as requested.

Scope of files read in full: `📒️journal/🟦️.ts` (446 lines), `🔁️replica/🟦️.ts` (1997 lines),
`🌿️git/🟦️.ts` (530 lines), `🚧️boundary/🟦️.ts` (scan and hash helpers), `🩺️attest/🟦️.ts` (retirement reader).
Line numbers below refer to these files (`M` = module root above).

Labels: **[measured]** = run on this Mac (throwaway scripts in `$TMPDIR/semio-audit-*`, no repo writes);
**[code]** = counted from the code path; **[est.]** = estimate, not measured.

Important context: on this Mac `/Users/Shared/semio-store` is a plain local directory (`mount` shows no NFS
mount), so every store number below is local APFS. NFS costs are estimated from RPC counts; the Linux NFS
report (`linux-nfs-report.md`, lines 136-147) already measured that `lookupcache=none` costs about 2 to 3 RPCs
per stat and that `actimeo=1` delays a remote save by up to 1 s.

---

## 1. Measurements

### 1.1 Live state and store

| Item | Value | Source |
|---|---|---|
| `.cache/f/state.json` | 106,031,725 bytes (95.1 M UTF-16 chars), 275,205 files, 0 queued, generation 22 | [measured] |
| `.cache/f/state.log` | 65,909 bytes, 22 generation lines of deltas since the last checkpoint | [measured] |
| `state.json` bytes per entry | about 385 B (the constant `stateBytesPerEntry=320` at `🔁️replica/🟦️.ts:92` underestimates it by about 17 %) | [measured + code] |
| Store origins | 5 (`g1-m01..m04`, `g1-server`), 76 journal segments | [measured] |
| Group server journal | 67 segments, 99,713,458 bytes, 272,532 events, about 366 B per event | [measured] |
| Heads | g1-m01 seg 1 at 426,897 B; g1-server seg 66 at 723,640 B (2,196 events in head segment) | [measured] |
| Snapshots | 523 MB on disk in 6 files (`g1-m01`, `g1-m02`, `g1-m03` one each, `g1-server` three), each 91 to 92 MB and about 334 B per entry; the same fold is written once per origin directory | [measured] |
| Blobs | 243,173 files, 13 GB. Three large blobs: 3,972,367,425 B (the git pack), 1,468,882,359 B, 857,516,312 B | [measured] |
| AppleDouble `._*` files | 4,923 under the store, 0 under `.cache/f` | [measured] |
| `.git/objects` | 3.9 GB; `pack/pack-303ed9a6...pack` is 3,972,367,425 B, its `.idx` 64 MB, `multi-pack-index` 57 MB, `.rev` 8 MB | [measured] |
| `.git/index` | 69,474,370 B, 272,319 entries | [measured] |
| Replica daemon (pid 76605) | RSS 675,600 KB, 4 min 32 s CPU over 64 min elapsed (about 7 %) | [measured] |

Note: `.git/objects` is a shared directory (`gitSharedV1`, `🚧️boundary/🟦️.ts:177-188`), so the whole 3.9 GB object
store is replicated as ordinary blobs. The store blob of the 3.97 GB pack is byte-identical to the local pack file.

### 1.2 Micro-benchmarks (throwaway scripts in `$TMPDIR`, real module code where exported)

| Operation | Result | Notes |
|---|---|---|
| `JSON.parse(state.json)` whole | 566 ms, RSS 542 MB after (object alive) | [measured] |
| `streamMembersV1` + per-entry `JSON.parse` (what `loadState` does) | 645 ms for 275,205 entries | [measured] |
| Compact tables built from the stream (`files` + `manifest`, `intern`, `write`) | 1,791 ms, RSS delta 385 MB, heap 127 MB | [measured] |
| Forced full `Bun.gc(true)` with the tables loaded | 1 ms (5 runs: 7.6, 1.1, 0.9, 0.9, 0.9 ms) | [measured], so `settleHeap` is not a cost driver |
| `ids()` scan of the files table (299k flag slots) | 0.36 ms per call | [measured] |
| Per missing path in `observe()`: full `book.name()` scan over all live ids | **81 ms per call** | [measured], see item 7 |
| `readNewestSnapshotV1` on the 91 MB snapshot (schema check per entry) | 1,272 ms, 275,280 entries | [measured] |
| Full read of the group server journal (`readEventSpansV1`, no bound) | **2,459 ms, 390 MB RSS, 272,542 events** | [measured] |
| of which `requireEventV1` (schema check per event) | **1,494 ms (5.5 µs per event)**; JSON parse 135 ms; line split 141 ms | [measured] |
| `stat` of one `head.json` (local) | 15 µs; `readFile` of one head 34 µs; `listOriginsV1` cached 15 µs | [measured, local FS only] |
| Small file create + write 4 KiB + fsync + rename (local APFS) | 0.40 ms; without fsync 0.31 ms | [measured, local only; NFS COMMIT is not measured] |
| SHA-256 of 256 MB, 64 KiB reads / 1 MiB reads | 1.43 GB/s / 2.08 GB/s | [measured] |
| Copy with 64 KiB stream + hash (writeBlobV1 shape) | 716 MB/s | [measured] |
| `copyFile` on the same APFS volume | 751 GB/s (APFS clone); cross-volume (NFS) copies are not clones | [measured, not representative for NFS] |
| Spawn of `git --version` | 10.8 ms | [measured] |
| `git update-index -z --index-info` on a temp copy of the 69 MB index: 1,000 entries / all 272k entries | 271 ms / 601 ms (each call rewrites the whole index) | [measured on a temp index only] |
| `git ls-files -s` on the same copy | 2.8 s, dominated by formatting 58 MB of output | [measured] |

Not measured, deliberately: `git write-tree` on the live index (it writes or freshens tree objects in the shared
`.git/objects`, which changes object mtimes and therefore retention), any daemon restart, any write to the store.
The restart and write-tree numbers below are estimates.

---

## 2. Answers to the seven questions

### 2.1 Follow: what is read per poll round, as a function of N origins

Code path of one idle follow round on a worker (`followOnceV1`, `🔁️replica/🟦️.ts:1374`, called once per round from
`runV1` at line 1425):

- `refreshIgnore` (line 906): local `stat` of `.gitignore`, revalidated every 2 s. No NFS.
- `listOriginsV1` (`📒️journal/🟦️.ts:254`): one `stat` of `v1/origins` per round; `readdir` only every 5 s.
- `readHeadV1` (`📒️journal/🟦️.ts:235`) for each of the N-1 other origins: one `stat` of `head.json` per round.
  Every 5 s (`headRevalidateMilliseconds`, line 23) it also `readFile`s the head even when the stat did not change
  (`readHeadV1` falls through to `readFile` after the revalidation window). That is 3 RPCs per origin per 5 s.
- Only for origins whose head changed: `readEventSpansV1` (line 278) repeats `readHeadV1`, then per segment
  `open` + `fstat` + `read` + `close`.
- `runV1` (outside my scope, but it is store traffic on every round): `storeReachableV1` (`stat` of the store root,
  line 519), `readRetirementV1` of the own origin (a `readFile` that usually returns ENOENT, `🩺️attest/🟦️.ts:193`).

Store operations per idle round [code]: `3 + (N-1) + 0.2*(N-1)` where the last term is the 5 s revalidation
amortised over 20 rounds. With the poll at 250 ms that is 4 operations per second per origin per worker.

| N origins | store ops per round | ops per second per worker | fleet of 50 workers (est.) |
|---|---|---|---|
| 5 | about 9 | about 36 | about 1,800 /s |
| 50 | about 62 | about 250 | about 12,500 /s to one store |

The scale report measured about 52 GETATTR per 250 ms per machine with `actimeo=1` (`scale-report.md`, line 197),
consistent with this count. On Linux with `lookupcache=none`, each `stat` is a server round trip (linux report,
line 145: 32 RPC/s at 2 origins × 4 polls/s). Estimate: one NFS GETATTR costs 0.2 to 1 ms on a LAN, so N=50 costs
about 12 to 60 ms of wall time per round, which is 5 to 25 % of the 250 ms period before any apply or publish.

Cheaper change signals (see design 2 in section 4):

- **Flat head directory**: move the heads to one directory `v1/heads/<origin>.json`. A rename inside a directory
  changes that directory's mtime, so one `stat` of `v1/heads` says whether any origin published since the last
  round. Only then read the changed heads (`readdir` + reads). This turns N stats into 1 stat (plus one rename per
  publish batch on the writer side). The current per-origin directories (`v1/origins/<id>/head.json`) do not work,
  because a rename there changes only the subdirectory's mtime, not `v1/origins`.
- **Hint from the group server**: the group server already runs `repo remote-cache serve --port 6090` (seen in
  `ps` on this Mac). A long-poll or push endpoint there would remove the polling entirely. I did not read that
  module; it needs its own check.
- Overlap: the excluded "hint channel" work may already cover this. Coordinate before implementing.

### 2.2 Publish: per changed file

Per changed file with new content, the code path is (`publishOnceV1` line 1033, `prepare` line 937, `writeBlobV1`
line 173, `appendEventsV1` line 197):

1. **Hash pass 1**: `prepare` calls `hashFileV1` (`🚧️boundary/🟦️.ts:276`): `stat`, full read through `createReadStream`
   (64 KiB chunks), `stat`.
2. **Hash pass 2 and copy**: `writeBlobV1` calls `exists(target)` (stat), then `ensureStoreDirectoryV1` for the
   prefix directory and `tmp` (`mkdir` each, EEXIST as the common case), `open(tmp, "wx")`, then reads the source
   **again** through a 64 KiB stream, hashing and writing each chunk to the NFS tmp file, `chmod`, `fsync`, `close`,
   `rename` (EEXIST handled). So a new file costs two full local reads and one NFS write of its full size, plus one
   COMMIT (`fsync`) and about 8 metadata RPCs [code]. A blob that already exists costs one read plus `utimes`.
3. **Journal, once per batch of up to 256 events** (`chunkSize`, line 92): `readHeadV1` (stat), `ensureStoreDirectoryV1`
   for `segments`, `exists(segment)`, `open`, `fstat`, write, `fsync`, `close`, then `replaceFile` for `head.json`:
   `mkdir`, `open(tmp)`, write, `fsync`, `close`, `rename`. About 14 RPCs and 2 COMMITs per batch [code].
4. `ensureHome` once per process (`stat`), `persist` after the batch (local writes only, 5 s throttle).

Single-file publish, end to end: about 23 RPCs and 3 COMMITs to the store, plus two full local reads of the file
[code, RPC count not wire-traced]. Batching across files already happens for the journal (256 per batch) but not
for the blobs: each blob has its own `fsync`, `rename` and metadata round trips. Blob writes run 8-wide through `pool`
inside a chunk, so the COMMITs overlap, but the chunk waits for its slowest blob.

The 3 GB git pack case, step by step:

- Origin publish: 2 full local reads (hash pass 1 and pass 2) + 1 NFS write of 3.97 GB + one COMMIT of 3.97 GB.
  The COMMIT is the expensive one (server flushes 4 GB before returning) [est. tens of seconds to minutes on a LAN].
- The publish loop is `await`ed inside `runV1` (line 1425 onward), so during these minutes the follow, the apply and
  the local change loop do not run on that machine. This is a latency problem for all users of the machine, not just
  for the pack.
- Relay (group server, `relayBlob` line 1857): `copyFile` from the origin's store (read 4 GB + write 4 GB, no fsync),
  then `hashFileV1` of the copy (read 4 GB), then `rename`. Three full passes on the NFS side per relay hop.
- Worker apply (`materialise` 1210 to 1299): `copyFile` from the store to local `.cache/f/tmp` (NFS read 4 GB), then
  `hashFileV1` of the local copy (local read 4 GB, about 2 s of CPU at 2 GB/s), `chmod`, `rename`.
- Per group of 10 workers [est.]: the origin writes 4 GB; the relay reads and writes about 12 GB; the 10 workers read
  4 GB each, so about 56 GB of store traffic per change of the pack. Across 5 groups this is about 280 GB for one
  repack. Repacks rewrite the whole pack, so every repack costs this much again.

### 2.3 Apply: per event, concurrency and order constraints

`applyQueue` (line 1301) processes `replica.queue.snapshot()` with `pool(items, concurrency=8)`. Within one
follow round there are no ordering constraints between paths. The constraints that do exist:

- Branch refs (`.git/refs/heads/...`) go through `linearise` (line 1193) and are processed per path only.
- The git stage (`applyIndexV1`, `🌿️git/🟦️.ts:321`) runs after the whole pool, serially, once per round.
- The follow loop itself is sequential: `followOnceV1` → `applyQueue` → `publishOnceV1(kept)` → `persist`. No
  apply runs while a publish is in progress on the same machine, and the pool does not span rounds, so one round can
  take as long as its backlog.

Per put event on a worker [code]:

- `lstat` of the target, and when a file of the same size is already there `hashFileV1` of it (a full local read).
- `copyFile` of the blob from the store (1 NFS read of the full size), `hashFileV1` of the local copy (1 local read),
  `chmod`, `lstat`, `mkdir -p` of the parent (1 to 2 syscalls), `rename` into place. No fsync on the worker, which is
  correct for a cache but means a crash can leave an applied file that the state does not record (`replayOwnTail`
  covers the journal side).
- Measured on a loaded daemon (scale report): about 2.1 ms per applied file, 8-wide.

**Racy classification after apply (publish-side waste)**: `materialise` takes `seenMs` before the copy (line 1211)
and `mtimeMs` from the temporary file after the copy (line ~1297). The copy's mtime is later than `seenMs`, so
`seen - mtime < 2000` holds and `racyAt` (line 266) is true. Every applied file is therefore "racy" until a publish
touches it, so the next full scan re-hashes every applied file once (`prepare` then returns `touch`). After a bulk
join or a large apply this is one extra full read of the applied bytes [code; cost est. equal to one scan-and-hash].
Fix: stamp `seenMs` with the `lstat` time after the rename (item 10 in section 3).

### 2.4 Git stage batches (`applyIndexV1`, `publishIndexV1`)

Per follow round that has pending git registers (`applyQueue` calls `applyIndexV1` every round, and it returns early
only when `pending.size===0`):

- `publishIndexV1` (line 280): `gitPresent` (2 lstat), `stat` of `.git/index`, `readKnown`, `rebaseRequested` (lstat).
  When the index signature changed, `writeTreeV1` (1 spawn, reads the whole index, writes missing trees).
- `missingObjectsV1` (line 171): 1 spawn (`cat-file --batch-check`).
- Only on a base change: `writeTreeV1` again (spawn), `diffTreeV1` (spawn, plus `emptyTreeV1` spawn when the base is
  null), `updateIndexV1` for the delta (spawn, index rewrite), `readTreeV1` on the first base, `updateIndexV1` for the
  ready entries (spawn, index rewrite), `normaliseGitConfigV1` (spawn of `config --list`, probes).
- Always at the end: `writeTreeV1` (spawn) and `writeKnown` (JSON tmp + rename).

Cost [code + measured spawn/rewrite]: a base-change batch is 6 to 8 spawns (about 65 to 90 ms at 10.8 ms each),
2 index rewrites (about 0.3 s each for 1,000 entries, measured above; the full 272k rewrite is 0.6 s) and 3 or
4 `write-tree` passes. `write-tree` on a 69 MB index is not measured: estimate 0.5 to 1 s each because the cache-tree
should make unchanged subtrees free. So a base-change batch is about 1 to 3 s of wall time, and a non-base batch with
pending entries still costs one `write-tree` plus one `update-index` (about 1 s).

Order constraints: the stage batch is serial per machine and must be, because it rewrites `.git/index` (a
`index.lock` makes it retry after one second, `RETRY_MILLISECONDS`, `🌿️git/🟦️.ts:48`).

### 2.5 Persistence, restart (about 30 s) and what a warm restart needs

Persistence today (`🔁️replica/🟦️.ts`):

- `appendLog` (864): one JSON line per changed key (about 350 B) into `state.log`, no fsync.
- `checkpoint` (839): rewrites the **whole** `state.json` (106 MB, `JSON.stringify` per entry, 1 MB chunks, no fsync)
  and truncates the log. It runs when the log exceeds 16 MB (`logLimitBytes`, line 92), when more than 50,000 keys
  changed (`checkpointChanges`), and on every daemon stop (`persist(final)` at line 879 runs `checkpoint` whenever
  there are any changes, so every stop rewrites the 106 MB).
- `persist` (879) also rewrites `clock.json` and `cursor.json` each time it runs (small, tmp + rename, no fsync).
- Measured cost of the full-state pieces: stringify 382 ms, the JSON write is the rest [measured stringify; write
  not measured].

Load at start (`openReplicaV1`, 814, `loadLocal` 654, `loadState` 627):

- Measured: stream + per-entry parse 0.65 s; compact table build 1.8 s total (the stream is part of it).
- Then `ReplicaFoldsV1.add` for every file (`count` decodes each path char by char, line 376-444), `requeue` (O(N)),
  `replayOwnTail` (O(events since own cursor), normally 0), `settleHeap`.
- Not measured but likely big: `attester.start` (in `runV1`, which runs `gitStateV1` → spawns and a `write-tree`, and
  possibly `gitViewV1` = `git status` of 272k files), and the first full publish walk (`walkLocalV1`, 16-way `lstat`
  of 272k entries, plus `candidates` over all ids).

So the 30 s is most likely the first walk and the git view at attester start, plus the 2.5 s load. This needs
`[DEBUG]` timers at the phases, not a guess; I did not restart the daemon.

What makes a warm restart near-instant (design 5 in section 4): a binary checkpoint of the tables (columns plus
path pages, mmap-able) so that load is typed-array views, no per-entry JSON; a sharded checkpoint so that only
changed shards are rewritten; no final full rewrite on a clean stop; and a stored git index signature so that the
start does not spawn `write-tree` when the index did not change. Skipping the first walk needs the change-detection
agent's signature work.

### 2.6 Memory: why 540 to 680 MB, and what can be trimmed

Measured components [measured]: `JSON.parse` of the whole state gives 542 MB RSS (object alive). The compact tables
give 385 MB RSS delta with 127 MB heap, of which the path pages are 46 MB (`bookPathBytes`, 1 MiB pages), and the
rest is typed-array growth (`widen`, 1.5× copies, line 109, old buffers freed only by GC) plus allocator retention.
The daemon at 676 MB is consistent with that plus the scan, `ReplicaFoundV1` (line 445), and the journal read buffers.

Trimming candidates [code, sizes est.]:

- `stateBytesPerEntry=320` (line 92) reserves too little (actual 385 B), so the tables grow and copy during load.
  Fix the constant or measure the file: about 15 % fewer copies.
- The `manifest` table (built in `tablesV1`, line 593, as `CompactTableV1(book,false)`) holds a second copy of every put (hlc, sha, size) for
  every path, even when it equals `files`. Store only the rows where `files` differs (queued or ahead), about 30 to 40 %
  of the table memory [est.].
- `readEventSpansV1` (line 278) creates one `{segment,bytes}` object per event in `ends` (272k objects for the
  server journal) even when the caller never needs the cut points. Compute them only when `admit` cuts (line 1363).
- Full journal reads without a bound (item 8): 390 MB RSS for 272k events.
- `hashFileV1` and `readRange` allocate a fresh `Buffer` per call; fine per file, but the 16 KiB `streamMembers`
  chunk and per-entry `JSON.parse` of 275k objects are the largest transient allocations on load.

Estimate: items above together remove 150 to 250 MB of the 676 MB RSS. The remainder was not attributed by a heap
profile; that needs a profile of the running daemon, not a guess.

### 2.7 Relay and compaction: growth with history

- **Relay per round** (`relayOnceV1`, 1921): for every origin homed at the peer, `relayAttestation` (1908) reads
  `view.txt` and `attest.json` from the peer and from the own store on every round (4 whole-file reads, no
  stat cache; the doc comment above `relayOnceV1` at line 1920 says so). `relayRetirement` (1913) reads two files per
  origin on every relay round. The relay interval is `topology.replica.pollMilliseconds*4` = 1 s (`🔁️replica/⌨️command/🟦️.ts:218`).
- **Relay of a new snapshot** (`relaySnapshot`, 1875): it parses all 275k manifest entries of the new snapshot and calls
  `relayBlob` for every `put` sha; each call does a `stat` of the destination (275k stats per new snapshot per origin,
  8-wide). With N origins compacting, each group server does this N times.
- **Compaction** (`compactV1` 1649): every machine has a compact service (`🗺️topology` provision, step 6). Each call
  runs `snapshotV1` (full `foldStoreV1`, line 1617), writes a full 91 MB snapshot into its own origin directory
  (`writeSnapshotV1` 329: `snapshots/<originId>/<hlc>.json`, the global folded manifest, identical bytes for every
  origin that folds the same hlc), then folds the store again (line 1657, second full fold) and reads all events
  again for `referencesSinceV1` when blob collection is on. So compaction is O(store) per origin, and writes O(N ×
  91 MB) per cycle. The 523 MB of snapshots on disk is this duplication (`snapshots/g1-m02` and `g1-m03` each hold a
  copy of the `g1-server` snapshot with the same hlc).
- **Hot path O(history)?** No: follow reads from its cursor, the relay from its own `have` head, and segments are
  removed only below the own snapshot cursor and older than `retentionDays` (line 1649-1660). The growth that matters
  is in the folds (compaction and join), not in the per-round path. A cold join reads the newest snapshot (1.3 s,
  91 MB) plus the events after it.
- Snapshot size: 334 B per entry (JSON, the hlc repeated per entry). A binary entry would be about 60 to 80 B [est.].

### 2.8 git objects as files

The current objects are mostly packs (19 files in `objects/pack`, 44 loose fan-out directories with about one loose
object each in the sample), so the loose-object explosion does not happen today. For the case in the question, a
`git add -A` of 1,000 new files creates about 1,000 loose blobs plus a few hundred trees, all in the shared
`.git/objects`. Per object that is: a scan entry, a `prepare` hash, a blob write (8 metadata RPCs and one COMMIT), a
journal line (1/256 of a batch) and on the worker a `copyFile` plus a local hash. [est.] Publish 2 to 10 s (COMMIT-
bound), apply 1 to 3 s, plus the git stage batch. Options in section 3, item 9.

---

## 3. Ranked optimisations

Gains are estimates; effort S/M/L; risk L/M/H. "Where" uses the module-relative paths.

| # | Change | Expected gain [est.] | Effort | Risk | Where |
|---|---|---|---|---|---|
| 1 | Stop re-replicating large git packs and indexes as whole files; single-pass copy+hash for the rest | Per pack change: store traffic 56 GB → about 6 GB per group; origin and relay passes -1 each | M | M | `🚧️boundary/🟦️.ts:177-195` (`gitSharedV1`); `📒️journal/🟦️.ts:173` (`writeBlobV1`); `🔁️replica/🟦️.ts:937` (`prepare`), `1857` (`relayBlob`), `1210` (`materialise`) |
| 2 | Flat head directory as the change signal (1 stat per round) | Store metadata ops per worker per second: 250 → about 10 at N=50 (-95 %) | S-M | M | `📒️journal/🟦️.ts:235` (`readHeadV1`), `197` (`appendEventsV1`), `254` (`listOriginsV1`); `🔁️replica/🟦️.ts:1374` (`followOnceV1`), `1921` (`relayOnceV1`) |
| 3 | Fix the racy classification of applied files (stamp `seenMs` after the rename) | One full re-hash of the applied tree after every bulk join or apply removed (on a 270k tree, the largest single read in the path) | XS | L | `🔁️replica/🟦️.ts:1210` (`materialise`, end), `1560` (`joinV1` adoption), `266` (`racyAt`) |
| 4 | Bounded catch-up in `followOnce` (one segment per round, like `foldInto`) | Peak memory and stall of a catch-up round: 2.5 s and 390 MB → about 0.1 s and 15 MB per round [est.] | XS | L | `🔁️replica/🟦️.ts:1374` (`readEventSpansV1(storeRoot,origin,cursor)` without the segment bound; compare `1514` `foldInto`) |
| 5 | Replace the generic schema walker with hand validators for `Event`, `Head`, `ManifestEntry`, `Path` (keep the JSON schema as the test oracle) | Journal read CPU -60 % (1.49 of 2.46 s); snapshot load and join faster by the same share | S-M | L | `📒️journal/🟦️.ts:26` (`check`), `90` (`requireEventV1`), `419` (`readNewestSnapshotV1` per-entry `check`) |
| 6 | Binary, sharded checkpoints; no full rewrite on a clean stop | Warm load 2.5 s → 0.1 to 0.3 s [est.]; checkpoint write -90 % for typical churn [est.] | M-L | M | `🔁️replica/🟦️.ts:627` (`loadState`), `839` (`checkpoint`), `879` (`persist`), `900` (`closeReplicaV1`) |
| 7 | Compaction once per group; binary snapshot in a shared directory; relay the snapshot's blobs from a manifest diff | Snapshot bytes -75 % per cycle; removes N × 91 MB writes and N × 2 full folds per cycle; relay: 275k stats per new snapshot removed | M | M | `📒️journal/🟦️.ts:329` (`writeSnapshotV1`), `🔁️replica/🟦️.ts:1617` (`snapshotV1`), `1649` (`compactV1`), `1875` (`relaySnapshot`) |
| 8 | `observe` without the O(N) scan per missing path (sorted path index or per-directory child count) | 81 ms per missing directory → microseconds; a `rm -rf` or `git checkout` that removes 300 directories spends 24 s in one publish now | S | L | `🔁️replica/🟦️.ts:956` (`observe`, the `replica.files.ids()` loop under `!replica.files.has(path)` and the expanded-directory loop) |
| 9 | Git stage: one `update-index` per batch (delta and ready entries together), drop the first `write-tree` (compare to the known tree) | Base-change batch -30 to -50 % of wall time [est.]; 1 index rewrite less per batch | S | M | `🌿️git/🟦️.ts:321` (`applyIndexV1`), `280` (`publishIndexV1`), `156` (`updateIndexV1`) |
| 10 | Loose git objects published as one pack per publish batch (2 files instead of about 1,000) | 1,000-file `git add -A`: publish 2 to 10 s → below 1 s [est.] | M | M | `🌿️git/🟦️.ts` (object files reach the publish path via `gitSharedV1`); `🔁️replica/🟦️.ts:1033` (`publishOnceV1`) |
| 11 | Cache relay attestation and retirement reads by stat signature | About 4 whole-file reads per origin per second removed on each relay | XS | L | `🔁️replica/🟦️.ts:1903` (`relayOriginFile`), `1908` (`relayAttestation`), `1965` (`relayV1` loop) |
| 12 | Memory trims: `stateBytesPerEntry` 320 → measured 385; manifest table stores only differences from `files`; no `ends` objects unless cut | 150 to 250 MB RSS of 676 MB [est.] | S | L | `🔁️replica/🟦️.ts:92` (constant), `33` (`manifest` table), `278` (`readEventSpansV1`) |

Item 12 (memory) and item 11 are the cheapest wins; items 1 and 2 are the biggest. Item 3 is a correctness-neutral
bug fix with a clear cost, and item 4 is a one-line change.

---

## 4. Design sketches for the top five

### Design 1: large objects are not whole-file blobs

Problem: `.git/objects` is one shared tree, so each pack and index of the repository is a blob. One repack of the
3.97 GB pack costs about 56 GB of store traffic per group (section 2.2), and the publish loop blocks for the COMMIT.

Sketch:

- Keep git objects replicated, but split blobs above a threshold (e.g. 64 MiB) into fixed 8 MiB chunks. The blob
  record becomes a small manifest `{chunks:[sha...], bytes}` and each chunk is an ordinary immutable blob. A
  repack still changes most chunks (packs are not deterministic under repack), so the gain is limited to
  append-only or stable packs. Measure first how often the packs change (`git count-objects -v` history is not kept;
  add a `[DEBUG]` counter in `publishOnceV1`).
- Stronger option: packs are immutable once written. Only the archivist repacks, and it publishes the new pack as a
  new blob and removes the old one through the normal compaction, so workers copy each pack once.
- Single pass everywhere: `writeBlobV1` stages in `tmp` while hashing (one read), dedup by `exists` after the hash,
  `rename` if new. `relayBlob` copies with hash in the same stream (no read-back). `materialise` copies from the
  store with hash in the same stream (no local read-back). Hashing at 1 MiB chunks is 45 % faster than 64 KiB (measured).
- Keep the stat-before/after stability check (`hashFileV1`) and the `sha256` comparison before `rename`; that is the
  part that makes the single pass safe.

Gain [est.]: origin 2 reads → 1; relay 3 passes → 1 read + 1 write; worker 2 passes → 1 read + 1 write. Per pack
change: store traffic about -80 %. Risk: git reachability of a worker during the transfer (the event must not overtake
its blob, which is already the rule at `relayOnceV1`). Test: a language-agnostic test with a hash oracle (sha256sum)
and the same output checked against a third-party tool, per the house rules.

### Design 2: one change signal per store, not one head per origin

Problem: each round stats N heads (section 2.1), about 250 store operations per second per worker at N=50.

Sketch:

- Writers: `appendEventsV1` and `relayOnceV1` write their head to `v1/heads/<origin>.json` (one flat directory)
  with the same tmp + rename, no fsync (the head is a hint; the journal bytes are fsynced before it).
- Readers: each round, `stat(v1/heads)`. If the mtime and the entry count are unchanged, skip the round's head reads.
  If changed, `readdir(v1/heads)` and `readFile` only the heads whose names are new or whose size changed (stat each
  one that moved). The full read of all heads stays as a safety net every `scanSeconds`.
- Correctness: the head is still the only truth about how far an origin's journal goes; the directory mtime is only
  a trigger. Under NFS, `actimeo` may delay the mtime by up to 1 s (linux report). That is the same staleness as
  today and is accepted by the scale report.
- Overlap check: the excluded "hint channel" work may already implement this (or the group-server push). Do not build
  both.

Gain [est.]: round cost 1 stat + (changed origins) reads instead of N stats + revalidation reads; metadata ops per
worker -95 % at N=50. Effort: S for the layout change (one path in `journal/🟦️.ts` plus the readers in
`replica/🟦️.ts:1374` and `relayOnceV1`), and a migration is not needed (greenfield). Risk: a missed rename would hide an
origin; the periodic full read covers that within `scanSeconds`.

### Design 3: single-pass blobs and no racy re-hash after apply

Problem: a changed file is hashed twice on publish, a relayed blob is read back once, an applied file is hashed once more
locally, and every applied file is re-hashed on the next full scan because `seenMs` is taken before the copy.

Sketch:

- `writeBlobV1(storeRoot, originId, source)` returns the sha: it streams `source` to `tmp` with a 1 MiB reader and hash,
  checks stability with the stat before and after, then `exists`, unlink or `rename`. `prepare` no longer hashes the
  file first; the blob path comes out of the same pass. The journal event then carries the sha as before.
- `materialise`: take `seenMs` from `lstat(target)` after the `rename` (the file's own mtime), so that
  `seen - mtime >= 2000` is false only when the file is really older than the write; equivalently store `seenMs = mtime`
  and treat the apply as already seen. No extra read.
- Relay: copy with hash in one stream, verify, rename. No `hashFileV1` of the temporary file.

Gain [est.]: publish reads per changed file 2 → 1; relay reads per blob 2 → 1 (plus the copy); no whole-tree re-hash
after a bulk apply (about one scan-and-hash of 270k files saved per join). Risk: low; the hash comparison before
`rename` is kept.

### Design 4: journal and snapshot reads without the generic validator

Problem: `requireEventV1` takes 5.5 µs per event (1.49 s of the 2.46 s full journal read), and `readNewestSnapshotV1` runs
two checks per manifest entry. A cold join or a fold reads every event once per origin, and compaction reads them twice.

Sketch:

- Write a hand validator per definition (`Event`, `Head`, `ManifestEntry`, `Path`, `Cursor`): a few typed field tests
  each, no walk of the schema per call. Generate or hand-write; keep the JSON schema as the oracle and run both on the
  same fixtures in tests (the house rule of "same output with a third-party library" holds).
- Bound `followOnceV1` to one segment per round (`readEventSpansV1(...,1)`), as `foldInto` already does. The loop comes
  back for the rest; no correctness change because the cursor advances per round.
- Move `ends` out of the hot path (compute only when `admit` cuts).

Gain [est.]: full journal read 2.46 s → about 0.8 s; catch-up memory bounded per round. Effort S-M. Risk L with the oracle
test.

### Design 5: binary, sharded checkpoints and a warm start

Problem: the state is one 106 MB JSON document rewritten whole on every checkpoint and on every clean stop; loading it
costs 0.65 s of parsing plus 1.8 s of table building; the warm start also repeats work that did not change.

Sketch:

- Checkpoint format: per path id, the columns already held in memory (`flags`, `ms`, `counter`, `origin`, `bytes`,
  `sha` 32 B, `mtime`, `seen`), plus the path pages as raw UTF-8 with lengths, plus the queue bits and the cursors and
  clock. Size about 110 B per entry plus the path bytes (about 75 MB for 275k entries, versus 106 MB now). Load: read,
  build views, no per-entry parse; estimate 0.1 to 0.3 s.
- Shards: 256 files by path hash, each with its own generation. `appendLog` already records the key; a checkpoint rewrites
  only the shards with changed keys, plus the log. Typical churn rewrites a few MB, not 106 MB.
- Clean stop: skip the final full checkpoint when the log is short; the log replay at start is cheap.
- Start: store the `.git/index` signature with the state and skip `write-tree` at start when it matches (the attester
  is the other agent's; coordinate). Skip `requeue` by storing the queue (it is already in the checkpoint).
- Durability: write each shard as tmp, `fsync`, `rename`. Today `checkpoint` has no fsync before the rename, so a
  power cut can leave a short `state.json` that `loadState` refuses to start from (see section 5).

Gain [est.]: warm load 2.5 s → 0.1 to 0.3 s; checkpoint write -90 % for typical churn; clean stop no longer 106 MB. The
30 s restart is still dominated by the first walk and git view (not measured), which this design does not remove.
Risk: M (format). Greenfield, so no migration is needed (AGENTS.md).

---

## 5. Side findings (not performance, but found on the same path)

- `checkpoint` (839-862) renames `state.json` without `fsync`; a power cut can leave a short or empty state that
  `loadState` rejects (`Closed replica state required`). Fix with design 5.
- `relayBlob` (1857) and `relaySnapshot` (1875) copy without `fsync` before `rename`, while `head.json` is written
  with `fsync` (`replaceText`, 1844). After a crash a head can reach the store before its blob bytes. Workers then
  see `blob-corrupt` or `blob-missing` and queue, so it self-heals, but it costs a retry cycle.
- `writeBlobV1` (173-195) fsyncs each blob, then `rename`s without a directory fsync. Acceptable for content-addressed
  blobs, but the same reasoning as for the head applies.
- The store contains 4,923 AppleDouble `._*` files. They are harmless (the readers filter by name) but they are a sign that
  the store was copied through a non-native filesystem; check the mount on the group servers.

---

## 6. What was not done and what to check next

- No NFS measurement: the store here is local. The next step is `nfs-rpc-cost.py` (linux report) with the write path of
  `writeBlobV1` and `appendEventsV1` on the real mount, to replace the RPC counts above with wall times.
- `write-tree` on the live 69 MB index was not timed (it writes or freshens shared tree objects). Time it on a copy of the
  repository in `$TMPDIR` if the estimate of 0.5 to 1 s matters for the decision in item 9.
- The restart breakdown needs `[DEBUG]` timers around `openReplicaV1`, `attester.start` and the first `walkLocalV1`, measured
  on a worker that is allowed to restart.
- The `remote-cache serve` on port 6090 was not read; design 2 depends on whether it can carry a hint.
- Idle CPU (about 7 % for the daemon measured here) is not explained by the follow path: per round the follow costs about
  N stats plus `listOriginsV1` cache hits (microseconds locally). The likely contributors are the excluded attestation tick
  and its `git` spawns (`gitStateV1`, 2 to 3 spawns per tick), which is the other agent's area.

Files created: only this markdown file. Benchmark scripts were written to `$TMPDIR/semio-audit-*` (outside the repo and the
ticket). No source file was edited, no git command that changes state was run, no process was stopped, and nothing was
written into `/Users/Shared/semio-store` or `/Users/Shared/semio/.cache/f`.
