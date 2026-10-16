# Current Public Workspace Native Sample

The genuine standalone 60-second diagnostic completed with terminal status 0 and 52639 ms remaining. It did not change production Source, inject a profiler, signal the workspace process, acquire image/graph/SDK leases, or modify SDK contents. It invoked only read-only process metadata commands and `/usr/bin/sample 45614 5 1 -fullPaths -file <ticket-generated-output>`. The sampler itself ran as an owned child under the diagnostic Original signal and deadline; cancellation would terminate only the sampler subtree. The workspace retains its separate authored Original.

## Exact Target

Before and after sampling, PID 45612 was the exact public package command `bun repo run workspace:test-workspace-build --skip-cache --exclude-dependencies`. PID 45614 had parent 45612 and the exact actual build dispatcher argv for the same selection. Both retained launch time October 15, 2026, 22:14:21 local. All four stdout/stderr descriptors referred to the expected retained `public-workspace-combined-current.log`, device 16777232, inode 344873831. The log grew from 25047644 to 25090501 bytes during the observation. This establishes ongoing output during this short window; it is not task acceptance or an independent completeness proof.

Diagnostic observation began 2026-10-15T20:35:15.756Z and ended 2026-10-15T20:35:22.463Z. Sample size was 1119443 bytes. The report records a requested one-millisecond sampling interval and 2,665 observations for each of 13 threads. Observations are not syscall counts or elapsed CPU durations. Physical footprint in the sample header was 2.8G, with recorded peak 4.5G.

## Recorded Stacks

Each of seven Bun Pool threads had all 2,665 leaf observations in `__ulock_wait2`. These worker-thread waits do not establish that the main thread or whole process was idle. The main thread had fresh native/Bun/JIT stack branches rather than one wholly repeated blocking frame. Reconstructing exclusive counts from its call tree gives `syscall_thread_switch` 203, `__psynch_cvwait` 189, `stat` 56, `lstat` 48, `__openat_nocancel` 46, and `__read_nocancel` 41 observations. Two unsymbolicated Bun offsets (`0x2a5f754`, `0x2a5f75c`) had 94 and 90 exclusive observations. The main root count is 2,665. Every thread's reconstructed self sum was separately checked using Python SQLite `SUM`, and matched its recorded root count.

The all-thread collapsed table is dominated by background waits (`__ulock_wait2` 18,661 and `__psynch_cvwait` 12,181). It must not be interpreted as the main-thread cost breakdown. Bun symbols and JavaScript Source names are mostly unavailable (`???` and unknown binary), so the sample cannot identify a repository function, validate a particular algorithmic hypothesis, or justify a Source optimization. The evidence is consistent with sampled runtime/JIT work, some synchronous metadata/read system calls, and synchronization in this window; it does not determine CPU percentage, I/O latency, scheduling delay, whole-run attribution, looping, or remaining duration. No runtime change is proposed from these unsymbolicated frames.

## Retained Evidence

- `🗑️generated/workspace-6843-dispatcher-45614.sample.txt`, SHA256 `ece2f40ace6de4b6c9fd2d25496872cfbf98e426e80428e777e6baf919a35846`.
- `🗑️generated/workspace-6843-sample-receipt.json`: exact before/after command lineage and FD/log identity.
- `🗑️generated/workspace-6843-sample-summary.json`: per-thread native self observations with SQLite sum checks.
- `🗑️generated/workspace-6843-sample.log`: actual diagnostic terminal output.
- `workspace-sample/📜️script.ts`, SHA256 `7c0a28567a140d13305cf888040cc0e9b9e3634386ebed7b8aeb52f976274155`: closed exact-target diagnostic input.

The workspace process was still the same exact target at the final observation. Its terminal status and Bun test aggregate remain exclusively the parent owner's live command result.
