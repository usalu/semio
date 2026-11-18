# os rows B: second half of products/os, CAD plugin, measure tile review

Primitive used: `SharedBorderRow` / `SharedBorderRowCell` (API in `nested-cells.md`). Native raw cells use `data-shared-border-cell`.
Tests (ticket-local `vitest-os-rows-b.config.ts`, run from the ui package): 4 files, 138 tests, all pass. Not run: tsc, the in-source CAD suite (wasm bindings), the real runner.

## Done (os, checklist items 4 and 6 to 15; first half 1 to 3 left to the other agent)

Base `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`:

- `🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx` (#15): colour swatch + hex Input are cells of one row; native swatch border dropped.
- `🕸️NodeGraph/🖼️dag-overlays/🟦️.tsx` (#14): align toolbar is a `SharedBorderRow`, six cells, no gap, no native border.
- `🛂️SpaceAdministration/🟦️.tsx` (#13): delete confirm is one vertical row (title cell, warning cell, nested horizontal row with confirm and cancel cells).
- `📌️ChromePanels/🎨️settings-theme/🟦️.tsx` (#10): both native colour inputs are row cells (native `rounded border` dropped); the alpha row is colour cell + alpha Input in one row.
- `🖋️InkCanvasHost/🪟️ink-overlays/🟦️.tsx` (#9): text overlay is one vertical row (format toolbar row of 4 cells, no gap + editor body cell); cell editor is the `Input` primitive.
- `🧩️BlockListHost/🟦️.tsx` (#8): step = vertical frame (header row cells, description cell, blocks list as an unframed nested vertical row); block = row (label cell + delete Button); `SortableRow` got `cell` prop; palette is a `SharedBorderRow` (native border dropped). Selection indicator is now `bg-accent/10` instead of `border-accent`.
- `🔐️HubSignIn/🟦️.tsx` (#7): `Input` primitives for address, email, password; hub `select` is the `Select` primitive plus Forget Button in one row; address Input + Add Button one row; submit + cancel one row.
- `✏️TextEditor/🟦️.tsx` (#6): footer buttons are `Button` primitives in one `SharedBorderRow` (no `gap-2`); discard-source is a `Button`; rename field is the `Input` primitive in a positioned wrapper.
- `🖌️Paint2dHost/✍️editing/🟦️.tsx` (#4), partial: tool grid (rows of 3), selection buttons, flip/rotate grid, colour/size, width/height, crop x/y are shared rows (native cells via `cell`/`fieldCell`, no border, no gap). Left on purpose: the floating panel's own frame and the label + native `select`/`input` fields (tests drive native comboboxes with `fireEvent.change`; converting to Radix `Select` would break them). The per-field `control` class keeps its border for those standalone fields.

Tests added or changed:
- `…/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx`: new row-structure test; the bootstrap-label test now reads the Select trigger text instead of an `<option>`.
- `…/🧪️tests/🧩️block-list-presentation/🟦️.tsx`: new test that steps, blocks and palette are shared-border rows without native border.

## Done (plugins)

Base `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/`:
- `🎛️repl/🟦️.tsx` (#5): three `SelectTrigger`s lost `rounded-md border px-single py-half` + `borderNormalClass`; unused import removed. Aside frame was already done (not touched).
- `📋️repl-panes/🟦️.tsx` (#12): triple border dropped; unused `borderNormalClass` and `cn` imports removed.

## Review: window-chrome measure tile (#16)

`🖱️ui/🎯️targets/⚛️react/🧱️window-chrome/🟦️.tsx` is outside `🧱️elements`, but I did not edit it: `windowMeasureTileClass` has no consumer in this repo (only re-exports in `⚛️react/🟦️.tsx` and the package source). The nested leaf class correctly has no border. Risk: a consumer putting a framed control into the bordered tile gets box-in-box; the fix belongs with the consumer (make the tile a `SharedBorderRow` owner or drop its `border`).

## Primitive gaps / notes

- `Input` root does not accept `style` or positioning classes, so floating inputs need a wrapper div (TextEditor rename, Ink cell editor). A `rootClassName`/`rootStyle` prop would help.
- `Input` forwards `ref` via `...props` and overrides its internal `inputRef`; fine here but fragile.
- `Button` requires `icon` and has no size variant.
- No cell notion for native `<input type="color">`, `<label>` wrappers and `contentEditable` surfaces: I used raw `data-shared-border-cell` on them. A documented `SharedBorderRowCell`-like attribute pattern for native controls (or a `ColorInput` primitive) would remove the need.
- Not done in os (belongs to the other agent): SpaceBrowser, AgentDelegations, ShellHost.
