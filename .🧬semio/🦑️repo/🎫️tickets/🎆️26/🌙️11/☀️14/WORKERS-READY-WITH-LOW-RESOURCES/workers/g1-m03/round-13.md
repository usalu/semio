# g1-m03 Round 13

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 82 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: two tries, marker absent, `ready` false. A third try was not started: round 14 was posted during the 180 s wait after try 2.
  - Try 1: `native_rc=75`, the typed refusal **`graph-plan-too-heavy`, stage `inference`, 6766 MB against the limit 6553 MB**, after 6 min 21 s. No hard stop of the runner.
  - Try 2: the graph read went through (`owners 703 of 703` was passed, the setup tasks ran, 12 tasks passed), then a **hard stop of the runner: memory pressure level 2 in six samples in a row** (02:14:00Z to 02:16:31Z), one SIGINT, `native_rc=1`. At that time the run was in the setup tasks (`> nx run workspace:deps-go`), not in the graph plan.
- No `native_rc=4`, no `closure-missing` line, no `Another process holds the dependency store` line in this round (try 2 was stopped before `workspace:deps-build-tools` started).

## Try 1: the typed refusal (verbatim) with the 20 lines before it

Graph service log `$TMPDIR/semio-repo/501/host-007c265f8fa9.log` (the 20 lines before the refusal, without JSON progress lines, then the refusal):

```
[script] build:projects · Inferring projects: 11 of 48 projects · 2 running, longest 15 s: @semio-tech/framework-process (project, 15 s, 0 rows), @semio-tech/framework-replication-rs (language:cargo, 0 s, 0 rows)
graph: waiting for memory headroom (1442 of -2072 MB reserved, 1 projects running)
graph: plan stage inference grew the plan by 4985 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory
[capacity] waiting 27s for exec: not enough free memory (need 1536 MB; -847 MB spare above the 2048 MB floor, load 9.03; 4 running, 0 ahead); behind graph-plan (running, 1 job(s), 9732 MB), graph-farm (running, 1 job(s), 1536 MB), workspace:setup (quiet, 1 job(s), 49 MB)
2026-11-15T01:47:34.826Z graph-plan-too-heavy {"stage":"inference","megabytes":6766,"limit":6553,"worker":{"heapMb":1073.8,"extraMb":714.8,"objectCount":5837471,"objectTypes":[{"type":"string","count":2719592},{"type":"Array","count":967658},{"type":"Object","count":931632},{"type":"Set","count":160498},{"type":"WeakRef","count":101745},{"type":"Structure","count":39005},{"type":"Cell Butterfly","count":34293},{"type":"Function","count":29542}],"at":1794707224446}}
```

Bootstrap log, the refusal (verbatim):

```
The project graph could not be read on this machine yet: the plan stage inference grew to 6766 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T01:47:34Z
native_rc=75
```

## Per try

| Try | Log | Start | End | `native_rc` | Restore line: A (minutes old), S (projects stale) | `stale projects` line | Reached |
|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-13.log` | 01:41:16Z | 01:47:37Z | 75 | no restore line in the service log of this try | none printed | `owners 11 of 125`, plan stage `inference` |
| 2 | `~/native-bootstrap-round-13-try-2.log` | 01:50:53Z | 02:16:32Z | 1 (SIGINT, hard stop) | A = 36 min, S = `?` (`from a state 36 min old with ? projects stale`) | none printed | graph read complete, setup tasks, stopped during `workspace:deps-go` |

Restore lines verbatim (S is printed as `?` in the line of try 2):

```
graph: generation 211 (5101 projects) restored from the remote cache, 5105 documents (46 MB) downloaded in 12 s, from a state 36 min old with ? projects stale
```

Wait between the tries: 180 s.

### Try 1: service log lines (plan stages, stale projects, fleet, too heavy), verbatim, each distinct line once

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-007c265f8fa9.log (lines after 0)
graph: refreshing 125 projects (generation 75)
graph: plan stage inference starts (managed 1136 MB, resident 2910 MB)
graph: project @semio-tech/framework-process (project) grew by 2441 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: plan stage inference grew the plan by 4985 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory
2026-11-15T01:47:34.826Z graph-plan-too-heavy {"stage":"inference","megabytes":6766,"limit":6553,"worker":{"heapMb":1073.8,"extraMb":714.8,"objectCount":5837471,"objectTypes":[{"type":"string","count":2719592},{"type":"Array","count":967658},{"type":"Object","count":931632},{"type":"Set","count":160498},{"type":"WeakRef","count":101745},{"type":"Structure","count":39005},{"type":"Cell Butterfly","count":34293},{"type":"Function","count":29542}],"at":1794707224446}}
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log (lines after 26884)
```

### Try 2: service log lines, verbatim, each distinct line once

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-007c265f8fa9.log (lines after 1998)
graph: generation 211 (5101 projects) restored from the remote cache, 5105 documents (46 MB) downloaded in 12 s, from a state 36 min old with ? projects stale
graph: refreshing 703 projects (generation 212)
graph: plan stage inference starts (managed 1040 MB, resident 3425 MB)
graph: plan stage inference took 52.1 s (managed 1494 MB, 454 MB more than at its start, resident 3490 MB)
graph: plan stage environment starts (managed 1494 MB, resident 3490 MB)
graph: plan stage environment took 1.9 s (managed 1543 MB, 49 MB more than at its start, resident 3493 MB)
graph: plan stage tests starts (managed 1543 MB, resident 3493 MB)
graph: plan stage tests took 10.8 s (managed 1669 MB, 126 MB more than at its start, resident 3593 MB)
graph: plan stage policies starts (managed 1669 MB, resident 3593 MB)
graph: plan stage policies took 0.8 s (managed 1669 MB, 0 MB more than at its start, resident 3593 MB)
graph: plan stage dangling starts (managed 1669 MB, resident 3593 MB)
graph: plan stage dangling took 0.6 s (managed 1670 MB, 1 MB more than at its start, resident 3594 MB)
graph: plan stage files starts (managed 1670 MB, resident 3594 MB)
graph: plan stage files took 0 s (managed 1670 MB, 0 MB more than at its start, resident 3594 MB)
graph: plan stage test-edges starts (managed 1670 MB, resident 3594 MB)
graph: plan stage test-edges took 0.1 s (managed 1670 MB, 0 MB more than at its start, resident 3594 MB)
graph: plan stage edges starts (managed 1670 MB, resident 3594 MB)
graph: plan stage edges took 5.7 s (managed 1892 MB, 223 MB more than at its start, resident 3597 MB)
graph: plan stage metadata starts (managed 1892 MB, resident 3597 MB)
graph: plan stage metadata took 0 s (managed 1892 MB, 0 MB more than at its start, resident 3597 MB)
graph: plan stage result starts (managed 1892 MB, resident 3597 MB)
graph: plan stage result took 0 s (managed 1892 MB, 0 MB more than at its start, resident 3597 MB)
graph: plan stage inference starts (managed 1831 MB, resident 4987 MB)
graph: plan stage inference took 353 s (managed 3223 MB, 1391 MB more than at its start, resident 4746 MB)
graph: plan stage environment starts (managed 3223 MB, resident 4746 MB)
graph: plan stage environment took 0.8 s (managed 3270 MB, 48 MB more than at its start, resident 4761 MB)
graph: plan stage tests starts (managed 3270 MB, resident 4761 MB)
graph: plan stage tests took 0 s (managed 3270 MB, 0 MB more than at its start, resident 4762 MB)
graph: plan stage policies starts (managed 3270 MB, resident 4762 MB)
graph: plan stage policies took 0 s (managed 3270 MB, 0 MB more than at its start, resident 4762 MB)
graph: plan stage dangling starts (managed 3270 MB, resident 4762 MB)
graph: plan stage dangling took 0 s (managed 3270 MB, 0 MB more than at its start, resident 4762 MB)
graph: plan stage files starts (managed 3270 MB, resident 4762 MB)
graph: plan stage files took 0 s (managed 3270 MB, 0 MB more than at its start, resident 4762 MB)
graph: plan stage test-edges starts (managed 3270 MB, resident 4762 MB)
graph: plan stage test-edges took 0 s (managed 3270 MB, 0 MB more than at its start, resident 4762 MB)
graph: plan stage edges starts (managed 3270 MB, resident 4762 MB)
graph: plan stage edges took 1 s (managed 3283 MB, 13 MB more than at its start, resident 4769 MB)
graph: plan stage metadata starts (managed 3283 MB, resident 4769 MB)
graph: plan stage metadata took 0 s (managed 3283 MB, 0 MB more than at its start, resident 4769 MB)
graph: plan stage result starts (managed 3283 MB, resident 4769 MB)
graph: plan stage result took 0 s (managed 3283 MB, 0 MB more than at its start, resident 4769 MB)
graph: plan stage inference starts (managed 3213 MB, resident 5385 MB)
graph: plan stage inference took 24.6 s (managed 2493 MB, -719 MB more than at its start, resident 5453 MB)
graph: plan stage environment starts (managed 2493 MB, resident 5453 MB)
graph: plan stage environment took 0.4 s (managed 2498 MB, 5 MB more than at its start, resident 5460 MB)
graph: plan stage tests starts (managed 2498 MB, resident 5460 MB)
graph: plan stage tests took 0 s (managed 2498 MB, 0 MB more than at its start, resident 5460 MB)
graph: plan stage policies starts (managed 2498 MB, resident 5460 MB)
graph: plan stage policies took 0 s (managed 2498 MB, 0 MB more than at its start, resident 5460 MB)
graph: plan stage dangling starts (managed 2498 MB, resident 5460 MB)
graph: plan stage dangling took 0 s (managed 2498 MB, 0 MB more than at its start, resident 5460 MB)
graph: plan stage files starts (managed 2498 MB, resident 5460 MB)
graph: plan stage files took 0 s (managed 2498 MB, 0 MB more than at its start, resident 5460 MB)
graph: plan stage test-edges starts (managed 2498 MB, resident 5460 MB)
graph: plan stage test-edges took 0 s (managed 2498 MB, 0 MB more than at its start, resident 5460 MB)
graph: plan stage edges starts (managed 2498 MB, resident 5460 MB)
graph: plan stage edges took 0.2 s (managed 2575 MB, 77 MB more than at its start, resident 5464 MB)
graph: plan stage metadata starts (managed 2575 MB, resident 5464 MB)
graph: plan stage metadata took 0 s (managed 2575 MB, 0 MB more than at its start, resident 5464 MB)
graph: plan stage result starts (managed 2575 MB, resident 5464 MB)
graph: plan stage result took 0 s (managed 2575 MB, 0 MB more than at its start, resident 5464 MB)
graph: plan stage inference starts (managed 2549 MB, resident 5312 MB)
graph: plan stage inference took 381.3 s (managed 3472 MB, 923 MB more than at its start, resident 4104 MB)
graph: plan stage environment starts (managed 3472 MB, resident 4104 MB)
graph: plan stage environment took 0.8 s (managed 3480 MB, 8 MB more than at its start, resident 4134 MB)
graph: plan stage tests starts (managed 3480 MB, resident 4134 MB)
graph: plan stage tests took 5.3 s (managed 3935 MB, 456 MB more than at its start, resident 4191 MB)
graph: plan stage policies starts (managed 3935 MB, resident 4191 MB)
graph: plan stage policies took 0.1 s (managed 3935 MB, 0 MB more than at its start, resident 4192 MB)
graph: plan stage dangling starts (managed 3935 MB, resident 4192 MB)
graph: plan stage dangling took 0 s (managed 3935 MB, 0 MB more than at its start, resident 4192 MB)
graph: plan stage files starts (managed 3935 MB, resident 4192 MB)
graph: plan stage files took 0 s (managed 3935 MB, 0 MB more than at its start, resident 4193 MB)
graph: plan stage test-edges starts (managed 3935 MB, resident 4193 MB)
graph: plan stage test-edges took 0 s (managed 3935 MB, 0 MB more than at its start, resident 4193 MB)
graph: plan stage edges starts (managed 3935 MB, resident 4193 MB)
graph: plan stage edges took 11.9 s (managed 3618 MB, -317 MB more than at its start, resident 4404 MB)
graph: plan stage metadata starts (managed 3618 MB, resident 4404 MB)
graph: plan stage metadata took 0.1 s (managed 3618 MB, 0 MB more than at its start, resident 4407 MB)
graph: plan stage result starts (managed 3618 MB, resident 4407 MB)
graph: plan stage result took 0 s (managed 3618 MB, 0 MB more than at its start, resident 4407 MB)
```

The plan stage with the largest growth in try 2:

```
graph: plan stage tests took 5.3 s (managed 3935 MB, 456 MB more than at its start, resident 4191 MB)
graph: plan stage inference took 381.3 s (managed 3472 MB, 923 MB more than at its start, resident 4104 MB)
graph: plan stage inference took 353 s (managed 3223 MB, 1391 MB more than at its start, resident 4746 MB)
```

## Sampler of `ROUND-NOTE-P12.md` (slice worker every 5 s)

### Try 1 (`round-13.try-1.p12-samples.txt`)

First lines:

```
service log: /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log
closure entries before: 14055
2026-11-15T01:41:16Z no slice worker
```

Row with the largest resident size, the three rows before it and the last six rows (no row has a footprint above 6 000 MB):

```
2026-11-15T01:47:18Z pid=82242 rss=3179MB footprint=[bun [82242]: 64-bit    Footprint: 1452 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1588 of 1005 MB reserved, 1 projects running)]
2026-11-15T01:47:23Z pid=82242 rss=4171MB footprint=[bun [82242]: 64-bit    Footprint: 1889 MB (16384 bytes per page)] last=[graph: project @semio-tech/framework-process (project) grew by 2441 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too he]
2026-11-15T01:47:28Z pid=82242 rss=5011MB footprint=[bun [82242]: 64-bit    Footprint: 4395 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1261 of -1038 MB reserved, 1 projects running)]
2026-11-15T01:47:33Z pid=82242 rss=5230MB footprint=[bun [82242]: 64-bit    Footprint: 5640 MB (16384 bytes per page)] last=[graph: plan stage inference grew the plan by 4985 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory]
…
2026-11-15T01:47:28Z pid=82242 rss=5011MB footprint=[bun [82242]: 64-bit    Footprint: 4395 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1261 of -1038 MB reserved, 1 projects running)]
2026-11-15T01:47:33Z pid=82242 rss=5230MB footprint=[bun [82242]: 64-bit    Footprint: 5640 MB (16384 bytes per page)] last=[graph: plan stage inference grew the plan by 4985 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory]
2026-11-15T01:47:38Z no slice worker
2026-11-15T01:47:43Z no slice worker
2026-11-15T01:47:48Z no slice worker
closure entries after: 12893
```

Largest footprint seen: 5640 MB; largest resident size: 5230 MB.

Stack sample `round-13.try-1.p12-samples.txt.sample-2026-11-15T01:47:23Z.txt`, section `Sort by top of stack` (first 40 lines):

```
Sort by top of stack, same collapsed (when >= 5):
        __psynch_cvwait  (in libsystem_kernel.dylib)        25528
        __ulock_wait2  (in libsystem_kernel.dylib)        23627
        __read_nocancel  (in libsystem_kernel.dylib)        4222
        kevent64  (in libsystem_kernel.dylib)        2860
        __openat_nocancel  (in libsystem_kernel.dylib)        546
        ???  (in bun)  load address 0x104ec8000 + 0x311514  [0x1051d9514]        138
        ???  (in bun)  load address 0x104ec8000 + 0x300b2c  [0x1051c8b2c]        124
        ???  (in bun)  load address 0x104ec8000 + 0xa088  [0x104ed2088]        122
        _platform_memmove  (in libsystem_platform.dylib)        97
        ???  (in bun)  load address 0x104ec8000 + 0x302630  [0x1051ca630]        81
        ???  (in bun)  load address 0x104ec8000 + 0x311450  [0x1051d9450]        81
        ???  (in bun)  load address 0x104ec8000 + 0x2ec336c  [0x107d8b36c]        76
        ???  (in bun)  load address 0x104ec8000 + 0x315950  [0x1051dd950]        67
        ???  (in bun)  load address 0x104ec8000 + 0x3152c4  [0x1051dd2c4]        65
        ???  (in bun)  load address 0x104ec8000 + 0x101840c  [0x105ee040c]        64
        ???  (in bun)  load address 0x104ec8000 + 0x315308  [0x1051dd308]        62
        ???  (in bun)  load address 0x104ec8000 + 0x2d89634  [0x107c51634]        61
        ???  (in bun)  load address 0x104ec8000 + 0x3154f8  [0x1051dd4f8]        59
        madvise  (in libsystem_kernel.dylib)        58
        ???  (in bun)  load address 0x104ec8000 + 0x3029d4  [0x1051ca9d4]        55
        ???  (in bun)  load address 0x104ec8000 + 0x300af8  [0x1051c8af8]        53
        ???  (in bun)  load address 0x104ec8000 + 0x302a08  [0x1051caa08]        46
        ???  (in bun)  load address 0x104ec8000 + 0x3152b0  [0x1051dd2b0]        46
        ???  (in bun)  load address 0x104ec8000 + 0x2d8985c  [0x107c5185c]        45
        ???  (in bun)  load address 0x104ec8000 + 0x3127ac  [0x1051da7ac]        45
        ???  (in bun)  load address 0x104ec8000 + 0x300ac0  [0x1051c8ac0]        43
        ???  (in bun)  load address 0x104ec8000 + 0x3152e8  [0x1051dd2e8]        42
        ???  (in bun)  load address 0x104ec8000 + 0x10183ec  [0x105ee03ec]        41
        ???  (in bun)  load address 0x104ec8000 + 0x3127a4  [0x1051da7a4]        41
        lstat  (in libsystem_kernel.dylib)        40
        ???  (in bun)  load address 0x104ec8000 + 0x3029ac  [0x1051ca9ac]        38
        ???  (in bun)  load address 0x104ec8000 + 0x300b24  [0x1051c8b24]        37
        ???  (in bun)  load address 0x104ec8000 + 0x302b88  [0x1051cab88]        37
        ???  (in bun)  load address 0x104ec8000 + 0x43f7e0  [0x1053077e0]        37
        ???  (in bun)  load address 0x104ec8000 + 0x2ec335c  [0x107d8b35c]        36
        ???  (in bun)  load address 0x104ec8000 + 0x311c08  [0x1051d9c08]        35
        __pread_nocancel  (in libsystem_kernel.dylib)        35
        ???  (in bun)  load address 0x104ec8000 + 0x300ac8  [0x1051c8ac8]        34
        ???  (in bun)  load address 0x104ec8000 + 0x311534  [0x1051d9534]        34
```

### Try 2 (`round-13.try-2.p12-samples.txt`)

First lines:

```
service log: /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-007c265f8fa9.log
closure entries before: 12893
2026-11-15T01:50:53Z no slice worker
```

Row with the largest resident size, the three rows before it and the last six rows (no row has a footprint above 6 000 MB):

```
2026-11-15T02:02:07Z pid=88609 rss=5450MB footprint=[bun [88609]: 64-bit    Footprint: 2965 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (1688 of 1863 MB reserved, 4 projects running)]
2026-11-15T02:02:12Z pid=88609 rss=5451MB footprint=[bun [88609]: 64-bit    Footprint: 2712 MB (16384 bytes per page)] last=[graph: plan stage environment starts (managed 2493 MB, resident 5453 MB)]
2026-11-15T02:02:17Z pid=88609 rss=5549MB footprint=[bun [88609]: 64-bit    Footprint: 2613 MB (16384 bytes per page)] last=[graph: plan stage result took 0 s (managed 2575 MB, 0 MB more than at its start, resident 5464 MB)]
2026-11-15T02:02:22Z pid=88609 rss=5550MB footprint=[bun [88609]: 64-bit    Footprint: 2529 MB (16384 bytes per page)] last=[graph: plan stage result took 0 s (managed 2575 MB, 0 MB more than at its start, resident 5464 MB)]
…
2026-11-15T02:16:38Z no slice worker
2026-11-15T02:16:43Z no slice worker
2026-11-15T02:16:48Z no slice worker
2026-11-15T02:16:53Z no slice worker
2026-11-15T02:16:59Z no slice worker
closure entries after: 14248
```

Largest footprint seen: 4608 MB; largest resident size: 5550 MB.

Stack sample `round-13.try-2.p12-samples.txt.sample-2026-11-15T01:55:24Z.txt`, section `Sort by top of stack` (first 40 lines):

```
Sort by top of stack, same collapsed (when >= 5):
        __psynch_cvwait  (in libsystem_kernel.dylib)        39163
        __ulock_wait2  (in libsystem_kernel.dylib)        35043
        kevent64  (in libsystem_kernel.dylib)        3519
        madvise  (in libsystem_kernel.dylib)        167
        ???  (in bun)  load address 0x10040c000 + 0x2ec336c  [0x1032cf36c]        119
        ???  (in bun)  load address 0x10040c000 + 0x327b08  [0x100733b08]        87
        ???  (in bun)  load address 0x10040c000 + 0x311514  [0x10071d514]        81
        ???  (in <unknown binary>)  [0x109b23934]        76
        _platform_memset  (in libsystem_platform.dylib)        70
        ???  (in bun)  load address 0x10040c000 + 0x311c08  [0x10071dc08]        69
        ???  (in bun)  load address 0x10040c000 + 0x2805b9c  [0x102c11b9c]        67
        ???  (in bun)  load address 0x10040c000 + 0x302a08  [0x10070ea08]        66
        ???  (in bun)  load address 0x10040c000 + 0x302630  [0x10070e630]        65
        ???  (in bun)  load address 0x10040c000 + 0x327b18  [0x100733b18]        64
        ???  (in <unknown binary>)  [0x108f884dc]        61
        ???  (in bun)  load address 0x10040c000 + 0x327af4  [0x100733af4]        59
        ???  (in bun)  load address 0x10040c000 + 0x315308  [0x100721308]        58
        ???  (in bun)  load address 0x10040c000 + 0x2ec335c  [0x1032cf35c]        57
        ???  (in bun)  load address 0x10040c000 + 0x2ec3348  [0x1032cf348]        56
        ???  (in bun)  load address 0x10040c000 + 0x3152e8  [0x1007212e8]        53
        ???  (in <unknown binary>)  [0x109b23bf4]        52
        syscall_thread_switch  (in libsystem_kernel.dylib)        52
        ???  (in bun)  load address 0x10040c000 + 0x2ec33d4  [0x1032cf3d4]        47
        ???  (in bun)  load address 0x10040c000 + 0x300af8  [0x10070caf8]        46
        ???  (in bun)  load address 0x10040c000 + 0x3029d4  [0x10070e9d4]        46
        ???  (in bun)  load address 0x10040c000 + 0x3029ac  [0x10070e9ac]        44
        ???  (in bun)  load address 0x10040c000 + 0x3152c4  [0x1007212c4]        42
        ???  (in bun)  load address 0x10040c000 + 0x31a680  [0x100726680]        41
        ???  (in bun)  load address 0x10040c000 + 0x3127a4  [0x10071e7a4]        39
        ???  (in bun)  load address 0x10040c000 + 0x311d18  [0x10071dd18]        38
        ???  (in bun)  load address 0x10040c000 + 0x311d20  [0x10071dd20]        37
        ???  (in bun)  load address 0x10040c000 + 0x311c10  [0x10071dc10]        36
        ???  (in bun)  load address 0x10040c000 + 0x31a688  [0x100726688]        36
        ???  (in <unknown binary>)  [0x109b23788]        35
        ???  (in bun)  load address 0x10040c000 + 0x302b6c  [0x10070eb6c]        35
        ???  (in bun)  load address 0x10040c000 + 0x311c24  [0x10071dc24]        34
        ???  (in bun)  load address 0x10040c000 + 0x3127ac  [0x10071e7ac]        34
        ???  (in bun)  load address 0x10040c000 + 0x3154f8  [0x1007214f8]        34
        ???  (in bun)  load address 0x10040c000 + 0x311d44  [0x10071dd44]        32
```

Closure entries (`find .cache/b/g/m/inference/selections/*/ce/e -type f | wc -l`): try 1 before 14055, after 12893; try 2 before 12893, after 14248.

## Hard stop of try 2

30 s samples `round-13.try-2.samples.txt`: pressure level 1 until 02:13:30Z, then level 2 at 02:14:00Z, 02:14:30Z, 02:15:00Z, 02:15:31Z, 02:16:01Z, 02:16:31Z; SIGINT at 02:16:31Z. The three largest processes in those six samples:

```
2026-11-15T02:14:00Z pressure=2
1701840  9746 47709 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.h
971360  9968  8480 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897fc
581072  8480  7200 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897fc
2026-11-15T02:14:30Z pressure=2
1301296 10466 47709 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.h
1039376 10179  8480 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897f
653952  3292 83513 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M
2026-11-15T02:15:00Z pressure=2
1217472  8480  7200 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897f
1103040  3292 83513 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_
445120 11075  8480 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897fc
2026-11-15T02:15:31Z pressure=2
1419504 11247 47709 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.h
780576  3292 83513 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M
557920  8480  7200 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897fc
2026-11-15T02:16:01Z pressure=2
1520608 11832 47709 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.h
784000  3292 83513 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M
557792  8480  7200 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897fc
2026-11-15T02:16:31Z pressure=2
1245040 12733 47709 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.h
777296  3292 83513 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M
676928  8480  7200 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/b/t/m/nx/d954e5a1b84fd51e3897fc
```

The last lines of the bootstrap log before the SIGINT:

```
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
> repo "$SEMIO_NX_COMMAND_RUNNER" 7148a0d4464a12bf9cf5629b072078c622886b952f568dde00794b56c189073b
[repo] still working after 1490 s; last phase started: source.served
[script] build:source-admission · Admitting the sources of the build
[repository:command] still running after 350 s
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
end 2026-11-15T02:16:32Z
native_rc=1
```

## Setup tasks of try 2 (in log order; `✔` passed, `>` started without a result line; no `✖`, no nx summary because of the interrupt)


- ✔ @semio-tech/ui-rs:generate
- ✔ @semio-tech/assets:build
- ✔ @semio-tech/ui-styling-tokens:generate
- ✔ @semio-tech/framework-schema:generate
- > workspace:deps-js-all
- ✔ @semio-tech/dsl-derive-rs:generate-schema-index
- ✔ @semio-tech/repo-lib:cpp-setup
- ✔ repo:generator-inputs
- ✔ repo-mcp:build
- ✔ @semio-tech/plugin-registry:generate
- ✔ repo-mcp:install
- ✔ @semio-tech/ui-styling-py:deps
- ✔ @semio-tech/energy-oracle-py:deps
- > workspace:deps-go

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4493 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4493 MB because of its assembly phase
[capacity] run: sized at 59% of the default from 11 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 11 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5900 MB because of its assembly phase
[capacity] admitted dependencies with 4165 of 5900 MB: only 4165 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5900 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 6 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (5926 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4492 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5171 MB reserved (7520 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4492 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 11 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 11 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5901 MB because of its assembly phase
[capacity] admitted dependencies with 4946 of 5901 MB: only 4946 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5901 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 6 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7543 MB available, 2048 MB floor)
[capacity] run: reservation grown from 2048 MB to 2513 MB of the 12565 MB it asked for, after 0s: nothing else of ours is running (other runs of ours only wait to grow) and only that much memory is free (5908 MB available, 2048 MB floor); the run goes on with 1 of 8 job(s)
[capacity] run: sized at 198% of the default from 6 earlier run(s) of its class
```

## Before the run

    Pre-run facts, 2026-11-15T01:41:15Z
    - replica code digest (.cache/f/health.json): 52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f, store ok, daemon pid 77232 , health.json age 0 s
    - replica agent: state = running
    state = active
    state = active
    - graph service job: 41996 0 tech.semio.repo.service.19c193692817
    - repo service host processes:
        pid 41996 ppid 1 age 01:10:37 rss 61872
        pid 42029 ppid 41996 age 01:10:33 rss 206784
    - memory pressure level 1, swap total = 3072.00M  used = 2480.19M  free = 591.81M  (encrypted), free disk 82Gi

Launcher (raw: `round-13.launcher.txt`): unchanged before try 1 and after tries 1 and 2, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`.

## Files

`run-round-13.sh`, `round-13.monitor.txt`, `round-13.prerun.txt`, `round-13.launcher.txt`, `round-13.try-<k>.samples.txt`, `round-13.try-<k>.p12-samples.txt` (+ `.sample-*.txt`), `round-13.try-<k>.service.txt`, `round-13.tail.txt` (last 200 lines of try 2), `round-13.try-1.tail.txt`.
