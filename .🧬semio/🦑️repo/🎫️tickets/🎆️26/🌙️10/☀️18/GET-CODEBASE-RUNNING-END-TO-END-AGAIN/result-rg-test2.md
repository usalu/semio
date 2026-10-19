# Result rg-test2: store one-item-batch test closure and serializer-rejections re-mount

Executor `rg-test2` (Sonnet 5.5, foreground, no sub-agents, no modifying git command). HARD FREEZE paths untouched (no `package.json`, `bun.lock`, `📚️library/`, root `📜️script.ts`, `🔣️taxonomy.json`). Generated output (`🗑️generated/rg-test2`) deleted.

## 1. `semio-framework-os-store-one-item-batch` test closure under the gate

Gate = `readRustSourceClosureV1` (`rg-test-tools/gate-full.ts`), host cfg + `test`, limits 32 files / 262,144 bytes / 4,000 lines.

| | files | bytes | lines | ratio |
| --- | --- | --- | --- | --- |
| before (test) | 26 | 335,667 | 3,921 | 1.280 |
| after (test) | 15 | 210,966 | 2,746 | 0.805 |
| prod (unchanged class) | 9 | 180,944 | 2,410 | 0.690 |

Method: the whole `paged-one-item/🧬️mutation-apply/🧪️tests` tree (root 56 KB, immutable-root, semantics, semantics/apply) plus the source-custody tests it mounts (23 KB) left the lib closure and became the `[[test]] mutation_apply` target (root file unchanged in place, so its `#[path]` submodules and `include_str!` fixtures keep working). The lib no longer mounts `mod tests` in `mutation-apply/🦀️.rs`.

Changes forced by the move (all paths now resolve to owner crates, the old ones were stale `crate::os_store::*` monolith paths):
- test root: `use semio_framework_os_store_one_item_batch::paged_one_item as paged;`, `use semio_framework_os_spr as protocol;`, every `super::super::X` -> `paged::X`, `crate::os_store::replay::` -> `semio_framework_os_store_replay::`, `crate::os_store::one_item::` -> `semio_framework_os_store_one_item::`, plus `#[global_allocator]` = `semio_framework_trace_memory_testing::RequestedAllocator` (the tests call `observe_heap_allocations_on_this_thread`; the dev-dependency already existed).
- source-custody: imports from `paged_one_item`, `crate::os_store::snapshot_read::..` -> `semio_framework_value_snapshot_read::..`.
- Visibility widened only where the relocated target needs it: `paged_one_item::{retain_seal_returns, returned_edit_demands, close_returned_edit, returned_post_demands, close_returned_post}` are `pub` (the failed-seal custody test drives exactly these); `Stage` is `pub` and re-exported from `paged_one_item` (the `pub stage` field of `MutationApplyEdit` already exposed it).
- lib root `one-item-owners/🦀️.rs`: `#[cfg(test)] #[global_allocator]` (the retained in-lib tests, e.g. the prepared-retirement test, assert actual allocator receipts and had no observer installed in this crate).
- `Cargo.toml`: `[[test]] name = "mutation_apply"`.

## 2. Serializer-rejections test re-mounted in `semio-framework-os-io`

`os-io` is the lowest crate that sees both: it depends on `os-store-read` (-> value snapshot-read) and owns `io_mechanism`/`owned_serialization`. New `[[test]] owned_serializer_rejections` at `🚪️io/📤️serialization/📦️owned/🧪️tests/🦀️.rs` (next to its fixture `🧫️fixtures/🔣️.json`, now read by a local `include_str!`). The test text is the saved one with imports moved to owner crates: `semio_framework_os_io::{io_mechanism, io_schema}`, `semio_framework_value_snapshot_read::{SnapshotRead, SnapshotReadRegistryHandle, artifact_retirement_box_demands, artifact_retirement_box_close_step}`, `semio_framework_value::{retained_clone::RetainedCloneGrant, retirement::{RetireOwned, RetirementCursor, RetirementStep}}`.

One deliberate rewrite, to avoid widening a private type: the private `SnapshotReadReturnRetirement::new/demands/step` loop is replaced by the public cursor of the same witness (`witness.retirement()`, `next_demand(0)`, `close_step(grant)` asserting no `RetirementStep::Failure`); same drained-to-terminal assertion. It also installs the `RequestedAllocator` global allocator. `Cargo.toml` of os-io: `[[test]]` plus dev-dependencies `semio-framework-trace` (workspace), `semio-framework-trace-memory-testing`, `semio-framework-value-snapshot-read` (the latter is already a transitive dependency, no new edge into the framework workspace; value-snapshot-read gets no dependency on os-io).

Gate after: os-io prod 0.623, test 0.789 (19 files, 206,725 B, 2,385 L). value-snapshot-read: prod 0.392, test 0.844 (unchanged by me).

## 3. Verification

- `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` (run from repo root): 127 pass, 0 fail (an earlier run during the dependency sync failed 64 tests with ENFILE only; rerun after the sync exited is clean).
- `bun C/rg-test-tools/gate-full.ts` on the three manifests above: numbers in the tables.
- `rustfmt +nightly-2026-07-20 --check --config skip_children=true,disable_all_formatting=true` (parse only) on the 6 edited Rust files: OK. `Bun.TOML.parse` of both edited manifests shows the new `[[test]]` rows.
- Cargo: the sync had exited, so `bun repo cargo ... check --manifest-path Cargo.toml -p semio-framework-os-io --test owned_serializer_rejections` was attempted; it stops before compiling: `cargo fetch --locked` -> "cannot update the lock file Cargo.lock because --locked was passed" (the lock needs `dependencies lock`; the frozen library and lock refresh are the coordinator's). **No compile evidence for either relocation.**

## 4. Open items

- Lock refresh (`bun repo dependencies lock`), then `check -p semio-framework-os-io --test owned_serializer_rejections` and `check -p semio-framework-os-store-one-item-batch --tests`.
- Pre-existing, not touched and visible in the moved tests: the one-item-batch crate does not look compilable at HEAD independent of this task: `protocol::` paths (`use protocol::MutationMessageConcat`, `protocol::MutationMessage..` in `paged-one-item/🦀️.rs` and `mutation-apply/🦀️.rs`) have no `protocol` alias in the lib root, and `mutation-apply/🦀️.rs` matches on `Stage::Birth/Decide/Source/Plan/Preflight/Commit/Join/Publish` while the enum has only `Inverse/Apply/Retire/Done` and the struct lacks `closing/semantics/plan/source/join/...`. The relocated tests inherit the same `Stage::Source/Commit` references. The monolith split of this crate needs its owner.
- The new `[[test]]` target needs registering wherever the crate's exact-law/test rows are enumerated (none found for these test names in the store tree).
- `value-fix-tools/owned-serializer-rejections.rs.txt` is now redundant (the test lives in os-io) and can be deleted by the coordinator.
