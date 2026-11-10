# Lane `suites`

## Findings

1. Memory `reservation` (3 fail): NOT the fixtures and NOT the governor. `readLiveHostCapacityReservationV1` (`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts`) threw `ReferenceError: readBootAnchor is not defined` (that helper and `bootAnchors` are private to `📖️status`); `graphMemoryAllocationV1` swallows it (`try{...}catch{}`), so every live grant read as "no reservation" and the parent fell to the declared minimum 1024 MB. Fix: use the exported `hostBootAtV1(directory, at)` (same anchor semantics: stored anchor, per-process cache, written on first use). Result: reservation 5 pass; host-capacity suites (all 26 files) 0 fail.
   - Follow-up worth doing: `graphMemoryAllocationV1`'s bare `catch{}` hides a programming error as "no grant"; it should only swallow the documented absence.
2. Observation `blocks` (18 fail): not the owner record key. Child reports `Original captured runtime entry provider required`: the suite needs the captured runtime of the suite runner (`repo ./📜️script.ts test build-observation`, each file in its own process with launchers); `bun test` alone cannot supply it. Same for `presence` (36), `projects` (2), `incremental` (1: `Original artifact namespace and immutable SDK required`). Could not run through the runner: the scheduler never admitted it (`need 964 MB; 924-929 MB spare above the 1536 MB floor`, 6 minutes), so these are UNVERIFIED, not failing for a code reason. Run: `repo scheduler run --key=build:test-build-observation --peak-mb=2500 -- repo run repo:test-build-observation --output=static`.
3. `generations/🧪️tests/🌊️stream/🔌️external`: 4 pass with the variable set.
4. Observation `reuse` test "neutral portable expansion ..." failed with `exports is not defined`: the test transpiled the memo statements of the split graph modules, which are now `export`ed. Fixed in the test (strip leading `export`). The two other `reuse` failures (SIGINT: expected status 130, got 1) are birth/cancellation tests that need the runner like item 2.
5. Path length: `SEMIO_TEST_ARTIFACT_DIR` under the ticket (`TICKET/🗑️generated/suites/artifacts`) exceeds the 256 char materialized path limit: `file-record-image` (2) and `custody/🔑️kernel` (8) fail with it and pass (3 and 7) with a short dir (`/tmp/claude-1000/a`). Use a short artifact dir.
6. `generations/🧩️claims/🏃️processes` queued SIGKILL test: NOT a leases kernel bug (retracted request, marked done). The worker `hang` awaited `new Promise(()=>{})` with no reference; the collector reclaimed the suspended frame together with its lease, whose database closed and released the SQLite lock. Worker now roots the pending promise; suite 8 of 8 (new test: a fresh exclusive lease held by another process refuses the nonwaiting probe and a second fresh claim, then both succeed after release). Reproducer `🧰️scripts/suites-hold.ts` had the same flaw; kept as a record.

## Follow-up round

1. Bare catches: `graphMemoryAllocationV1` no longer catches anything (the reservation read returns `undefined` for no grant and throws for a malformed plan; the plan parse throws). `host-capacity` read path narrowed: `readJson` returns `undefined` only for ENOENT/ENOTDIR/SyntaxError and rethrows the rest, `modified` only swallows absence, `readAttached` only absence, and the two dead try/catch wrappers around `readJson` were removed (`absentV1` exported from `📖️status`). Test `an unreadable grant row surfaces its error and is never read as the declared minimum` (directory in place of the grant file) in `🧠️memory/🧪️tests/🎫️reservation`: 6 of 6. Not changed: the remaining best-effort `catch{}` blocks in `host-capacity` (unlink/utimes of own files in cleanup, heartbeat touches, `/proc` scans, estimate store housekeeping): they never feed a decision; narrowing them to ENOENT is a separate pass. host-capacity suites: 0 fail.
2. Long artifact dir: the two tests did not need the caller dir at all; the 256 character budget applied to paths under it (`Materialized absolute path`, block staging path). `custody/🔑️kernel` and `file-record-image` now derive their scratch under `os.tmpdir()` with short prefixes (`ck-`, `fri-`); both pass (7 and 3) with the long ticket artifact dir.
3. Runner (see below).


## Files changed

- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts` (reservation read uses `hostBootAtV1`).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📥️reuse/🟦️.ts` (strip `export`).
- Ticket: `🧰️scripts/suites-run.sh`, `suites-debug.ts`, `suites-fresh.ts`, `suites-hold.ts`, `📌️requests.md`.

## Counts (`repo tools exec --light bun -- test ./<file>`, 73 files under memory, observation, generations; helper files excluded)

First pass with the long artifact dir: 65 files 0 fail, 983 pass. After fixes with the short dir:
- memory: `tests/🟦️.ts` 11 pass; `🎫️reservation` 5 pass.
- generations: all files 0 fail except `🧩️claims/🏃️processes` 6 pass 1 fail (queued SIGKILL, leases kernel); `custody/🔑️kernel` 7 pass; `stream/🔌️external` 4 pass.
- observation: all 0 fail except runner-only suites: `blocks` 0/18, `presence` 0/36, `projects` 0/2, `incremental` 2/1 fail, `reuse` 11+ pass, 2 fail (SIGINT birth tests); `file-record-image` 3 pass; `field` pass.
3. Runner-only observation suites (`blocks`, `presence`, `projects`, `incremental`, `reuse` SIGINT): submitted once (`repo scheduler run --key=build:test-build-observation --peak-mb=2500 -- repo run repo:test-build-observation --output=static`, log `🗑️generated/suites/obs-suite2.txt`, timeout 2400 s). The first job waited 900 s and timed out; the second has waited more than 6 minutes: the host never has the 964 MB the run needs above the 1536 MB floor (about 925 MB spare, load 9 to 14, other agents). STILL UNVERIFIED. Several orphaned `bun` claims workers from earlier hang tests (about 25 MB each, hours old, plus two of mine from manual experiments) remain; I may not kill processes, so they are listed for the coordinator (`ps -eo pid,args | grep worker/🟦️.ts`).

## Last round

- Process leaks: every spawn in the claims `🏃️processes` suite now goes through `spawnV1`; `afterEach` SIGKILLs and awaits every spawned child (also after a failed or timed-out test) and asserts none of them is alive. 8 of 8 pass and no new processes remained. The other suites I touched spawn nothing (`file-record-image` only uses `spawnSync`). The old orphans (six `bun` processes: four from hours ago and two of my manual experiments) were not killed.
- Runner job (`obs-suite2.txt`): admitted after 907 s, then stuck in the project graph planning (`owners 55 of 726` after 686 s of reading, the known owner stall of the 32nd pass) and ended at its 2400 s timeout before any test ran. So `blocks`, `presence`, `projects`, `incremental` and the `reuse` SIGINT tests are still UNVERIFIED; the runner cannot reach the tests until the planning of the owners finishes. Not queued again.
