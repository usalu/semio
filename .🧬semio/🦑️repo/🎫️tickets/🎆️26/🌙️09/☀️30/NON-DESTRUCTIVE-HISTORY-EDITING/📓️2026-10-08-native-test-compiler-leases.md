# Native Test Compiler Lease Admission

Native Cargo test execution now requires explicit compiler-build and lease directories in the existing Cargo test policy schema. The repository policy author resolves the actual environment/configured Cargo build directory and supplies the existing canonical resource-lease directory. Runtime invocation binds `CARGO_BUILD_BUILD_DIR` to that admitted authority.

The test driver admits the real Cargo artifact profile, acquires the existing exclusive build/profile lease before list/compilation, and retains it through assertions, optional coverage reporting, child termination and captured metadata handling. Its finalizer releases the lease on success, failure, queued cancellation and process timeout. Existing command arguments, budgets and nonzero selection fences remain unchanged. Clippy uses the same compiler authority. This cannot protect a raw compiler that bypasses the canonical owner or a process that loaded the previous unguarded driver.

Nextest `--profile long` selects assertion configuration and leaves the normal compiler lease at debug. Nextest `--cargo-profile`, ordinary Cargo `--profile`, release selection and coverage release resolve against the existing native profile schema. Empty, malformed and conflicting declarations refuse. Eight language-neutral vectors are checked against independent Node CLI parsing and Ajv admission.

## Actual Captures

| Capture | Actual result |
| --- | --- |
| 50649, schema/neutral/exclusion RED | 0/3, eight expectations, 10.59s. Actual missing profile API and an actual child spawned while the shared compiler lease was held. |
| 51715, first GREEN attempt | Profile and real SQLite exclusion laws passed; cancellation matcher ordering stalled the test harness. This owned capture was cancelled, not credited as full GREEN. |
| 52284, normal process owner | **3/3, 57 expectations, 1.74s**, terminal 19:11:34.846 UTC. Visible DEBUG for every profile and list/run exclusion, cancellation and timeout closure. |
| 53573, full existing Cargo driver | 5/6. Actual Cargo/Nextest execution passed; Kernel neutral target fixture still expected arguments from before the required mutation-testing feature. |
| 55389, aligned current driver | **6/6, 225 expectations, 10.93s**, terminal 19:16:05.770 UTC. Actual selected library/integration/default Kernel fixture binaries and existing Nextest capture/empty-selection laws executed with DEBUG. |

The Kernel target fixture now declares `mutation-testing` and expects the exact feature arguments already authored in the current Kernel package script. Its original target-selection expectations and native assertions remain intact. Temporary compiler directories belong to this ticket and are removed by the existing fixture finalizers; no shared cache was deleted by these captures.

Independent `bun:sqlite` transactions refused while both Node list and assertion children held the admitted compiler lease, then succeeded after terminal release. Queued cancellation produced no child. A timed-out child terminated before the same compiler resource could be reacquired. These are actual execution observations, separate from full application/browser proof.

## Canonical Controls and Files

The new process target `test-cargo-test-leases` calls the existing `📜️script.ts test cargo-test-leases`. Canonical launch seed 900.0581192 registers that normal owner with ticket-scoped artifacts. Generated launch publication remains gated by the current compiled PNG receipt.

Authored changes are in Process Cargo policy schema/neutral fixture/driver, its new `🔒️lease` neutral corpus and tests, the existing Cargo driver tests, Process script/project target, repository `repositoryCargoTestPolicyV1`, the current Kernel runner fixture, and the canonical launch seed. No runtime third-party dependency was added.

## Recovery Scope

This admission closes the demonstrated Nextest/Clippy gap for current canonical execution. Safe recovery still requires excluding already-running older or unmanaged compilers and rescanning stale finalized sessions while the exact compiler profile and cache-prune authorities are held. The earlier read-only inventory measured only 1.390 GiB of stale finalized sessions; it does not authorize deleting active/newest sessions, current build units, or whole compiler roots.

## Scoped Incremental Compaction

The neutral fast-glob/real SQLite compiler-exclusion law executed RED57065 (0/1,3 expectations), then GREEN57661 (1/1,17 expectations). The canonical dry run58462 planned64 finalized debug sessions. After a fresh empty compiler census, canonical apply62260 held cache-prune plus the exact debug compiler lease, rescanned and removed64 superseded finalized sessions totaling1,570,211,957 logical bytes at19:27:49.231Z. All64 paths are physically absent; all64 affected crate directories retain sessions. No newest/working/foreign-profile output, ticket input/report or whole cache unit was removed. Disk observed356MiB before admission and1.5GiB afterward; logical bytes are not a promise of exact APFS space reclamation. The original read-only audit remains timestamped separately. Actual path/retention receipt is generated/ui-execution/oct8-compaction-apply-verification.json.
