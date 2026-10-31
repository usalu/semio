# Watchdog Process Census Audit

Actual retained profile summary:711,571 bytes,2694 nodes,3105 samples,526,332ms duration and526,331ms sampled extent. spawnSync receives251,268ms weighted attribution through liveChildPidsV1, sampler, leaseActivityV1 and probeLeaseActivityV1. Standing component stringify receives29,904ms; renameSync16,102ms. These weights include synchronous native waits and are not measured OS CPU or invocation counts. The separate three-second CPU sample shows unchanged user/system counters while sleeping.

## Actual Call Frequency

Process leases/standing/TS:68–83 maintains one process-wide state.probedAtMs and skips activity sampling until30,000ms since its previous probe. Registry flushes may request probing more often; they do not necessarily spawn a table command on every flush. Its shared activity sampler launches child census and, when children exist, CPU census. Routing watchdog/TS:36–38 has an independent15,000ms activity sampler while activity is demanded. Its stall watchdog runs every30,000ms but does not read children before600,000ms idle and skips during active declared waits. Standing/yield also uses liveChildPidsV1(pid) for other roots. Sampling streams therefore need not align and can repeat expensive whole-table observations.

## Current OS Behavior

Linux and macOS both synchronously execute ps -axo pid=,ppid=, with10-second timeout, then build parent links and follow each candidate ancestor at most64steps. Current process and the snapshot subprocess are excluded. Windows synchronously executes PowerShell Get-CimInstance Win32_Process with the same timeout. Unknown table returns undefined; liveChildCount maps this conservatively to one child. The sampler treats unknown children, unknown CPU, or first CPU observation as activity, preserving no-idle-judgment under uncertainty. Windows CPU observation is always unknown.

For nonempty Unix child lists the CPU reader starts another ps command with the explicit PID list and parses elapsed CPU timestamps. It currently checks output type but not exit status; malformed timestamps are unknown. The child-table parser currently discards malformed rows rather than refusing the complete snapshot. Thus a future pure census replacement should explicitly define complete/unknown handling and avoid turning partial observation into proof of no descendants. This audit did not establish a new production failure from those inherited boundaries.

## Existing Independent Proof Surfaces

Process host-capacity gate/🧪️tests/🚪️gate/TS:321–322 compares the production reader availability against an independently located /bin/ps or /usr/bin/ps call. The surrounding real child test proves capacity lending only during declared waits without live children. Process routing/invocation/process/expiry/tests/progress/TS:67 consumes neutral child/CPU sample rows. Standing/tests/TS:179–204 checks probe cadence and work renewal; standing schema owns30s cadence and minimum200ms process CPU. These are suitable owners for added schema-first census vectors and independent ps parity; no process signals, runtime calls, Native jobs or tests were launched here.
