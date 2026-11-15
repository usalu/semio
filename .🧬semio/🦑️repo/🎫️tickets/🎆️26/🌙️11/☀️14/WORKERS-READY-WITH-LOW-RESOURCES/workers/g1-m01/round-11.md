# g1-m01 Round 11

Rounds 9 and 10 were posted while round 8 was still running its ten tries (21:04Z to 00:56Z); round 8 was not interrupted because it got no exit 4 after 21:54Z and was never waiting for a click after that. No separate round 9 or round 10 run exists for this machine.

## Before The Run (2026-11-15T00:57:30Z)

- Replica agent: `state = running`; kickstarts needed before the run: 0.
- Replica code digest (`.cache/f/health.json`, field `code`): `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc`; `bootVersion` 6.

## Result (written 2026-11-15T01:17:18Z)

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1, free disk 85Gi of 228Gi.
- Start 2026-11-15T00:57:33Z, end 2026-11-15T01:16:35Z. Three tries, all `native_rc=1`; `.cache/f/native-bootstrap.done` absent; **not ready**. The round ended because the same missing-record failure survived a retry and one `repo cache gc --area=graph-inference`.
- No nx task ran in any try: succeeded, failed, not-run task lists = none.
- Memory pressure level 1 in all 13 samples; no hard stop, no stall. Peak resident size of a process of the run: 3 609 952 KiB.
- No fleet refusal, no exit 4, no reference 3420bc16. Replica agent running; 0 kickstarts.
- Launcher unchanged in all 6 records (link inode 101440789 → `versions/cf25f2b751f83336/repo`, file inode 101440788, mtime Nov 14 12:59 local, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`); `round-11.launcher.txt`.
- The whole bootstrap before the graph step now takes about 40 to 50 s (tries of 2 min in total).

## Blocker: Missing Record in `.cache/b/g/m/build-input-closures`

All three tries got through facts, verification, flush and owners (`owners 1 of 1`, the single stale project `workspace (root:language:cargo)` took about 16 s) and failed at or right before `assembly 0 of 703`: further than any try of the earlier rounds.

| try | start (UTC) | end (UTC) | native_rc | graph step | last progress line | reference |
|---|---|---|---|---|---|---|
| 1 | 00:57:33 | 00:59:58 | 1 | 114 s | `assembly 0 of 703` | 742ceab8 |
| 2 | 01:00:32 | 01:02:11 | 1 | 72 s | `assembly 0 of 703` | 742ceab8 |
| 3 (after the gc) | 01:14:34 | 01:16:25 | 1 | 78 s | `owners 1 of 1` | 29bbd28b |

Service log `host-ac6bfe8f337c.log`, tries 1 and 2 (identical; line 15627 and 16788):
```
2026-11-15T00:59:58.863Z request-failed 742ceab8 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/287b569798f6854f56082d3a0cd1b5638507649556b6edc64c75ce8160921a77.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
```
The six service log lines before it in try 1 (`Assembling the graph: 97 of 703 projects`, then `Expanding input closures: 0 of 233 closures`):
```
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:assembly:97:703","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Assembling the graph: 97 of 703 projects
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:233","mode":"shared","elapsedMs":0}}
[script] graph:slice-worker · Expanding input closures: 0 of 233 closures
```
Try 3, after the gc (line 18172; the message is empty now, the stack is the same):
```
2026-11-15T01:16:25.294Z request-failed 29bbd28b Error
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (unknown:7:39)
```
Bootstrap log text (verbatim, try 3):
```
The resident service met an unexpected failure (reference 29bbd28b); try again, the details under that reference are in the service log /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
```
`ls` of the record at report time: No such file or directory. Number of files in `build-input-closures`: 15989.

The round 8 note says a missing `build-input-closures` file now counts as a miss. On this path it does not: `readInventoryRecordLeanV1` (`…/📥️inputs/🪶️lean-inventory/🟨️.mjs:12`) called from `…/🕸️graph/📁️workspace/🟦️.ts:86` still throws. No `closureRepaired` line in the service log.

Retry rule used, stated openly: round 11 does not list this failure. I applied the rule of round 6 for exactly this error (try again; if it survives, `repo cache gc --area=graph-inference` once, then try again) and stopped after the third failure.

## The gc

`repo cache gc --area=graph-inference`, 01:02:33Z to 01:14:34Z (12 min). It did not finish its work; output (`round-11.gc.txt`, progress lines removed):
```
[script] build:source-admission · Admitting the sources of the build
[script] build:implementation-source · Reading the sources of the build implementation
This process (repository:build) made no progress for 10 min and has no live child: it exits now so that the leases it holds are released.
repository:command failed with exit code 124
```
`round-11.samples.txt` says "GC end: exit 0": that is a mistake of my runner (it printed the status of `date`), the real exit status was not captured; the text above says 124.

## macOS Denial and the Fallback (try 1)

Bootstrap log, try 1, line 561 (verbatim): `The graph service under the launcher was denied access; it is started again under the access of the calling app / Der Graphdienst unter dem Launcher wurde abgewiesen; er wird erneut mit dem Zugriff der aufrufenden App gestartet`

Record as asked: **runs under the app's access**. Service log around the denial (lines 14270–14278; the denial is from the service that had run since round 8):
```
2026-11-15T00:58:00.510Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T00:58:00.510Z shutdown access-denied
```
`where: root=… entry=… code=…` line: 0 such lines exist in `host-ac6bfe8f337c.log`. Tries 2 and 3 had no denial.

## Graph Lines (service log, from try 1 of this round on; consecutive repeats removed, prefix = line number)

```
14423:graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 7 s
14424:graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
14902:graph: refreshing 1 projects (generation 75)
14903:graph: 17 stale facts (readdir 15, exists 1, readdirNames 1) leave 1 projects stale
14904:graph: workspace is stale: the workspace file ♻️mit-bestand/bun.lock was changed
14905:graph: workspace is stale: the workspace file ✏️s/bun.lock was changed
14906:graph: workspace is stale: the workspace file 🌎️hub/bun.lock was changed
14907:graph: workspace is stale: the workspace file 🎓️teaching/bun.lock was changed
14908:graph: workspace is stale: the workspace file 🏢️semio-tech/bun.lock was changed
14909:graph: workspace is stale: the entries of the directory /Users/ueli/Documents/semio changed
14910:graph: 16 more stale inputs are not listed
15538:graph: headroom of 1024 MB is below one project (1640 MB), inferring one project at a time
15593:graph: generation 75 published, 1 projects refreshed in 96 s (0 kept their edges)
15594:graph: fact index of generation 75: 758394 facts of 5101 projects, 87.4 MB expanded (17.1 % of the 512 MB limit), 13.8 MB packed
15595:graph: 7 derived results asked again after 17 stale facts: 5 unchanged, 0 changed
15649:graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 7 s
15650:graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
16062:graph: refreshing 1 projects (generation 75)
16063:graph: workspace is stale: the workspace file ♻️mit-bestand/bun.lock was changed
16064:graph: workspace is stale: the workspace file ✏️s/bun.lock was changed
16065:graph: workspace is stale: the workspace file 🌎️hub/bun.lock was changed
16066:graph: workspace is stale: the workspace file 🎓️teaching/bun.lock was changed
16067:graph: workspace is stale: the workspace file 🏢️semio-tech/bun.lock was changed
16694:graph: headroom of 1024 MB is below one project (1640 MB), inferring one project at a time
16754:graph: generation 75 published, 1 projects refreshed in 53 s (0 kept their edges)
16755:graph: fact index of generation 75: 758362 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed
17024:graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
17025:graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
17446:graph: refreshing 1 projects (generation 75)
17447:graph: workspace is stale: the workspace file ♻️mit-bestand/bun.lock was changed
17448:graph: workspace is stale: the workspace file ✏️s/bun.lock was changed
17449:graph: workspace is stale: the workspace file 🌎️hub/bun.lock was changed
17450:graph: workspace is stale: the workspace file 🎓️teaching/bun.lock was changed
17451:graph: workspace is stale: the workspace file 🏢️semio-tech/bun.lock was changed
18080:graph: headroom of 1024 MB is below one project (1640 MB), inferring one project at a time
18139:graph: generation 75 published, 1 projects refreshed in 57 s (0 kept their edges)
18140:graph: fact index of generation 75: 758362 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed
```

## `[capacity]` Lines (verbatim, distinct)

Try 1:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2972 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3637 MB reserved (7193 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 38% of the default from 16 earlier run(s) of its class, at least 4773 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4773 MB reserved (7908 MB available, 2048 MB floor)
[capacity] run: sized at 102% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7200 MB available, 2048 MB floor)
```
Try 2:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2972 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3637 MB reserved (6912 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 29% of the default from 16 earlier run(s) of its class, at least 4774 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4774 MB reserved (6839 MB available, 2048 MB floor)
[capacity] run: sized at 102% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7925 MB available, 2048 MB floor)
```
Try 3:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2974 MB because of its plan phase
[capacity] waiting 0s for run: the host is swapping out too fast (need 2974 MB; 7919 MB spare above the 2048 MB floor, load 3.48; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 8 job(s), 3625 MB reserved (9414 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 29% of the default from 16 earlier run(s) of its class, at least 4777 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4777 MB reserved (9003 MB available, 2048 MB floor)
[capacity] run: sized at 102% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8990 MB available, 2048 MB floor)
```

## Files

`round-11.samples.txt`, `round-11.launcher.txt`, `round-11.gc.txt`, `round-11.tail.txt` (last 200 lines of the try 3 log); logs `~/native-bootstrap-round-11.log`, `~/native-bootstrap-round-11-try-{2,3}.log`.
