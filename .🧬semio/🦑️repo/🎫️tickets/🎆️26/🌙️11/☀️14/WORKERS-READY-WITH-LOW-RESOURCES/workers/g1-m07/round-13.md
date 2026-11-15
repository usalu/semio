# Round 13 on g1-m07

## Machine
- Mac mini, 16 GiB memory, macOS 27.2.1, 102 GiB free on the data volume.
- Replica agent running (`runs = 51` at 01:40Z, kickstarted 0 times); code digest at 01:40Z `52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f`, now "code":"97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea".
- Launcher unchanged before and after all four tries: target inode 3496898, mtime 2026-11-14T22:55:52 (CET), CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`. No `native_rc=4`.

## Tries (4 of 12; ended because `ROUND.md` showed `round: 15` after try 4, round 14 never seen here)
| try | start | end | native_rc | peak resident of the run (30 s sampler) |
|---|---|---|---|---|
| 1 | 01:41:03Z | 02:04:54Z | 75 fleet refusal | 3934208 KiB |
| 2 | 02:08:17Z | 02:27:08Z | 75 fleet refusal | 3608464 KiB |
| 3 | 02:30:28Z | 02:47:57Z | 1 service failure | 5346720 KiB |
| 4 | 02:51:12Z | 03:01:07Z | 75 plan too heavy | 2501456 KiB |

No hard stop, no 15-minute stall. Pressure samples (level 1 / 2 / 4): 48/0/0  38/0/0  35/0/0  20/0/0  . All four failed in `workspace:setup` before any setup task ran: succeeded 0, failed 0, not run: all. Wall time per phase not measured.

## Final client line per try, verbatim
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 651 and delivered 39 within 870 s, 612 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 6 more (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 696 and delivered 0 within 901 s, 696 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), and 90 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The resident service met an unexpected failure (reference 47fd9038); try again, the details under that reference are in the service log /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-3af1e67bb957.log.
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6570 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## P12 sampler per try (files `round-13[-try-k].p12-samples.txt`, whole files beside this report)
### try 1
- peak sample: `2026-11-15T01:48:50Z pid=24620 rss=4679MB footprint=[bun [24620]: 64-bit    Footprint: 1449 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (4127 of 3062 MB reserved, 2 projects running)]`
- closure entries before: 42537; closure entries after: 42537
- samples with a slice worker: 270
### try 2
- peak sample: `2026-11-15T02:17:00Z pid=47285 rss=3523MB footprint=[bun [47285]: 64-bit    Footprint: 298 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 696 projects arrived]`
- closure entries before: 42537; closure entries after: 42537
- samples with a slice worker: 212
### try 3
- peak sample: `2026-11-15T02:47:04Z pid=65044 rss=5221MB footprint=[bun [65044]: 64-bit    Footprint: 2777 MB (16384 bytes per page)] last=[graph: plan stage result took 0 s (managed 3830 MB, 0 MB more than at its start, resident 5141 MB)]`
- closure entries before: 42537; closure entries after: 41808
- samples with a slice worker: 199
### try 4
- peak sample: `2026-11-15T02:58:17Z pid=82360 rss=2413MB footprint=[bun [82360]: 64-bit    Footprint: 773 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (893 of 135 MB reserved, 2 projects running)]`
- closure entries before: 41808; closure entries after: 42014
- samples with a slice worker: 104

`sample` files (worker above 4 GB resident): `round-13.p12-samples.txt.sample-2026-11-15T01:48:45Z.txt` (try 1), `round-13-try-3.p12-samples.txt.sample-2026-11-15T02:33:35Z.txt` (try 3). Section `Sort by top of stack` of the try 3 one:
```
Sort by top of stack, same collapsed (when >= 5):
        __psynch_cvwait  (in libsystem_kernel.dylib)        26322
        __ulock_wait2  (in libsystem_kernel.dylib)        24477
        kevent64  (in libsystem_kernel.dylib)        2463
        ???  (in <unknown binary>)  [0x109d0c1bc]        137
        madvise  (in libsystem_kernel.dylib)        118
        ???  (in <unknown binary>)  [0x109d0c038]        83
        ???  (in <unknown binary>)  [0x109d0c17c]        80
        ???  (in bun)  load address 0x100368000 + 0xa088  [0x100372088]        58
        _platform_memmove  (in libsystem_platform.dylib)        51
        stat  (in libsystem_kernel.dylib)        50
        ???  (in bun)  load address 0x100368000 + 0x2ec336c  [0x10322b36c]        49
        ???  (in bun)  load address 0x100368000 + 0x33ec08  [0x1006a6c08]        44
        _platform_memcmp  (in libsystem_platform.dylib)        37
        ???  (in <unknown binary>)  [0x109bf49dc]        23
        ???  (in <unknown binary>)  [0x109bf49d0]        19
        ???  (in bun)  load address 0x100368000 + 0x5d8000  [0x100940000]        17
        ???  (in bun)  load address 0x100368000 + 0x3127a4  [0x10067a7a4]        16
        ???  (in bun)  load address 0x100368000 + 0x5d80cc  [0x1009400cc]        16
        ???  (in <unknown binary>)  [0x109bf49e4]        15
        ???  (in bun)  load address 0x100368000 + 0x300af8  [0x100668af8]        15
        ???  (in bun)  load address 0x100368000 + 0x3127ac  [0x10067a7ac]        15
        ???  (in bun)  load address 0x100368000 + 0x327b18  [0x10068fb18]        15
        ???  (in bun)  load address 0x100368000 + 0x5d8038  [0x100940038]        15
        ???  (in <unknown binary>)  [0x109ba4714]        13
```

## Service logs of this round, verbatim (restore line with A and S, plan stages, fleet, too heavy, closure-missing)
### host-007c265f8fa9.log
```
graph: generation 211 (5101 projects) restored from the remote cache, 5105 documents (46 MB) downloaded in 17 s, from a state 27 min old with ? projects stale
graph: refreshing 703 projects (generation 212)
graph: plan stage inference starts (managed 1109 MB, resident 2886 MB)
graph: plan stage inference took 69.5 s (managed 1657 MB, 548 MB more than at its start, resident 3050 MB)
graph: plan stage environment starts (managed 1657 MB, resident 3050 MB)
graph: plan stage environment took 2.2 s (managed 1684 MB, 27 MB more than at its start, resident 3057 MB)
graph: plan stage tests starts (managed 1684 MB, resident 3057 MB)
graph: plan stage tests took 16.3 s (managed 1830 MB, 146 MB more than at its start, resident 3080 MB)
graph: plan stage policies starts (managed 1830 MB, resident 3080 MB)
graph: plan stage policies took 0.1 s (managed 1830 MB, 0 MB more than at its start, resident 3081 MB)
graph: plan stage dangling starts (managed 1830 MB, resident 3081 MB)
graph: plan stage dangling took 0.1 s (managed 1275 MB, -555 MB more than at its start, resident 3081 MB)
graph: plan stage files starts (managed 1276 MB, resident 3081 MB)
graph: plan stage files took 0.1 s (managed 1736 MB, 460 MB more than at its start, resident 3081 MB)
graph: plan stage test-edges starts (managed 1736 MB, resident 3081 MB)
graph: plan stage test-edges took 0.2 s (managed 1772 MB, 35 MB more than at its start, resident 3081 MB)
graph: plan stage edges starts (managed 1772 MB, resident 3081 MB)
graph: plan stage edges took 5.7 s (managed 1755 MB, -17 MB more than at its start, resident 3085 MB)
graph: plan stage metadata starts (managed 1755 MB, resident 3085 MB)
graph: plan stage metadata took 0 s (managed 1755 MB, 0 MB more than at its start, resident 3085 MB)
graph: plan stage result starts (managed 1755 MB, resident 3085 MB)
graph: plan stage result took 0 s (managed 1755 MB, 0 MB more than at its start, resident 3085 MB)
graph: plan stage inference starts (managed 1969 MB, resident 3839 MB)
graph: 651 stale projects, 0 taken from the fleet, 12 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 6 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 651
graph: nothing to wait for any longer: the fleet was asked for 651 stale projects and delivered 39 within 870 s; 612 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 6 more, which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
```
### host-3af1e67bb957.log
```
graph: 696 stale projects, 0 taken from the fleet, 96 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), and 90 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 696
graph: nothing to wait for any longer: the fleet was asked for 696 stale projects and delivered 0 within 901 s; 696 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), @semio-tech/plugin-publication-mounted-rs (about 6484 MB), and 90 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: generation 216 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 13 s, from a state 22 min old with 0 projects stale
graph: refreshing 569 projects (generation 217)
graph: plan stage inference starts (managed 1007 MB, resident 3970 MB)
graph: plan stage inference took 23.9 s (managed 1522 MB, 515 MB more than at its start, resident 4053 MB)
graph: plan stage environment starts (managed 1522 MB, resident 4053 MB)
graph: plan stage environment took 0.4 s (managed 1527 MB, 5 MB more than at its start, resident 4057 MB)
graph: plan stage tests starts (managed 1527 MB, resident 4057 MB)
graph: plan stage tests took 0 s (managed 1527 MB, 0 MB more than at its start, resident 4057 MB)
graph: plan stage policies starts (managed 1527 MB, resident 4057 MB)
graph: plan stage policies took 0 s (managed 1527 MB, 0 MB more than at its start, resident 4057 MB)
graph: plan stage dangling starts (managed 1527 MB, resident 4057 MB)
graph: plan stage dangling took 0 s (managed 1527 MB, 0 MB more than at its start, resident 4057 MB)
graph: plan stage files starts (managed 1527 MB, resident 4057 MB)
graph: plan stage files took 0 s (managed 1527 MB, 0 MB more than at its start, resident 4057 MB)
graph: plan stage test-edges starts (managed 1527 MB, resident 4057 MB)
graph: plan stage test-edges took 0 s (managed 1527 MB, 0 MB more than at its start, resident 4057 MB)
graph: plan stage edges starts (managed 1527 MB, resident 4057 MB)
graph: plan stage edges took 0.3 s (managed 1599 MB, 71 MB more than at its start, resident 4057 MB)
graph: plan stage metadata starts (managed 1599 MB, resident 4057 MB)
graph: plan stage metadata took 0 s (managed 1599 MB, 0 MB more than at its start, resident 4058 MB)
graph: plan stage result starts (managed 1599 MB, resident 4058 MB)
graph: plan stage result took 0 s (managed 1599 MB, 0 MB more than at its start, resident 4058 MB)
graph: plan stage inference starts (managed 1796 MB, resident 4873 MB)
graph: 50 projects taken from the fleet, 0 inferred here
graph: 78 projects taken from the fleet, 22 inferred here
graph: plan stage inference took 416.9 s (managed 2588 MB, 792 MB more than at its start, resident 4414 MB)
graph: plan stage environment starts (managed 2588 MB, resident 4414 MB)
graph: plan stage environment took 1 s (managed 2605 MB, 17 MB more than at its start, resident 4429 MB)
graph: plan stage tests starts (managed 2605 MB, resident 4429 MB)
graph: plan stage tests took 0 s (managed 2605 MB, 0 MB more than at its start, resident 4430 MB)
graph: plan stage policies starts (managed 2605 MB, resident 4430 MB)
graph: plan stage policies took 0 s (managed 2605 MB, 0 MB more than at its start, resident 4430 MB)
graph: plan stage dangling starts (managed 2605 MB, resident 4430 MB)
graph: plan stage dangling took 0 s (managed 2605 MB, 0 MB more than at its start, resident 4430 MB)
graph: plan stage files starts (managed 2605 MB, resident 4430 MB)
graph: plan stage files took 0 s (managed 2605 MB, 0 MB more than at its start, resident 4430 MB)
graph: plan stage test-edges starts (managed 2605 MB, resident 4430 MB)
graph: plan stage test-edges took 0 s (managed 2605 MB, 0 MB more than at its start, resident 4430 MB)
graph: plan stage edges starts (managed 2605 MB, resident 4430 MB)
graph: plan stage edges took 1.1 s (managed 2707 MB, 102 MB more than at its start, resident 4435 MB)
graph: plan stage metadata starts (managed 2707 MB, resident 4435 MB)
graph: plan stage metadata took 0 s (managed 2707 MB, 0 MB more than at its start, resident 4435 MB)
graph: plan stage result starts (managed 2707 MB, resident 4435 MB)
graph: plan stage result took 0 s (managed 2707 MB, 0 MB more than at its start, resident 4436 MB)
graph: plan stage inference starts (managed 2569 MB, resident 4820 MB)
graph: 50 projects taken from the fleet, 0 inferred here
graph: 100 projects taken from the fleet, 0 inferred here
graph: plan stage inference took 43.2 s (managed 2392 MB, -177 MB more than at its start, resident 4868 MB)
graph: plan stage environment starts (managed 2392 MB, resident 4868 MB)
graph: plan stage environment took 0.6 s (managed 2436 MB, 44 MB more than at its start, resident 4870 MB)
graph: plan stage tests starts (managed 2436 MB, resident 4870 MB)
graph: plan stage tests took 0 s (managed 2436 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage policies starts (managed 2436 MB, resident 4870 MB)
graph: plan stage policies took 0 s (managed 2436 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage dangling starts (managed 2436 MB, resident 4870 MB)
graph: plan stage dangling took 0 s (managed 2436 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage files starts (managed 2436 MB, resident 4870 MB)
graph: plan stage files took 0 s (managed 2436 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage test-edges starts (managed 2436 MB, resident 4870 MB)
graph: plan stage test-edges took 0 s (managed 2436 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage edges starts (managed 2436 MB, resident 4870 MB)
graph: plan stage edges took 0.3 s (managed 2440 MB, 4 MB more than at its start, resident 4870 MB)
graph: plan stage metadata starts (managed 2440 MB, resident 4870 MB)
graph: plan stage metadata took 0 s (managed 2440 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage result starts (managed 2440 MB, resident 4870 MB)
graph: plan stage result took 0 s (managed 2440 MB, 0 MB more than at its start, resident 4870 MB)
graph: plan stage inference starts (managed 2471 MB, resident 4935 MB)
graph: 48 projects taken from the fleet, 2 inferred here
graph: 96 projects taken from the fleet, 4 inferred here
graph: 143 projects taken from the fleet, 7 inferred here
graph: plan stage inference took 281.4 s (managed 3696 MB, 1225 MB more than at its start, resident 5100 MB)
graph: plan stage environment starts (managed 3696 MB, resident 5100 MB)
graph: plan stage environment took 0.8 s (managed 3743 MB, 46 MB more than at its start, resident 5107 MB)
graph: plan stage tests starts (managed 3743 MB, resident 5107 MB)
graph: plan stage tests took 2 s (managed 4010 MB, 267 MB more than at its start, resident 5131 MB)
graph: plan stage policies starts (managed 4010 MB, resident 5131 MB)
graph: plan stage policies took 0 s (managed 4010 MB, 0 MB more than at its start, resident 5131 MB)
graph: plan stage dangling starts (managed 4010 MB, resident 5131 MB)
graph: plan stage dangling took 0 s (managed 4010 MB, 0 MB more than at its start, resident 5131 MB)
graph: plan stage files starts (managed 4010 MB, resident 5131 MB)
graph: plan stage files took 0 s (managed 4010 MB, 0 MB more than at its start, resident 5131 MB)
graph: plan stage test-edges starts (managed 4010 MB, resident 5131 MB)
graph: plan stage test-edges took 0 s (managed 4010 MB, 0 MB more than at its start, resident 5131 MB)
graph: plan stage edges starts (managed 4010 MB, resident 5131 MB)
graph: plan stage edges took 6.4 s (managed 3830 MB, -180 MB more than at its start, resident 5141 MB)
graph: plan stage metadata starts (managed 3830 MB, resident 5141 MB)
graph: plan stage metadata took 0 s (managed 3830 MB, 0 MB more than at its start, resident 5141 MB)
graph: plan stage result starts (managed 3830 MB, resident 5141 MB)
graph: plan stage result took 0 s (managed 3830 MB, 0 MB more than at its start, resident 5141 MB)
graph: generation 220 published, 569 projects refreshed in 974 s (0 kept their edges)
[graph] closure-missing build-input-closures/c90f7d707bb1778170c7d891638c3b5f4eea3c036c308e84c90dd2f5a59fda99 named by @semio-tech/repo-identity-go
2026-11-15T02:47:57.942Z request-failed 47fd9038 Error
```
### host-79a74804a430.log
```
graph: plan stage inference starts (managed 189 MB, resident 990 MB)
graph: 0 projects taken from the fleet, 50 inferred here
graph: 0 projects taken from the fleet, 100 inferred here
graph: 0 projects taken from the fleet, 150 inferred here
2026-11-15T03:01:07.838Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":6570,"limit":6553,"worker":{"heapMb":856.4,"extraMb":502.2,"objectCount":3762503,"objectTypes":[{"type":"string","count":1820849},{"type":"Object","count":771602},{"type":"Array","count":395717},{"type":"WeakRef","count":146127},{"type":"Structure","count":84589},{"type":"JSLexicalEnvironment","count":59412},{"type":"Function","count":55801},{"type":"Promise","count":34142}],"at":1794711667117}}
graph: plan stage inference took 486.8 s (managed 1356 MB, 1167 MB more than at its start, resident 1517 MB)
```

## Failure reference 47fd9038, stack
```
2026-11-15T02:47:57.942Z request-failed 47fd9038 Error
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:87:451)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (unknown:7:39)
```

## The typed refusal with the 20 lines before it
```
[script] build:projects · Inferring projects: 182 of 4774 projects · 3 running, longest 25 s: @semio-tech/stdio-artifact-contract-native-print-rs (language:rust, 25 s, 0 rows), @semio-tech/stdio-epw-rs (language:rust, 5 s, 0 rows), @semio-tech/stdio-zip-rs (language:rust, 0 s, 0 rows)
[capacity] waiting 30s for exec: not enough free memory (need 1536 MB; -700 MB spare above the 2048 MB floor, load 8.62; 7 running, 0 ahead); behind graph-plan (running, 1 job(s), 12362 MB), graph-farm (running, 1 job(s), 1536 MB), workspace:setup (quiet, 1 job(s), 52 MB)
[script] build:projects · Inferring projects: 183 of 4774 projects · 3 running, longest 25 s: @semio-tech/stdio-artifact-contract-native-print-rs (language:rust, 25 s, 0 rows), @semio-tech/stdio-epw-rs (language:rust, 10 s, 0 rows), @semio-tech/stdio-zip-rs (language:rust, 0 s, 0 rows)
graph: waiting for memory headroom (339 of 114 MB reserved, 1 projects running)
[script] build:projects · Inferring projects: 184 of 4774 projects · 5 running, longest 25 s: @semio-tech/stdio-artifact-contract-native-print-rs (language:rust, 25 s, 0 rows), @semio-tech/stdio-epw-rs (language:rust, 10 s, 0 rows), @semio-tech/stdio-zip-rs (language:rust, 5 s, 0 rows)
2026-11-15T03:01:07.838Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":6570,"limit":6553,"worker":{"heapMb":856.4,"extraMb":502.2,"objectCount":3762503,"objectTypes":[{"type":"string","count":1820849},{"type":"Object","count":771602},{"type":"Array","count":395717},{"type":"WeakRef","count":146127},{"type":"Structure","count":84589},{"type":"JSLexicalEnvironment","count":59412},{"type":"Function","count":55801},{"type":"Promise","count":34142}],"at":1794711667117}}
```

## End
- Not ready, marker absent. Going on with round 15.
