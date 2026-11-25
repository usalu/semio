# UI red tests handoff

Root causes of the 8 failing UI vitest tests (recipes2.md section 1) and the law 4 Layout change.

| # | Test | Verdict | Fix |
| --- | --- | --- | --- |
| 1, 2 | level-context: Layout floor, standalone bands | stale (law 4: bands are panel-level glass) | tests assert `data-level="panel"`, `data-glass-chip`, `ui-glass`, no `ui-surface` for Navbar and Footer; Canvas and Mode-body unchanged |
| 3 | CardStack chip-row token | product drift | `WindowChrome` enlarge and close buttons use `min-h-chip-row` instead of `min-h-medium` |
| 4, 5 | navbar and footer seam on hover | product wrong (law 2) | `WindowChrome/shared-border.ts`: emphasis is decided against the rest-state drawn sides resolved up the whole parent chain (`restSides`), so a cell flush with a group that is itself flush with an undrawn band edge adds no line on hover |
| 6, 7 | scroll canvas, native panel overlay | environment | Playwright headless shell on this Mac never delivers wheel or key scroll input (`mouse.wheel` hangs); both tests launch `channel: "chromium"` (new headless), where input works |
| 8 | Icons mask on alias change | stale | reference markup normalised for inter-tag whitespace (the renderer strips it, same as the full renderer) |

## Law 4 (Layout)
- `Layout` (non scroll mode): the canvas is `data-slot="layout-canvas-bleed"`, absolute, `z-0`, isolated, from the top to the bottom client edge (bottom stops above a subfooter). Bands stay in flow as `data-layout-band="start|end"` rows at `z-[1]`, glass, so bands win hit testing and the compositor paints their seams (`data-elevation-root`). Panels and the bottom chip row stay between the bands, so they clear the bands by construction.
- `useBandClearance` publishes `--layout-clearance-top`, `--layout-clearance-bottom`, `--layout-subfooter-height` on the layout root (ResizeObserver). CSS: `.ui-band-clearance` (padding and scroll padding) and scroll padding on the bleed layer, in `🎨️styling/🖌️ui/🎨️.css`. `scrollCanvas` mode is unchanged (sticky bands, content scrolls beneath).
- Consumers: only OS `ShellHost` uses ui `Layout` (hub admin, demonstrator, play, staffage, course do not import it); nothing to change there. The canvas content of the OS is edgeless; scrolling documents inside it should use `ui-band-clearance`.
- `ContextMenuTriggerButton` forwards `data-*`; OS triggers (`ShellHost`, `TextEditor`) now carry `data-glass-chip`.

## Tests
New: Layout canvas beneath the bands (geometry in Chromium for desktop and mobile short pages, row structure, clearance vars), trigger data forwarding. Subfooter row and column tests adapted to the bleed layer.
Commands: `lockf -k /private/tmp/claude-501/semio-vitest.lock sh compositor-vitest.sh <filter>` (Layout 44 pass, level-context and chip-row pass, Icons 10 pass). Trigger test is outside the stock include list; run with a direct config (`🗑️generated/trigger-vitest.config.ts`, 3 pass).

## Open
- The trigger test and the ContextMenu tests folder are not in the stock vitest include list.
- 49 other `chromium.launch({ headless: true })` call sites share the headless-shell scroll problem on this Mac.
- Visual check of the OS shell in a browser not done (no dev server allowed).

## Update: docks between the bands, stacking, final run
- Canvas (`🎨️Canvas`) wraps its content and bottom chip row in `data-slot="canvas-stage"`, absolute with `top/bottom = --layout-clearance-top/bottom` (0 outside a Layout). Following wgpu: docks, windows, chips, resize handles and compositor edges lie between the bands; only the canvas floor runs beneath them. Bleeding the content plane was not done.
- Regression found and fixed by the new geometry test: the empty middle row of Layout (after the bleed layer in DOM order) intercepted every pointer event over the canvas. The bleed layer now follows the middle row in DOM order (z-0), panels (z-panel) and bands (z-1) stay above it.
- New test (Layout component file): real `Canvas` and `Mode` inside `Layout` in Chromium with 1 and 2 docked windows: canvas floor 0 to 600, stage equals the span between bands, every `[data-window-frame-chip]` rect lies outside both bands and `elementFromPoint` at its centre is the chip; plus a jsdom test that the band heights are published as clearance. The geometry test sets the vars with the same sum the hook computes (static markup has no effects).
- Other test updates: stepper hover expectation now follows law 2 (no top and right line on undrawn row edges), footer-chips and shared-border-dock Layout structure.
- Final: `compositor-vitest.sh` full stock run: 83 files, 1705 tests passed. `🗑️generated` logs removed.
