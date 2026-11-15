# g1-m01 Round 8

## Before The Run (2026-11-14T21:04:00Z)

- Replica agent: `state = running`; kickstarts needed before the run: 0.
- Replica code digest (`.cache/f/health.json`, field `code`): `590fdef8b75853d124d6f615cd2612d607146e27aaf2b94715a5d7b7606ad2f1`; `bootVersion` 6.
- One `repo preflight --lang en` was run at 21:04Z to learn its output: exit 0, and it prints no line containing "access" on this machine (only `[capacity]` and progress lines). The consent wait of this round therefore also reads the receipts in `~/Library/Application Support/semio/access/*/` (`state` of launcher, service, client).

## Result (written 2026-11-15T00:57:03Z)

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1, free disk 85Gi of 228Gi.
- Start 2026-11-14T21:04:03Z, end 2026-11-15T00:56:06Z. All ten tries used. Final `native_rc=1` (hard stop of try 10); `.cache/f/native-bootstrap.done` absent; **not ready**.
- No try got past the graph step: no nx task ran; succeeded, failed, not-run task lists = none.
- Replica agent: running before every try; 0 kickstarts.
- Consent: tries 1 and 2 ended with `native_rc=4`. From try 3 on no bootstrap ended with 4.
- Launcher: **unchanged over the whole round**, all 20 records identical (link inode 101440789 → `versions/cf25f2b751f83336/repo`, file inode 101440788, mtime Nov 14 12:59 local, Identifier `tech.semio.repo`, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`). `~/.local/bin/repo` also unchanged (inode 101432264). The new signed launcher was already installed by try 2 of round 7 (20:59Z), not by try 1 of this round. Records: `round-8.launcher.txt`.
- Memory: 320 samples at level 1, 10 at level 2, 10 at level 4. Four hard stops (tries 6, 8, 9, 10), each "level 4 in two samples in a row". Peak resident size of a process of the run: 4 597 456 KiB.

## Tries

| try | start (UTC) | end (UTC) | native_rc | how it ended | stale | taken | wanted | arrived | missing | seconds |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 21:04:03 | 21:06:38 | 4 | consent (dialog waiting) | – | – | – | – | – | – |
| 2 | 21:38:49 | 21:41:38 | 4 | consent (dialog waiting) | – | – | – | – | – | – |
| 3 | 21:54:32 | 22:26:10 | 75 | fleet refusal | 656 | 0 | 656 | 158 | 498 | 1092 |
| 4 | 22:28:38 | 22:56:53 | 75 | fleet refusal | 656 | 158 | 498 | 142 | 356 | 1484 |
| 5 | 22:59:13 | 23:28:28 | 75 | fleet refusal | 656 | 142 | 514 | 361 | 153 | 1457 |
| 6 | 23:30:48 | 23:47:52 | 1 | hard stop, level 4 twice (own refresh of 657, at `owners 47 of 657`, 975 s) | 657 | 361 | not printed | – | – | – |
| 7 | 23:51:22 | 00:09:30 | 75 | fleet refusal | 657 | 285 | 372 | 0 | 372 | 900 |
| 8 | 00:11:55 | 00:17:57 | 1 | hard stop, level 4 twice (own refresh of 657, at `owners 48 of 657`, 333 s) | 657 | 0 | not printed | – | – | – |
| 9 | 00:21:27 | 00:39:32 | 1 | hard stop, level 4 twice (own refresh of 610, at `owners 60 of 610`, 761 s) | 610 | 608, then 2 | 2 | not printed | – | – |
| 10 | 00:43:02 | 00:55:37 | 1 | hard stop, level 4 twice (generation 68 restored, refresh of 1 project, at `owners 1 of 1`, 725 s) | 562 | 561 | 1 | not printed | – | – |

Waits: consent waits after tries 1 and 2 (below); 120 s after each refusal; 180 s after each hard stop.

What the numbers show, without interpretation beyond them:
- "taken from the fleet" does not accumulate: 158 → 142 → 361 → 285 → 0 → 608 → 2 → 561 from try to try.
- Try 10 is the nearest any try came: `generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s`, then `refreshing 1 projects (generation 69)`, and that single project (`workspace`, announced as "too heavy for this machine (workspace (about 13896 MB)…)" five lines earlier) ran 725 s at `owners 1 of 1` until level 4. Round 5 try 4 ended the same way (generation 60 restored, 1 project, level 4 after 174 s).
- In the hard-stopped tries the service stops single projects itself: `graph: project @semio-tech/framework-sqlite-snapshot-rs (language:cargo) grew by 5279 MB, more than the 1756 MB this machine gives one project; its inference is stopped …` (also `framework-os-kernel` 1793 MB, `framework-replication-rs` 2598 MB, `framework-schema` 4729 MB). Level 4 came anyway.
- At the level 4 samples the three largest processes are small (largest 0.5 to 2.2 GiB, together under 3 GiB of 16 GiB). What held the memory was not measured (not visible in `ps` resident sizes of the top three):
```
23:47:21Z pressure=4
733568 92190 92004 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^
449696  1151 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
446272  1149 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
23:47:51Z pressure=4
1537936 92190 92004 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM
423408 81255 81244 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
190960 63603 63576 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer) --type=renderer --user-data-dir=/User
00:17:26Z pressure=4
2205840 13617 92004 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM
143568 16637 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
142832 16657 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
00:17:56Z pressure=4
1460800 13617 92004 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM
476544 16727 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
286224 81255 81244 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
00:39:01Z pressure=4
996640 21569 21481 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^
372704 28722 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
372592 25664 21569 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
00:39:31Z pressure=4
505504 24733 21569 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
311328 26495 21569 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
229312 29090 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
00:52:34Z pressure=4
1578608 30918 30805 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM
379392 35892 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
281344 81255 81244 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
00:54:05Z pressure=4
286416 36655 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.u
213488 81255 81244 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
197728 63608 63576 /Applications/Claude.app/Contents/Frameworks/Claude Helper.app/Contents/MacOS/Claude Helper --type=utility --utility-sub-type=node.mojom.NodeService --
00:55:05Z pressure=4
1706064 30918 30805 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM
359776 81255 81244 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
183568 37352 81255 /Library/Developer/CommandLineTools/usr/bin/git -c core.excludesFile= -c core.attributesFile= -c core.quotepath=false -c core.precomposeunicode=true -c
00:55:35Z pressure=4
1576848 30918 30805 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM
 79984 81255 81244 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-
 76480 63576     1 /Applications/Claude.app/Contents/MacOS/Claude
```
- One more macOS denial inside the round, at the start of try 9 (it did not end the try; a new service answered):
```
6164:2026-11-15T00:21:51.906Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
6168:2026-11-15T00:21:51.906Z shutdown access-denied
```

## Consent

Text of tries 1 and 2 (verbatim, same in both):
```
A macOS dialog waits for your answer: "repo would like to access files in your Documents folder" (/Users/ueli/.local/share/semio/launcher/bin/repo). Click "Allow" in the session of the logged-in user (or switch on Documents Folder for `repo` under System Settings, Privacy & Security, Files & Folders), then run the command again. /Users/ueli/Documents/semio cannot be read before that.
```
- After try 1: 30 checks, one a minute, 21:07:08Z to 21:37:49Z. Every one: `repo preflight --lang en` exit 0, no line containing "access" printed, receipts `launcher=waiting service=ok client=ok`. No consent within 30 minutes; try 2 ran and ended with 4 again.
- After try 2: 12 checks with `launcher=waiting`; at 21:54:32Z the `launcher.json` receipt no longer had state `waiting` (the state read back empty), so the wait ended and try 3 started. Try 3 and all later tries read the Documents folder (no exit 4). Whether the owner clicked "Allow" at that moment was not measured; `tcc-attribution.py --last 4h` at 00:57Z prints `0 rows (of 4 consulted requests in the window)`.
- `repo preflight --lang en` never printed an `access` line on this machine, neither while the dialog waited nor after: the wait of step 3 cannot be decided from that line here. I used the receipts.
- Full record: `round-8.consent.txt`.

## Graph Service Log Lines

`round-8.service.txt`: every line with `waiting for the fleet`, `taken from the fleet`, `inferred here`, `restored`, `closureRepaired`, `refreshing N projects`, `stale facts`, `too heavy`, `graph-wanted`, `request-failed`, `shutdown` of the four service logs of the round (`host-4bed1562d519`, `host-9888631afea4`, `host-ac6bfe8f337c`, and the tail of `host-6f286e53e68a` from round 7), consecutive repeats removed, prefix = line number. No `inferred here` and no `closureRepaired` line exists. Without the progress lines:
```
# host-4bed1562d519.log
912:graph: 656 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-rs (about 6518 MB), @semio-tech/framework-plugin (about 4018 MB); it can give about 3512 MB per project), it can infer 19
1168:graph: nothing to wait for any longer: the fleet was asked for 656 stale projects and delivered 158 within 1092 s; 498 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machi
1169:2026-11-14T22:26:10.680Z graph-wanted-expired {"wanted":"656","arrived":"158","seconds":"1092","missing":"498","local":"192","heavy":", among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB) (this machine can give about 3512 MB per project)"}
1220:2026-11-14T22:32:30.329Z shutdown superseded
# host-9888631afea4.log
661:graph: 656 stale projects, 158 taken from the fleet, 1 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 498
1009:2026-11-14T22:56:53.038Z graph-wanted-expired {"wanted":"498","arrived":"142","seconds":"1484","missing":"356","local":"192","heavy":""}
1558:graph: 656 stale projects, 142 taken from the fleet, 5 too heavy for this machine (@semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-plugin (about 5392 MB), @semio-tech/print (about 3628 MB); it can give about 3512 MB per project), it can infer 1
1900:graph: nothing to wait for any longer: the fleet was asked for 514 stale projects and delivered 361 within 1457 s; 153 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/print (about 3628 MB), which this machine must never infer itself (it can give about 3512 MB
1901:2026-11-14T23:28:28.453Z graph-wanted-expired {"wanted":"514","arrived":"361","seconds":"1457","missing":"153","local":"192","heavy":", among them the too heavy project(s) @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/print (about 3628 MB) (this machine can give about 3512 MB per project)"}
1933:2026-11-14T23:32:25.685Z shutdown superseded
# host-ac6bfe8f337c.log
827:graph: 657 stale projects, 361 taken from the fleet, 5 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), @semio-tech/framework-os-dev (about 21330 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/print (about 3628 MB); it can give about 3512 MB per project), it can infer 192 
1504:graph: refreshing 657 projects (generation 62)
1505:graph: 102 stale facts (analysis 77, file 25) leave 657 projects stale
2150:graph: inferring projects with 1 worker processes (1536 MB each)
3267:graph: 657 stale projects, 285 taken from the fleet, 2 too heavy for this machine (@semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 372
3478:graph: nothing to wait for any longer: the fleet was asked for 372 stale projects and delivered 0 within 900 s; 372 remain, among them the too heavy project(s) @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a
3479:2026-11-15T00:09:30.802Z graph-wanted-expired {"wanted":"372","arrived":"0","seconds":"900","missing":"372","local":"192","heavy":", among them the too heavy project(s) @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this machine can give about 3512 MB per project)"}
4022:graph: 657 stale projects, 0 taken from the fleet, 12 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 6 more; it can give abo
4531:graph: refreshing 657 projects (generation 62)
4532:graph: 102 stale facts (analysis 77, file 25) leave 657 projects stale
6164:2026-11-15T00:21:51.906Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
6168:2026-11-15T00:21:51.906Z shutdown access-denied
6882:graph: 610 stale projects, 608 taken from the fleet, 1 too heavy for this machine (workspace (about 13896 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 2
7460:graph: refreshing 610 projects (generation 63)
7461:graph: 129 stale facts (analysis 103, file 25, readdir 1) leave 610 projects stale
8482:graph: project @semio-tech/framework-os-kernel (language:cargo) grew by 1793 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
9178:graph: 610 stale projects, 2 taken from the fleet, 11 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), and 5 more; it c
9778:graph: refreshing 610 projects (generation 63)
9779:graph: 129 stale facts (analysis 103, file 25, readdir 1) leave 610 projects stale
10426:graph: inferring projects with 1 worker processes (1536 MB each)
12141:graph: project @semio-tech/framework-replication-rs (language:cargo) grew by 2598 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
12201:graph: project @semio-tech/framework-sqlite-snapshot-rs (language:cargo) grew by 5279 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
12208:graph: project @semio-tech/framework-schema (language:cargo) grew by 4729 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
12901:graph: 562 stale projects, 561 taken from the fleet, 1 too heavy for this machine (workspace (about 13896 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 1
12995:graph: generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
13475:graph: refreshing 1 projects (generation 69)
13476:graph: 12 stale facts (readdir 10, exists 1, readdirNames 1) leave 1 projects stale
```

## Refusal Lines of the Bootstrap Logs (verbatim)

```
try 3: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 656 and delivered 158 within 1092 s, 498 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 4: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 498 and delivered 142 within 1484 s, 356 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 5: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 514 and delivered 361 within 1457 s, 153 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/print (about 3628 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 7: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 372 and delivered 0 within 900 s, 372 are still missing, among them the too heavy project(s) @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## `[capacity]` Lines (verbatim, distinct over all ten tries, with count)

```
  29 [capacity] run: sized at 46% of the default from N earlier run(s) of its class
  20 [capacity] run: sized at 5% of the default from N earlier run(s) of its class, at least N MB because of its plan phase
  16 [capacity] dependencies: sized at 38% of the default from N earlier run(s) of its class, at least N MB because of its plan phase
  10 [capacity] admitted run: 8 job(s), N MB reserved (N MB available, 2048 MB floor)
  10 [capacity] admitted run: 1 job(s), N MB reserved (N MB available, 2048 MB floor)
   7 [capacity] admitted dependencies: 1 job(s), N MB reserved (N MB available, 2048 MB floor)
   4 [capacity] dependencies: sized at 33% of the default from N earlier run(s) of its class, at least N MB because of its plan phase
   3 [capacity] admitted dependencies with N of N MB: only N MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
   2 [capacity] waiting Ns for run: not enough free memory (need 2048 MB; N MB spare above the 2048 MB floor, load N; 0 running, 0 ahead); behind the host itself (nothing of ours)
   1 [capacity] waiting Ns for dependencies: not enough free memory (need 4747 MB; N MB spare above the 2048 MB floor, load N; 0 running, 0 ahead); behind the host itself (nothing of ours)
   1 [capacity] run: work has to run, waiting Ns to grow the reservation from 2048 MB to 8190 MB: not enough free memory (1211 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 6165 MB), graph-farm (running, 1 job(s), 0 MB), graph-farm (running, 1 job(s), 0 MB)
   1 [capacity] run: work has to run, waiting Ns to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-71 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 6165 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
   1 [capacity] run: work has to run, waiting Ns to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-481 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 6165 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
   1 [capacity] run: work has to run, waiting Ns to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-403 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3546 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
   1 [capacity] run: work has to run, waiting Ns to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-358 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3880 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
   1 [capacity] run: work has to run, waiting Ns to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-186 MB spare above the 2048 MB floor; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 3880 MB), graph-farm (running, 1 job(s), 1536 MB), graph-farm (running, 1 job(s), 0 MB)
   1 [capacity] run: sized at 102% of the default from N earlier run(s) of its class
   1 [capacity] run: reservation grown from 2048 MB to 4446 MB with 3 of 8 job(s) after 30s: the host cannot carry more right now (N MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
   1 [capacity] run: grown from 3 to 4 of 8 job(s) after 885s, reservation 4446 MB -> 4446 MB: memory freed up and nothing of ours waits (N MB available, 2048 MB floor); work that starts from now on uses the new jobs
```
(Numbers that differ from try to try are replaced by N here; the exact lines are in the logs.)

## Wall Time Per Phase

Total per try = end minus start in the table. Graph step: try 3 1730 s, try 4 1523 s, try 5 1492 s, try 6 975 s, try 7 1063 s, try 8 333 s, try 9 761 s, try 10 725 s; tries 1 and 2 did not reach it. The part before the graph step is about 2.5 to 3 minutes per try (total minus graph step; not measured separately).

## Files

`round-8.samples.txt`, `round-8.launcher.txt`, `round-8.consent.txt`, `round-8.service.txt`, `round-8.tail.txt` (last 200 lines of the try 10 log); logs `~/native-bootstrap-round-8.log`, `~/native-bootstrap-round-8-try-{2..10}.log`.
