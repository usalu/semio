# W3a — Remove Legacy Per-App Selection/Hover Surface From The Manifest

## What changed

### Manifest (Rust source of truth: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, real struct home: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`)
- Deleted `UiTreeNode.selected_ids` / `highlighted_ids` / `selection_change`.
- Deleted `UiTreeItemNode.hover_action` / `unhover_action`.
- Deleted `ViewModel.selection_json`.
- Added `UiTreeNode.interaction_domain: Option<String>` (camelCase `interactionDomain?: string`).
- Re-pointed `TutorialUiChange::Selection` from `{ selection_json: String }` to `{ domain_id: String, granularity: String, ids: Vec<String> }`, carrying the resolved `DomainSelection` directly (never re-dispatching `interactionSelect`, which would be non-deterministic on replay).
- Added `TutorialUiSnapshot.interaction_selection: HashMap<String, DomainSelection>` (replaces the deleted `selection_json`), and updated `apply_tutorial_ui_change`/`compose_tutorial_ui`'s tests to cover it.
- `DomainSelection` (from `semio-framework-os-kernel`, re-exported at crate root via the interaction module) is now typegen-exported too (`crate::DomainSelection::export()`), since `TutorialUiSnapshot`/`TutorialUiChange` reference it — the same "generated + hand-written duplicate" pattern the crate already uses for `HierarchyProvider`/`HoverSpec`/`SelectionSpec`/`MergeMode` (generated copy in `manifest/component.ts`'s `🤖️generated` mirror, hand copy in `🕹️interaction/component.ts`).
- Regenerated `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts` via `bun nx run @semio-tech/framework:generate`; `:check` passes.
- Hand-mirrored the same shape in `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`.

### Framework-layer breaks fixed (all within semio-framework / semio-framework-os-kernel / semio-framework-plugin / semio-framework-ui / semio-framework-os-renderer-wgpu — never under `✏️s/🔌️plugins/**`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `PanelTreeBuilder`: dropped `.selection_change()`, kept `.selected()/.highlighted()` (still stamp `presence` via `ui_tree_stamp_presence`), added `.interaction_domain(id)`; fixed the framework-injected History-panel tree literal.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️events.rs` (feature `wgpu-engine`) — deleted the per-item `hover_action`/`unhover_action` re-derivation in `is_plain_stack_container`/`update_hover` (hover is now framework-owned via `interaction_domain`, not an ad hoc per-item action); replaced the now-dead test `hovering_a_tree_row_fires_its_hover_action_and_leaving_fires_unhover_action` with `hovering_a_tree_row_no_longer_fires_a_per_item_action`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️engine.rs`, `🦀️reconcile.rs`, `🦀️paint.rs` (same feature) — updated every `UiTreeNode`/`UiTreeItemNode` struct literal (reconciliation to the retained `WidgetNode` tree, golden-JSON tests, etc.) to the new field set. `WidgetNode::Tree`'s own `selected_ids`/`highlighted_ids`/`selection_change` fields (a distinct retained-mode type in `widgets.rs`, not `UiTreeNode`) are untouched — they were already hardcoded empty/no-op per a pre-existing comment ("tree-level id lists are gone, not re-derived").
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` (part of `semio-framework-os-renderer-wgpu`) — dropped `ViewModel.selection_json`/`TutorialUiSnapshot.selection_json` read/write sites; deleted `context_menu_selection_groups` (parsed the now-gone opaque `ViewModel.selectionJson` into `ContextMenuSurfaceTarget.selection` groups) and left that request field as an explicit empty `Vec` with a comment — the framework's `InteractionState` isn't threaded into the Shell's context-menu request yet, a follow-up.
- Confirmed **not** broken (own, unrelated fields sharing the same names, left untouched): `🗺️surface/🎨️paint`'s `PaintHost.selected_ids`, `🎠️kernel`'s `PatchWorld3dChrome.document_selected_ids/document_highlighted_ids`, `🌊️flow/🖥️host`'s `collapse_selection(selected_ids)`, `♾️infinite`'s `WorldState.selected_ids`/board's `highlighted_ids` (product-role, wave-4 scope anyway), `📺️renderer/…/Scenes` and `EngineCanvas`'s ink-canvas/DAG local `selected_ids`/`highlighted_ids` vars, `🗺️tiled-map`'s unrelated `set_selection_json`, and every scene-specific `selection_json` field (`World3dScene`, `InkCanvasScene`, `TableScene`, `Paint2dScene`, `GisMapScene`, `Board2dScene` …).

### Merge vocabulary unification (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`)
- Deleted the bespoke `SelectionMergeMode` union (`"default"|"additive"|"subtractive"|"invertive"`); `marqueeModeFromModifiers`/`selectionMergeIds` now use the framework `MergeMode` (`"replace"|"additive"|"subtractive"|"invertive"|"range"`) — `"default"` → `"replace"`; `"range"` is accepted by the type but never produced by a topology-free marquee drag (`selectionMergeIds` treats it as `"replace"`, documented inline).
- Updated every TS call site this ticket owns (all under `🧰️framework/`, none under `✏️s/🔌️plugins/`):
  - `🖱️ui/🧱️elements/🐚️ShellScope/🟦️component.tsx` — `SelectionModeStore` now typed/keyed by `MergeMode`, default `"replace"`.
  - `📺️renderer/…/ShellHelpers/🟦️component.tsx` — `SelectionUtilityOptions`'s toggle group now emits `"replace"` instead of `"default"`.
  - `📺️renderer/…/Paint2dHost/🟦️component.tsx`, `…/TiledMapHost/🟦️component.tsx` — retyped to `MergeMode` (imported from `@semio-tech/framework`, which already re-exports the ts-rs-generated `MergeMode` via the manifest barrel).
  - `📺️renderer/…/World3dHost/🟦️component.tsx` — heaviest call site; `resolveWorldSelectionMergeMode` renamed `resolveWorldMergeMode` (mechanical, every call site updated), all `"default"` literals → `"replace"`.
- `bunx tsc --noEmit -p tsconfig.json` before and after this change reports the exact same 19 pre-existing errors (all inside `✏️s/🔌️plugins/🔱️trinity`, `✏️s/🔌️plugins/🗄️stdio`, and an unrelated vscode-extension file) — zero new errors. Every touched package's `package.json` `exports` field points straight at its live `.tsx`/`.ts` source (no built `dist/`), so this is a real, non-stale check.
- No plugin-side TS callers of `SelectionMergeMode`/`marqueeModeFromModifiers`/`selectionMergeIds` exist (grepped repo-wide) — nothing to migrate for the next wave here.

## Acceptance (real output saved alongside this file)
- `w3a-cargo-test-semio-framework.txt` — `cargo test -p semio-framework`: **105 passed, 0 failed**.
- `w3a-cargo-test-semio-framework-typegen.txt` — `cargo test -p semio-framework --features typegen`: **106 passed, 0 failed** (includes `exports_typescript_bindings`).
- `w3a-cargo-check-semio-framework-plugin.txt` — `cargo check -p semio-framework-plugin`: **0 errors**.
- `w3a-bun-script-test.txt` — `bun ./📜️script.ts test` (framework rust package: cargo test + vitest): **105/105 Rust + 146/146 TS passed**.
- `w3a-renderer-wgpu-informational-check.txt` — `cargo check -p semio-framework-os-renderer-wgpu` (NOT one of the three required-green crates; kept for transparency): 49 errors, **every one of them inside `✏️s/🔌️plugins/🧩️puzzle/**` or `🧰️framework/…/🌊️flow/…/📖️playbook/component.rs`** (the `semio-framework-os-flow` product crate) — see work-list below. Zero errors from `semio-framework-ui`, `semio-framework`, `semio-framework-plugin`, or the renderer's own `Shell`/`EngineCanvas`/`Interpreter`/`Scenes` files.

## Next-wave work-list (plugin-side breaks — NOT touched here, per instructions)

Grepped the whole repo for `hover_action:`/`unhover_action:`/`selected_ids: `/`highlighted_ids: `/`selection_change:` struct-literal usages. Every hit outside the framework layer falls under `✏️s/🔌️plugins/**`, one per app directory (rough hit counts, several plugins have >1 app/window/panel touching the tree):

| Plugin dir | hits | Plugin dir | hits |
|---|---|---|---|
| 🧩️puzzle | 40 | 📋️forms | 9 |
| 🧱️block | 27 | 🗒️note | 5 |
| 📏️layout | 22 | 🕸️dag | 5 |
| 🌍️gis | 21 | 💠️lowpoly | 2 |
| 🎞️animate | 18 | 🏭️process | 2 |
| 🏛️architect | 15 | 🎬️sequence | 2 |
| 🖨️raster | 13 | ✒️writer | 2 |
| 🖍️draw | 13 | 🔱️trinity | 1 |
| 💡️reasoning | 12 | 📐️cad | 1 |
| 📖️playbook (plugin, distinct from the `🌊️flow`-mounted product file of the same name) | 11 | 🎥️shooting | 1 |
| 🌀️procedural | 10 | 🌿️vcs | 1 |

Verified compile-error shapes (from `semio-framework-os-renderer-wgpu`'s direct `puzzle` dependency — the only plugin wired in as a hard Cargo dependency today, so the only one with real rustc diagnostics):
- `error[E0560]: struct 'UiTreeItemNode' has no field named 'hover_action'/'unhover_action'` — delete the two lines (or the whole two-field block) from every `UiTreeItemNode { ... }` literal.
- `error[E0560]: struct 'UiTreeNode' has no field named 'selected_ids'/'highlighted_ids'/'selection_change'` — delete; add `interaction_domain: Some("<domain-id>".into())` once the app declares an `InteractionDefinition` for that tree (or `None` if it doesn't yet).
- `error[E0599]: no method named 'selection_change' found for struct 'PanelTreeBuilder'` — drop the `.selection_change(...)` call from the builder chain; use `.interaction_domain("<id>")` instead once the app has one.
- `error[E0063]: missing field 'interactions' in initializer of 'WindowKindDefinition'` — **pre-existing from wave 1** (not introduced by W3a): add `interactions: vec![]` (or real `InteractionRef`s) to every `WindowKindDefinition { ... }` literal.

`semio-framework-os-flow`'s `📖️playbook/🦀️component.rs` (product-role `semio-framework-os-flow` crate, not one of the three required-green crates, not under `✏️s/🔌️plugins/`) has the identical `hover_action`/`unhover_action`/`selected_ids`/`highlighted_ids`/`selection_change` shape at lines 598-599, 623-624, 650-651, 662 — same mechanical fix, left for the crate's owning wave since it's product-role like the plugins.

`♾️infinite` (`semio-framework-os-infinite`, product role) was NOT broken by this wave — its `selected_ids`/`highlighted_ids` are its own `WorldState`/board fields, unrelated to `UiTreeNode`. Still explicitly wave-4 scope per the ticket's task list ("migrate 17 plugin crates + infinite"), for unrelated reasons (manifest/interaction wiring generally).
