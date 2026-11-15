# g1-m01 Round 13

## Before The Run (2026-11-15T01:40:43Z)

- Replica agent: `state = running`; kickstarts needed before the run: 0.
- Replica code digest (`.cache/f/health.json`, field `code`): `52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f`.
- Sampler of `ROUND-NOTE-P12.md`: started before every try and stopped after it, one file per try (`round-13.p12-try-<k>.txt`). One difference to the pasted block: it looks up the newest service log at every sample instead of once, because the service is replaced between tries on this machine.

## Result (written 2026-11-15T03:37:43Z)

- Start 2026-11-15T01:40:51Z, end 2026-11-15T03:34:50Z. Six tries done; the runner was then ended from outside (the agent session that hosted it stopped at about 03:35Z), not by a rule of the round. Rounds 14 to 17 were posted meanwhile and were not seen until 03:37Z; this report covers all six tries under the round 13 rules.
- Last `native_rc=75`; `.cache/f/native-bootstrap.done` absent; **not ready**. No try reached the setup tasks: succeeded, failed, not-run lists = none.
- Memory pressure:  196 pressure=1 . No hard stop of the runner, no stall stop. Peak resident size of a process of the run: 151728 KiB.
- Launcher: 12 records, distinct CDHash values: CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357 ; distinct link lines: 1.

## Tries

    01:40:51Z TRY 1 start, log /Users/ueli/native-bootstrap-round-13.log; service log host-ac6bfe8f337c.log from line 18261
    01:45:57Z TRY 1 end: native_rc=75; hard stop: none; failed tasks: none; reference: none
    01:49:01Z TRY 2 start, log /Users/ueli/native-bootstrap-round-13-try-2.log; service log host-007c265f8fa9.log from line 1928
    02:05:06Z TRY 2 end: native_rc=75; hard stop: none; failed tasks: none; reference: none
    02:08:08Z TRY 3 start, log /Users/ueli/native-bootstrap-round-13-try-3.log; service log host-007c265f8fa9.log from line 2852
    02:28:18Z TRY 3 end: native_rc=75; hard stop: none; failed tasks: none; reference: none
    02:31:20Z TRY 4 start, log /Users/ueli/native-bootstrap-round-13-try-4.log; service log host-3af1e67bb957.log from line 1263
    02:45:55Z TRY 4 end: native_rc=1; hard stop: none; failed tasks: none; reference: 675127cb
    02:48:58Z TRY 5 start, log /Users/ueli/native-bootstrap-round-13-try-5.log; service log host-3af1e67bb957.log from line 10939
    03:05:08Z TRY 5 end: native_rc=75; hard stop: none; failed tasks: none; reference: none
    03:08:10Z TRY 6 start, log /Users/ueli/native-bootstrap-round-13-try-6.log; service log host-79a74804a430.log from line 7517
    03:34:50Z TRY 6 end: native_rc=75; hard stop: none; failed tasks: none; reference: none

### Try 1

Bootstrap log, last message before `end` (verbatim):
```
Planning 1 targets.
The project graph could not be read on this machine yet: the plan stage environment grew to 6853 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T01:45:32Z
native_rc=75
```
Sampler `round-13.p12-try-1.txt`: 49 samples with a slice worker; closure entries before: 42533;closure entries after: 42533;
Sampler sample with the largest resident size (its `footprint` and progress line are in the same row):
```
2026-11-15T01:45:30Z pid=54243 rss=6606MB footprint=[bun [54243]: 64-bit    Footprint: 5996 MB (16384 bytes per page)] last=[graph: plan stage environment grew the plan by 4943 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory]
```

### Try 2

Bootstrap log, last message before `end` (verbatim):
```
Planning 1 targets.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 46 within 845 s, 657 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 8 more (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T02:04:52Z
native_rc=75
```
Sampler `round-13.p12-try-2.txt`: 183 samples with a slice worker; closure entries before: 42533;closure entries after: 42533;
Sampler sample with the largest resident size (its `footprint` and progress line are in the same row):
```
2026-11-15T01:50:47Z pid=59708 rss=3051MB footprint=[bun [59708]: 64-bit    Footprint: 467 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 703 projects arrived]
```

### Try 3

Bootstrap log, last message before `end` (verbatim):
```
Planning 1 targets.
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 703 and delivered 0 within 900 s, 703 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-surface-paint-rs (about 6950 MB), and 97 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T02:27:44Z
native_rc=75
```
Sampler `round-13.p12-try-3.txt`: 225 samples with a slice worker; closure entries before: 42533;closure entries after: 42533;
Sampler sample with the largest resident size (its `footprint` and progress line are in the same row):
```
2026-11-15T02:12:47Z pid=73970 rss=3542MB footprint=[bun [73970]: 64-bit    Footprint: 338 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 703 projects arrived]
```

### Try 4

Bootstrap log, last message before `end` (verbatim):
```
Planning 1 targets.
The resident service met an unexpected failure (reference 675127cb); try again, the details under that reference are in the service log /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-3af1e67bb957.log.
end 2026-11-15T02:45:44Z
native_rc=1
```
Sampler `round-13.p12-try-4.txt`: 166 samples with a slice worker; closure entries before: 42533;closure entries after: 41804;
Sampler sample with the largest resident size (its `footprint` and progress line are in the same row):
```
2026-11-15T02:42:30Z pid=91376 rss=6823MB footprint=[bun [91376]: 64-bit    Footprint: 3066 MB (16384 bytes per page)] last=[graph: inferring 48 projects in this process (identity)]
```

### Try 5

Bootstrap log, last message before `end` (verbatim):
```
Planning 1 targets.
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 7264 MB, more than the 6553 MB this machine gives a plan. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T03:04:41Z
native_rc=75
```
Sampler `round-13.p12-try-5.txt`: 180 samples with a slice worker; closure entries before: 41804;closure entries after: 41859;
Sampler sample with the largest resident size (its `footprint` and progress line are in the same row):
```
2026-11-15T02:59:59Z pid=5024 rss=5004MB footprint=[bun [5024]: 64-bit    Footprint: 3101 MB (16384 bytes per page)] last=[graph: waiting for memory headroom (131 of 960 MB reserved, 2 projects running)]
```

### Try 6

Bootstrap log, last message before `end` (verbatim):
```
Planning 1 targets.
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 267 and delivered 199 within 1495 s, 68 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-15T03:34:22Z
native_rc=75
```
Sampler `round-13.p12-try-6.txt`: 302 samples with a slice worker; closure entries before: 41859;closure entries after: 41859;
Sampler sample with the largest resident size (its `footprint` and progress line are in the same row):
```
2026-11-15T03:09:32Z pid=22794 rss=3312MB footprint=[bun [22794]: 64-bit    Footprint: 322 MB (16384 bytes per page)] last=[graph: waiting for the fleet: 0 of 267 projects arrived]
```

## What Each Try Shows

| try | native_rc | end | named in the refusal |
|---|---|---|---|
| 1 | 75 | plan too heavy | stage `environment`, 6853 MB of 6553 MB |
| 2 | 75 | fleet refusal | asked 703, delivered 46 in 845 s, 14 too heavy, 3512 MB per project, can infer 192 |
| 3 | 75 | fleet refusal | asked 703, delivered 0 in 900 s, 103 too heavy, **0 MB per project, can infer 0** (the fault round 17 names as fixed) |
| 4 | 1 | service failure 675127cb | stack below |
| 5 | 75 | plan too heavy | stage `inference (during inference)`, 7264 MB of 6553 MB |
| 6 | 75 | fleet refusal | asked 267, delivered 199 in 1495 s, 68 missing, can infer 0 |

The typed guard worked in tries 1 and 5: exit 75 and no pressure stop of the runner.

Try 4, service log `host-3af1e67bb957.log`:
```
2026-11-15T02:45:44.221Z request-failed 675127cb Error: ENOENT: closure-missing:cd3e7ecc745458cb0cf068d81a6f31595b0c5a5c42f8711468f6ae2ea46a4c62 build-input-closures leaf is not on this machine
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:87:451)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
```

## Service Log Lines of the Round

`round-13.service.txt` holds, for every service log written since round 12, each line with restore, `stale projects`, `taken from the fleet`, `inferred here`, `too heavy`, `plan stage`, `graph-plan-too-heavy`, `closure-missing`, `refreshing`, `graph-wanted`, `request-failed`, `shutdown`, `published` (consecutive repeats removed, prefix = line number). The restore, first plan and typed refusal lines:
```
# host-007c265f8fa9.log
146:graph: generation 211 (5101 projects) restored from the remote cache, 5105 documents (46 MB) downloaded in 12 s, from a state 26 min old with ? projects stale
1891:graph: plan stage inference took 44.7 s (managed 1632 MB, 554 MB more than at its start, resident 3177 MB)
1896:graph: plan stage environment grew the plan by 4943 MB, more than the 4915 MB this machine gives a plan; it is stopped before the host runs short of memory
1899:2026-11-15T01:45:32.179Z graph-plan-too-heavy {"stage":"environment","megabytes":6853,"limit":6553,"worker":{"heapMb":2441.7,"extraMb":1646.8,"objectCount":9630533,"objectTypes":[{"type":"string","count":4799063},{"type":"Array","count":1886639},{"type":"Object","count":933096},{"type":"Set","count":163226},{"type":"WeakRef","count":104190},{"type":"Structure","count":30625},{"type":"Promise","count":27882},{"type":"Cell 
2626:graph: 703 stale projects, 0 taken from the fleet, 14 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 8 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fl
2825:graph: nothing to wait for any longer: the fleet was asked for 703 stale projects and delivered 46 within 845 s; 657 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 8 more, which this machine m
# host-3af1e67bb957.log
1021:graph: 703 stale projects, 0 taken from the fleet, 103 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-surface-paint-rs (about 6950 MB), and 97 more; it can give about 0 MB per project), it can infer 0 of the others (0 
1232:graph: nothing to wait for any longer: the fleet was asked for 703 stale projects and delivered 0 within 900 s; 703 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), @semio-tech/framework-surface-paint-rs (about 6950 MB), and 9
1273:graph: generation 216 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 7 s, from a state 22 min old with 0 projects stale
2700:graph: plan stage inference took 22.6 s (managed 1458 MB, 510 MB more than at its start, resident 4106 MB)
2702:graph: plan stage environment took 0.7 s (managed 1506 MB, 47 MB more than at its start, resident 4109 MB)
2704:graph: plan stage tests took 0 s (managed 1506 MB, 0 MB more than at its start, resident 4109 MB)
2706:graph: plan stage policies took 0 s (managed 1506 MB, 0 MB more than at its start, resident 4109 MB)
2708:graph: plan stage dangling took 0 s (managed 1506 MB, 0 MB more than at its start, resident 4109 MB)
2710:graph: plan stage files took 0 s (managed 1506 MB, 0 MB more than at its start, resident 4109 MB)
2712:graph: plan stage test-edges took 0 s (managed 1506 MB, 0 MB more than at its start, resident 4109 MB)
2716:graph: plan stage edges took 0.3 s (managed 1510 MB, 4 MB more than at its start, resident 4109 MB)
2719:graph: plan stage metadata took 0 s (managed 1510 MB, 0 MB more than at its start, resident 4110 MB)
2721:graph: plan stage result took 0 s (managed 1510 MB, 0 MB more than at its start, resident 4110 MB)
5345:graph: plan stage inference took 302.6 s (managed 2544 MB, 809 MB more than at its start, resident 3914 MB)
5347:graph: plan stage environment took 0.5 s (managed 2549 MB, 5 MB more than at its start, resident 3950 MB)
5349:graph: plan stage tests took 0 s (managed 2549 MB, 0 MB more than at its start, resident 3950 MB)
5351:graph: plan stage policies took 0 s (managed 2549 MB, 0 MB more than at its start, resident 3950 MB)
5353:graph: plan stage dangling took 0 s (managed 2549 MB, 0 MB more than at its start, resident 3950 MB)
5355:graph: plan stage files took 0 s (managed 2549 MB, 0 MB more than at its start, resident 3951 MB)
5357:graph: plan stage test-edges took 0 s (managed 2549 MB, 0 MB more than at its start, resident 3951 MB)
5362:graph: plan stage edges took 1.5 s (managed 2609 MB, 60 MB more than at its start, resident 3894 MB)
5365:graph: plan stage metadata took 0 s (managed 2609 MB, 0 MB more than at its start, resident 3894 MB)
5367:graph: plan stage result took 0 s (managed 2609 MB, 0 MB more than at its start, resident 3894 MB)
6783:graph: plan stage inference took 80.4 s (managed 2946 MB, 404 MB more than at its start, resident 4262 MB)
6785:graph: plan stage environment took 0.6 s (managed 2991 MB, 44 MB more than at its start, resident 4273 MB)
6787:graph: plan stage tests took 0 s (managed 2991 MB, 0 MB more than at its start, resident 4273 MB)
6789:graph: plan stage policies took 0 s (managed 2991 MB, 0 MB more than at its start, resident 4273 MB)
6791:graph: plan stage dangling took 0 s (managed 2991 MB, 0 MB more than at its start, resident 4273 MB)
6793:graph: plan stage files took 0 s (managed 2991 MB, 0 MB more than at its start, resident 4273 MB)
6795:graph: plan stage test-edges took 0 s (managed 2991 MB, 0 MB more than at its start, resident 4273 MB)
6800:graph: plan stage edges took 0.3 s (managed 2995 MB, 4 MB more than at its start, resident 4273 MB)
6803:graph: plan stage metadata took 0 s (managed 2995 MB, 0 MB more than at its start, resident 4273 MB)
6805:graph: plan stage result took 0 s (managed 2995 MB, 0 MB more than at its start, resident 4273 MB)
7939:graph: plan stage inference took 96.2 s (managed 4330 MB, 1318 MB more than at its start, resident 5188 MB)
7941:graph: plan stage environment took 0.5 s (managed 4337 MB, 7 MB more than at its start, resident 5192 MB)
7943:graph: plan stage tests took 0 s (managed 4337 MB, 0 MB more than at its start, resident 5192 MB)
7945:graph: plan stage policies took 0 s (managed 4337 MB, 0 MB more than at its start, resident 5192 MB)
7947:graph: plan stage dangling took 0 s (managed 4337 MB, 0 MB more than at its start, resident 5192 MB)
7949:graph: plan stage files took 0 s (managed 4337 MB, 0 MB more than at its start, resident 5192 MB)
7951:graph: plan stage test-edges took 0 s (managed 4337 MB, 0 MB more than at its start, resident 5192 MB)
7955:graph: plan stage edges took 0.2 s (managed 4346 MB, 9 MB more than at its start, resident 5193 MB)
7958:graph: plan stage metadata took 0 s (managed 4346 MB, 0 MB more than at its start, resident 5193 MB)
7960:graph: plan stage result took 0 s (managed 4346 MB, 0 MB more than at its start, resident 5193 MB)
8822:graph: plan stage inference took 27.7 s (managed 3472 MB, -918 MB more than at its start, resident 3887 MB)
8824:graph: plan stage environment took 0.5 s (managed 3480 MB, 7 MB more than at its start, resident 3905 MB)
8826:graph: plan stage tests took 0 s (managed 3480 MB, 0 MB more than at its start, resident 3906 MB)
8828:graph: plan stage policies took 0 s (managed 3480 MB, 0 MB more than at its start, resident 3906 MB)
8830:graph: plan stage dangling took 0 s (managed 3480 MB, 0 MB more than at its start, resident 3906 MB)
8832:graph: plan stage files took 0 s (managed 3480 MB, 0 MB more than at its start, resident 3907 MB)
8834:graph: plan stage test-edges took 0 s (managed 3480 MB, 0 MB more than at its start, resident 3907 MB)
8838:graph: plan stage edges took 0.2 s (managed 3482 MB, 2 MB more than at its start, resident 3904 MB)
8841:graph: plan stage metadata took 0 s (managed 3482 MB, 0 MB more than at its start, resident 3904 MB)
8843:graph: plan stage result took 0 s (managed 3482 MB, 0 MB more than at its start, resident 3904 MB)
9871:graph: plan stage inference took 70.6 s (managed 3786 MB, 204 MB more than at its start, resident 4538 MB)
9873:graph: plan stage environment took 0.6 s (managed 3832 MB, 46 MB more than at its start, resident 4521 MB)
9879:graph: plan stage tests took 4.1 s (managed 4377 MB, 545 MB more than at its start, resident 4532 MB)
9881:graph: plan stage policies took 0.2 s (managed 4377 MB, 0 MB more than at its start, resident 4532 MB)
9883:graph: plan stage dangling took 0 s (managed 4377 MB, 0 MB more than at its start, resident 4532 MB)
9885:graph: plan stage files took 0 s (managed 4377 MB, 0 MB more than at its start, resident 4532 MB)
9887:graph: plan stage test-edges took 0 s (managed 4377 MB, 0 MB more than at its start, resident 4532 MB)
9892:graph: plan stage edges took 3.8 s (managed 3875 MB, -502 MB more than at its start, resident 4551 MB)
9895:graph: plan stage metadata took 0 s (managed 3875 MB, 0 MB more than at its start, resident 4551 MB)
9897:graph: plan stage result took 0 s (managed 3875 MB, 0 MB more than at its start, resident 4551 MB)
10693:graph: plan stage inference took 31.2 s (managed 4021 MB, 125 MB more than at its start, resident 4361 MB)
10695:graph: plan stage environment took 0.4 s (managed 4026 MB, 5 MB more than at its start, resident 4387 MB)
10697:graph: plan stage tests took 0 s (managed 4026 MB, 0 MB more than at its start, resident 4387 MB)
10699:graph: plan stage policies took 0 s (managed 4026 MB, 0 MB more than at its start, resident 4387 MB)
10701:graph: plan stage dangling took 0 s (managed 4026 MB, 0 MB more than at its start, resident 4387 MB)
10703:graph: plan stage files took 0 s (managed 4026 MB, 0 MB more than at its start, resident 4387 MB)
10705:graph: plan stage test-edges took 0 s (managed 4026 MB, 0 MB more than at its start, resident 4387 MB)
10710:graph: plan stage edges took 4.5 s (managed 4202 MB, 176 MB more than at its start, resident 4595 MB)
10713:graph: plan stage metadata took 0 s (managed 4202 MB, 0 MB more than at its start, resident 4596 MB)
10715:graph: plan stage result took 0 s (managed 4202 MB, 0 MB more than at its start, resident 4596 MB)
10908:[graph] closure-missing build-input-closures/cd3e7ecc745458cb0cf068d81a6f31595b0c5a5c42f8711468f6ae2ea46a4c62 named by test-framework-products-os-modules-plugin-host-809d0a, test-framework-products-repo-modules-library-caching-f1ba7f, test-framework-products-repo-modules-library-discovery-7a00f6
10909:2026-11-15T02:45:44.221Z request-failed 675127cb Error: ENOENT: closure-missing:cd3e7ecc745458cb0cf068d81a6f31595b0c5a5c42f8711468f6ae2ea46a4c62 build-input-closures leaf is not on this machine
# host-79a74804a430.log
146:graph: generation 216 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s, from a state 40 min old with 0 projects stale
1835:graph: plan stage inference took 39.9 s (managed 1665 MB, 543 MB more than at its start, resident 3685 MB)
1838:graph: plan stage environment took 3.7 s (managed 1802 MB, 137 MB more than at its start, resident 3694 MB)
1840:graph: plan stage tests took 0 s (managed 1802 MB, 0 MB more than at its start, resident 3694 MB)
1842:graph: plan stage policies took 0 s (managed 1802 MB, 0 MB more than at its start, resident 3694 MB)
1844:graph: plan stage dangling took 0 s (managed 1802 MB, 0 MB more than at its start, resident 3694 MB)
```

## Files

`round-13.samples.txt`, `round-13.launcher.txt`, `round-13.service.txt`, `round-13.p12-try-{1..6}.txt` (whole sampler output; `.sample-*.txt` for tries 1, 4, 5 where the worker passed 4 GB), `round-13.tail.txt` (last 200 lines of the try 6 log); logs `~/native-bootstrap-round-13.log`, `~/native-bootstrap-round-13-try-{2..6}.log`.
