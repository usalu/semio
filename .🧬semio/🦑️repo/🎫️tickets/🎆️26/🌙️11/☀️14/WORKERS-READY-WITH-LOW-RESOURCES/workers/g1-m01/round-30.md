# g1-m01 Round 30 Report

Before the run: `round-30.pre.md`. Per-try files: `round-30.steps.txt` (cut lines, highest step, final lines), `round-30.p12-try-<k>.txt` (5 s slice worker samples), `round-30.service-try-<k>.txt` (service log extract), `round-30.samples.txt` (30 s pressure and largest processes), `round-30.stall.txt`, `state.json`.

## Try 1 (17:38:01Z to 20:06:07Z, 8886 s): `native_rc=130`, NOT a result of the build: my runner interrupted it

- **Runner error.** The runner sent SIGINT at 20:05:30Z on its stall rule (no new `graph:` line in the service log for 15 minutes). The service was not standing still: its log shows `build:projects · Inferring projects: 211 … 215 of 4774 projects` in the last minute before the stop (`round-30.stall.txt`). At step 5 the service prints these `[script]` lines and few `graph:` lines; the last `graph:` line was `headroom of 157 MB is below one project (771 MB), inferring one project at a time`. The rule was corrected at 20:08Z (it now also counts `Inferring projects: n of N` lines) and the runner restarted with try 2 two minutes after try 1 instead of five.
- **Highest step reached: step 5 of 6.** The `cut smaller` lines, verbatim (the log prints `(0 s)` in each):
  - `the slice worker grew to 9927 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 2 of 6: …)`
  - `the slice worker grew to 6642 MB, … (step 3 of 6: …)`
  - `the slice worker grew to 8271 MB, … (step 4 of 6: …)`
  - `the slice worker grew to 9868 MB, … (step 5 of 6: …)`
  - No exit 75 and no `graph-plan-too-heavy` refusal text in the bootstrap log: the read went on after each cut, as the round expects.
- **Peak resident size of the graph child (slice worker):** 7002 MB in the 5 s samples (the cut lines name up to 9927 MB, between samples). Largest process of the run in the 30 s samples: 6647520 KiB. No hard stop (8 GB) was met by the sampler.
- **Speed at the lower steps.** The counter `Inferring projects: n of 4774` starts again after each cut (three runs to about 250 to 270 in the newest service log) and advanced about 4 projects per minute at step 5 (211 to 215 in the last minute). `x of 4774` complete: not reached. Whether the projects inferred before a cut were skipped after it: the counter does not show it; closure entries on this machine went from 42222 before to 40799 after the try.
- **Fleet:** no `waiting for the fleet` line in this try. `3 projects taken from the fleet with 0 closure entries, 1 of them did not hold here and were inferred again, 0 not shared, 47 … 247 inferred here` (first worker), later `5 projects taken from the fleet with 0 closure entries, 3 of them did not hold here … 45 inferred here`.
- **Access:** `access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=direct pid=82558 ppid=82521 responsible=80265`. No EPERM, no `request-failed`.
- **Memory headroom lines** (expected, round 23): many `waiting for memory headroom (… of … MB reserved, n projects running)`, among them again a reserve total of 0 MB (`774 of 0 MB reserved`, `1864 of 0 MB reserved`), and `project @semio-tech/space-space (language:typescript) grew by 2253 MB, more than the 1756 MB this machine gives one project; its inference is stopped …`.
- **Pressure** (30 s samples): level 1 in 269 samples, level 2 in 18, level 4 in 7. Not a stop rule.
- **Slice worker pids** over the try: 90071, 62622, 92285, 4880 (one per step).
- Setup tasks: not reached. Marker absent.

## Try 2 (20:06:48Z to 20:30:14Z, 1406 s): `native_rc=1`, reference `bd0b2e16`

- Bootstrap log, last line before the end: `The resident service met an unexpected failure (reference bd0b2e16); try again, the details under that reference are in the service log …/host-f13cda8149a9.log.`
- `grep -a 'request-failed bd0b2e16' -A12` (whole output in `round-30.reference-try-2.txt`): `2026-11-15T20:29:35.540Z request-failed bd0b2e16 Error: Inventory record changed while opening` at `readInventoryRecordLeanV1` (`🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:16:92`), called from `🏗️build/👁️observation/📥️reuse/🟨️.mjs:71:68` and `🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491`.
- Highest step: no `cut smaller` line in this try (a new service, `host-f13cda8149a9.log`, started with `graph: no usable generation (invalidated), building the first one`).
- `graph:` lines of the new service, verbatim: `plan stage inference starts (managed 202 MB, resident 936 MB)`; `asking the host for up to 4 worker processes for 4774 projects; the projects wait for them`; `inferring projects with 2 worker processes (1536 MB each)`, then twice `with 1 worker processes (1536 MB each)`; `@semio-tech/hub-layout-drawing-rs waited 68 s for 64 MB of headroom (0 MB free, 1 project(s) running) and now runs because it fits the 1756 MB one project is granted`; `17 of 4774 projects inferred by 4 worker processes in 1159 s (0 failed, 0 lost, 0 restarts)`; `plan stage inference took 1164.6 s (managed 1686 MB, 1484 MB more than at its start, resident 2496 MB)`; `plan answered, managed 108 MB, resident 172 MB (released 0 MB)`.
- Observation: 17 projects in 1159 s is about 68 s per project; the service log's last counter line reads `Inferring projects: 283 of 4774 projects`. Which of the two counts is the progress of this try: not measured.
- Peak resident size of the slice worker: 3681 MB (5 s samples). Closure entries 40799 before, 40802 after.
- Setup tasks: not reached. Marker absent. Treated as a retry case (`try again` wording); try 3 follows after 5 minutes.

## Try 3 (20:35:17Z to 23:44:04Z, 11327 s): `native_rc=75` after step 6 of 6

- **Highest step reached: step 6 of 6.** `cut smaller` lines, verbatim sizes: `grew to 7935 MB … (step 2 of 6`, `grew to 8190 MB … (step 3 of 6`, `grew to 10048 MB … (step 4 of 6`, `grew to 10159 MB … (step 5 of 6`, `grew to 8928 MB … (step 6 of 6` (each `more than the 6553 MB a plan may take on this machine`; full lines in `round-30.steps.txt`).
- **Refusal after step 6**, verbatim: `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9545 MB, more than the 6553 MB this machine gives a plan, and only 1663 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.`
- So the expected result of round 30 (setup's graph part finishes on a 16 GB Mac) was **not** seen here: one project at a time still grew past the plan limit.
- **Peak resident size of the slice worker:** 6766 MB in the 5 s samples (the cut lines and the refusal name up to 10159 MB). No hard stop by the runner.
- Fleet: `35 projects taken from the fleet with 0 closure entries, 32 of them did not hold here and were inferred again, 0 not shared, 15 inferred here` (21:52Z). No `waiting for the fleet` line.
- Closure entries 40802 before, 41039 after.
- **Overlap with round 31.** Round 31 (20:55 UTC) was first read at 23:45Z, after this try had ended; the try ran from 20:35Z, so it was not interrupted and no further try was started (the runner was stopped at 23:45Z, between tries). The fleet agent's `ready` child ran its `workspace` stage 8 times between 22:35Z and 23:42Z, while this try held the graph: each ended after about 95 s with `the slice worker ended 4 times in a row with exit code 75 because the code on this machine kept changing under it`. Those 8 attempts therefore competed with my try.

## End Of Round 30 On g1-m01

Not ready: three tries (130 by my runner's error, 1 on `Inventory record changed while opening`, 75 after step 6 of 6). Setup tasks never reached; marker `.cache/f/native-bootstrap.done` absent. `state.json`: round 30, try 3, `native_rc` 75.
