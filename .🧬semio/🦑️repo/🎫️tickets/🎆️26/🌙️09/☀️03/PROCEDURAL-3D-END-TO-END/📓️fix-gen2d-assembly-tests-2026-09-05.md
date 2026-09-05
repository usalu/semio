# Fix: generation2d missing-`.await` tests (class 1)

Scope: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/`. `🧩️assembly` was audited
(editor + every `🧬️mutations`/`💡️inferences` file, `grep`-checked for `.await`/`async fn`/`testkit::`)
and found already correct — every async test there already carries
`#[semio_framework_async_macros::async_test]` with proper `.await`s (e.g.
`mounted_semio_infer_routes_exact_assembly_job_through_checkpoint_restart`). No changes made to assembly.

## Root cause

`generation2d`'s editor testkit (`✏️editor/🦀️.rs`, `//#region 🧪️Testkit`) declared `app()`,
`app_with_registry()`, `dispatch()`, `render()` as plain `pub fn` bodies calling the framework's
`async fn new_app`/`new_app_with_registry` (🧰️framework `🔨️modules/🔌️plugin/🦀️.rs:6757,6763`) and
`VcsArtifactApp::dispatch_typed`/`PluginApp::render` (both `impl Future`/`async fn`) without `.await`.
Every `#[test]` calling these helpers, or calling `dispatch_typed`/`import_media`/`context_menu`
directly, inherited the same defect. Fixed by making the four testkit helpers `pub async fn` (mirroring
the already-correct `🧊️generation3d` editor testkit) and converting every dependent test to
`#[semio_framework_async_macros::async_test] async fn` with `.await` added at each async call site.

## Files changed (13), tests converted (23)

1. `✏️editor/🦀️.rs` — testkit `app()`, `app_with_registry()`, `dispatch()`, `render()` → `pub async fn` +
   internal `.await`. Tests converted (9): `declared_actions_bridge_to_commands`,
   `add_widget_materializes_declared_kind_default_into_an_operation`, `add_widget_undo_redo_round_trip`,
   `two_instances_converge_disjoint_widget_moves`,
   `an_unknown_body_key_renders_a_diagnostic_instead_of_panicking`,
   `context_menu_stays_within_disclosure_budget`, `export_drawing_out_returns_vector_media`,
   `export_document_out_returns_flow_media`, `import_params_in_patches_matching_input_slider`.
2. `✏️editor/📌️panels/🗿️artifact/🦀️.rs` — `document_lists_widgets` (1).
3. `✏️editor/📌️panels/🔍️inspection/🦀️.rs` — `generation2d_labels_translate_catalogue_and_inspector_in_german` (1).
4. `✏️editor/📌️panels/🛍️catalogue/🦀️.rs` — `catalogue_lists_show_modes`,
   `generation2d_labels_resolve_native_english_by_default` (2).
5. `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs` — `generate_preview_hints_without_evaluated_output` (1).
6. `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs` — `generate_form_hints_without_a_selected_generation` (1).
7. `✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` — `generate_mode_renders_surfaces` (1).
8. `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` — `renders_preview_canvas_scene` (1).
9. `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` — `renders_main_graph_scene`,
   `main_graph_scene_exports_flow_backed_node_graph_fields` (2); `definition_declares_the_node_graph_surface_and_body_key`
   left `#[test]` (no async call).
10. `✏️editor/🎮️commands/🧩️add-widget/🦀️.rs` — `add_widget_emits_op_and_grows_document` (1).
11. `✏️editor/🎮️commands/🗺️reorganize/🦀️.rs` — `reorganize_emits_operations`, `node_graph_viewport_sets_camera` (2).
12. `✏️editor/🎮️commands/🗣️set-locale/🦀️.rs` — `set_locale_updates_config_locale` (1).
13. `✏️editor/🎮️commands/➕️add-generation/🦀️.rs` — `add_generation_records_an_undoable_generation_operation`,
    `generate_is_a_view_action_with_no_artifact_mutations` (2).
14. `✏️editor/🎮️commands/👁️set-show-mode/🦀️.rs` — `set_show_mode_is_config_only` (1).
15. `✏️editor/🎮️commands/📤️set-eval-outputs/🦀️.rs` — `set_eval_outputs_does_not_mutate_the_document` (1).

Total: 15 files touched, 23 tests converted from `#[test] fn` to
`#[semio_framework_async_macros::async_test] async fn`, all assertions preserved verbatim. The prior
survey's "~14 call sites" undercounted — the real count across generation2d's editor subtree is 23,
because every test transitively depending on the now-corrected `app()`/`app_with_registry()` testkit
helpers had to convert, not only the ones calling `dispatch`/`render` directly.

`semio-framework-async-macros` is already a `[dev-dependencies]` entry in
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:69` (used by the pre-existing correct
generation3d/assembly async tests), so no manifest change was needed.

## Verified NOT touched (checked, no defect)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🦀️.rs`, `🧬️schema/🦀️.rs`, `👁️viewer/🦀️.rs`,
  `🧬️schema/🧬️mutations/🦀️.rs` (29 `#[test]`s), `🧬️schema/💡️inferences/🦀️.rs`,
  `✏️editor/🗣️terminology/🦀️.rs`, `✏️editor/🎚️config/🦀️.rs`, `✏️editor/🌉️wasm/🦀️.rs`,
  `✏️editor/🎭️modes/🧬️generate/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🦀️.rs`,
  `📚️examples/🎬️demo/🧪️tests/🦀️.rs`, `✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️.rs`,
  `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` — plain sync tests, no `testkit::app`/async calls.
- Every per-mutation `🧪️tests/<case>/🦀️.rs` leaf under `🧬️schema/🧬️mutations/` (14 dirs, e.g.
  `🌱️create-widget`, `🔗️connect-synapse`, `🎛️set-camera`, …) — pure mutation-law tests against sync
  helpers only, no async involvement.
- `🧩️assembly` subset in its entirety — already uses the correct `async_test`/`.await` pattern.

## Noted but NOT fixed (out of class-1 scope)

- `✏️editor/🦀️.rs`, test `import_params_in_patches_matching_input_slider` (now async, line ~1250):
  `app.import_media("params:in", &media, &semio_framework_plugin::testkit::meta("local")).await…` passes
  `&media` (a `&Media`) but `PluginApp::import_media` (🧰️framework `🔨️modules/🔌️plugin/🦀️.rs:25464`)
  takes an owned `media: Media`. This is a pre-existing type mismatch unrelated to the missing-`.await`
  defect (adding `.await` does not change the argument type problem) — left as-is per the assigned
  class-1-only scope. Someone should drop the `&` (or confirm the framework signature) before this file
  will compile.
- No `serde_json`-vs-`ToValue`/`FromValue` blockers were found or touched anywhere in scope; the prior
  survey's "~180 serde_json blockers" claim was not reproduced — `serde_json` is used correctly
  throughout (fixture JSON, `serde_json::json!`, `to_string`/`from_str` round trips) and was left alone.
