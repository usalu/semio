# Package AC: what a graph read costs after a landing, measured before and bounded (2026-11-14 10:45 to 11:50, live tree g1-server, 32 GB, load 4 to 36)

## Verdicts (against: unchanged read under 10 s, stale read of 3 projects under 30 s)
| read | before (measured) | after my landing | target |
|---|---|---|---|
| (1) nothing changed | **24.6 s** (b2, 10:50, generation 86, load 5 to 8; phases below) | **NOT MEASURABLE**: since 11:20 the tree is stale in 4 853 projects (edits of `graph-state/📥️replica`, `cursor` and about 30 new test files of another package), every read is a 97-wave rebuild; machine load 20 to 36 | < 10 s: **NOT MET**. Phase effect measured: classification 6.4 s -> 2.4 s (live, same index); stamp write 1.9 s and the 1.6 s row walk of `settled` removed (offline numbers); expected 24.6 s -> about 17 s; the floor without a resident service is about 14 s (section 3) |
| (2) stale by one project after a peer edit | **126 s** (b3, 1 project, load 7) and 66 s of epoch before the first project in b4 | **NOT MEASURABLE** (same reason) | < 30 s: **NOT MET**. Cut by landing: pass 2 classification -4 s, patch -2 s, upload -7 s. Bound: custody wait 26 s, second pass 15 s, closure verification 15 s, tail 29 s (section 2) |
| parent footprint | 1.85 GB unchanged (AB), 2.4 GB stale | not re-measured | < 3 GB: as AB |
| queueing | b6: a 1.2 GB read waited 59 s, c1: a 2.1 GB read waited 60 s ("not enough free memory ... -4 600 MB spare") behind rebuilds of 8 to 17 GB | unchanged | the read is already admitted by its own phases (assembly 1.2 GB, plan), the wait is host memory pressure caused by rebuilds, not the class of the read (section 4) |
Honest state: the targets are not met and could not be measured after the landing, because the tree went stale in almost all projects while I worked (hot edits by other packages) and my own measurement loop made it worse (below). What I can show: where the 24.6 s and the 126 s go to the second, which parts the landing removed, and what bounds the rest.

## 1. The unchanged read, 24.6 s (b2, `SEMIO_REPO_PROFILE=1`, timestamps from `🧰️scripts/AC/ts.py`)
| s | what |
|---|---|
| 0 to 1.6 | outer CLI, capacity (admitted at once, 1.2 GB), source capture |
| 1.6 to 3.6 | implementation source, dependency verification (1.3 s), store opens |
| 3.6 to 10.6 | **7 s with no output**: manifest cursor (`since`: trust check, settle wait 1.5 s, read of the 125 MB checkpoint + 13 MB WAL 1.0 s, snapshot and hash 0.15 s, snapshot store, `.gitignore` compile; measured port `cursor`+`since` 3.4 s at idle) and the load of the generation (index open 0.4 s, manifest 0.02 s) |
| 10.6 to 11.2 | membership (frontier stat) 0.65 s |
| 11.2 to 18.7 | classification of 678 427 facts (`[graph-phase] doubt` 6 389 ms: 3.4 s the path verdict of 367 000 paths, 2.4 s stat of 156 000 directories, rest row access) |
| 18.7 to 20.9 | precise check of 91 875 doubtful rows (closure 54 171 skipped by the host, resolve 25 215, derived 6 881, rest) 2.3 s |
| 20.9 to 24.1 | `epoch.settled` (1.6 s row walk to decide which stamps to keep + 1.9 s rewrite of the 35 MB stamp file), generation verified |
Not on the path (checked): GC (a forced `Bun.gc(true)` on a 1.1 GB heap costs 137 ms, then 10 ms), brotli (`pack` of 76 MB = 0.8 s), `git ls-files --cached --ignored` seen in `ps` belongs to the fleet boot sequence (`repo run`, `gitGuardsV1`), not to the read.

## 2. The stale read, 126 s for one project (b3, load 7)
| s | what | status |
|---|---|---|
| 0 to 13 | boot, admission | as above |
| 13 to 22 | pass 1: membership, doubt 6.4 s, facts 2.3 s | landing: doubt -4 s |
| 22 to 39 | verification of 1 input closure: **15 s** (the first one loads the whole inference library; the second closure in pass 2 took 3.3 s) | unchanged |
| 39 to 65 | **26 s with no output**: capacity upgrade to the rebuild class, builder lease (custody), prime. During it a peer edit arrived, so the plan of pass 1 could not be reused | now visible as `graph.host.capacity`, `graph.host.custody`, mark `graph.host.custody-held` |
| 65 to 80 | **pass 2**, a full second pass (membership 0.5 s, doubt, facts, closures 3.3 s) because `planUnchanged` needs the cursor to be equal | landing: -4 s; not removed |
| 80 to 88 | collection stages and inference of 1 project 8 s | unchanged |
| 88 to 117 | **tail 29 s** between the last project and the published generation (patch, orphan scan, pack, publish, doubt stamps, retire) | the patch is 5.5 s offline for a 109 000 row owner (-2.3 s landed); the rest is not located: now visible as `graph.build.{fragments,patch,pack,keep,publish,indexed,stamps,retire}` |
| 117 to 126 | upload 8.7 s | landing: -7 s under load (cold 10.9 s -> warm 3.5 s offline, same rows) |
b4, 4 minutes later, started with `the replica manifest cannot vouch for the tree (the replica daemon is not healthy)` (the daemon restarted at 11:03) and therefore with the epoch: **66 s** of full verification before the first project, although b3 had published generation 87 with a cursor. A daemon restart costs every reader one full epoch; that is the health tier package's area.
Hot files: all seven files I edited are held by 389 projects and 2 810 closure answers (`🧰️scripts/U/owners-of.ts`). An edit of a file that is already changed since the last generation costs nothing more, a first edit or a new file in the tool closure stales thousands. I landed at 11:41 when the tree was already stale in 4 853 projects for that reason, so my landing added about nothing; after the rebuild finishes the next landing in these files costs again. Land such changes in one batch.

## 3. What was built (landing 1, `landings.md` "Package AC landing 1")
1. **Remembered path verdicts** (`📍️cursor/🧠️verdicts`, schema `VerdictPolicy`/`VerdictsHeader`): the verdict of a path that no snapshot names (ignore class, boundary rules, roster) was recomputed for 367 000 to 482 000 strings in every pass (7.9 s offline, 3.4 s in the pass); it is read from a brotli file keyed by the text of `.gitignore` and a digest of the code that judges (5 files); a path that the snapshot names is never excluded whatever the memory says. Offline 7 849 ms -> 478 ms for all strings of the index; live doubt phase 6 389 -> 2 372 ms. Tests (3 new, cursor 23): memory of one process read by the next and equal to a port without memory over 3 000 random paths plus every snapshot path; a doctored entry is believed (proves the read), a rule edit or another code digest gives an empty memory; damaged, foreign, oversized files are empty memories; no rewrite below 256 new entries; old keys pruned.
2. **Stamp table written only when a stamp changed** (35 MB JSON, 1.9 s per pass). Test red on the old code (mtime moves), green now; plus a missing file is written.
3. **`settled` reads only the unverified and stale rows** (was a walk over all 678 000 rows, 1.6 s). Test: the forgotten directories equal the old full walk over 40 random row/verified/stale sets.
4. **Binary search of the stream patch remembers decoded strings**: 5.5 s -> 3.2 s for a 109 000 row owner, identical output bytes (sha256 equal), stream-patch 4 and patch-drift 2 pass.
5. **Upload moves only new documents**: `neutralizeGenerationFromMemoryV1` + memory `<store>.neutral.json` keyed by root, compression, runtime and code. A remembered document is not read or rewritten; one the cache lacks is rewritten and compared with the remembered hash before any byte is sent (stale memory throws, is deleted, the full path runs). Offline on generation 90: 5 104 documents, 6 moved, rows equal the full transform; tests: equality with the full transform for cold and warm, doctored memory detected and a third root restores the result (remote-exchange 13).
6. **Spans** in the profile table (`SEMIO_REPO_PROFILE=1`, zero cost off): `graph.plan.{index,doubt,facts,settle}`, `graph.build.{prime,membership,plan,compute,fragments,patch,pack,keep,publish,indexed,stamps,retire}`, `graph.refresh.{membership,plan}`, `graph.host.{membership,stamps,capacity,custody,custody-held}`, `graph.upload.{read,transform,missing,write}`. They answer the open questions of section 2 on the next completed stale read (the table is printed at exit; a killed read prints none).
Tests live after landing: cursor 23, doubt 8, stream-patch 4, patch-drift 2, stamp-soundness 16, generations 30, remote-exchange 13, plan-once 8, load 92, graph-hooks 48, file-rows 9: all pass. Real command through the changed path: `repo show projects` (c1, load 20) printed `doubt` 2 372 ms, `facts` 3 311 ms, ran its first pass and was stopped by me.

## 4. Decisions
- **Resident service or summary for the unchanged read**: neither. The cursor already is the summary; what is left is not classification (now 2.4 s, of which 2.4 s is the stat of 156 000 directories the journal cannot see) but fixed costs: boot 3.6 s, cursor 3.4 s (two manifest reads and the 1.5 s settle), membership 0.65 s, stamps 2.4 s, 91 000 doubtful rows 2.3 s. A summary keyed by the cursor would have to skip the unseen rows (ignored and generated directories, outside the workspace), i.e. trust them for a window; that is a relaxation of what the design checks on every read and I did not take it. A resident answer needs the stamps and the unseen rows kept hot and a watcher for them (`repo service host` does not watch them): about a week, not built, and it only reaches about 8 s.
- **Reads behind rebuild grants**: the host already sizes a read by phase (assembly 1.2 GB, plan) and upgrades it to the rebuild class only when a plan finds stale projects (`upgradeBuildCapacityBeforeRebuildV1`); the waits I saw (59 s, 60 s) are `not enough free memory`, spare -2 to -5 GB above the 4 GB floor, with rebuilds of 8 to 17 GB running: a memory pressure wait, not a class mistake. Lending (package V) is only for workers of a holder.

## 5. What bounds the rest and the next cuts (measured, not built)
1. **Second pass** (15 s): make the plan delta-based when the cursor moved during the wait. Rows touched by `since(old cursor)`: ids whose path string is in the changed set (binary search of the sorted string table, both path spellings), all `resolve` rows and everything when the set holds a directory or more than 200 paths; verdicts of untouched rows are reused from pass 1 (same trust as `planUnchanged`). Needs the verdict array of pass 1 kept in the hint. Test oracle: equal stale set to a full pass over random edit sequences (the cursor property test of package S).
2. **Closure verification 15 s** is the library load: preload `libraryBootstrap` while the capacity wait runs.
3. **Custody/capacity wait 26 s**: take the spans of the next stale read first; if it is the builder lease, the wait is behind a peer builder and a read should restore the peer's generation instead of waiting (`remote-exchange` has it).
4. **Tail 29 s**: read `graph.build.*`; the patch itself is 3.2 s offline.
5. **Per-wave fixed stages** (collection 8 s) are not skipped: they have no input digest to compare; needs one.
6. **Persisted unseen-row table** (rows whose static level is not 0) keyed by (index digest, snapshot instance) and updated by the rows of `since`: removes the 678 000 row loop (about 1.3 s) but not the stat of 156 000 directories.
7. **Manifest read per call** (1.0 s + 1.5 s settle, two calls on a stale read): replica module, not touched.

## 6. Not verified
Any read after the landing against the targets (tree stale in 4 853 projects, load above 30); the effect of landings 3 to 5 on a real publish and upload (offline and test numbers only); the persisted stamp skip and `settled` on the live tree; the verdict memory across two real processes (offline: yes, 7.8 s -> 0.5 s); Windows and Linux; a 16 GB machine; a peer-edit read after the landing.

## 7. Incidents and what others must know
- I started three reads in a loop for a baseline (`m/base1..3`) and only stopped the first; the second ran a rebuild of the stale tree for 20 minutes (generation 90 -> 96, holding 15 GB, load up to 36) until I stopped it at 11:47 (progress per wave is kept). It starved my own post-landing read for 60 s and probably other agents' builds. `ts.py` loops must be killed with their shell.
- Landing lock: held by package AD from 11:01 to 11:35 (over the 20 minutes of the rule); taken by me at 11:40:54, released 11:46.
- Scripts: `🧰️scripts/AC/` (`land1.py` anchored landing, `edit_*.py`, `ts.py` timestamped stderr, `tests1.sh`, `cpuprof.py`, `memo.ts`, `settled.ts`). The tree stays stale until the rebuild is finished by someone; the first reader afterwards gets the unchanged-read numbers of section 1 minus the landing.
