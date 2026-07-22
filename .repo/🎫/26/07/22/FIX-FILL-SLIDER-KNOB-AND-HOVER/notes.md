# Fix Fill Slider Knob And Hover

## Problem

Puzzle 3d fill-count slider (ready extent + loading while planning) did not show the knob and lost regular hover chrome:
- row `hover:bg-hover-interactive-fill`
- knob / range `group-hover:bg-emphasized`
- tree guide line emphasis

## Root cause

`renderWindowMeasure` passed `loading={measure.loading === true}` into `WindowMeasureTreeLeaf`, which applied `border-loading` on the whole `window-measure-tree-row`. That utility paints a `::after` at `z-index: 1` over every descendant, covering the Radix thumb (z-auto) and obscuring hover fill / emphasis.

## Fix

1. `Slider` accepts `loading` and applies `border-loading` on a track-only wrap (`data-slot="slider-track-wrap"`) that is a **sibling** of the thumb — never an ancestor.
2. Framework measure rendering passes `loading` into `Slider` and no longer stamps it on the measure row.

## Verification

```text
bunx vitest run -t "renders sliders with element gray|renders a ready extent|puts the loading ring on the track|keeps measure-row hover fill"
# 4 passed
```
