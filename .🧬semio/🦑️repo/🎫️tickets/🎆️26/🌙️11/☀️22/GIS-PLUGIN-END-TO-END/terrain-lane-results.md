# Terrain Lane Results (interim, test runs not yet green)

Status: no Terrain test has been seen to run yet. Every cargo invocation since the source-boundary fix either waited for host capacity or was cancelled by the host warden (memory) before the editor crate compiled. Nothing below claims a pass.

## What the logs showed (all in `🗑️generated/terrain-*.log`)

| Log | Result seen |
| --- | --- |
| `terrain-exag.log` | `Rust crate semio-s-artifact-gis-gisterrain-editor exceeds its source boundary before the compiler: 29 of 32 files, 266890 of 262144 bytes`. Reproduces the digest item "lib exceeds its source boundary". |
| `terrain-exag2..4.log`, `terrain-all6/7.log`, `terrain-all8-1`, `terrain-all9-1..4` | cancelled by the host warden (`host was critically short of memory`, cargo launcher 0.8 to 1.4 GB) or repo-tool source revision faults caused by concurrent edits. No compile verdict. |
| `terrain-exag5.log` | `error[E0432]: unresolved import crate::MapFeature` in `semio-s-artifact-gis-gismap-tile-coordinates` (unrelated crate in the dependency graph). |
| `terrain-exag6.log` | editor crate (non-test) compiled; `error[E0425]: cannot find function assert_mounted_history_overwrite in module semio_plugin::app::history_edit_acceptance` in the new `commands` test target. |
| `terrain-all3.log` | `Internal error: Rust crate Source boundary exceeded before compiler: ...editor (active lines 4589/4000)`. |
| `terrain-all5.log` | `Rust crate semio-s-artifact-gis-gisterrain-inferences exceeds its source boundary before the compiler: 33 of 32 files`. |
| `terrain-all9-5.log` | `compiler broker admission failed ... semio_plugin_actor_input_decoder ... compiler needs an estimated 2191 MiB`, exit 101 (host memory). |

## Fixes made (each needs a green run to be confirmed)

1. Source boundary of the editor crate (files, bytes, active lines), by taxonomy split, no limit raised:
   - `✏️editor/🎮️commands/🧪️tests/🔬️unit/🦀️.rs` moved to the new integration target `commands`: `✏️editor/🎮️commands/🧪️tests/🧩️mounted/🦀️.rs` (modules `example`, `exaggeration`, `view`; the `exaggeration::` filter keeps matching). Cargo `[[test]] name = "commands"`.
   - `✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🧪️tests/🔬️unit` and `.../🎚️config/🧪️tests/🔬️unit` moved to the new integration target `windows`: `✏️editor/🎭️modes/👁️view/🧪️tests/🧩️mounted/🦀️.rs`. The two `cfg(test)` mounts in `📦️packages/🦀️rust/🦀️.rs` were removed.
   - `✏️editor/🧪️tests/🔬️unit/🦀️.rs` content (History acceptance laws, `history_operations_receive_under_mounted_copy_quanta`, retained command oracle) moved to the new integration target `history`: `✏️editor/🧪️tests/🧬️history/🦀️.rs`; the library unit file now only mounts the shared context.
   - Media export scene projection (`TerrainSceneRange`, `TerrainSceneView`) moved to the inferences crate next to `TerrainGridMeshProjection`: `🧬️schema/💡️inferences/📤️media-export/📐️scene/🦀️.rs`, mounted as `terrain_scene_export` in the inferences package root; `✏️editor/📤️media-export/🦀️.rs` imports it. The elevation view stays in the editor (inferences is at 32 of 32 files).
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: `history_edit_acceptance` now also re-exports the existing `assert_mounted_history_overwrite`, which the restored press-History test uses.
3. `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🌍️gis-policy/📦️packages/🦀️rust/🦀️.rs`: re-export `MapFeature` from the snapshot crate (unblocked the dependency graph).
4. Ticket scripts: `terrain-run.sh`, `terrain-retry.sh` (own compile lock `compile.lock3`, offload prefix).

## Open

- Run, in order: `exaggeration::` (target `commands`), `mounted_terrain_contours`, `history_operations_receive` (target `history`), `window_sections_project_while_the_host_utility_register_is_populated` (target `mounted`), `history_edits_end_to_end` (target `history`), full `--lib` plus all four integration targets, the DSL receiving tests, the plugin-runtime archive fault test, and the gisterrain root crate lib.
- The `semio-os-dsl-module-tests` compile errors and the record-body-cursor Drop panic listed in the evening digest were not re-run by this lane; the current receiving test file no longer contains the `semio_framework_replication` import named there.

## Latest state (end of this session)

- `terrain-all9-1..6.log`: warden cancellations (host memory), repo-tool source revision faults, and `compiler broker admission failed` for `semio_plugin_actor_input_decoder` (needs 2191 MB, grant 1536 MB; the same unit was admitted in `terrain-exag6.log`, so this is transient). Attempt 6 got through the editor's dependency graph up to that unit and then exit 101.
- `terrain-all10-1..3.log`: `EFAULT` in the repo tool, a disk collector restart, then attempt 3 queued on `semio.compiler.host-capacity/v1` for more than 3400 s with 26 rustc processes from other lanes running.
- A retry loop is still running in the background: `terrain-retry.sh all10 test -j 3 --no-fail-fast ... -p semio-s-artifact-gis-gisterrain-editor --lib --test commands --test windows --test history --test mounted -- --test-threads=2` (up to 10 attempts, lock `🗑️generated/compile.lock3`, logs `🗑️generated/terrain-all10-N.log`). Read the last log for the first real verdict.
- Not done: nothing in the Terrain lane has a test verdict; the DSL receiving crate test, the plugin-runtime archive fault test (including the `📋️preserved.json` regeneration) and the gisterrain root crate lib run were not started.

## Pass/fail table (final for this session)

| Test or filter | Result |
| --- | --- |
| `exaggeration::` (target `commands`) | not run, no verdict |
| `mounted_terrain_contours` (lib) | not run |
| `history_operations_receive` (target `history`) | not run |
| `window_sections_project_while_the_host_utility_register_is_populated` (target `mounted`) | not run |
| `history_edits_end_to_end` (target `history`) | not run |
| full editor `--lib` plus targets `commands`, `windows`, `history`, `mounted` | not run |
| gisterrain root crate lib | not run |
| DSL `receive_to_completion_names_the_unpayable_quote_and_the_quantum_when_no_turn_progresses` | not run |
| plugin-runtime `retained_window_input_recursive_document_archive_rejected_history_frame_ends_in_a_typed_fault_with_terminal_progress` | not run |

Reason: `terrain-all12-1.log` ended "Waiting for host capacity was cancelled by SIGTERM" and `terrain-all13-1.log` ended exit 130 (SIGTERM while waiting for capacity, 1650 s in the cargo-orchestration queue) when the coordinator's exclusive planner window started. `🗑️generated/exclusive-plan.active` still existed at the last check.

`terrain-retry.sh` now gates only on `🗑️generated/exclusive-plan.active`. The loop `terrain-retry.sh all13` (up to 10 attempts, lock `compile.lock3`) is alive and starts `terrain-all13-2.log` when the file disappears. Read the newest `terrain-all13-N.log` for the first verdict, then run the remaining rows above with the same wrapper.
