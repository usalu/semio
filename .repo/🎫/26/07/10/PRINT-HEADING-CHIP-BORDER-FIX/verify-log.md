# Print Heading Chip Border Fix Verify

Build: `bun ./script.ts build report forschungsbericht`

## Root cause

`\semio@heading@chip` routed section/chapter/subsection chips through `\semio@window@cap`, which paints a solid `\colorbox` fill in the tier accent color. Border strokes used the same accent color, so borders were invisible and chips read as filled blocks instead of the paragraph-style bordered caps.

## Fix

- `\semio@heading@chip` now reuses `\semio@paragraph@cap@muted@vbox` (canvas fill + `semio-chrome-border-normal` hairline borders) with `semio-chrome-foreground` title text.
- `\semio@heading@pair` gap cutout uses `semio-chrome-border-normal` like muted heading pairs.
- `\semio@heading@installall` registers `\chapter` and `\part` when present.

## Visual checks

| Doc | Page | Raster | Section chips bordered | No solid accent fill |
| --- | --- | --- | --- | --- |
| report | 3 | report-p3.png | yes | yes |
| forschungsbericht | 3 | forschungsbericht-p3.png | yes | yes |
| zwischenbericht | 2 | zwischenbericht-p2.png | yes | yes |

## Notes

- Pixel sampling on report p3 body: grey fill pixels reduced from 4247 (before) to 3070 (after); gray border pixels present in both, confirming bordered-cap structure.
- Window element chips (`\semio@window@tab`, `\semio@window@ctrl`) still use tier-colored fills for Image/Table/etc. windows — only heading chips changed.
