# Norm Plugin Handoff, 2026-11-23 01:40 CET

The ticket `26/11/10/GET-NORM-PLUGIN-WORKING-END-TO-END` is still open. This handoff replaces `handoff-2026-11-22-evening.md`. The running results are in `norm-final-verification-2026-11-22.md`; its `RESULTS_PENDING` part is still to be filled.

## Components

| Component | State |
|---|---|
| `norm` | Rebuilt at 19:21 on 11-22. din4108, din16798, iso16757 and vdi3805 pass the strict Play probe. |
| `norm-energy` | Rebuilt at 20:44 on 11-22. Not yet probed. |
| `norm-eurocode` | Rebuilt at 00:51 on 11-23 after the describe-fuel fix (`describe-fuel-norm-eurocode-2026-11-22.md`). Not yet probed. |
| `norm-eurocode-materials` | Building. |
| `norm-eurocode-structural` | Queued after materials. Still carries the 11-22 10:21 build. |

## Running (check `ps` for `finish-chain.sh`, `rebuild-and-probe.sh`, `build-play-components` and `cargo test` before starting anything)

**Chain.** `COMPONENTS="norm-eurocode norm-eurocode-materials norm-eurocode-structural" ./finish-chain.sh` runs four steps:

1. Rebuilds the three components one at a time. `rebuild-and-probe.sh` retries until the core mtime changes.
2. Runs the 15-route Play sweep. Log: `🗑️generated/routes-sweep.log`.
3. Runs the official `norm-plugin:test`. Log: `🗑️generated/normtest-rs-test-h2.log`.
4. Prints `[DEBUG] … exit` lines.

**Agents and their reports.**

- **EN 1992:** done. The two compliant examples were under-designed and the engine was correct. The fix changed both DSL assets, and the tests pass 7/7. The temporary `[DEBUG]` lines were deleted after that run, which was not repeated. Report: `en1992-compliant-examples-2026-11-22.md`. The asset change landed before the queued `norm-eurocode-structural` build started, so that build includes it.
- **EN 1993, 1994 and 1998:** turns the temporary `[DEBUG]` dump tests into verdict assertions and removes the dumps. It appends to `norm-example-test-mounting-2026-11-22.md`.
- **EN 1999:** redesigns the compliant `aluminium-roof-purlin` (asset, pack twin, reference snapshot), which fails 10 checks. Report: `en1999-compliant-purlin-2026-11-23.md`.
- **Verification:** finished without any verdicts. Report: `norm-example-verification-2026-11-23.md`; the agent wrote it without reading the existing file first.
  - **Cancelled:** the runner sent SIGTERM to en1991 (after a 2h50m wait for memory admission), din4108 and din18599. The iso16757 crate and the en1997 editor test were cancelled after about a minute.
  - **vdi3805:** the runner refused the crate before compiling because it has 33 source files against a limit of 32. This needs a source-tree fix.
  - **Next:** rerun the five cancelled crates one at a time when the host is quiet.

## Remaining steps

1. Read and verify every agent report. Rerun `test-family-examples.sh <family-dir>` where a report is missing or a claim is unverified.
2. Make every family's example crate green, with real verdict assertions and no `[DEBUG]` left.
3. Rebuild any component whose assets or engine changed after it was built:
   - `norm-eurocode-structural` after the EN 1992 fix.
   - `norm-eurocode-materials` if the EN 1999 fix lands after its build.
   - `norm-eurocode` if the EN 1998 assertions changed assets.
4. Then rerun the full 15-route sweep.
5. Get the official `norm-plugin:test` green; it has never completed so far.
6. Recommit stale descriptor pairs with `repo run workspace:components -- describe <component> --profile prototype`. The build prints `[local-iterate] … differs from the committed pair`.
7. Fill in `norm-final-verification-2026-11-22.md`, delete `🗑️generated/`, and close the ticket with `ticket_close` and the full file list from the reports.

## Constraints and notes

- Run only one component build at a time. Each takes 40–70 min with 0 units reused, and example `cargo test` runs from agents slow it through the shared compiler admission.
- Exit 75 is a temporary failure, for example `Cargo.lock` or another file changing under the build because of concurrent edits. The retry handles it.
- Several framework `[DEBUG]` prints from other agents are compiled into guests, on failure paths only. They are not ours.
- The git index holds a temporary sampler in `🧠️interpreter/🏛️instance-state/🦀️.rs`, although its working tree is clean. The archivist must re-stage that path before committing.
