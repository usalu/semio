# Sweep: Framework Elements onto the State Silhouette Mechanism

Scope: inventory sections 2 (P1-P10) and 3 (E1-E41). Root: `🧰️framework/🔨️modules/🖱️ui`. Lines are approximate (shared worktree).

## Shared presets (section 2)

| Row | Done |
|---|---|
| P1 | `🔨️modules/🖱️interaction-presentation/🟦️.ts`: `interactiveActiveBorderClass` deleted (and its export in `🎯️targets/⚛️react/📤️adapter-exports` and the `ui-react` source barrel). `interactiveActiveFillClass` is fill and text only. |
| P2 | Same file: `hoverExcludingHandleActiveBorderClass` deleted. |
| P3 | `interactiveOnClass`: `data-[state=on]:border-active-base` and `:hover:` removed. |
| P4 | `interactiveTabActiveClass`: both border classes removed. |
| P5 | `🎛️chrome-control-presentation`: `chromeControlTabActiveClass` had no consumer; deleted. |
| P6 | `📝️form-control-presentation`: `formControlFocusBorderClass` renamed `formControlResetClass` = `outline-none focus-visible:ring-0 shadow-none` + transition. Renamed in Input, Textarea, Select, ActionGroup, chrome-control-presentation, adapter exports, barrel. Focus/open/invalid colours dropped. |
| P7 | `📋️menu-item-presentation`: `data-[selected=true]:border-active-base` removed. |
| P8 | Kept (fill and text). |
| P9 | `🎨️chrome-class-names`: static `shadow-lg`, `border-t` kept. `panelResizeEdgeAccentClass` (`border-l-accent`/`border-r-accent`, a drag state line) deleted with its exports; Window passes `data-shared-border-state="active"` to the measures pane stack while dragging instead (`🪟️Window` ~line 377). |
| P10 | `🎯️targets/⚛️react/🎨️chrome-classes`: static only, unchanged. |

## Elements (section 3)

| Row | Done |
|---|---|
| E1/E2 | `📑️Tabs`: every trigger is a frame; edges `""` when unselected, all four when selected; `focus-visible:border-active-base` and `data-[state=active]:shadow-sm` removed. |
| E3 | `🪜️Stepper` group: `focus-within:border-accent` and the border-color transition removed. |
| E4/E6 | Stepper buttons: fill-only focus kept (`focus-visible:bg-muted`). |
| E5 | Stepper input: only removals (`outline-none`, `focus-visible:border-0`); unchanged. |
| E7 | `🖱️ContextMenu/🔘️trigger`: outline classes removed, `sharedBorderlessBoxProps`. |
| E8 | `📜️Scrollable`: ring and outline removed; box props only when a consumer passes `tabIndex >= 0` (a box on every scroller would be hoverable or scroll the overlay). |
| E9 | `↔️Resizable` handle: ring removed, `sharedBorderlessBoxProps`. Handle is 0px wide, so its silhouette is a line. |
| E10 | `🎚️Slider` thumb: single closed `focus-visible:ring-2 ring-accent`, `ring-offset-2` set to 0, so no spacer plus ring pair. |
| E11 | Slider dial: kept its own closed invalid ring (circle, no seam). |
| E12 | Slider value: outline removed, `sharedBorderlessBoxProps`. |
| E13 | `💬️Dialog` close: ring classes removed, `sharedBorderlessBoxProps`. |
| E14 | `🥞️LayeredOverview/🟢️overview-button`: hover border and focus ring removed; stays a cell. |
| E15 | `💡️hints`: ring removed; already a frame with all edges. |
| E16/E17 | `✏️Input`: `aria-invalid:border-destructive` and `aria-invalid:ring-destructive/20` removed (see needs). |
| E18 | `⌨️Command` item: `sharedBorderlessBoxProps`; `aria-selected` marks it. |
| E19/E20 | Removals only; unchanged. |
| E21 | `🔽️Select` item box props; trigger gets `data-shared-border-state="active"` while open (replaces `data-[state=open]:border-accent`). |
| E22 | `🕸️Diagram`: `DIAGRAM_KEYBOARD_FOCUS_CLASS` replaced by `DIAGRAM_KEYBOARD_FOCUS_ATTRIBUTES` (box props, `data-shared-border-state="focus"`, `data-keyboard-focused`) set on the focused node's `domAttributes`; exports and test updated. |
| E23 | Tree row loading/waiting: classes kept as the compositor marker, but the `-active` ring modifier is no longer requested. |
| E24 | Tree: all 7 `role="treeitem"` shells (`🍃️item`, `↕️sortable-item`) are borderless boxes; fill stays. |
| E25 | Drag ghost kept. |
| E26 | `📊️Table`: clickable rows (both row components) are borderless interactive boxes, `data-active` when selected; drop `!border-b-[...]` replaced by `data-shared-border-state="active"`. |
| E27 | `⚡️ActionGroup`: dropdown options are boxes marked `data-active`; items are cells already. `outline-none` kept (native suppression). |
| E28/E29 | Canvas app tab already carries `data-state="on"`; Engagement options now get `data-state="on"`; fills stay. |
| E30/E31 | `shadow-sm` removed from Engagement and WindowSearch session (already `data-shared-border-state`). |
| E32 | WindowSearch results: `data-active` already set; fill stays. |
| E33 | `🧭️PanelTabBar`: drop target `!border-accent` removed, `data-shared-border-state="active"` set. |
| E34 | Window quick actions already carry `data-state`; fill stays. |
| E35 | Consistency pass done (presets above). `🎛️ToggleGroup` root `data-state="on"` marked the whole group active; renamed `data-selection` (same bug class as the original red outline). |
| E37/E38/E39 | Window chrome markers kept; Window no longer requests the `-active` ring modifier. |
| E40 | `📋️MenuItem`: `sharedBorderlessBoxProps`. ContextMenu fusion rows (both buttons) also boxes. |
| E41 | Kept `hover:underline` (typography). |
| Extra | `🔀️Toggle` dropdown option buttons got box props so keyboard focus has a silhouette. |

Intentionally unchanged: `📨️UIDialog` destructive tone border (static, not a state), Canvas/Tree drop previews, HistoryTable avatar ring, `border-loading/waiting` markers, native Checkbox.

## Tests

- New `🧱️elements/🗂️WindowChrome/state-boxes.test.tsx`: 6 passed (presets free of state lines, box attributes on MenuItem, context trigger, Command item, Slider value, Tabs edges/state, Resizable handle, Table rows, Slider thumb single ring).
- Updated `🕸️Diagram/🧪️tests/🧩️component/⌨️keyboard-tests` for the new attributes.
- `🧱️elements` full run: 659 passed, 6 failed. Failures: Tree text-flow x3 and Layout x1 (Playwright, known), `🗂️WindowChrome/focus-seam.test.ts` x2 (assert the old `course` product CSS, owned by the products sweep). Files with 0 tests (`bun:test` imports, helper fixtures) fail as before. One Diagram force test flaked under load and passes alone.

## Open needs (for the coordinator)

1. Compositor marker: add `[data-selected="true"]` (and `aria-checked="true"`, `data-state="checked"` optional) to `activeSelector` in `🗂️WindowChrome/shared-border.ts`. MenuItem, ContextMenu rows and Tree rows use `data-selected` and otherwise show only fill when selected.
2. Compositor state for invalid: no `invalid` state exists, so Input/Textarea lose their destructive line (`aria-invalid`). Needs an `invalid` state (or a destructive tone) in `border-presentation` and the compositor.
3. CSS: `border-loading-active` / `-element` / `border-waiting-*` modifier utilities become dead once the `::after` rings go; `status-border-presentation` can drop them afterwards.
4. Compositor: confirm a frame on `<tr>` (Table rows) and on a 0px-wide separator (Resizable) paints and sizes its lazy overlay correctly; check the `window-measures-stack` receiving `data-shared-border-state` resolves on the stack element.
5. `🗂️WindowChrome/focus-seam.test.ts` expectations on `course` CSS belong to the products sweep.
