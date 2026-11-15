# g1-m01 Round 12 (pause, read-only)

Written 2026-11-15T01:29:46Z. No bootstrap was run; no try was running when the round arrived (01:29Z). Last `native_rc`: 1 (round 11, try 3).

Note for the reason given in the round: on this machine the last three tries (round 11) did not find about 650 owners stale. Each restored generation 74 with 5100 projects from the remote cache in 7 to 8 s, refreshed 1 project and published generation 75; they then failed on a missing `build-input-closures` record (`round-11.md`).

## 1. Graph Lines of the Last Try (round 11 try 3, 01:14:34Z to 01:16:25Z)

Service log `host-ac6bfe8f337c.log`, lines 17024 to 18172; every line that starts with `graph:` and contains `restored`, `generation`, `stale projects`, `refreshing`, `taken from the fleet` or `too heavy`; verbatim, prefix = line number:
```
17024:graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s
17025:graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
17446:graph: refreshing 1 projects (generation 75)
18139:graph: generation 75 published, 1 projects refreshed in 57 s (0 kept their edges)
18140:graph: fact index of generation 75: 758362 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed
```
All other `graph:` lines of that try:
```
17447:graph: workspace is stale: the workspace file ♻️mit-bestand/bun.lock was changed
17448:graph: workspace is stale: the workspace file ✏️s/bun.lock was changed
17449:graph: workspace is stale: the workspace file 🌎️hub/bun.lock was changed
17450:graph: workspace is stale: the workspace file 🎓️teaching/bun.lock was changed
17451:graph: workspace is stale: the workspace file 🏢️semio-tech/bun.lock was changed
18080:graph: headroom of 1024 MB is below one project (1640 MB), inferring one project at a time
```
How the try ended (line 18172):
```
2026-11-15T01:16:25.294Z request-failed 29bbd28b Error
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
```
For comparison, the same kind of lines of the last try that did run into a large stale set (round 8 try 9, same log):
```
6882:graph: 610 stale projects, 608 taken from the fleet, 1 too heavy for this machine (workspace (about 13896 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 2
6913:graph: No verified change journal cursor is recorded for generation 62; verifying the tree once.
7460:graph: refreshing 610 projects (generation 63)
8482:graph: project @semio-tech/framework-os-kernel (language:cargo) grew by 1793 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
8632:graph: No verified change journal cursor is recorded for generation 62; verifying the tree once.
9178:graph: 610 stale projects, 2 taken from the fleet, 11 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), and 5 more; it can give about 3512 MB per project), it can infer 192 of the 
9248:graph: No verified change journal cursor is recorded for generation 62; verifying the tree once.
9778:graph: refreshing 610 projects (generation 63)
12141:graph: project @semio-tech/framework-replication-rs (language:cargo) grew by 2598 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
12201:graph: project @semio-tech/framework-sqlite-snapshot-rs (language:cargo) grew by 5279 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
12208:graph: project @semio-tech/framework-schema (language:cargo) grew by 4729 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
12387:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
```

## 2. Newest Service Logs and Replica Code

`ls -laT` of the newest three `host-*.log` (local time, UTC-8):
```
-rw-r--r--  1 ueli  staff   124391 Nov 14 14:32:30 2026 /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-4bed1562d519.log
-rw-r--r--  1 ueli  staff   188419 Nov 14 15:32:25 2026 /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-9888631afea4.log
-rw-r--r--@ 1 ueli  staff  2080174 Nov 14 17:26:33 2026 /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-ac6bfe8f337c.log
```
Replica code digest (`.cache/f/health.json`, field `code`): `eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc`

## 3. Machine State Now (01:29:46Z)

- Memory pressure level (`kern.memorystatus_vm_pressure_level`): 1
- Swap (`sysctl vm.swapusage`): total = 4096.00M  used = 2672.00M  free = 1424.00M  (encrypted)
- Memory 16 GiB; free disk 81Gi of 228Gi; macOS 27.2.1.

Five largest processes (`ps -axo rss,pid,ppid,etime,args`, rss in KiB, cut at 230 characters):
```
1744848 50644 63608       00:05 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null
1208176 81255 81244    02:20:31 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Opro
712064 63603 63576 02-01:01:58 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Application Support/Claude --stan
521088 63608 63576 02-01:01:57 /Applications/Claude.app/Contents/Frameworks/Claude Helper.app/Contents/MacOS/Claude Helper --type=utility --utility-sub-type=node.mojom.NodeService --lang=en-US --service-sandbox-type=none --user-da
464304 50665 63608       00:02 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null 
```
`repo service host` processes and `repo show projects` children (`ps -axo pid,ppid,etime,rss,args`; not stopped):
```
(none: no `repo service host` process and no `repo show projects` child is running)
```
The second largest process above (pid 81255, parent 81244 = `bun …/fleet/entry/boot.mjs run`, age 2 h 20 min, 1 208 176 KiB) is the replica daemon (`…/shared-worktree/…/replica/… --boot-child run`), not a graph process. The largest and fifth are the desktop app's `git` (parent 63608, `Claude Helper`); see `round-3.md`.
Graph service jobs:
```
tech.semio.repo.service.1cf729ef0794
	state = not running
	runs = 1
	last exit code = 124
```
