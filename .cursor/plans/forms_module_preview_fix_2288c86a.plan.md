---
name: Forms Module Preview Fix
overview: Fix the two concrete regressions that make `forms` look "largely incomplete" (fresh document opens empty instead of the Building Component fixture; the `forms-module-procedural` plugin panics on load because its app declares zero window kinds), then re-verify the rest of the already-implemented premigration-parity work (Edit/Try wizards, Generate mode in flow/procedural-2d/procedural-3d, React+wgpu parity) end-to-end so the "module contributes a question kind with exposed params + live preview" mechanism is proven to work, not just present in code.
todos:
 - id: ticket
   content: Reopen repo ticket 2026/07/08/forms-blueprint-try-module-preview via ticket_reopen before starting work
   status: completed
 - id: fix-window-kind
   content: Add window kinds + default layout to forms-module-procedural's App builder; drop dead imports; extend its test module
   status: completed
 - id: fix-default-doc
   content: Seed FormsPlayApp::initial_document_json() from the Building Component fixture instead of an empty projection; extend forms-plugin tests
   status: completed
 - id: build-wasm
   content: cargo build --target wasm32-wasip2 for forms-plugin, forms-module-procedural, flow-plugin, procedural2d/3d-plugin to confirm no regressions
   status: completed
 - id: runtime-verify-react
   content: Boot React dev host, verify fresh Forms load shows Building Component fixture with working sliders + live 3D preview, and default/onboarding examples still render every kind
   status: completed
 - id: runtime-verify-wgpu
   content: Repeat the same manual verification against the wgpu shell for React/wgpu parity
   status: completed
 - id: runtime-verify-generate
   content: Exercise Generate mode in flow, procedural/2d, procedural/3d (add/rename/remove generation, live preview)
   status: completed
 - id: close-ticket
   content: Update plan todo statuses and close the repo ticket with a full touched-files summary
   status: completed
isProject: false
---

## Root causes found

1. **Crash**: [forms/module/procedural/rs/lib.rs](forms/module/procedural/rs/lib.rs) `create_module_app()` builds an `App` with no `.window_kind(...)`:

```441:442:forms/module/procedural/rs/lib.rs
fn create_module_app() -> App {
    App::from_builder(App::builder(MODULE_APP_ID, "Forms Module Procedural").document(["semio", "forms"]))
}
```

`AppBuilder::build_definition` in [framework/plugin/rs/lib.rs:216-220](framework/plugin/rs/lib.rs) asserts `!window_kinds.is_empty()`, so `module_bundle()` panics the moment the plugin's manifest is built, trapping the whole wasm component (`unreachable` abort — matches the pasted console error exactly). This is a real, universally-enforced framework invariant (mirrored in the hot-swap validator at [framework/product/os/core/rs/lib.rs:467-471](framework/product/os/core/rs/lib.rs)), not something to bypass — the app itself needs real window kinds.

- Direct render calls (`plugin_render`, used by `ui_external_slot`/`ExternalSlot` resolution in both React's `resolveExternalSlots` and wgpu's `resolve_external_slots_in_tree`) dispatch on `body_key` directly and don't require the key to match a declared window kind, so adding window kinds here is purely to satisfy the app-level invariant (and, as a bonus, makes the module directly launchable/debuggable on its own).

2. **"No questions show" on a fresh document**: premigration's `FormsPlayController` (`git show 4eb2e63927:forms/play/index.ts`) seeded its `spec` field with the **`building-component`** fixture by default, not an empty spec:

```444:447:forms/play/index.ts (premigration)
private spec: FormSpec = (() => {
    const json = FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID["building-component"];
    return json ? formSpecFromJson(json) : defaultFormSpec();
})();
```

Today's [forms/plugin/rs/lib.rs](forms/plugin/rs/lib.rs) `FormsPlayApp::initial_document_json()` calls `default_envelope()` → `empty_forms_projection()`, which is a single step with **zero questions**. Opening Forms fresh therefore shows nothing until a user manually picks an example — this is the "no questions show" regression.

## Fix 1 — `forms/module/procedural/rs/lib.rs`

- Add two real window kinds to `create_module_app()` mirroring the body keys it already renders: `BODY_PARAMS` (label "Params", `SurfaceKind::NodeGraph` or similar generic surface) and `BODY_PREVIEW` (label "Preview", `SurfaceKind::World3d`, matching what `render_preview_body` actually emits). Add a `default_layout` (row split, mirroring `create_forms_app`'s pattern) so the module is independently valid/launchable (useful for isolated debugging), without changing its `PluginBundle`/`Contribution::FormsQuestionKind` wiring or its two `render()` bodies.
- Extend the existing `#[cfg(test)]` module in that file with a regression test asserting `create_module_app().definition.window_kinds.len() >= 1` (or the exact ids), so this exact panic can't silently reappear.
- Remove the now/already-unused imports (`ui_external_slot`, `UiButtonNode`, `UiNumberStepperNode`, `FormSpec`, `apply_generation_values_to_fixture`, `LazyLock`) flagged by `cargo check` while touching the file (clean long-term code, no dead imports).

## Fix 2 — `forms/plugin/rs/lib.rs`

- Change `FormsPlayApp::initial_document_json()` to seed the envelope's projection from `BUILDING_COMPONENT_EXAMPLE_JSON` (parsed via `create_document_vcs_envelope`) instead of `empty_forms_projection()`, matching premigration's default. Keep `default_envelope()`/`empty_forms_projection()` available (still used by the `"empty"` example entry and as the fallback in `parse_envelope`).
- Update/extend the existing test module: the `app_has_blueprint_and_try_windows_only` and related tests should keep passing; add/extend a test asserting the fresh `initial_document_json()` materializes to the Building Component steps (non-empty, contains the `buildingComponent`-kind question) so this can't regress silently either.

## Re-verify the rest of the premigration-parity plan (full scope, as requested)

The existing [.cursor/plans/forms_premigration_parity_both_renderers_b3d29b3a.plan.md](.cursor/plans/forms_premigration_parity_both_renderers_b3d29b3a.plan.md) marks phases 1–8 "completed" and phase 9 (verify-all) "in_progress" — the two bugs above were found by code audit, not by running anything, so phase 9 clearly never actually ran end-to-end. Spot-checks already done in this planning pass that look correct (no action needed, but will be re-confirmed at runtime):

- wgpu nested `ComponentScene` recursion (`render_ui_node_inner` in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) recurses through `Stack`/`Section` before falling back to leaf widgets).
- wgpu `ExternalSlot` resolution (`resolve_external_slots_in_tree` at [framework/renderer/wgpu/rs/lib.rs:7571](framework/renderer/wgpu/rs/lib.rs), run before the sync widget-conversion fallback at line 4241) and React's `resolveExternalSlots` ([framework/core/js/index.ts:712](framework/core/js/index.ts)) are both implemented.
- Generate mode is wired (`.mode("generate", "Generate")` + dedicated window kinds) in [flow/plugin/rs/lib.rs:1245-1266](flow/plugin/rs/lib.rs) and [procedural/plugin/rs/app_3d.rs:1406-1450](procedural/plugin/rs/app_3d.rs) (and `app_2d.rs`), backed by `flow_fixture_to_form_spec`/`apply_generation_values_to_fixture` in `flow_core::forms_bridge`.

Actual verification to run (native `cargo test`/`cargo check` for these plugin crates is a pre-existing, unrelated, already-documented blocker — `plugin_exports!` needs `wasm32`+`p2` and can't link `component_export_anchor` natively, per `.repo/🎫/26/07/09/GIS-2D-MAP-PARITY-RESTORE/verify-log.md` and others — so this is not something to fix here):

1. `cargo build --target wasm32-wasip2 --release` (or the `bun nx run <pkg>:wasm` equivalent) for `forms-plugin`, `forms-module-procedural`, `flow-plugin`, `procedural2d-plugin`/`procedural3d-plugin` (whatever their nx package names are), confirming no compile regressions from the two fixes above.
2. Boot the React dev host, open Forms fresh, confirm with `[DEBUG]` console logs:
   - The Building Component fixture loads immediately (no manual example pick needed) and every non-extension question renders (text/longText/single/multi/boolean/date/color/number/slider/vector/note/image/file).
   - The `buildingComponent` "Hexagonal Column" question renders its procedural params (height/radius/sides sliders) via the now-fixed `forms-module-procedural` slot, and the 3D preview renders and live-updates as sliders move, in both Blueprint (Edit) and Try modes.
   - Switching to `default`/"Contact" and `onboarding` examples still renders every question kind correctly (no regression from changing the default seed).
3. Repeat the same manual pass against the wgpu shell (`framework/product/os/hub`) for React/wgpu parity.
4. Exercise Generate mode in `flow`, `procedural/2d`, `procedural/3d`: add/rename/remove a generation, confirm the generated `FormSpec` form renders and the preview (text for flow, canvas-2d/world-3d for procedural) updates per-generation.
5. Update the existing plan's todo statuses (or note discrepancies) and, per repo rules, reopen ticket `2026/07/08/forms-blueprint-try-module-preview` (currently closed, exact same scope/goal `🎯forms🎯formstechnology`) via `ticket_reopen` before starting, then close it again with a full touched-files summary once verification passes.

## Files to touch

- [forms/module/procedural/rs/lib.rs](forms/module/procedural/rs/lib.rs) — add window kinds + layout, drop dead imports, extend tests.
- [forms/plugin/rs/lib.rs](forms/plugin/rs/lib.rs) — seed default document from the Building Component fixture, extend tests.
- No changes planned to the unrelated files currently showing as modified in git status (`framework/product/os/core/rs/lib.rs`, `framework/product/os/hub/rs/bin.rs`, `print/tex/*.sty`, `s/rs/lib.rs`, `s/plugin/rs/lib.rs`) — those belong to the separate in-flight "VCS backbones and sync tools" effort and are out of scope here.
