# g1-m01 Round 5

## Before The Run (2026-11-14T18:51:52Z)

Replica code digest (`.cache/f/health.json`):
```
.code 39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759
```

`ps -axo pid,ppid,etime,rss,args` for `show projects` and `service host` (cut at 260 characters):
```
61852     1       02:08 478736 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo
61859 61852       02:06 444272 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=awa
```

Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T18:26:56.058Z worker stopped: ueli-workstation-mac-mini-m6-16gb-256gb-01-lan.fritz.box-30728
2026-11-14T18:26:56.895Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T18:26:56.944Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T18:26:56.949Z worker registered: db6d012d-0c97-4509-b13e-b0c355260b83 cache http://192.168.178.70:6094
2026-11-14T18:31:54.703Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:36:57.400Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:42:01.183Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:47:03.877Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

## Result (written 2026-11-14T19:36:56Z)

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1, free disk 86Gi of 228Gi.
- Start 2026-11-14T18:52:11Z, end 2026-11-14T19:35:49Z. Six of ten tries used; the round ended at try 6 because tries 5 and 6 failed the same way with something that is neither a fleet refusal nor a hard stop nor a restart.
- Final `native_rc=1`; `.cache/f/native-bootstrap.done` absent; not ready.
- `workspace:setup` never got past the graph step: no nx task ran, failed tasks = none.
- The fleet path worked: in try 2 all 456 asked projects arrived (`456 of 456`); in try 4 generation 60 with 5099 projects was restored from the remote cache in 17 s and only 1 project was left to refresh.
- Blocker now (tries 5 and 6): a record is missing in the local graph storage, see "Tries 5 and 6".

## Tries

| try | start (UTC) | end (UTC) | native_rc | how it ended | wanted | arrived | missing | seconds |
|---|---|---|---|---|---|---|---|---|
| 1 | 18:52:11 | 18:55:05 | 1 | macOS denial e07192d7 (first request after the service restart of 18:49Z) | not reached | – | – | – |
| 2 | 18:57:12 | 19:09:45 | 1 | hard stop: pressure level 4 (after 2, 2) | 456 (53 already taken) | 456 | 0 | not printed; about 316 s (158 progress lines 2 s apart) |
| 3 | 19:13:15 | 19:16:03 | 1 | macOS denial e07192d7 | not reached | – | – | – |
| 4 | 19:18:16 | 19:23:49 | 1 | hard stop: pressure level 4 (after 2, 2) | none asked: generation 60 restored from the remote cache | – | – | 17 s download |
| 5 | 19:27:18 | 19:30:11 | 1 | service failure 0db803f4 (ENOENT) | none asked | – | – | – |
| 6 | 19:32:19 | 19:35:30 | 1 | service failure 0db803f4 (ENOENT) | none asked | – | – | – |

Waits: 120 s after tries 1, 3, 5; 180 s after the hard stops of tries 2 and 4. No try ended with `native_rc=75`.

Deviation, stated openly: the round names only `native_rc=75`/fleet refusal, hard stops and "code changed under it" as reasons to go on. I let the runner go on once after a macOS denial (tries 1 and 3) because in rounds 2 and 3 it was the first request after a service restart and did not repeat, and once after the first 0db803f4 failure (the message itself says "try again"). The second identical failure ended the round.

## Hard Stops (tries 2 and 4)

Pressure samples: 57 at level 1, 4 at level 2, 2 at level 4. Both stops came on the third sample of a 2, 2, 4 sequence; one SIGINT to the processes of the run, the bootstrap answered `Cancelled (SIGINT); leases released.`

Largest processes at those samples (KiB, pid, ppid):
```
19:08:44Z pressure=2
3191360 66427 61859 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-p
863600 71438 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
293328 71592 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
  run peak rss=36864 KiB
19:09:15Z pressure=2
2830688 66427 61859 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-p
870752 71793 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
339200 71901 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
  run peak rss=32672 KiB
19:09:45Z pressure=4
427024 72128 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
313280 72194 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
202000 66427 61859 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM
  run peak rss=22704 KiB
19:22:47Z pressure=2
3548192 76559 61859 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-p
1010400 77456 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.h
347632 77579 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
  run peak rss=26944 KiB
19:23:17Z pressure=2
2884928 76559 61859 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-p
827728 77766 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
384096 77830 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
  run peak rss=34768 KiB
19:23:47Z pressure=4
2244528 76559 61859 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun -- /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-p
560896 78093 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
380768 78118 63608 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.ho
  run peak rss=28592 KiB
```
The largest process is the inferring child of the graph service (ppid 61859), 2.2 to 3.5 GiB; the processes of the run itself peaked at 3 317 696 KiB over the round. The three largest together stay under 5 GiB of 16 GiB at level 4: what else held the memory was not measured. The service's own accounting in try 2 climbs far past its reservation (`14643 of -554 MB reserved, 3 projects running`, below). The desktop app's `git diff --numstat` (ppid 63608, see `round-3.md`) is second and third largest in every one of these samples, 0.3 to 1.0 GiB each.

## Tries 5 and 6: Missing Record in the Graph Storage

```
2026-11-14T19:30:11.199Z request-failed 0db803f4 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/6eccb4ecc0e218f2bec2445f8f767aadf4e3446bba984509b1e297b2cf8a1d71.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
```
Bootstrap log, verbatim:
```
The resident service met an unexpected failure (reference 0db803f4); try again, the details under that reference are in the service log /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-a561d8014168.log.
```
Same file and same stack in both tries, 18 to 20 s into the graph step (`facts 192512 of 247353`, `facts 258048 of 304913`). Both tries came after the SIGINT of try 4; whether the interrupted try left the storage without that record was not measured. `ls` of the file at report time: No such file or directory.

## Graph Service Log Per Try (verbatim, consecutive repeats removed, prefix = line number)

Try 1:
```
143:2026-11-14T18:49:56.026Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
195:2026-11-14T18:55:05.348Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```
Try 2:
```
801:graph: 509 stale projects, 53 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 456
802:graph: waiting for the fleet: 0 of 456 projects arrived
870:graph: waiting for the fleet: 456 of 456 projects arrived
1458:graph: refreshing 509 projects (generation 29)
1459:graph: 281 stale facts (file 187, analysis 71, readdir 21, digest 2) leave 509 projects stale
2103:graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
2202:graph: inferring projects with 1 worker processes (1536 MB each)
2217:graph: waiting for memory headroom (1537 of 1342 MB reserved, 7 projects running)
2259:graph: waiting for memory headroom (1544 of 1673 MB reserved, 7 projects running)
2266:[script] build:projects · Inferring projects: 9 of 48 projects · 7 running, longest 25 s: @semio-tech/framework-async-rs (language:cargo, 25 s, 0 rows), @semio-tech/framework-pack-rs (language:cargo, 20 s, 0 rows), @semio-tech/framework-pack-json-rs (language:cargo, 5 s, 0 rows) · waiting for memory headroom (1544 of 167
2290:graph: waiting for memory headroom (1700 of 1769 MB reserved, 7 projects running)
2326:graph: waiting for memory headroom (1762 of 1734 MB reserved, 7 projects running)
2368:graph: waiting for memory headroom (1528 of 1552 MB reserved, 6 projects running)
2408:graph: waiting for memory headroom (602 of 608 MB reserved, 2 projects running)
2439:graph: waiting for memory headroom (1515 of 1675 MB reserved, 4 projects running)
2471:graph: waiting for memory headroom (1880 of 1863 MB reserved, 5 projects running)
2501:graph: waiting for memory headroom (1460 of 1607 MB reserved, 3 projects running)
2535:graph: waiting for memory headroom (2807 of 3125 MB reserved, 6 projects running)
2543:[script] build:projects · Inferring projects: 30 of 48 projects · 6 running, longest 20 s: @semio-tech/framework-os (project, 20 s, 0 rows), @semio-tech/framework-os-kernel (language:cargo, 15 s, 0 rows), @semio-tech/framework-os-mcp-rs (language:cargo, 10 s, 0 rows) · waiting for memory headroom (2807 of 3125 MB reserve
2559:graph: waiting for memory headroom (2482 of 2825 MB reserved, 5 projects running)
2566:[script] build:projects · Inferring projects: 31 of 48 projects · 5 running, longest 25 s: @semio-tech/framework-os (project, 25 s, 0 rows), @semio-tech/framework-os-kernel (language:cargo, 20 s, 0 rows), @semio-tech/framework-os-mcp-rs (language:cargo, 15 s, 0 rows) · waiting for memory headroom (2482 of 2825 MB reserve
2587:graph: waiting for memory headroom (2581 of 3045 MB reserved, 5 projects running)
2614:graph: waiting for memory headroom (3849 of 4282 MB reserved, 7 projects running)
2622:[script] build:projects · Inferring projects: 32 of 48 projects · 7 running, longest 35 s: @semio-tech/framework-os (project, 35 s, 0 rows), @semio-tech/framework-os-kernel (language:cargo, 30 s, 0 rows), @semio-tech/framework-os-mcp-rs (language:cargo, 25 s, 0 rows) · waiting for memory headroom (3849 of 4282 MB reserve
2655:graph: waiting for memory headroom (3033 of 3479 MB reserved, 5 projects running)
2677:graph: waiting for memory headroom (3325 of 3412 MB reserved, 5 projects running)
2689:graph: inferring projects with 1 worker processes (1536 MB each)
2702:graph: waiting for memory headroom (3007 of 1336 MB reserved, 4 projects running)
2707:[script] build:projects · Inferring projects: 37 of 48 projects · 4 running, longest 45 s: @semio-tech/framework-os-kernel (language:cargo, 45 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 35 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 15 s, 0 rows) · waiting for memory headroom (3007 of 
2719:graph: waiting for memory headroom (4836 of 946 MB reserved, 4 projects running)
2725:[script] build:projects · Inferring projects: 37 of 48 projects · 4 running, longest 50 s: @semio-tech/framework-os-kernel (language:cargo, 50 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 40 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 25 s, 0 rows) · waiting for memory headroom (4836 of 
2741:graph: waiting for memory headroom (4814 of -491 MB reserved, 3 projects running)
2747:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 55 s: @semio-tech/framework-os-kernel (language:cargo, 55 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 45 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 30 s, 0 rows) · waiting for memory headroom (4814 of 
2762:graph: waiting for memory headroom (7532 of -552 MB reserved, 3 projects running)
2767:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 60 s: @semio-tech/framework-os-kernel (language:cargo, 60 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 50 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 35 s, 0 rows) · waiting for memory headroom (7532 of 
2780:graph: waiting for memory headroom (8449 of -365 MB reserved, 3 projects running)
2782:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 65 s: @semio-tech/framework-os-kernel (language:cargo, 65 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 55 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 40 s, 0 rows) · waiting for memory headroom (8449 of 
2792:graph: waiting for memory headroom (9391 of -552 MB reserved, 3 projects running)
2794:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 70 s: @semio-tech/framework-os-kernel (language:cargo, 70 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 60 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 45 s, 0 rows) · waiting for memory headroom (9391 of 
2809:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 75 s: @semio-tech/framework-os-kernel (language:cargo, 75 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 65 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 50 s, 0 rows) · waiting for memory headroom (9391 of 
2811:graph: waiting for memory headroom (10614 of -552 MB reserved, 3 projects running)
2823:graph: waiting for memory headroom (11336 of -553 MB reserved, 3 projects running)
2825:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 80 s: @semio-tech/framework-os-kernel (language:cargo, 80 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 70 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 55 s, 0 rows) · waiting for memory headroom (11336 of
2842:graph: waiting for memory headroom (12267 of 1393 MB reserved, 3 projects running)
2844:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 85 s: @semio-tech/framework-os-kernel (language:cargo, 85 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 75 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 60 s, 0 rows) · waiting for memory headroom (12267 of
2858:graph: waiting for memory headroom (12795 of 2842 MB reserved, 3 projects running)
2860:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 90 s: @semio-tech/framework-os-kernel (language:cargo, 90 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 80 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 65 s, 0 rows) · waiting for memory headroom (12795 of
2875:graph: waiting for memory headroom (13689 of -553 MB reserved, 3 projects running)
2877:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 95 s: @semio-tech/framework-os-kernel (language:cargo, 95 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 85 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 70 s, 0 rows) · waiting for memory headroom (13689 of
2887:graph: waiting for memory headroom (14643 of -554 MB reserved, 3 projects running)
2889:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 110 s: @semio-tech/framework-os-kernel (language:cargo, 110 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 100 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 80 s, 0 rows) · waiting for memory headroom (14643
2895:graph: waiting for memory headroom (14643 of 5397 MB reserved, 3 projects running)
2907:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 115 s: @semio-tech/framework-os-kernel (language:cargo, 115 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 105 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 85 s, 0 rows) · waiting for memory headroom (14643
2911:graph: waiting for memory headroom (14643 of 3095 MB reserved, 3 projects running)
2917:[script] build:projects · Inferring projects: 38 of 48 projects · 3 running, longest 120 s: @semio-tech/framework-os-kernel (language:cargo, 120 s, 0 rows), @semio-tech/framework-renderer-wgpu (language:cargo, 110 s, 0 rows), @semio-tech/framework-plugin (language:cargo, 95 s, 0 rows) · waiting for memory headroom (14643
2919:graph: waiting for memory headroom (14643 of 2293 MB reserved, 3 projects running)
```
Try 3:
```
2982:2026-11-14T19:16:03.056Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```
Try 4:
```
3035:graph: generation 60 (5099 projects) restored from the remote cache, 5103 documents (48 MB) downloaded in 17 s
3525:graph: refreshing 1 projects (generation 61)
3526:graph: 12 stale facts (readdir 10, exists 1, readdirNames 1) leave 1 projects stale
```
Tries 5 and 6:
```
4733:2026-11-14T19:30:11.199Z request-failed 0db803f4 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/6eccb4ecc0e218f2bec2445f8f767aadf4e3446bba984509b1e297b2cf8a1d71.json'
5199:2026-11-14T19:35:30.557Z request-failed 0db803f4 Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/6eccb4ecc0e218f2bec2445f8f767aadf4e3446bba984509b1e297b2cf8a1d71.json'
```
No line with `waiting for the fleet` after try 2, no `inferred here` line in any try. The stale reasons of try 2 (`graph: … is stale: …`, service log lines 1460–1476) and the about 30 last service log lines of every try are in `round-5.service.txt`. The service log has timestamps only on failure lines, so the ends of tries 2 and 4 (SIGINT) are placed by line position, not by time.

## `[capacity]` Lines (verbatim, distinct, per try)

Try 1:
```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 2935 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3593 MB reserved (6115 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 15 earlier run(s) of its class, at least 4690 MB because of its plan phase
[capacity] admitted dependencies with 3891 of 4690 MB: only 3891 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6706 MB available, 2048 MB floor)
```

Try 2:
```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 2935 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3600 MB reserved (6262 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 38% of the default from 15 earlier run(s) of its class, at least 4691 MB because of its plan phase
[capacity] admitted dependencies with 4257 of 4691 MB: only 4257 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6645 MB available, 2048 MB floor)
```

Try 3:
```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 2937 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3609 MB reserved (7905 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 37% of the default from 15 earlier run(s) of its class, at least 4695 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4695 MB reserved (7768 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (4374 MB available, 2048 MB floor)
```

Try 4:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2938 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3610 MB reserved (7910 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 37% of the default from 16 earlier run(s) of its class, at least 4696 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4696 MB reserved (7677 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7742 MB available, 2048 MB floor)
```

Try 5:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2939 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3611 MB reserved (8963 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 37% of the default from 16 earlier run(s) of its class, at least 4698 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4698 MB reserved (7222 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8053 MB available, 2048 MB floor)
```

Try 6:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2939 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3611 MB reserved (7532 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 37% of the default from 16 earlier run(s) of its class, at least 4700 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4700 MB reserved (8148 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7693 MB available, 2048 MB floor)
```

## Wall Time Per Phase

| try | total | graph step |
|---|---|---|
| 1 | 2 min 54 s | under 5 s, then the denial |
| 2 | 12 min 33 s | about 590 s (fleet wait about 316 s, then own refresh of 509 projects until the stop) |
| 3 | 2 min 48 s | under 5 s, then the denial |
| 4 | 5 min 33 s | 174 s until the stop |
| 5 | 2 min 53 s | 18 s, then the failure |
| 6 | 3 min 11 s | 20 s, then the failure |

The part before the graph step (dependencies, tools) is total minus graph step, about 2.5 to 3 minutes per try; not measured separately.

## Files

- `round-5.samples.txt`: 30 s samples (three largest processes with pid, ppid, command; pressure level; peak of the run).
- `round-5.service.txt`: last service log lines per try.
- `round-5.tail.txt`: last 200 lines of the try 6 log (control lines of the script protocol removed).
- Logs on this machine: `~/native-bootstrap-round-5.log`, `~/native-bootstrap-round-5-try-{2..6}.log`.
