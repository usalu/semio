# Graph-plan starvation deadlock: root cause, fix and probe

`BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `CAP` = `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity`. Follows `graph-slice-starvation.md`.

## Root cause

The 8192 MB is not the `estimates` policy (`maximumShareBasisPoints`, `fallbackCeilingMegabytes` size task requests, not the graph plan). It is the slice pool's own booking:

- `BUILD/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` (`run`): with no cgroup limiter (macOS) `booked = max(min(elasticity.maximumMegabytes, learned.read()), unlearnedMegabytes)`.
- `learned.read()` is the peak file `$TMPDIR/semio-slice-peak-<uid>.json` = 8607 MB, learned when a whole-closure build ran in one process (before the 48-owner waves). The cap `sliceHostElasticity.maximumMegabytes` (`BUILD/🏡️resident/🧬️schema/🔣️.json`) is 8192. So every graph plan asked for 8192 MB in one piece, for ever (the learned peak only grows).
- `reserveGraphPlanCapacityV1` (`BUILD/🌡️host-capacity/🟦️.ts`) turns it into a fixed gate request; `evaluateHostCapacityV1` (`CAP/📏️decision/🟦️.ts`) waits while `need > headroom`. On a 16 GB host with 5.6 GB available and a 2 GB floor the headroom is 3.6 GB: never granted, and nothing in the gate ever reduces a fixed request (only the 120 s `override` for non-memory reasons existed). Yesterday it ran only because the peak file was smaller then.
- The same hole exists for any fixed request, e.g. the `course-build` class ceiling (12288 MB) and the `show`/`graph`/`dependencies` operations (5120/6144 MB): `need()` is capped by `total - floor` (14336 MB), not by what the host can obtain.

## Design and change

Two layers, both pure-function decisions, degrade to slower, never to never:

1. Slice pool clamp (before asking). `sliceAdmissionV1` in `BUILD/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs` (+ `.d.mts`): reservation = `min(requested, max(minimumReservationMegabytes, (available - hostFloorMegabytes) * hostShareBasisPoints))`, plus the granted share of the request as `waveBasisPoints`. The pool (`slice-pool/🟨️.mjs`) reserves the clamped amount, emits the progress decision `reduced` (new bilingual text `host-slice-owners-reduced` in `BUILD/🏡️resident/🧬️schema/🔣️.json`, en + de) and starts the worker with `SEMIO_SLICE_WAVE_BASIS_POINTS`. `scaledOwnerWaveV1` in `BUILD/🕸️graph/🕰️generations/🟨️.mjs` (+ `.d.mts`) scales the owner wave (48) to that share (never below 1); a worker memory is bounded by the wave, so memory scales down and the build is slower. Schema: new `sliceHostElasticity.minimumReservationMegabytes` (1024). Unknown availability or plenty of memory: unchanged.
2. Gate fallback for any grant (the host moves after the clamp, other fixed classes). `unsatisfiableHostRequestV1` in `CAP/📏️decision/🟦️.ts`: a request that waits for memory, is first in the queue, has waited `wait.bypassMilliseconds`, would not fit even if every live grant of ours ended (`drained < need`: only foreign processes hold the memory), is not quiet/elastic/replacing and has no soft stop is admitted with the room available now (at least `estimates.minimumMegabytes`; below it the host really has nothing and it keeps waiting). Applied in the wait loop of `CAP/🎫️gate/🟦️.ts` (`withHostCapacityV1`), not inside `evaluateHostCapacityV1`, so the SQLite/Python oracles of the evaluator are untouched. The grant is reported with the new bilingual `unsatisfiable` text (`CAP/🧬️schema/🔣️.json`, en + de; type list in the decision module). `reserveGraphPlanCapacityV1` now returns `megabytes` (what was granted).
3. Per-class entries (`course-build` 12288, `fixedMegabytes`): not lowered; they are covered by layer 2 (test with the 12288 ceiling). Their request is still capped by `total - floor`.

## Files changed

- `BUILD/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`, `…/🧱️limit/🟨️.mjs`, `…/🧱️limit/🟨️.d.mts`
- `BUILD/🏡️resident/🧬️schema/🔣️.json`
- `BUILD/🕸️graph/🕰️generations/🟨️.mjs`, `BUILD/🕸️graph/🕰️generations/🟨️.d.mts`
- `BUILD/🌡️host-capacity/🟦️.ts`
- `CAP/📏️decision/🟦️.ts`, `CAP/🎫️gate/🟦️.ts`, `CAP/🧬️schema/🔣️.json`
- Tests: new `BUILD/🏡️resident/🧪️tests/🛟️reservation/🟦️.ts` + `🔣️.json` (language-agnostic fixture, independent Python oracle; 8 pass: 16 GB / 5.6 GB / 2 GB floor / nothing running gives 3093 MB and 18 owners per wave, plenty gives 8192 unchanged, minimum, unknown availability, and a real pool test that reserves the clamped amount, reports `reduced` and starts the worker with `3775`); new `BUILD/🌡️host-capacity/🧪️tests/🛟️unsatisfiable/🟦️.ts` (7 pass: lone 8 GB plan on 5.6 GB gets 3612 MB after the bypass time, de text, course-build 12288 clamped, plenty unchanged, nothing above floor keeps waiting, grants of ours that hold little do not block while a request that fits once they end still waits, pure rule); updated `BUILD/🏡️resident/🧪️tests/📨️protocol/🟦️.ts` (new text key).

Command: `repo tools exec --light bun -- test ./🟦️.ts` per directory. Passing: host-capacity library 17/23 files above (`🟦️.ts` 23 pass, `🧮️graph-plan` 4, `🚪️gate` 31, `🔼️upgrade` 10, `🌤️followup` 11, `🪜️phases` 10, `📐️estimates` 14, `🤫️quiet` 4, `🧺️unmanaged` 4), resident `📨️protocol` 23, `🌡️pressure` 7, generations `🌊️waves` 5.
Not mine: `BUILD/🌡️host-capacity/🧪️tests/🏊️farm` (3), `👥️agents` (1), `🧾️label` (1) fail because `requestBuildFarmCapacityV1` now defaults to `policy.kinds.script.megabytes` (768) where the tests expect `kinds.build.megabytes` (2048); a concurrent change in `BUILD/🌡️host-capacity/🟦️.ts`, untouched by me.

## Probe

`repo service host stop`, then `repo run @semio-tech/print:test-toolchain --lang=en --output=stream --skip-remote-cache` (log `🗑️generated/starvation-probe.log`, exit `starvation-probe.exit`): exit 0 after 672 s (11 min). With the clamp, the slice reserved at once instead of waiting: `reservation 0 of 1966; … the host has 4158 MB available, so only 1966 MB are reserved and the owners are re-inferred in smaller waves …` (host was loaded, swap 3 GB, other agents' runs; the second graph-plan queued behind the first for about 2 min for real memory, then ran). The graph had meanwhile reached generation 101 (verified 5099 of 5099 by other agents), so the re-inference was small (assembly 31 of 31, about 100 s); the large wave scaling was therefore exercised only by the pool test, not by a full 4600-owner re-inference. The run then grew its reservation reduced (`2574 MB with 1 of 8 jobs`) and finished (2 tasks, critical path 19.8 s).
The first half of the probe ran on the service with only layer 1 (the gate fallback was added after seeing a second graph-plan wait behind grants of other runs that hold almost nothing); `repo service host stop` and the client's automatic restart then ran the rest on the final code. The gate fallback message did not fire in the probe (not needed after the other plan ended); it is covered by the test.

## Not verified / notes

- Full cold re-inference of ~4600 owners with the narrowed wave on a loaded host.
- The learned peak file (8607 MB) is left as is; the clamp makes it harmless.
- Logs in `🗑️generated/` are kept until you have read them.
