# Cargo Evidence Lifetime Independent Review

Read-only inspection on 2026-10-15. No tests, builds, Source capture, SDK launches, registry scopes, source edits, or Git modifications performed.

## Concrete Findings

### Coverage Output Escapes the Operation Cleanup Namespace

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts:21` rewrites every non-null coverage destination to the shared `store.captureDirectory/coverage.lcov`. The lower runner creates its unique `semio-nextest-*` namespace at `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts:190`, but its report plan still writes to the policy coveragePath (:143-144), outside that unique namespace. Its finally block (:197) deletes only the unique directory. A successful coverage-enabled routine operation therefore leaves coverage evidence despite retainArtifacts=false; subsequent owners overwrite the same shared name. The storage containment is correct after the admission rewrite, but the evidence lifetime and owner isolation are not. Put report output inside the exact per-run namespace, or explicitly define and supervise a separately retained coverage product with an owner-specific publication policy. The new ticket native capture branch forces coverageEnabled=false and coveragePath=null, so it cannot detect this gap.

### Policy Construction Creates Uncharged Metrics Directories

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔣️policy/🟦️.ts:28` calls coverageDir unconditionally, including disabled coverage and Clippy policy construction. `🏃️process/📋️context/🔣️policy/🟦️.ts:17` implements coverageDir with mkdirSync under repository metadata, outside the issued compiler backing. The later admission rewrite prevents actual report writes there, but does not undo the earlier uncharged filesystem mutation. Make policy construction pure and derive its coverage destination from the admitted operation rather than creating an unused external directory.

### Native Capture Fixture Can Leak Its Own Directory on Policy Construction Failure

Ticket `cargo-callers/📜️script.ts:13` creates `e-*` and then constructs repositoryCargoTestPolicyV1 in the same declaration before entering try/finally at :14. A missing/malformed manifest, invalid policy input, or policy construction filesystem failure leaves that fixture-owned directory behind. Enter the cleanup try immediately after directory creation, then construct policy/request inside it. This is a narrow fixture failure path, not authority to sweep the shared captures directory.

## Confirmed by Source Inspection

- Missing SEMIO_TEST_ARTIFACT_DIR now selects the declared compiler capture layout rather than an unbounded OS temporary directory. A valid explicit build mode is still required by readBuildMode; the new default corpus provides it. This is consistent with the repository's explicit build-mode contract.
- Closed corpus/schema cover trimmed explicit destinations, normalized relative paths, empty-string refusal, prototype/release defaults, and retain=false. The test inspects the path projection only; it does not prove physical storage issuance, actual runtime cleanup, failure cleanup, or concurrent ownership.
- Canonical execute rewrites target/build/capture fields to those of the actual issued compiler store. Lower admission validates those paths and non-null coverage paths against the actual backing root and requires compiler-workspace reservation custody. The ticket branch likewise uses the issued store environment for target/build and its own capture child.
- Lower runner cleanup removes only its own mkdtemp child; ticket cleanup removes only its own e-* child. Neither broadly deletes sibling agents' captures. The shared captures parent is deliberately retained.
- The ticket branch's two repeated native runs inspect only their own e-* directory. That is appropriate concurrent-agent isolation, but coverage and failure/cancellation schedules remain uncovered.

## Suggested Focused Validation

Use a controlled admitted runner to exercise success, failure, and cancellation with retain=false, plus two concurrent owners with sibling sentinel evidence. Assert each operation deletes only its own namespace and leaves no coverage report outside it. Include policy-construction failure immediately after the fixture allocates e-*. Add a missing-artifact-env projection case without workspace-root hints while running from a known workspace subdirectory; retain explicit build mode.

## Correction to Preliminary Message

An initial concern that every canonical default Cargo call would reject its metadata coveragePath was withdrawn after following execute(): it rewrites that path under actual store backing before invoking the lower runner. The findings above concern residual policy side effects and report lifetime, not a blanket default-call admission failure.
