# Architecture Rule Authority

The current TypeScript/JavaScript graph verifier checks the names in `summary.ruleSetUsed.forbidden`, but does not check selectors or severity. A clean graph can therefore admit a same-name rule whose source, target, exemption, severity, or scope differs from the authored policy. Deletion independence requires the graph receipt to retain the exact policy used to enforce owner direction.

The existing dependency-direction schema and portable fixture will declare graph rule authority cases first. An independent dependency-cruiser run provides real normalized rule receipts; Ajv validates the authored strict contract. The first-party checker will accept parser normalization of path arrays into alternations and the default module scope while rejecting semantic policy changes and selectors its evaluator cannot enforce. Rule order and explanatory comments do not change authority.

The change uses the existing `@semio-tech/repo-lib:test-dependency-direction` Nx route and launch registration. No runtime library or script is added. All generated test output is kept in this ticket's `🗑️generated` folder and removed after verification. Existing root, product, and repository AGENTS.md instructions were read; no Git mutation or AGENTS.md edit is involved.

Verification evidence will be appended after the failing test and implementation runs.

The initial normal Nx invocation stalled after a daemon termination during graph preparation. It was cancelled; a daemon-free run also waited in shared graph preparation and was cancelled. Verification now uses the same registered Nx target with a ticket-owned `NX_WORKSPACE_DATA_DIRECTORY` and `NX_CACHE_DIRECTORY`, plus `NX_DAEMON=false`, to isolate its graph state from other simultaneous developers. These preparation failures are not test results.

A direct Bun runtime diagnostic ran the authored `changed-source-owner` portable case before the verifier edit. The original verifier returned normally with `accepted=true`, while the fixture declares `expected=false`; the diagnostic exited 1 and printed its `[DEBUG]` receipt. This confirms the actual same-name selector admission bug. It is explicitly a targeted runtime diagnostic, not evidence that the canonical Nx test target executed.

The first-party verifier now canonicalizes complete name/severity/source/target authority, admits the independent parser's array-to-alternation normalization and default module scope, and rejects unknown rule/selector constraints, changed scope/severity, omitted exemptions, or altered paths. Explanatory comments and ordering of whole rules do not change authority. The implementation adds no runtime dependency.

## Verification Evidence

The shared minimal ticket Nx workspace at `v` executed actual current production sources without copying them. Its `architecture-source-verification:strict` target checked the actual dependency-direction test file and its production imports with strict TypeScript flags and exited 0 (874 ms). Its `architecture-source-verification:test` target ran that entire actual test file and exited 0: **18 tests passed, 0 failed, 2,051 assertions**, in 3.62 seconds of test execution. Both were uncached runs.

The new test emitted `[DEBUG] portable rule authority cases=17; independent oracle=dependency-cruiser normalization and Ajv equality`. The same file's existing independent graph, deletion, authored package, wildcard owner, and Node export checks passed; actual public package checks reported 62 cases, 14 named contracts, and zero failures.

A prior direct targeted Bun invocation crashed in Bun 1.3.14 before it produced assertions. The later full current-source Nx run above completed successfully, so the crash is recorded as a runner incident rather than a failing feature test.

The canonical repository target `@semio-tech/repo-lib:test-dependency-direction` did **not** reach test execution: shared daemon attempts failed or stalled, and the isolated root invocation remained in graph/hash preparation for over five minutes before the turn interruption. The successful minimal Nx source checks prove the changed source contracts and regressions, not the repository-wide canonical architecture sweep. No whole-workspace architecture pass is claimed.

## Independent Review

The review checked semantic parity against dependency-cruiser's own normalized receipt format, including its scalar alternations and module scope. Unknown selectors are rejected because the first-party evaluator cannot honor them. Rule order and comment text are excluded from authority; severity and both source/target exemptions remain required exact authority. The exact sorted receipt comparison also rejects duplicate or missing reported rules. Existing graph totals, independent source inventory, source resolution, and violation parity checks are preserved.

No production source copies, runtime external libraries, extra permanent scripts, Git mutations, AGENTS.md edits, or root router edits were introduced by this enforcement change. Existing canonical command and launch registrations cover it. `git diff --check` reported no whitespace errors for the four enforcement files.

## Owned Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧱️dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️dependency-direction/🟦️.ts`
- This ticket research and verification report.

Generated evidence logs and isolated Nx state are temporary. The coordinating agent owns final cleanup of the shared ticket's `🗑️generated` folder after integrating all groups' evidence, and owns closing the existing central ticket.
