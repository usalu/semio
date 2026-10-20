# Result: lease-fair (executor)

Nothing under `📚️library/`, no `package.json`/`bun.lock`, no root `📜️script.ts`, no `🔣️taxonomy.json` and nothing else in the repo was edited. Everything is staged in `C/lease-fair-tools/` and applied by `C/lease-fair-tools/apply.sh` (refuses if a target changed since staging; it does NOT refuse because a sync runs, so stop the sync loop first).

## 1. Finding: the measured blockage is not reader starvation

The premise (13 `bun repo run` readers starve the sync's exclusive request) does not match the live lease state I inspected (00:52-01:30, read-only).

Resource `native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d` lives at `.cache/b/l/m/data/aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b.{sqlite,queue}` (sha256 of the resource string). Its `acquireQueuedResourceLease` queue is already a FIFO, writer-fair queue (tickets ordered by arrival, shared tickets join a cohort only if every earlier ticket is a shared cohort ticket, dead requesters reaped through an independent exclusive claim database, cancellation removes the ticket).

Evidence at 00:52 (all read-only):
- Queue head `00mvfpwtng-00009o0-…` = pid 12528, mode `exclusive`; 14 tickets behind it, all `shared`/`sharedCohort:true` (pids 9626, 52145, 55073, 20700, 18685, 44423, 44424, 85577, 44704, 76917, 41787, 72419, 3898 = `show projects`, 4620 = sync attempt 52).
- `lsof <lease sqlite>` showed ONLY pid 12528 holding the image lease database open (a waiter parks in the ticket loop and does not open it; a holder keeps it open).
- 120 of 120 read-only SQLite probes (`BEGIN; SELECT`, timeout 0, 13 ms apart) got `database is locked`: an exclusive transaction was HELD (a polling writer would let probes through between 40 ms polls). So pid 12528 held the image EXCLUSIVE for hours; nobody was starving it.
- pid 12528 = `bun repo run @semio-tech/repo-lib:test-build-caller-birth --skip-cache --output=stream` (parent chain 12525 -> codex 31454), started 21:58. Its log (`DASHBOARD-LAUNCH-COCKPIT/🗑️generated/e2e-build-caller-birth.log`, 318 MB) ends in `dependencies:store-ownership:{list,selection,state}` progress counters with mode `exclusive` and elapsedMs ~8.0e6: it was running the dependency STORE-OWNERSHIP WALK (the producer step, `runJavascriptDependencyStoreProducerV1`), the same work that ate sync attempt 50's last 99 minutes.
- Sync attempts 51/52 were genuine queue waits: their image activity events carry a growing `elapsedMs` (272, 15335, 30503 …) emitted by the 15 s queue-wait loop, i.e. "shared request waits behind the exclusive head".
- The lines `still waits for exclusive access to dependencies:store-ownership:state:N` in `deps4-sync50.log` are NOT waits: they are walk progress counters (`N` grows by 64 per event to 11,449,728 while `elapsedMs` is the constant-rate walk time 5.95e6 ms). The generic formatter mislabels any activity as "waits for"/"remains active on". Sync50 was a 99-minute walk (~1,900 entries/s under load average 180), not a lock wait.
- Current state (01:30): 12528 and the old sync pid 4620 are gone; sync pid 38019 (`dependencies sync --all`, 21 min old) HOLDS the image exclusive and one `bun repo dashboard` waits shared behind it. The FIFO works in that direction too.

Where the reader-preferring behaviour IS real: the raw, unqueued primitive `acquireResourceLease` (SQLite `BEGIN EXCLUSIVE` polled every 40 ms with `busy_timeout=0`). Measured with 5 overlapping readers (300 ms hold, staggered) and one writer (`C/lease-fair-tools/exp.ts`, overlay of the unmodified module):
- raw readers + raw writer: writer acquired after 7,730 ms (only when the readers stopped at 8 s) = starvation.
- queued readers (`sharedCohort`) + queued writer: 339 ms.
Mixing is the hazard: any unqueued shared requester starves a queued writer. The image paths use the queue unless `maximumLeaseDatabaseBytes` is set (then raw). I did NOT change the raw primitive to hold SQLite's PENDING lock while waiting: nested shared acquisitions in one process (or a child of a holder) would then deadlock behind a waiting writer and I cannot audit every raw caller. The queue is the sanctioned fair path; the new test pins the rule (see 4).

Consequence for the apply window: stopping the sync loop does not free the image while a peer run holds it exclusively for hours. Before restarting a sync, check the head ticket (commands in section 6). The structural cause is that an ordinary `bun repo run` runs the store producer/ownership walk under the image's exclusive custody after a node_modules generation change (inference from the log + lock state); walkperf's receipt verify (6-12 s) removes most of it. A durable fix is outside the lease layer (ordinary runs must not run the producer; only the sync should).

## 2. What the staged patch changes (all inside `🧰️framework/🔨️modules/🏃️process`)

1. Queue waiting no longer burns a core (`🔒️leases/🟦️.ts`, `acquireQueuedResourceLease` loop now `awaitQueuedTurn`):
   - A waiter reaps (tries the exclusive claim of) only its IMMEDIATE predecessor, chain-wise, instead of every other ticket including those behind it; per-ticket sweeps are rate-limited to once per `sweepIntervalMilliseconds`.
   - Poll interval backs off 250 -> 500 ms (x3/2). Constants live in new schema `⏳️wait/🧬️schema/📜️policy.json` (`WaitPolicy`, `Policy.const`).
   - Measured (`exp2.ts`, 14 shared waiters behind one exclusive holder, 15 s): total waiter CPU 15,195 ms -> 2,247 ms (including ~100 ms bun start-up per waiter); 4 s fairness test: 21-42 ms CPU per waiter vs 220 ms on the old code. This idle polling (about 7 percent of a core per waiter, 13 waiters for hours, many SQLite opens/s) is a plausible contributor to load average 180 and the ENFILE seen earlier.
2. Wait diagnostics that name the blocker (schema-first, localized EN/DE like `parserSource`): every 15 s a queued waiter at position >= 2 additionally publishes `lease-queue:{"version":1,"position":3,"queued":15,"holderMode":"exclusive","holderPid":12528,"holderSeconds":8040,"resource":…}` (elapsedMs 0). `formatScriptProcessActivityV1` renders: `bun:dependency-storage · Waiting for access to native-fixed-image:… (shared): position 3 of 15, behind the request of process 12528 (exclusive, queued 8040 s ago)`. New `queueWait` block + `QueueWait`/`QueueCases` definitions in `📈️progress.json`; the original event is unchanged.
3. Explicit upgrade rule: new `upgradeQueuedResourceLease(sharedLease, {timeoutMs})`. It rewrites the held shared ticket atomically to an exclusive one (keeps its ARRIVAL POSITION), releases the shared hold, then waits its turn. The shared lease is consumed; a failed upgrade holds nothing; the old lease's `release()` becomes a no-op. Two concurrent upgraders cannot deadlock (each drops its shared hold when it converts). The naive "hold shared and request exclusive on the same resource" still waits for itself until its timeout (test shows the bounded timeout); I did not add a throwing guard because a same-process concurrent task may legitimately wait for another task's shared hold.
4. Not changed on purpose: the non-atomic exclusive->shared "gap" in `completeFixedReceiverV1` (release, then re-queue shared at the back). It lets queued exclusives (the sync) run between a run's open phase and its long shared phase; an atomic downgrade would make every sync wait for the longest-running reader. Ticket and claim formats are unchanged, so old and new processes interoperate during the roll-out.

## 3. Files

Modified (7): `🔒️leases/🟦️.ts`; `🧭️routing/📥️invocation/🏃️process/🟦️.ts`, `…/🧬️schema/📈️progress.json`, `…/🧪️tests/📈️progress/🟦️.ts`, `…/🧪️tests/📈️progress/🔣️.json`; `📜️script.ts` (suite `resource-lease-fairness`); `📋️project.json` (target `test-resource-lease-fairness`).
New (5): `🔒️leases/⏳️wait/🧬️schema/📜️policy.json`; `🔒️leases/⚖️fairness/🧪️tests/🟦️.ts`, `…/🧪️tests/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json`, `…/🧪️tests/📥️worker/🟦️.ts`.
Staged copies: `C/lease-fair-tools/staged/<same relative path>`; base hashes: `bases.json`. Regenerate from the current tree with `stage.sh` (patch scripts `patch_leases.py`, `patch_progress.py`, `patch_register.py` assert their anchors and fail loudly). All paths are < 256 UTF-16 units (max 128 absolute).

## 4. Tests (`resource-lease-fairness`, language-agnostic fixture + third-party cross-check)

Fixture `fairness-fixture.json` (schema `fairness-schema.json`, Ajv-strict) declares every bound and case. 11 tests:
- starvation matrix over separate processes: queued bun readers + queued bun writer, queued node readers + node writer (both bounded, ~280-660 ms vs bound 4 s); Python `sqlite3` readers (independent third-party client) starve both a raw writer and a queued writer (documents the "all shared requesters must queue" rule).
- dead predecessors: 2 SIGKILLed queued requesters (bun, node) between a holder and a survivor are reaped while the holder still holds; the survivor acquires after release.
- cost: 8 waiters (bun/node) behind a holder use <= 4 percent CPU of the wait.
- progress: the real issued control emits the `lease-queue:` event (position 2 of 2, holder pid, mode) and the localized stderr line.
- upgrade: position preserved (a later exclusive waits behind the upgrader), concurrent upgraders never overlap, failed upgrade holds nothing, naive upgrade times out boundedly.
- fixtures/policy: closed schemas validated.
Progress suite: 8 new `queueCases` with an independent SQLite oracle (SQL `json_*`/`replace` builds the expected string), EN, DE, fallback, refusal cases.

## 5. Verification (exact commands; run on a scratch overlay of the real tree with the staged files, `C/lease-fair-tools/verify-overlay.sh` -> `/private/tmp/claude-501/lease-fair-ovf`)

- `bun test --timeout 120000 ./🔒️leases/⚖️fairness/🧪️tests/🟦️.ts ./🔒️leases/⏳️wait/🧪️tests/🟦️.ts` (SEMIO_TEST_ARTIFACT_DIR set): 22 pass, 0 fail.
- `bun test ./🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts`: 4 pass (with the wait suite: 15 pass).
- `bun wrap.ts <file>` (mints a real process invocation, like `bun repo run`): `📦️bounded/🧪️tests` 4 pass, `♻️retirement/🧪️tests/📋️claims` 4 pass, `♻️retirement/🧭️continuation/🧪️tests` 9 pass.
- `bun driver.ts` = `testResourceLeases` (the whole `test-resource-leases` suite: FIFO order, crashed waiter swept, cancelled waiter, namespace, retention, native cargo lease queue) with the concurrency step removed: `DRIVER-OK`.
- `bun wrapfn.ts … proveCargoBuildLeasesV1`: `FN-OK` (arrival order, crash sweep, cancel).
- `tsc --noEmit --strict --skipLibCheck … --types bun` over leases, formatter, new tests, worker, progress test: rc 0, no diagnostics.
- Negative control: the same fairness tests against the UNMODIFIED queue (`ov0`): cost test fails (220.51 ms > 160), progress test fails (no `lease-queue:` event).
- `apply.sh` exercised in a throw-away root: dry-run, apply, idempotent re-check, drift refusal, revert.

NOT run: `test-resource-leases` through the real `bun repo run` (overlay only), `native-storage-platform`/fixed image suites (hdiutil images; ran none), `test-resource-lease-fairness` through `bun repo run` (registered, unexecuted), `bun repo verify taxonomy`/name laws on the new `⚖️fairness` directory and `📜️policy.json` (run after apply), `.vscode` launch entry (`.vscode` no longer exists; the seed owner must add `test-resource-lease-fairness` when launch generation returns).
Pre-existing failures reproduced on the UNMODIFIED overlay, so not caused by this patch: `🧪️tests/⚔️concurrency` (its worker `📜️script.ts` registers only `cancel|retire|claim-read` but the test starts modes `hold`/`wait`: `unknown command "hold"`), `🎟️writer` (5 fail without a per-test process birth), `📦️namespace/🧪️tests/📥️pending` (EEXIST on its fixed root), `🧪️testing/🦀️cargo/🔒️lease` (needs nightly cargo `-Z`).

## 6. Apply plan and operator commands

1. Stop the sync loop (and wait for it to release the image: queue head must not be a sync ticket).
2. `C/lease-fair-tools/apply.sh check`, then `apply.sh apply --dry-run`, then `apply.sh apply` (backups under `C/lease-fair-tools/backup/`; `apply.sh revert` restores them while files still equal the staged content). It touches none of the walkperf targets (no overlap with `C/walkperf-tools`).
3. `bun repo run @semio-tech/framework-process:test-resource-lease-fairness` and `…:test-resource-lease-waits`, `…:test-process-progress`, then restart the sync loop.
Who blocks the image (read-only): `ls .cache/b/l/m/data/aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b.queue` (oldest name = head; `cat` shows pid/mode), `lsof …/aa5bbea….sqlite` (the holder is the only opener), then `ps -o pid,etime,command -p <pid>`.

## 7. Open items
- Structural: ordinary `bun repo run` must not run the dependency producer/ownership walk under the image's exclusive custody (or must wait for the sync instead of racing it); needs a deps4/build-system owner. Interim: walkperf + start the sync only when the head ticket is not a peer run.
- The generic `remains active on` text hides that the store-ownership counters are work, not waits (by design of the neutral formatter); the new `lease-queue:` line is the explicit wait signal.
- Windows: `renameSync` over an existing ticket in the upgrade path relies on Node's replace semantics; untested on win32.
- Unqueued shared requesters on a queued resource remain a starvation hazard by design; new callers must use `acquireQueuedResourceLease` with `sharedCohort`.
