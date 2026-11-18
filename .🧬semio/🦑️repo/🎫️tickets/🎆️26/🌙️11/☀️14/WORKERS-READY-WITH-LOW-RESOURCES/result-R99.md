# R99: stage `tools` never ends on g1-m04 (Linux) and g1-m05 (Windows)

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, P = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/`. Scripts: `T/🧰️scripts/R99/` (`edit_platform.py`, `edit_funding.py`, `mkstage.py`, `land.py`, `run_tests.sh`, `make_probe.py`, `probe/run.ts`, `note.txt`). All times UTC, read 21:43 to 22:07 on 2026-11-16.

## 1. Linux: who sends the SIGTERM

Our own ready sidecar. `LIB/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` (run loop, `kill(reason)` -> `stopTree("SIGTERM")`, `SIGKILL` after `graceMilliseconds`) is the only sender. Two reasons produce exactly the three closing lines of both retained logs (`SIGTERM received ...`, `Cancelled (SIGTERM); leases released.`, `Internal error: Original Script process terminated`, which is the stage script's own SIGTERM handler text, not a crash):
- `kill("stall")`: no new line for `stallStopMilliseconds` (600 s). Try 38 18:28 to 18:54 (`stalled`, 1576 s): the stage sat in a counterless standing line (`native-artifact-input-stage`; counters for it landed in R85 p at 18:31, after the running stage code was loaded).
- `kill("timeout")`: `readyDeadlineV1` verdict `timeout`/`stalled` after the soft cap of `tools` (`timeoutMilliseconds` 3600000). Try 38 18:54 to 19:54 (`stage-timeout`, 3601 s) inside `dependencies:workspace-edges:58687`, counter growing by 64: the cap was not yet extended by the stall tracker (R85 q, landed 20:3x).
So the stage is simply too slow for its cap and the cap killed real progress. Since R85 q the adopted try (started 20:34:17) is past the soft cap (5671 s at 21:46, 33943 steps at 22:06) because its counter keeps growing; the remaining bound is `hardCapMultiplier` 4 x 3600 s = 00:34 on 11-17. Rate: 28070 (21:44:46) -> 29195 (21:46:57) -> 33943 (22:06) steps: 4 to 9 steps per second.

## 2. Why 30000+ `native-artifact-input-read:<n>` steps

Not 30000 inputs. `<n>` is one process-wide counter (`nativeArtifactInputStepV1` in `P🟦️.ts`) that counts every 64 KiB chunk of every file of every pass. The group is at most 64 roots / 32768 files (`ProducerStageProtocol`). The "exclusive resource" is only a progress line (`control.publish`), not a lock. The cost is quadratic:
- `runRepositoryDependencyProducerProbeV1` (`LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts` lines 346 to 349) per root runs `readNativePlatformProtectedFreshArtifactInputGroupRootV1` (named "one root", but `verifyProtectedArtifactInputFilesV1` hashed every file of every root: the loop over `batch.files` had no root filter) and, after the installer, `readNativePlatformProtectedArtifactInputGroupV1` (hashes all again); the production path (lines 397 to 404) does the same plus `measureNativePlatformProtectedArtifactInputGroupFundingV1`, which reads the whole group once more per measurement. About `4 + 2 x roots + 2` full passes.
- Each file was reached by its own walk from the root (a chain of `withDirectoryPath` opens: O(depth squared) per file).
- The bytes had been proven at recording time and a change is already caught by the stamp (device, inode, mode, links, size, mtime, ctime) that every read compared first: the probe shows a same-size rewrite with restored mtime throws at the stamp compare in the OLD code too, so the repeated hashing proved nothing the stamp did not.
- Nothing is skipped across tries: a killed try leaves the roots; the next try's `observeNativePlatformProtectedArtifactInputGroupV1` hashes every file twice (two identical `verify` calls).

## 3. Windows: PID 15372

Evidence: `ready-log-tools-containment.txt` (21:45:49) lists 15372 only among rows `table.some(pid)`, i.e. the pid exists in the process table; registry start 20:31:56.644 (before the stage child 76840 at 20:35:17.885, so not its descendant), hold of `dependencies store` since 20:48:00.173, verdict `not-ours` (no owner token of a stopped try; the verdict is decided before liveness and identity, so it proves neither). `pool.json` of g1-m05 is 6 h old (final rename refused, 21 `.tmp` leftovers), not part of this fix.
- The lock cannot outlive a dead owner: the lease is a SQLite database under a kernel lock (`LockFileEx`), released when the process ends. A wait that persists means a process holds it. The registry names the holder (pid, start, `updatedAtMs` heartbeat every 60 s, stale after 300 s).
- What was wrong: the waiter (`observeDependencyStoreHoldersV1`, funding `🟦️.ts`) read raw registry entries, ignored the heartbeat, and judged a reused pid only when the OS start time lookup answered; the lookup is a `powershell Get-Process` with a 15 s timeout that fails on a loaded Windows host, and "an unknown start never counts as reused". A dead owner's entry whose pid was reused therefore blocked for the 21600 s bound (the earlier case pid 48352 was the same). Whether 15372 is a reused pid or a live `show projects` doing the same slow pass (section 2; it would then be a healthy holder that is merely slow) cannot be told from the store: both fit the log, the process is not reachable. Both are handled: the reused/dead case by the heartbeat rule below, the live slow case by section 2.
- The stage timeouts there (4555 s, 4567 s) are the waiter being cut at the cap while the holder runs on; the earlier try 8 timeout waited in `native-artifact-input-stage`, the same pass.

## 4. What is changed (landed 2026-11-16 23:00 UTC, lock held 46 s; fixture follow-up 23:30 UTC, 39 s; entries in `T/landings.md`)

1. New `P🧮️read-plan/` (schema-first: `🧬️schema/🔣️.json`, `🟦️.ts`, `🧪️tests/` with fixture `🔣️.json`, Python oracle `oracle.py`).
   - `protectedArtifactInputHashedV1(mode, file, rootIndex, times, recordedAtNs)`: mode `all` hashes everything; `root` hashes the one issued root; `changed` hashes nothing but racy files (newest of mtime/ctime within 5 s before the start of the recording pass, or no recording time): the rest is vouched for by the full stamp, as git does for its index (https://git-scm.com/docs/racy-git).
   - `visitProtectedArtifactInputDirectoriesV1(root, paths, visit)`: each requested directory once, descending from its parent; every step re-reads the entry and refuses a non-directory, a symlink or another device.
2. `P🟦️.ts`: `verifyProtectedArtifactInputFilesV1` uses both (mode `all` for the recording pass and the complete fresh reads, `root` for the root read, `changed` for every other read and every funding measurement); a stat-only pass publishes a progress step every 256 files so the stall rule still sees growth; the observed-group path (resume after a killed try) walks the same way and hashes once, not twice. Unhashed files are not opened at all.
3. Funding wait (`LIB/⚡️caching/.../⚙️setup/💰️funding/🟦️.ts`): `dependencyStoreHolderPresentV1` (alive, not a later process under the pid, and, when the OS start is unknown, heartbeat within `heartbeatStaleMilliseconds` 300000); holders carry `silentMs`; an ancestor of the waiting process is never judged by its heartbeat (it blocks in a synchronous spawn while its child waits, so its timer cannot renew the registration); a live holder silent past the bound is named with its own bilingual text `dependency-store.waiting-for-silent-holder` (`LIB/⚡️caching/📖️texts/🔣️.json`, EN and DE).

## 5. Proof

Red first:
- `P🧮️read-plan` tests did not exist (module missing); the source law test fails on the live `P🟦️.ts` before the edit (`toContain` red), green on the staged file.
- `💰️funding/🧪️tests/🫀️heartbeat`: `SyntaxError: Export named 'dependencyStoreHolderPresentV1' not found` on the live funding module (red), 3 pass on the staged module.
Green (staged, run against the real descriptor package): read-plan 5 pass (decision table of 12 cases equals the Python oracle; the probe-of-64-roots count test expects `4 x files` hashed against `files x (4 + 2 x 64 + 2)` before; directory visitor visits each directory once and refuses a symlink-replaced directory), heartbeat 4 pass (the ancestor law is red without its guard: 3 pass 1 fail), existing `🧬️reused-pid` 2 pass, `♻️retirement` busy-driver 12 pass and 5 pass on the new code. `tsc -p` over the staged files: no error in the platform file or the read-plan module (two errors in the funding file: the new text key is not yet in the old texts type, resolved by landing both together, and one `Type 'number' is not assignable to type '200'` that the unmodified funding file has too).
Behaviour of the real function (`probe/run.ts` runs the OLD and the NEW `verifyProtectedArtifactInputFilesV1` over a real tree; same machine, macOS, hashed files = published steps):

| pass | old publishes / ms (2016 files) | new publishes / ms |
|---|---|---|
| record | 2016 / 3296 | 2016 / 1006 |
| complete fresh read | 2016 / 3823 | 2016 / 1530 |
| any later group read or measurement | 2016 / 2759 | 7 / 413 |
| one-root read | 2016 / 2266 | 258 / 547 |

A same-size rewrite with restored mtime of a file of root 2 is caught by `changed`, `root 2`, `root 1` and `all` (stamp compare, change time) in both versions; a directory walk of 2000 directories: 932 ms per-file walk vs 204 ms tree walk. For 64 roots the number of hashed passes goes from about 134 to 4.

## 6. Not done / not verified

- Cross-try resume of finished work is only partly there: a killed try's roots are observed again and hashed once (was twice) and no longer once per root; a persisted receipt that skips even that hash after a restart (stamp plus content digest in the store) is NOT built. It needs a design in the funding journal.
- The running Linux try (adopted, started 20:34:17) keeps its old code in memory; it ends at the hard cap (00:34 on 11-17) or on completion; the new code is used from the next stage start. The sidecar restarts a stage on code change only while it waits for a holder (not changed).
- Windows: the directory visitor uses `withDirectoryPath` on nested directories (as production code at `📤️publication/🟦️.ts:313` and a descriptor test do); the Windows implementation of that call was read, not run.
- Real hardware: Linux (Steam Deck class) and Windows speed of the new reads is not measured (macOS numbers above); the per-file cost of 116 ms on g1-m04 is 100 times the macOS cost and is not explained by the code alone (load, disk). No change was made to `hardCapMultiplier` or the 3600 s cap.
- The identity of pid 15372 (reused or live and slow) and the reason `pool.json` of g1-m05 is not replaced (rename refused) are not established.
- Type check (`tsc`) of the edited files was not run; they were transpiled by bun and executed. `P📚️witness` tests, `📥️parent`, `⏳️busy-funding` (1 red) and the dependency storage reading tests are red the same way before the change (baseline kept in the scratchpad).

## 7. Landing log

- 22:30 window: the script aborted and restored every file (wrong path of the observation test in my test list, nothing applied); 23:00 window: landed, all listed member tests green plus both standing preflights (10 s, 25 s).
- After the landing `⏳️busy-funding` showed one new red (`the wait line names the holder ...`): its fixtures register holders with `updatedAtMs:1`, which the heartbeat rule rightly treats as a dead owner. Fixture renewed (`Date.now()`), landed 23:30; the one red that was there before (`the real process ancestry equals an independent walk with ps ...`) is unchanged. All other baseline reds (`📚️witness`, `📥️parent`, dependency storage tests) show the identical failure set before and after.
- Not yet observed: whether g1-m04 / g1-m05 finish `tools` with the new code (their current tries started before the landing).
