# r13-integrate: BIM crate integration pass

Gate label `r13-integrate`, logs `T/🗑️generated/r13-integrate/` (T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`).
Crate: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model` (lib at `🦀️.rs`, package `📦️packages/🦀️rust`).

## Compile status (updated 2026-10-09 14:15)

| target | result |
|---|---|
| `cargo check -p semio-s-artifact-bim-model --lib` | GREEN (gate log `c22.txt`) |
| `cargo check ... --lib --tests` | GREEN (`t3.txt`, 84 test-compile errors fixed in 30 test files) |
| `cargo check ... --lib --target wasm32-wasip2` | GREEN (`w1.txt`) |
| `T/r13-compiles.flag` | CREATED 14:09 |

## What has been done so far

Only investigation; no BIM file has been edited yet.
- Read the brief, `r11-exec-store.md`, `r11-exec-store-m.md` and the coordination tail.
- Ran one gate `cargo check` (`c1.txt`): the failures above are the only errors, all upstream of the BIM crate.
- Located the retirement API surface in the plugin crate (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`): `mounted_job_close_demands(instance_id, body)`, `build_{document,config,draft}_store_owners() -> Option<Result<store::DocumentStoreOwners<..>, ValueError>>`.
- Found that `#[derive(RetireOwned)]` is used across the framework (value crate retirement, 2d, diagnostic, async cancel-return). Still to do: find the derive crate and one migrated artifact (flow, dag, playbook) as the model for the BIM impls.

## Next steps (in order)

1. Find where the BIM crate implements the plugin app trait. A grep for `mounted_job_`, `store_owners` and `ArtifactApp` found these only in the standards tree (`🏅️standards/🔖️1/🪆️subsets/✳️any/` editor, viewer, window-config), none in the package root.
2. Apply `RetireOwned` to the BIM `Snapshot`, `Mutation`, transient and presence types, and add the mounted-job and store-owner hooks, following the migrated framework artifacts. No shims.
3. Re-run the gate check once os-flow and stdio-contract compile. If os-flow is still broken at 13:30, take it over (per the coordinator).
4. Fix BIM compile errors minimally, then create `T/r13-compiles.flag`.
5. Bless and write steps from the reports, `cargo test --lib` counts, python oracles (add `pypdf` to the python test group), the four generators, and the failure table below.

## Test and oracle state (updated 2026-10-09 23:25, PARTIAL: final table follows run3)

Flags: `r13-compiles.flag` (14:09), `r13-lib-green.flag`, `r13-blessed.flag` (19:34). Logs `T/🗑️generated/r13-integrate/`.

### Counts
- Run 1 (before any bless, serial, 11 stack-overflow tests skipped): 7410 pass / 1513 fail (1018 mutations: unblessed `after`/`diff` placeholders; 82 inferences; 64 io; examples house/office/demo: snapshots missing `phase`).
- Bless pass (BIM_BLESS=1): wrote 992 added + 757 modified + 205 AM fixtures; the failures printed in the same run are expected (fixtures are `include_str!`-embedded, they pass only after a rebuild). Mutation case audit (1561 cases: 639 applied, 922 rejected): 9 files of 5 cases still hold placeholders (see table).
- Run 3 (rebuilt binary, 16 threads, no bless): pending, numbers will replace this block.

### Tests that hang or overflow (skipped in the counts; owner = who must fix)
| test | symptom | owner |
|---|---|---|
| `examples::house::tests::the_attic_north_and_west_walls_are_the_mirror_images_of_the_south_and_east_walls` | no result after 5 and 25 min | w2-wp08-walldepth |
| `examples::house::tests::raising_the_ground_floor_height_does_not_move_the_entrance_ramp` | timeout 25 min | w2-wp08-walldepth |
| `examples::house::tests::the_house_zones_area_schemes_and_room_finishes_add_up` | timeout 250 s alone | w2-wp08-walldepth (house inference loops) |
| `examples::house::tests::the_house_wall_type_states_the_properties_its_walls_inherit_and_its_data_raises_no_finding` | timeout 250 s alone | w2-wp08-walldepth |
| `examples::house::tests::the_house_infers_levels_heights_and_valid_openings` | no result after 927 s alone: the whole asset house hangs in inference (every house example test shares this) | w2-wp08-walldepth |
| `export::ifc::schema4_tests::the_committed_ifc4_files_are_the_current_exports` | timeout 250 s alone | w2-wp17-ifc4 |
| `model_graph::incremental_tests::one_wall_move_in_511_walls_and_64_windows_updates_in_under_15_ms` | `#[ignore]` in the probe, debug build cannot meet 15 ms | r11-z-incremental |
| 11 tests in `editor::bim::component::unit_tests::two_plan_windows_*` and `gestures::component::app_tests::*` | `STATUS_STACK_OVERFLOW` even on a 64 MB thread (explicit 8 MB `stack_size` in `✏️editor/🧪️tests/🔬️unit/🦀️.rs:454` and `🧵️gestures/🧪️tests/🧷️app/🦀️.rs:14`): infinite recursion in the mounted-window path, new since the 677 retirement refactor | r11-store (mounted window / ladders) with the editor tests owner |
| `editor::bim::gestures::component::app_tests::debug_stack_probe` | peer debug probe left in the tree | gestures owner (delete) |

### Python oracles (`.venv`, `python 🐍️.py check <fixtures>`, driver `T/r13-integrate-oracles.py`; the harness `test oracle quick --owner bim` exceeds its 300 s budget)
- 21 of 33 oracles agree: energy, phases, zones, spaces, psets, sheets, solids-rest, ceilings, views, plan-and-diagnostics, ramps, gestures, stair-runs, wall-depth, modify, families, quantities, wall-joins, components, levels-and-wall-heights, finishes.
- After `write` (my filter did not apply, so every oracle's tables were regenerated from its third-party library) these still fail: export-bim-1-svg, ifc, ifc4, energy, ifc-energy, zoning (measure tables missing: the committed exports they read are absent or not yet blessed: owners wp08-io/wp18/wp17/wp20/w07), opening-frames (`KeyError: 'mullion'` in the oracle for the curtain-wall type change; owner w2-wp19-frame / w12), sheets-pdf (needs `pypdf`, installed; table now written).
- Before the write these disagreed with the subject and must be re-checked after run3: schedules (house takeoff values differ by 0.07 %: geometry changed under the house, owner w13), annotations (the `room` snapshot holds no annotations while the table listed `st-ground`: owner w11), wall-solids (components-mep table stale: owner f3-components).

### Mutation fixtures still unblessed (applied status, empty diff, no `after.schema`)
`create-beam/adds-an-arc-beam`, `create-column/adds-a-leaning-column`, `split-beam/splits-an-arc-beam`, `trim-extend-wall/extends-an-arc`, `delete-curtain-wall/cascades-its-overrides`: the leaf rejects or errors before blessing (arc beam / leaning column from wp19, arc trim): owner w2-wp19-frame.

### Law findings
- `r3-f1-check-names.ts`: 5 duplicate emoji directories: `io/sqlite/snapshot/schema vs .sql` (WP-19), `tests/solids-bim-1-wall-depth-three vs infer-bim-1-wall-depth` (wp08), `fixtures/ifc/psets/measure vs library.ifc` (wp18), `wall-layout/joins vs attach` (old), `mutations/wall-depth vs create-stair` (wp08).
- Generators run: mutation-facets 185 leaves, oracle 1444 scenarios, feature 185 kinds (`r3-f1-gen-model.ts` NOT run: it would emit the removed `between`).
- Open items: IFC import of leaning columns imports plumb; the gate directory lost its `slot-*` dirs (all callers picked slot 1) and BIM `Cargo.lock` serde was downgraded to 1.0.228 by the coordinator.

## BIM-side 677 migration (done blind, uncompiled)

- `semio_framework_value::RetireOwned` derive added to every `ToValue`+`FromValue` derive line under `🏅️standards` (508 lines, 257 files, via `T/r13-integrate-derive.py`, tests/oracles excluded). Generators updated byte-for-byte: `r3-f1-gen-model.ts` (4 templates), `r3-f1-gen-leaf.ts`, `r12-w2-wp18-psets-leaves.ts`.
- Editor app (`✏️editor/🦀️.rs`): dropped document/config owner+disposer, presence factories and presence disposer overrides (the framework defaults are identical); draft owners now `Some(no_draft_store_owners())` returning `Result`; `mounted_job_{maintenance,close}_{demands,step}` in grant currency.
- Viewer (`👁️viewer/🦀️.rs`): dropped document owner/disposer overrides, `no_config_store_owners()` returns `Result`, `mounted_job_close_demands/step`.
- `GestureOwner` (`✏️editor/🧵️gestures/🦀️.rs`): `retirement_demands`, grant-based `maintenance_step/close_step`, `PluginLifecycleStep`.
- Window-config macro (`🖌️render/🪟️window-config/🦀️.rs`): `build_store_owners() -> Result<..., ValueError>`.
- Upstream fix: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🎬️media-export/🦀️.rs` line 196 had `pub struct RetireOwnedSnapshotDisposerpub struct RetireOwnedSnapshotDisposer<T...` (mtime 07:56, untouched for 4 h): fixed to a single declaration.

## Compile fixes after the framework chain compiled (all minimal, author intent kept)
- `bodies/🦀️.rs`: stale tail bytes after `mod tests;` removed (scripted rewrite of an open file). `export-sheets`: raw byte string held `ä` -> `r#"..."#.as_bytes()`.
- Derive collisions: `BimPresence`, `BimPresenceMutation`, `BimWindowTransient` already have hand-written `RetireOwned` -> derive dropped there.
- Framework (needed by BIM, tiny): `RetireOwned` derive on `Viewport3dOrbit` and `WorldProjectionConfig`; `app_commands!` enum derives `RetireOwned` (trait bound `Command: RetireOwned`).
- `members_of`/`takeoff`/`curtain_quantity` now take `&FamilyProfiles` (f2-families intent: `curtain_solid_in` already passed them); `curtain_layout` projection unwraps the `Arc`; `copies` uses `crate::mutations::cascade::INVERSE_ROWS`; `PARENTS_SET`, `Point::ZERO`, `delete-selection` `&[String]`, phase-visibility JSON rows owned.
- Schedule vocabulary: arms for `ScheduleField::{Surface,FinishArea}` and `ScheduleCategory::Finish` + labels `sf_surface`, `sf_finish_area`, `sc_finish` (en/de) in `terminology`.
- `set-sheet`, `set-sheet-revision`, `set-viewport` inverses: `Patch::restoring` (there is no `negate`).
- IFC import of columns/beams follows wp19: `tilt: None`, `Axis::Line{start,end}`, `end_top_offset: None` (a leaning column imports plumb: open item for wp19).
- Test files: `use protocol::Inference` (7 files, incl. 4 inline modules), `super::super::` paths -> absolute `crate::standards::...`, `Replace` variant, `between` (removed from `DiffAlgebra` by 677) replaced in the dirty test by a JSON-built `Created/Deleted/Replaced` delta and in the schedule-config test by `assert_diff_algebra_inverse_law`, struct literals completed (`tilt`, `cut_height`, `base_slab`, `phase`, `end_join`), gesture owner test in grant currency, `label.0.as_str()`, `PluginApp as _`.

## Files touched
The 257 derive-edited files plus the files above; generators `r3-f1-gen-model.ts`, `r3-f1-gen-leaf.ts`, `r12-w2-wp18-psets-leaves.ts`; `pyproject.toml` + `uv.lock` (`pypdf`).
