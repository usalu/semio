---
name: Forms Premigration Parity Both Renderers
overview: Restore the `forms` plugin (plus its `flow`/`procedural 2d`/`procedural 3d` "Generate mode" siblings, which pre-migration shared forms-core) to full pre-migration feature parity, rendered identically by both the React and wgpu renderers via the shared `UiNode` protocol.
todos:
  - id: domain-model
    content: Expand FormQuestion/FormExpr/validation in forms/rs/lib.rs to full pre-migration field set + condition evaluation + runtime helpers
    status: completed
  - id: wgpu-nested-scene-fix
    content: Fix wgpu ComponentScene rendering to recurse at any nesting depth (interpreter.rs + widgets.rs), matching React parity
    status: completed
  - id: generic-widget-extensions
    content: Extend React Input inputKind handling (longText/date/color/file) and confirm wgpu text-edit path covers all kinds
    status: completed
  - id: forms-edit-restoration
    content: Restore full per-question-kind structural editing in forms Inspection panel (placeholder/min/max/step/unit/options/vector-fields/note/image/file/buildingComponent params)
    status: completed
  - id: forms-try-wizard
    content: "Replace forms Try table with a real multi-step wizard: per-kind controls, step nav, validation, conditional visibility"
    status: completed
  - id: building-component-preview
    content: Port procedural/3d's evaluate-tessellate pipeline into forms-plugin and add a dedicated live mesh Preview window for buildingComponent questions
    status: completed
  - id: forms-fixtures
    content: Restore default/Contact and onboarding example fixtures with full field coverage and a conditional step
    status: completed
  - id: generate-mode-shared
    content: Add flow_fixture_to_form_spec + apply_generation_values_to_fixture to flow_core and a shared generate-mode CRUD/render helper
    status: completed
  - id: generate-mode-wiring
    content: Wire Generate mode (list+form+preview) into flow, procedural/2d, procedural/3d plugins with mode-specific layout
    status: completed
  - id: verify-all
    content: Build native+wasm for all touched crates, run/extend E2E screenshot-diff for forms/flow/procedural2d/procedural3d, manual runtime pass with DEBUG logs, open/close repo ticket
    status: in_progress
isProject: false
---

## Audit summary

Compared pre-migration TypeScript (`git show 5ecbe3dbf^:forms/react/index.tsx`, `git show e1369ca57^:forms/core/js/{index,internal}.ts`, and ticket `.repo/🎫/26/06/30/FORMS-TECHNOLOGY-AND-GENERATE-MODE/`) against the current Rust/WASM [forms/rs/lib.rs](forms/rs/lib.rs) + [forms/plugin/rs/lib.rs](forms/plugin/rs/lib.rs). Findings:

- The domain model regressed hard: `FormQuestion` today only has `{id, label, kind, default, text, options}` — it silently drops `required`, `description`, `placeholder`, `min/max/step/unit`, vector `fields`, `schema`, `src`, `accept`, `fixtureSlug`/`params`, and `condition` on every parse. The already-handcrafted [forms/example/building-component.forms.json](forms/example/building-component.forms.json) fixture authors all of these fields today, but they're silently discarded.
- "Try" mode (`render_try_table` at [forms/plugin/rs/lib.rs:406](forms/plugin/rs/lib.rs)) is a label-only `TableScene` — there is no interactive form at all (no inputs, no step wizard, no validation, no conditional visibility).
- "Edit" mode is a flat table; the old per-question structural editor (placeholder/min/max/step/unit/options-editor/vector-fields-editor/note-text/image-src/file-accept editing) has no equivalent.
- `buildingComponent` questions carry `fixtureSlug`/`params` but there's no evaluation/tessellation/preview pipeline in Rust for forms (procedural/3d has one, forms doesn't reuse it).
- `FlowGenerateSurface` (named "generations" list + bound form + preview) was never part of forms itself pre-migration — it was a second `"generate"` app mode on `flow`, `procedural/2d`, and `procedural/3d` (reusing forms-core types). None of the three currently have a `generate` mode.
- Widget parity between React/wgpu is otherwise already done ([.cursor/plans/wgpu_full_widget_parity_f31a2811.plan.md](.cursor/plans/wgpu_full_widget_parity_f31a2811.plan.md)), **except**: wgpu's `UiNode::ComponentScene` only renders at a pane's root — nested inside a `Stack`/`Field`/`Section` it collapses to an empty text stub ([framework/renderer/wgpu/rs/interpreter.rs:127-130](framework/renderer/wgpu/rs/interpreter.rs)), while React supports arbitrary nesting depth. This blocks inline image/mesh previews unless fixed or avoided by design.
- No native file/color/date pickers exist in the wgpu widget set, and `UiInputNode.input_kind` today only distinguishes `text`/`number` in the React interpreter ([framework/renderer/react/ui-interpreter.tsx:97](framework/renderer/react/ui-interpreter.tsx)).

## Scope decisions (confirmed with dev)

- Restore the `buildingComponent` **live 3D mesh preview**, reusing procedural/3d's proven Rust evaluate→tessellate→`World3dScene` pipeline ([procedural/3d/plugin/rs/lib.rs:388-437](procedural/3d/plugin/rs/lib.rs)), not the old JS/WASM-kernel-bridge approach.
- Include restoring `Generate` mode on `flow`, `procedural/2d`, and `procedural/3d` in this same effort (touches those plugins, but forms-core is the shared type foundation, so this is one coherent piece of work, not "mixing technologies").
- `date`/`color`/`file` question kinds get functional-but-approximated wgpu controls (plain editable text field for date/color value, filename-text for file — no native OS pickers in wgpu); this is a documented, explicit limitation, not a silent gap.

## Phase 1 — Domain model (`forms/rs/lib.rs`)

Expand `FormQuestion` to carry every pre-migration field (`required`, `description`, `placeholder`, `min`, `max`, `step`, `unit`, `fields: Vec<FormVectorField>`, `schema`, `src`, `accept`, `fixture_slug`, `params: Value`, `condition: Option<FormExpr>`). Add:
- `FormVectorField { key, label, value }`.
- `FormExpr` enum (`Const`/`Var`/`Eq`/`And`/`Or`/`Truthy`) + `eval_form_expr(expr, values) -> Value` + `is_question_visible(question, values) -> bool`, porting `evalFormExpr`/`isQuestionVisible` from `/tmp/forms_internal_premigration.ts` (lines ~900-961).
- `default_value_for_question(question) -> Value`, `visible_questions(step, values)`, `step_errors(step, values) -> Vec<{question_id, message}>`, `can_advance(step, values) -> bool` — Rust port of `FormRuntime`'s validation (`getStepErrors`/`canAdvance`, same file lines 1004-1032): required + non-empty check, skipping `note`/`image`, treating `buildingComponent`-style extension questions as requiring a non-empty `params` object.
- Update `FormOp`/`apply_form_edit_op` only as needed for the richer struct (should be mostly pass-through).
- Extend the existing `#[cfg(test)]` module (no new test files) to cover round-tripping every field and condition evaluation.

## Phase 2 — wgpu nested `ComponentScene` fix (generic framework, root-cause)

Fix [framework/renderer/wgpu/rs/interpreter.rs:127-130](framework/renderer/wgpu/rs/interpreter.rs) and the `Stack`/`Section`/`Field` child-rendering path in [ui/wgpu/rs/widgets.rs:351-368](ui/wgpu/rs/widgets.rs) so a `ComponentScene` nested at any depth recurses into `render_ui_node`/`render_component_scene` (mirroring `scenes.rs:558-587`) instead of degrading to an empty text stub. This is what unblocks inline `image` question previews (and any future nested scene use) with true React parity.

## Phase 3 — Generic widget extensions

- `framework/renderer/react/ui-interpreter.tsx:92-119`: extend the `Input` control's `inputKind` handling to map `longText`→`Textarea`, `date`→`<Input type="date">`, `color`→`<Input type="color">`, `file`→`<Input type="file">` (dispatch filename via `onChange`).
- `ui/wgpu/rs/widgets.rs` / wgpu shell input handling: keep single-line text editing for all non-`number` kinds (already generic); only the numeric-parse branch needs to stay gated on `input_kind == "number"`. Document the date/color/file simplification inline as a known wgpu limitation (comment, not silent).
- No new `UiNode`/`UiControlNode` variants needed for `multi` (build via a `Stack` of `Toggle` controls, one per option, mirroring the old `FormMultiSelectControl`) or `vector` with arbitrary field counts (a `Stack` of `Field`+`NumberStepper` per configured field, not the fixed-3 `Vec3` control).
- `image` question rendering: decode `question.src` via the `image` crate (already a dependency of `raster-plugin`, add to `forms-plugin`) into RGBA pixels and emit a nested `build_raster_scene` (reuses the already-parity-proven Raster host in both renderers); fall back to a muted "No image" text node when `src` is empty/undecodable (e.g. SVG, which `image` doesn't decode — swap the onboarding fixture's avatar to a bundled raster asset when restoring that fixture).

## Phase 4 — `forms/plugin/rs/lib.rs`: Edit mode restoration

Keep the Edit window's overview table, but restore full per-question structural editing in the Inspection panel (`build_inspector_tree`, extending the existing pattern already used for `label`/`kind`/`required`) so it exposes, per selected question kind: `description`, `placeholder` (text/longText), `min`/`max`/`step`/`default` (number/slider) + `unit` (slider), boolean default, options editor with individual add/remove rows for `single`/`multi` (mirror the "S" plugin's restored per-option-row pattern, [s/plugin/rs/lib.rs:663-689](s/plugin/rs/lib.rs)) + a default-option select/multi-toggle, `date`/`color` default, vector `schema`/`step` + per-field add/remove editor, `note` text, `image` `src`, `file` `accept`, and `buildingComponent`'s `fixtureSlug` (readonly) + `params` (one numeric field per known param key). Extend `patchQuestions` (and add sibling commands only if a single patch shape can't cover options/vector-field add-remove) to cover every field.

## Phase 5 — `forms/plugin/rs/lib.rs`: Try mode wizard

Replace `render_try_table`/`try_table_rows` with a real multi-step wizard `UiNode` tree: title + "Step N / M" text, `visible_questions`-filtered `Field`s mapping each kind to its control (text→`Input`, longText→`Input(longText)`, number→`Input(number)`, slider→`Slider`, boolean→`Toggle`, single→`Select`, multi→`Stack` of `Toggle`s, date/color→`Input(date|color)`, vector→`Stack` of `Field`+`NumberStepper`, note→`Text`, image→nested raster scene, file→`Input(file)`, buildingComponent→params as `Field`+`NumberStepper` rows), plus Back/Next/Submit `Button`s gated by `can_advance`. Add `current_step_index` to `FormsPlayEnvelope` and commands `previousStep`/`nextStep`/`submit` (mirroring `FormRuntime.previousStep/nextStep/submit`), plus a generic `setTryValue` command replacing the current stubbed `tryEngagementInput`.

## Phase 6 — `buildingComponent` live mesh preview

- Add `flow_core`, `flow_module_brep`, `kernel_3d_brepkit`, `kernel_3d_engine` to [forms/plugin/rs/Cargo.toml](forms/plugin/rs/Cargo.toml) (mirrors [procedural/3d/plugin/rs/Cargo.toml](procedural/3d/plugin/rs/Cargo.toml)).
- Bundle `hexagonal-mushroom-column.procedural.json` via `include_str!` and a small `fixture_json_for_slug(slug) -> Option<&str>` lookup (local to forms-plugin; the one bundled slug today, extensible).
- Port `evaluated_preview_payload` ([procedural/3d/plugin/rs/lib.rs:388-437](procedural/3d/plugin/rs/lib.rs)): parse fixture → `FlowHost::from_fixture` → apply the question's `params` via `FlowHost::set_neuron_params` ([flow/core/rs/lib.rs:2244-2256](flow/core/rs/lib.rs)) → `evaluate()` → tessellate preview geometry handles → mesh/instance JSON.
- Add a third window (`FORMS_PLAY_WINDOW_PREVIEW`/"Preview") whose body renders `build_world_3d_scene` for the current form's first visible `buildingComponent` question (empty-state message when none), added to the default layout as a 3-way row split. A dedicated window (not inline-nested) is deliberate: matches procedural/3d's own dedicated preview window and gives the mesh real interactive/orbit space in both renderers.

## Phase 7 — Fixtures

Restore `forms/fixture`-equivalent `.example(...)` entries in `create_forms_app()`: a simple `default`/"Contact" example and the full `onboarding` example exercising every kind + a `condition`-gated step (per `git show 4eb2e6392^:forms/fixture/onboarding.forms.json`), swapping its avatar `src` for a bundled raster asset. Extend the existing test module to assert every kind round-trips and that conditional questions are hidden/shown correctly.

## Phase 8 — `Generate` mode (`flow`, `procedural/2d`, `procedural/3d`)

- `flow/core/rs/lib.rs`: add `flow_fixture_to_form_spec(fixture: &Widget-graph) -> forms::FormSpec` (map `InputSlider`→`slider`, `InputStepper`→`vector`, `InputNote`→`note`, `InputImage`→`image`, `Variable`→`text`/`single`, porting the humanized-label heuristic from `/tmp/forms_internal_premigration.ts:691-753`) and `apply_generation_values_to_fixture(fixture_json, values) -> String` (non-destructive widget-value patch). Add `forms` as a path dependency of `flow_core`.
- Add a small shared "generate mode" helper (new module in `semio-framework-plugin`, reused by all three plugins per the "use existing generic infra, don't triplicate" precedent from the S-parity restoration): generations `Vec<{id, name, values}>` CRUD (`addGeneration`/`removeGeneration`/`renameGeneration`/`selectGeneration`/`updateGenerationValues`) + tree-list rendering + form-body rendering from a `FormSpec`.
- Wire `.mode("generate", "Generate")` + 3 new `window_kind`s (Generations list / Form / Preview text or mesh) + a `named_layout` (mirroring [lowpoly/plugin/rs/lib.rs:2069-2092](lowpoly/plugin/rs/lib.rs)) into `flow/plugin/rs/lib.rs`, `procedural/2d/plugin/rs/lib.rs`, `procedural/3d/plugin/rs/lib.rs`. Preview pane: flow uses the existing eval-JSON-as-text path ([flow/plugin/rs/lib.rs:682-686](flow/plugin/rs/lib.rs)); procedural/2d/3d reuse their existing `evaluated_preview_payload`-style pipelines for a real canvas-2d/world-3d preview per generation.
- Persist `generations`/`selected_generation_id` on each plugin's document envelope.

## Phase 9 — Verification

- `cargo check`/`cargo test` (native) for `forms-plugin`, `flow-plugin`, `procedural2d-plugin`, `procedural3d-plugin`, `semio-framework-plugin`, `semio-framework-core`, `semio-framework-renderer-wgpu`.
- `cargo build --target wasm32-unknown-unknown` for the same crates; rebuild wasm bindings.
- `bun nx run @semio-tech/framework-renderer-react:test` after the interpreter changes.
- Re-run/extend the WGPU playground E2E harness ([.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts](.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts)) for `forms`, `flow`, `procedural2d`, `procedural3d`, screenshot-diffing wgpu vs React for Edit/Try/Preview/Generate.
- Manual runtime pass with `[DEBUG]` console logs (per repo rules): author a question of every kind, fill the Try wizard to submission, edit a `buildingComponent`'s params and confirm the mesh updates in Preview (both renderers), and add/rename/remove a Generate-mode generation with live preview updates (flow + procedural 2d + procedural 3d, both renderers).
- Open/reopen the appropriate repo-MCP ticket (read `repo://goals` first) before starting implementation; close it with a full touched-files summary when done.