# 📓️ engineering — plugin native test debt (🏗️fem · 🔋️energy · 🏭️process · 🌍️gis · 🪵️sourcing)

Session 5 (fleet v4), 2026-09-21. Every number below comes from a run made in this session (or from the
coordinator's own baseline, marked as such); nothing is claimed that was not executed.
Logs: `🗑️generated/engineering/*.txt`; running notes: `🗑️generated/engineering/STATUS.md`.

## Per crate — first run → status

| crate | first run | log | after the fixes |
| --- | --- | --- | --- |
| `semio-s-plugin-fem` | ok 5 / 0 (coordinator baseline 14:00) | `🗑️generated/baseline/semio-s-plugin-fem.txt` | untouched |
| `semio-s-artifact-fem-2d` | **fail** 1258 / 3 (coordinator baseline) | `🗑️generated/baseline/semio-s-artifact-fem-2d.txt` | 1 fixed, 2 are wall-clock laws (below); re-run queued |
| `semio-s-artifact-fem-3d` | **fail** 1130 / 1 | `🗑️generated/engineering/first-assembly.txt` | not fixed (below) |
| `semio-s-artifact-energy-model` | ok 6292 / 0 (1 ignored) | `first-noassembly.txt` | untouched |
| `semio-s-plugin-energy` | **fail** 3 / 1 | `first-noassembly.txt` | fixed; re-run queued |
| `semio-s-plugin-process` | ok 3 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-process-concrete` | ok 5 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-process-metal` | ok 5 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-process-robotic` | ok 5 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-process-wood` | ok 6 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-artifact-process-process3d` | **fail** 315 / 44 | `first-noassembly.txt` | six root causes fixed; re-run queued |
| `semio-s-plugin-gis` | **fail** 5 / 1 (lib); native-codecs target ok 3 / 0 | `first-noassembly.txt` | not fixed (needs `describe`, below) |
| `semio-s-artifact-gis-gismap` | **fail** 262 / 1 | `first-assembly.txt` | not fixed (below) |
| `semio-s-artifact-gis-gisterrain` | **fail** 88 / 4 | `first-assembly.txt` | 2 fixed, 2 open (below); re-run queued |
| `semio-s-plugin-sourcing` | ok 3 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-sourcing-beams` | ok 2 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-sourcing-slabs` | ok 2 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-plugin-sourcing-windows` | ok 2 / 0 | `first-noassembly.txt` | untouched |
| `semio-s-artifact-sourcing-curation` | **fail** 151 / 1 | `first-noassembly.txt` | fixed; re-run queued |

The confirming re-runs (`verify2-noassembly` = process3d + sourcing-curation + plugin-energy, then
`verify2-assembly` = gisterrain + fem-2d, plus `verify1-assembly` = fem-2d on the shared dir) were still
queued on the shared build-dir lock when this report was written — 12 fleet cargos, 7 live `rustc`, my two
invocations idle for 70 min. **The fixes below are landed on disk but NOT yet proven green by a run.**

## Root causes fixed

1. **Mounted app never settled its typed operations** (process3d, 26 of the 44 failures).
   `context::dispatch`/`action` called `dispatch_typed`/`handle_action` and returned. A bound (mounted) app
   answers BEFORE its migrated command publishes, so: the document never changed (`snapshot()` stale, every
   catalogue/inspector/workshop render projected the pre-dispatch tree), `InvocationResult.mutations` was
   empty by construction, and — because the store still owed a publication — `Process3dApp::drop` drained
   1 048 576 close turns without reaching the terminal-empty shallow shell ("Process3d fixture never reached
   its terminal-empty witness", 11 tests). The harness now settles (`settle_registered_typed_operation`),
   folds the receipt's effects into the answer, and exposes `settled_dispatch`/`settled_action` plus
   `published_a_document_mutation(receipt)`; the ten `!result.mutations.is_empty()` assertions became
   document-lane assertions on the receipt, which is the mounted app's observable "this command wrote the
   document". `seed_domain_catalog_contributions` dispatches through the same helper, which is why the
   contribution-config law and every seeded render were failing too.
2. **Framework-reserved verbs were only ADMITTED** (process3d, gisterrain).
   `handle_action("interactionSelect")` / `"undo"` hand back an `Effect::SpawnJob` for the reserved job;
   without `settle_framework_reserved_admission` the selection never lands in the interaction store (three
   process3d inspector renders showed "No selection") and an undo never rewinds (gisterrain's
   `exaggeration_drag_coalesces_into_one_undo_step` read 3.0 where the fixture's 1.5 was due). process3d's
   `settled_action` now settles it; gisterrain gained `context::history_verb`.
3. **Bare `ArtifactStore::new` in the process3d wasm fixtures** (5 failures).
   No owner catalog → `reserve_edit_history_slot` refuses every `Apply` with
   `edit history insertion requires its exact mutation retirement factory`. The fixtures now install the
   app's own `crate::spr::process3d_document_store_owners()` and close through a bounded
   `ArtifactDocumentStoreDisposer` loop (the gismap `close_gis_map_candidate` shape).
4. **A process-global four-slot publication lease registry with per-file test lanes** (process3d, 3 failures,
   `process3d-publication.saturated`). `🔬️retained-laws` serialised itself on a `static` lane of its own while
   `🔬️mounted-registry` admitted into the same direct-mapped table from another thread. The lane moved next to
   the registry as `crate::spr::process3d_publication_authority_lane()` and both suites take it.
5. **`window_measures` needs window INSTANCES** (process3d, 2 failures). `VcsArtifactApp::window_measures`
   projects one entry per `ViewModel.window_instances` and keys it by the instance id, so the harness's
   `ViewModel::default()` answered an empty map ("main window measures" / "no entry found for key").
   Added `context::main_window_view()` (the shape gisterrain's harness already uses) and routed both tests
   through it.
6. **A test that dispatched without settling could never close** (sourcing-curation).
   `view_kind_config_only_commands_pass_kind_discipline` used a bare `dispatch_typed`, leaving the migrated
   command's operation pending, so `SourcingTestApp::drop` could not reach the terminal-empty shell. It now
   goes through the context's settling `dispatch`, which proves exactly the same config-only property.
7. **A route test's window-view map was stale** (fem-2d). `every_route_declares_the_lane_its_handler_emits`
   mapped every window-scoped command to a `ViewModel` except the newer `SetTransformGumballFlag`, which fell
   into `_ => command.dispatch(…)` (no view) and faulted `fem2d.gumball-flag.window-context-required`.
8. **A viewer law aimed at a seam this viewer cannot use** (plugin-energy). `assert_viewer_never_mutates`
   drives the STATELESS `ViewerApp::handle`; since `setCamera` became `InteractiveJobClassification::Migrated`
   (2026-09-16) the energy model viewer's emission is a window-config write, which `ViewEmit` structurally
   cannot carry, so `handle` refuses loudly by design and the helper's
   `expect("viewer adapter command succeeds")` is unsatisfiable. `energy_model_viewer_never_mutates` now
   asserts the property over the seam that decides it — every declared viewer verb is `Migrated`, so none can
   reach the stateless seam — while the emission itself stays proved window-config-only by the artifact
   crate's `a_camera_gesture_becomes_an_addressed_window_config_write_and_nothing_else` and
   `every_viewer_publication_lane_is_a_window_config_lane`. See the proposed framework diff below.
9. **A direct-leaf scope named a relative descriptor path** (gisterrain). The test expected
   `provenance.descriptor_path == "{owner}/../../🧫️fixtures/🧬️direct-leaves/🔣️.json"` and passed the same string
   as `MutationLeafSourceScope.descriptor_filename`, but `MutationLeafSourceScope::validate` admits only one
   safe normalized portable FILENAME, and the leaf's descriptor is on disk at
   `🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json`. Both expectations are now `🔣️.json`.

No production (non-`#[cfg(test)]`) code was changed in any crate, so no `--target wasm32-wasip2` check was
required; nothing under `🧰️framework` was touched.

## Files changed (absolute)

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🧪️tests/🔬️surface/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛠️workshop/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (adds the `#[cfg(test)]` publication lane only)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏔️exaggeration/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🧬️direct-leaves/🦀️.rs`

## Open, with the exact reason and the next step

- **`semio-s-plugin-gis` · `descriptor_is_fresh`** — the committed `✏️s/🔌️plugins/🌍️gis/🛂️.descriptor.semio` no
  longer equals what the plugin's own `describe_plugin()` produces (the two byte vectors differ from their
  first length prefix on). Next step: `bun nx run @semio-tech/gis-plugin:describe` and commit the refreshed
  `🛂️.descriptor.semio` + `🔣️.json`. That target builds the wasm component, so it MUST go through the fleet
  wasm mutex (`📜️wasm-build-mutex.sh`), and it should run AFTER the peers' `LocalizedLabel` sweep settles —
  a label change anywhere in gis re-stales the descriptor. Not run here (the mutex holder was busy and every
  build slot was serialized behind the fleet).
- **`semio-s-artifact-fem-2d` · two wall-clock laws** —
  `analyses::tests::assembly_job_one_fuel_steps_stay_below_eight_milliseconds` (42 096 µs against an 8 ms
  ceiling) and `mesh::tests::mesh_job_large_boundary_never_runs_to_completion_in_one_step` (9.70 ms against
  8 ms), both in `✏️s/🔨️modules/🏗️fem/⚙️engine`. Measured at load average 112 with 13 concurrent fleet cargos;
  these are the known jitter-prone per-step laws. Next step: re-measure alone
  (`cargo test -p semio-s-artifact-fem-2d --features component-app-assembly --lib -- --test-threads=1
  assembly_job_one_fuel mesh_job_large_boundary`) on a quiet machine, and only if they still overshoot, probe
  per-step `(elapsed, step, phase)` and fix the hot step. Not re-measured here.
- **`semio-s-artifact-fem-3d` · `result_animation_frame_cost_stays_flat_across_a_long_run`** — the 1 200-frame
  run dies with `presence local read registry is busy or exhausted`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4578`): `PresenceStore::local_read` issues a lease per
  tick out of a fixed `SnapshotReadLeaseRegistry`, and the test's one fair `maintenance_step` per frame does
  not free them fast enough. Next step: decide whether the tick should take a presence read at all (it reads
  no peer presence) or whether `maintenance_local_reads_step` must drain more than one returned lease per
  turn — a `🏪️store` change, outside the peer no-touch list but wide-reaching, so it wants its own run of
  that crate's tests plus two dependents.
- **`semio-s-artifact-gis-gismap` · `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`**
  — the live maintenance turn faults `plugin.internal: candidate parent child projection is invalid`. This is
  the composed-child shape (`child_id` must equal the target id and be unique); the same class of drift that
  process3d's `every_example_fixture_carries_its_canonical_child_handles` guards. Next step: compare the
  fixture envelope `gis_map_envelope_wire()` mints against what genesis re-derives today, and regenerate the
  fixture with the crate's own printer if the code is right.
- **`semio-s-artifact-gis-gisterrain` · two strictness rows** —
  `strict_snapshot_and_aggregate_json_vectors` (`GisTerrainWindowConfig accepted {"cameraJson":"{}"}`) and
  `sparse_diff_vectors_and_ordered_absorption` (`GisTerrainWindowConfigDiff accepted {"steps":[{}]}`), both
  against `🎚️config/🧫️fixtures/🧬️direct-leaves/🔣️.json`. I did NOT change these: the fixture's `config.invalid`
  list is `[{}, {"cameraJson":"{}"}, {}, {"cameraJson":null}, {"cameraJson":"{}"}, {"cameraJson":7},
  {"cameraJson":"{}","selectedIds":[]}]` — it repeats `{}` and `{"cameraJson":"{}"}`, while `config.valid`
  accepts `{"cameraJson":""}`. `camera_json` is a plain `String`, so `"{}"` is well-typed and the rows read
  like `{"cameraJson": {}}` (an object where a string is required) mistyped during a fixture regeneration.
  Next step: confirm that reading with the crate's own vector generator and regenerate, rather than loosening
  either side by hand.
- **`semio-s-artifact-process-process3d` · the rest of the 44** — beyond the six root causes above, these were
  still red at first run and are not individually addressed:
  `export_brep_out_returns_step_text_structured_payload` (`unknown process export format kind 'step'` after
  the test registers the stdio STEP descriptors — the global format catalog is shared by the whole test
  binary, suspect the same cross-test interference the publication lane had),
  `every_example_loads_through_the_member_less_archive_door` (drilled-plate:
  `document-archive-replacement.initializer-failed`),
  `vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` ("production envelope load
  did not reach terminal"). Several of the remaining ones (`world_pointer_down_resets_active_utility_to_select`,
  `registry_backed_example_action_emits_the_requested_document`, `arg_form_set_stock_emits_ops_reading_kind_arg`,
  `undo_after_add_*`) should be carried by the settle fixes, but that is a prediction — the re-run is what
  decides. Next step: read `verify2-noassembly.txt` when it lands and work the residue.

## Proposed framework diff (peer no-touch file — NOT applied)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `artifact_app_laws::assert_viewer_never_mutates`
(around line 8018) currently does `let (artifact_is_empty, draft_is_empty) = result.expect("viewer adapter
command succeeds");`. A viewer whose only verb is `InteractiveJobClassification::Migrated` cannot serve that
verb at the stateless seam at all: `ViewEmit` has no window-config lane, so the correct production behaviour
is a loud refusal. Suggested shape — keep the law, admit the refusal:

```rust
match result {
    // 👁️ A refusal emits nothing at all, which is strictly stronger than "emitted no store mutation" —
    // a viewer whose verbs are all retained (window-config writes `ViewEmit` cannot carry) serves none
    // of them here by design.
    Err(_) => {}
    Ok((artifact_is_empty, draft_is_empty)) => {
        assert!(artifact_is_empty, "a viewer must never emit document mutations");
        assert!(draft_is_empty, "a viewer must never emit draft mutations");
    }
}
```
