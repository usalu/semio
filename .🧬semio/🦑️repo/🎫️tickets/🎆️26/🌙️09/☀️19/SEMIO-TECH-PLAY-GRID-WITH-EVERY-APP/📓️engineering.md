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

---

## 2026-09-21 22:15 — session 2 (after a usage-limit kill): measured results

The tree was uncompilable twice today from peers' in-flight framework rewrites (17:30–20:20:
`🔌️plugin/🦀️.rs` calling `ArtifactStore::{send_member_mutations, take_member_inbound}` that no longer exist;
from 22:13: `semio-framework-os-kernel` with 43 unresolved imports). Every number below is from a run that
actually completed between those windows.

| crate | first run | now | log |
| --- | --- | --- | --- |
| `semio-s-artifact-sourcing-curation` | fail 151 / 1 | **ok 152 / 0** | `retry2-noassembly.txt` |
| `semio-s-plugin-energy` | fail 3 / 1 | **ok 4 / 0** | `retry2-noassembly.txt` |
| `semio-s-artifact-fem-2d` | fail 1258 / 3 | fail **1259 / 2** | `retry2-assembly.txt` |
| `semio-s-artifact-gis-gisterrain` | fail 88 / 4 | fail **91 / 1** | `retry2-assembly.txt` |
| `semio-s-artifact-process-process3d` | fail 315 / 44 | fail **351 / 8** | `retry3-process3d.txt` |

Unchanged since their first run (no edits, no re-run): `semio-s-plugin-fem` ok 5/0,
`semio-s-artifact-energy-model` ok 6292/0, `semio-s-plugin-process{,-wood,-concrete,-metal,-robotic}` ok,
`semio-s-plugin-sourcing{,-beams,-slabs,-windows}` ok, `semio-s-artifact-fem-3d` 1130/1,
`semio-s-plugin-gis` 5/1, `semio-s-artifact-gis-gismap` 262/1.

### Root causes found and fixed in this session

10. **The config lane declared half its fold footprint** (process3d, 32 failures — the whole cascade the
    unsettled harness had been hiding). `admit_process3d_config_mutation` returned `work_items: 1` while
    `prepare_process3d_config` ALWAYS emits one inverse row beside the forward one. `work_items` counts staged
    edit ROWS, so every config gesture folds 2 and each was refused
    `batched item candidate failed its exact fixed fold contract`. Now
    `ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes)`. This is the one PRODUCTION
    (non-test) change of the session; it is target-independent, and the wasm mutex was held by a multi-hour
    peer build, so `--target wasm32-wasip2` was not run (coordinator's instruction).
11. **The gisterrain neutral vectors contradicted their own schema** (2 failures). Repaired from evidence, not
    taste: `🎚️config/🧬️schema/🔣️.json` types `cameraJson` as a plain string; the independent Ajv oracle
    `🧫️fixtures/🔬️window-config-ownership/🔣️.json` lists `{"cameraJson":"{}"}` under `accepted`; and the
    direct-leaves file itself already lists that row under `payloads.camera.valid`. So its two
    `config.invalid` rows were the wrong-type case `{"cameraJson": {}}` mistyped into a string — repaired,
    with the same repair on the `mutations.invalid` twin — and `{"steps":[{}]}`, which appeared in BOTH
    `diff.valid` and `diff.invalid` and is driven by the file's own `missing-null-identity` law, was removed
    from `invalid`. There is no generator for this fixture anywhere in the repo; it is hand-authored.
12. **A stale contributions law** (process3d). `host_contributions_resolve_to_the_event_sourced_config_lane`
    asserted the host's pack verbatim, but the lane DISTILLS (`installable_contributions` keeps only
    `process.machines` entries addressed to this app — the same discipline `🪵️sourcing` applies). Rewritten
    over a real addressed roster plus a foreign entry that must be dropped, which proves more than before.
13. **A refusal masked by a moved owner** (process3d). `Process3dArtifactPreparation::advance` moves the
    mutation into `prepare_process3d_document` before it can refuse, so the next turn reported
    `lost its mutation owner` instead of the real reason. The refusal is now captured and re-reported.
14. The publication lane (fix 4) also had to be taken by the two envelope laws in the editor suite.

### Files changed in this session (absolute; in addition to session 1's list)

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧫️fixtures/🧬️direct-leaves/🔣️.json`

### Still open, with the exact reason and next step

- **fem-2d, 2 reds** — `assembly_job_one_fuel_steps_stay_below_eight_milliseconds` and
  `mesh_job_large_boundary_never_runs_to_completion_in_one_step` measured 191 605 µs and 152 ms at load 70
  (they read 42 ms / 9.7 ms at load 112 this morning, and the ceiling is 8 ms). That spread IS the answer:
  the laws measure host scheduling, not the guest step. `chain6.sh` is queued to re-measure them alone with
  `--test-threads=1` once `uptime` reports load < 40; read `fem-timing-solo.txt`.
- **gisterrain, 1 red** — `parse_op(&format!("{line} unknown-field 1"))` returns `Ok`. The shared DSL text
  parser does not deny unknown trailing fields on the INLINE variant spec (`#[value(deny_unknown_fields)]`
  governs `FromValue`, not `dsl::parse`); the snapshot law one line above passes only because `cameraJson` is
  required. This assertion is the only one of its kind in the whole plugin tree. Next step: decide in the
  `🗣️dsl` module whether an inline record spec must reject unknown keys, then run that module's tests and two
  dependents — too wide to land here.
- **process3d, 8 reds** — 3 × `process3d-publication.saturated` (the process-global FOUR-slot direct-mapped
  lease table is churned by every test that triggers a document replacement, so a lane taken only by the
  fixture laws is not enough; next step: key the registry per app instance, or take the lane in every test
  that loads a document); `export_brep_out…` (cross-plugin, below); `host_contributions…`,
  `repeated_world_pointer_down…` (both fixed, awaiting the re-run in `chain7`/`chain8`);
  `vcs_artifact_app_production_maintenance_swap…` "real maintenance replay must publish the complete deep
  semantic state"; `selected_stock_id_renders_its_dimensions` (the inspector now renders a REAL selection —
  `ID: beam` — but not the expected `Width: 1`, so the seeded document differs from what the law assumes).
- **`export_brep_out_returns_step_text_structured_payload` is NOT test interference.** Every stdio artifact
  definition derives its format rows from `source_format_descriptors`, which sets
  `short_id = representation.id` and leaves `aliases` empty — I checked `📐️step` and `🔺️stl`, and
  `grep` finds no alias anywhere. The catalog therefore has no `step` / `obj` / `stl` / `glb` key at all,
  while `MeshExporter::format_kind` (documented as "the short stdio format kind id") returns exactly those.
  Next step (owner: the stdio topic): give each representation a short id derived from its primary extension
  in `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs`, or move the short ids into `aliases`.
- **fem-3d `presence local read registry is busy or exhausted`** — the 1 024-slot `SnapshotReadLeaseRegistry`
  runs out at ~frame 1 024 of a 1 200-frame animation. The lease is taken per dispatched typed command
  (`🔌️plugin/🦀️.rs:28887`) and only released when the job payload's close ladder reaches its
  `presence_local` stage — ~12 close items per payload against the ONE fair maintenance item per frame the
  test grants, which is the reactor's real budget. That is a retirement-debt leak with no back-pressure
  (`maintenance_under_pressure` stays false). Both the lease site and the ladder are in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, which is on the peer no-touch list — hand to the
  peer with these numbers.
- **gismap `candidate parent child projection is invalid`** — `ChildRestoreProjection::from_snapshot` refuses
  the candidate during the live envelope swap. The slot kinds are canonical (`s.stdio.semio` ×3) and the
  handles are `child_id == target.artifact_id` and unique (`gismap-drawing`, `gismap-value`, `image: None`),
  so the refusal comes from `visit_child_refs`, not the slot check. Next step: instrument
  `ChildRestoreProjectionError` at that call site to learn which row is refused — I could not get a build
  through to do it.
- **Verification still owed**: `chain7.sh`/`chain8.sh` are running detached and retry the process3d and
  gisterrain+fem-2d batches every 10 minutes until the tree compiles; their results land in
  `🗑️generated/engineering/retry4-process3d.txt` and `final<N>-{noassembly,assembly}.txt`.

---

## 2026-09-22 (session 6, successor after a coordinator restart)

The predecessor's detached retry chains (`chain6/7/8.sh`) were already DEAD on intake — nothing of theirs
had written since 2026-09-21 22:17 (`fem-timing-solo.txt` stops at `Blocking waiting for file lock`) and no
cargo of theirs was alive. So `retry4-process3d.txt` and `final<N>-*.txt` never existed; the last measured
numbers remain the ones in the section above.

### Root causes found and fixed in this session

15. **A returned snapshot read whose root is still LIVE was charged a full retirement** (fem-3d's
    `result_animation_frame_cost_stays_flat_across_a_long_run`, `presence local read registry is busy or
    exhausted`). `SnapshotReadLeaseRegistry` keeps a fixed 1 024-slot table; a returned lease is reclaimed
    by the maintenance pump at ONE lease per turn, and each reclaim costs three turns (take → the domain
    retirement's `Arc::into_inner` → drop the completed retirement). But the roots that dominate that
    traffic are not displaced at all: `🔌️plugin/🦀️.rs` takes `presence_store.local_read()` in the
    `ephemeral` hook of EVERY dispatched command and again for every bounded tool job, and returns them
    while `PresenceStore::local` still aliases the same `Arc`. The pump then spends its fair turn
    rediscovering that `Arc::into_inner` fails and simply drops the handle — which is precisely what
    `ReturnedSnapshotReadRetirement::close_step` and the presence local ladder already do for an aliased
    root. A mounted app that dispatches once per frame therefore issues leases faster than one fair
    maintenance step per frame can reclaim them and runs the table out mid-run.
    Fix (general, in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, which is not frozen):
    `SnapshotReadLeaseRegistry::try_release_aliased` — on the returning thread, under the registry's own
    state lock, a returned lease whose slot is not the LAST `Arc` (`Arc::strong_count > 1`) frees its slot
    immediately and never enters the returned queue. Semantically identical to what the pump discovers two
    turns later, O(1), and it leaves exactly the displaced roots (the ones `PresenceStore::apply` hands
    over, where the slot IS the last owner) to the bounded domain retirement. `SnapshotReadLease::return_now`
    records the one-shot `returned` flag on that path too, so a second return is still refused.
16. **The shared DSL had no exact op-line parser, and 20 hand-rolled copies of the same `OpText` body**
    (gisterrain's `parse_op(&format!("{line} unknown-field 1"))` returning `Ok`). `dsl::parse` stops at the
    end of the record it recognises and drops the rest — the DOCUMENT-mode contract. An operation line is
    ONE terminal record, which is what `dsl::parse_exact` is for (`📕️norm`'s config text codec already uses
    it, and its `📜️script.ts` even asserts "config text must use the shared exact record boundary"). Added
    `dsl::variants_text::{parse_op, print_op}` next to `variants_binary` — the text twin, on `parse_exact` —
    with a new law `derived_op_text_refuses_every_token_outside_its_own_record` in the dsl unit suite
    (the text twin of the trailing-byte refusal the binary law already proved), and replaced all 20
    hand-rolled bodies in 🏗️fem / 🔋️energy / 🌍️gis / 🏭️process / 🪵️sourcing with a delegation.
17. **process3d's app self-grant shared the host's four-slot DIRECT-MAPPED publication table**
    (`process3d-publication.saturated`, 3 reds). `FixedOperationRegistry::can_admit` requires the slot
    `index(key)` maps to to be free — a committed law (`🧪️fixed-operation-registry-cases`, `collision`)
    pins that direct-mapped refusal, so the registry is not the place to fix this. But a lease the app
    grants ITSELF for a host-begun `Effect::LoadDocument` is not a host publication: it is admitted by
    `Process3dStoreInitializationAuthority::new`, at most one exists at a time (the old code already drained
    every previously recorded app key), and in a test binary hosting several app instances beside the
    fixture laws it is exactly what occupies the slot an unrelated fixture key hashes to while three slots
    stay free. It now lives in its own single-slot authority (`process3d_app_publication_lease`), with
    `process3d_publication_lease_by_key`/`_by_operation` consulting the host table first and the self-grant
    second; the now-meaningless `app_admitted` discriminator is gone.
18. **gismap's live-load refusal is unattributable from the test** — the framework reports only
    `plugin.internal: candidate parent child projection is invalid`, swallowing the
    `ChildRestoreProjectionError` with `map_err(|_| …)`. Added
    `every_gis_map_parent_snapshot_projects_its_canonical_child_handles` to the gismap editor suite: it runs
    `ChildRestoreProjection::from_snapshot` over `empty_gis_map_snapshot()` and `default_document()` and over
    each one's pack and DSL round trip, and names the exact error variant plus the offending handles.

### Files changed in this session (absolute)

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🔬️unit/🦀️.rs`
- the 20 `OpText` codecs listed by `🗑️generated/engineering/adopt-variants-text.py` (fem 2d/3d ×7,
  energy ×5, gis gismap/gisterrain ×4, process3d ×2, sourcing ×2)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

19. **`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` located its
    machine POSITIONALLY** (process3d), and it is the head of the publication cascade. Every workshop
    mutation addresses a machine by ID; the expectation edited `expected.workshop.machines.first_mut()`,
    but the initial workshop already carries the three generic machines (saw, drill, attacher) before the
    one `production_initial_snapshot` appends — so the expectation renamed `saw` while the real id-keyed
    replay renamed `machine`. The expectation now finds the machine the mutations name and reads label,
    icon, stock label, stock solid and cursor off the mutations themselves. This law panics BEFORE
    `owned_store_measured` releases its host publication leases, and those leaks are what the three
    `process3d-publication.saturated` reds then collide with: the lane-holding laws are downstream, not
    independent defects.
20. **`next_step_id()` hashed a compile-time constant** (process3d — the real defect behind
    `repeated_world_pointer_down_each_dispatch_a_mutation`, which the predecessor's refusal-reporting fix
    finally made legible: `A step with id "step-545333ff4364" already exists`).
    `format!("step-{}", &hash_bytes(concat!(file!(), line!(), "step-{}").as_bytes())[..12])` — the hashed
    bytes are a `&'static str` fixed at compile time, so every call in the life of the binary returned the
    SAME id and the second step inserted into any document was refused by the vocabulary, while the doc
    comment claimed "pseudo-random, collision odds astronomically low". It now derives the id from the
    timeline it is inserted into (`next_step_id(&Process3dSnapshot)`): a pure function of the document, so
    a replay mints the same id, salted over `0..=len` so that by pigeonhole one of `n + 1` distinct
    candidates is free of the `n` ids already present. Threaded through all three mint sites
    (`🪜️step::add_step`, `🌍️world::world_pointer_down`, `process3d_step_from_face_drag`). This is a
    PRODUCTION change; it is target-independent, and `cargo check -p semio-s-artifact-process-process3d
    --all-targets` is clean (exit 0, 125 warning lines), but `--target wasm32-wasip2` was not run — the
    fleet's wasm mutex is held by other topics and the brief forbids waiting on it.
21. **The inspector stock law asserted a literal 1 × 1 × 1 box** (process3d). The curated default example
    is a timber beam (3 × 0.2 × 0.3); `selected_stock_id_renders_its_dimensions` now reads the expected
    extents off the same `stock_payload.solid` the panel renders, so it proves the same property and
    cannot drift with the example.

### Measured this session (every number from a run made here)

| crate | before (2026-09-21) | this session | log |
| --- | --- | --- | --- |
| `semio-framework-os-kernel` (store + dsl host) | not run | first **1116 / 2** (both from the store change), then **ok 1118 / 0** | `s6-kernel.txt`, `s6d-kernel.txt` |
| `semio-s-artifact-energy-model` | ok 6292 / 0 | **ok 6292 / 0** | `s6-noassembly.txt` |
| `semio-s-artifact-sourcing-curation` | ok 152 / 0 | **ok 152 / 0** | `s6-noassembly.txt` |
| `semio-s-plugin-energy` | ok 4 / 0 | **ok 4 / 0** | `s6-noassembly.txt` |
| `semio-s-plugin-gis` | fail 5 / 1 | fail **5 / 1** (`descriptor_is_fresh`, still owed the coordinator's `describe`) | `s6-noassembly.txt` |
| `semio-s-artifact-process-process3d` | fail 351 / 8 | fail **352 / 7** (`host_contributions…` confirmed green) | `s6-noassembly.txt` |
| gis native-codecs target | ok 3 / 0 | **ok 3 / 0** | `s6-noassembly.txt` |

`cargo check --all-targets` clean (exit 0, real warnings) for `semio-framework-os-kernel` (`s6-check-kernel.txt`),
fem-2d/fem-3d/gismap/gisterrain under `--features component-app-assembly` (`s6-check-artifacts.txt`),
process3d/sourcing-curation/energy-model (`s6-check-plain.txt`) and process3d again after fixes 19–21
(`s6c-check-process3d.txt`).

**Queued behind the play fleet's native cargo mutex** (`📜️native-test-mutex.sh`, adopted by the fleet at
03:28; `engineering` is 7th in line): `s6c-kernel` (the store crate's own tests), `s6c-assembly`
(fem-3d, gisterrain, gismap, fem-2d — four store dependents, full suites) and `s6c-process3d`.
Detached as `chain11.sh` / `chain12.sh`; both retry until a real `test result:` lands.

### Proposed diffs for peer-owned files (NOT applied)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23348` swallows the projection error:

```rust
let projection = store::ChildRestoreProjection::from_snapshot(candidate.snapshot_ref()).map_err(|_| plugin_sdk_fault("candidate parent child projection is invalid"))?;
```

Two changes worth making together: report the `ChildRestoreProjectionError` in the message, and route
through `A::child_restore_projection`, which apps already implement precisely so their own diagnostic
wins (gismap's says `gis map child projection failed: {error}`). As it stands the gismap live-load red is
unattributable from outside the framework — hence the crate-side law added in fix 18.
