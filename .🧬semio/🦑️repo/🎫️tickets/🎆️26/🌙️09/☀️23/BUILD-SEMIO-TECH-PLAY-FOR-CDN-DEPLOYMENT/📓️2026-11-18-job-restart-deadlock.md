# describe-tool job "deadlock" after exit-75 restarts: it was not capacity

## What the evidence shows
- Restart path (`runJob` in `🧩️component-pass/🟦️.ts`): each attempt only re-spawns the `repo` child with the same environment. Capacity is not granted by the driver per attempt: the driver holds its compiler CPU cores (5 of 10 on this host, share policy 50 %) and the cargo custody lease for its whole run, and the nested `repo` child acquires its own leases (own lane `.cache/b/r/m/u/l/q…`, inherited custody via ancestor pid, cores 5..9 free). Nothing is released into or re-issued from the driver pool on exit 75, so the restart does not lose a grant.
- The stalled child's log (`.cache/b/r/m/components/runs/wtkyoQ6/l/job-describe-tool.log`) shows what the 0 % CPU child was doing: the build-layer churn convergence loop of the repository ("Another process edited …/🧵️job/⏱️context/🦀️.rs while it was read (one of its inputs changed). Taking a new snapshot, attempt N of ∞, in 30000 ms", 38 attempts, bounded only by `convergence.wallClockMilliseconds` = 2 h in `🔁️churn/🧬️schema`). It sleeps 30 s per attempt, holds its shared store leases and queue claims, and never waits on a CPU token. The driver's `compiler:capacity-subdivision` activity is just its own last published activity.
- Why the driver never noticed: (1) every child output line, including those retry notices, reset `lastMessageMs` and the process activity clock; (2) the watchdog returned early whenever `stagesRunning > 0`, and a running job counts as a running stage.

## Fix
- New pure unit `🧩️component-pass/⏳️job-wait/🟦️.ts`: classifies child lines. A churn retry notice (derived from the declared EN/DE templates of the churn protocol) or a `[script]` lease activity with `elapsedMs > 0` is a wait naming its path/resource; any other line is progress. `watchNativeBuildJobV1` tracks progress vs wait per child, `nativeBuildJobStallV1` returns the refusal `Native build stalled: job:<id> made no progress for N s and has only waited on <resource>`.
- Driver: wait notices no longer reset the activity clocks; the watchdog checks every running job watch first and stops the run with that refusal (ignoring the `stagesRunning` exemption for a job that only waits). Normal progress, silent jobs and all other stall rules are unchanged.
- Test: `⏳️job-wait/🧪️tests/🟦️.ts` (4 tests, runs under plain bun, no script envelope): `bun test ./…/⏳️job-wait/🧪️tests/🟦️.ts` passes.

## Not changed / open
- The cause of the endless "input changed" snapshots on `🧵️job/⏱️context/🦀️.rs` (mtime 20:55, stable) is a separate question: the churn layer keeps re-snapshotting for up to 2 h; with this change the driver now refuses after `stallMs` (15 min) instead.
- The driver was not exercised end to end (no build was started); only the pure unit was run and the driver file parses.
