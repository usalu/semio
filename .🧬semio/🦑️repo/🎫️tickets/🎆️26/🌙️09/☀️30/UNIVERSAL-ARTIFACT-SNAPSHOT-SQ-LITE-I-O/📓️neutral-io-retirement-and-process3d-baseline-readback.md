# Neutral I/O Retirement and Process3d Baseline Readback

## Retirement Move

Read-only source verification; no Cargo or runtime execution. The neutral I/O schema owns the definitions and now owns exactly one explicit retirement implementation each:

- `🧰️framework/🔨️modules/🚪️io/🧬️schema/♻️retirement/🦀️.rs:3`: ArtifactDialect fields artifact_kind, standard, subset.
- Same file4: ArtifactRef fields artifact_id, dialect.
- Schema root `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:8` mounts the adjacent retirement facet.
- Package root `…/🧬️schema/📦️packages/🦀️rust/🦀️.rs:2` mounts vocabulary and reexports those exact types.

Framework and artifact Rust scans found only these two retirement macros and no explicit duplicate RetireOwned impl for either type. Store retirement source no longer contains either identity macro. The package already depends on first-party semio-framework-value; current dependencies are four first-party path crates. serde_json is dev-only here. No external runtime dependency was introduced by the retirement facet. This source arrangement addresses the orphan ownership reason for E0117; compiler success remains Root-owned and unverified here.

Existing neutral ownership test facet is mounted in package root under cfg(test): `…/🏛️ownership/🧪️tests/🦀️.rs` has neutral_vocabulary_preserves_shared_literal_identity_and_independent_json and neutral_vocabulary_preserves_shared_controlled_refusal_categories. These prove authored identity/conversion/refusal coverage, not an explicit retirement-grant law: no retirement test is present in that facet. A mechanical implementation relocation does not itself change the field retirement sequence.

## Process3d Exact Baseline Selection

`SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-native --skip-nx-cache` selects the existing package route. Package script delegates cargo name semio-s-artifact-process-process3d with snapshotSqliteTestFeatures=[], so no extra features are required. Generic package runner `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:77` passes exactly `--lib sqlite_snapshot_ --no-fail-fast`. Native mode skips Source execution at78. The eight Process3d names all carry sqlite_snapshot_process3d_; no second selector is required by this target.

Source route is `SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-source --skip-nx-cache`; it bypasses Cargo and executes actual snapshot/🧪️tests/🪶️sqlite/🟦️.ts. Consumer route remains check-snapshot-sqlite-source. Package lib path resolves to its artifact root ../../🦀️.rs; actual Snapshot mounts Native SQLite tests at659.

Staged Native test facet uses existing serde_json dependency (package Cargo48), existing ArtifactDsl/ArtifactPack, actual neutral fixture via include_str and existing owned types. No missing local fixture or dependency prerequisite was identified in this source readback. Compilation is unverified; genuine controlled producer absence is intended assertion-level baseline behavior, not permission to mount an ordinary fallback before execution. Current missing capability and unmounted/unpaid Native SQL candidate remain documented in the adjacent Process3d wiring audit.
