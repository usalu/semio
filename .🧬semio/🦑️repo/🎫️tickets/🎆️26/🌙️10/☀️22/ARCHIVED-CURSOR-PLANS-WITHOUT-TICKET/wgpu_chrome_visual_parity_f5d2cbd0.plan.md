---
name: Wgpu Chrome Visual Parity
overview: "Close four concrete parity gaps between the wgpu shell and the React shell: wrong default font, invisible SVG icons, icon-only controls missing their text labels, and an incomplete Tree widget. Ship full wire-protocol parity for Tree (data model, rendering, interaction, and drag/drop where the shared `UiTreeNode` protocol supports it)."
todos:
 - id: font-anta
   content: Swap embedded/fetched boot font from Kelly Slab to Anta in framework/renderer/wgpu/rs/lib.rs
   status: completed
 - id: icon-uv-fix
   content: Fix icon atlas UV normalization to use fixed 2048 GPU texture size instead of packed atlas dimensions (JS + shared constant), verify semio logo and all icons render
   status: completed
 - id: chrome-labels-audit
   content: Audit all render_chrome_group/render_cap_button call sites in shell.rs and dock.rs; add explicit text labels to panel toggles, fullscreen toggle, and dock focus/close buttons to match React
   status: completed
 - id: tree-data-model
   content: Expand TreeItem/TreeSection structs in ui/wgpu/rs/widgets.rs and interpreter.rs mapping to carry description, icon_id, default_open, control, actions, draggable/drag_data, is_hidden, highlighted_ids/selected_ids/selection_change
   status: completed
 - id: tree-render
   content: "Rewrite render_tree/render_tree_item geometry: theme-driven row height/indent, default+custom icons, clickable chevron hit target, guide lines, selection/hover/highlight/hidden styling, description text, inline control rows, action icons, correct scroll height"
   status: completed
 - id: tree-interaction
   content: Wire handle_shell_hit for tree/section expand-collapse, label click command dispatch, hover/unhover commands, action clicks, and selection_change dispatch
   status: completed
 - id: tree-dnd
   content: Implement pointer-based drag for draggable tree items with drop-position indicator and payload-drop dispatch to matching targets
   status: completed
 - id: verify-parity
   content: Rebuild wasm, run cargo tests, run E2E for a tree-bearing program, and screenshot-diff against the React shell to confirm font/icon/label/tree parity
   status: completed
isProject: false
---

# Wgpu Chrome Visual Parity

## 1. Default font: Anta, not Kelly Slab

`semio_renderer_boot` in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) hardcodes Kelly Slab as the only boot font:

```289:296:framework/renderer/wgpu/rs/lib.rs
const KELLY_SLAB_LATIN: &[u8] = include_bytes!("../../../../ui/asset/font/kelly-slab/latin.ttf");
let font_bytes = match fetch_font_bytes("/asset/font/kelly-slab/latin.ttf").await {
    Ok(bytes) if bytes.len() > 256 => bytes,
    _ => KELLY_SLAB_LATIN.to_vec(),
};
```

But [ui/styling/tokens.json](ui/styling/tokens.json) assigns Kelly Slab to `fontStacks.serif` (decorative, unused by chrome) and **Anta** to `fontStacks.sans` (`--font-sans`, applied via `font-sans` on every React shell root). Anta's TTF already exists at `ui/asset/font/anta/latin.ttf`.

- Swap the embed/fetch path to `ui/asset/font/anta/latin.ttf` / `/asset/font/anta/latin.ttf`.
- `FontAtlas` (`ui/wgpu/rs/text.rs`) only holds one `fontdue::Font`, so this is a straight swap, not a multi-font system. No chrome text today uses Kelly Slab intentionally, so no dual-font support is needed for this pass.

## 2. Icons not rendering (semio logo and others)

Root cause is a UV/texture-size mismatch, not a missing atlas or stale reference:

- GPU icon texture is a fixed `2048x2048` allocation (`ui/wgpu/rs/draw.rs:918`), and `write_texture` always writes at `Origin3d::ZERO` (`ui/wgpu/rs/draw.rs:1436-1450`), so packed pixel data always lands in texel range `[0,width) x [0,height)` of that 2048x2048 texture.
- But `buildIconAtlas` in [framework/renderer/wgpu/js/index.ts](framework/renderer/wgpu/js/index.ts) normalizes UVs against the **packed** atlas size (~384x192 for ~125 icons), not against the fixed 2048 texture:

```82:framework/renderer/wgpu/js/index.ts
entries[item.id] = [ox / width, oy / height, (ox + ICON_SIZE) / width, (oy + ICON_SIZE) / height];
```

This makes every icon UV sample the wrong texel region of the 2048x2048 texture (mostly empty/transparent), so icons silently fail to draw even though `icon_uv()` returns `Some(...)`.

- Fix: compute entries as fractions of the fixed GPU texture size (`2048`) instead of the packed atlas `width`/`height`. Add a shared constant (e.g. `ICON_ATLAS_TEXTURE_SIZE = 2048`) used both when creating the GPU texture in Rust and when normalizing UVs in JS, so the two can never drift again.
- Secondary (cosmetic) race: `start_frame_loop` runs before JS calls `uploadIconAtlas` (`framework/renderer/wgpu/rs/lib.rs:334`), so the first frame(s) paint with an empty atlas. This self-heals next frame; leave as-is unless a visible flash is confirmed after the UV fix.

## 3. Buttons/toggles missing labels

React resolves a visible inline label for every chrome control via `useControlInlineText`, and `Navbar` forces `policy="always"` so navbar items always show text regardless of compact mode ([ui/js/react/index.tsx:9017-9019](ui/js/react/index.tsx), `useControlInlineText` at 2273-2279). Window/action controls (`ActionGroupItem`) always pass explicit `text` (e.g. "Focus"/"Unfocus", "Close", "New Window" in the `Window` component, `ui/js/react/index.tsx:15466-15481`).

Wgpu's `ChromeGroupItem`/`render_chrome_group` ([framework/renderer/wgpu/rs/shell.rs:1631-1721](framework/renderer/wgpu/rs/shell.rs)) only draws text when `label: Some(...)` is explicitly passed, with no id-based fallback resolution. Audit and fix every construction site that currently passes `label: None`:

- Navbar panel toggle group: `ui.panelToggle.display/workbench/details/settings` (`shell.rs:2148-2207`) - add `Some("Display")`, `Some("Workbench")`, `Some("Details")`, `Some("Settings")`.
- Navbar fullscreen toggle: `ui.fullscreen.toggle` - add `Some("Fullscreen")` (React resolves the same id to "Fullscreen" via humanized id fallback).
- Dock stack cap buttons in [framework/renderer/wgpu/rs/dock.rs:600-601](framework/renderer/wgpu/rs/dock.rs) (`render_cap_button` for focus/maximize and close) - these are icon-only today; React's equivalent `Window` controls always show "Focus"/"Unfocus" and "Close" text. Extend `render_cap_button` to optionally draw a label next to the icon, matching `ActionGroupItem` sizing, and pass the matching text at both call sites.
- Sweep `render_window_measures_rail` / `render_window_engagement_rail` (`shell.rs`) and `render_studio_canvas_bars` for any remaining `label: None` group items and fill them in with the same text React shows for the equivalent id.
- Verify base `WidgetNode::Button` / `WidgetNode::Toggle` (`ui/wgpu/rs/widgets.rs`) already render whatever `label` the declarative `UiNode` provides (they do) - no changes needed there beyond confirming call sites in inspector/settings panels actually pass a label.

## 4. Tree widget: full wire-protocol parity

Both renderers share one declarative wire type, `UiTreeNode`/`UiTreeSectionNode`/`UiTreeItemNode` ([framework/core/rs/ui.rs:203-268](framework/core/rs/ui.rs)), carrying: `label`, `description`, `iconId`, `selected`, `defaultOpen`, `command`, `hoverCommand`, `unhoverCommand`, `actions` (icon+label+command+revealOnHover), `draggable`, `dragData`, `items`, `control`, `isHidden`; and at tree level `selectedIds`, `highlightedIds`, `selectionChange`. React consumes 100% of this via `uiTreeItemsToTreeData`/`uiTreeNodeToTreePanelConfig` ([framework/renderer/react/ui-interpreter.tsx:252-295](framework/renderer/react/ui-interpreter.tsx)). Note: React's `getItems` (async lazy children) and `contextMenu` are **not** part of this wire protocol (they're used only by non-framework-driven, host-local trees) - so "full parity" here means matching everything the shared protocol actually carries, including drag/drop, not adding new async-loading or context-menu wire fields.

Wgpu's current `TreeItem`/`TreeSection`/`render_tree` ([ui/wgpu/rs/widgets.rs:36-88, 611-663](ui/wgpu/rs/widgets.rs)) and the interpreter mapping ([framework/renderer/wgpu/rs/interpreter.rs:108-191](framework/renderer/wgpu/rs/interpreter.rs)) only carry `id`, `label`, `selected`, `command`, `children` - dropping everything else, and rendering is flat text rows with a non-clickable unicode chevron and no working expand/collapse (no `handle_shell_hit` branch for `tree.*`/`section.*`).

### 4.1 Data model

- Extend `TreeItem<E>` / `TreeSection<E>` in `ui/wgpu/rs/widgets.rs` to carry: `description`, `icon_id`, `default_open`, `control: Option<Box<WidgetNode<E>>>`, `hover_event`/`unhover_event`, `actions: Vec<TreeItemAction<E>>` (icon, label, command, reveal_on_hover), `draggable`, `drag_data: HashMap<String,String>`, `is_hidden`. Add `id`/`default_open` to `TreeSection`.
- Add `highlighted_ids`, `selected_ids`, `selection_change` to the top-level tree widget node so multi-select and highlight paths are representable.
- Update `tree_item_to_widget`/`tree_section_to_widget` in `framework/renderer/wgpu/rs/interpreter.rs` to map every field 1:1 with `ui-interpreter.tsx`'s mapping, so both renderers stay in sync from one source of truth.

### 4.2 Rendering (match React geometry in `render_tree`/`render_tree_item`)

- Row height and indent from theme tokens instead of magic `22.0`/`12.0` (mirror `ui/styling/tokens.json` tree tokens: ~24px row, ~10px indent/level + ~14px toggle slot).
- Default icons: `"folder"` for expandable items/sections, `"file-text"` for leaves, overridable by `icon_id`.
- Clickable chevron as its own hit target (toggle collapse) separate from the label hit target (fires `command`), matching React's split between gutter chevron button and content click.
- Ancestor guide lines (simple vertical hairlines per depth level, with last-sibling truncation) to visually match React's branch guides.
- Selection/hover/highlight fills on the content column only (leave gutter transparent so guides read through), plus `is_hidden` → dimmed/muted rendering.
- Render `description` as secondary muted text after the label; render `control` (if present) as a right-aligned inline widget via the existing `render_widget` dispatch (property-row layout).
- Render `actions` as small icon buttons at the row's right edge, honoring `reveal_on_hover` (only paint when the row is hovered or an action is focused).
- Fix scroll-content height measurement to recurse into expanded children instead of the current flat `items.len() * 22` estimate.

### 4.3 Interaction wiring

- Add `tree.*`/`section.*` branches to `handle_shell_hit` in `framework/renderer/wgpu/rs/shell.rs` to toggle `collapsed_sections` on chevron/row click, matching `default_open` initial state.
- Dispatch `item.command` on label click, `hover_command`/`unhover_command` on pointer enter/leave, and `action.command` on action-icon click.
- Implement `selection_change` dispatch: clicking a row updates the tracked `selected_ids` set and fires the tree's `selection_change` command with the new id list (single-select by default; extend for modifier-key multi-select if `InputState` exposes modifier keys - otherwise ship single-select and note range/multi-select as a documented follow-up limitation, since wgpu's input layer may not currently expose shift/meta state to hit-testing).

### 4.4 Drag and drop

- For items with `draggable`/`drag_data` set, add pointer-based drag support (press-and-move threshold on the row) using the existing `InputState` drag primitives already used for split-resize in `dock.rs`, generalized to carry an arbitrary string payload.
- Render a drop-position indicator (thin insertion line before/after a target row, or a highlighted rect for "inside") while dragging, matching React's `TreeReorderDropPreview` visual language.
- On release over a valid drop target, dispatch the same command path the row's drag payload implies (this targets the practical existing use - dragging catalogue/palette rows onto a canvas or inspector drop target with their `drag_data` payload; there is no generic "reorder" wire command in `UiTreeNode` today, since React's in-tree reordering is handled by host-local `TreeDragAndDropController` callbacks rather than the wire protocol - wgpu will support payload-drop-to-target, not intra-tree row reordering, since the latter has no wire representation to dispatch).

## 5. Verification

- `cargo test` (including any new dock/tree unit tests), rebuild wasm bindings for `ui_wgpu` + `semio-framework-renderer-wgpu`.
- Run the wgpu E2E harness for at least one plugin with a real document/catalogue tree panel, confirm: Anta font renders, semio logo + navbar/footer icons are visible, panel toggle and fullscreen controls show text labels, dock focus/close show text labels, and the tree panel shows correct indentation/icons/chevrons/expand-collapse/selection.
- Side-by-side screenshot diff against the equivalent React shell view for the same program to confirm visual parity, same approach as the prior layout-parity verification pass.
