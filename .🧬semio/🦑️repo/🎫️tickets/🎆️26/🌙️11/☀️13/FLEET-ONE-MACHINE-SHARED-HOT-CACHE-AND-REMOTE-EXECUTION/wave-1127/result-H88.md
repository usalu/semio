# H88: finishing H76 (shared children, Windows job objects, style test, one kill record)

Lane H88, wave 1127. Landing through `repo land` in three changesets (see "Landings"). `P` = `🧰️framework/🔨️modules/🏃️process`.

## 1. The `📤️style` timeout was a real defect, not load
`owned-execution/🧪️tests/📤️style` hung for 120 s even with the timeout raised (locally and on three workers), so it was never "load".
- Cause (found by a spawnSync-stack probe, `probe2.ts`): `runOwnedCommand`'s `terminate()` started a new graceful termination (one `ps -axo ... lstart` table read, synchronous, up to four per wave) on EVERY call. A sink that fails on every write (the test's `sinkFailure` cases 6 and 7) calls it once per write error. H76 made a termination cost a table read; before it was one synchronous kill. Thousands of table reads starved the event loop, so Bun never reaped the already killed child (it stayed `<defunct>`) and the command never settled.
- Fix: `terminate()` starts the tree termination once (`terminating` flag); the first reason stays the recorded one. Test `🎛️owned-execution/🧪️tests/🔂️terminate-once`: a sink failing on every write, command ends < 20 s, the ledger holds one SIGTERM wave (1..3 lines), EN/DE line.
- Result: `📤️style` 1 pass in 2.5 s (was a 20 s timeout). The temporary 120 s timeout I put into the test while diagnosing is back at 20 s in the changeset.

## 2. Shared children wired into the real shared cargo build
There is no resident cargo owned by the broker (the TS broker `startAdmissionBrokerV1` only serves the wrappers over a socket and signals no process; `forget(entry)` on a closed socket is bookkeeping). The one cargo that several requesters ask for in one process is the scheduler crate build `buildSchedulerV1` (every driver/pass/test of a process calls it with the same target and build directory; two cargos would only queue on the build-directory lock).
- New member `📦️artifacts/🏗️native-build/🎟️admission/🤝️shared-build`: `createSharedBuildsV1` over H76's `createSharedChildRegistryV1`: first requester of a key starts the child (own group, reaper-registered), later ones join; a requester that leaves (signal/deadline) is refused with its own reason and releases only itself; the last release stops the whole tree with the graceful identity-checked termination (SIGTERM, 5 s, SIGKILL by pid+creation time) and a recorded EN/DE line; the build outcome (status, executables, diagnostics) is read once and delivered to everyone who still waits.
- `buildSchedulerV1` runs through it (key = hash of cwd, target dir, build dir, arguments, environment); its old private spawn + `terminateOwnedChildTree` is gone. A cancelled requester still gets the lease-control refusal first (`checkResourceLeaseControl`).
- Tests `🤝️shared-build/🧪️tests` (4 pass): two requesters, first leaves -> only it is refused, cargo and its rustc survive, second leaves -> both gone, record line carries `Victim created at .. sender created at ..`; outcome reaches both with one start, later requester starts afresh; already aborted requester starts nothing; **broker integration**: a real `startAdmissionBrokerV1` admits two requesters (`stats` running+waiting = 2), one leaves (its broker entry goes: 1; build alive), the other leaves (0 and cargo+rustc gone).
- Not done: the admission broker is a server, not a child; the resident slice pool (`🏡️resident/.../👷️slice-pool`) already counts clients per load and aborts the load when the last client leaves with its own reaper (`🧹️reap`); I did not replace it (its workers are not cargo, it has its own tests); `component-pass` cargo is per pass, one requester.

## 3. Windows job objects
- `🪓️termination/🧹️reaper/🪟️job`: `createJobTableV1(kernel)` (add: create + arm kill-on-job-close + assign by pid; stop: terminate + close; release: disarm + close for a child that ended, kills nothing; stopAll). Real kernel `windowsJobKernelV1()` over `bun:ffi` + kernel32 (CreateJobObjectW, SetInformationJobObject class 9 / 144 bytes / flags at 16 / 0x2000, OpenProcess 0x101, AssignProcessToJobObject, TerminateJobObject, CloseHandle): system library, no external dependency.
- The job handle lives in the reaper sentinel (outside the child's tree). Parent death = pipe end = `stopAll` then taskkill fallback; a new reaper line `stop <pid>` (sent by `terminateOwnedProcessTree` on Windows after taskkill, through `stopRegisteredV1`, which starts no sentinel) ends one registered tree now; POSIX sentinel handles `stop` as a graceful tree termination of that registration (test with a real tree). No job available -> EN/DE log line and taskkill only.
- Tests: table against a recorder (6 pass), line grammar vs Python regex extended with `stop` (5 pass incl. stop of one registered child while the other lives), plus a Windows-only real-kernel test (child + grandchild whose parent link is gone). The ready runner is unchanged: its schema still says no job object (a job that kills on close would kill the resident service a successful stage leaves running); owned tasks only.

## 4. One kill record
- Finding: H70's `readyKillRowsV1` matched only the stage runner's sentence ("from the ready stage runner of this worker (runner process ..., stage ...,"), H76's rows ("from owned execution <label> (killing process ..):") were NOT read. Now both are parsed (`stage` = the label), so `readyStopSenderV1` names `runner` for them.
- Rows only reach `kills.log` if someone writes them there: `ownedKillRecorderV1`'s default writer (`defaultKillWriterV1`) now also appends to the file named by `SEMIO_KILL_LEDGER`, and the ready runner puts its `kills.log` into the stage environment, so every owned execution inside a stage lands in the one file the reader tails. The reaper keeps its own `<tmp>/semio-reaper/<parent>.log` (same layout).
- Test `🚦️ready/🧪️tests/🔗️one-record` (2 pass): rows written by the real recorder + ledger writer are read back (pid, signal, label as stage, killer), garbage ignored, `sinceMs` honoured, `readyStopSenderV1` -> `runner`; default writer appends only when the environment names a ledger.
- Also fixed two stale tests of the ready folder that blocked its gates: scaled-policy test lacked `staleWaitingMilliseconds` (pre-existing failure), `capacity-wait` TS2769 (`row.exit ?? null`).

## Landings
1. stage1 (termination, reaper, job, owned-execution, shared-build module and tests, style): see `T/landings.md` `## Wave 1127 H88: ...`.
2. stage2 (admission wiring + `🦀️cargo` test helper): needs 1.
3. stage3 (ready reader, ledger variable, test fixes): needs 1.
Status of each is appended at the end of this file.

## Gate finding (not mine, worth a ticket)
`repo land` overlays are no managed workspace: `native-build/🧪️tests/🦀️cargo` `schedulerBinaries()` builds the scheduler with `repo cargo` in the overlay and fails with "exact managed workspace Source binding required" (2 failures `real Cargo through the wrapper ...` in `🎟️admission/🧪️tests`), though the same test passes 18/0 on the live tree; any landing touching `admission/🟦️.ts` is refused by this. stage2 makes the helper use the live checkout (found through the overlay's `.cache` link) when it runs in an overlay.

## Unverified
- Windows: job object path never executed (no Windows host); type-checked only, real-kernel test is skipped off Windows.
- Shared build is not yet run by a real `repo cargo`/fleet build; the only caller is `buildSchedulerV1`, proven with fake cargo processes and a real broker.
- `📤️style` was run on the local machine (g1-m01); not re-run on a loaded worker after the fix.

## Status at the end of the lane
- Landed 16:03 (stage1, rollback `stage1-20261127T150314-2ea29d`): termination once-per-command fix, reaper `stop` line, Windows job module, ledger default writer, shared-build module (+4 tests incl. broker), style test back to 20 s. Gate tests 84.6 s passed.
- Landed 16:13 (stage3, rollback `stage3-20261127T151325-ea4f6f`): stop-sender reader reads owned-execution rows, runner puts `SEMIO_KILL_LEDGER` into the stage, one-record test, two stale ready tests fixed. Gate tests 216.8 s passed.
- NOT landed: `buildSchedulerV1` wiring (`admission/🟦️.ts`). Refused by the declared-test gate: `admission/🧪️tests` "real Cargo through the wrapper" fails inside the overlay with "exact managed workspace Source binding required" (the gate runs the test CLI with the overlay as the workspace and the test calls `repo cargo`), passes 18/0 on the live tree. My attempt to build from the live checkout inside the overlay did not help (the Source binding comes from the outer environment), so I dropped that test-helper change. The staged patch is `T/🧰️scripts/H88/admission-shared-scheduler-build.patch` (diff against the live file at 15:00; apply with `repo land checkout` + patch + `repo land`). It needs either a gate exemption for tests that need a managed workspace (such tests should run on the live tree after apply, with the rollback point) or the overlay test run to scrub the Source-binding environment. Until then `buildSchedulerV1` still spawns its own cargo; the shared-build module is landed and tested.
