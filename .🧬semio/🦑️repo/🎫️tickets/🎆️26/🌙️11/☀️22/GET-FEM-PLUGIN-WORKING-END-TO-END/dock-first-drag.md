# First Display-panel drag into a dock zone ignored (fixed)

## Cause
Display "Windows" leaves (`dragData` = window-template MIME) get the drag role `transfer` only. `TreeDataItemView` (`Tree/🪢️views`) passed the pointer-palette gesture (`buildPalettePointerProps(...).onPointerDown`) only as `transferPointerDown`, which `renderTreeDragHandles` wires to the small "move" `DragHandle`. The row itself has `dragInitiation="handle"`, so it is `draggable="false"` and had no `onPointerDown`, although it already carried `touch-none` (the class meant for a surface pointer drag).
Pressing the label and dragging therefore started nothing; it only selected text across the page. The "identical second drag" then worked by accident: the press landed on the selected label text, the browser started a native text drag, `dragstart` bubbled to the row's `onDragStart`, which filled `dataTransfer` from `item.dragData`, and the mode's native `drop` handler completed the drop. Probe evidence (events on window, 3D 6087):
- drag 1: `pointerdown, moves x11, pointerup` only. Template session was `null` and `windowTemplatePointerDragRef.active` was false at `pointerup`.
- drag 2: `pointerdown, dragstart (SPAN tree-label), pointercancel, drop`; zone resolved to `split right` of `[]`.
Dock-zone registration, hit testing and layout were fine: both paths resolve the same zone.

## Fix (TS only)
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🪢️views/🟦️.tsx`: transfer-only rows (`transfer` role, no `sort` role) also pass the pointer-palette `onPointerDown` to the row (`surfaceTransferPointerDown`). A press that does not move 6 px stays a click; the handle still works (its handler stops propagation, so the gesture starts once).
- `…/🌳️Tree/🟦️.tsx`: the gesture ignores presses inside `input, select, textarea`, header actions and branch navigation (`PALETTE_SURFACE_DRAG_EXCLUDED_SELECTOR`), so nested controls keep their behaviour. Pointer and touch use the same pointer-event path (`touch-none` is already on the row).
- Keyboard: unchanged. Dock-by-keyboard is not part of this path (the Display leaf has no key activation and no keyboard drag existed before); the row keeps its focus/Enter behaviour and the click is still delivered. A keyboard "open here" action for Display leaves would be a separate feature.

## Tests
`…/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`, describe "the first palette drag from a row surface": label press + move + release starts the palette drag once and ends it (third party `@testing-library/user-event` pointer sequence), a no-move press stays a click, the move handle still starts exactly once (`fireEvent`). Verified the first test fails without the fix (0 begins) and passes with it. Tree vitest run (`run-ui-vitest.sh Tree/`): 3 files, 83 tests green.

## Live verification
Private copy of the harness in `C:\t\dd` with the retry reduced to one attempt:
- before: `3d.win.reopen-display` FAIL (`Results tabs=0`, attempts=1).
- after: 3d PASS and 2d PASS, both `drag attempts=1`, Results tab back at x 803.

## Restart
No vite restart needed: the dev server's stat-guard serves the changed modules on the next page load (verified on 6086/6087 without restarting anything). Reload open tabs only.

## Harness note
The retry in `matrix-v3/checks-core.mjs` `win.reopen-display` (`attempts < 3` loop) can be removed; one drag now passes.
