# r11-z-incremental: dirty-key incremental recompute (engine + model-graph)

Status: engine verified (32 engine tests green in an isolated crate), BIM side WRITTEN BUT UNVERIFIED (the BIM crate cannot compile since commit 677; see Blockers). No timing exists yet.

## What was built

### Framework engine `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs` (public API kept; additions only)
- `InferredField::touched_by(snapshot, key, &TouchedPaths) -> bool` (default `true`): the per-key twin of `reads()`. `false` lets an incremental update carry the previous value and hash of an entity without evaluating `dep_input`.
- Session state per field is now `Stored { values, links (hash, parents, digest share, epoch), digest, epoch, sound }`; `InferenceSession::root()` is an incremental order-independent digest over `blake3(key ‖ hash)` of every entity (replaces the string-based `result_root` merkle); `field_values::<P, F>()`.
- New diff-driven update API: `InferenceSession::begin_update` / `step_field_update` (fuel + cancellation, like `infer_field_step`) / `finish_update`, `infer_field_delta` (sync, returns `FieldDelta { gated, done, changes: Vec<FieldChange { key, old }>, computed, hits, carried, confirmed }`). `infer_field_after_diff` is now `infer_field_delta` + one clone of the values.
- Per entity: carried (no dependency evaluation, no hashing) when it has a previous link, the same parents, was not touched (`touched_by`) and no parent moved; confirmed (dependency evaluated, hash equal, value kept, no compute) when its hash did not move; otherwise cache lookup, then compute. Entities that left the plan are swept when the plan is done.
- Laws kept: with the cache disabled every update is a plain recompute; a cancelled/failed update marks the stored values unsound, so the next update examines every entity (nothing is carried, the session is never gated); the plan-order violation is still a loud `MissingParent`.
- Driver: no per-node `step.clone()` (the plan is borrowed next to the cursor fields), parent values are gathered only on a cache miss, `DepHash::chain` hashes the sorted parent hex directly (byte-identical to `merkle_node`, proven by a test), `hex` module removed.
- `TouchedPaths::intersects_parts(&[segments])` (allocation-free, a part may contain `/`) added in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` (additive, one method, edited at ~02:00 before `r11-store` took ownership of the 📡️spr crate; 🌱️value/♻️retirement was NOT touched by me).

### BIM `S/🧬️schema/💡️inferences/🕸️model-graph`
- `[DEBUG]` timers removed (`🧮️compute`: `DEBUG_TIMES`, `value_timed`); `zz_debug_timing` replaced by the `#[ignore]` release benchmark `one_wall_move_in_511_walls_and_64_windows_updates_in_under_15_ms` in `🧪️tests/📈️incremental` (511 walls, 64 windows; nine wall moves, min/median/max, equality with a fresh inference, asserts `< 15 ms` in release only).
- New module `🎯️dirty/🦀️.rs`: `touched(snapshot, key, &TouchedPaths)`, the per-key dependency mapping (row granularity: element row, its type row, host row, storey + building placement, materials for quantities and glass railings, rail host rows; aggregates and unknown kinds answer `true`), wired as `ModelGraph::touched_by`.
- `📡️session`: the session (w03's stepped API kept as is) now drives `begin_update` / `step_field_update` / `finish_update`; `adopt` copies only the `FieldChange`s into the held inference (`project` on the first and on a stale run); a cancelled or failed run drops the engine state and sets `stale`. `UpdateReport` gained `carried` and `confirmed`.
- Tests: `🎯️dirty/🧪️tests/🔬️unit`: (1) law: for every row edit (delete / create / perturb) of nine fixtures, every node the diff did not report as touched has an unchanged `dependency`; (2) an incremental update equals a fresh inference after every row edit and back on three fixtures.

### Engine tests (language-agnostic fixture `💡️inference/🧫️fixtures/🔄️diff-update/🔣️.json`, oracle `serde_json`)
`a_diff_update_serves_exactly_the_entities_the_diff_and_their_parents_moved_and_equals_a_pure_recompute`, `a_cancelled_diff_update_leaves_the_session_distrusting_its_values_until_a_whole_update_ran`, `a_diff_update_without_an_enabled_cache_is_a_plain_recompute_of_every_entity`, `the_chain_hash_equals_the_merkle_node_it_replaced_and_the_default_touched_by_says_yes`, `touched_paths_match_a_part_with_slashes_and_ancestors_without_splitting`.

## Verification so far
- Engine in isolation (scratch crate that `#[path]`-includes the engine file with stubs for `os_spr::command` and `io::text::inferences::encode`, real `semio-framework-hash/value/pack-json`, via the gate): `cargo test --lib` -> **32 passed, 0 failed** (27 pre-existing + 5 new), no warnings in the engine file.
- BIM crate: not compiled (see below). Benchmarks: not run.

## Blockers
- Owner commit 677 (retirement refactor) leaves `semio-framework-os-kernel` and everything above it uncompilable; r11-store/r13-integrate are migrating it. Last observed state: kernel, plugin, dag, playbook, flow-artifact compile; `semio-framework-os-flow` and stdio-contract still fail, then the BIM crate needs its own migration.

## To run once `T/r13-compiles.flag` exists
1. `T/🚦️gate.sh r11-z-incremental -- cargo test --release --manifest-path S-crate/Cargo.toml -p semio-s-artifact-bim-model --lib -- --ignored --nocapture one_wall_move` (before = 353 ms measured by the previous agent; after = to be filled).
2. `cargo test ... --lib model_graph` (session, incremental, dirty, annotation tests) and `os_inference` in the kernel.
