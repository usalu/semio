# Puzzle Native Red Repair

## Scope

This checkpoint repairs the three Puzzle5d retained-import failures from the first native run, then repairs the complete 28-error Puzzle compiler packet exposed by the second run. The staged Puzzle3d and Puzzle2d runtime gates remain pending because the shared Framework Plugin source became syntactically incomplete during an active external edit.

## Puzzle5d Semantic Repairs

- The 65-mutation semantic cap is unchanged. Mutation storage is split into a const-derived array of separately reserved pages, and every page is bounded by the native/Wasm 16,384-byte job payload page. The successful transfer flattens pages only at the completion ownership boundary. Cancellation recursively retires mutation owners and then each page backing, preserving incremental close and exact terminal emptiness.
- Mutation insertion has one checked helper that enforces the global 65-item cap and refuses any page growth not covered by its exact reserve.
- Recursive JSON object retirement preflights the current key against the fixed key scratch and byte cap before descending into the child value. An oversized key therefore returns the untouched owner instead of partially mutating it.
- The import source classifier now requires exact production-prefix occurrence counts for `PartVortices`, `CatalogMutation`, and `puzzle5d_retire_part_kind_step`. Hostile replacement or duplication is rejected.
- The backing law validates every actual mutation page separately and retains the exact 32-row catalog cap, 65-mutation cap, two-page native layout, and 16,384-byte invariant.

## Compiler Lockstep Repairs

The native r2 compiler packet contained 28 errors. Two were local Puzzle5d implementation errors: an overlapping mutable borrow while constructing the catalog mutation and a redundant mutable reference in mutation-page close. The remaining 26 were stale test/source calls left by synchronous Framework transitions. Puzzle2d, Puzzle3d, and Puzzle5d now call synchronous `vcs::apply_mutation`, app snapshot, `HistoryView::empty`, and `ArtifactView::new` APIs directly.

## Puzzle3d Catalog Repair

The first Puzzle3d focused runtime stage compiled, then failed catalog authority for `acceptSuggestion` with `generated_migrated=false`. The retained proof, generated tool id, factory key, and action classification row existed, but the app action itself was absent; `action_interactive_job` therefore had no action definition to classify. A localized `acceptSuggestion` app view action now exists beside the close/hover suggestion actions. The destructor abort in the log is secondary cleanup during the primary catalog-authority panic, not an independent owner leak diagnosis.

## Native Evidence

All commands used `CARGO_INCREMENTAL=0`, `--locked`, package `semio-s-plugin-puzzle`, library tests, short compiler messages, and a single test thread.

```text
CARGO_TARGET_DIR=.../🎯️target-puzzle-checkpoint-native cargo test --locked -p semio-s-plugin-puzzle --lib puzzle5d_retained_retirement_laws --message-format short -- --test-threads=1
r2: exit 101; 28 compiler errors
r3: compiled in 1m02s; 8 ran; 7 passed, 1 failed
r4: compiled in 44.94s; 8 ran; 8 passed, 0 failed; 1230 filtered out; test time 0.02s
```

Complete logs:

- `🧪️puzzle5d-retained-native-r2.log`
- `🧪️puzzle5d-retained-native-r3.log`
- `🧪️puzzle5d-retained-native-r4.log`

The Puzzle3d focused stage selected projection packing, both `kit_in_import_media` tests, and all `tests_installs_vortex_kind_catalog` mutation laws.

```text
CARGO_TARGET_DIR=.../🎯️target-puzzle-checkpoint-native cargo test ... puzzle3d_play_projection_pack_round_trips ...
r1: compiled in 1m50s; runtime rejected missing migrated `acceptSuggestion` action; secondary destructor abort
```

After the action repair, an unrelated workspace-wide check occupied the same ticket target. The retry was moved to `🎯️target-puzzle-checkpoint-native-r2`. The global compiler cache then retained every fresh rustc request for more than two minutes, so only the local retry was interrupted and restarted with `RUSTC_WRAPPER=`. That isolated build advanced normally until the active external Framework Plugin edit produced:

```text
🧰️framework/…/🔌️plugin/🦀️component.rs:35925:3: error: this file contains an unclosed delimiter
error: could not compile `semio-framework-plugin` (lib) due to 1 previous error
```

Complete staged logs:

- `🧪️puzzle3d-sync-focused-native-r1.log`
- `🧪️puzzle3d-sync-focused-native-r2.log` (shared-target wait, locally interrupted)
- `🧪️puzzle3d-sync-focused-native-r3.log` (fresh target, global cache wait, locally interrupted)
- `🧪️puzzle3d-sync-focused-native-r4.log` (fresh target, cache bypass, external Framework parse blocker)

Puzzle3d is not claimed green after the app-action repair. Puzzle2d has not run. No Puzzle compiler process remains, and the compiler lease was explicitly returned to the coordinator.

## Verification Remaining

Once Framework Plugin is stable and the compiler lease is returned:

1. rerun the three Puzzle3d focused filters in the fresh ticket target;
2. if green, run Puzzle2d projection-pack and handle-catalog mutation filters;
3. run rustfmt parsing/formatting for the exact Puzzle files and Puzzle-owned `git diff --check`;
4. append exact final results here.
