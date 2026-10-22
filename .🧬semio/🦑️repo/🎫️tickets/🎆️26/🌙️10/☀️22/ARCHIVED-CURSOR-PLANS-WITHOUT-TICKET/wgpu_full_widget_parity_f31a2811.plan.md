---
name: Wgpu Full Widget Parity
overview: "Bring every framework-used UI component in the wgpu renderer to full visual and interaction parity with React: all 16 declarative widgets (Section collapse, working Select/Input/Slider/Ring/Stepper, icons on Button/Toggle, editable Vec3, Field/KeyValue layout), remaining Tree gaps, and the ComponentScene Table and VirtualFileSystem specializations (row chrome, selection, document, icons, multi-select, double-click)."
todos:
 - id: interpreter-fields
   content: "Carry all dropped wire fields through interpreter.rs and widget structs: button/toggle icon_id, input kind/commit/on_change, slider/stepper step/uniform/on_delta, ring disabled, section default_open"
   status: completed
 - id: section-select-input
   content: Wire Section collapse (hit branch + default_open + atlas chevrons), Select dropdown open/close/pick with on_change dispatch, Input focus_input seeding + commit dispatch
   status: completed
 - id: slider-ring-stepper
   content: Implement Slider/Ring drag-to-value with step and on_change dispatch, NumberStepper on_delta minus/plus + editable center + bordered 3-segment chrome
   status: completed
 - id: misc-widgets
   content: Button icon_id, Toggle icon + pressed args, editable Vec3 triple input, KeyValue grid alignment, Field gap tokens, Text wrapping, Separator measure, IconSelect icon rendering
   status: completed
 - id: tree-leftovers
   content: "Tree: skip is_hidden rows, content-driven measure width, hovered action labels"
   status: completed
 - id: table-parity
   content: "Table scene: remove zebra + column separators, hairline row borders, React row heights/padding, empty message, single-click double-fire guard"
   status: completed
 - id: vfs-parity
   content: "VFS scene: expand/collapse document with visibility filtering + chevrons, glyph icons, schema column labels + 32% name column, typed descriptor formatting, shift/meta multi-select, selected text emphasis, drag affordance, empty message"
   status: completed
 - id: text-editor-commands
   content: "Text editor scene: rename setDocument to textEdit, add submit (Cmd+Enter) and formatDocument (Cmd+S) dispatch"
   status: completed
 - id: verify-all
   content: Build native+wasm, rebuild bindings, run E2E for s/forms/draw/flow, screenshot-diff vs React shell
   status: completed
isProject: false
---

# Wgpu Full Widget Parity

## Scope

The framework wire protocol ([framework/core/rs/ui.rs](framework/core/rs/ui.rs)) has 17 `UiNode` variants + 10 `UiControlNode` variants. Tables and the virtual file system are not `UiNode` variants - they are `ComponentScene` payloads (`TableScene`, `VirtualFileSystemScene`) rendered by [framework/renderer/wgpu/rs/scenes.rs](framework/renderer/wgpu/rs/scenes.rs). This plan closes every remaining wgpu gap versus the React interpreter ([framework/renderer/react/ui-interpreter.tsx](framework/renderer/react/ui-interpreter.tsx)) for components the framework actually exercises.

## 1. Interpreter: stop dropping wire fields

`ui_node_to_widget`/`control_to_widget` in [framework/renderer/wgpu/rs/interpreter.rs](framework/renderer/wgpu/rs/interpreter.rs) currently drop:

- `UiButtonNode.icon_id` (Button renders icon by looking up the _label_ string in the atlas - wrong)
- `UiToggleNode.icon_id` (Toggle renders no icon at all; React always shows icon + optional text)
- `UiInputNode.input_kind`, `commit`, `on_change`
- `UiSliderNode.step`, `UiNumberStepperNode.step`/`uniform`/`on_delta`
- `UiRingNode.disabled`, `UiSectionNode.default_open`

Extend the matching `WidgetNode`/`ControlNode` variants in [ui/wgpu/rs/widgets.rs](ui/wgpu/rs/widgets.rs) to carry these fields and map them 1:1.

## 2. Widget-level parity fixes (`ui/wgpu/rs/widgets.rs` + shell wiring)

- **Section**: collapse is dead - header registers hit id `section.{id}` but `handle_shell_hit` in [framework/renderer/wgpu/rs/shell.rs](framework/renderer/wgpu/rs/shell.rs) only matches `section.chevron.*`. Add a `section.` branch (or emit `section.chevron.{id}`), seed `collapsed_sections` from `default_open`, and replace unicode `"▸️"/"▾️"` with the `chevron-right`/`chevron-down` atlas icons used by tree sections.
- **Select**: dropdown never opens because `open_selects` is never written. Toggle `open_selects[id]` on `HitKind::Select` click in `handle_shell_hit`, close on outside click/escape (reuse `dismiss_overlays`), and dispatch `on_change` with `{ value }` args when a `DropdownItem` is clicked. Render the open dropdown into the overlay layer so it isn't clipped by panel scissors.
- **Input**: on `HitKind::Input` click call `input.focus_input(id, current_value)` (shell currently only sets `focused_id`, so the buffer starts empty). On Enter/blur dispatch the input's `on_change`/`commit` command with `{ value: text_buffer }`.
- **Slider / Ring**: begin an `InputState` drag when pressing a `HitKind::Slider` hit (same pattern as `dock.split.` / panel resize in `handle_pointer_button`), map drag position to value (honoring `step` for Slider), and dispatch `on_change` with `{ value }` on release (and throttled during drag to match React's continuous updates).
- **NumberStepper**: minus/plus must dispatch `on_delta` with `{ delta: -step / +step }` (both currently fire the same `on_absolute` with no args); center segment focuses an input and commits via `on_absolute { value }`. Render as bordered three-segment group matching React's `Stepper` (`w-medium` button segments with hairline dividers).
- **Toggle**: render `icon_id` icon (falling back to text-only), dispatch `on_change` with `{ pressed: !pressed }` args.
- **Button**: use the carried `icon_id` for the icon slot instead of the label-as-icon hack.
- **Vec3**: replace the three read-only text rows with three focusable numeric inputs (reuse `render_input` per axis), dispatching `on_change` with `{ value: [x,y,z] }` on commit - matching React's 3-column input grid.
- **KeyValue**: align to React's two-column grid: muted label column sized to the longest label (not a fixed 40% split), tabular values, `theme`-token row height.
- **Field**: keep stacked layout (React's interpreter Field is also stacked) but use theme gap tokens instead of the fixed 18px offset, and size the measure from label + child rather than a square.
- **Text**: route through the existing `wrap_text` helper so long text wraps instead of overflowing; `emphasize` keeps current handling.
- **Separator**: fix measure to `(available width, 1px + margin)` instead of `(control_height, 1.0)`.
- **IconSelect**: render the current `value` as an icon from the atlas (fall back to the raw string) inside a bordered control; full picker UI is out of scope (only used by puzzle plugins, not general framework chrome) - note as known limitation.
- **Tree** (remaining gaps from the previous pass): skip `is_hidden` rows entirely instead of reserving 24px; measure width from content instead of fixed 200; show action `label` as tooltip-substitute text next to the icon when the row is hovered.

## 3. Table scene parity (`render_table` in `framework/renderer/wgpu/rs/scenes.rs:662-763`)

React `Table` ([ui/js/react/index.tsx](ui/js/react/index.tsx) ~18679-18912) has: sticky header ~32px (`h-large`), body rows ~24px (`h-medium`) with bottom hairline borders, **no zebra striping**, **no vertical column separators**, hover fill, selection fill + emphasized text, `px-single` cell padding, double-click support, empty message. Wgpu currently draws zebra stripes, full-height column separators, 24/22px rows, no selection, no double-click, no empty message.

- Remove zebra striping and vertical column separators; draw a hairline bottom border per row.
- Header height `theme.control_height * 1.33` (~32px at h-large), body rows `theme.control_height` (~24px); cell padding from `theme.padding_standard`.
- Add selection support (fill `theme.selected` + `active_foreground` text) - `TableScene` has no selected-ids field on the wire, so match React's `TableHost` which also passes none; keep the rendering path ready but driven by hover/selection state only where the scene provides it.
- Add table row double-click to `hit_double_click_target`/`double_click_command` (dispatching `selectRow` twice is fine only if React does the same - React's `TableHost` wires no `onRowDoubleClick`, so just ensure single-click doesn't double-fire).
- Draw `emptyMessage`-style centered muted text when rows are empty (React shows an empty tbody row).

## 4. VirtualFileSystem scene parity (`render_vfs` in `scenes.rs:1189-1312`)

React `VirtualFileSystem` ([ui/js/react/index.tsx](ui/js/react/index.tsx) ~19396-19507) is a hierarchical Table: expand/collapse chevron buttons per folder row, visibility filtering by `expandedIds`, 14px per-node glyph icons chosen from schema/kind/extension (`VirtualFileSystemNodeGlyph`, ~19222-19298), indent `level * 14px`, Name column ~32% width, schema-driven descriptor columns with typed rendering (text / formatted time), shift/meta multi-select, selected rows with emphasized text, drag-drop. Wgpu today renders a flat list with path-derived indent only, zebra stripes, raw column ids as headers, single-select, no chevrons, no icons.

- **Document**: maintain an `expanded_ids` set in scene state (client-side, like React - expand/collapse is not a wire command); build visible rows by filtering children of collapsed nodes (port `buildVirtualFileSystemVisibleRows` logic); render a clickable chevron (`chevron-right`/`chevron-down`) per row that has children, indent by `level * 14`.
- **Icons**: port the glyph mapping (schema `iconId` → kind → file-extension fallback) and draw a 14px atlas icon before the name.
- **Columns**: use schema column _labels_ for headers (not raw ids), Name column fixed ~32% width, descriptor columns share the rest; format time-kind descriptor values like React does.
- **Selection**: selected fill + emphasized text; multi-select with shift (range from anchor) and meta/ctrl (additive) using `InputState.modifiers`, dispatching `selectRows { surfaceId, ids }` with the full computed set (command name/args already match React).
- **Row chrome**: drop zebra, hairline bottom borders, `theme.control_height` rows.
- **Click semantics**: suppress the single-click `selectRows` when a double-click fires (currently both fire); double-click command routing (`openInstance`/`exportMedia`/`navigateVirtualFileSystemNode`) already matches React and stays.
- **Drag-drop**: `dragDropEnabled` rows become drag sources reusing the tree-drag pipeline built previously (drop dispatch mirrors whatever React's Table drag-drop dispatches - React's host currently dispatches nothing on drop, so wgpu only needs the visual drag affordance).
- **Empty message**: use `scene.empty_message` with React's default "No file system nodes".

## 5. Text editor command parity (`render_text_editor` in `scenes.rs:1389-1563`)

Wgpu commits edits via `setDocument` on Enter/Escape; React's `TextEditorHost` dispatches `textEdit` on change, `textSelect` on selection, `formatDocument` on Cmd+S, `submit` on Cmd+Enter. Rename wgpu's commit command to `textEdit` with the same args shape, add `submit` on Cmd+Enter and `formatDocument` on Cmd+S. Full WASM-highlighting parity stays out of scope; token-color rendering already exists.

## 6. Verification

- `cargo build` native + wasm32 for `ui_wgpu` and `semio-framework-renderer-wgpu`; run native dock/tree unit tests.
- Rebuild wasm bindings (`bun ./📜️script.ts wasm` in `framework/renderer/wgpu`).
- E2E via the existing harness for plugins covering each component class: `s` (VFS + catalogue tree + stack/inputs), `vcs` or `forms` (TableScene), `draw` (Slider + Field controls), `flow` (text editor + node graph).
- Screenshot-diff wgpu vs React shell for `s` and `forms` to confirm table/VFS/tree/control chrome parity.
