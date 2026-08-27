# Mutation Input Carrier Enforcement

## Scope

The direct-owner folder shape does not prevent a semantic mutation from accepting an arbitrary whole-schema diff. Independent textual and glTF audits confirmed this defect in 145 previously converted leaves. None of those leaves is accepted on the strength of the earlier structural-only results.

## Test-First Guard

The new language-neutral fixture covers twelve cases: enum Restore, differently named replay, nested Box/Option, snapshot vectors, ordinary and grouped import aliases, type aliases, nested payload structs, legitimate typed prior values, scoped non-aggregate deltas, unreachable helpers, and comment/string decoys. A qualified aggregate alias is also covered.

The independent oracle is the pinned Rust compiler's structural auto-trait solver: a test-only auto trait has negative implementations for the aggregate Diff and Snapshot. Requiring that trait on the leaf independently determines whether either forbidden state type occurs transitively in its input. The oracle does not reuse the repository's lexer or graph traversal. No runtime dependency is introduced.

The first registered Nx attempt stopped before tests because an unrelated taxonomy edit temporarily disagreed about the external CAD consumer roster. Its transcript is retained and is not a regression result. The next run executed the test and failed as intended: the compiler rejected an enum Restore carrier while the existing policy reported no carrier violation.

## Implementation

The repository inspector reads the aggregate's actual `mutations(snapshot=..., diff=...)` type identities once per root, resolves local type/import aliases, and follows reachable struct/enum payload fields from the wrapped direct leaf. It reports a high-severity `mutation/no-generic-snapshot-fallback` finding with the carrier path. Method return types, imports alone, unrelated helper types and comment/string content do not count as mutation inputs.

This guard is not the entire semantic verifier. Cross-file opaque aliases, payloads decoded dynamically from untyped bytes/JSON, executable codec-delegation analysis, and behavioral footprint validation still require the broader closure work. The current root-level code must never treat absence of this one finding as full acceptance.

## Live Checkpoint

The first hardened rerun covered the same 29 roots as the previous descriptor-identity checkpoint. It reported 143 carrier findings: glTF 120, SVG 9, XML 6, JSON 5 and TXT 3. TXT was already under active repair, so this is a live observation, not an immutable 145-leaf baseline. No additional roots were flagged by this new check.

## Transcripts

- `🧪️mutation-input-carrier-red.log`: unrelated taxonomy pre-test failure.
- `🧪️mutation-input-carrier-red-rerun.log`: executed expected failing regression.
- `🧪️mutation-input-carrier-green.log`: test-module load blocked by the terminality lane's not-yet-exported test seam; no green claim.
- `🧪️mutation-input-carrier-green-rerun.log`: 15 tests passed, including all twelve carrier oracle vectors; one expected terminality-lane red test failed. The overall run failed and is not a green suite.
- `🧪️mutation-input-carrier-focused-green.log`: later isolated retry exceeded the 30-second quick budget before assertions; not a passing result.
- `🧪️mutation-input-carrier-focused-budgeted.log`: supported `SEMIO_TEST_BUDGET_MS=180000` retry passed the isolated carrier test with 45 expectations, zero failures and 279 filtered tests. Actual test run took 7.59 seconds; the carrier test body took 1.74 seconds.
- `🧪️direct-roots-input-carrier-checkpoint.log`: exact live 29-root findings.
