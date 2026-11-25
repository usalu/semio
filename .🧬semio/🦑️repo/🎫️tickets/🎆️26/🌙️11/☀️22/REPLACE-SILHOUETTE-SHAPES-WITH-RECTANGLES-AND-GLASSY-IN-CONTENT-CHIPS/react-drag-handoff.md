# React drag handoff: UI law 5 in the design system

Law (second ticket, section 5): a dragged element leaves its place (hidden, keeps its room) and travels as one floating rectangle of menu-level glass with the chip tint (`data-glass-chip`) on a layer above everything; while it travels every place it can be dropped on is tinted 10 %, the one under the pointer 32 %.

All paths below are relative to `🧰️framework/🔨️modules/🖱️ui/`.

## One shared mechanism: `🔨️modules/🫳️pointer-drag/`

| File | What |
| --- | --- |
| `places.ts` | Pure policy: `dropPlaceAt` (smallest place under the point, later wins ties), `dropPlaceStates`, `dragGhostPosition`/`DRAG_GHOST_OFFSET`, `dragStarts`/`DRAG_START_THRESHOLD_PX` (4), `dragCancelKey` (Escape), `insertSlot` (rectangle over half of the next tab, inside the row), `treeDropBand` (upper/lower quarter, or whole row), `DROP_TINT_PERCENT` (10/32). |
| `places.schema.json`, `places.cases.json` | Language-neutral cases (Ajv-validated); `places.test.ts` checks them and the hit rule against a lodash oracle. |
| `ghost.tsx` | `DragGhost` (portal, `data-drag-ghost`, `data-level="menu"`, `data-glass-chip`, `ui-glass`, inert, aria-hidden), `useDragActive` (page mark `data-drag-active`, ref-counted), `useDragEscape`, `dropPlaceAttributes`, `dragSourceAttributes` (`data-drag-hidden`), `beginNativeDragGhost` (native HTML5 drags: blank drag image, copy follows `dragover`, source hidden after the browser took the drag, ends on `dragend`/`drop`, Escape = native cancel), `useNativeDropOver`, `useDndGhost` + `dragLabelOf` (dnd-kit drags). |
| `🟦️.tsx` | Re-exports both; `usePointerDrag` unchanged. Package export `./modules/pointer-drag` added to `package.json`. |
| `layer.test.tsx` | jsdom tests of ghost, marks, hooks, native/dnd-kit detach, and of the CSS (10 %/32 % tints, hidden source, layer above `--z-tutorial`, no animation so reduced motion is a no-op, forced-colours outlines). |
| `🎨️styling/🖌️ui/🎨️.css` | New block after the chip veil: `[data-drag-hidden]`, `[data-drag-ghost]`, `:root[data-drag-active] [data-drop-place]` (+ `[data-drop-active]`), forced-colours outlines (`CanvasText` dashed / `Highlight`). The tint is an image laid over the place's own fill. |
| `🔨️modules/⚓️panel-anchor/` | `anchorPlaceRect`, `anchorPlaces` (3x3 grid cells of the eight anchors) with `places.schema.json`, `places.cases.json`, `places.test.ts`. |

## Drags brought into line

| Drag | Change |
| --- | --- |
| Panel tabs (`🎯️targets/⚛️react/🧲️panel-dock`, `🧱️elements/🧭️PanelTabBar`) | `DockDragChip` replaced by `DragGhost`; Escape via `useDragEscape`, plus `pointercancel`; dragged tab hidden in place (no `opacity-40`); tab rows are places; branches are nest places (active when the pointer nests); the old 2 px insert line is a measured rectangle inside the row (`PanelTabInsertSlot`, half of the next tab). Tree units onto leaf tabs: leaf tabs are places, active on `dragenter`. |
| Panel (`🧱️elements/🖼️Panel`) | Tree-unit headers: native ghost + place attributes; empty dock zone is a place. |
| Canvas dock (`🧱️elements/🎨️Canvas`) | `ModeDockDragPreview` (rounded, shadow, content copy) deleted, replaced by a label+icon `DragGhost`; new `🧲️drop-places` shows every stack as a place and the half the drop takes as the active rectangle (`data-slot="mode-dock-drop-indicator"`, as the OS dock acceptance reads it); tab rows are places and the slot in the row is the active place; `computeTabInsertPreview` (line geometry) and `data-drag-source` filter removed; `drag-preview` keeps only `modeDockTabInsertPreviewClass` (no fill of its own). Escape via `useDragEscape`, plus `pointercancel`. |
| Tree rows and sections (`🧱️elements/🌳️Tree`) | Native drags call `beginNativeDragGhost` (the old `puzzle3d` shadow/rounded ghost is gone); `isDropReady: boolean` became `dropPlace: DropPlaceState`; source hidden by attribute, not `opacity-40`; drop preview line/box became `treeDropBand` rectangle as the active place; rows are places only where the row-reorder MIME is accepted. dnd-kit `SortableTreeItems`: no row shifts (`keepPlaces`), ghost via `useDndGhost`, `isOver` is the active place. |
| Table (`🧱️elements/📊️Table`) | dnd-kit rows no longer translate; ghost + places; native row drags get the ghost; unused `renderDragOverlay` option removed. |
| Window drag: `Pane` (`🧱️elements/📌️WindowPane`) | The pane no longer re-anchors live: it is hidden in place, a ghost (icon + label) travels, the eight anchor places show over the `PaneHost` (portal `pane-drop-places`), the active one is the nearest anchor; `onAnchorChange` fires once on drop; Escape/`pointercancel` abandon; 4 px threshold. |
| Window drag: Introduction info box (`🎯️targets/⚛️react/🎓️introduction`) | It already is the one floating rectangle (no drop places); Escape now returns it to where it was. |
| Class names | `dropZoneReady*Class` removed from `🔨️modules/🎨️chrome-class-names` and from both export lists; `drop-nest`, `insert-preview` classes gone. |

Old shapes that are gone: 2 px insert line, `rounded-full` bar, `border-2` nest box, `bg-accent/20` slot fill, `shadow-lg` ghost.

## Tests

Written first (cases/tests before the code that satisfies them), run one process at a time with `lockf -k /private/tmp/claude-501/semio-vitest.lock node react-drag-verify.ts --run <filter>` (copy of the recipe in `verify-ui.ts` of the second ticket, vitest jsdom, `maxWorkers: 1`). New files, all added to `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`:
`🔨️modules/🫳️pointer-drag/places.test.ts`, `layer.test.tsx`, `🔨️modules/⚓️panel-anchor/places.test.ts`, `🧱️elements/🧭️PanelTabBar/drag.test.tsx`, `🧱️elements/🎨️Canvas/🧪️tests/drag.test.tsx`, `🧱️elements/🌳️Tree/🧪️tests/drag-places.test.tsx`, `🧱️elements/📊️Table/drag.test.tsx`. Updated: omnibus tests of Pane drag (now drop-driven + Escape/cancel case), Introduction (Escape), removed the `computeTabInsertPreview` case.

Results (exact):

- Targeted after the last edit: `--run pointer-drag Table/ PanelTabBar Canvas Tree/ panel-anchor`: 13 files, 210 tests, all pass.
- Pane and Introduction cases: `--run owned-locale-detector-retirement -t "Pane"` 55 passed; `-t "info box"` 2 passed.
- Full config run (83 files, 1699 tests, before the final type-only edits): 1691 passed, 8 failed, none in a drag path: `Layout` (4: navbar/footer seam hover, tinted bars scroll, canvas full width timeout), `Icons` (1: theme alias glyph), omnibus `Level Context` (2: Navbar/Layout base level) and `Panel puts every tab row ... one top chip` (passes alone; it asserts WindowChrome chip structure that the concurrent WindowFrame lane is changing). Not touched by this change; the same two Level Context failures existed in the first omnibus run.
- Type check, scoped tsconfig over all changed files and tests (`tsc --noEmit`): clean after fixing the dnd-kit event typing.
- Not run: cargo, browser/visual check, Playwright, the OS dock browser acceptance.

## Open

1. Canvas keeps the layout contract of the shared stack-drop vectors (`🎯️stack-drop-destination`: drop paths address the layout with the dragged tab/stack extracted, `modeDockOutLayout`). So a Canvas source is removed from the layout, not hidden with its room kept; the remaining tabs close up while dragging. Honouring "keeps its room" there needs a contract change in `dropWindow`/`dropStack` addressing (and the vectors, shared with wgpu).
2. wgpu/Rust twins are untouched: `🧱️elements/🌳️Tree/🧪️tests/🧫️fixtures/🧲️drag-drop/🔣️.json` still states `line`/`box` previews (`tree_drop_preview`); the new React bands are only in `places.cases.json` (`treeBands`). The wgpu Dock/Tree drags and `🎯️targets/🧊️wgpu` ghost need the same law.
3. New helpers are not re-exported from `ui-react` (`📥️source`, `🗿️framework-exports`); consumers outside this package import `@semio-tech/ui/modules/pointer-drag`.
4. `DragHandle`-less keyboard alternatives did not exist for tab, pane or row moves and none were added (arrow-key focus in the Canvas tab bar is unchanged); a keyboard move command for panes and tabs is open.
5. The course `🤏️drag` keeps its own path (grab offset, edge scroll); it could use `places.ts` for the hit rule and tints.
6. Not covered: `Diagram` node drag, `Slider`, `Ring`, `Scene` gumball and `Resizable` handles are value/position manipulation, not drag-and-drop, and were left alone.
7. Tinted places use a background image over the place's own fill; a place that has its own `background-image` (chip veil) would lose it while tinted. No such place exists today.
8. No visual browser check was made; the 10 %/32 % tints, ghost layer and forced-colours outlines are asserted by tests only.
