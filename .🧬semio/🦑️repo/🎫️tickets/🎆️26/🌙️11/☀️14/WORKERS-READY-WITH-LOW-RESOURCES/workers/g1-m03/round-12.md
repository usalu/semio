# g1-m03 Round 12 (pause, read-only)

Written 2026-11-15T01:35:08Z. No bootstrap was run in this round. Try 4 of round 11 was running when the round was posted; it got one SIGINT at 01:33:39Z and ended 01:33:54Z with `native_rc=1` (report: `round-11.md`). Last `native_rc`: 1 (interrupt). In that last try the graph was current (no stale projects printed) and the run stood in the setup task `workspace:deps-build-tools` on `Another process holds the dependency store`, not in the graph read.

## 1. Graph service log of the last try

The graph service of tries 1 to 4 of round 11 (and of round 10) is one process with one log, `$TMPDIR/semio-repo/501/host-ac6bfe8f337c.log`. Every line that starts with `graph:` and contains `restored`, `generation`, `stale projects`, `refreshing`, `taken from the fleet` or `too heavy`, verbatim, with its line number, in log order (the whole file, so rounds 10 and 11):

```
143:graph: No verified change journal cursor is recorded for generation 59; verifying the tree once.
899:graph: 659 stale projects, 0 taken from the fleet, 7 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 1 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 659
1058:graph: No verified change journal cursor is recorded for generation 59; verifying the tree once.
1611:graph: refreshing 659 projects (generation 60)
6317:graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
6835:graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
7332:graph: refreshing 459 projects (generation 62)
12150:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
12931:graph: 259 stale projects, 0 taken from the fleet, 11 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), and 5 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 259
13141:graph: nothing to wait for any longer: the fleet was asked for 259 stale projects and delivered 153 within 901 s; 106 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 4250 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
13171:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
13757:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
14282:graph: refreshing 259 projects (generation 64)
15289:graph: project @semio-tech/framework-process (project) grew by 1806 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
15334:graph: project @semio-tech/framework-os (project) grew by 2712 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
16767:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
17310:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
17832:graph: refreshing 259 projects (generation 64)
18834:graph: project @semio-tech/framework-renderer-wgpu (language:cargo) grew by 2092 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
19115:graph: project @semio-tech/repo-lib (project) grew by 2051 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
19272:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
19801:graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
20329:graph: refreshing 259 projects (generation 64)
25587:graph: generation 67 published, 260 projects refreshed in 1006 s (0 kept their edges)
25588:graph: fact index of generation 67: 723153 facts of 5101 projects, 92.3 MB expanded (18.0 % of the 512 MB limit), 12.8 MB packed
25815:graph: generation 67 (5100 projects) uploaded to the remote cache, 5104 documents (24 MB) in 8 s
26027:graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 9 s
26028:graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
```

Lines of the same kind written after the start of the last try (try 4, 01:13:40Z):

```
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 9 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
(end; 2 lines starting with graph: after the upload of generation 67)
```

## 2. Newest three `host-*.log` files and the replica code digest

```
-rw-r--r--  1 ueli  staff  3296861 Nov 15 02:35 /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log
-rw-r--r--  1 ueli  staff   230144 Nov 14 23:57 /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-9888631afea4.log
-rw-r--r--  1 ueli  staff   128593 Nov 14 21:39 /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6f286e53e68a.log
```

Replica code digest (`.cache/f/health.json`): `52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f`, store ok, daemon pid 77232 , file age 1 s.

## 3. Machine state now (2026-11-15T01:35:08Z)

- Memory pressure level (`kern.memorystatus_vm_pressure_level`): 1
- Swap (`vm.swapusage`): total = 3072.00M  used = 2480.19M  free = 591.81M  (encrypted)
- Free disk: 82Gi of 228Gi
- The five largest processes (`ps -axo rss,pid,ppid,etime,args | sort -nr | head -5`; rss in KiB, arguments shortened):

```
2356688 79039 47709       00:28 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null -c filter.lfs.clean= -c filte
1096272 77232 77231       04:37 bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M-$M^]M-oM-8M^Oshared-worktree/M-pM^_M^TM^AM-oM-8M^Oreplica/M-pM^_M^[M^_M-
856800 47698 47677 02-02:03:31 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Application Support/Claude --standard-schemes=cowork-artifact,c
465296 79290 47709       00:03 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null -c filter.lfs.clean= -c filter
367472 50400 50399 02-01:19:07 /Users/ueli/Library/Application Support/Claude/claude-code/2.1.321/ddbdf2b334e1/claude.app/Contents/MacOS/claude --output-format stream-json --verbose --input-format stream-json --effort=medium --model=claude-fable-5-2 --permissi
```

- `repo service host` processes and `repo show projects` children still running (not stopped):

```
41996     1    01:04:30  61840 bun <worktree>/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M^SM^ZM-oM-8M^Olibrary/M-pM^_M^OM^WM-oM-8M^Obuild/M-pM^_M^ZM-*M-oM-8M^Odoor/M-pM^_M^SM^\M-oM-8M^Oscript.ts repo service host run
42029 41996    01:04:26 187264 bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('node:module');\012const hash=value=>crypto.createHash('sha256').update(value).digest('hex'),check=()=>{const wire=process.env.SEMI
(2 process(es))
child: 42029 41996    01:04:26 187264 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012con
```

Graph service job: 41996 0 tech.semio.repo.service.19c193692817

Now waiting for round 13 (check every 60 s, at most 6 hours).
