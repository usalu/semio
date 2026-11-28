# Warden Guard Level: Root Cause and New Level Law

## Root cause
The warden reads three sources that all say "compressed memory", and it counts swap traffic from a single sample.

1. **Derived pressure.** The host sampler (`📡️sampling`) sets `pressure=warn` when compressed memory is at least 35 % of RAM (`compressedWarnBasisPoints`), whatever the kernel says. On this 16 GB Mac compressed memory sits at 3.2 to 7.9 GB (20 to 48 %) while `kern.memorystatus_vm_pressure_level` is 1 (normal) and 57 % of memory is free. `pressure=warn` alone is level `guard`, and guard holds every `cargo` admission (`holdOperations`), whatever the unit needs. That is the "unit proc_macro2 is held back ... level guard (4159 MB available, needs 256 MB)" line.
2. **Compressed memory counted twice more.** The compressed depth line (35 % guard, 50 % danger) also set `guard` and, together with `pressure=warn` and a non-calm previous level, the `acute` flag (`sustainedWarning`). So the same number made a level of its own, a pressure of its own and the "acute" condition.
3. **Swap traffic from one sample.** `swapOutMegabytesPerSecond` is (swapouts + swapins) per second over the last tick, and the line was 16 MB/s (`swapActiveMegabytesPerSecond`). One sample at or above it plus guard-depth compressed memory (always true here) was `acute`, hence `severe`, hence `danger` with cancels (`trigger: swapping`). Available memory was 2.6 to 4.4 GB in all 390 swapping alarms against a danger line of 819 MB.

Evidence (all from `~/Library/Caches/semio/host-capacity/warden/alarms.jsonl` and `warden.log`, and a 240-sample series of `state.json`, 1 sample per second, 04:50 to 04:54):
- Alarms since the file began: danger suspend 244, danger cancel 132. Every danger action had `pressure warn`; triggers: swapping 390, pressure 41, compressed 9. Available memory at those actions: 2396 to 4396 MB (median about 2800), swap traffic 16 to 973 MB/s (median about 70).
- 04:3x cancels: 04:36:43 cargo 588 MB and stage with 2731 MB available and 530 MB/s; 04:39:21 cargo 971 and 858 MB with 2907 MB available and 285 MB/s; 04:00 to 04:31 the same with 17 to 83 MB/s.
- Sample series: available 2735 / 3464 / 5086 / 5874 / 6106 MB (min / p50 / p90 / p99 / max); swap traffic 0 / 0 / 2.9 / 10.9 / 285 MB/s, with a single sample above 64 MB/s and no two in a row; compressed 3214 / 6547 / 7470 / 7784 / 7893 MB; swap used 2.9 to 3.3 GB. Old law on the series: 146 of 240 samples `guard` (trigger pressure, all from compressed memory), 1 `danger` (a single 285 MB/s sample), 93 calm. New law on the series: 240 calm (kernel normal assumed; with the recorded pressure kept as the worst case 147 guard, 93 calm, 0 danger).

## New rules
- **Kernel pressure** stays the authority: critical is danger whatever the available memory (suspend and hold all admissions; cancels only on a severe shortage as before). Derived pressure no longer comes from compressed memory alone: the sampler counts the compressor only while the available memory is below its warn share (`availableWarnBasisPoints`, 15 %), so a host with memory to spare and the kernel at normal reads `normal`.
- **Swap traffic** counts only when sustained: at or above `swapActiveMegabytesPerSecond` (16 to 64 MB/s, above the 99th percentile of 10.9 MB/s and 22 times the median-to-p90 span) for `swapSustainedSeconds` (10 s). A leaky timer (`swapMilliseconds`, carried tick to tick in `previous` and the decision, gained by the sample interval `intervalMilliseconds` on a busy sample, lost on a quiet one, bounded to twice the span) keeps a burst from counting and a thrash from flickering. Every use of "swap worked" (guard by swap, `quiet`, acute) uses the sustained value.
- **Acute** (severe, cancels) = sustained swapping and OS pressure at least warn and available memory below the resume line (15 %, 2457 MB). Compressed memory takes no part. Real shortage that stays: available below the danger line (819 MB), headroom below its danger line, swap or compressed memory at danger depth with available below the guard line, kernel pressure critical.
- **Compressed memory is reclaimable:** its lines rise from 35 % / 50 % to 55 % / 70 % of RAM (9011 / 11469 MB), above the measured healthy maximum of 7.9 GB (48 %).
- **Admission:** at level guard (never at danger) a request passes when the available memory minus its need stays at or above the guard line (`hostWardenHoldsV1`): a 256 MB unit passes at 4159 MB, a 2600 MB one is still held. Danger holds everything as before.
- **Gate swap stop** (`hostSwapStopV1`, shipped `hostCapacity.swap.activeBasisPointsPerSecond`): 0.6 (1 MB/s) to 10 (16 MB/s). With swap in use above 25 % and available memory below 25 %, ordinary paging at 1 MB/s or more (the median to p90 here is 0 to 3 MB/s) was a hard stop for every admission.

## Live host, old versus new (`repo capacity status --json --lang=en`, 05:00 and 05:05)
The running warden restarted on the new code (digest 956128f676029d1c) at about 04:57 and has made 0 cancels since; the last cancel is 04:40:16 (old code).

| sample | old law | new law |
| --- | --- | --- |
| now: 3025 MB available, 1.7 MB/s, compressed 7400, swap used 6820, kernel warn | guard (pressure), 256 MB cargo held | guard (pressure), 256 MB cargo admitted (3025 - 256 is above the 1638 MB guard line) |
| 05:05: 4344 MB available, kernel normal | calm | calm |
| 04:36:43 cancel sample: 2731 MB, 529.75 MB/s once, compressed 7860, warn, previous guard | danger, cancels | guard, nothing cancelled (not sustained) |
| 04:3x admission sample: 4159 MB, 2.5 MB/s, compressed 7761, derived warn | guard, unit held | guard, unit admitted |

Scripts (inputs, kept): `warden-sample-state.py`, `warden-level-replay.py`, `warden-live-compare.py`; the old law is the HEAD oracle read with `git show`.

## Tests
- New rows `🛡️warden/🧪️tests/🧘️healthy-paging/🔣️.json` (9 cases: live healthy sample, one burst, 120 MB/s for 20 s with the kernel normal, 400 MB/s for 20 s with warn and 3200 MB, compressed 9000 MB alone, real thrash cancels and holds, the same thrash for 6 s does not, kernel critical, available below the danger line) checked by TypeScript (`🟦️.test.ts`, 2 tests) and by the Python oracle (`🐍️.py --paging`).
- Oracle `🐍️.py` mirrors the timer, acute, the guard-level admission and the new decision field; fixture `🧫️fixtures/🔣️.json` regenerated with `--write` (`--check` ok): 13 rows changed meaning (compressed lines, sustained swap) and 3 hold rows were added.
- Schema `🧬️schema/🔣️.json` of the warden: `swapActiveMegabytesPerSecond` 64, `swapSustainedSeconds` 10, compressed lines 5500/7000, `Host.intervalMilliseconds`, `Previous.swapMilliseconds`, `Decision.swapMilliseconds`, measured data in the policy description (English and German).
- `repo test-file`: warden 60 pass, healthy-paging 2, swap-churn 4, grant-shelter 2, sampling 13, warden-hold 3, scaling 4, unit-admission 15, shared-budget 11, native-cargo-grant 4 (pass on rerun; one run flaked on timing), memory-fit 4. The whole host-capacity test directory ran 46 of 49 files green; `🌤️followup` (`followup-2`, queue-order versus memory) and `🧾️blockers` (`upgrade-8`) fail on the head-of-line and upgrade work of other agents in `📏️decision` (not touched here; their fixtures have no swap or pressure input). The warden MOUNTED suspend test is timing flaky on a loaded host (failed once, passed twice).

## Not changed
- The `repo` product policy `compressedWarnBasisPoints` (35 %) and critical (55 %) keep their values; only their derivation rule changed.
- `cargo` stays a cancel-only kind; victim choice, claims and sheltering are unchanged.
