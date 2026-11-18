# R162: dependency verification before a task runs (living report)

Status: LANDED 2026-11-18 00:03:16 to 00:03:55 UTC (lock `landing.lock` owner claude-R162, hold 39 s; bun workspace root, churn, refresh, frontier and the new receipt test files: 111 pass, 0 fail, baseline before 107/0; the call-count law was red before). Services and reads run the old code until restarted. Scripts: `T/🧰️scripts/R162/` (`measure.cjs` counting wrapper, `lstat.cjs`, `land.py`, `window.sh`). Law: `LIB/🗂️workspaces/🟦️bun/🧪️tests/🧾️receipt/🟦️.ts` (4 tests).

## 1. What runs before a task executes, and where the 1100 s go

Two different verifications are meant by "dependency verification". Only the first one is the wall of R159/R157 wall 68:

1. **Source ownership walk** (`dependencies:store-ownership:<operation>:<counter>`, `dependencies:lock-ownership:...`, `dependencies:preservation-presence:ownership:...`, all exclusive/shared leases of the `dependencies` family). Code: `LIB/🗂️workspaces/🟦️bun/🟨️.cjs` (`withCurrentBunWorkspaceOwnershipV1`, `withPublishedBunWorkspaceOwnershipV1`), called from `LIB/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (lines 753/866/902) and `…/📤️publication/🟦️.ts`. It derives which directories of the WHOLE repository tree own a `package.json` (every non-hidden, non-output directory below the member prefixes: one `lstat` + one listing per directory) and records every path state (`observations.paths`, bound 524,288 paths / 1,048,576 facts, schema `PhysicalLimits`). The counter in the log is the cumulative operation counter (derivation: 2 steps per directory; every replay adds one per observed path). Observed in the real logs: 2,080,320 (fixture line of a real g1 read, 3461 s), 1.4M in the `tools` tails of g1-m05.
2. **Generation store verification** (`dependencies:sync-gate`, `store-stage`, `store-input-replay`, `workspace-edges`): `verifyJavascriptDependencySyncCurrentV1` reads the sync receipt, per scope the generation receipt and the installed-paths ledger (one `lstat` per installed directory against a sealed ledger; `🧾️incremental/💾️storage/🟦️.ts`). One generation on this Mac: 7,364 directories, 58,295 files, 1.5 GB; 12 generations. `sync-gate` shows 17 to 41 s per attempt in the logs of g1-m01, g1-m03, g1-m08 (58 lines each; counters in `ready-log-dependencies.txt`). Existing memo layers: the sync receipt (`current` gate skips the whole sync), installed-ledger receipts, the persisted ownership index (listings under their stamps).

How often: the ownership custody (`withCurrent…`/`withPublished…`) is opened once per stage command and then fenced by `custody.recheck()`: 6 call sites in the store-stage code (`fence`), one more at the end of the callback, and the publication path validates twice per scope. A `recheck` is `validate()`.

## 2. Measured (this Mac, 10 cores, load average 31 to 42, counting wrapper at `node:fs`, synthetic trees in scratch, nothing read from the live workspace)

Fixture 8,420 directories, 164 owner manifests, all settled. `measure.cjs`:

| phase | calls (lstat) | ms |
|---|---|---|
| derivation (first walk, listings cached) | 37,678 | 252 |
| one `recheck()` BEFORE | 38,020 | 205 (hot) |
| one `recheck()` AFTER (this report's change) | 20,830 | 118 |

Fixture 64,000 directories (load average 48): derivation 278,318 `lstat` in 4.8 s; one recheck BEFORE 279,620 `lstat` in 4.5 s (70 µs per directory), AFTER 147,030 `lstat` in 0.86 s (the time ratio 5.3 is better than the call ratio 1.9; the two runs were not isolated from the load of the machine, so only the call ratio is a claim).

* `lstat` bigint costs 5.6 µs hot (`lstat.cjs`; 15 µs with `throwIfNoEntry:false`); the JSON.stringify and stamp string work is 1.2 µs per path. The replay is therefore syscall bound: 4.5 `lstat` per directory per recheck, CPU is not the cost.
* Why 4.5: a recheck replays the whole frontier TWICE: `validate()` called `requireCurrentMembershipV1` itself and then `currentFence` called it again (`owner.report` is set), and each replay is two passes (one before the continuation, one after) over every observed path. The first replay was redundant: the second one runs after the last await of `validate()` and carries the same proof (same function, same refusal/rederivation).
* Extrapolation (arithmetic, not a run): at the bound of 524,288 observed paths one recheck is 4 passes x 524,288 = 2.1M `lstat`: 11.7 s hot at 5.6 µs; at 50 to 100 µs per call (cold cache, load 38, nice class 10 on the worker) 105 to 210 s per recheck. Eight rechecks per command (6 `fence` sites + end + publication) give the 1100 s of R159 and the 115 to 280 s "stalls inside a single `readText` step".

**Why a `readText` step stalls for minutes** (the question of the task): it is real, synchronous I/O, not a lock wait and not a queue. The second pass of `replayCurrentMembershipV1`, `authenticateCurrentMembershipV1` and `readManifestText` loops are fully synchronous by design (the last fence must have no await after it), so one pass over 500k paths blocks the event loop for N x lstat with no progress line and no `yieldContinuation`; the counter stays on its last value (the label stays `readText`/`frontier`) until the pass ends. `publish` itself is a non-awaited `process.stderr.write` through a pacing writer (`createScriptProcessProgressWriterV1`, `yieldContinuation` is one `setImmediate`): they cost microseconds and are not the wait. A lock wait would show `remains active on ... (waiting)`; the logs show `exclusive` with the counter frozen, i.e. the holder itself is busy. NOT proven on a worker: that needs a CPU profile of a real worker (`SEMIO_REPO_CPU_PROFILE`), not available without a worker run.

## 3. What changed (landed)

`🗂️workspaces/🟦️bun/🟨️.cjs`, `withCurrentBunWorkspaceOwnershipV1.validate`: the explicit `await requireCurrentMembershipV1(owner, control, current)` before the manifest loop is removed; `currentFence` at the end of `validate` does the same replay after the last await. Effect: every recheck of a custody costs 2 passes instead of 4 (-45 percent `lstat`, -43 percent ms measured). Same refusals (the tests of the churn, refresh, frontier and root files are the guard), same fence point.

## 4. Laws (new `🧾️receipt/🟦️.ts`, red first)

* call-count law: one recheck of an unchanged custody performs at most `2 x records + 50 x manifests` `lstat` (red before: 1725 against 1186 for 281 records and 13 manifests).
* growth law: the second and third recheck cost exactly as much as the first (no memo that could go stale; green before and after).
* tamper law: a new owner directory, a changed owner manifest and a changed directory mode are each refused by the next recheck (green before and after: the exactness is kept).
* throughput law: under 120 µs per observed path and recheck times the R135 speed factor (`localMachineSpeedFactorV1`).

## 5. NOT built (the asked design) and why

* **Sealed receipt with an O(1) second verification: not built.** The ownership frontier has no cheap exact witness: a directory stamp (mtime, ctime) covers its own entries but a change of the mode of a child directory (part of the recorded state) or of a manifest's content moves no parent stamp, and no directory stamp covers a new directory deep below. The only exact O(1) witnesses are an fs-events stream with a barrier (FSEvents / ReadDirectoryChangesW, started before the derivation, flushed with a sentinel write at each fence; any event, overflow or error voids the receipt) or a time-bounded trust, which weakens the fence. Neither fits a fleet-wide landing in one window; the cost left is 2 passes x N `lstat` per fence (see the extrapolation above).
* What the cheap witness can and cannot detect (design for the next step): the receipt keyed by (generation digest = sha256 of the owner manifests and policy, workspace root identity dev:ino, event cursor) detects any create/delete/rename below the root and any content write (events), cannot detect changes made while the watcher was not running (it then does not exist: the full walk runs), and cannot be trusted across machines or after a clone (inodes and absolute paths in the stored records).
* Single-flight over concurrent commands (concurrency law), streamed parallel verification with a derived width, and verification outside the exclusive lease: not built. The walk and the fence stay under the lease of the stage.
* Loop-1 skip (do not re-lstat settled records before the continuation) would halve the passes again but changes the progress counter that the frontier law (`⏩️frontier`, 17+paths callbacks) pins: needs a rewritten law.
* The generation store part (`sync-gate`, `store-input-replay`, `workspace-edges`): measured sizes only, nothing changed.

## 6. For R161 (seed by clone) and the workers

A seed must NOT carry the ownership index, the sync receipt or the installed ledgers of another checkout: they hold absolute workspace paths and `dev:ino` stamps (`installedStampV1`, `decodeOwnershipIndexV1(workspace, ...)`), a clone changes both, so they are rejected and the full walk (derivation + every recheck) runs once anyway. What a clone may carry: the immutable generation directories (`.cache/b/d/m/j/<generation>` content); then `sync-gate` finds the receipts missing, scans, seals new ones. A `repo dependencies` stage that already ran in the lab leaves valid receipts for that exact checkout path.

## 7. Logs (evidence of the workers)

The `store-ownership:frontier` lines of g1-m08 at 23:26 UTC are no longer in `/Users/Shared/semio-store/v1/origins/g1-m08/ready-log-workspace.txt`: the log is a 16 KB tail that was overwritten (now only `dependencies:store-stage` lines and "still running after 365/375 s"). Only g1-m05 `ready-log-tools.txt.*.tmp` of 2026-11-16 keep `…-ownership:frontier:1400439` lines (`funded-ownership`, 51 to 85 lines per file). `remains active on` lines in the current logs are `dependencies:sync-gate` only (g1-m01, g1-m03, g1-m08: 58 lines each, 16 to 41 s per line). Before/after seconds from the workers: see the end of this file after the landing.

## 8. Before/after seconds on the workers

NOT measurable yet: at 00:05 UTC no worker log holds a `store-ownership:frontier` line (the tails were overwritten) and no worker has restarted on the new code. Prediction from the arithmetic of section 2, to be checked on the next `workspace` cycle of g1-m08: the ownership recheck part of the `dependencies:store-ownership` time falls by 45 percent of the calls (1.9x fewer `lstat`); the derivation (first walk, 278k `lstat` per 64k directories) is unchanged, so the first-command wall of R159 shrinks by the recheck share only (about 8 of 9 passes-equivalents in the arithmetic above, i.e. not to milliseconds). Whoever reads the logs next: search `remains active on dependencies:store-ownership:frontier:` and compare the highest counter reached per 100 s with 23:26 (counter 444,864 at 79 s, 475,200 at 86 s: about 360 operations per second).

## 9. Window rule of 00:06 UTC (hh:00-05 and hh:30-35, new tests first run in scratch against staged files)

My landing (00:03:16 to 00:03:55) came before the rule and does not meet it fully: the staged change was measured in scratch (`measure.cjs` against `staged.cjs`), and the new law ran red on the live old code, but the first run of the battery and the law against the CHANGED code was under the lock (111 pass, 0 fail, no restore needed). No further landing from R162 is planned; any follow-up (loop-1 skip, fs-events receipt) must stage a scratch copy of the whole `🗂️workspaces` member and run `🧾️receipt` and the frontier laws there before a window.
