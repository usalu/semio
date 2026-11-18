# Presentation and CAD plugin shared-border fixes

Source: `audit-products.md` (presentation + `✏️s/🔌️plugins/**`). Findings P1, P2, C1 verified and fixed. All others in scope are rated low or "reviewed, no action", so skipped.

## Fixed
- **P1 PDF pager** (`🎭️morph-views/🟦️.tsx`): the nav is now a `SharedBorderChipGroup` (frame, compositor-wired) and both buttons are `data-shared-border-cell`. CSS (`🔲️interaction-handles-and-controls/🎨️.css`): gap 0, the frame owns the hairline outline and radius, per-button border and hover `border-color` removed, outer radius moved onto first/last cell.
- **P2 reset/enlarge actions** (`🧲️interactive-disposition/🟦️.tsx`): same group/cell treatment. Reset is conditional; the cell mechanism's last-cell rule leaves a single enlarge chip without a divider, and its frame supplies the outline.
- **C1 CAD REPL aside** (`✏️s/🔌️plugins/📐️cad/.../📺️renderer/🎛️repl/🟦️.tsx`): `border-l-0 border-t` override removed. The aside now carries `data-shared-border-frame` + `data-shared-border-edges="top right bottom left"` and `useSharedBorderFrameRef`, so the compositor attaches/transparents the host-coincident sides.

New shared helper: `🎤️presentation/🎯️targets/⚛️react/🔨️modules/🔗️shared-border-chips/🟦️.tsx` (`SharedBorderChipGroup`, `SHARED_BORDER_CELL`). ButtonGroup/ActionGroup were not reused: they require registered control ids, labels, hotkey badges and `h-medium` sizing, which do not fit the tiny icon chips in a scaled reveal slide. The helper uses the same frame/cell attributes and hook.

## Skipped
- P3, P4: reviewed clean in the audit.
- C2 (redundant `border` + `borderNormalClass` on Select triggers): low, no seam.

## Tests
- `🧪️tests/♻️enlarge-and-reset-interaction` and `🧪️tests/📄️pdf-and-figure-interaction` extended: frame and cell markers, CSS gap 0, frame owns border, no per-button border. Full presentation-react suite: 15 files, 140 tests pass (run via `🎚️config/🟦️.ts` with vitest directly).
- New CAD suite `📺️renderer/🧪️tests/🧱️repl-aside-frame/🟦️.ts` (registered as `registerTests3` in the renderer in-source block): frame attributes and ref present, no per-side border utilities. Passed through the ticket runner `cad-aside-frame.test.ts` + `vitest-cad-renderer.config.ts`; the full in-source renderer suite cannot run here because the spatial-kernel wasm bindings are not built.

## Not verified
No visual check in a browser; the compositor SVG overlay inside a CSS-scaled reveal slide is untested at runtime.
