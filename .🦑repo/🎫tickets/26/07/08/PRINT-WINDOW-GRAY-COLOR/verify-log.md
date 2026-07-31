# Print Window Gray Color

## Change

Window chrome (title bar chips and box frame) now uses the same muted gray styling as `\paragraph` headings:

- Title/number chips: `\semio@heading@cap@muted` with `semio-chrome-border-normal` border, `semio-chrome-canvas` fill, `semio-chrome-text-normal` text
- Box frame: `semio-chrome-border-normal` (tier no longer overrides with primary/secondary/tertiary)

## Before

Tier-colored window headers (red/teal/orange) and matching `colframe` on `tcolorbox` windows.

## Verification

- `bun ./script.ts build paper` — success (light + dark)
- `bun ./script.ts build kompaktbericht` — success (light + dark)
- Window-specific visual check blocked by unrelated template/register build errors when adding `Blockquote`/`Figure` to templates
