# Current Native and Renderer Gate Checkpoint

## Subsequent Full Native Gates

The separate cold-rebase prerequisite also passes: one test, zero failed, 890 filtered, 0.03 s runtime after 0.21 s compilation. The coordinator read its DEBUG evidence that cold cursor reconstruction preserves admitted capacity for a real retained second publication. Log: `🧪️member-store-cold-rebase-r1-native-2026-08-27.txt`.

The coordinator subsequently read all three actual result logs:

- Latest-wins r17: **10 passed, zero failed, 428 filtered, 0.05 s runtime, 28.93 s compilation**. This fixes the r16 foreign-factory fixture without weakening its guard. DEBUG evidence includes exact 8,192-byte keys for six target scopes, 65 sequential claims through 64 live slots, deferred contended finish, fair ready publication, the real count-97 registered path, five Presence cancellation boundaries and ten Document/exhausted-ACK boundaries. Log: `🧪️member-latestwins-all-r17-native-2026-08-27.txt`.
- Presence r2: **10 passed, zero failed, 881 filtered, 0.00 s rounded runtime, 24.72 s compilation**. Actual DEBUG output covers all four opaque reader returns, injected release ordering, atomic contended erased transfer, mounted captured-read cancellation while Store remains live, overlapping local roots, cross-worker overlapping peer roots, all seven admission vectors with 65,536-byte backing allocations and exact actor byte counts, and original factory 1/foreign factory 0 after source publication closes. Log: `🧪️member-presence-all-r2-native-2026-08-27.txt`.
- Local interaction r1: **9 passed, zero failed, 429 filtered, 0.06 s runtime, 10.46 s compilation**. These execute exact Store capture, byte/allocation retirement, worker transfer and read-return, fixed-page ACK backpressure and wrong-token rejection, zero grants/cancellation, and a real partial canonical-encoder failure. The live-Drop panic is intentionally caught by its passing guard test. Log: `🧪️member-local-interaction-r1-native-2026-08-27.txt`.

The source self-test target independently passes 851 in `📓️coordinator-tool-job-selftests-r13-2026-08-27.md`. The coordinator also independently passes the expanded source interaction suite (12 semantic, nine hostile contract, two retirement/two hostile, two query roots across three partitions/three hostile) in `📓️coordinator-local-interaction-source-r2-2026-08-27.md`. No user-visible transport/restore or all-app timing claim follows from these isolated tests. The new full census still reports 773 rows, 350 source-admitted, 316 BatchOnly, two forbidden, 269 remaining and 851 self-tests; its mandatory release gate remains RED.

## Native Execution

The coordinator read the actual registered-dispatch r15 output. It passed one test, zero failed, 432 filtered, 0.04 seconds of test execution after 3m40s compilation. The DEBUG evidence covers same-target supersession, different-target independence, a fresh rebased worker publishing document count 97 with generation 1→2, lost-vacant-slot rejection and preservation of a foreign slot. Both rejection cases retired seven UTF-8 bytes and delivered rejection only after vacancy. This is the registered path's scoped pass, not the full latest-wins cohort.

Full latest-wins r16 aborted during the fairness case. The fixture supplied a new local-Presence retirement factory instead of the installed exact instance; the production identity guard rejected it, followed by strict Store Drop during unwind. The executor has a one-line fixture correction retaining the production guard. A full ten-test pass has not yet been observed. The same executor holds the exclusive fleet Rust compiler lease, currently for ten retained-Presence laws while the interaction executor mounts its plugin-only page tests.

Earlier build blockers are historical: r11 lacked the facade's derive export; a narrow verified repair cleared that checkpoint. A separate peer then added the real MutationLeaf macro. r12 hit Nx graph duplication, r13 a relative owned-dependency path, and r14 an Option<PathBuf> mismatch in that new peer macro. The observed path and macro fixes preceded r15. None of these failures ran tests. No peer change was reverted, and no Cargo watchdog or runtime step budget was widened.

## Independent Renderer and Composition Checks

The coordinator independently ran the full React renderer suite: 484 passed in four files, exit 0, 4.47 seconds total and 1.64 seconds of tests. Full captured output: `📓️coordinator-renderer-react-full-r5-2026-08-27.md`.

The separate full typecheck remains RED with exactly seven diagnostics. They concern real local-interaction tutorial producer/consumer joins, not BoardSession or the loader. No empty fallback or old selectionJson compatibility shape is being used to hide them. Full output: `📓️coordinator-renderer-typecheck-r3-2026-08-27.md`.

The existing dev target's linkedSessionEngines tests independently pass two tests with fifty filtered/skipped, exit 0; full output is in `📓️coordinator-linked-session-engines-test-2026-08-27.md`. Puzzle's actual generated Wasm package still requires its canonical build and complete product composition verification. Mocked Wasm DOM tests do not establish deployed browser or all-app correctness.

## Open Gates

The active packets are exact ownership/native cancellation, paged local-interaction capture and restore, and persistent renderer index plus complete retained patch processing. The six old shared-reserved semantic tails, full app command coverage (269 remaining registrations and 316 separately classified BatchOnly rows in the last complete census), fresh Wasm/browser/native shell checks, hard timing/preview gates, and dependency-removal phases remain open. No ticket or goal is complete.
