# Norm Plugin Handoff, 2026-11-22 22:50 CET

The ticket `26/11/10/GET-NORM-PLUGIN-WORKING-END-TO-END` is still open. It replaces `handoff-2026-11-22.md`. Results so far are in `norm-final-verification-2026-11-22.md`; its `RESULTS_PENDING` part is still to be filled.

## Done and verified

- **`norm`** rebuilt at 19:21. The strict Play probe passed din4108, din16798, iso16757 and vdi3805 (`🗑️generated/routes-h1.log`).
- **`norm-energy`** rebuilt and published at 20:44. Exit 75 came after publication. DIN V 18599 was not yet probed on the new build.
- **JS tests:** `norm-js:test` 12051/0 and `norm-js:test-wire-twins` 11914/0.
- **Contract verifiers:** `verify-norm-document-contract` and `verify-norm-results-window-ownership` pass.
- **Example documents:** schema audit of all 79 (`norm-example-schema-audit-2026-11-22.md`). Eight EN 1990 headers fixed.
- **DIN 18599 examples:** 15/15 after the `cooled-office` asset fix (`din18599-cooled-office-example-2026-11-22.md`).
- **ISO 16757 examples:** 4/4 after the `demo` test assertion fix (`iso16757-demo-example-2026-11-22.md`).
- **EN 1996 examples:** 2/2 per the agent. The engine now emits a coupled thickness and bearing-depth remedy (`en1996-examples-2026-11-22.md`). The confirmation run with the final edits passed 2/2.
- **Disk:** removed 19 GiB of idle private cargo target directories from `26/08/29/AI-MCP-END-TO-END/🗑️generated` (`cg`, `host-spill`, `native-parser-test`, `native-guests`).

## Running when this was written (do not start duplicates; check `ps` for `build-play-components` and `rustc`)

1. **Materials rebuild.** `rebuild-and-probe.sh materials norm-eurocode-materials en1995 en1996 en1999`. Logs: `🗑️generated/rebuild-norm-eurocode-materials-materials-*.log` and `routes-materials.log`.
2. **EN 1992 agent.** Fixing the compliant examples, which produce 2 and 10 failing checks. Report: `en1992-compliant-examples-2026-11-22.md`.
3. **Mounting agent.** Mounting all unmounted example tests in en1990, en1991, en1993, en1997 and iso16757, and adding tests for din16798 and en1997. Report: `norm-example-test-mounting-2026-11-22.md`.
4. **Describe-fuel agent.** `norm-eurocode` failed at describe: `fuel exhausted` at the 32e9 cap after 896 s, although it described fine at 08:12. The agent is looking for the root cause. Report: `describe-fuel-norm-eurocode-2026-11-22.md`.

## Remaining steps

1. Read the four reports above and verify their claims: rerun `test-family-examples.sh <family-dir>` for each touched family.
2. After the describe-fuel fix:
   - Build `norm-eurocode`, then `norm-eurocode-structural` after the EN 1992 fix, one at a time: `COMPONENTS="norm-eurocode norm-eurocode-structural" ./finish-chain.sh`.
   - That run does the full 15-route sweep, then the official `norm-plugin:test`.
   - `rebuild-and-probe.sh` retries until the core mtime changes.
3. If `norm-eurocode-materials` failed at describe for the same reason, rebuild it after the fix.
4. The official `norm-plugin:test` has never run green. The first run died with `No space left on device`; 53 GiB are free now. Fix any failures.
5. Recommit stale descriptor pairs. The build reports `[local-iterate] … differs from the committed pair`. For each component run `repo run workspace:components -- describe <component> --profile prototype`.
6. Several framework `[DEBUG]` prints from other agents are compiled into guests (poll_wit refusal, FEM cleanup, natural-file worker, retained close, receipt). They only print on failure paths and are not ours.
7. Fill in `norm-final-verification-2026-11-22.md`, delete `🗑️generated/`, and close the ticket with `ticket_close`. The file list is in the reports and their "changed files" sections.

## Constraints

- Run only one large Rust component build at a time.
- Each component build compiles about 900–1050 units from scratch, about 40–70 min, with 0 reused (see `component-warm-rebuild-audit.md`).
- Run `repo cache gc --lang=en` before each build; `rebuild-and-probe.sh` does this.
- Concurrent edits by other agents can stall snapshots. One example is `🗣️dsl/✨️derive/🗂️schema-document-index.json`. Stop and restart rather than wait for 120 retries.
