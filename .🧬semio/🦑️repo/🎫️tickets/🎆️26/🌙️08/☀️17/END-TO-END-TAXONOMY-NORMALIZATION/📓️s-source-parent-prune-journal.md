# Journal-Bound Source-Parent Pruning

## Safety Decision

The CAD/Draw transaction regression exposed empty source directories in the verification inventory before commit. Exact bottom-up pruning is now modeled before verification without deleting any directory before durable commit. Independent review found that recomputing this set during terminal recovery could expand cleanup after another actor removed a previously blocking child. Recovery must use the exact committed set, not a new emptiness census.

Every newly created journal therefore requires `sourceParentPrunePaths`. It is empty before commit, unique and bytewise sorted in persisted JSON, and may change only in the `verifying -> committed` transition. The committed record is the durable pruning authority. The parser rejects missing fields, duplicates, non-path data, and nonempty precommit authority; there is no compatibility parser for older journal artifacts.

Each recorded path must be a strict parent of a planned move source and not a parent of a planned destination. Cleanup additionally protects the ticket root and all its ancestors, performs no-follow ancestry checks, rejects any child outside the approved set, and uses nonrecursive empty-directory removal. Partial pruning is resumable. Once executor staging and backup roots have been removed, terminal replay never prunes newly recreated directories.

## Test-Driven Verification

A permanent language-neutral fixture defines the exact approved paths, retained parents, unsafe paths, and input bytes. The ticket-local test uses an independent Ajv schema to validate the journal's exact path set.

The first fixture attempt used a Cargo lock without its required manifest and correctly failed inventory. That fixture was corrected to use the existing permanent-script fixed-name contract as the retained parent blocker. The intended preimplementation run then failed all three cases because the commit boundary event and journal field were absent.

After implementation, the expanded packet ran:

```text
bun test --timeout 120000 './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️source-parent-prune-journal.test.ts'
5 pass, 0 fail, 65 assertions, 5.93 seconds
```

The cases cover interrupted committed cleanup, an exact recorded set, newly emptied but unapproved parents, unsafe/foreign/duplicate paths, impossible phase data, concurrent child insertion, symlink replacement, partial pruning, terminal replay after directory recreation, and precommit rollback with directory mode preservation. Fixture Git commands affect only disposable repositories below this ticket. Actual Compose trees and the real Git index are untouched.

## Frozen Boundary Fixture Refresh

The existing transaction golden contained 60 journal nodes across 63 transaction ledgers and 98 boundary assertions. Those nodes were mechanically extended with the expected empty pruning array; only dependent canonical journal bytes, sizes, SHA-256 values, Base64 payloads, ledger hashes, and boundary hash references changed. No workspace ledger was changed.

- Previous golden SHA-256: `e3c9dbad890beda23b7ed8233cb027ccd9374dc77ed72beb077c55ba2fd4138d`.
- Updated golden SHA-256: `3d0aeadbf0a9458597ae9a13c5f6169515802e6b30034a26119cc0f19e30081b`.
- Golden self-audit: 1 pass, 0 fail, 804 assertions, 74 milliseconds.

The complete uncached Nx aggregate is a separate required regression gate; its result is recorded below when available. The new packet alone is not a claim of monorepo convergence or timing acceptance.

## Fixture Baseline Stability and Complete Capture

The first complete Nx attempt failed the frozen audit snapshots. Inspection showed that the fixture's Git baseline included the current production taxonomy schema: unrelated schema edits therefore changed the baseline commit, plan digest, and journal paths, despite identical transaction behavior. The isolated fixture now leaves its physically present and validated taxonomy outside the committed fixture tree. An assertion verifies that exclusion. This is not a plan-hash mask or a production inventory exclusion. The aggregate runner additionally snapshots the schema alongside its compiled modules so child processes use one coherent schema for that run.

A direct source capture with concurrent child compilation completed with 52 passing tests and 10 test-level timeouts (45.87 seconds); no semantic assertion failed. A filtered Nx invocation was also unsuccessful because its forwarded shell argument contained an unquoted pipe; its incomplete-plan subtest passed, but the wrapper did not. Neither attempt counts as acceptance.

The complete compiled capture, with maximum concurrency three and the fixed schema/module snapshot, completed with 62 passing tests, zero failures, 1,691 assertions, and 63.86 seconds. Capture mode intentionally does not compare golden boundary hashes, so this is evidence collection, not the non-capture regression gate.

All 98 expected boundary keys were captured, with no extras or omissions. All nine workspace ledgers remained byte-identical. A separate comparison of every transaction node proved that all 98 ledgers were semantically identical after substituting only their exact old/new plan identity in paths and JSON strings and disregarding the consequent JSON payload encoding, size, and hash. Modes, states, revisions, operation sets, backups, and the newly explicit empty pruning arrays were unchanged.

The reviewed capture mechanically refreshed the 63 transaction ledgers and their boundary references. The final fixture is 403,163 bytes with SHA-256 `7b700d79e5474417f0c92ddce61f5ffdd24603af56241d0fbdc3cdd5ba560296`. The retained capture is `🧪️source-parent-journal-boundary-capture-stable`. The refreshed self-audit passed: 1 test, zero failures, 804 assertions, 375 milliseconds overall. A complete non-capture run remains required separately.

## Subsequent Regression Attempts

An Nx rebuild taken during another agent's schema-first edit correctly rejected the newly declared artifact-facet contract before its parser branch was installed. That run is not acceptance. Once the schema/parser pair was coherent, the complete uncached Nx attempt passed every completed boundary comparison but hit its unchanged 14-second aggregate watchdog; it did not finish all cases.

A separate complete compiled non-capture run, maximum concurrency three, completed with 61 passing tests and one failure, 1,958 assertions, 98.97 seconds. The failure was the mixed-generator child readiness marker timeout; all completed golden comparisons matched. This is not a passing aggregate. The next work is to expose child failure evidence promptly and complete a coherent run without weakening the gate.

The document-owner and pruning packet passed together after scoped-walker integration: 10 passing tests, zero failures, 154 assertions, 53.32 seconds. The separate empty-facet generator-consuming fixture exposed an additional input-transition defect in existing resume validation, documented independently when fixed; this is not a failure of journal-bound source-parent authority.

## Complete Non-Capture Regression

The child launcher now drains bounded stdout/stderr evidence, reports an early child exit immediately instead of waiting for an absent marker, and terminates owned children in failure cleanup. A deliberately exiting child first failed the new readiness diagnostic assertion, then passed after implementation. The filtered uncached Nx gate passed one test with six assertions in 2.15 seconds. Test child processes explicitly disable the Nx daemon; the original timing limits remain unchanged.

The complete compiled non-capture aggregate subsequently passed all 62 tests with zero failures and 1,987 assertions in 50.27 seconds (maximum concurrency three). Every frozen golden comparison was enabled; this was not capture mode. This also covers the mixed-generator process-tree interruption and second-attempt commit that previously timed out. The separate 14-second uncached Nx aggregate remains required; a passing slower diagnostic run does not satisfy its timing gate.

A subsequent full uncached Nx attempt again reached the unchanged watchdog: the four shards terminated at 14.08, 14.08, 14.12, and 14.13 seconds. Only three tests completed before termination; their assertions passed. The target exited one. This is recorded as a timing failure, not as a complete passing aggregate, and no unrelated host process was stopped.

## Subsequent Launcher Correction

While rerunning the compiled aggregate after generator-input integration, the coordinator incorrectly supplied a bare relative test path instead of an absolute path. Bun treated it as a filter, enumerated 168,685 file names, and ran no tests in 7.91 seconds. The invocation exited one. No Compose file was intentionally opened, restored, or modified, but this accidental discovery invocation cannot certify opaque-directory traversal exclusion. It is not test evidence and is retained as a safety deviation, alongside the earlier separately documented launcher deviations. The immediate retry uses absolute paths for the test, compiled module, and schema snapshot.

The corrected complete compiled retry finished with 60 passing tests, two test-level timeouts, and 1,982 assertions in 71.20 seconds. The mixed-generator interruption case reached 15.01 seconds, and the forged backup/restore case reached 17.03 seconds against the unchanged 15-second test limits. Every completed frozen assertion passed. This is not a complete passing regression and does not supersede the earlier complete 62-case green run. A separate two-case diagnostic is being run serially with the same compiled snapshot; it cannot qualify as aggregate acceptance.

A read-only host census during investigation showed a 10-logical-CPU, 32-GiB machine running concurrent Rust compilation, repository enumeration, application rendering, and other developers' work. None of those processes was terminated or modified. Performance acceptance still requires the complete uncached target, unchanged identities, and the original time budget.

The same-snapshot serial two-case diagnostic finished with one pass, one timeout, 38 assertions, and 22.79 seconds overall. Forged backup/restore rejection passed in 7.12 seconds; mixed-generator interruption/retry again exceeded 15 seconds. This is a diagnostic result only, not a passing aggregate or a reason to weaken its time limits.

## Package Authority Before Committed Pruning

The JCO package lifecycle exposed a distinct ordering defect. Its source leaves had moved correctly, but the now-empty source schema directory remained physically present until durable commit, as intended. Canonical package authority consumed that old directory before the later inventory prune projection and rejected the otherwise complete destination. The retained preimplementation lifecycle failed with zero passes, one failure, sixteen assertions, in 62.33 seconds; its four convergence findings are recorded under the package lane's `🧪️nested-cargo-integration-dhSRzW/🧪️evidence/🔣️empty-source-plan.json`.

Public inventory remains strict. A private final-verification path now supplies only the exact set computed by `emptySourceParents` to package authority. It omits only those directory entries from package facts and verifies their direct no-follow directory kind before omitting their physical child entries. The existing post-inventory digest projection, before-commit emptiness recomputation, committed journal set, and terminal prune validation remain unchanged. No public ignore option or serialized schema field was added.

The root reran the actual JCO lifecycle after this correction: one pass, zero failures, twenty-three assertions, 23.75 seconds overall. Both real Nx adapter generation/check attempts ran; the first rolled back, the second committed at ordinal two, Cargo metadata agreed, and the fresh plan was empty. The package lane independently proved ordinary public inventory still rejects both an unplanned empty source directory and a nonempty one, preserving its added child.

An uncached filtered transaction-v2 Nx rebuild also passed one test, six assertions, in 2.23 seconds with a 2.27-second shard runtime. That result is compilation/focused-regression evidence only, not the complete aggregate timing gate.

The first dedicated five-case pruning rerun failed during fixture setup because its old full production schema now required a registry producer that this isolated non-generator fixture does not materialize. Its fixture schema now removes only the unrelated registry `inputDiscovery`, matching the already established isolation in the transaction and input-transition fixtures. No production selector or generator authority was relaxed. The corrected packet result is recorded after completion.

The corrected dedicated pruning packet passed all five tests with zero failures and 65 assertions in 20.75 seconds. It revalidated exact committed sets, foreign/unsafe journal rejection, new-child and symlink protection, partial cleanup/replay, and precommit rollback with original directory modes after the private package-authority projection was installed.
