# Warden: wrong victim and no resume (2026-11-27 20:23)

Code: `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/` (`🟦️.ts` decision `decideHostWardenV1`, `🔁️loop/🟦️.ts`).

## 1. What "a build-critical process crossed its memory cap" is
- Trigger `cap`: some subject with `capCritical` (compile unit, graph pool child, slice worker) had more megabytes than its cap (`capOf`: kind cap, or the host cap share when it holds a grant) or its unit ceiling. Not pid 16820. That crosser is only reported, never killed below danger, and is excluded from shedding while the host is quiet.
- The level became `guard` (tight) although the host had no shortage (5164 MB available, headroom 13356 MB, pressure normal): `base` calm raised to guard only by the crossing.
- Old rule: with a crossing, the other subjects were suspended until the overage was covered. The only non-critical running subject was the `granted` tree of pid 16820 (3348 MB, within its 5400 MB grant), so it was suspended. Wrong victim: a sheltered subject paid for another's crossing on a host without shortage.

## 2. Why it never resumed
- `recovered` (and the `calmTicks` count) required `crossed.length===0`. The critical crosser kept running over its cap, so `calmTicks` stayed 0 and the "calm" resume never fired, however much memory came back. Only the manual SIGCONT worked.
- Second gap: `pageIn` of a stopped subject without a measured non-resident size was Infinity unless `level==="calm"`, which a crossing prevents; so even the room check refused.
- Third gap: a `granted` subject (grant on a plain pid) was built with `suspendedSeconds:0` always, so it could never become overdue or hit the hard limit, the safety net that resumes after N seconds.

## Fix
- Victims: when the host itself has no shortage (`base` calm) only crossed subjects are suspendable (a non-critical one over its grant); critical crossers are reported and hold admissions, sheltered subjects are never paused. With a shortage the crossers are suspended first (new first sort key), then the old order (non-critical, sheltered last).
- Resume: `calmTicks` counts consecutive quiet ticks with available >= resume line (hysteresis above the guard line, `calmTicks` ticks, `resumeSettleSeconds` between wakes); a running critical crosser no longer blocks recovery. A subject that is itself crossed is not woken by the calm path. `pageIn` uses `base` instead of `level` for the unknown-footprint case.
- `grantedWardenSubjectsV1(..., suspendedSince)`: the loop passes its `since` map, so granted trees carry real `suspendedSeconds` (overdue and hard limit work).
- Docstrings updated. The 2,048 MiB desktop floor is untouched. Broker admission file not touched (other agent's lane; the broker already treats SIGSTOP as alive).

## Files
- `🛡️warden/🟦️.ts`, `🛡️warden/🔁️loop/🟦️.ts`
- Oracle `🧪️tests/🐍️.py` (same changes; expectations regenerated with `python3 🐍️.py --write`)
- Fixtures `🧪️tests/🧫️fixtures/🔣️.json`: 7 new decision cases (incident replay; victim only the over-grant subject; victim crosser/over-grant first under shortage; suspend then recover then resume; one tick short of hysteresis; dip below resume line; overdue resume) and 1 new `granted` case (suspendedSince). Names of 3 existing cases corrected (no longer "others suspended first"); their expectations regenerated.
- `🧪️tests/🟦️.ts`: updated watchdog cap test (no suspension at calm host; crosser first at shortage), new loop test (stopped granted tree resumed exactly once after quiet ticks, never re-suspended, crosser never signalled).
- `🧪️tests/🧱️containment/🟦️.test.ts`: passes `suspendedSince` to the granted oracle comparison.

## Results (repo test-file)
- `🧪️tests/🟦️.ts`: 61 pass, 0 fail (an earlier run had one flaky MOUNTED identity test fail, passed on rerun; the second failure there was my own stale assertion, fixed).
- containment 8, swap-churn 4, healthy-paging 2, grant-shelter 2, tree 2: all pass (run before the last assertion edit, which touched only `🧪️tests/🟦️.ts`).
- Oracle independent: Python standard library (`🐍️.py --check` fresh). Not run: `repo typecheck`; shared warden service not restarted (it picks up the new code on its source-change cycle).

## Finishing steps (coordinator)
1. `repo typecheck` on the 4 changed warden files plus importers: 28 files (24 importers), 0 errors, no drift.
2. Containment (8), swap-churn (4), healthy-paging (2), grant-shelter (2), tree (2): all pass after the last edit.
3. Live warden: one long-lived `bun` process (pid 21807, mode enforce) that replaces itself on a source change through the successor handover (`--handover`, lock wait), so no restart command is needed or run. It started 20:28:29, after the last source edit (`🟦️.ts` 20:28:20, loop 20:27:14), and `repo capacity status` shows it calm with code digest 791d48027820b3b8, its grants intact (2 running grants, 3836 MB reserved). Nothing was killed or signalled by me.
4. Pinned-rustc MOUNTED test: 3 consecutive full runs of `🧪️tests/🟦️.ts` pass (61/61 each). The single earlier failure happened while six warden test files ran in parallel; the test waits at most 20 x 10 ms for SIGSTOP of a real child, so it is load-sensitive (timing flake), not a logic regression. Not changed.
