# Fix seven editor_app tests (FEM 3D)

## Shared custody root cause: "sealed source requires original granted handback"

Message comes from `Drop for SealedShared<T>` (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs:173`, debug assertion): the LAST owner of a sealed value (here `SealedShared<PersistentList<FemNode>>` inside `Fem3dSnapshot`) was dropped plainly instead of being retired through a funded handback. A `Fem3dSnapshot` therefore must never be dropped as the last `Arc`/owner outside the retirement machinery.

Two distinct sources, only the first is fixed:

1. Test code (FIXED, `transform_utility_arms_the_gumball_over_the_selection`): the test held a raw `Fem3dSnapshot` (`fem3d_demo_snapshot()`) and dropped it at scope end. Fix: hold it as `crate::context::Retiring::new(...)` (it Derefs, retires on Drop). Rule for every test: any snapshot you birth yourself (`fem3d_demo_snapshot`, `fem3d_boot_snapshot`, `Fem3dSnapshot::default()`, `apply_mutation(...).0`) goes into `Retiring`.
2. Production/framework (NOT fixed, `context::history_edits_end_to_end`, macro `history_edit_acceptance_law!`): backtrace (run `🗑️generated/fx7/r4.log`) shows the drop inside the history time-travel replay path: `SealedShared::drop` <- `drop_glue<Fem3dSnapshot>` <- `Arc<Fem3dSnapshot>::drop_slow` <- a drop in `semio_os_store_read::derived_snapshots_replays_and_history_read_plan` (`ArtifactDerivedReplay` / history read retirement, `🏪️store/🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs`) <- `semio_plugin_time_travel_turns::turn_driving_and_replay_steps` <- `acceptance_pump`. So a last `Arc<Fem3dSnapshot>` (preview/replay `state`, `displaced_operation`, or `derived_alias`) is dropped without going through `ReturnedSnapshotReadRetirement::admit_original`. This matches the open item 1 in `26/11/22/.../fem-history-replay.md` (Round 3): "plain drop of the last `Arc<Fem3dSnapshot>` in the close rung (history_edits only)". It only hits snapshots with sealed lists (FEM 3D), which is why 2D-style fixes do not show it. Owner: the history-replay agent; next probe is the symbolized backtrace (`fhr3-sym.py`) to name the exact field.

Windows build trap seen while verifying: with the default new build-dir layout `semio-os-spr` fails with `os error 206` even when target and build dir are the same. Working recipe: separate build dir (`C:\t\fx7b`) worked (script `fx7-law.sh`); fem-history-replay.md also documents `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false`.

## Per test

1. `add_combination::add_combination_refusals_have_complete_localized_notices_3d`: cause: `natural_pair` split "dead 1 2" at the last whitespace into case "dead 1" and factor 2, so it reported `terms-case-unknown`. Fix (parser; unreadable text must report unreadable): in the whitespace spellings the case token may not contain whitespace (`single` helper), so "dead 1 2" is unreadable. `:`/`=`/`*` spellings still allow spaced case names. File: `✏️editor/🎮️commands/🔗️add-combination/🦀️.rs`. Law unchanged. PASS (run r4).
2. `add_nodal_load::add_combination_invalid_terms_json_is_a_no_op`: cause: law predates the refusal; `dispatch` expects settlement success. Fix: new test helper `dispatch_refused` (returns the refusal debug text) in `🧪️tests/🔬️app/🦀️.rs`; law asserts `fem3d.combination.terms-unreadable` and an unchanged document. File: `🎮️commands/📍️add-nodal-load/🧪️tests/🔬️unit/🦀️.rs`. Status: see below.
3. `add_solid::add_solid_refuses_an_unknown_material_and_a_degenerate_extent`: handler was already correct (`fem3d.solid.extent` for non-positive extents). Law bug: it took the material id from the demo app but `refusal()` validates against the boot snapshot, so the material was unknown first. Fix: take the material from the boot snapshot for the extent refusal. File: `🎮️commands/🧊️add-solid/🧪️tests/🔬️unit/🦀️.rs`. PASS (run r4).
4. `artifact_panel::every_tree_row_icon_id_is_registered_3d`: the code assigns 10 row icons (`circle-dot minus box triangle list arrow-down link layers beam settings-2`) but the boot demo has no solids, so only 9 appeared and the `>= 10` coverage floor failed. Registry check itself passes for all. Fix: the law adds one solid to its demo so every entity kind is covered. File: `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`. PASS (run r4).
5. `context::history_edits_end_to_end`: see shared root cause (2). NOT fixed here.
6. `model_window::transform_utility_arms_the_gumball_over_the_selection`: see shared root cause (1). PASS (run r4).
7. `model_window::model_scene_renders_solid_mesh_and_oriented_member_instances_3d`: scene ids are now scope-qualified (`scope:1:7:element2:e1`, load glyph `interactionId` `scope:1:4:load4:live2:l2`), and node instances exist too; nothing is missing. Fix: the law matches the id suffix with its granularity/mesh (`:e1","interactionGranularityId":"element","meshId":"box"`, `:s_00",...support...cone`, `:live2:l2"}`). File: `🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs`.

## Files changed
- `✏️editor/🎮️commands/🔗️add-combination/🦀️.rs`
- `✏️editor/🎮️commands/📍️add-nodal-load/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/🧊️add-solid/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧪️tests/🔬️app/🦀️.rs` (new `dispatch_refused`)
- script `fx7-law.sh` (run: `/c/t/fx7-run.sh <tag>`; logs in `🗑️generated/fx7/`)
