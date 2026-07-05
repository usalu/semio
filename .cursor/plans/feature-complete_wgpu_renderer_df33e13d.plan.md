---
name: Feature-Complete Wgpu Renderer
overview: "Bring the raw wgpu renderer to feature parity with the React OS shell: full chrome (navbar, footer, floating resizable side panels, window tabs, context menu), fully interactive widgets, and real implementations of all eight component scenes."
todos:
  - id: toolkit-clip-scroll
    content: Scissor stack + scroll regions in ui/wgpu draw.rs/widgets.rs
    status: completed
  - id: toolkit-input
    content: Hover/drag/focus state machine and full keyboard events in input.rs
    status: completed
  - id: toolkit-text
    content: Text editing (cursor, commit) and word wrap in text.rs
    status: completed
  - id: toolkit-icons
    content: "Icon atlas: JS SVG rasterization, upload, icon_uv lookup, draw in widgets"
    status: completed
  - id: toolkit-overlay
    content: Overlay layer for dropdowns, context menus, drag ghosts
    status: completed
  - id: widgets-interactive
    content: Interactive slider/ring/stepper/select/toggle/input/section/tree widgets
    status: completed
  - id: chrome-navbar-footer
    content: Full navbar item set and new footer in shell.rs
    status: completed
  - id: chrome-panels
    content: Floating resizable side panels with working tab switching and toggles
    status: completed
  - id: chrome-windows-menu
    content: Window tab bar, context menu, keyboard shortcuts, unified input path
    status: completed
  - id: scene-raster-table
    content: Raster textured quad + interactive scrollable table
    status: completed
  - id: scene-graph-flow
    content: Node-graph and flow-canvas with pan/zoom/drag/select/connect
    status: completed
  - id: scene-canvas-text-vfs
    content: Canvas-2d pointer commands, text-editor editing, VFS selection/routing
    status: completed
  - id: verify-e2e
    content: Extend E2E with interaction smoke + zero-warning assertion, run 25-plugin suite
    status: completed
isProject: false
---

# Feature-Complete Wgpu Renderer

## Reference contract

The React shell ([framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx)) is the behavioral contract: navbar + footer (both `h-large` ~36-40px), floating side panels (absolute, inset, left 280px / right 320px, resizable 200-600px, tab bars), resizable window layout, engagement command line, context menus, and 8 component scenes. The wgpu renderer ([framework/renderer/wgpu/rs](framework/renderer/wgpu/rs) on top of [ui/wgpu/rs](ui/wgpu/rs)) currently has a skeleton shell, display-only widgets, and 7 of 8 scenes at stub level (world-3d is the only complete one).

## Phase 1 — Toolkit foundations (`ui/wgpu/rs`)

The blockers for everything else:

- **Clipping and scrolling**: per-draw-call scissor stack in `DrawList` (the `scissor` field exists but is never set; render pass must split into scissored segments). Add a `ScrollRegion` widget: wheel-scrollable, clipped, with scroll offset state kept in a `HashMap<String, f32>` on the shell. Panels, trees, tables, and text editors all need this.
- **Interaction state machine** in `input.rs`: per-frame `hovered_id` (hit test on pointer move), `DragState` (press origin, current, target id) driving slider/ring/stepper/resize drags, and a proper focus model. Extend the keydown listener to full key events (Backspace, Enter, Escape, arrows, modifiers) instead of first-char-only.
- **Text editing**: cursor position, insert/delete, commit-on-Enter/blur dispatching the control's `on_change` command with `{ value }` args (matching React `dispatchUiCommand` arg-merge semantics).
- **Text wrapping** in `text.rs`: `measure_text` + `draw_text_wrapped` with word wrap for panel text and tree descriptions.
- **Icon atlas**: rasterize `ui/asset/icon/*.svg` in JS at boot via offscreen Canvas2D (`Image` + `drawImage`), pack into an RGBA atlas, upload once through a new `upload_icon_atlas`. Rust side gets `icon_uv(icon_id)` lookup. Icons unlock navbar/footer/tab/tree/button parity.
- **Overlay layer**: a second render list drawn after the main UI (same pipelines) for dropdowns, context menus, and drag ghosts, with click-away dismiss.

## Phase 2 — Widgets to full interactivity (`ui/wgpu/rs/widgets.rs`)

- Slider and Ring: drag to change, dispatch `{ value }` / `{ t }` on release (and live while dragging).
- NumberStepper: minus/input/plus segments, `on_delta` and `on_absolute`, "Mixed" state when `!uniform`.
- Select and IconSelect: open dropdown in the overlay layer, item hover, dispatch on pick.
- Toggle: pressed visual state + icon.
- Input: focus ring, cursor, editing per Phase 1.
- Section: collapsible with chevron, honor `default_open`.
- Tree: icons, selection (`selected_ids`) and highlight styling, collapse/expand per item, inline controls on rows, `selection_change` command dispatch, scroll region, hover row background.
- Button: render `icon_id` from the icon atlas.

## Phase 3 — Shell chrome (`framework/renderer/wgpu/rs/shell.rs`)

- **Navbar**: logo mark + app label, history back/forward/up button group, breadcrumb, filler, search/find toggles, left/right panel toggles (actually toggling `left_panel_open`/`right_panel_open`), theme select. Same ordering as `os-shell.tsx` 1244-1344.
- **Footer**: new `render_footer` at `theme.navbar_height`; app label + icon, studio Undo/Redo/Checkpoint dispatching commands. Window body height accounts for it.
- **Floating side panels**: draw as floating rounded panels inset by a spacing token over the canvas (not docked columns), matching `SidePanel` in [ui/js/react/index.tsx](ui/js/react/index.tsx) 14053-14153. Resize handle strip on the inner edge with drag (200-600 clamp). Right panel becomes a real tab switcher (active tab state + hit targets) instead of stacking all tabs.
- **Window area**: window-kind tab bar when an app has multiple window kinds; clicking switches `active_window_id`. Scroll surface for window content.
- **Context menu**: right-click opens overlay menu; populated per surface (scene hosts contribute items; default: window/panel actions). Dismiss on click-away/Escape.
- **Keyboard shortcuts**: Ctrl/Cmd+B and Ctrl/Cmd+Shift+B panel toggles; app manifest keybindings dispatching commands.
- Wire the orphaned `handle_pointer`/`handle_world3d_input` shell methods into the `lib.rs` runtime (single input path).

## Phase 4 — Component scenes (`framework/renderer/wgpu/rs/scenes.rs`)

- **raster**: upload decoded RGBA pixels to a wgpu texture and draw as a textured quad (new textured-quad instance kind in `draw.rs`); click dispatches `rasterClick`.
- **table**: header row, scrollable body, row hover + click -> `selectRow`, column separators, alternating row tint.
- **node-graph**: pan (drag pane) and zoom (wheel) into a viewport transform, node bodies with port dots, node drag -> `moveMediaNode`, node click -> `selectInstance`/`selectNode`, edge rendering with bezier-ish polylines, port-to-port connect drag -> `connectMediaPorts`.
- **flow-canvas**: parse `fixture_json` fully (nodes, wires, widgets), camera pan/zoom, selection -> `setMediaNodeSelection`, double-click -> `openInstance`, context menu items when `editable`.
- **canvas-2d**: pan/zoom dispatching `canvasPointerDown/Move/Up` and `canvasWheel` with surface args (plugin owns camera, matching React).
- **text-editor**: line rendering with scroll, visible cursor, click-to-position, text editing dispatching `setDocument`.
- **virtualFileSystem**: schema-driven columns, row selection -> `selectRows`, double-click URI routing (`os://instance/` -> `openInstance`, etc.), hover row.
- **world-3d**: already complete; keep as is.

## Phase 5 — Verify end to end

- Extend [.repo tickets E2E script](.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts): per plugin assert boot, zero console errors/warnings (including WebGPU validation), canvas paint, and screenshot capture for manual chrome inspection.
- Add interaction smoke: click a panel tab, toggle a panel, open a select dropdown via Playwright coordinates on `s` and one scene-heavy plugin (`flow`, `cad`).
- `cargo test -p ui_wgpu` for layout/text/scissor units; rebuild wasm; run full 25-plugin suite.

## Notes

- All work stays in the existing files (regions per repo rules); no new script files.
- Search/find palettes (Mod+P/Mod+F) and named-layout management are included as chrome items but implemented as overlay list pickers (same overlay primitive as dropdowns) rather than fuzzy-search ports of Fuse.js — exact-substring filter is the Rust-side equivalent.
- Theme values already mirrored in [ui/wgpu/rs/theme.rs](ui/wgpu/rs/theme.rs); extend with footer height, panel inset, overlay colors.