# Lane `job-limits`: hard per-job memory limits

`BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. New module `BUILD/🧱️job-limits`.

## Result

An overrun now fails exactly one job as a typed outcome `memory-exceeded` (EN+DE text, peak recorded, learned estimate raised), instead of a SIGKILL from the kernel OOM killer on an arbitrary process.

Verified live on this host (Linux, systemd user manager): a real delegated cgroup v2 killed a 1024 MB allocator at a 160 MB limit (signal SIGKILL, peak <= limit, well under the 1500 ms budget), a sibling process kept running, and `MemAvailable` did not drop by more than the limit.

## Design

- Port (`🟦️.ts`): `JobLimiterV1.create(policy, job) -> LimitHandleV1 {attach(pid), onBreach(cb), usage(), release()}`; `runLimitedJobV1` spawns, attaches, cancels via `AbortSignal`, reports progress, always returns a `JobOutcomeV1 {kind: ok|failed|memory-exceeded|cancelled, enforcement, limitMegabytes, peakMegabytes, exitCode, signal, durationMilliseconds}`.
- `JobLimitPolicyV1` per plan plus `kind: "job"|"service"`. Finding: `memory.high` on a job delays the kill by tens of seconds (measured 31 s for a 1 GB allocator at limit 160 MB, since there is no swap and the allocator is throttled in reclaim). Jobs therefore get no `memory.high`; services (daemon) do.
- `🐧️cgroup`: finds the highest user-writable ancestor of the process' cgroup with `memory` in `cgroup.subtree_control` (the systemd `user@UID.service` delegation), creates `semio-jobs/`, enables `+memory +pids`, then one `job-<pid>-<n>` cgroup per job: `memory.max`, `memory.swap.max=0`, `memory.oom.group=1` (kernel kills the whole job together, nothing else), `pids.max`, `cpu.weight`. `attach` writes `cgroup.procs`; `memory.events` (`oom_kill`) and `memory.peak` are polled every 20 ms; `release` writes `cgroup.kill` and removes the directory; leftover empty `job-*` directories of crashed runs are removed at detection. No systemd-run needed (plain `mkdir` in the delegated cgroup was verified to work, with exact `oom_kill` and `memory.peak` kept until `rmdir`).
- `🪟️job-object`: std-only Rust helper `semio-job-limit` (hand-declared `extern "system"`, no crates): job with `JobMemoryLimit`, `ActiveProcessLimit`, kill-on-close, polls `PeakJobMemoryUsed`, `TerminateJobObject` at the limit, NDJSON frames `ready|usage|breach`, stdin EOF releases the job. The TS side (`createJobObjectLimiterV1`) compiles the helper once per source digest with `rustc` into the user cache.
- `🐕️watchdog`: cooperative fallback for macOS, Linux without delegated cgroup, Windows without helper. Polls the process tree (`/proc`, `ps -A -o pid=,ppid=,rss=`, CIM query) every 100 ms and kills the process group at the limit. It enforces after the fact by at most one poll interval. `RLIMIT_AS` is not set (not reachable from Node/Bun without a native helper; an address-space limit is also a poor memory measure).
- `🎚️select`: cgroup-v2 on Linux, job-object on Windows, otherwise watchdog; a `hard` policy that only gets the watchdog reports `fellBack` (text `enforcement-fallback`, EN+DE).
- `🎫️admission`: `grantJobLimitV1(grant)` makes the limit equal to the grant reservation (reservation = limit); `learnedRunAfterOutcomeV1` records a killed run with peak = max(peak, 1.25 x limit) and status failed, so `learnableHostRunV1` accepts it and the next reservation (and limit) grows.
- Estimate: `jobEstimateV1(class, floor, learnedPeak?)` = max(1.25 x peak, floor, 64 MB).
- Wiring: `BUILD/🌡️host-capacity/🟦️.ts` `withBuildCapacityV1` body receives `limit` and `report(outcome)`; `learn` applies `learnedRunAfterOutcomeV1`. Other callers (planner, daemon, executors) are owned by other lanes: see `📌️requests.md` `## → scheduler, daemon, integration (from job-limits)`.

## Tests (`BUILD/🧱️job-limits/🧪️tests`)

Command: `cd BUILD/🧱️job-limits && repo --service=off tools exec --light bun -- test ./🧪️tests/🟦️.ts` -> 13 pass, 0 fail, 85 expects.

Language-agnostic: `🔣️.json` fixtures (estimates, learned peaks, process trees, allocation cases validated against the schema `Fixture`), `🥒️.feature`, independent Python oracle `🐍️.py` (limit, learned peak, tree memory must equal the TS results), language-agnostic allocator child `🐍️allocator.py`. Covered: schema/texts parity EN/DE, fake limiter (overrun/ok/cancel/exit code), cgroup limiter against a fake kernel directory (every file written, breach detection, release), detection without delegation, selection per platform, real watchdog kill, real cgroup kill with sibling survival, job-object helper protocol with a scripted helper, admission/learning.

The Rust helper type-checks for `x86_64-pc-windows-gnu` (`rustc --emit=metadata`; I installed that rustup std target for the check) and runs on Linux as an `unsupported` stub (exit 6). It has NOT been executed on Windows. macOS watchdog sampler (`ps`) is not executed here either.

## Follow-up: unmanaged memory in admission and the host-capacity test suites

Unmanaged memory (`PROC/🏗️build/🌡️host-capacity`):
- `HostSampleV1.unmanagedMegabytes` (optional) is the memory the machine uses beyond the free memory minus what the live grants hold: `total - available - sum(observed ?? reserved of non-lent grants)`, measured live at every evaluation by `withUnmanagedHostV1` (gate lock path and `readHostCapacityStatusV1`). `evaluateHostCapacityV1` divides only `max(0, cap - unmanaged)` by the budget share, both for the live budget and for the drained budget; the `available`-based room already saw the unmanaged memory, the budget side did not, which is what let agent CLIs, editors and watchers be double-sold. The evaluation reports `unmanagedMegabytes` when the sample carries it. Fields are optional so existing hand fixtures and the SQLite/Python oracles stay valid.
- `🧺️unmanaged/🟦️.ts`: `censusUnmanagedHostV1(rows, grantRoots)` counts every process outside the granted trees into the schema classes (`agent`: claude/codex/droid/kiro-cli/..., `editor`, `watcher`, `tool`: bun/node/..., `other`), `readHostProcessRowsV1` reads `ps -A -o pid=,ppid=,rss=,comm=` (CIM on Windows). It is exported and tested but not yet printed by `repo capacity status` (that would change the stable status lines); admission uses the derived number, which needs no process scan per poll.
- Tests `🧪️tests/🧺️unmanaged` (4 pass): fixtures, independent Python oracle for the unmanaged number and the census, admission example (8 jobs free vs 2 jobs with 8000 MB unmanaged), live gate status.

Host-capacity suites, root causes (no test deleted):
- Language: a `{}` environment chooses no language, and the shared language rule reads the OS language list only on macOS and Windows, so the tests passed only on those machines. Every environment the tests hand to the gate now names `LANG`; the estimates test (a notice in every declared language) clears the language variables for its duration because it asserts the no-language case.
- `tools setup` is a heavy `command` block in the authored policy; the fixture and name claimed it was light. The fixture now states the real ask and adds `tools list` as the light case.
- `tool-format`/`tool-query` are capacity operations declared by the tools registry, not catalogue operations; the policy/catalogue test now accepts operations named by the registry.
- The body of `withBuildCapacityV1` gained `limit` and `report`; the tests that compared the whole argument with `toEqual` now use `toMatchObject`.
- German notice uses the typographic `13 %`; the regex follows.
- The footprint fallback notice printed the machine code of a `RepoErrorV1` (`invalid process.capacity.empty-process-table {}`); `sampling` now renders the reason per language through `renderRepoErrorV1` (real bug, not test drift).
- Linux process start is `tick:N`, not `ps lstart`; the handover test asserts the Linux form. `totalmem()` is not megabyte aligned; the followup test floors like the code. The grow test read the live free memory of the machine; it now injects a Linux probe from its own sample.
- New test for the grant path: `limit` equals the reservation (kind job, no swap), `report(memory-exceeded)` raises the next reservations of the class (killed sequence ends more than 2x above the measured one), light commands carry no limit.

Results: PROC `🌡️host-capacity` all 14 test files, 208 pass 0 fail (each file run with `repo --service=off tools exec --light bun -- test <file>`); BUILD `🌡️host-capacity` 17 + all sub-suites pass (`🧮️arguments` needs the managed python binding, `SEMIO_TEST_PYTHON_BINARY=.venv/bin/python`, then 6 pass); `🔁️restart/📜️child` is a child script, not a test; `🧱️job-limits` 13 pass.

## Not done / open

- Process census is not printed by `repo capacity status` yet (see follow-up).
- Wrapping the real planner, daemon, executors and rustc/tsc invocations is the job of the lanes that own those call sites (requests filed).
- 2 h soak with zero unrelated OOM kills not run.
