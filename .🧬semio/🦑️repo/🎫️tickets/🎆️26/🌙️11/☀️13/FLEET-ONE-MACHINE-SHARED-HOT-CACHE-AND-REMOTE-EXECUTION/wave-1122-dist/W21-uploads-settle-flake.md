# W21: "uploads did not settle" flake in the unit-cache fleet main test (2026-11-25)

UC = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/`. Lock `lock.unit-upload` was taken at 09:47 and removed after landing. Nothing was restarted, nothing ran against a live store, and no scratch build ran on the coordinator. Every test ran through `repo test-file`, one at a time, with the 1-minute load below 30 (`W21/run-gated.sh` waits for that). Probe scratch under `$TMPDIR/w21-*` is deleted.

**Verdict:** these are real races in the uploader's start and stop path. The test's timing was not the problem. Each race leaves spooled units with no live drainer, so in production they would sit until the next build or the sidecar sweep. That is lost or late sharing. Fixing the code made the fleet main file deterministic.

## Root causes

### 1. A changed credential is never handed over when the build starts within 3 s of the last worker start (the flake W18 hunted)
Before the fix, `UC🛰️fleet/📤️uploader/🟦️.ts` `startUploaderV1` returned on the spawn throttle (`uploader.spawnThrottleMilliseconds` 3 s) *before* it wrote the uploader identity file. Here is the sequence in the test:
- Build 1 runs with the wrong token. Its drainer D1 (identity `…5645a758`) defers every job as `denied`.
- D1 lives as long as any job waits, and it exits only when the identity file names someone else.
- Build 2 runs with the right token. All of its units are local hits, so the build is short.
- If every `startUploaderV1` call of build 2 falls within 3 s of D1's spawn stamp, the identity file is never rewritten and no successor is spawned.
- D1 then holds the lock with the wrong credential indefinitely (until the 6 h job age). The test's own `drainUploadsV1(idle 0)` gets `busy` forever: 587 of 587 calls in the probe.

Proof: `W21/probe.ts` replays the test steps and samples the lock holder, identity file, stamp and spool every 100 ms.
- Old code: **2 of 3 runs wedged** (`W21/🗑️generated/run-2.jsonl`, `run-3.jsonl`). Build 2 ran from 2.5 s to 4.3 s against a stamp at 1.47 s, and the identity stayed `5645a758`.
- The passing run 1 had build 2's last call at 7.1 s, more than 3 s after its stamp at 3.0 s, so its takeover happened.
- Fast builds (low load) make the failure more likely. The run that timed out at 300 s in W18 was the same wedge under a stretched budget.

### 2. A unit queued just after the drainer went idle has no worker (found by this lane; the fleet main tests "two machines race" and "manifest directory" failed with "the upload worker did not finish")
The drainer exits once the spool has been empty for `idleMilliseconds` (1.5 s in the test). Two lost-wakeup windows:
- **a.** A job is queued while the ending drainer still holds the lock. `startUploaderV1` sees a live worker and does nothing. The drainer has already decided to exit.
- **b.** A job is queued right after the drainer exited, but within 3 s of that drainer's spawn stamp. The throttle cannot tell "a worker is booting" from "the worker already left", so it spawns nothing.
- The race test hits this exactly: `core` is delayed 2.5 s. The drainer uploads `mac` and the build scripts, goes idle and exits. `core`, `leaf1`, `leaf2` and `app` are then throttled.
- Evidence from the new diagnostics in `waitForUploader`: `waiting: leaf1 try 0 …; core …; app …; leaf2 … | lock holder: none` (runs 8 and 11). After fix 1 alone, the fleet main file failed 3 of 16 runs this way.

## Changes (`T/landings.md`, W21)
1. `UC🛰️fleet/📤️uploader/🟦️.ts` `startUploaderV1`: the spawn stamp now holds the cache identity of the last start. The throttle holds only if that identity equals the caller's. A build whose credential, endpoint or generation changed therefore always hands over at once: it writes the identity and spawns a `--takeover` successor, and the old drainer leaves at its next loop. A storm under one identity is still throttled.
2. Same file, `drainUploadsV1` is now a loop over `drainPassV1`, and the results are summed by `sumDrainV1`. The release order on an idle end is fixed:
   - release the lock;
   - clear the spawn stamp;
   - look at the spool again, and run another pass if it is not empty.
   - This closes both windows. An enqueuer writes the job before it checks the lock and the stamp, so either the releaser sees the job, or the enqueuer finds no worker and no throttle and spawns one.
   - A pass that ended for any other reason (identity changed, aborted, `idle 0` with jobs that are not due yet) returns as before.
3. `UC🛰️fleet/📤️uploader/🧪️tests/🟦️.ts`, two new tests:
   - "a build with another credential hands the spool over even within the spawn throttle …". An in-process drainer holds the lock under the denied credential and the stamp is fresh. `startUploaderV1` with the right credential must return `true` and write identity and stamp. A second call returns `false`. The old drainer's promise resolves, and the spawned successor uploads the job (the test awaits `hasResult`). On the old code the first assertion is `false`.
   - "a worker that ends idle leaves no spawn throttle behind …". Two rounds of enqueue + `startUploaderV1` → `true`. Each round awaits the worker's own end signals (result stored, spool empty, lock released, stamp cleared). There are no wall-clock asserts.
4. `UC🛰️fleet/🧪️tests/🟦️.ts`: `settled` and `waitForUploader` share `uploaderState`, which reports waiting jobs, the lock holder (pid, alive or gone, heartbeat age) and the last uploader events. The test laws are unchanged.

## Tests (`repo test-file`, load < 30, one at a time)
| file | result |
|---|---|
| uploader `📤️uploader/🧪️tests` | 12/0 after fix 1, **13/0** after fix 2 |
| fleet main `🛰️fleet/🧪️tests/🟦️.ts`, fix 1 only (runs 1-16) | 13 of 16 green; 3 failures from cause 2 (runs 1, 2, 11 race test; run 8 manifest-dir test), never again "uploads did not settle" |
| **fleet main, final code (runs 17-22)** | **6 of 6 in a row, 18/0 each (108 pass, 0 fail)**, load 17-29, 82-96 s each |
| probe, final code (`W21/🗑️generated/after/`) | 6 of 6 settle in 7-13 s (old code: 1 of 3) |
| race probe `W21/race.ts` (fix 1 code) | 5 of 5 (did not reproduce outside the suite; the suite's runs did) |
| fleet `🥀️stale` 1/0, `🌳️rooted` 4/0, `🧷️session` 2/0, UC `📋️manifest` 1/0 | pass |
| offload cluster `☁️fleet-store` | 0/1, **pre-existing**: it fails identically with both uploader changes reverted (A/B run `🗑️generated/fleet-store-old.txt`, file restored to b8c57c9e at once) |
| offload cluster `🔏️signed-fleet` | 0/1, **not this lane**: the offload itself fails ("der Ausführungsdienst ist ausgefallen", the execution service is down) and the requester compiles locally, before any upload |

## Open
- `☁️fleet-store` ("the unit entry is stored with a slot") and `🔏️signed-fleet` (offload service down in the test cluster) fail on the current tree regardless of this change. They were last reported green in wave-1122 E31.
- In the "idle end" test, the second-round assertion detects the old throttle bug only when round 1 finishes within 3 s. The stamp-cleared wait detects it every time.
- Production effect: both fixes take effect at each worker's next build, when the wrapper spawns a new drainer.
