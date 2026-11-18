# Primitive gaps (🖱️ui) and Paint2dHost / hub migrations

## 🖱️ui (generic)

`🧱️elements/🪢️SharedBorderRow/🟦️.tsx` (exports also via `@semio-tech/ui-react`, component-exports + source barrel):

- `SharedBorderRow` / `SharedBorderRowCell`: `as` (`div|ul|ol|li|section|nav|form|fieldset|dl`), `ref` forwarded to the element, `tone="destructive"`.
- `SharedBorderRowLabel` (`width="fixed"|"auto"`, fixed uses `--shared-border-label-width`, default 40%): real `<label htmlFor>` in a cell.
- `SharedBorderRowField` (`label` + one native `select`/`input` incl. `type=color`/`textarea`/contentEditable inside a `<label>` cell, `data-shared-border-field`).
- `useSharedBorderFrameRef` explicitly exported from `@semio-tech/ui-react`.
- Scroller rule documented in the `SharedBorderRow` docstring: the scroller goes INSIDE the row (a nested `SharedBorderRow` / cell with `overflow-auto`), never around it. Used in Paint2dHost.
- CSS (`🎨️styling/🖌️ui/🎨️.css`, before "Shared cells use one real divider"):
  - `[data-shared-border-cell]:where(select, textarea, input:not(checkbox|radio|range|file), [contenteditable])`: no border/radius/background, stretched (a native control or contentEditable as DIRECT cell just carries `data-shared-border-cell`).
  - `[data-shared-border-field] > :where(native)`: boxless, fills the cell; color input fixed 2em.
  - `[data-tone="destructive"]`: destructive surface (10% mix), overlay segments in state `normal` use `--destructive-border`; first rule of `@layer shared-border-priority` so hover/active/focus/waiting etc. win; cell seams adjacent to a destructive cell follow it. No new compositor state (CSS only, no schema change).
- `Input` and `Textarea`: `rootClassName` / `rootStyle` on the frame root (positioned/floating inputs need no wrapper div). `Button` untouched (icon still required, no new variant).

Tests: `shared-border.test.tsx` +5 (ul/li + refs + overlay, label cell, native field/direct cells without frames + CSS, destructive tone + priority order, Input/Textarea root props).

## os

- `🔗️HubConnection/🪢️hub-rows`: `HubFieldRow` uses `SharedBorderRowLabel` (no width class); `HubRowList`/`HubRowItem` are `ul`/`li` (no `role` props); `HubSelectField` accepts trailing `children` cells.
- `🏘️SpaceBrowser`: members `<ul role=list>` -> `HubRowList` + `SharedBorderRowCell as="li"`.
- `🤖️AgentDelegations`: revoke confirmation row `tone="destructive"`.
- `🔐️HubSignIn`: reuses `HubFieldRow` (address, email, password), `HubSelectField` (+ forget button cell), `HubRowList`/`HubRowItem` (signed-in line), destructive-tone error row; test updated to the hub-field-row structure.
- `🖌️Paint2dHost/✍️editing/🟦️.tsx`: panel is one vertical `SharedBorderRow as="section"` (no own border/padding/gap, so the frame cannot double with rows); the toggle button and a nested scrolling `SharedBorderRow` body are its cells; every label+native select/input/range is a `SharedBorderRowField`; details is its own group-owner cell; buttons/paragraphs are cells (errors destructive). Native controls kept (comboboxes still driven by `fireEvent.change`). `control`/`fieldCell` constants removed. New structural test in `🖱️selection-focus`.

## Results

- 🖱️ui regression set: 608 tests passed, 0 failed (37 "No test suite found" aggregator children, expected).
- `vitest-os-rows-b.config.ts`: 4 files, 138 passed.
- Full `vitest-os.config.ts` was started but its summary line was cut off by the `head` filter (killed). The first 30 failures seen were all unrelated to hub/Paint2d/border files (Chromium/WebGPU, audio/video hosts, store fences, procedural descriptors, tree row geometry, aggregator children without suites). Hub, SpaceBrowser, AgentDelegations, HubSignIn, Paint2dHost and block-list/space-administration suites pass via `vitest-os-rows-b.config.ts`.
