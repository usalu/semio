# Vanishing Artifacts in Norm Test Runs

## Verdict

The artifacts did not vanish. The rustc stand-in (`rustc-wrapper`, which runs `🦀️cargo/🗃️unit-cache/📜️script.ts` from the live worktree) ended with exit status 0 before it had compiled or restored anything. Cargo took the unit as built and started its dependants, which then found no `.rmeta` or build script (`extern location for <crate> does not exist`, `could not execute process .../build_script_build (never executed)`). Bun ends a process with status 0 when an awaited promise has nothing left to keep the event loop alive; in the incident windows that was a failing request of the fleet cache client that was being rewritten in the same worktree at the time. Gc, eviction, two writers on one target dir and artifact promotion are ruled out (below).

## Evidence

Incidents (all `repo cargo --store=shared ...`, one core admitted, lanes `p4fdb48` and `p5e7217`):

| Run | Time | Symptom |
| --- | --- | --- |
| normtest-c-forms1 | 05:43 | `serde_core` `out/build_script_build` "never executed" |
| d4108-4 (`check --tests --workspace`) | 05:48 | 24 `extern location ... does not exist`, 52 errors incl. cascades |
| normtest-c-forms2 | 05:50 | `native-number` `.rmeta` missing |

1. The failing units have no event at all in `.cache/b/r/m/u/x/<lane>.jsonl` (no `remote-miss`, `miss`, `hit`, `bypass`). Every successful unit has one, and so do the same units in the later healthy runs (forms3, d4108-5). The first line the wrapper writes for a unit comes after the fleet lookup, so the process ended during or before it.
2. The out directory of the missing unit was never written: `out/` mtime equals the plan time (05:44), while cargo's fingerprint `.json` for the unit is 05:48. A removal would have bumped the directory mtime. Nothing was removed; nothing was produced. The d4108-4 log has no warnings and no compile line for the failed unit, so rustc never ran.
3. Cargo only skipped the unit because the wrapper exited 0. The code already documents the consequence (`✅️complete`: "A unit reported done without them lets Cargo start the dependants, whose compilers then fail with `extern location for <crate> does not exist`").
4. Bun reproduces it: `bun -e 'new Promise(()=>{}).then(()=>{process.exitCode=3})'` exits 0; with `process.exitCode=70` set first it exits 70 (verified with the repo's bun 1.4.2).
5. The wrapper runs from source that was being rewritten during the incident: `📡️client/🟦️.ts` (new file, 05:51:19), `🛰️fleet/🟦️.ts` (05:47:34), `📜️script.ts` and `🏁️entry.ts` (uncommitted, +138 lines). The failing windows end where the guard (`UNFINISHED_EXIT_STATUS_V1`, the fleet ride-through, test at 05:56:03) landed; runs started afterwards (forms3 from 05:51:17, d4108-5 from 05:57) have no extern errors.
6. The same unsettled-then-exit-0 pattern was still unguarded in the build script stand-in (`🏗️buildscript/🟦️.ts`: `.then(status=>{process.exitCode=status})`), whose failure shows up as the `serde_core` "never executed" and a missing `OUT_DIR`.

I did not capture the exact promise that stalled in the 05:43 to 05:50 versions of the fleet client (the code changed since), so item 5 is inference from timing plus items 1 to 4.

## Ruled out

- Unit gc (`🧹️gc/🧱️units`, `collectUnitsV1`): removes whole `build/<crate>/<hash>` directories, only units idle for 30 min (3 h and not newest in a busy profile), never a unit whose `.lock` is flocked, and takes the exclusive build lease or queues compilers. Passes ran at 05:41:42 and 05:51:56 only (`fill.jsonl`), and removed units of the idle lanes `q0c9bae` and `d`. The failed unit directories exist and are fresh.
- Lane eviction (`removeLaneV1`, `trimPoolV1`): takes the lane claim first and refuses with a live marker. `p4fdb48` and `p5e7217` were never removed.
- Two cargo writers in one build dir: one process tree per (build dir, profile) through the lane claim (exclusive file) and the exclusive custody lease (`semio.cargo.custody-lease/v1`, `repo leases status` shows exactly one holder per lane). Both lanes had a single driver.
- Prune (`🧹️pruning`): needs the exclusive store lease, which every cargo holds shared for its whole run.
- Promotion or dedup: `publishUnitV1` and `restoreUnitV1` only copy (reflink) between lane and CAS; nothing moves or links a lane output out. `placeV1` renames the new file over, so a reader never sees an absent name.
- Fixed ctime fence: not on the path of any failing unit (no store fence error in the logs).

## Fix

- New leaf module `🦀️cargo/🗃️unit-cache/🏁️exit/🟦️.ts`: `UNFINISHED_EXIT_STATUS_V1` (70) and `exitWithStatusOfV1(work)`, which sets the failing status before the work starts, replaces it with the real status, and ends with 1 and the message on a rejection. Every process Cargo executes for a unit now ends through it:
  - `📜️script.ts` (compiler stand-in main; re-exports the constant the fleet test imports),
  - `🏁️entry.ts` (native stand-in, previously a bare top-level await),
  - `🏗️buildscript/🟦️.ts` (build script stand-in, previously unguarded).
- The sibling's already live guard in `📜️script.ts`/`🏁️entry.ts` was kept in behaviour and moved into the shared module so the three entries cannot diverge.

## Tests

`🦀️cargo/🗃️unit-cache/🏁️exit/🧪️tests/🟦️.ts` (added to suite `cargo-unit-cache-processes`), run with `repo tools exec bun -- test <absolute path>`: 5 pass.

1. Work that never settles ends the process with 70, not 0 (the reproduction of the incident).
2. The constant is 70 in the module and in the compiler stand-in.
3. Settled work ends with its own status (0, 1, 7).
4. A rejection ends with 1 and prints its message.
5. Conformance: no non-test source of the unit cache sets its status from a bare `.then(status=>...)` or `process.exitCode=await`; the buildscript line that had it matches the pattern at HEAD.

Not done: the full `cargo-unit-cache-processes` suite (it needs real cargo and ran against a machine with no spare memory); the fleet ride-through test of the sibling is theirs and untouched.

## Residual risk

- `collectUnitsV1` and `measureIncrementalV1` flock every unit and session `.lock` of live lanes for a moment to probe them; with `-Zfine-grain-locking` a probe that collides with Cargo's own lock attempt is a possible but unobserved source of a second kind of skip. Not changed; worth a separate ticket if extern errors reappear with events present.
- The wrapper runs from the shared worktree, so a sibling's half edit of `unit-cache` breaks or silently alters every concurrent cargo run. A digest-pinned, copied wrapper would remove this; not done here.
