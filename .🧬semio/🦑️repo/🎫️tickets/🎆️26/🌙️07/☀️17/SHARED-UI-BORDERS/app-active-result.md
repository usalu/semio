# App Active Outline Result

## Root Cause

`ModeDockStack` passed the interaction-only `useSurfaceActive` value to `WindowChrome.active`, rather than deriving it from `activeWindowId` and the stack's selected `activeId`. Consequently the selected left tab rendered active while its stack supplied `false` before any interaction.

Inspected `WindowChromeSilhouetteBorder`'s `windowEl` query, `resolveWindowSilhouetteBorderKind`, its mutation-observer effect, and the shared compositor. The resolver intentionally obtains activation from its explicit stack-active argument, not the nested window's `data-active`. Its observer tracks introduction/celebration/loading classes, not controlled activation. No query/observer patch is required: passing the authoritative Mode selection updates the silhouette kind, root `data-active`, and compositor registration together. Keeping stale interaction activity ORed with selection could leave the wrong sibling active after a controlled change, so it is not used as a second activation source.

## Changes

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/⚓️dock/🟦️.tsx`: derive chrome activation from the globally active selected window; preserve surface interaction bindings, shared dimensions, and splitter behavior. Only the hook destructuring and `active` expression were changed by this assignment; other existing diff hunks were preserved.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/shared-border-dock.test.tsx`: actual controlled Mode fixture matching App's left/right windows, with measured 584×370 adjacent stacks and title chips. Covers initial activation without pointer/focus, neutral sibling, `onActiveWindowChange` followed by host state change, full perimeter/chip coverage, hover precedence, and active restoration. Edge coverage accumulation uses existing third-party Decimal as an independent numeric oracle.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/active-outline.feature`: language-neutral acceptance scenarios.
- Ticket `check-app-active.mjs`: reusable Playwright check against the real running App story, without modifying story/configuration.

No Window/WindowPane positioning, CSS, Storybook configuration, or WindowChrome source changes.

## Validation

- Red: focused regression failed before fix, expected `active`, received `normal`.
- Green: normal `run-ui-tests.mjs shared-border-dock shared-border.test` — **3 files, 32 tests passed**. Includes real Mode integration and existing WindowChrome/compositor/shared-outline tests.
- Real Chromium on Storybook port 6011: left silhouette `active`, right `normal`; hover wins and active restores after pointer leave; zero page errors; stacks remain x=16,width=584 and x=600,width=584; splitter width remains zero.
- Reviewed scoped source diff; `git diff --check` clean. No modifying Git commands.

## Remaining Considerations

The static App story does not supply `onActiveWindowChange`, so live transfer is not asserted against that read-only fixture; transfer is tested in the actual controlled Mode integration fixture. Concurrent pane-boundary work changes nested utility geometry; its acceptance remains owned by that separate assignment. No remaining activation blocker.
