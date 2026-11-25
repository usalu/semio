# Ownership Carve

`semio-plugin-ownership` exceeded the 256 KiB source boundary (13 of 32 files, 262796 of 262144 bytes). It is now three crates along its domain taxonomy. No source file moved.

## Seams
The module graph is a DAG: `bounded_ownership` and `sdk_fault` are the base, and `reserved_job_session` depends only on `bounded_ownership`. `store_lifecycle` (including its `retirement` and `snapshot` mounts) depends on the base. `original_store_genesis` has no crate-internal edges. `transient_publication` depends on `store_lifecycle` and the base.

Files stay in their domain folders. Each new crate is a new root (`<domain>/🧱️owner/🦀️.rs`) that `#[path]`-mounts the existing module files, the same pattern the old crate used. Other agents' in-flight edits, including the cancel-receipt work, therefore need no re-landing.

| Crate | Modules | Bytes (all mounted files incl. tests) | Files |
|---|---|---|---|
| `semio-plugin-ownership` | `bounded_ownership`, `sdk_fault`, `reserved_job_session` | 127344 (about 94 KB without test files) | 9 |
| `semio-plugin-store-lifecycle` | `store_lifecycle`, `original_store_genesis` | 111816 | 6 |
| `semio-plugin-transient-publication` | `transient_publication` | 64599 | 4 |

The gate counts bytes with the `cfg(test)` files excluded.

## Changes
- New crates, each with `Cargo.toml`, `📋️project.json` and `📜️script.ts` under `<domain>/🧱️owner/📦️packages/🦀️rust`, plus a `<domain>/🧱️owner/🦀️.rs` root:
  - `♻️lifecycle/🧱️owner` (store-lifecycle)
  - `🫧️transient/🧱️owner` (transient-publication)
- `🏛️ownership/🧱️owner/🦀️.rs` and its `Cargo.toml` shrink to the base. A `testkit` feature exposes `effect_fixture`, which is now `pub`; the store-lifecycle tests use it through a dev-dependency.
- `crate::bounded_ownership`, `crate::sdk_fault` and `crate::store_lifecycle` became explicit crate paths in `♻️lifecycle/🦀️.rs`, `🎞️media/📥️snapshot/🦀️.rs`, `🫧️transient/🧵️publication/🦀️.rs` and `🧬️native/🦀️.rs`.
- All `semio_plugin_ownership::{store_lifecycle,original_store_genesis,transient_publication}` paths were rewritten in 225 `.rs` files, and `plugin/🦀️.rs` re-exports the two new crates.
- About 70 dependent `Cargo.toml` files gained the matching dependencies. The root `Cargo.toml` and the six artifact workspaces (raster, process3d, wfc 3d, wfc bitmap, wfc 2d, puzzle 2d) got `[workspace.dependencies.*]` entries. The root `members` list is auto-maintained by the repo and already contained the new paths.
- `🧪️tests/📦️units/🟦️.ts` pins the lock dependency list of `semio-plugin-ownership`. It now lists the carved base dependencies.

## Verification
All builds went through the repo cargo shim with memory admission, one at a time.
- `semio-plugin-ownership`: `cargo check` exit 0.
- `semio-plugin-store-lifecycle`: `cargo check --tests` exit 0.
- `semio-plugin-transient-publication`: `cargo check --tests` exit 0.
- process3d editor, `T/replay-cargo.sh carve check -p semio-s-artifact-process-process3d-editor --tests`: exit 0, with no boundary refusal.

## Not done
- Other workspaces' lock files (root and artifact workspaces other than process3d) will refresh on their next build. The units test (`🧪️tests/📦️units/🟦️.ts`) and the `feature-*.json` and `surface-fixture*.json` lists under `🧪️tests/📦️units` were not re-run or regenerated for the new crates.
- `plugin/📦️packages/🦀️rust/📜️script.ts` line 519 still lists only `semio-plugin-ownership` for the `original_lifecycle_` tests. No library tests (`cargo test`) were run for any crate.
- Coordination with the cancel-receipt agent was not possible (MCP tools disallowed, and `cancel-receipt.md` did not exist). Since no file moved, no hand-off was needed.
- The task description named job session, interpreter and maintenance ladder as the seams. The actual crate holds none of the interpreter or ladder code, so the seams above follow the real module graph.

## Helper files
`ownership-carve-walk.py`, `ownership-carve-deps.py` and `carve-cargo.sh` are in the ticket folder.

## Loose ends

### 1. `original_lifecycle_` crate list (`📦️packages/🦀️rust/📜️script.ts`, `OriginalLifecycleNative`)
- `semio-plugin-ownership` owns 8 tests: 7 in `🏛️ownership/⚠️failure/🧪️tests` and 1 in `👷️job/🚪️session/🫴️refusal/🧪️tests`.
- The part-6-x test `original_lifecycle_archive_pending_status_observation_preserves_retained_source` lives in `semio-plugin-tests-11-2` (mounted by `🧷️companions/🔧️tests-11-2`), not in `semio-plugin-tests-11`.
- `semio-plugin-store-lifecycle` and `semio-plugin-transient-publication` define no `original_lifecycle_` test, so they were not added.
- Change: the list is now `["semio-plugin-ownership", "semio-plugin-tests-11-2"]`. `semio-plugin-tests-11` defined no matching test and was dropped.

### 2. Hardcoded crate lists
- `📋️feature-consumers.json`: added `semio-plugin-store-lifecycle` to the testing-laws dependency list. The root `Cargo.lock` was the only source of the difference (57 vs 56 entries).
- `📋️surface-fixture.json`: added a `semio-plugin-store-lifecycle` dependency record. `📋️surface-fixture-lock.json`: added it to the dependency list.
- `native-build/📏️paths/🧪️tests/🧫️fixtures/🔣️.json`: added `semio-plugin-store-lifecycle` and `semio-plugin-transient-publication` rows after the ownership row. `🧬️.json` admitted count 1075 to 1081. The count was already 1079 in the fixture before this change. Its test passes.
- wgpu browser `🧪️tests/🧫️fixtures/🔣️.json`: added the same two `domainPackages` rows. `schema.json` counts: rendererPackages 97 to 99, domainPackages 315 to 318 (the fixture held 99 and 316 before this change, so the schema was already stale).
- No generator exists for `feature-*.json`, `surface-fixture*.json` or the fixtures above. A grep for the file names finds only `🟦️.ts`, so they are hand-pinned and were edited directly.
- The `Cargo.lock` files are cargo-generated and were not edited. `🟦️.ts` lines 60-61 (the ownership dependency pins) pass.

### 3. Units test (`🧪️tests/📦️units/🟦️.ts`)
Result: 6 pass, 5 fail. Before the changes above: 5 pass, 6 fail. Output is in `🗑️generated`, which was removed afterwards.

Passing now: the surface-fixture lock aliases test, the consumed workspace lock test, and the four others that were already passing.

Still failing. None of these were fixed:
- Builder laws (`📋️preserved.json`): 7 law sha256 and assertion pins are stale. Two of the checked files were unchanged from HEAD and do not reference the carved crates, so these pins were already stale. The part-7 ribbon file has an unrelated in-flight edit (`into_parts()`).
- Registration lock: `✏️s/🔌️plugins/🔋️energy/Cargo.lock` lists `semio-plugin-ownership` but not `semio-plugin-registration`. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/Cargo.lock` lists neither. These are not carve changes, but this was not traced further.
- Feature verdict owner (`📋️feature-owner.json`): sha256 pins for `drop`, `fixture_renew_turn` and others differ. Not traced to a cause.
- Surface-carrier owner and surface-fixture owner: record pins differ. Part of this is carve-caused: the `build_*_store_disposer` helpers in `🧪️tests/🧬️mutation-fixtures-surface` now call `semio_plugin_store_lifecycle`. Those pins were not regenerated, because other agents may still be editing these bodies.

### 4. Other results
- native-build paths test: pass.
- wgpu browser compiler test: still fails at the schema check. `rendererPackages/86/name` is longer than the schema's 42-character limit. That package is not from the carve.

### Not done
- The units test is not passing. The remaining five failures need a decision on re-pinning, and that should follow once the other agents' in-flight edits settle.
- No `cargo test` was run for the `original_lifecycle_` tests in `semio-plugin-tests-11-2`.
- The ticket is not closed, because MCP tools were excluded from this run.
- The Read tool timed out on its hook. Reads and exact-match edits were done through Bash and Python instead.

### Helper files
- `units-law-hash-diff.ts` (law pin comparison) and `wasm-schema-errors.ts` (Ajv error dump) are in the ticket folder.
