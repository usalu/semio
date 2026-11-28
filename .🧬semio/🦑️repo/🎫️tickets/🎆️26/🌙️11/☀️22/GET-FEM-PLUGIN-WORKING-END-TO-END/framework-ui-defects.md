# Framework UI defects (F4, F6, F8, F12, F13, X2, X3, U1)

Method: re-checked every item on the current vite-served TS in headless Chromium (3D on 6087, 2D on 6086, own drivers 6277/6278, now stopped), read the source and the earlier fix report `$M25/framework-ui.md`. Most defects were already fixed after the v2 matrices (11-25); this pass verified them live and closed the two that were still open.

| Id | State | Evidence / root cause | Fix and test |
|---|---|---|---|
| F4 Actions pane hides viewport | Fixed earlier (verified live) | `useChromePanelSafeArea` graded the whole pane against every dock (`🛟️chrome-safe-area`). Live now: pane open in Model, canvas 500x942 at x=300 with grid, Results canvas 800 wide. | `chromePanelHeadBox`, law `🧪️tests/🛟️chrome-panel-safe-area` (unchanged here). |
| X3 Artifact panel over window tab bar | Fixed by the same change (verified live) | With Artifact open the Model/Results tab buttons (Focus, Close) sit at x>=303 and `elementFromPoint` hits them. | Same law. |
| F6 Set Locale arg zero size | Not reproducible (verified live) | `command.os.os.setLocale.arg.locale.input` is 160x27.6, section 300x28, inside the 76 px command scroller (bottom-anchored). | none |
| F8 Editor disabled after Viewer | Fixed earlier (`roleButtonDisabledV1`, busy/refusal only) | Code + truth-table law in `🧪️tests/🔀️surface-switch`. A fresh live round trip was not possible: the 3D boot wedged in the known plugin runtime stall ("Loading plugins…"), earlier agent verified it live. | existing law |
| F12 literal icon id | Open in framework, fixed now | `coerceIconSource`/`decodeIcon` turned every unregistered bare id (`ruler`, `mouse-pointer-click`) into a `text` icon, i.e. its own letters. Plugin ids were corrected earlier (`timer`->`clock`, registry law), but any future unknown id would paint text again. | New `looksLikeIconId`; identifier-shaped unknown strings now resolve to the `missing` glyph: empty dashed box in the icon box, `data-icon-id` carries the id, `aria-hidden`. Authored text stays via `text:`. Tests in `🔣️Icons/🧪️tests/🧩️component` (3 ids, size box, text icon unchanged). |
| F12 Poisson clipped | Fixed earlier (`px-tiny` on the slider readout) | `$M25/framework-ui.md` | none |
| F13 guide line after drag | Fixed earlier (`gumballDragEndPointerState`); hardened now | Residual path: a cancelled pointer or a blurred window mid-drag never ended the drag, leaving the handle active (guide drawn). | `GUMBALL_DRAG_END_EVENTS = pointerup, pointercancel, blur` (model) used for add/remove of the window listeners in `🎬️Scene/🕹️gumball`; test in `🧿️gumball-model/drag-end.test.ts`. |
| X2 Focus hides other window / Reset panels | Works as designed (verified live) | Focus maximizes and the button turns into `Unfocus`, which restores the Results window. Close Results then Settings > Reset panels brings it back (tabs Model@3, Results@803). | none |
| U1 Direct Select not pressed at boot | Fixed (verified live in 2D) | 2D plugin declares `initial_utility_id = selectDirect` (law `canvas_windows_arm_direct_select_initially`), framework `resolveInitialWindowUtility`/`tree-windows` shows it armed. 2D boot snapshot: `button "Direct Select" [pressed]`, Method/Mode radiogroups present. | none |

## Files changed

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🟦️.tsx`, `…/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🕹️gumball/🟦️.tsx`, `…/🧿️gumball-model/🟦️.ts`, `…/🧿️gumball-model/drag-end.test.ts`

## Tests

Runner `$T22/run-ui-vitest.sh`: filters `Icons gumball icon tree-windows` (3 files, 28 tests) and `shell-components control-chrome safe-area window-pane` (3 files, 14 tests), all green. No Rust law added: none of the changed logic is in Rust (the U1 Rust law already exists in the 2D plugin).

## Not verified

F8 round trip live this session (3D boot stalled in the known runtime issue, unrelated). The new `pointercancel`/`blur` end of a gumball drag is covered by the constant law, not by a live drag.
