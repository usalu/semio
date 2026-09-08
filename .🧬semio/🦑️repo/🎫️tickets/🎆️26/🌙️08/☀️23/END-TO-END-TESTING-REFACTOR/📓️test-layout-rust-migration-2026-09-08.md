# Rust Test Layout Migration

## Outcome

The final Rust-only contract census inspected 19,860 live Rust files and reported zero test-layout findings. Canonical Rust implementations now use `<semantic-owner>/🧪️tests/<emoji-kebab-case>/🦀️.rs`; test bodies no longer remain in the production files or legacy Rust test paths handled by this lane.

The extraction moved 299 nested inline modules from 147 sources, 3,791 top-level inline modules from 3,426 sources, and 82 standalone functions from 34 sources. Those batches account for 8,810 plain test attributes. Macro-generated plugin tests were moved into `🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs` through hygienic helper macros so downstream invocations still generate their test cases. Existing assertions were retained.

The migration also moved 303 whole-file Rust violations, applied five special ownership moves, renamed 253 invalid named-case directories as whole directories, rebased Rust include and fixture paths, and repaired explicit Cargo test targets. Nineteen test-only proxy modules and five inline helper wrappers were eliminated by mounting their canonical implementations directly. A final 17-path wiring batch flattened four remaining helper proxies and repaired eight nested or stale canonical edges.

Three resident-memory mounts referenced ticket-local proposal files that were already absent from the repository before this goal. The retained `INTERACTIVE-JOB-RUNTIME-REFACTOR` reports explicitly describe those files as ticket-only proposals; its pre-goal baseline also lacks them. The dead mounts and their observation-only hooks were removed instead of inventing replacement test bodies. The package's 17 extant tests compile and execute.

## Verification

- Rust contract census: 19,860 files, zero findings.
- Post-rename full literal traversal inspected 43,820 Rust `include!`, `include_str!`, `include_bytes!`, and explicit `#[path]` edges. Its one transient test-fixture finding was corrected concurrently; an immediate final scan of all 7,166 canonical Rust test files inspected 20,740 literal edges and found zero broken test edges.
- The full traversal also found 11 current production-only edges outside this test-layout work: one process utility module and ten concurrently changed Trinity schema includes. They are preserved as production failures rather than hidden by this migration.
- `rustfmt --edition 2021 --config skip_children=true` completed on the 56 recorded final repair files, followed by formatting of the resident and causal files.
- `NX_DAEMON=false ... bun nx run @semio-tech/value-resident-rs:test --skip-nx-cache`: 17 tests run, 17 passed, zero skipped.
- `NX_DAEMON=false ... bun nx run @semio-tech/framework-replication-rs:test-quick --skip-nx-cache`: 257 tests run, 257 passed, zero ignored or filtered; the corrected causal document-backbone fixture test passed.

The pre-goal tree contains 12,501 plain Rust test attributes, while the final live tree contains 12,552. The net increase of 51 belongs to concurrent repository development; the extraction journal's 8,810 moved attributes and the final zero-finding census provide the migration-specific preservation evidence.

## Evidence

The exact deduplicated authored-path arrays and old-to-new/reference mappings are indexed by `📓️test-layout-rust-changed-files-2026-09-08.md`. Recovery uses read-only base revision `6152f9ca6a0fbb55aa61992077837a230996b51d`, the final explicit wiring graph, normalized-content matches, test-declaration identity matches, whole-directory relative-path matches, and retained guarded mutation records. Each mapping records its evidence class. All deleted legacy test paths are included in the authored-path array; 154 are separately listed as unmatched because the surviving evidence does not prove one exact destination association. Two deleted production paths with test declarations are listed as unmatched and excluded because their deletion can belong to concurrent non-layout work.
