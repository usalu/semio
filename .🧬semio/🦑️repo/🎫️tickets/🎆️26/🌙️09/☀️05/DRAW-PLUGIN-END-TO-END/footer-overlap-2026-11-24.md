# Utility bar unfold button covered by the canvas footer chips

## Root cause
Regression, not locale dependent. Commit a6e34dbae6f (754) moved the footer chips ("Display" dock tab, centre chip, end chips) from a band below the canvas into an absolute overlay inside the canvas rectangle (`🎨️Canvas/🦶️bottom-chips`, `data-slot="canvas-footer"`, `z-10`). The window fills the canvas, so the bottom-left window pane (`Window` utility bar, `anchor="bottom-left"`, flush `bottom: 0`) now sits at exactly the same rect (0,947) as the start chip. German is no different: the same probe on the current code gives the unfold button 87x24 at (0,947) under "Anzeige". The earlier German journey passed before the overlay landed.

Window panes already yield to anchored shell panels through `chrome-safe-area` (`publishShellChromePanelBox` / `useChromePanelSafeArea`), but the chip row never published its boxes. A second latent defect: `useChromePanelSafeArea` ran its layout effect before the parent host ref was attached (children's layout effects run before the parent's refs), found `host === null`, and only recovered if `panels` changed later. Probe runs were flaky for that reason.

## Fix (host TS, no bridge, no z-index hack, footer unchanged)
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🦶️bottom-chips/🟦️.tsx`: each chip slot (`CanvasBottomChipSlot`) publishes its measured box as an occupied chrome box (bottom-left / bottom-middle / bottom-right) with a ResizeObserver and window resize, and retracts it on unmount. Bottom-anchored (and any other) window panes whose rect overlaps a chip now step clear by the chip height, whatever the label widths or the window layout; no overlap, no reserve.
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🛟️chrome-safe-area/🟦️.tsx`: `useChromePanelSafeArea` re-evaluates after mount (`attached` state) so the host ref is guaranteed to be set.
- Test: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/footer-chips.test.tsx` gains a language-agnostic geometry test (bottom-left pane at three label widths lifts by exactly 24 px, a pane clear of the chips stays put) and a DOM test for publish/retract.

## Verification
Probe: `$T/probe-footer-overlap.mjs <locale> <w> <h>` (elementFromPoint at the button centre, then a real click; expects the fold button afterwards).

| locale | viewport | button rect | hit at centre | footer top / button bottom | opens |
|---|---|---|---|---|---|
| en | 1600x1000 | 0,923 74x24 | yes | 947 / 947 | yes |
| de | 1600x1000 | 0,923 87x24 | yes | 947 / 947 | yes |
| en | 1024x700 | 0,623 74x24 | yes | 647 / 647 | yes |
| de | 1024x700 | 0,623 87x24 | yes | 647 / 647 | yes |

Before the fix: en and de at 1600x1000 and 1024x700 all missed (hit "Display"/"Anzeige") and the click timed out.

Tests via `repo test-file ...footer-chips.test.tsx`: the two pure-geometry tests pass (the existing `chromeHostedOpenPanelPositionStyle` one and the new reserve test). The remaining DOM tests of that file, including the new publish test, fail in the plain bun route with `document is not defined`; that is pre-existing (the sibling `shared-border-dock.test.tsx` and `app-mode-nav.test.tsx` fail the same way), the file needs the jsdom vitest runner, which `test-file` does not route to. The publish behaviour is covered end to end by the browser probe. `shell-components-9` (safe-area suite) still registers and passes.

Debug logs were added temporarily and removed (grep for `[DEBUG]` in the touched files is empty).
