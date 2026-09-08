# Test Layout Migration — 2026-09-08

The user requires every executable test at `<nearest semantic owner>/🧪️tests/<test-name>/<implementation>`, with implementation-kind basenames and no legacy test naming. This includes inline test bodies, package-scoped misplaced suites, discovery configuration, and runner references. Historical reports and data are not executable test suites.

Repo MCP `repo://goals` was read; the existing End To End Testing Refactor ticket covers this task under AI-optimized Repo. `ticket_reopen` with `26/08/23/END-TO-END-TESTING-REFACTOR` returned “ticket is already open”; work continues in that open ticket. No goals were changed.

The fleet has four concurrent slots including the coordinator. Two Terra exploration workers establish inventories; Sol workers execute bounded migrations and enforcement, with Terra returning for final audits. No modifying Git commands or worktrees are used.

## Work Ownership

- Policy worker: test domain policy, policy tests and associated root runner/Nx inputs.
- Coordinator: ticket/MCP, partition decisions, integration and validation.
- Migration workers: assigned scope plus imports and local configuration.

## Completion Criteria

- No active legacy test names or misplaced executable test bodies.
- Each case under its nearest language-neutral semantic owner, implementation files directly under the named case.
- Runners, manifests, relative imports and file fixtures resolve new locations.
- Layout enforcement has language-neutral fixtures and independent oracle validation.
- Appropriate test discovery and runtime validation executed, with concrete failures documented and resolved where caused by the migration.

## Resolved Scope Decisions

The initial native exploration report proposed forbidding Rust external `#[path]` test-module wiring and requiring one directory per individual function. Those are recommendations from that audit, not user requirements. Canonical external module wiring is accepted: the actual executable test implementation is in the required semantic case path and normal Rust module privacy remains intact. An existing named suite may be represented by one named case directory, consistent with the requested migration of existing `*.test.*` suites. Inline executable bodies in production source are still extracted.

Current execution ownership: Sol Rust worker owns all Rust test moves/extraction; Sol TypeScript worker owns all JS/TS tests outside the repo test-domain policy; Sol policy worker owns the repo test domain and shared root runner/Nx policy changes. Remaining Go/Python work will follow as slots become available. Read-only audits will be repeated after migration.

## First Scope Review

Renderer tests under `engine/📦️packages/🟦️typescript/🎯️targets/⚛️react` belong at `engine/🧪️tests/<suite>/🟦️.ts(x)`, not inside that package/target directory. Tests for an individual `🧱️elements/<element>` remain under the element owner. Runner configuration may remain beside delivery manifests. This correction was sent to the TypeScript worker before broader migration.

## Remaining Execution Queue

1. Go canonical source migration with ephemeral Go overlay discovery; Python test paths and discovery/fixture rebasing.
2. Script test implementations: 58 named self-test functions in four scripts and 57 assertion-library-importing scripts, body-level review required. Script entrypoints remain `📜️script.ts`; test implementations become canonical language files.
3. Independent Terra audit of all active test layouts and updated runner references, then focused runtime verification and cleanup.

Rust edge cases may use test-gated `include!("canonical case implementation")` wiring when a standalone function or mixed module needs its original namespace preserved. This is equivalent authored-file ownership to external `#[path]` modules; all actual test bodies still leave the production source. Validation must record nonzero discovered/executed suite counts, avoiding false success from empty test selection.
