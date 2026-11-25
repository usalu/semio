# F43 result: the no-progress watch under load (20:50)

## Why the 20:24 stop happened (log of that read is gone: `read.log` is from 19:09; findings from code)
- The read started 19:12, after landing A (19:00-19:12), so it most likely ran the F30 code (own CPU counted). It still stopped for three causes, all in the progress sample:
  1. `takeWorkCpuV1` returned undefined whenever `ps -A` failed (5 s timeout, easy at load 85-108); `createCpuProgressV1` then counts no progress, the own-CPU term was thrown away with it. Fixed: own CPU stays in the sample.
  2. The CPU threshold was fixed at 10 % of one core per 10 s tick. At load 100 on 10 cores a runnable thread gets about 10 %, right at the edge, and below it every tick counts as "no progress". Fixed: threshold divided by the load stretch.
  3. The window (300 s x speed factor) is static (cores/memory only), not load aware; `stretchedBudgetMillisecondsV1` is a test-only helper. Under load 85-108 a 390 s silence is normal for a time-sliced owner.
- Not proven: whether the process ran pre-A code; if it did, cause 0 is simply the old code. Check `read.log` of the next read.

## Landed (T/landings.md "Wave 1125 F43")
1. Window scales with load: allowed silence = window x min(12, load/cores).
2. Positive stuck signal: stop only if silent >= stretched window AND work CPU delta < 50 ms AND load/cores < 0.7; else extend (`alive`/`starved`); hard ceiling 60 min (schema `noProgressCeilingMilliseconds`). Extension also re-enables the 60 s tail reports.
3. Quarantine: on a stop, running owners are written atomically to `<data dir>/owner-quarantine.json` (TTL 2 h); the next pass (the existing restart) puts them in `alone` (run last, alone, in a child under the farm of one worker, whose own job timer retries 2x then sets aside). Not done: killing an in-process owner without stopping the pass (synchronous, uncancellable) and per-owner kill in the farm; the farm already times out CPU-silent jobs after 30 min x speed.
Tests: no-progress 20/0 (7 new, one assertion of the old undefined-on-ps-failure behavior removed on purpose), tail-report 11/0, slow-last 5/0, owner-withheld 4/0, too-heavy 7/0. No read started.
