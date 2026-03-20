---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed tree indentation lines 0.5px misalignment by subtracting half the line width from positioning. Playwright measurements confirm perfect 0.0px alignment across all tree rows. 13/13 unit tests pass.
## Changes

- `semio/js/sketchpad/elements.tsx`: In `IndentationLines` component, changed line positioning from `indentationLinePx(i)` to `indentationLinePx(i) - 0.5` to center the 1px-wide line div exactly on the chevron center.

## Log

- Analyzed alignment math: chevron center = `paddingLeft + 7px`, line visual center was at `indentationLinePx(i) + 0.5px` due to 1px line width
- Used Playwright to measure all 17 tree rows in storybook Default Tree story
- Before fix: consistent 0.5px offset between line positions and chevron centers
- After fix: all diffs = 0.0px (perfect alignment)
- All 13 unit tests pass
- Kit e2e test has pre-existing failure unrelated to this change (folder selection assertion)

## Todos

- [x] Gather current code state
- [x] Identify alignment issue (0.5px offset from 1px line width)
- [x] Fix IndentationLines positioning
- [x] Verify fix with Playwright measurements
- [x] Run existing tests

## Plan

1. Measure tree alignment in storybook using Playwright
2. Identify root cause of line-chevron misalignment
3. Fix the CSS positioning offset
4. Verify with fresh storybook measurements
5. Run tests to ensure no regressions
