# 🏃 Norm Runtime Integration

Date: 2026-11-04.

The TypeScript package test entry point referenced a deleted plugin-root descriptor and a deleted Rust parity suite. The deployed descriptor is owned by the hub norm composition. It declares fifteen editor apps, with each retained command at app scope. The old loop searched every window for every retained command, falsely reporting 220 missing actions even after resolving the descriptor path.

The runner now reads the deployed component through a committed language-neutral test plan, validates all 75 app-command identities and requires each admission classification, explicitly executes the existing document-contract oracle, and runs the existing wire-twin suite instead of the removed parity path. Wire twins independently compare all fifteen families and every committed mutation/snapshot/diff against the strict Ajv oracle.

Added a small Bun regression test driven by the test plan and retained-command fixture. It checks the descriptor path, committed test suite paths, all fifteen editors and all 75 retained classifications using the existing third-party Bun assertion library. Execution first failed before Bun due read-only default host-capacity queue. With the shared ticket capacity directory, it waits on the host swap ceiling. No runtime dependency was added.

Files: norm TypeScript package-command test runner, language-neutral package-runner fixture, package-runner regression test.

## Verification

- Runner plan regression: 1 pass, 0 fail, 79 assertions, 141 ms.
- Registered Python ingress and runner regression after the public facade fix: 2 pass, 0 fail, 81 assertions, 2.96 s. The field-meta Python checker reports zero mismatches.
- The process tools public facade previously exported only ToolErrorV1. A real test failed on its missing runToolV1 export; the facade now explicitly exports the registered runner and its owned option/result types, and the same check passes.
- Schema-sync and field-meta-choices now execute asynchronously through the registry Python interface, with UTF-8 and bytecode generation disabled. They are included in the committed main package test plan.

## Generated Contract Repairs

EN1991 snapshot and three insertion payload twins were regenerated from canonical schemas. Railway derailment, rail load groups, and dust explosions now use their own schema-owned records and readers. Added JSON action fixtures and independent strict Ajv witnesses, with malformed foreign record, missing field and undeclared field cases. Regenerated all twelve EN1991 artifact, snapshot and diff Rust/TypeScript/protobuf/GraphQL projections from their schemas using the owned twin planner; authored Rust behavior is retained by the planner splice operation.

SQLite consumers now import actual schema-owned exports: six EN1990 enum aliases, EN1991 FireOccupancy, two EN1996 enums and twenty-two EN1997 record aliases. No compatibility exports were introduced. The TypeScript compiler AST regression covers all fifteen SQLite projections and all fourteen projections that consume the snapshot interface (ISO16757 owns its separate binary64 representation).

The strict schema annotation vocabulary now declares x-semio-rust and x-semio-dsl as object annotations. Their omission had prevented Ajv from compiling the norm snapshot schemas at all. Strict keyword checking remains enabled.

The initial action/schema import regression passed five tests with 106 assertions in 1.104 seconds. Broader checks exposed malformed duplicate required arrays and further artifact/diff twin drift, reported to the root agent for repair. The workspace lock importer now includes the added test-only workspace process dependency, with no dependency version changes.

## Further Source Conformance

The independent schema checker found seven EN1991 artifact-array Rust annotations naming RotatingMachine despite owning distinct referenced record schemas. Corrected those annotations to FatigueModel, SupportCollision, LocalFire, SnowGuard, RailDerailment, RailLoadGroup and DustExplosion, then regenerated the canonical Rust structs.

The checker also misread protobuf enum names containing an uppercase boundary, e.g. NodeARestraint: its prefix became NODE_ARESTRAINT_ instead of the schema projection spelling NODE_A_RESTRAINT_. Added committed protobuf/JSON fixtures. The test reproduced the exact incorrect A_RESTRAINT_* and B_RESTRAINT_* values, then the checker was corrected to derive the exact prefix with its existing acronym-aware word splitter. Enum member spelling and order comparisons remain unchanged.

The expanded action and SQLite regression passed all four tests across all fifteen projections. Broader schema checks fell from 71 to 27 mismatches before the root canonical pass; the remaining source fixes and latest full checks are coordinated by the root agent.

## Strict Oracle Admission

After the canonical pass, all three schema-sync tests passed: every checked JSON/Rust/TypeScript/protobuf/GraphQL structure agrees, and canonical checking reports zero semantic drift and zero style drift.

The strict wire oracle then exposed malformed UI metadata previously masked by schema compilation failures. Corrected EN1997 measurement argument role to value while preserving its measurement group. Corrected ten reference annotations whose property was accidentally renamed from kind to the edited field name (EN1991, EN1993, EN1999). Removed two unsupported, redundant catalogue hints from EN1995 strength-class references; their kind and actual schema enumeration remain. These are source annotation corrections, with strict meta-schema validation preserved.

The strict annotation audit now passes all 8,988 UI annotations. Corrected the remaining two EN1995 select option arrays into bilingual value-to-label objects and removed four empty unit annotations for dimensionless EN1999 fatigue slopes. Registered the existing string-valued x-semio-catalogue annotation and catalogue-id format under the owned policy: the first-party Rust validator explicitly asserts only seven listed formats and treats all other format names as annotations. Ajv uses the same policy; no asserted format or strict keyword check was disabled.

Latest strict wire run is queued behind the normal host capacity gate (no active workload in our shared ticket gate; the host itself is below the required free-memory floor). It has not yet executed the full corpus after metadata repairs. Do not read prior setup failures as completed wire-corpus results, or the passing static/schema tests as a native Rust compilation.

## Corpus Formatting and EN1997 Semantics

The first fully executing strict wire suite ran 11,296 tests: 4,245 passed and 7,051 failed. A bounded census separated 1,920 source-schema failures, 147 reader/schema disagreements, zero decoded-value changes, and 5,131 formatting/order mismatches. Exact-byte comparisons remain enforced; failure diagnostics now report a bounded mismatch message rather than whole snapshot diffs.

An inline independent Python JSON/schema-aware formatting pass corrected all 5,131 initially admitted format mismatches. Before every write it independently parsed both texts and asserted equal values, keys, IDs and signed zero, refusing nonfinite JSON. Zero failures occurred. No one-off formatter or migration script was retained. Details: fixture-format-audit.md.

EN1997 native row_patch! creates Option setters and applies default decoding; its nineteen local patch definitions and three shared Case*Patch definitions incorrectly required all fields. Corrected these source contracts and regenerated the four diff facets. Three new language-neutral sparse-patch fixtures test omission and reject undeclared fields with independent Ajv and production readers; the test reproduced red, then passed.

Corrected forty-one EN1997 null case lanes to the native empty CaseRowDelta after proving each fixture before.cases equals after.cases. Corrected eight stale invalid enum diff values ending in -alt to the exact typed mutation payload and after-state values. Seven already described real changes. The applied load-case situation fixture had the same persistent value before and after despite claiming an applied diff; its prestate now uses valid transient while preserving the intended mutation and resulting persistent value, target IDs and all other fields. Native equality guard and emitted patch assignments were inspected before these edits.

## Final Source and Strict Gate Handoff

The source work and full strict wire gate are complete. The final all-fifteen-artifact census checks 11,282 committed wires with zero source-schema failures, zero production-reader disagreements and zero decoded-value changes. Its remaining 1,583 formatting differences were corrected by an independent inline Python JSON/schema-aware pass, asserting unchanged parsed values, keys, IDs and signed zero before every write; zero failures. No formatter or migration file was retained.

The full strict wire suite then passed **11,298 tests, zero failures, 62,137 assertions in 35.94 seconds**. Schema validation, negative refusals, undeclared-member rejection and exact byte comparisons remain enabled. Command: repo --service=off tools exec bun -- test ./✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts.

### Actual Package Entry Still Requires a Fresh Worker

The proper declared entry is repo --service=off run @semio-tech/norm-js:test. First inspect repo --service=off run @semio-tech/norm-js:test --dry-run so any declared build dependencies can be coordinated with the root-owned native job. The raw repo ./📜️script.ts test route in the package directory reaches the receiver without an issued parent Script birth and therefore refuses; do not fabricate its envelope. Passing --service=off before a raw script path also selects the build frontend and is not the script route.

This existing agent still runs in the old restricted PID namespace (a fresh registered Bun query reports pid 29, ppid 2), although the parent now executes on full-access host PIDs in the millions. The proper target dry-run failed at the already-proven descriptor 3 / end-callback / EPERM / write boundary before task planning. This does not prove a transport defect in the current full-access environment. Transport was not changed. The root owns the separate actual native invocation and its lifecycle. No runtime job from this agent remains live.

A fresh full-access worker should keep SEMIO_AGENT_SESSION_DIRECTORY pointing to the ticket generated/agent-sessions directory and open its own attributed session through repo session open --agent=norm-runtime-full --client=codex --export. It must **omit SEMIO_HOST_CAPACITY_DIRECTORY**, restoring the authored machine-wide capacity registry now that the default cache is writable; the parent verified other live jobs and unrealized reservations in that registry. Never lower or bypass admission gates.

### Retained Evidence

- final-wire-census.md: all fifteen artifacts, zero semantic/reader/value failures.
- fixture-format-final-audit.md: all 1,583 final formatting writes, zero value changes or failures.
- auxiliary-publication-diagnostics.md: six focused tests, 40 assertions, plus the root-confirmed actual restricted FD 3 error boundary; no transport fix claimed.
- Generated final-wire-census.log and final-wire-census.json: complete final semantic census output and row inventory.
- Generated final-fixture-format.log: independent final formatter output.
- Generated final-wire-strict.log: complete 11,298-test strict wire result.
- Generated final-main-entry.log: actual raw-script parent-birth refusal.
- Generated final-main-plan.log: proper declared target dry-run and typed auxiliary publication failure in this restricted agent.
- Earlier generated wire-corpus-census.json and en1997-wire-census.json retain the semantic versus formatting classification used for repairs; the latest final census supersedes them.

All generated logs and inventories are under the ticket generated folder for deletion only after the ticket is actually finished. Read-only audit harnesses and retained Markdown reports are input/evidence files and remain.
