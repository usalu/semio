# Fix Edit Window Tests (flow + preview) — 2026-09-05

Scope: the two edit-mode window files only.

- A: `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`
- B: `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`

## What was wrong

Both files' `#[cfg(test)] mod tests` called the async testkit
(`crate::editor::generation3d::testkit::{app, render, drain_flow_eval_ticks}`) from plain
`#[test] fn`, with no `.await` — the shared test target does not compile.

## File A — 🕸️flow

- `renders_node_graph_scene`
  - before: `#[test] fn renders_node_graph_scene() { let mut app = app(); assert!(render_body(&mut app, ...).contains(...)); }`
  - after: `#[semio_framework_async_macros::async_test] async fn renders_node_graph_scene() { let mut app = app_with_registry().await; assert!(render_body(&mut app, ...).await.contains(...)); }`
- `main_graph_scene_exports_flow_backed_node_graph_fields`
  - before: plain `#[test] fn`, `app()`, `render_body(&mut app, ...)` (no await).
  - after: `#[semio_framework_async_macros::async_test] async fn`, `app_with_registry().await`, `render_body(&mut app, ...).await`. All existing assertions (fixtureJson contains "flow.fixture", an operator id containing "math.add"/"brep.", capabilitiesJson contains "flow") preserved verbatim.
- testkit import switched from `{app, render as render_body}` to `{app_with_registry, render as render_body}`.

## File B — 👁️preview

- `renders_world_preview_scene`
  - before: plain `#[test] fn`, `app()`, `drain_flow_eval_ticks(&mut app)` and `render_body(&mut app, ...)` called synchronously.
  - after: `#[semio_framework_async_macros::async_test] async fn`, `app_with_registry().await`, `drain_flow_eval_ticks(&mut app).await`, `render_body(&mut app, ...).await`. The regression-guard assertions are untouched: `assert_ne!(meshes_json, "[]", ...)` and `assert_ne!(instances_json, "[]", ...)` for the boot `hexagonal-mushroom-column` fixture.
- New test `switching_active_example_changes_preview_meshes`:
  - Boots `app_with_registry()`, drains the initial `flowEvalTick` chain, renders the preview body and captures `meshesJson` for the boot example.
  - Dispatches `Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_FILLET.into() })` via `testkit::dispatch` (real dispatch/classification path, not a raw mutation).
  - Drains `flowEvalTick` again (required — `preview_eval_text` is only written by that chain, per `⏱️flow-eval-tick/🦀️.rs`'s `SetPreviewEval` config mutation).
  - Re-renders, asserts `meshesJson`/`instancesJson` are non-empty for the new example (`box-fillet-preview`) and that `meshesJson` differs from the boot-example capture.
- Imports added: `crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET`, `crate::editor::generation3d::commands::set_active_example`, `crate::editor::generation3d::Generation3dCommand`, and `dispatch`/`drain_flow_eval_ticks` added to the existing `testkit::{...}` import.

## Registryless → registry-backed

All three converted tests, plus the new one, use `app_with_registry()` instead of `app()`, per the
ticket's instruction: this plugin declares `bounded_first_step_tool_proofs!` with an app-owned
`Generation3dBoundedCommandJobFactory`, so only the registry-backed app exercises real dispatch
classification (the registryless app would keep passing even if every action were rejected at runtime
with `interactive-job.missing-owned-reducer`).

## Drop/close-drain question (step 3 of the brief)

I checked gen3d's `//#region 🧪️Testkit` (editor `🦀️.rs`, ~line 1911) and the whole file for any
`Drop`-based close-drain wrapper analogous to `process3d`'s `Process3dApp` newtype (`🏭️process/…/process3d/…/✏️editor/🦀️.rs`, `//#region 🧪️Testkit`, ~line 2090, which pumps `close_step` in `Drop`).
**gen3d's testkit has no such wrapper** — `Generation3dApp` is a bare
`VcsArtifactApp<EditorApp<Generation3dPlayApp>>` type alias, and every other `app_with_registry()`
call site already in this same file (`registry_backed_editor_installs_every_declared_bounded_command_proof`,
`generation3d_interaction_selection_owns_its_persisted_history`,
`context_menu_grouped_disclosure_stays_within_budget`, and `set-active-example`'s own
`set_active_example_via_string_action_loads_fixture`) uses `app_with_registry()` directly with no
drop/close-drain guard. Per the brief's instruction I did **not** invent one in files A/B — I followed
the existing precedent used everywhere else in this crate.

## Things I was not fully certain of (static review only, no `cargo` run)

- `PROCEDURAL_EXAMPLE_BOX_FILLET` ("box-fillet-preview") tessellates to a mesh signature different from
  the boot `hexagonal-mushroom-column` example — inferred from
  `each_example_loads_distinct_fixture_and_preview_geometry` (same file, ~line 2304) which asserts all 8
  bundled examples produce distinct widget-id signatures; I did not execute the flow/BRep pipeline to
  confirm the *serialized mesh JSON* (not just the widget signature) differs, since `cargo` is off-limits
  here. If it turns out two examples' tessellated JSON happens to coincide byte-for-byte, the coordinator
  should tell me and I'll swap `PROCEDURAL_EXAMPLE_BOX_FILLET` for a different constant from the same list
  (`🧬️schema/🦀️.rs` lines 275-282).
- Whether holding `test_support::lock()`'s `MutexGuard` across `.await` points compiles/is sound here —
  I did not invent this pattern; it is already used pervasively by every existing async test in the same
  file (e.g. `declared_actions_bridge_to_commands`, `registry_backed_editor_installs_every_declared_bounded_command_proof`), so I copied it verbatim rather than re-deriving it.

## Files touched

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`

No other files were touched. No `cargo` was run (per constraint) — this is a static, type-matched review only; the coordinator owns compile verification.
