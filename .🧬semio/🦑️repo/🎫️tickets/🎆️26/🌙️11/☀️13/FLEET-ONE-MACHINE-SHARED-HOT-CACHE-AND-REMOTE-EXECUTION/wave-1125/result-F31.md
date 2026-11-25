# F31: ownerless native crates and Nx exit 130 (STAGED, not landed)

Written 18:20. Quiet window 18:15-19:00: nothing landed. A foreground wait to 19:00 is not possible (a foreground sleep is moved to the background), so the landing is one command for the coordinator or a re-launched lane.

## 1. Native dependency without build project owner

Evidence
- `S/origins/g1-m01/pool.json*.tmp` lastError: `Native dependency has no build project owner: <native root> (required by <owner>:build; candidates=N)` (seen for `@semio-tech/norm-din4108-rs` and `semio-s-artifact-energy-model-mut-year-schedule-holidays`). The throw site is `🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs:308` (`BUILD_SOURCE_AUTHORITY`).
- `semio-s-artifact-norm-en1993-scene-schema-grips` is in 4 of m01's `pool-published-*.txt` unit lists but has no row in `📋️build-owners.json`. Its crate (`✏️s/.../🔩️en1993/.../🎬️scene/🧪️tests/🕹️schema-grips/📦️packages/🦀️rust/Cargo.toml`) is committed (11-24 07:39), is a member of the en1993 workspace `Cargo.toml` and is not excluded by the owner-tree policy.

Cause: the registry (`📋️build-owners.json`, source of truth) is only repaired scoped (graph-read follow of a named root, scaffold/mutation-tree tools); nobody ran a full `verify build-owners sync`. Peers add hand-written crates, so 50 package manifests have no row or native root (the read-only `repo ./📜️script.ts verify build-owners` at 18:16 reports exactly `cargo-without-row` x50): norm en1991/1992/1993/1994/1995/1998/vdi3805 editor, mutations, model-rows, diff-rows and scene crates, process3d harness, os-store-original-batch-abort-tests, 14 hub apps/shards. List: `python3 T/🧰️scripts/F31/scan-ownerless.py` (the one extra `project-without-row` line is the root `📋️project.json`, a false positive of the script).

Fix, class level (both staged in `T/🧰️scripts/F31/`)
1. Registry: `repo ./📜️script.ts verify build-owners sync` adds the 50 rows (the canonical writer, atomic, re-reads the file).
2. Pool skip-and-report: `ownerPoolFailureClassV1` (owner-pool `🟦️.ts`) files a "no build project owner" failure under code `ownerless`, `deterministic:false` (no fleet failure note that other machines would honour after the row exists), `skipped:true`; child `🟨️.mjs` uses it; runner prefixes lastError with "skipped, no build owner row / übersprungen, keine Build-Owner-Zeile". `failures.ownerless` counts them in `pool.json`. A unit never retries forever and never blocks the loop (it is `finished` in the wave).
3. Test: appended to `owner-pool/🧪️tests/🟦️.ts` (4 expectations).

## 2. Nx run-many exit 130

Evidence (only surviving log: `S/origins/g1-m01/ready-log-workspace-failed.txt`, 16:58:07; the other 8 workers rotated their logs)
- Nx itself ended with failure on its own: task `@semio-tech/framework-rs:generate:prototype` failed (typegen: cargo `failed to write .../b/r/m/b/p/prototype/build/semio-framework-pack-http/.../invoked.timestamp: No such file or directory`, i.e. a build directory removed under cargo; F33 owns that), `repo-mcp:build:prototype` not run, then `Nx run failed: 1 task(s) did not succeed after 0 rerun(s)` (that line is only printed when `!cancelled`) and the `[units]` summary. Only after that the process exit status became 130 and `Owned Nx invocation failed (130)` was thrown.
- Therefore 130 is a consequence, not the cause, and not our own cancel after the first failure: no code in the tree sends SIGINT (all terminations use SIGTERM/SIGKILL: `terminateOwnedProcessTree`, `readyKillTreeV1`, warden `signal` ports SIGSTOP/SIGCONT/SIGKILL/SIGTERM). The warden is excluded too: `memory.json` of m01 shows `cancelled:0`; only m11 has one cancel (a local-unit), m02/m08/m14 only suspend/resume.
- The SIGINT reached the bootstrap process in the window between the end of Nx and the end of the script (`nxFleet.finish`, spool drain); its sender is not recorded anywhere (supervisor 16217 had handed the stage to supervisor 51145 at 16:53, a plausible but unproven source). The old code then rewrote the real failure status to 130 (`process.exitCode=cancelled?130:code`) and the log read as if a cancel was the cause.

Fix (staged): `nxFinalStatusV1` (failure-tail `🟦️.ts`, texts in its schema, en+de): a signal that arrives at or after the Nx close of a failed run keeps the failure status and prints `[nx] SIGINT reached this run after Nx had already ended with exit status 1 (failed: <tasks>); ... the sender is unknown (this run sends only SIGTERM and SIGKILL)`; a signal during the run keeps 130/143 and says its sender is unknown. `script.ts` records `cancelledAtMs`/`closedAtMs` and uses it. Test appended to `failure-tail/🧪️tests/🧾️summary/🟦️.ts` (5 rows against an independent reference, plus the en/de text checks).

## Landing (after 19:00)
Locks (alphabetical): `T/landing.lock.bootstrap`, `T/landing.lock.graph`, `T/landing.lock.tools` (each with `owner`). Then from the checkout: `T/🧰️scripts/F31/land.sh` (apply.py anchored edits, `verify build-owners sync`, rescan). Tests via `repo test-file`: owner-pool `🧪️tests/🟦️.ts`, failure-tail `🧪️tests/🧾️summary/🟦️.ts`, the bootstrap tests that import `📜️script.ts`. Then `python3 T/🧰️scripts/coordinator/dangling-imports.py` on the touched folders, and the `landings.md` entry `## Wave 1125 F31: ...`. `apply.py --dry` passed at 18:16 (all 14 anchors unique); source sha256 prefixes before: owner-pool ts 95f4ced95012, child a1ae6c5ee072, runner 5aa636608611, pool tests 5d0adf8ac1d2, failure-tail ts baea7e08eacb, its schema 7b3bb4f5f086, summary test 92e4b46250fc, script.ts f882633740fc.

## Not verified
Nothing was run against the live tree (no test of the new code yet, no landing). Whether `verify build-owners sync` adds all 50 rows without leftover findings (some crates may need native-root placement) is proven only by the read-only report. The SIGINT sender stays unknown; the patch makes it visible and stops it from masking the failure. The cargo ENOENT root cause is outside this lane (F33).
