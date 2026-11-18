# Round 3 plugin fixes

## Framework
- `SharedBorderRow wrap` and `ToggleGroup wrap`: cells stretch per line and share one hairline across lines. `useSharedBorderWrap` tags cells `data-wrap-end` / `data-wrap-last`. CSS added before the priority layer in the ui `.css`. A dropped divider leaves a same-size margin, so a cell never re-wraps when tagged.
- Tests: `SharedBorderRow/wrap.cases.json` + `wrap.schema.json` (language-agnostic) and `wrap.test.tsx`. Passed, 69 tests across SharedBorderRow, ToggleGroup, WindowChrome and ButtonGroup.
- `floatingPaneAsideClass` is now borderless, gapless, paddingless and non-scrolling. It is used only by the CAD REPL.
- `floatingFieldSurfaceClass` and `floatingTagClass` (with On/Off) are now unused. They are still exported; removing them touches the giant barrel lines the other agent edits, so left for a follow-up.

## Not-verifiable items, resolved
- Compositor vs native border: the ownership layer only makes attached sides transparent. The aside also scrolled its own overlay away. Fix: the aside is a vertical `SharedBorderRow` with no native border, and the scroller is nested inside it.
- Checkbox: a native `input[type=checkbox]` with no border or frame CSS. It does not draw a frame. It sits as content in a `SharedBorderRowField` cell.
- Raw buttons: replaced by framework `Button`, so the question no longer applies.

## CAD REPL aside (`🎛️repl/🟦️.tsx`)
- One vertical row owner, a title cell, and a nested scroller.
- Transition buttons in a `wrap` row.
- Command field as a row of Input and toggle Button cells, with the ghost completion overlay absolutely positioned.
- Suggestion menu flush at `top-full` as a framed row with no top edge; the key is an unframed `<kbd>`.
- Select blocks are label + trigger rows.
- Four `ReplToggleSection`s: a master checkbox cell over a wrapping multiple `ToggleGroup`. Prune-on-disable logic kept.
- State line and last response are cells, with no `border-t`.
- `repl-panes`: `fieldRow` is a horizontal row of label, control and Clear cells. The pane is a vertical row, and the fallback paragraphs are cells.
- Test `🧱️repl-aside-frame` rewritten: 4 pass.

## GIS and stories
- GIS inference port: a vertical row with a progress cell and one row of 5 framework Buttons. Existing panel test: 2 pass. Cancel and reject still dispatch the same action (unchanged).
- Stories converted to `SharedBorderRow` / `Button`: fem x4, block x3, remodel x4, multi-shell harness, cad renderer story.
- `5d-timeline` and `2d-fixtures`: panes are cells, with the inline `borderLeft` removed.
- `ui-react` dependency added to the fem, block and catalog dev `package.json`. `bun.lock` not updated; remodel has no package.

## Live probe
- The harness is `round3-cad-live.tsx` with `vite-round3-cad.config.ts` on port 5451, using a replica of the aside, attribute-row and menu structure plus the real GIS `InferencePortPanel`.
- It does not mount the full wasm-dependent CAD renderer.
- Probe count was 0 at 1440x900 and at 375x812, with no overlaps.
- The harness lacks the icon catalog and theme, so chip icons show red placeholders there.

## Unverified
- The real CAD renderer was not mounted.
- Story DOM was not run.
- The `react-export-surface` suite was not run to confirm exports for the new `useSharedBorderWrap` and `sharedBorderWrapPlacements`.
- An active chip's top seam in the line below stays neutral, a minor asymmetry.

## Follow-up: left-open items closed
- Removed `floatingFieldSurfaceClass`, `floatingTagClass`, `floatingTagOnClass` and `floatingTagOffClass`. A repo-wide grep (tests and products included) found only their definitions, the two barrels and a negative assertion in the CAD aside-frame test. Old tickets mention them only as history.
- `bun.lock`: the repo binary has `repo dependencies lock`, but it rewrites the lock for the whole workspace, and bun is not installed. The lock has no entries for the fem, block or catalog dev packages; it does not even list the puzzle dev package that already depends on `ui-react`. No lock change is needed, and none was made.
- `react-export-surface` and `react-package-export`, with SharedBorderRow and ToggleGroup: 7 files, 56 tests, all pass. The new wrap exports need no barrel change.
- Story DOM tests: the stories are storybook-only with no DOM test suite; no failures found there. The CAD aside-frame test is green after the export removal.
