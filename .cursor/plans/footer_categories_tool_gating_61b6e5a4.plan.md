---
name: Footer Categories Tool Gating
overview: Restructure the wgpu OS shell footer to remove the dead app button, always render four fixed sections in order (Selection, Tools, Commands, History), enforce a single active tool per app, and gray out Commands while a Tool is active unless that tool explicitly opts out.
todos:
  - id: core-schema
    content: Add ToolCategory enum + category field/getter/with_category builder to framework/core ToolNode
    status: completed
  - id: wgpu-remove-dead
    content: Remove useless app button and duplicate studio undo/redo/checkpoint footer chrome + handlers
    status: completed
  - id: wgpu-sections
    content: Rewrite render_footer to bucket active_tools into fixed Selection/Tools/Commands/History sections with dividers
    status: completed
  - id: wgpu-gating
    content: Add active_tool_id derivation, ChromeGroupItem disabled state, and Commands gating with allow-list
    status: completed
  - id: lowpoly-categories
    content: Tag lowpoly edit_tools/paint_tools collections with correct categories
    status: completed
  - id: cad-categories
    content: Tag cad build_cad_play_toolbar collections with correct categories + zoom-like allow-list entry
    status: completed
  - id: sequence-categories
    content: Tag sequence edit_tools categories and split layout collection into Reorganize command + Orientation tool group
    status: completed
  - id: s-categories
    content: Tag s home_create_tools and studio history collection with correct categories
    status: completed
  - id: verify-build
    content: cargo check touched crates and manually verify footer behavior across lowpoly/cad/sequence/s
    status: completed
isProject: false
---

# Consistent Footer Categories, Single Active Tool, Command Gating (wgpu renderer)

## Scope
Touches the shared tool schema in [framework/core/rs/lib.rs](framework/core/rs/lib.rs) (needed by any renderer, purely additive/non-breaking so the React renderer keeps compiling unchanged), the wgpu shell in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs), and the four plugins that currently feed the wgpu footer with real toolbars: `lowpoly`, `cad`, `sequence`, `s`. No other plugin (flow, gis, puzzle, mindmap, forms, ...) is touched — their tools fall back to a generic default categorization and keep working as-is. The React renderer (`framework/renderer/react/os-shell.tsx`) is not touched.

## 1. Shared schema — `framework/core/rs/lib.rs` (`pub mod tools`)
- Add `ToolCategory { Selection, Tools, Commands, History }` (serde camelCase, `Copy`).
- Add an optional `category: Option<ToolCategory>` field to `ToolNode::Button`, `Toggle`, `Collection` (not `Separator`).
- Add `ToolNode::category(&self) -> ToolCategory`: returns the explicit value if set, else defaults `Toggle | Collection -> Tools`, `Button -> Commands`. `Selection` and `History` are always explicit opt-in.
- Add `ToolNode::with_category(self, category: ToolCategory) -> Self` builder, mirroring the existing `with_order`/`with_disabled` pattern (currently duplicated locally in `cad/plugin/rs/lib.rs` as `ToolNodeExt` — leave that trait as-is, just add the new capability in core).

This is additive-only (`Option`, defaulted), so every currently-untouched plugin keeps compiling and rendering exactly as today, just bucketed by the new default rule.

## 2. wgpu shell — `framework/renderer/wgpu/rs/lib.rs`

### Remove dead/duplicate chrome
- Delete the first `footer_items` entry `"framework.footer.app"` (~line 10918-10924) — it duplicates the app name/icon already shown in the navbar title (`app_document_label` at ~line 10726-10730) and has no click handler (`handle_shell_hit` never matches it).
- Delete the hardcoded studio-mode `undo`/`redo`/`checkpoint` footer buttons (~line 10925-10948) and their handlers `"framework.footer.undo" | "framework.footer.redo" | "framework.footer.checkpoint"` (~line 9178-9204). These are an exact duplicate of `s`'s own `tool_collection("s-play.history", ...)` (same `s_play_cmd("undo"/"redo"/"commitCheckpoint")`), which already flows through `active_tools` — this is the concrete "inconsistency" causing duplicated history controls.

### Fixed four-section footer
- Add `ShellState.active_tool_id: Option<String>`, recomputed in `refresh_ui` right after `self.active_tools` is assigned: scan `active_tools` recursively for the first `Toggle` whose `category() == Tools` and `pressed == true`.
- Add `fn partition_tools_by_category(tools: &[ToolNode]) -> [Vec<ToolNode>; 4]` bucketing top-level nodes into `[selection, tools, commands, history]` via `ToolNode::category()`.
- Rewrite `render_footer` to render each non-empty bucket in fixed order **Selection → Tools → Commands → History**, with a divider (reusing the existing hairline separator visual) between non-empty sections. Empty sections render nothing (no empty dividers).

### One active tool + command gating
- Extend `render_footer_tool_nodes` to take `active_tool_id: &Option<String>` and the section's `ToolCategory`.
- Add `const TOOL_ID_PREFIXES_ALLOWING_COMMANDS_WHILE_ACTIVE: &[&str]` and `fn tool_allows_commands_while_active(tool_id: &str) -> bool` (prefix match) — the hardcoded, wgpu-shell-local allow-list.
- For nodes in the `Commands` section: if `active_tool_id.is_some()` and not allowed by the list above, render the item disabled (muted icon/label via `theme.text_muted`, no hit registered) instead of skipping/hiding it, so users see *why* it's unavailable. `Selection` and `History` sections are never gated.
- Add a `disabled: bool` field to `ChromeGroupItem` (default `false` at all existing call sites) and use it in `render_chrome_group` to pick muted colors and skip `register_hit`.

### Concrete "zoom-like" allowance
- Register `cad.play.view.` (the pane-focus toggle prefix from `build_cad_play_toolbar`) in `TOOL_ID_PREFIXES_ALLOWING_COMMANDS_WHILE_ACTIVE` — switching which model-space pane is focused is a passive/navigational tool like zoom, not an edit-blocking one, so Save/Transfer commands must stay enabled while it's "active".

## 3. Plugin re-categorization

### `lowpoly/plugin/rs/lib.rs` (`edit_tools`, `paint_tools`)
- `lowpoly-tools-selection` (Mesh/Face/Edge/Vertex) -> `Selection`
- `lowpoly-tools-transform` (Move/Rotate/Scale) -> `Tools`
- `lowpoly-tools-edit` (Extrude/Inset/.../Decimate) -> `Commands`
- `lowpoly-tools-history` -> `History`
- `lowpoly-paint-tools` (Brush/Eraser/Fill/Eyedropper) -> `Tools`
- `lowpoly-paint-uv` (Unwrap/Mark Seam/Clear Seam) -> `Commands`
- `lowpoly-paint-history` -> `History`

### `cad/plugin/rs/lib.rs` (`build_cad_play_toolbar`)
- `view` (pane focus toggles) -> `Tools` (+ allow-list entry above)
- `save` -> `Commands`
- `transfer` -> `Commands`

### `sequence/plugin/rs/lib.rs` (`edit_tools`)
- `sequence-tools-execution` (Run/Stop) -> `Commands`
- Split `sequence-tools-layout` into a standalone `Reorganize` button (`Commands`) and a new `sequence-tools-orientation` collection holding the left-right/top-bottom toggles (`Tools`), since today it incorrectly mixes a one-shot command with a mutually-exclusive tool pair under one label.

### `s/plugin/rs/lib.rs`
- `home_create_tools()`: `s-home.create` collection and `s-home.import` button -> `Commands`
- studio `mode_tools("main", [tool_collection("s-play.history", ...)])` -> `History`

## Verification
- `cargo check -p semio-framework-renderer-wgpu -p semio-framework-core -p lowpoly-plugin -p cad-plugin -p sequence-plugin -p s-plugin` (or the nx-wrapped equivalents per `script.ts`) to confirm the schema change and all four plugin edits compile.
- Manually exercise the wgpu playground for lowpoly (edit + paint modes), cad, sequence, and s/studio to confirm: no app button, four ordered sections, only one active tool highlighted at a time, Commands gray out while a Tool is active except cad's pane-focus toggle, and studio undo/redo/checkpoint still work via the single `s-play.history` path.
