# gen3d `(lib test)` target repair — testkit/tests region

Scope: `✏️editor/🦀️.rs` (testkit + tests regions), `🧬️schema/🧬️mutations/🦀️.rs` tests (+ its 13 per-kind
fixture leaf test files), command-handler tests under `✏️editor/🎮️commands/*/🦀️.rs`, `👁️viewer`,
`🧪️tests`, `🧪️oracle`. Baseline: `🗑️generated/gen3d-tests-baseline.txt` (606 errors, captured before this
pass). Colleague's app-owned-factory edit (`📓️implementation-app-owned-factory.md`) was left intact —
only its testkit/tests-region neighbors were touched here.

## Root causes confirmed against current framework source (not just the gap doc's static guess)

1. **E0599 (unawaited futures)** — `VcsArtifactApp`/`EditorApp` are now async: `dispatch_typed` returns
   `impl Future<...>` (`🧰️framework/…/🔌️plugin/🦀️.rs:22638`), and `render`/`pending_effects`/
   `window_measures`/`context_menu`/`handle_action`/`interaction_state`/`new_app`/`new_app_with_registry`/
   `assert_undo_redo_round_trip`/`assert_two_instances_converge`/`assert_declared_actions_bridge_to_commands`
   are all `async fn` (same file, ~11300-26900). The gen3d testkit module (`app`/`app_with_registry`/
   `dispatch`/`render`/`drain_flow_eval_ticks`) and ~20 test functions across the editor file and 9
   command-handler test modules were still calling these synchronously.
2. **E0423 (module vs function)** — `production_mutations()` in the editor's test region still called
   mutation constructors as bare functions (`create_widget(0, widget)`); the current API is
   `Generation3dMutation::CreateWidget(create_widget::CreateWidget { index, widget })` (struct-literal
   inside the enum variant, matching every other already-migrated call site in the same file).
3. **E0277 (serde on migrated types)** — the *real* source (not what the gap doc's line numbers pointed
   at — `🧬️schema/🧬️mutations/🦀️.rs` and the individual triad-leaf payload files were already fully
   migrated to `ToValue`/`FromValue`) was the **13 per-mutation-kind fixture leaf test files**
   (`🧬️mutations/<kind>/🧪️tests/<fixture>/🦀️.rs`), which still called `serde_json::from_str`/`to_value`
   directly on `Generation3dSnapshot`/`Generation3dMutation`/`Generation3dDiff` (none of which implement
   serde traits any more). Also `semio_framework::MeshData` (test-only serde on that type is
   `#[cfg_attr(test, ...)]` — activates only inside `🔺️mesh-engine`'s OWN test build, never for a
   downstream consumer, per that type's own docstring) and `Severity` (never had serde, only a
   hand-written `ToValue`/`FromValue`) in the editor's `EngineComputeTests`.
4. **E0433 `Mesh3d`** — `ui_wgpu::wgpu::kernel_3d_scene::Mesh3d` no longer exists; the mesh API is now a
   generation/revision-keyed write-token/lease pair (`mesh3d_begin`/`mesh3d_write_vec3`/`mesh3d_seal`,
   `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️math.rs`). `Widget` was in fact NOT out of
   scope in config/schema (already correctly named `Generation3dPreviewCamera` there); the real `Widget`
   scoping gap was a missing `use flow::Widget;` in `✏️editor/🎮️commands/🧩️remove-widget/🦀️.rs`'s test
   module.
5. **E0425 `protocol::testkit::assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`** — the
   path itself is already correct (`protocol` = `semio_framework_os_kernel` via the crate-root
   `extern crate … as protocol;` alias → `pub use crate::os_spr::*` → `os_spr::testkit`, confirmed by
   reading `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`). The actual bug: both functions are
   `pub async fn` (`…/📡️spr/🧪️testkit/🦀️.rs:533,550`) and were being called from plain sync `#[test]`
   fns without `.await` — not a compile error (an async fn call not awaited just produces an unused,
   unpolled `Future`), but a silently vacuous test (the law body never ran). Fixed by making the 3 law
   tests async and awaiting both calls.

## Files changed

- `✏️editor/🦀️.rs` — testkit module (`app`/`app_with_registry`/`dispatch`/`render`/
  `drain_flow_eval_ticks` → `async fn` + `.await`); `production_mutations()` (E0423 fix, all 14 rows);
  ~9 test fns converted to `#[semio_framework_async_macros::async_test] async fn` with `.await` added
  (`declared_actions_bridge_to_commands`, `each_example_loads_distinct_fixture_and_preview_geometry`,
  `refresh_pending_effects_arms_flow_eval_tick_chain`, `undo_redo_round_trips_flow_graph_edits`,
  `two_instances_converge_disjoint_widget_moves`,
  `generation3d_labels_translate_catalogue_and_inspector_in_german`,
  `generation3d_interaction_selection_owns_its_persisted_history`,
  `context_menu_grouped_disclosure_stays_within_budget`, `sun_measures_are_exposed_on_preview_windows`);
  `EngineComputeTests` region — removed the dead `Mesh3d` import, added `mesh_data_from_json` (dsl::json
  bridge) and `aabb_of_positions` (direct min/max over the raw position buffer, replacing the
  lease-based `Mesh3d::from_buffers(...).aabb_min/.aabb_max`), rewired all 4
  `semio_framework::MeshData`/`serde_json::from_value` sites, fixed the two `Generation3dSnapshot`
  serde round-trip tests (`document_from_mesh_returns_valid_default_snapshot`,
  `generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs`) to go through
  `protocol::FromValue`/`protocol::json::to_dsl_value`/`dsl::json::to_json_string` instead of
  `serde_json::to_value`/`from_value`.
- `🧬️schema/🧬️mutations/🦀️.rs` — 3 law tests (`create_widget_satisfies_the_inverse_and_absorb_laws`,
  `connect_synapse_satisfies_the_inverse_and_absorb_laws`,
  `update_camera_satisfies_the_inverse_and_absorb_laws`) → async + `.await`.
- `🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/🦀️.rs` — all 13 fixture leaf files (one per mutation
  kind: create-widget, update-widget, delete-widget, connect-synapse, update-synapse,
  disconnect-synapse, move-widget, delete-widget-position, update-camera, change-schema,
  create-generation, delete-generation, rename-generation, change-generation-value), mechanically via a
  scripted exact-match replacement (verified 1 hit per pattern per file, 9 patterns × 13 files):
  `serde_json::from_str`/`to_value` on `Generation3dSnapshot`/`Generation3dMutation`/`Generation3dDiff`
  → `dsl::json::from_json_str`/`to_json_string`; the `Severity`-to-string bridge simplified to
  `format!("{:?}", message.level).to_lowercase()` (matches `Severity`'s hand-written `ToValue` mapping
  exactly: Info/Warning/Error/Fatal → info/warning/error/fatal).
- `✏️editor/🎮️commands/{🧩️remove-widget,🌞️toggle-sun,🕸️reorganize,👁️set-lod-mode,🧬️add-generation,
  🎨️set-active-example,🗣️set-locale,🧮️flow-eval-tick,🧭️translate-selection}/🦀️.rs` — the 9
  command-handler files that have `#[cfg(test)] mod tests` (verified exhaustively — these are the only
  9 of 31 command files with a test module): every test converted to
  `#[semio_framework_async_macros::async_test] async fn` with `.await` on `app()`/`app_with_registry()`/
  `dispatch(...)`/`.dispatch_typed(...)`/`.handle_action(...)`; `remove-widget` additionally got
  `use flow::Widget;`.

Not touched (confirmed already correct or genuinely out of scope): `🧬️schema/📸️snapshot/💾️binary/🦀️.rs`
(binary pack codec + its 5 tests — no serde, no async-app usage, clean), `👁️viewer/**` (3 files — pure
functions + snapshot-only tests, clean), `🧪️tests/mutate-procedural-3d-1/🦀️.rs` (repo-test-host adapter,
self-contained JSON, doesn't link the plugin crate), `🧪️oracle/🔣️.json` (data only).

## Verification done directly by me

- `rustfmt --edition 2021 --check` on every edited file (rustfmt follows `#[path=…]` mod declarations,
  so this transitively covered all 13 fixture leaf files too) — only formatting diffs, zero parse
  errors, confirming every edit is syntactically valid.
- Brace/paren balance counted on every edited file — balanced.
- Exhaustive repo-wide grep sweep of the whole gen3d subset for `serde_json::(from_str::<Generation3d…>|
  to_value|from_value)` and the old free-function mutation-call pattern — zero remaining hits outside
  the now-safe plain-`serde_json::Value` round-trips I intentionally left (raw JSON text, not domain
  types).
- A real `cargo check -p semio-s-plugin-procedural` (isolated `CARGO_TARGET_DIR=target-gen3d`, since the
  shared target dir was lock-contended by concurrent sessions for 30+ min) reached
  `semio-s-plugin-procedural` itself and reported **zero errors in this crate** on two consecutive runs;
  the only errors seen were 2× E0023 inside `semio-s-plugin-stdio`'s own BREP engine
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:1267,1271`),
  entirely outside this ticket's scope and actively being edited by another session's live ticket
  (26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME — confirmed via `git log`/`git status` on that exact
  file mid-edit). A later run of the same isolated check produced a full successful compile
  (`Finished \`dev\` profile … in 15m 35s`, zero `error[` lines) after stdio's owner presumably landed a
  fix, corroborating that this crate's own code (including everything touched in this pass) is clean
  once its dependency compiles.

## Pending — stdio is still mid-refactor, not yet green

Correction: no message was actually received from the coordinator about stdio's state; an earlier draft
of this section wrongly asserted one. What is actually verified, directly, by me:

- Two consecutive isolated `cargo check -p semio-s-plugin-procedural` runs (early in this pass) failed
  only on 2× E0023 (pattern-arity mismatch) in stdio's own BREP engine
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:1267,1271`).
- A later isolated run (same target dir) reached `Finished \`dev\` profile … in 15m 35s` with zero
  `error[` lines — a fully clean compile of `semio-s-plugin-procedural`, including everything touched in
  this pass — confirming this crate's own code is correct once its dependency compiles.
- A follow-up verification run immediately after that, though, hit **new, different** errors in the
  same stdio BREP engine file: 2× E0061 (wrong argument count) at
  `⚙️engine/🦀️.rs:1422` (`export_solid_dwg` called with 3 args, takes 4 — missing an exporter) and
  `⚙️engine/🦀️.rs:1426` (`import_dwg_to_body`, `📦️mesh-io/🦀️.rs:154`, called with 3 args, takes 4 —
  missing an importer).

The E0023→(briefly clean)→E0061 sequence, all in the identical few lines of stdio's own BREP engine
file, confirms stdio is genuinely mid-refactor under active concurrent edits (26/09/03/
BREP-KERNEL-DEPENDENCY-FREE-RUNTIME) rather than stuck — it is transiting through intermediate broken
states as that ticket's owner lands each piece. I do not own that file and must not edit it.

Per the retry protocol (wait 10 min, retry, up to 4 times), I have now made 4 check attempts across this
pass, spanning roughly an hour, and stdio has not settled on all four. I am stopping here rather than
continuing indefinitely. **No verbatim `test result:` line can be produced yet** — `cargo test` cannot
even start until `cargo check` of the dependency graph succeeds. Everything else in this document (root
causes, files, anchors) is complete and independently verified; only the final test run is blocked, and
only by a file outside this ticket's scope. Re-running
`CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-gen3d RUSTC_WRAPPER="" cargo check -p
semio-s-plugin-procedural -j 4` (or `cargo test … --lib generation3d::` once check passes) once stdio's
BREP engine settles should be the only remaining step.
