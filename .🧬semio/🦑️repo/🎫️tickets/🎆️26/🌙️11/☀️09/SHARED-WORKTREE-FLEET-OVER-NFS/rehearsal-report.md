# Shared worktree fleet: rehearsal report

Single host (macOS, APFS, Bun 1.4.2, load average 2 to 9 from other work), four clones of the real repo (`g1-a`, `g1-b`,
`g2-a`, `g1-c`: 269,484 files, 9.1 GB incl. ignored files, no `.git`), two local stores (`store-g1`, `store-g2`), all under
`$SCRATCH/rehearsal`. The real repo was never touched (`/Users/ueli/Documents/semio/.cache` does not exist). Raw logs:
`$SCRATCH/rehearsal/logs`. Scripts: `🧰️scripts/` next to this file (`rehearsal-env.sh`, `daemons.sh`, `compare-trees.py`,
`live-latency.py`, `live-bulk.py`, `live-race.py`, `live-misc.py`, `idle-sample.py`, `dummy-origins.py`).
The clone, `cp -Rc` of 67 top-level entries, was not timed. Peak disk use of the rehearsal was about 20 GB.

Not run: the unit tests (`bun test` fails in a clone with `Cannot find package 'ajv'`; no `bun install` allowed). The
`scanSeconds` edits to the topology tests and Python oracle are therefore unexecuted.

The `repo` launcher was not needed: `bun "./<module>/🔁️replica/📜️script.ts" <command>` run inside the clone works (the
shebang `#!/usr/bin/env repo` only matters for direct execution). No driver was written.

## Step results

| Step | Result |
|---|---|
| 1 Clone, stores, peers | PASS after defect 1 (no way to point `relay` at local peers; `--peers` added). |
| 2 Seed, snapshot, archive | PASS. |
| 3 g1-b by tar, settle, join, verify, oracle | PASS after defects 3 and 4 (first attempts failed). |
| 4 Relay both ways, g2-a join without archive | PASS. |
| 5 Live sync (a) to (j) | PASS after defects 5 and 7 (latency spikes, slow directory rename). |
| 6 Idle cost, 50 origins | Partly: CPU met only after defects 6 and 8 and is borderline. RSS met after Round 5 (idle 238 MiB, peaks below 520 MiB; see the Round 5 block under 6). |
| 7 Compact, verify, join g1-c | PASS. |
| 8 Stop daemons | PASS: every recorded PID checked gone; relays and `run` logs end with `stopped`. |

## Measurements

### 2 Seed (origin g1-a, group g1, store-g1)

- Wall 56.5 s (user 37.8 s, sys 110.4 s), max RSS 2.01 GB (peak footprint 556 MB).
- Published 269,321 files, 4,847,551,233 bytes (4.51 GiB), 231,489 distinct blobs.
- Refused 87 paths, all `unsafe-link`: absolute symlinks in ticket `mirror/` trees and one `sb-config/🎨️styling` link (as the runbook says on purpose; the runbook writes 88).
- Store: 4.8 GB (blobs 4.7 GB, journal 93 MB in 67 segments). `state.json` of the replica: 102 MB.
- `snapshot`: 6.9 s, max RSS 1.18 GB (269,321 paths, 89 MB file).
- `bootstrap-archive`: 62.5 s, max RSS 1.26 GB, `worktree.tar` 5.34 GB, 269,321 files, 0 skipped. Printed `sha256 {sha256}` literally (defect 2).

### 3 Machine g1-b

- `tar -xf` (bsdtar 3.5.3): 44.9 s, 269,321 files.
- `bootstrap-settle`: 51 to 68 s, max RSS 1.2 to 1.5 GB. `join`: 45.0 s (first try 57.8 s), max RSS 2.29 GB (first try 2.43 GB).
- `verify --json`: g1-a 23.2 s / 2.31 GB, g1-b 20.5 s / 1.96 GB, both `ok:true`, 0 drift, checked 269,321. After all tests: g1-a, g1-b, g2-a and g1-c all `ok:true`, 269,950 checked.
- Independent check (`compare-trees.py`, 5,000 random common files by sha256 and exec bit, full path-set comparison): g1-a vs g1-b 0 mismatches; path sets differ only by 163 files that exist on g1-a and are ignored (`.nx/workspace-data`, `jco` outputs); g1-b vs g2-a and g1-b vs g1-c: 0 only-A, 0 only-B, 0 mismatches.
- Python oracle `compare store-g1 g1-b` and `compare store-g1 g1-c`: `ok:true`, no drift (33 s).

### 4 Groups

- `relay` g1 to g2 (`--once`): 34.6 s, max RSS 931 MB, 92.75 MiB journal and all 4.8 GB of blobs copied.
- `join` g2-a from store-g2 without an archive (script of another clone, `--worktree`): 33.6 s, max RSS 2.75 GB, 269,321 paths. `verify` ok.
- Reverse direction: the running `relay-g1` logged `relayed g2-a` and g2-a's edits reached g1-a/g1-b (see latencies below).

### 5 Live sync (daemons for g1-a, g1-b, g2-a, relays g1 and g2, poll 250 ms, debounce 150 ms)

(a) 20 edits, 50 ms polling. Shipped code (`scanSeconds` 15, scan blocks the loop):
g1-a to g1-b median 579 ms, p95 4,877 ms; g1-a to g2-a median 6,525 ms, p95 8,344 ms (values alternate between about 0.4 s and 4.6 s or 8.1 s).
Final code:

| Writer to reader | median | p95 | max |
|---|---|---|---|
| g1-a to g1-b | 397 ms | 460 ms | 460 ms |
| g1-a to g2-a (across groups) | 403 ms | 685 ms | 685 ms |
| g2-a to g1-a (across groups) | 403 ms | 706 ms | 706 ms |
| g2-a to g1-b (across groups) | 377 ms | 706 ms | 706 ms |

(b) 2,000 small files created on g2-a: all present with correct bytes on g1-a/g1-b after 3,820 / 3,897 ms (shipped), 2,008 / 1,925 ms (final).
(c) Directory delete on g1-b (2,000 files): gone including the empty directories on g1-a and g2-a after 1,921 / 2,076 ms (shipped), 799 / 859 ms (final).
(d) Same-file race, 3 trials, writes 5 to 8 ms apart on g1-a and g2-a: all three machines converged each time to one value within 0.47 to 0.71 s. The winner differed (A, B, B) because the clock is minted at publish time, not at write time, so writes a few ms apart are ordered by publish timing. The loser's bytes are not in `.cache/f/lost/` (that only receives unpublished local edits) but in the blob store of both groups (`blobs/<2 hex>/<sha256>`), where they stay until `compact --collect-blobs` after the retention.
(e) `node_modules/x/y.js` and `.cache/z` on g1-a: after more than one full scan neither file's content appears anywhere in store-g1/store-g2 (grep of the unique tokens), nor in g1-b/g2-a; a visible file written at the same time arrived.
(f) Rename of a directory with 500 files (10 subdirectories): complete on g1-b/g2-a after 12,223 / 12,340 ms (shipped, waited for the next full scan), 1,262 / 1,540 ms (after defect 7); the old directory was pruned everywhere.
(g) Executable bit on: 274 ms (g1-b), 620 ms (g2-a); off again: 278 / 388 ms.
(h) File with a 250-character relative path (4 components of 61 or 62 symbols U+26xx, 744 UTF-8 bytes, absolute path 997 bytes): visible after 459 ms (g1-b), 344 ms (g2-a). 4-byte emoji of that length would exceed macOS `PATH_MAX` 1024 under this long scratch prefix, so this was not tried.
(i) Offline: g1-b stopped; g1-a made 100 edits, g1-b 5 offline edits, of which 3 were on files only it changed and 2 (`x1` older, `x2` NEWER than the g1-a edit) on files that g1-a also edited. After restart g1-b applied all 102 remote events 0.5 s after start; the 3 own files were published by the first full scan and reached g1-a and g2-a (not timed, within about 25 s); `x1` and `x2` were overwritten by the remote version and the offline bytes are in `.cache/f/lost/<hlc>/rehearsal-offline/x{1,2}.txt` with `overwritten` events. So an offline edit that is newer in wall clock time than a remote event still loses if the remote event was published before the local one: preserved, not published (as the contract's follow step says; open point below).
Offline edits, Round 5 (real tree, two daemons `n1` and `n4` on one store, `--poll 250`): `n4` stopped; `x1` edited on `n4`, then 2 s later on `n1`; `x2` edited on `n1`, then 2 s later on `n4`; `x3` deleted on `n4` and edited on `n1`. After restarting `n4`: `x1` got the `n1` version and `n4`'s older bytes are in `lost/<hlc>/rehearsal-offline/x1.txt` (`overwritten`); `x2` was `kept` on `n4` and published (one `put` by `n4`), `n1` received it; `x3` came back with the `n1` bytes and `n4` published nothing for it. No `error` events.

(j) Store unreachable for 20 s for g1-b (symlink removed and put back): the daemon (same PID) stayed alive, one `store-unreachable` and one `store-reachable` event, 11 edits made on g1-b during the outage and 5 on g1-a all converged on every machine 7.6 s after the store returned; no `error` events anywhere in any log.

### 6 Idle cost (60 s windows, `idle-sample.py`, CPU % of one core from `ps` cputime)

| Case | `run` daemons | relays |
|---|---|---|
| Shipped, `scanSeconds` 15, 3 origins, 3 daemons scanning | 117 %, 159 %, 165 % | 0.5 %, 0.7 % |
| `scanSeconds` 600, 3 origins | 2.1 to 2.3 % | 1.7 %, 2.2 % |
| 50 origins (47 dummy), before poll fixes | 5.7 to 6.1 % | 5.2 %, 11.7 % |
| 50 origins, after poll fixes | 2.6 to 2.8 % (3.2 to 3.4 % in the final run) | 0.8 %, 3.1 to 3.2 % |

- One full scan on the 270k tree (daemon alone, `--scan 30`): 6.6 to 7.1 s wall, 16.6 CPU seconds (mostly `lstat`/`readdir` system time); with three daemons scanning at the same time 8.8 s wall and 23 to 24 CPU seconds each. The NDJSON `scan` event has no duration; the figures come from CPU sampling of the process. At `scanSeconds` 900 this adds 1.8 % of a core.
- RSS idle, rehearsal (before Round 5): 0.81 to 1.06 GB per `run` daemon, relays 45 to 70 MB. Breakdown from a harness: opening the replica 640 MB (JS heap 195 MB, external strings 198 MB; `files`, `manifest`, `lower` hold about 270k long paths each), after one scan 1.2 GB. Peaks of the one-shot commands: seed 2.0 GB, join 2.3 to 2.75 GB, verify 2.0 to 2.3 GB.
- **Round 5 re-measurement** (same tree, one APFS clone under `$SCRATCH/rehearsal`, 269,331 shared files, Bun 1.4.2, load average 2 to 3, `/usr/bin/time -l` and `idle-sample.py`). Before = the code of the rehearsal, measured again the same day; after = Round 5.

| Measure | Before | After |
|---|---|---|
| Idle `run` daemon, RSS (120 s window after the 25 s start scan) | 1,261 MiB (45 s after start) | **238 MiB** (234 to 287 MiB in six runs while the code was being tuned) |
| Idle `run` daemon, CPU | 1.7 % | 1.2 % |
| `run` start-up scan, CPU / RSS max | 20 s CPU | 19.8 s CPU, 238 MiB |
| `seed`: peak RSS / wall | 1,695 MiB / 54.5 s | **480 MiB** / 46.8 s |
| `verify`: peak RSS / wall | 2,184 MiB / 21.9 s | **402 MiB** / 19.6 s |
| `snapshot`: peak RSS / wall | 1,090 MiB / 1.3 s | 464 MiB / 1.8 s |
| `join` into a clone with all files (adopt, from a snapshot): peak RSS / wall | 1,929 MiB / 47.9 s | **479 MiB** / 18.9 s |
| `join` into an empty directory, journal fold, no snapshot: peak RSS / wall | 2.75 GB / 33.6 s (rehearsal) | **490 MiB** / 33.3 s |
| `verify` of that joined tree | not measured | 471 MiB / 18 s, `ok:true` |

All four trees verified `ok:true` with 269,331 files checked. Two daemons on the real tree (origins `n1`, `n4`) also showed the new conflict rule live, see "Offline edits" under 5.

### 7 Compaction

- `compact` (worker, 14 days): 1.47 s, 0 removed, new snapshot (the two newest per origin are kept). `compact --collect-blobs` (server): 1.53 s, 0 removed. `compact --collect-blobs --retention-days 0`: 1.76 s, removed 67 segments and 2,068 blobs (94.34 MiB) while the three daemons ran. `verify` ok on g1-a, g1-b, g2-a afterwards (about 21 s each).
- `join` g1-c from the compacted store without archive: 32.8 s, max RSS 2.75 GB, `verify` ok, path-set/sample check against g1-b and the oracle: no drift.

## Defects found and fixed

All edits were made with a fresh read just before and applied in one step because another agent edits the same files.

1. **`relay` cannot use local peers.** Peers come only from the topology (`/mnt/semio-peers/...`). Added `relay --peers g1=<dir>[,g3=<dir>]` (replaces the topology peers) and the message `error.peers` in EN and DE. Files: `🔁️replica/⌨️command/🟦️.ts`, `🔁️replica/📖️texts/🔣️.json`, `README.md` (command table and text).
2. **Message variables with digits were not substituted** (`sha256` printed as `{sha256}` by `bootstrap-archive`). The substitution regex was `[a-z]+`. Now `[a-z0-9]+`. File: `🔁️replica/⌨️command/🟦️.ts` (`replicaTextV1`).
3. **macOS `tar -xf` writes NFD names.** 109 files with accents (`é`, `ü`) came out decomposed; the scanner refuses them as `not-nfc`, `join` reported them as applied but `verify` showed 109 `missing-local`, and the daemon would have forgotten them. `bootstrap-settle` now renames refused `not-nfc` names back to NFC (`healNamesV1`, repeated per directory level, never overwriting a literal NFC sibling) and reports `renamed-to-nfc`. File: `🔁️replica/🥾️bootstrap/🟦️.ts`. 29 renames were made in the extracted tree.
4. **`bootstrap-settle` moved the subsystem's own changed code aside**, so the following `join` died with `Cannot find module './⌨️command/🟦️.ts'` (the archive is up to an hour old; any code change since then triggers this). Settle now puts the store's version of a moved file back at once (blob copy, mode 0755/0644). File: `🔁️replica/🥾️bootstrap/🟦️.ts`; runbook text in `README.md` step 5.
5. **The periodic full scan blocked the daemon loop** (6.6 to 9 s on 270k files): saves and remote events arriving meanwhile waited, giving latencies of 4.6 s (in group) and 8.1 s (across groups). `publishOnceV1` takes an optional finished scan (4th parameter `walked`); `runV1` now walks the tree in the background and publishes the result from the loop (a walk made under an older `.gitignore` is dropped and a full publish is done instead). Median/p95 in group 579/4,877 ms to 397/460 ms. File: `🔁️replica/🟦️.ts`.
6. **Full-scan CPU.** 16.6 CPU seconds per scan at the default `scanSeconds` 15 is 117 to 165 % of a core per daemon. Default changed to 900 (idle 1.8 % for the scan). Files: `🗺️topology/🔣️.json`, `🗺️topology/🧪️tests/🟦️.ts`, `🐍️.py`, `🧫️fixtures/🔣️.json`, design-contract line (default 900 with the measurement). Not executed: those tests.
7. **A renamed or newly created directory was found only by the next full scan** (12.2 s for 500 files): the watcher event names only the directory and `observe` skipped directories. `rename` watcher events now also queue `<path>/`; `observe` expands a queued existing shared directory to its children, recursively. Now 1.3 to 1.5 s. File: `🔁️replica/🟦️.ts`.
8. **Poll cost with many origins.** (a) `validateJsonSchemaSubset` compiles and snapshots the whole schema on every call (profile: most of the CPU); both validators now compile each definition once (`🔁️journal/🟦️.ts` `check`, `🔁️replica/🟦️.ts` `requireReplicaShapeV1`). (b) `storePathsV1` re-joined six paths per call; memoized. (c) `readHeadV1` re-read and re-validated every head on every poll; it now `stat`s and reuses the parsed head while inode, size and mtime are unchanged (forced re-read after 5 s). (d) The relay re-read `home.json` of every origin on every poll and re-checked home/snapshot of unchanged origins; `homeOf` is cached and an origin whose head is already equal is skipped except for a home/snapshot re-check every 30 s. Effect with 50 origins: `run` 5.7 to 6.1 % to 2.6 to 2.8 %, `relay-g2` 11.7 % to 3.1 %. Files: `📒️journal/🟦️.ts`, `🔁️replica/🟦️.ts`.

## Open problems

- Memory: **closed in Round 5** (idle 238 MiB, peaks 402 to 490 MiB, was 0.8 to 1.06 GB and 2.0 to 2.75 GB; the target was 300 MB and 1 GB). The idle figure varies by about 15 % between runs (234 to 287 MiB observed), so there is no large margin on an 8 GB machine that runs compilers next to the daemon; the first full scan after start still adds about 25 MiB.
- Idle CPU is borderline, not "well under 5 %": 2.6 to 3.4 % polling with 50 origins plus 1.8 % for the scan is 4.4 to 5.2 %. The poll is dominated by 4 `stat`s per second per origin; a coarser poll, a sync `stat`, or one store-level change marker would cut it. The relay of a group with 49 foreign origins adds 3 %.
- A full scan still costs 16.6 CPU seconds (6.6 s wall); a worker-thread sync scanner measured 10.5 CPU seconds in a prototype but was not adopted.
- Offline-newer edit loses to an older already-published remote event: **closed in Round 5**. A dirty local file whose mtime is later than the wall time of the remote event is published instead of overwritten (see "Offline edits" under 5). Still true and documented: an offline edit that is older than the remote publish, and both-machines-offline, are decided by publish time.
- Same-file race winners depend on publish timing, not on write time (writes less than about 100 ms apart on different machines are effectively unordered). Not changed; now stated in the contract and the README.
- Not exercised: relay-built bootstrap archive refresh while live (relays ran without a machine identity to avoid 5 GB rewrites), Linux/Windows behaviour, inotify limits for 122k directories on Linux (the tree has 121,884 directories with files), 4-byte emoji paths near `PATH_MAX`, GNU tar and bsdtar on Windows, NFS attribute caching (stat-signature head cache assumes `rename` changes the inode), clone timing, all unit tests.
- File counts: `find -type f` saw 269,484 regular files in the clone; 269,321 are shared and the other 163 are ignored files (path-set comparison); the 87 refused symlinks are not regular files.
