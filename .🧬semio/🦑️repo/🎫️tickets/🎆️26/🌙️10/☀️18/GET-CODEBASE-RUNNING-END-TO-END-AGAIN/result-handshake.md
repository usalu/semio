# Lane handshake: the Bazel action birth handshake counts as work, and a settled birth is never cancelled

FOR coordinator:
- apply : `python3 -I /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/handshake-tools/staged/handshake/stage.py apply`   (check | revert likewise; drift exits 2; keeps file modes)
- rebase (live files changed since 12:30): `python3 -I /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/handshake-tools/build.py` (re-derives executor + scheduling policy from the CURRENT live files; the test and fixture are fixed inputs)
- verify : `sh /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/handshake-tools/staged/handshake/verify.sh <root>` (after) and `... <root> before` (RED on base, re-applies)  -> on mirror: "VERIFY OK handshake" both ways; laws 12/12 after, 0/12 before; failure sets of the scopes, framing, control, purpose, mode, scheduling and contention suites == base (6 pre-existing reds)
- proof run (coordinator, not run here): `bun repo run print-rs:test` one-line edit probe (`gate-g-tools/edit-probe.py`) at host load >= 15 on 10 cores: no `birth-handshake-deadline-exhausted`.
- executor key changes (new import in the executor closure + policy port), so the next product run rebuilds actions once.

## Cause (g48p-edit.log, lines 255-290)
`git grep birth-handshake` -> `🏗️bazel/🎬️executor/🟦️.ts`, armed by `launchBazelActionBirthV1` (the action child's birth, 10 000 ms from `protocol.maximumHandshakeMilliseconds` in `🏗️bazel/🧬️schema/🔣️.json`). The parent (`📜️script.ts` control server) already answers `waiting` as soon as the birth is admitted and then waits for the run's grant to grow (`upgradeBuildCapacityV1`) and for a job (`width.enter`). In the log the grant needed 15 s ("waiting 11s to grow the reservation ... grown after 15s") at load 18.5 / 10 cores.

Two clocks guarded the same phase and only one knew about `waiting`:
1. `bazelBirthHandshakeV1` timer: `waiting` clears it (correct).
2. `initial()`: a second wall clock `performance.now()-started >= 10000`, polled by `readBazelBirthFileV1`, `readBazelRuntimeV1` and by `remaining()` in the `connect` and `joined` handlers. `waiting` never touched it, so the `joined` that arrived after 15 s ran `remaining()` -> `initial()` -> `RepoError cancelled build.bazel.birth-handshake-deadline-exhausted` (stack: initial 560 <- remaining 569 <- data handler 630), the birth failed, `bazel:test failed (3)`.
Neither clock scaled with load (a fixed 10 s), unlike the test budgets (`🧪️testing/🎚️budget/⏲️contention`: work, not wall time, stretch 1..8 from loadavg per core). Note: in this log the child had not started yet (the birth failed before the test ran); the same unsettled clocks could also cancel after a long grant wait of a child about to run, and the fix makes it impossible after `waiting`/`started`/`settle`.

## Fix
`bazelBirthHandshakeV1(maximumMilliseconds, started, options)` is now ONE work clock (reuses `createBudgetClockV1`, `budgetStretchV1`, `sharedHostContentionV1` of the contention module; no new constants):
- phase `birth`: every wall ms is worth 1/stretch ms of the 10 s (load 18.5 on 10 cores -> expires at 18.5 s; hung parent on a saturated host -> at most 8x = 80 s; idle host -> 10 s, as before). Sampling is lazy: nothing is measured before the plain 10 s have passed (no Windows spin probe on healthy births); ticks of at most 1 s afterwards.
- `waiting` -> phase `waiting` for good: neither `check()` nor the timer can fail on the handshake; only the owner's deadline bounds the birth (`bazel.birth-deadline-exhausted`, unchanged).
- `settle` -> phase `settled`: timer cancelled, `fail` unreachable; a child that finished its work is never cancelled by the handshake.
- `check()` is the single poll of the pre-start steps: `initial=()=>handshake.check()`; the second wall clock is gone. `arm(deadline, fail)` starts the timer once the socket exists (as before).
- A probe that throws counts as an idle host.
- `🗓️scheduling/🔣️.json`: `process/📖️texts/🟦️.ts` becomes a port of `progress-reporting` (the executor closure now reaches the contention module, which refuses through the process texts; without the port the scheduling-split law gains a fourth violation).
Files: executor `🟦️.ts` (M), scheduling `🔣️.json` (M, one line), new `🏗️bazel/🧪️tests/📥️handshake/🟦️.ts` + `🔣️.json`. No build-guide edit (hot file); optional: one sentence next to "Test budgets" that the birth handshake uses the same clock.

## Laws (`🏗️bazel/🧪️tests/📥️handshake`, 12 tests)
- 9 table rows from the independent Python model `handshake-tools/oracle.py` (regenerates `🔣️.json`; deterministic simulated host: injected `now`, `epoch`, contention `source`, timers): idle host expires at 10 000; load 18.5/10 cores expires at 18 500 (checks at 9, 15, 17 s pass, 20 s refuse); waiting at 600 ms never fails and the late `joined` check at 15 s / 120 s passes (the g48p-edit sequence); saturated host ceiling 80 000; load rising mid-wait (late by at most one tick, tolerance 2000); owner deadline bounds a waiting birth; settled birth ignores clock and deadline.
- phase law: waiting/settled never fail, even one simulated hour later at stretch 12; throwing probe = idle; source law: `initial=()=>handshake.check()`, exactly one use of `maximumHandshakeMilliseconds`, no `performance.now()-started>=` (RED on base: the second wall clock).
- RED on live (base executor): 0 pass / 12 fail (`handshake.arm is not a function` + the second-wall-clock law); GREEN applied 12/12.

## Notes
- Mirror `hand` created and dropped (mine). Parse check: `gate-g-tools/scan.ts` 2 files, 0 bad.
- Limits: the real parent/child TCP exchange is not run (no product builds); the law covers the clock and the polling site, the protocol is untouched.
- The lenient choice: ticks attribute their interval to the stretch sampled at its end, so a load rise is noticed one tick late (never a false failure).
