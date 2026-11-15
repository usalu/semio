# Round 15 on g1-m07 (round 14 was never seen here; files are named `round-15`)

## Machine
- Mac mini, 16 GiB memory, macOS 27.2.1, 102 GiB free on the data volume.
- Replica: `pending: null` and code digest `97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea` at 03:02Z before try 1; now "code":"97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea". Agent running, kickstarted 0 times.
- Launcher unchanged before and after both tries: target inode 3496898, mtime 2026-11-14T22:55:52 (CET), CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`, Identifier `tech.semio.repo`.

## Tries (2 of 12; stopped on `native_rc=4` as the round says)
| try | start | end | native_rc | peak resident of the run (30 s sampler) | pressure samples 1/2/4 |
|---|---|---|---|---|---|
| 1 | 2026-11-15T03:02:22Z | 2026-11-15T03:21:44Z | 75 | 5178160 KiB | 39/0/0 |
| 2 | 2026-11-15T03:25:11Z | 2026-11-15T03:37:55Z | 4 | 3153296 KiB | 26/0/0 |

No hard stop (level 2 recorded only, as round 15 says), no stall, no dependency store wait. Both failed in `workspace:setup` before any setup task ran: succeeded 0, failed 0, not run: all.

## Final client lines, verbatim
```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6586 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
Reading the project graph slice (0 s): the code on this machine changed under the slice worker (it ended with exit code 75), so it is started again with the current code, attempt 1 of 3.
The graph service under the launcher was denied access; it is started again under the access of the calling app / Der Graphdienst unter dem Launcher wurde abgewiesen; er wird erneut mit dem Zugriff der aufrufenden App gestartet
macOS denied the resident service access to your Documents folder while it answered a request (reference da146983). The service has stopped itself; it runs with the file access consent of the process that started it and not of `repo`. Switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-c5f530f58f3a.log.
```

## "consent" (try 2, `native_rc=4`)
- The service under the launcher was denied, was started again under the access of the calling app, and that one was denied as well while answering (reference da146983). The calling process chain is: Claude desktop app -> `claude` -> zsh -> bash (`drive-round.sh`, `run-round.sh`) -> `native-bootstrap.sh`.
- Receipts now:
```
{"version":1,"role":"client","pid":15245,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794713158512,"launcherSha256":"cf25f2b751f833367b632787eddfd0a7179ead0122c51f8f4f65fd355e66a482"}
{"version":1,"role":"service","pid":24368,"state":"denied","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794713875588,"launcherSha256":"cf25f2b751f833367b632787eddfd0a7179ead0122c51f8f4f65fd355e66a482"}
```
- `exit.log`, last 4 lines (cut at 300 characters):
```
{"event":"exit","at":1794713229796,"pid":82198,"rssMegabytes":236,"heapMegabytes":58,"workers":[{"index":2,"heapMb":1879.6,"extraMb":1186.3,"objectCount":8363519,"objectTypes":[{"type":"string","count":3785819},{"type":"Array","count":1340268},{"type":"Object","count":1280629},{"type":"Set","count":
{"event":"start","at":1794713729203,"pid":24332,"placed":false}
{"event":"exit","at":1794713729237,"pid":15306,"rssMegabytes":435,"heapMegabytes":56,"workers":[{"index":2,"heapMb":11.2,"extraMb":6.1,"objectCount":92274,"objectTypes":[{"type":"Object","count":175080},{"type":"string","count":60052},{"type":"Array","count":33577},{"type":"Function","count":21401},
{"event":"exit","at":1794713875585,"pid":24368,"rssMegabytes":445,"heapMegabytes":56,"workers":[{"index":2,"heapMb":18.1,"extraMb":8.7,"objectCount":151475,"objectTypes":[{"type":"Object","count":190038},{"type":"string","count":174723},{"type":"Array","count":46540},{"type":"Function","count":34309
```

## P12 sampler per try (whole files `round-15[-try-2].p12-samples.txt` beside this report; `sample` file of try 1: `round-15.p12-samples.txt.sample-2026-11-15T03:08:59Z.txt`)
### try 1
- peak sample: `2026-11-15T03:13:20Z pid=93324 rss=5089MB footprint=[bun [93324]: 64-bit    Footprint: 1483 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1258 of 1051 MB reserved, 2 projects running)]`
- closure entries before: 42014; closure entries after: 42066
### try 2
- peak sample: `2026-11-15T03:37:47Z pid=24481 rss=3079MB footprint=[bun [24481]: 64-bit    Footprint: 242 MB (16384 bytes per page)] last=[graph: No verified change journal cursor is recorded for generation 219; verifying the tree once.]`
- closure entries before: 42066; closure entries after: 42066

## Service logs of this round (EPERM and `where:` lines follow in the next section), verbatim (restore line with A and S, plan stages, fleet, too heavy, closure-missing)
### host-79a74804a430.log
```
graph: plan stage inference starts (managed 189 MB, resident 990 MB)
graph: 0 projects taken from the fleet, 50 inferred here
graph: 0 projects taken from the fleet, 100 inferred here
graph: 0 projects taken from the fleet, 150 inferred here
2026-11-15T03:01:07.838Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":6570,"limit":6553,"worker":{"heapMb":856.4,"extraMb":502.2,"objectCount":3762503,"objectTypes":[{"type":"string","count":1820849},{"type":"Object","count":771602},{"type":"Array","count":395717},{"type":"WeakRef","count":146127},{"type":"Structure","count":84589},{"type":"JSLexicalEnvironment","count":59412},{"type":"Function","count":55801},{"type":"Promise","count":34142}],"at":1794711667117}}
graph: plan stage inference took 486.8 s (managed 1356 MB, 1167 MB more than at its start, resident 1517 MB)
graph: generation 216 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 12 s, from a state 55 min old with 0 projects stale
graph: refreshing 657 projects (generation 217)
graph: plan stage inference starts (managed 1119 MB, resident 3604 MB)
graph: plan stage inference took 51.9 s (managed 1636 MB, 517 MB more than at its start, resident 3675 MB)
graph: plan stage environment starts (managed 1636 MB, resident 3675 MB)
graph: plan stage environment took 4 s (managed 1685 MB, 49 MB more than at its start, resident 3679 MB)
graph: plan stage tests starts (managed 1685 MB, resident 3679 MB)
graph: plan stage tests took 0 s (managed 1685 MB, 0 MB more than at its start, resident 3679 MB)
graph: plan stage policies starts (managed 1685 MB, resident 3679 MB)
graph: plan stage policies took 0 s (managed 1685 MB, 0 MB more than at its start, resident 3679 MB)
graph: plan stage dangling starts (managed 1685 MB, resident 3679 MB)
graph: plan stage dangling took 0 s (managed 1685 MB, 0 MB more than at its start, resident 3679 MB)
graph: plan stage files starts (managed 1685 MB, resident 3679 MB)
graph: plan stage files took 0 s (managed 1685 MB, 0 MB more than at its start, resident 3679 MB)
graph: plan stage test-edges starts (managed 1685 MB, resident 3679 MB)
graph: plan stage test-edges took 0 s (managed 1685 MB, 0 MB more than at its start, resident 3679 MB)
graph: plan stage edges starts (managed 1685 MB, resident 3679 MB)
graph: plan stage edges took 0.7 s (managed 1728 MB, 43 MB more than at its start, resident 3680 MB)
graph: plan stage metadata starts (managed 1728 MB, resident 3680 MB)
graph: plan stage metadata took 0 s (managed 1728 MB, 0 MB more than at its start, resident 3680 MB)
graph: plan stage result starts (managed 1728 MB, resident 3680 MB)
graph: plan stage result took 0 s (managed 1728 MB, 0 MB more than at its start, resident 3680 MB)
graph: plan stage inference starts (managed 1899 MB, resident 4716 MB)
graph: plan stage inference took 515.2 s (managed 2813 MB, 914 MB more than at its start, resident 4083 MB)
graph: plan stage environment starts (managed 2813 MB, resident 4083 MB)
graph: plan stage environment took 2.3 s (managed 2850 MB, 37 MB more than at its start, resident 4090 MB)
graph: plan stage tests starts (managed 2850 MB, resident 4090 MB)
graph: plan stage tests took 0 s (managed 2850 MB, 0 MB more than at its start, resident 4090 MB)
graph: plan stage policies starts (managed 2850 MB, resident 4090 MB)
graph: plan stage policies took 0 s (managed 2850 MB, 0 MB more than at its start, resident 4090 MB)
graph: plan stage dangling starts (managed 2850 MB, resident 4090 MB)
graph: plan stage dangling took 0 s (managed 2850 MB, 0 MB more than at its start, resident 4090 MB)
graph: plan stage files starts (managed 2850 MB, resident 4090 MB)
graph: plan stage files took 0 s (managed 2850 MB, 0 MB more than at its start, resident 4090 MB)
graph: plan stage test-edges starts (managed 2850 MB, resident 4090 MB)
graph: plan stage test-edges took 0 s (managed 2850 MB, 0 MB more than at its start, resident 4090 MB)
graph: plan stage edges starts (managed 2850 MB, resident 4090 MB)
graph: plan stage edges took 2.3 s (managed 2806 MB, -44 MB more than at its start, resident 4090 MB)
graph: plan stage metadata starts (managed 2806 MB, resident 4090 MB)
graph: plan stage metadata took 0 s (managed 2806 MB, 0 MB more than at its start, resident 4092 MB)
graph: plan stage result starts (managed 2806 MB, resident 4092 MB)
graph: plan stage result took 0 s (managed 2806 MB, 0 MB more than at its start, resident 4092 MB)
graph: plan stage inference starts (managed 2820 MB, resident 4440 MB)
graph: plan stage inference took 34.5 s (managed 2489 MB, -331 MB more than at its start, resident 4108 MB)
graph: plan stage environment starts (managed 2489 MB, resident 4108 MB)
graph: plan stage environment took 2.6 s (managed 2512 MB, 23 MB more than at its start, resident 4146 MB)
graph: plan stage tests starts (managed 2512 MB, resident 4146 MB)
graph: plan stage tests took 0 s (managed 2512 MB, 0 MB more than at its start, resident 4146 MB)
graph: plan stage policies starts (managed 2512 MB, resident 4146 MB)
graph: plan stage policies took 0 s (managed 2512 MB, 0 MB more than at its start, resident 4146 MB)
graph: plan stage dangling starts (managed 2512 MB, resident 4146 MB)
graph: plan stage dangling took 0 s (managed 2512 MB, 0 MB more than at its start, resident 4146 MB)
graph: plan stage files starts (managed 2512 MB, resident 4146 MB)
graph: plan stage files took 0 s (managed 2512 MB, 0 MB more than at its start, resident 4146 MB)
graph: plan stage test-edges starts (managed 2512 MB, resident 4146 MB)
graph: plan stage test-edges took 0 s (managed 2512 MB, 0 MB more than at its start, resident 4146 MB)
graph: plan stage edges starts (managed 2512 MB, resident 4146 MB)
graph: plan stage edges took 0.5 s (managed 2515 MB, 3 MB more than at its start, resident 4158 MB)
graph: plan stage metadata starts (managed 2515 MB, resident 4158 MB)
graph: plan stage metadata took 0 s (managed 2515 MB, 0 MB more than at its start, resident 4160 MB)
graph: plan stage result starts (managed 2515 MB, resident 4160 MB)
graph: plan stage result took 0 s (managed 2515 MB, 0 MB more than at its start, resident 4161 MB)
graph: plan stage inference starts (managed 2579 MB, resident 4764 MB)
2026-11-15T03:21:44.568Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":6586,"limit":6553,"worker":{"heapMb":1879.6,"extraMb":1186.3,"objectCount":8363519,"objectTypes":[{"type":"string","count":3785819},{"type":"Array","count":1340268},{"type":"Object","count":1280629},{"type":"Set","count":166806},{"type":"WeakRef","count":100297},{"type":"Structure","count":46241},{"type":"Cell Butterfly","count":33119},{"type":"Function","count":26257}],"at":1794712867144}}
```
### host-c5f530f58f3a.log
```
graph: 318 stale projects, 0 taken from the fleet, 22 too heavy for this machine (@semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), workspace (about 6364 MB), semio-framework-job-native-turn (about 6196 MB), repo-schema-commands (about 6070 MB), and 16 more; it can give about 1732 MB per project), it can infer 144 of the others (36 per wave); asking the fleet for 318
2026-11-15T03:35:29.199Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
2026-11-15T03:37:55.410Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
```

## Failure reference da146983, stack
```
2026-11-15T03:35:29.199Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T03:35:29.199Z shutdown access-denied
graph: plan answered, managed 108 MB, resident 435 MB (released 0 MB)
repository:command failed with exit code 4
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
--
2026-11-15T03:37:55.410Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T03:37:55.412Z shutdown access-denied
graph: plan answered, managed 109 MB, resident 445 MB (released 0 MB)
repository:command failed with exit code 4
```

## The typed refusal with the 20 lines before it
```
[script] build:projects · Inferring projects: 182 of 4774 projects · 3 running, longest 25 s: @semio-tech/stdio-artifact-contract-native-print-rs (language:rust, 25 s, 0 rows), @semio-tech/stdio-epw-rs (language:rust, 5 s, 0 rows), @semio-tech/stdio-zip-rs (language:rust, 0 s, 0 rows)
[capacity] waiting 30s for exec: not enough free memory (need 1536 MB; -700 MB spare above the 2048 MB floor, load 8.62; 7 running, 0 ahead); behind graph-plan (running, 1 job(s), 12362 MB), graph-farm (running, 1 job(s), 1536 MB), workspace:setup (quiet, 1 job(s), 52 MB)
[script] build:projects · Inferring projects: 183 of 4774 projects · 3 running, longest 25 s: @semio-tech/stdio-artifact-contract-native-print-rs (language:rust, 25 s, 0 rows), @semio-tech/stdio-epw-rs (language:rust, 10 s, 0 rows), @semio-tech/stdio-zip-rs (language:rust, 0 s, 0 rows)
graph: waiting for memory headroom (339 of 114 MB reserved, 1 projects running)
[script] build:projects · Inferring projects: 184 of 4774 projects · 5 running, longest 25 s: @semio-tech/stdio-artifact-contract-native-print-rs (language:rust, 25 s, 0 rows), @semio-tech/stdio-epw-rs (language:rust, 10 s, 0 rows), @semio-tech/stdio-zip-rs (language:rust, 5 s, 0 rows)
2026-11-15T03:01:07.838Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":6570,"limit":6553,"worker":{"heapMb":856.4,"extraMb":502.2,"objectCount":3762503,"objectTypes":[{"type":"string","count":1820849},{"type":"Object","count":771602},{"type":"Array","count":395717},{"type":"WeakRef","count":146127},{"type":"Structure","count":84589},{"type":"JSLexicalEnvironment","count":59412},{"type":"Function","count":55801},{"type":"Promise","count":34142}],"at":1794711667117}}
--
graph: waiting for memory headroom (3137 of 3509 MB reserved, 4 projects running)
[script] build:projects · Inferring projects: 194 of 314 projects · 4 running, longest 135 s: @semio-tech/s-dev-composition (project, 135 s, 0 rows), @semio-tech/value-rs (language:cargo, 65 s, 0 rows), @semio-tech/plugin-registry (project, 20 s, 0 rows) · waiting for memory headroom (3137 of 3509 MB reserved)
[repository:command] still running after 1771 s
[script] build:projects · Inferring projects: 194 of 314 projects · 5 running, longest 140 s: @semio-tech/s-dev-composition (project, 140 s, 0 rows), @semio-tech/value-rs (language:cargo, 70 s, 0 rows), @semio-tech/plugin-registry (project, 20 s, 0 rows)
2026-11-15T03:21:44.568Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":6586,"limit":6553,"worker":{"heapMb":1879.6,"extraMb":1186.3,"objectCount":8363519,"objectTypes":[{"type":"string","count":3785819},{"type":"Array","count":1340268},{"type":"Object","count":1280629},{"type":"Set","count":166806},{"type":"WeakRef","count":100297},{"type":"Structure","count":46241},{"type":"Cell Butterfly","count":33119},{"type":"Function","count":26257}],"at":1794712867144}}
```

## EPERM and `where:` lines of the service logs since 03:02Z
```
host-c5f530f58f3a.log:2026-11-15T03:35:29.199Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
host-c5f530f58f3a.log:2026-11-15T03:35:29.199Z shutdown access-denied
host-c5f530f58f3a.log:2026-11-15T03:37:55.410Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
host-c5f530f58f3a.log:2026-11-15T03:37:55.412Z shutdown access-denied
```

## End
- Not ready, marker absent. Stopped on `native_rc=4`; `ROUND.md` showed `round: 17` at 03:38Z (round 16 never seen here).
