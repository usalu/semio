# g1-m03 Rounds 14 to 18: one series of twelve tries (files named `round-14.*`)

Written 2026-11-15 17:10Z, late: the runner finished try 12 at 07:59:56Z, the agent session ended before it could report and was restarted at about 17:00Z. Rounds 19 to 29 were not run here. The series began under round 14 (02:23Z), round 15 (level 2 no stop), round 16 (step 0), round 17 and round 18 were applied while it ran; rules of rounds 19 to 23 (posted 04:05Z to 05:44Z) were NOT applied by the runner, which matters for tries 7 and 8 (see there).

- Machine: Mac mini M4, 16 GB, macOS 27.2.1. Worktree `/Users/ueli/Documents/semio`.
- Result: 12 tries, none with `native_rc=0`, marker absent, `ready` false. Step 0 of round 16 (`repo tools setup all`) failed once (exit 1, `round-16.md`).
- **Try 6 ran `workspace:setup` to the end (87 min of nx): 13 tasks passed, 3 failed, 1 not run.** Failed: `workspace:deps-build-tools`, `@semio-tech/framework-os-mcp-rs:build`, `workspace:deps-cargo`.
- Launcher unchanged during the whole series (CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`, raw: `round-14.launcher.txt`).

## Tries

| Try | Start | End | `native_rc` | Outcome |
|---|---|---|---|---|
| 1 | 02:23:11Z | 02:42Z | 75 | fleet refusal: asked 569, delivered 0 within 902 s; `it can infer only 0`; 208 too heavy |
| 2 | 02:45:21Z | 02:47Z | 75 | `the fleet cache cannot be reached` (657 stale, could infer 72), 0 s |
| 3 | 02:50:29Z | 02:52Z | 75 | `the fleet cache cannot be reached` (657 stale, could infer 0), 0 s |
| 4 | 02:55:38Z | 03:22Z | 75 | fleet refusal: asked 657, delivered 144 within 1495 s, 513 missing; `it can infer only 0`; 226 too heavy |
| 5 | 03:52:07Z | 04:06:35Z | 1 | graph read 267 of 267 owners, then `closure-missing … leaf is not on this machine` (reference 51f8a5ee, named by `@semio-tech/framework-os-mcp`); `round-14.try-5.failure.txt` |
| 6 | 04:09:51Z | 06:20:55Z | 1 | **setup tasks ran to the end**: 13 passed, 3 failed, 1 not run (below) |
| 7 | 06:24:25Z | 06:46:37Z | 130 (SIGINT of the runner) | stall rule of round 14 fired during `waiting for the fleet: 0 of 1203`; under round 22 (posted 05:24Z, not yet in the runner) this wait was healthy and should NOT have been interrupted |
| 8 | 06:50:36Z | 07:06:15Z | 130 (SIGINT of the runner) | the same: interrupted during `waiting for the fleet: 0 of 1203`; not a result |
| 9 | 07:09:55Z | 07:25:44Z | 75 | `graph-plan-too-heavy`: plan stage inference grew to 7674 MB (limit 6553 MB), 3414 MB available |
| 10 | 07:29:17Z | 07:37:35Z | 75 | `graph-plan-too-heavy`: plan stage inference grew to 8895 MB (limit 6553 MB), 2788 MB available |
| 11 | 07:41:00Z | 07:50:46Z | 1 | graph service denied access to the checkout although this process can read it; restarted once under the access of this process; the second start failed too |
| 12 | 07:54:15Z | 07:59:38Z | 1 | `The graph service could not answer (lost …)`, `Internal error: Relative filesystem fstatat failed (EPERM)` |

Waits between tries: 180 s. Tries 7 and 8 are a mistake of this worker's runner against rounds 22 (and 19 to 23 in general, which arrived while no agent was reading `ROUND.md`); they say nothing about the build.

## Last message of every try (bootstrap log, verbatim)

```
try 1: The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 569 and delivered 0 within 902 s, 569 are still missing, among them the too heavy project(s) @semio-tech/framework-os-dev (about 39020 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-plugin (about 7894 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), and 202 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 2: The project graph could not be read on this machine: it can infer only 72 of the stale projects, the fleet was asked for 657 and delivered 0 within 0 s, 657 are still missing, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more (this machine can give about 264 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 3: The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 657 and delivered 0 within 0 s, 657 are still missing, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 4: The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 657 and delivered 144 within 1495 s, 513 are still missing, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/s-dev-composition (about 3702 MB), @semio-tech/cargo-workspaces (about 2950 MB), @semio-tech/framework-compiler-rs (about 2884 MB), @semio-tech/framework-mesh-engine-rs (about 2662 MB), and 94 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 5: The resident service met an unexpected failure (reference 51f8a5ee); try again, the details under that reference are in the service log /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-c5f530f58f3a.log.
try 6: Internal error: Owned Nx invocation failed (130): nx run workspace:setup --parallel=1
try 7: Cancelled (SIGINT); leases released.
try 8: Cancelled (SIGINT); leases released.
try 9: The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 7674 MB, more than the 6553 MB this machine gives a plan, and only 3414 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 10: The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 8895 MB, more than the 6553 MB this machine gives a plan, and only 2788 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 11: The graph service was denied access to the checkout although this process can read it, so this is not a missing file access consent of this session; a service that was started for another route or another app is judged against another identity. The service ended and was started once more under the access of this process; the second one was denied as well. macOS denied the resident service access to /Users/ueli/Documents/semio/.cache/b/.reset.json (your Documents folder) while it answered a request (reference da146983; the service was started as route direct, pid 60693, parent pid 60647). The service has stopped itself and never serves without access. If the command started the service again under your own session and this message returns, send the line that starts with `where:` below reference da146983 in /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-74145656f35a.log. Otherwise switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-74145656f35a.log.
try 12: The graph service could not answer (lost the service was killed without an exit record (its cgroup is gone) [log: [script] build:implementation-source · Reading the sources of the build implementation | Internal error: Relative filesystem fstatat failed (EPERM) | repository:command failed with exit code 1]); look at `repo service host status` and try again
```

## Graph facts per try (graph service logs, verbatim, each distinct line once; from `round-14.try-<k>.service.txt`)

### Try 1

```
graph: 569 stale projects, 0 taken from the fleet, 208 too heavy for this machine (@semio-tech/framework-os-dev (about 39020 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-plugin (about 7894 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), and 202 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 569
graph: nothing to wait for any longer: the fleet was asked for 569 stale projects and delivered 0 within 902 s; 569 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 39020 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-plugin (about 7894 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), and 202 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: nothing to wait for any longer: the fleet was asked for 569 stale projects and delivered 0 within 902 s; 569 remain, among them the too heavy project(s) @semio-tech/framework-os-dev (about 39020 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-plugin (about 7894 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), and 202 more, which this machine must never infer itself (it can give about 0 MB per project); t
waiting for the fleet, largest x: 0 of 569
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 3694 MB, largest footprint 1650 MB; closure entries before: 14248;closure entries after: 14248;
30 s samples: 38, pressure level other than 1: 0, largest process 3782736 KiB pid 16453 ppid 16185
```

### Try 2

```
graph: 657 stale projects, 0 taken from the fleet, 187 too heavy for this machine (@semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more; it can give about 264 MB per project), it can infer 72 of the others (18 per wave); asking the fleet for 657
graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 0 within 0 s; 657 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more, which this machine must never infer itself (it can give about 264 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: the fleet cache cannot be reached, so 657 stale projects cannot be requested from it; this machine can infer 72
graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 0 within 0 s; 657 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more, which this machine must never infer itself (it can g
waiting for the fleet, largest x: 
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 3375 MB, largest footprint 561 MB; closure entries before: 14248;closure entries after: 14248;
30 s samples: 4, pressure level other than 1: 0, largest process 3416128 KiB pid 32346 ppid 32233
```

### Try 3

```
graph: 657 stale projects, 0 taken from the fleet, 187 too heavy for this machine (@semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 657
graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 0 within 0 s; 657 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: the fleet cache cannot be reached, so 657 stale projects cannot be requested from it; this machine can infer 0
graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 0 within 0 s; 657 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), and 181 more, which this machine must never infer itself (it can g
waiting for the fleet, largest x: 
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 3454 MB, largest footprint 765 MB; closure entries before: 14248;closure entries after: 14248;
30 s samples: 4, pressure level other than 1: 0, largest process 3078832 KiB pid 34977 ppid 32233
```

### Try 4

```
2026-11-15T02:56:13.900Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
graph: 657 stale projects, 0 taken from the fleet, 226 too heavy for this machine (@semio-tech/framework-os-dev (about 39020 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-plugin (about 7894 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), and 220 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 657
graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 144 within 1495 s; 513 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/s-dev-composition (about 3702 MB), @semio-tech/cargo-workspaces (about 2950 MB), @semio-tech/framework-compiler-rs (about 2884 MB), @semio-tech/framework-mesh-engine-rs (about 2662 MB), and 94 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 144 within 1495 s; 513 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/s-dev-composition (about 3702 MB), @semio-tech/cargo-workspaces (about 2950 MB), @semio-tech/framework-compiler-rs (about 2884 MB), @semio-tech/framework-mesh-engine-rs (about 2662 MB), and 94 more, which this machine must never infer itself (
waiting for the fleet, largest x: 144 of 657
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 3011 MB, largest footprint 359 MB; closure entries before: 14248;closure entries after: 14248;
30 s samples: 53, pressure level other than 1: 0, largest process 3083968 KiB pid 38243 ppid 38143
```

### Try 5

```
graph: generation 221 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 10 s, from a state 10 min old with 0 projects stale
[graph] closure-missing build-input-closures/5d52130ef60a0c3016d05460bad79e740c2d0bca1c1a6c1f151da50830ba608a named by @semio-tech/framework-os-mcp
2026-11-15T04:06:35.786Z request-failed 51f8a5ee Error: ENOENT: closure-missing:5d52130ef60a0c3016d05460bad79e740c2d0bca1c1a6c1f151da50830ba608a build-input-closures leaf is not on this machine
waiting for the fleet, largest x: 
plan stage lines: 100; largest growth: 1499 MB in: inference took 211.6 s
P12 sampler: largest resident 5019 MB, largest footprint 4718 MB; closure entries before: 14248;closure entries after: 13034;
30 s samples: 29, pressure level other than 1: 0, largest process 5139920 KiB pid 72876 ppid 72738
```

### Try 6

```
graph: generation 221 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s, from a state 31 min old with 0 projects stale
2026-11-15T04:46:46.137Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=88966 ppid=88925
2026-11-15T04:46:54.511Z shutdown settled
graph: 277 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/framework-process (about 9854 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), @semio-tech/ui-contract-builder-rs (about 4454 MB), @semio-tech/repo-lib (about 4394 MB), @semio-tech/s-dev-composition (about 3702 MB); it can give about 3512 MB per project), it can infer 48 of the others (12 per wave); asking the fleet for 277
graph: nothing to wait for any longer: the fleet was asked for 277 stale projects and delivered 0 within 300 s; 277 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/ui-contract-identity-rs (about 4894 MB), @semio-tech/ui-contract-builder-rs (about 4454 MB), @semio-tech/repo-lib (about 4394 MB), @semio-tech/s-dev-composition (about 3702 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-15T04:47:05.941Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=7003 ppid=6933
graph: 104 stale projects, 0 taken from the fleet, 8 too heavy for this machine (@semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/plugin-registry (about 4576 MB), @semio-tech/repo-lib (about 4394 MB), and 2 more; it can give about 3512 MB per project), it can infer 68 of the others (17 per wave); asking the fleet for 104
graph: nothing to wait for any longer: the fleet was asked for 104 stale projects and delivered 0 within 301 s; 104 remain, among them the too heavy project(s) @semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/plugin-registry (about 4576 MB), @semio-tech/repo-lib (about 4394 MB), and 2 more, which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: 633 stale projects, 0 taken from the fleet, 9 too heavy for this machine (@semio-tech/framework-process (about 9854 MB), @semio-tech/framework-renderer-wgpu (about 7348 MB), @semio-tech/framework-os-mcp (about 5530 MB), @semio-tech/framework-os (about 5424 MB), @semio-tech/ui-rs (about 5398 MB), @semio-tech/plugin-registry (about 4576 MB), and 3 more; it can give about 3512 MB per project), it can infer 36 of the others (9 per wave); asking the fleet for 633
graph: nothing to wait for any longer: asked the fleet for 633 stale projects and received 0 within 300 s, 633 remain and this machine can infer only 36; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: 648 stale projects, 0 taken from the fleet, 9 too heavy for this machine (@semio-tech/framework-process (about 9476 MB), @semio-tech/framework-renderer-wgpu (about 6980 MB), @semio-tech/ui-rs (about 5138 MB), @semio-tech/ui-contract-identity-rs (about 4658 MB), @semio-tech/plugin-registry (about 4346 MB), @semio-tech/ui-contract-builder-rs (about 4240 MB), and 3 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 648
graph: plan stage inference settled from 6635 to 6906 MB
waiting for the fleet, largest x: 0 of 650
plan stage lines: 246; largest growth: 1032 MB in: inference took 93.4 s
P12 sampler: largest resident 5899 MB, largest footprint 7628 MB; closure entries before: 13034;closure entries after: 14192;
30 s samples: 256, pressure level other than 1: 23, largest process 4797120 KiB pid 17550 ppid 17351
```

### Try 7

```
graph: 1203 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 1203
graph: generation 232 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 68 s, from a state 31 min old with 141 projects stale
[graph] closure-missing 7143a7deaa3ba919893aefa5354df74795ac63f70f9e90262dc144994f2965bb named by 70 projects: re-derived
[graph] closure-missing a32e8e410630d55255504482814fd86de14118524a0dcbeb5b8c2db68f2a9e5b named by 70 projects: re-derived
[graph] closure-missing f828673b2db20f132aba65197c4f2af79ff125e2c9c42c8f2dbf409911540a6d named by 1 projects: re-derived
[graph] closure-missing f8361925b3100b32ee3815a6faebee0ac1ffdf8b2640c9e25ff7f8bee8dc0298 named by 1 projects: re-derived
[graph] closure-missing f836d19fdc47d953f4e316d82126ce04ae97cec03462d289be680a179ce2ad91 named by 1 projects: re-derived
[graph] closure-missing f839dd5fcf08344d93f91277424907b08eeddaf4189a272fe8cb7c9b73012382 named by 1 projects: re-derived
[graph] closure-missing f83acbae1c1c0c1286e4f00e761aead07fc623b4eb8d00cdd14835f4e8f42f56 named by 1 projects: re-derived
[graph] closure-missing f83c0a3b73a4540119d60258df670aed4cac7823344a7945842c5d7e6a4db20e named by 1 projects: re-derived
[graph] closure-missing f83e88b6f288e8aaa8c1c4a9b3ee26f0fb32196a7f94eecac47884b03670cb79 named by 1 projects: re-derived
[graph] closure-missing f844baf71e873242dae6334e392fb04e5690883d9bcf427e382bc75a37894cf9 named by 1 projects: re-derived
[graph] closure-missing f844bce21308a1e930b9d242e8d890a2a3d6d55614082d129fff3fee472f60f8 named by 1 projects: re-derived
[graph] closure-missing f84985e9b90d948cfa4171b1f9dc51d2b3a3864078985255e349d2643dc1170f named by 1 projects: re-derived
waiting for the fleet, largest x: 0 of 1203
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 2895 MB, largest footprint 1761 MB; closure entries before: 14192;closure entries after: 14192;
30 s samples: 45, pressure level other than 1: 0, largest process 3038224 KiB pid 68477 ppid 68476
```

### Try 8

```
graph: 1203 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 1203
graph: 0 of 1203 projects arrived from the fleet, 0 inferred here, 1203 left
waiting for the fleet, largest x: 
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 3254 MB, largest footprint 2770 MB; closure entries before: 14192;closure entries after: 14192;
30 s samples: 32, pressure level other than 1: 0, largest process 3211920 KiB pid 5442 ppid 27268
```

### Try 9

```
graph: plan stage inference settled from 6728 to 6903 MB
2026-11-15T07:25:42.788Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":7674,"limit":6553,"available":3414,"reserve":2048,"reason":"pressure","worker":{"heapMb":804.9,"extraMb":578.1,"objectCount":4259651,"objectTypes":[{"type":"string","count":1961559},{"type":"Object","count":1116864},{"type":"Array","count":372073},{"type":"WeakRef","count":132537},{"type":"Cell Butterfly","count":56477},{"type":"Structure","count":35814},{"type":"SparseArrayValueMap","count":27517},{"type":"Function","count":26743}],"at":1794727351062}}
graph: 0 of 1203 projects arrived from the fleet, 0 inferred here, 1203 left
graph: 0 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 50 inferred here
graph: 192 stale projects prepared by this command in 274 s
graph: 0 of 1203 projects arrived from the fleet, 192 inferred here, 1011 left
graph: plan stage ? settled from 6924 to 5661 MB
2026-11-15T07:15:40.659Z graph-plan-too-heavy {"stage":"?","operation":"?","megabytes":8770,"limit":6553,"available":2803,"reserve":2048,"reason":"pressure","worker":{"heapMb":3927.9,"extraMb":2819.8,"objectCount":26485612,"objectTypes":[{"type":"string","count":13819659},{"type":"Object","count":8609659},{"type":"Array","count":1732252},{"type":"Set","count":159378},{"type":"WeakRef","count":102126},{"type":"Structure","count":64182},{"type":"Cell Butterfly","count":25078},{"type":"Promise","count":24894}],"at":1794726882942}}
2026-11-15T07:15:40.659Z graph-plan-too-heavy {"stage":"?","operation":"?","megabytes":8770,"limit":6553,"available":2803,"reserve":2048,"reason":"pressure","worker":{"heapMb":3927.9,"extraMb":2819.8,"objectCount":26485612,"objectTypes":[{"type":"string","count":13819659},{"type":"Object","count":8609659},{"type":"Array","count":1732252},{"type":"Set","count":159378},{"type":"WeakRef","count":102126},{"type":"Structure","count":64182},{"type":"Cell Butterfly","count":25078},{"type":"Promise","co
waiting for the fleet, largest x: 
plan stage lines: 6; largest growth: 
P12 sampler: largest resident 7311 MB, largest footprint 8768 MB; closure entries before: 13497;2026-11-15T07:13:29Z pid=5442 rss=3456MB footprint=[bun [5442]: 64-bit    Footprint: 2239 MB (16384 bytes per page)] last=[graph: 0 projects taken from the fleet with 0 closure entries, 0 of them did not hold here and were inferred again, 0 not shared, 50 inferred here];closure entries after: 136;
30 s samples: 32, pressure level other than 1: 2, largest process 6062704 KiB pid 5442 ppid 27268
```

### Try 10

```
2026-11-15T07:31:03.297Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":8212,"limit":6553,"available":2357,"reserve":2048,"reason":"pressure","worker":{"heapMb":3676.1,"extraMb":3022.7,"objectCount":14923590,"objectTypes":[{"type":"string","count":8417524},{"type":"Object","count":5269246},{"type":"Array","count":675539},{"type":"Uint8Array","count":624470},{"type":"WeakRef","count":84568},{"type":"Structure","count":56860},{"type":"Function","count":24909},{"type":"FunctionExecutable","count":20915}],"at":1794727826657}}
graph: plan stage inference settled from 6594 to 4577 MB
2026-11-15T07:37:34.647Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":8895,"limit":6553,"available":2788,"reserve":2048,"reason":"pressure","worker":{"heapMb":2249,"extraMb":1792.6,"objectCount":10132654,"objectTypes":[{"type":"string","count":8331934},{"type":"Object","count":6393769},{"type":"Array","count":875716},{"type":"WeakRef","count":66663},{"type":"Structure","count":46887},{"type":"Function","count":23766},{"type":"FunctionExecutable","count":20231},{"type":"UnlinkedFunctionExecutable","count":20198}],"at":1794728191631}}
2026-11-15T07:37:34.647Z graph-plan-too-heavy {"stage":"inference","operation":"inference","megabytes":8895,"limit":6553,"available":2788,"reserve":2048,"reason":"pressure","worker":{"heapMb":2249,"extraMb":1792.6,"objectCount":10132654,"objectTypes":[{"type":"string","count":8331934},{"type":"Object","count":6393769},{"type":"Array","count":875716},{"type":"WeakRef","count":66663},{"type":"Structure","count":46887},{"type":"Function","count":23766},{"type":"FunctionExecutable","count":20231},{"
waiting for the fleet, largest x: 
plan stage lines: 3; largest growth: 
P12 sampler: largest resident 7562 MB, largest footprint 8325 MB; closure entries before: 269;closure entries after: 513;
30 s samples: 17, pressure level other than 1: 1, largest process 6611248 KiB pid 35425 ppid 24921
```

### Try 11

```
2026-11-15T07:49:33.591Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=56873 ppid=56801
2026-11-15T07:50:46.464Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=direct pid=60693 ppid=60647
waiting for the fleet, largest x: 
plan stage lines: 0; largest growth: 
P12 sampler: largest resident 316 MB, largest footprint 171 MB; closure entries before: 513;closure entries after: 638;
30 s samples: 20, pressure level other than 1: 1, largest process 8299392 KiB pid 56085 ppid 52434
```

### Try 12

```
waiting for the fleet, largest x: 
plan stage lines: 0; largest growth: 
P12 sampler: largest resident  MB, largest footprint  MB; closure entries before: 638;closure entries after: 789;
30 s samples: 11, pressure level other than 1: 1, largest process 7311936 KiB pid 56085 ppid 52434
```

## Try 6: setup tasks (`nx run workspace:setup --parallel=1`, run duration 87m 9s, cache 0/26)

In log order (`✔` passed, `>` started):


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
- ✔ @semio-tech/repo-test-dotnet:deps
- > workspace:deps-wasm-opt
- ✔ @semio-tech/framework-graph:generate
- > workspace:deps-build-tools
- > workspace:setup-git
- > workspace:deps-browsers
- > @semio-tech/framework-os-mcp-rs:build
- > workspace:deps-cpp
- > workspace:deps-python
- > workspace:deps-dotnet
- > workspace:deps-cargo
- > workspace:deps-trunk
- > workspace:deps-wasm
- > workspace:deps-tools

nx summary (verbatim):

```
Tasks not run because their dependencies failed or --nx-bail=true:

- workspace:setup:prototype

Failed tasks:

- workspace:deps-build-tools:prototype
- @semio-tech/framework-os-mcp-rs:build:prototype
- workspace:deps-cargo:prototype


Output of 14 successful tasks were not shown. Run with --verbose or --output-style=static to see it.
Hint: run the command with --verbose for more details.

  Run duration:      87m 9s
  Cache:             0/26 hit (0%)
  Critical path:     44m 34s (1 task)
  Recoverable time:  42m 34s (49% of the run)

  Recommendation: Drastically reduce your run duration by sharing a cache across your team and CI → https://cloud.nx.app/connect/gJ89SARELF.
Nx run failed: 3 task(s) did not succeed after 0 rerun(s) of the whole run.
- workspace:deps-build-tools:prototype:  (another rerun is unlikely to help: the task itself failed)
- @semio-tech/framework-os-mcp-rs:build:prototype:  (another rerun is unlikely to help: the task itself failed)
- workspace:deps-cargo:prototype: 79 |  let resolved=0;for(const input of cargoPreparationResolutionBatchesV1(row.reso (another rerun is unlikely to help: the task itself failed)
Not run because a task they depend on failed (1): workspace:setup:prototype
Internal error: Owned Nx invocation failed (130): nx run workspace:setup --parallel=1
```

Passed in try 6 and failed in round 5: `@semio-tech/repo-lib:cpp-setup`, `@semio-tech/plugin-registry:generate`. `workspace:deps-build-tools` no longer stands still on the dependency store: it printed once

```
Process 34100 (d79e5ded682226cd1dd93558560308e76fa0533e83f30b1c9276ad81aca54d8b run workspace:setup --parallel=1) holds the dependency store since 2026-11-15T04:49:26.655Z and the funding is already current; continuing under that hold without exclusive access
```

and then failed. First error line with the 15 lines before it, per failed task:

#### `workspace:deps-build-tools`

```
wasm-bindgen 0.2.129 already matches its pin.
trunk 0.21.14 already matches its pin.
cargo-nextest 0.9.140 already matches its pin.
cargo-llvm-cov 0.8.7 already matches its pin.
chromium 1248 already matches its pin.
Preparing compiler-helpers: The memory-admitting rustc scheduler and the unit-cache compiler wrapper (a native executable on Windows) that every Cargo build starts with.
[repository:command] still running after 20 s
[repo] still working after 3290 s; last phase started: source.served
compiler-helpers is ready.
Undeclared Repo-to-Nx child selection: ["nx","--version"]
191 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
192 |     if(auxiliaryFailed)throw auxiliaryFailure;
193 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
194 |     if (stopped&&stopped!=="output delivery refused") throw stopRefusal ?? new Error(`${label} stopped: ${stopped}`);
195 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
196 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
```

#### `@semio-tech/framework-os-mcp-rs:build (first error)`

```
   |                        |
   |                        argument has type `std::option::Option<pack::record::RetainedRecordBodyToken>`
   |
help: use `let _ = ...` to ignore the expression or result
   |
34 -   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:usize::from(self.phase==Phase::Bytes),..Default::default()};self.phase=next;self.field=field;self.seen=seen;self.revision_at=revision_at;drop(pending.take());self.receipt=Some((grant,progress));Ok(Some(progress))
34 +   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:usize::from(self.phase==Phase::Bytes),..Default::default()};self.phase=next;self.field=field;self.seen=seen;self.revision_at=revision_at;let _ = pending.take();self.receipt=Some((grant,progress));Ok(Some(progress))
   |

warning: `semio-framework-tool-run-tick-wire` (lib) generated 7 warnings (run `cargo fix --lib -p semio-framework-tool-run-tick-wire` to apply 4 suggestions)
   Compiling pulley-macros v47.0.4
   Compiling allocator-api2 v0.2.21
   Compiling fnv v1.0.7
   Compiling hashbrown v0.16.1
   Compiling gimli v0.33.0
error[E0460]: found possibly newer version of crate `hashbrown` which `indexmap` depends on
```

#### `@semio-tech/framework-os-mcp-rs:build (Cargo artifact build failed)`

```
[cargo-provenance] reading build records: 144623 file operation(s), 3033403485 of 3033403485 bytes
[cargo-provenance] reading build records: 152253 file operation(s), 3699558060 of 3699558060 bytes
[cargo-provenance] reading build records: 155397 file operation(s), 4655213880 of 4655213880 bytes
[cargo-provenance] reading build records: 158240 file operation(s), 5698379761 of 5698379761 bytes
[cargo-provenance] reading build records: 165327 file operation(s), 6001737497 of 6001737497 bytes
[cargo-provenance] reading build records: 180357 file operation(s), 6031554931 of 6031554931 bytes
[native:owner-command] still running after 1460 s
[cargo-provenance] reading build records: 193121 file operation(s), 6058234488 of 6058234488 bytes
985 |       await diagnosticsDrained;
986 |       diagnosticsOpen = false;
987 |       closeSync(diagnosticsLog);
988 |       for (const row of diagnostics.finish(compilerStatus !== 0 || cancelled, diagnosticsPath)) process.stderr.write(`${row}\n`);
989 |       if (compilerStatus === 0 && !cancelled && diagnostics.suppressed === 0) rmSync(diagnosticsPath, { force: true });
990 |       if (compilerStatus !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
                                                             ^
error: Cargo artifact build failed: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml
```

#### `workspace:deps-cargo`

```
[repo] still working after 7521 s; last phase started: source.served
[build:nx-command] still running after 2630 s
[repository:command] still running after 5474 s
[build:nx-command] still running after 2640 s
[repository:command] still running after 5484 s
[build:nx-command] still running after 2650 s
[repository:command] still running after 5494 s
[repo] still working after 7551 s; last phase started: source.served
75 | /** 🔒️ Refuses changed producer source, input presence/bytes/rosters, or published output bytes. */
76 | export async function assertCargoPreparationObservationCurrentV1(value:unknown,control:CargoPreparationControlV1):Promise<void> {
77 |  const actual=cargoPreparationControlSnapshotV1(control),row=admitCargoPreparationObservationV1(structuredClone(value));
78 |  for(const edge of row.resolutions){cargoPreparationProgramRootV1(row.root,row.sourceRoot,edge.source);cargoPreparationProgramRootV1(row.root,row.sourceRoot,edge.selected);}
79 |  let resolved=0;for(const input of cargoPreparationResolutionBatchesV1(row.resolutions)){requireIssuedResourceControlV1(actual.leaseControl);actual.signal.throwIfAborted();const batch=JSON.parse(input)as CargoPreparationResolutionV1[],selected=await captureCargoPreparationProgramV1(row.root,row.script,"resolve-program",[],actual,input);if(!Array.isArray(selected)||selected.length!==batch.length)throw Error("Inva
80 |  let completed=0;for(const item of cargoPreparationCurrentInputsV1(row)){requireIssuedResourceControlV1(actual.leaseControl);await actual.advance({operation:"recipe",path:item.input.path,completedOperations:completed++});requireIssuedResourceControlV1(actual.leaseControl);if(cargoPreparationInputV1(item.root,item.input.path,item.input.kind).sha256!==item.input.sha256)throw Error(`Preparation custody changed: ${i
                                                                                                                                                                                                                                                                                                                                                                                             ^
error: Preparation custody changed: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts
```

Later in the same run, not in the failed list of nx: `Executable not found in $PATH: "trunk"` (`workspace:deps-trunk`) and `Executable not found in $PATH: "wasm-pack"` (`workspace:deps-wasm`).

## Tries 11 and 12: access refusal (bootstrap log, last 12 lines each)

```
--- try 11
Reading the project graph slice (46 s): reservation 0 of 6553; running (name, kind, seconds, rows): ; waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 4605 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not even 2048 MB are free.
Reading the project graph slice (48 s): reservation 0 of 6553; running (name, kind, seconds, rows): ; waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 4173 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not even 2048 MB are free.
Reading the project graph slice (50 s): reservation 0 of 6553; running (name, kind, seconds, rows): ; waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 4173 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not even 2048 MB are free.
Reading the project graph slice (52 s): reservation 0 of 6553; running (name, kind, seconds, rows): ; waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 4173 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not even 2048 MB are free.
[repo] still working after 260 s; last phase started: door.script
Reading the project graph slice (2 s).
Reading the project graph slice (4 s).
Reading the project graph slice (6 s).
Reading the project graph slice (8 s).
The graph service was denied access to the checkout although this process can read it, so this is not a missing file access consent of this session; a service that was started for another route or another app is judged against another identity. The service ended and was started once more under the access of this process; the second one was denied as well. macOS denied the resident service access to /Users/ueli/Documents/semio/.cache/b/.reset.json (your Documents folder) while it answered a request (reference da146983; the service was started as route direct, pid 60693, parent pid 60647). The service has stopped itself and never serves without access. If the command started the service again under your own session and this message returns, send the line that starts with `where:` below reference da146983 in /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-74145656f35a.log. Otherwise switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/wg
end 2026-11-15T07:50:46Z
native_rc=1
--- try 12
[semio] workspace:setup…
[capacity] run: sized at 70% of the default from 6 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (7340 MB available, 2048 MB floor)
Starting the repo graph service for this host.
The graph service runs under the access of the calling app: the launchd route was refused for /Users/ueli/Documents/semio earlier since this machine started / Der Graphdienst läuft mit dem Zugriff der aufrufenden App: die launchd-Route wurde für /Users/ueli/Documents/semio seit dem Start dieses Rechners bereits abgewiesen
Waiting for the repo graph service (10 ms).
Waiting for the repo graph service (2042 ms).
Waiting for the repo graph service (4076 ms).
[repo] still working after 5 s; last phase started: door.script
The graph service could not answer (lost the service was killed without an exit record (its cgroup is gone) [log: [script] build:implementation-source · Reading the sources of the build implementation | Internal error: Relative filesystem fstatat failed (EPERM) | repository:command failed with exit code 1]); look at `repo service host status` and try again
end 2026-11-15T07:59:38Z
native_rc=1
```

Service log lines of tries 11 and 12 with `where:`, `access-proof`, `EPERM`:

```
2026-11-15T07:49:33.591Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=56873 ppid=56801
2026-11-15T07:50:46.464Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=direct pid=60693 ppid=60647
Internal error: Relative filesystem fstatat failed (EPERM)
```

## Other files of the series

`round-14.monitor.txt` (timeline, runner swaps at 02:33Z and 03:13Z), `round-14.prerun.txt`, `round-14.fleet-endpoints.txt` (fleet ports at 02:53Z), `round-14.try-5.failure.txt`, `round-14.try-<k>.samples.txt`, `round-14.try-<k>.p12-samples.txt` (+ `.sample-<date>T<HHMMSS>Z.txt`, renamed today: the names had colons and did not replicate), `round-14.try-<k>.service.txt`, `round-16.md` and `round-16.tools-setup.txt` (step 0), `handoff-prompt.md`.
