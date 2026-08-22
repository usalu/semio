# Phase 8 Schema and UI Warning Audit

## Scope

Owned warning cohorts: `semio-framework-schema` and `semio-framework-ui`, plus the icon generator that owns UI's generated `IconName` source.

## Corrections

- Replaced public-trait `async fn` declarations in the schema metadata traits with explicit `Future + Send` return bounds while preserving asynchronous callers and generated implementations.
- Made immutable facet and descriptor value objects `Copy`; added `ArtifactSchemaRegistry::is_empty`; removed redundant `#[must_use]` attributes; and applied Clippy's direct `map_or_else`/borrow fixes.
- Modernized UI option handling, simplified a manual option map, removed clones of `Copy` icon ids, and made an orphaned prose comment non-documenting.
- Added `FromStr` for generated `IconName`.

## Narrow lint exception

`IconName::from_str(&str) -> Option<Self>` retains its established Option-returning lookup contract for generated callers. The generated type also implements `std::str::FromStr`; Clippy still rejects the intentionally retained inherent name. Therefore its generator emits one item-level `#[allow(clippy::should_implement_trait, reason = "…")]`, with the reason recorded in source. No broad lint allowance was introduced.

## Verification

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts generate rust` (assets package) | Passed; regenerated 249 catalog icons. |
| `cargo fmt -p semio-framework-schema -p semio-framework-schema-derive -p semio-framework-ui -- --check` | Passed after formatting. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo clippy -p semio-framework-schema --all-targets -- -D warnings` | Passed. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo clippy -p semio-framework-ui --all-targets -- -D warnings` | Passed. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo check -p semio-framework-schema -p semio-framework-ui --all-targets` | Passed. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo test -p semio-framework-schema -p semio-framework-ui --all-targets` | Passed: 15 schema tests; UI has 0 tests. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo test -p semio-framework-ui --all-targets` (post-icon generation) | Passed: 0 tests. |

## Gate progression

The initial exact plugin gate emitted 22 owned errors: 14 schema and 8 UI. After the corrections it compiles both owned packages cleanly and reaches a different, unowned `semio-framework` cohort: 81 errors, led by `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, followed by manifest and OS workflow files. No Puzzle, FEM, Animate, renderer, prepared-render, or framework-2d source was modified.
