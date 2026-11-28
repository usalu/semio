# Capacity over-reservation (2026-11-27)

## Symptom
`repo fleet status` on this 16 GB Mac: admitted 9039 MB vs observed 4167 MB (over-reserved 5196). `repo run @semio-tech/flow-extension-math-rust:package -c prototype --exclude-dependencies` waited "to grow the reservation from 512 MB to 8190 MB" and two attempts ended with exit 143 (host warden kills while the host swapped).

## Where the numbers come from
- 8190 MB is not a measured need. The class `run:build:@semio-tech/flow-extension-math-rust:package` has never completed, so it borrows the pooled `run:build` ratio (p90 x 1.25 of runs such as `workspace:setup`, 7 to 13 GB; "sized at 71% of the default from 2 earlier runs") for the full 8-job request, then the class bound falls back to `estimates.fallbackCeilingMegabytes` (8192). The native build report of the same run shows 13 compiled units, largest rustc 0.19 GiB.
- The upgrade (quiet 512 MB grant -> full request, `upgradeHostGrantV1`) asked for all 8 jobs as its minimum and only narrowed to 1 job after `wait.bypassMilliseconds` (30 s). In that window the 8190 MB ask is a queue head that nothing can satisfy (3.5 GB spare), so it also blocks smaller requests ("an earlier request is still waiting ... behind build (waiting ahead, 8 job(s), 8190 MB)").
- "N MB reserved but not yet resident" in the waiting lines is the deliberate charge of a ramping grant at its full reservation for its learned ramp (300 s default, up to 30 min). Several fresh admissions at once (graph-plan 3673 retained, cargo 3481, run-many 3510) therefore starve a 512 MB ask for minutes. The over-reserved figure of `fleet status` is the nominal reservation sum; once a grant finished its ramp the gate charges what it holds (`committedOf`).
- A pass that needs more than the parent grant (`ParentComponentMemoryV1`) declared its need and waited for local growth even while a worker slot was only briefly busy; the declared need teaches the class a bigger local grant.

## Tried and reverted
A linear release of the not-yet-resident part of a measured, ramping grant (new policy `memory.rampReleaseBasisPoints`, schema, shipped policy, fixture `ramp-release`). It freed memory as intended but broke two deliberate invariants pinned by existing tests: `farm-width` ("a busy grant is charged in full", 14336 became 12618) and `memory-fit` ("never more shrunk reservations growing at once than the slots allow"; 3 > 2 under a cold-mode burst). A busy ramping grant can still be growing, and the row carries only the current observation (no plateau evidence), so releasing it risks the swap bursts the ramp rule exists for. Removed again; no trace left in code, schema, policy or fixtures. A safe release needs plateau evidence on the grant row (peak and time since the peak last rose), which is a row-schema change and was not made.

## Fix
1. `upgradeHostGrantV1` (`host-capacity/upgrade`): an upgrade whose size is a guess (`request.rampSlot === true`: the class is not measured) narrows to the jobs that fit at once instead of waiting `bypassMilliseconds` with its whole preferred size. It starts with fewer jobs and grows through the existing `growHostGrantV1` as memory frees; it no longer holds the queue head with an 8190 MB ask. Measured classes keep the fair-queue behaviour unchanged.
2. `runPassesInJobSlotsV1` (`component-pass/job-slots`): while a worker slot is only busy (`fleetBusy`, within `fleetBusyWaitMilliseconds`) a pass that does not fit the parent grant waits for the fleet instead of declaring a need and growing the local grant; local growth (and the declare) starts only when the fleet cannot take it. Heavy linked units now prefer offload before demanding a bigger local grant. `offloadExpenseV1` placement itself is unchanged: it already sends expensive units to workers with headroom.

## Tests
- `host-capacity/tests/upgrade/agents`: new test "an upgrade whose size is a guess ... asks for the jobs that fit at once" (world with a busy peer; bypass window disabled; expects 4 of 8 jobs upgraded). The existing measured-class bypass test (en/de) still passes.
- `job-slots/tests/growth`: new law "a heavy pass waits for a worker slot instead of declaring a need ..." (fleet busy for 3 polls then free: runs on `fleet`, declared need 0).
- Regression run (`repo test-file`, 92 files of host-capacity process and library plus job-slots): everything passes except `blockers` and `followup` (oracle worlds `upgrade-8` and `followup-2`; they fail without my changes as well, the decision code is under concurrent edit by others) and three restart/lazy-graph probe child scripts that are not test files.
- `repo typecheck` on the touched files: 8 errors, all in files not touched (`warden/tests`, `tests/retained.test.ts`, `tests/scaling`).
- No third-party oracle: both rules are one-line guards on existing logic; the existing Python oracles cover the unchanged decision.

## Before / after (`fleet_status`)
Before: admitted 9039, observed 4016-4167, over-reserved 5196-5538 (3 grants: run-many 3510/810, cargo 3481/1082, run 512/73).
After (process-wide code is not reloaded by already running builds): admitted 10501, observed 7205, over-reserved 3296 (cargo 3481/1030, run-many 4446/4292, run math-rust 2574/1883). The figure is dominated by what happens to be running; the change takes effect for runs started after it.

## Limits and follow-ups
- Sizing of unmeasured classes (pooled p90) is unchanged; a per-class prior from the target's kind plus project size or a plateau-based release (row schema) are the structural next steps.
- Retained graph-plan reservations (3673 MB) stay charged in full by design (`retained` test).
