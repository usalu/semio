# T5: remote-execution scheduler test file hangs (2026-11-24 18:54-19:15)

RE = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/`. Lock `lock.tests-remote-execution` (released). Landing: `T/landings.md` (T5). Scripts: `T5-scripts/watch.sh` (watchdog around `repo test-file`), `T5-scripts/edit-t5.py` (the one-burst edit, anchors checked first).

## 1. Evidence
- Run 1 (`repo test-file`, watchdog): the bun test child (`bun test --timeout 1672339 …/🗓️scheduler/🧪️tests/🟦️.ts`) sat at 0.1 % CPU for over 5 min. `sample 87277 5`: main thread 3930 of 3938 samples in `kevent64` (idle event loop, no busy loop, no lock): a promise that never settles. `repo test-file` buffers bun's output and drops it on cancel, so the run could not name the test. The suite's per-test timeout is 1,672,339 ms (28 min, the stretched suite budget). That is why one stuck test holds the whole file. CAS8 saw the same two hangs (560 s, 285 s at 0 % CPU).
- Run 2: I added a per-test bound to the file (section 3). Result 26 pass / 1 fail in 67.9 s: `(fail) an action nobody can run fails after the grace period; one the cache already knows is answered without a worker; the queue is bounded [63120 ms] ^ this test timed out`.

## 2. Cause: product defect (the queue bound was not enforced under concurrent submissions)
- In `RE🗓️scheduler/🟦️.ts` `prepare()`, the order was: the `maximumQueued` check, then `await this.inputs.hold(...)` (O25's input-root lease), then the push. The test submits three actions at once with `maximumQueued:2`. All three pass the check while the queue is still empty, all three await the lease, and all three are queued. The test then waits for `third.done`, expecting code 8 ("Execution queue is full"). That operation sits QUEUED forever (sweep disabled, linger off while the client is attached), so the wait never ends.
- A real scheduler has the same flaw: concurrent submissions can push the queue past `maximumQueued`.
- Landing: O25 itself names this bug ("first placement had an await between the bound check and the push … the lease now comes before the check. Fixed", scheduler 27/0 at 14:4x). Its `O25-patched/edit-o25.py` also writes hold before the check. The live file (mtime 17:38:45, sha256 6f7e1788df814a14, content committed by the 18:46 auto-commit `8d17d8ba399`) has the hold after the check again. No report or landing claims the 17:38 write: CAS8, H4, W10, W11 and R5 all deny touching the scheduler. The evidence fits a stale write that undid the ordering, but I cannot name its author. Everything else in the 17:38 image matches O23 and O25 (`git diff 3b15468f901 8d17d8ba399`).
- Ruled out: lease-renew timer (`renewInputs` only runs from `sweep`; the sweep timer is `unref`ed, and in tests `sweepMilliseconds:0`); journal handle (the journal is written by the sidecar, not the scheduler); H4's load-report change (the scheduler tests use no load reports in the hung test); a lease/missing-blob deadlock (`Memory.missing` resolves at once).

## 3. Changes
- `RE🗓️scheduler/🟦️.ts` 6f7e1788df814a14 -> 2f5853e117158f6a:
  - The lease and the missing-blob answer now come before the queue-bound check, so the check and the push run without an await between them. A refused submission still releases its lease through `finish`.
  - `close()` sets `closed`, clears the linger timers and wakes every subscriber. `watch` returns when it is closed, and `detach` sets no new linger timer after close. A shutdown or handover therefore never waits on an open operation stream, whatever the transport does (gRPC `stop` already destroys sessions). The client re-attaches by name (O23).
- `RE🗓️scheduler/🧪️tests/🟦️.ts` 5a8d006acda2ac5f -> 724a0510c6756413:
  - `setDefaultTimeout(stretchedBudgetMillisecondsV1(20_000))`, so a test that never settles fails with its name instead of holding the file for 28 min.
  - New test: "closing the scheduler ends every open operation stream at once, queued or executing".

## 4. Tests (`repo test-file`, one at a time, host load ~50)
| file | result |
|---|---|
| scheduler (before the fix, with the bound) | 26/1 (the queue-bound test timed out) |
| scheduler (after) | **28/0** (9.8 s) |
| `👷️worker/🧪️tests` | 17/0 |
| `🧪️tests/🌐️fleet/🔁️restart` | 5/0 |
| `🚀️offload/🧪️tests/⏱️timeouts` | 15/0 |
| remote-execution main `🧪️tests/🟦️.ts` | 17/0 |

## 5. Live and open items
- This change is in the live scheduler's closure. The scheduler sidecar restarts once by its own quiet-period rule (300 s quiet, 1,800 s at the latest). The journal restores operations, and clients with the O23 patch ride through. I restarted nothing by hand. Everything landed in one burst (19:1x) plus one follow-up line (`detach` guard) under a minute later, before any restart.
- Open: the author of the 17:38 write is unknown. Any lane that re-applies an old image of `🗓️scheduler/🟦️.ts` will bring the bug back. The new per-test bound now turns that into a named failure within a minute.
- Open (from O25): `restore()` still takes no lease for restored operations.
