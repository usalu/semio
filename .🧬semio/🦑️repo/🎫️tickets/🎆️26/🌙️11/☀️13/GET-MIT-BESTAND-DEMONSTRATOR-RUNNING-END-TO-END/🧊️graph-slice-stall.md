# Graph slice stall of activate-14 (2026-11-15 05:11-05:23)

## Evidence
- Client 40166, service 40708, graph child 40713 and the first slice worker 40790 were all at 0-1 % CPU. Nothing was computing; the whole chain waited on the slice worker.
- `sample 40790`: main thread in `openat$NOCANCEL` -> `__openat_nocancel` for the whole interval (1544 of 1544 samples), all other threads parked. This is a blocking open(2) syscall, not a JavaScript await, so no lost wakeup, no lease and no capacity wait: `repo leases status` showed 0 waiting, 0 deadlock cycles, capacity granted (graph-plan 2048 MB).
- The worker had only a unix socket, /dev/null and a kqueue open (no sqlite lease db, no module files) and had emitted no `graph:slice-worker` activity line: it was stuck before its first lease activity (module load / first file open), 298 MB resident.
- Host log: only `build:source-admission` heartbeats after `[capacity] admitted graph-plan` (line 446) and no `graph:slice-worker` line; at line 547 the Script watchdog ended the worker ("no progress for 12 min and has no live child", exit 124), reported at `slice-pool/🟨️.mjs:176` as `Slice worker exited with code 124` (reference 04c2fb6f).
- The earlier `exit code 4` is the previous host (pid 40186, launchd-started): `exit.log` reason `access-denied`, caused by `Relative filesystem fstatat failed (EPERM)` at 04:11:13 (`📁️descriptor/🟦️.ts:148`). macOS TCC logs show the Documents-folder access attributed to `tech.semio.repo` (pid 40183) at 05:10-05:11, then to `com.anthropic.claude-code` (pid 39644) from 05:11:18, i.e. the service was restarted under the calling app exactly when the first worker started.
- The second worker (pid 45501), started by the same service right after, ran normally at once: boot-recovery, lease-sweep, directory stamps 1253 of 1253, file stamps, 146 % CPU, 2 GB. The same code, host, leases, disks and NFS (192.168.178.70, answers in 60 ms) work, so the first stall was transient, outside the code path of the worker.
- No FIFOs, no dataless files and no unresponsive disk-image mounts exist (all `.cache/b/*/m` list in 0 s).

## Conclusion
The first worker was blocked in the kernel in an open(2) with no timeout, most likely the macOS privacy (TCC) / file-access arbitration of the freshly changed responsible process (launcher -> calling app) at 05:11; the exact path cannot be recovered (root-only `fs_usage`, process already gone). It is not a deadlock in repo code. The wait ended only through the 12-minute Script watchdog; the service stayed alive and started a second worker which is making progress; it will eventually finish its plan (owner phase cost as in `🕸️graph-slice-slowness.md`).

## Fix
No source change: nothing in the repo tooling defect-wise was proven (no lock, lease or promise at fault), and a guess would be a speculative change. Improvement worth a separate ticket: the slice worker should report its first-activity phase so a stall before the first lease activity is shown (and cut) well before 12 min; the client printed only "Reading the project graph slice (N s)" for 699 s with no detail because the worker never sent a `progress` message.
