# Glass Alignment Fix Log

## Issues

- Glass PNG misaligned from border (grey fill offset from white frame)
- No visible frosted content behind panel on dark theme

## Root causes (updated)

1. **sharp blur on RGBA canvas PNGs** without `.removeAlpha()` corrupts colors to neon cyan/magenta
2. **Excessive blur sigma** (`/2` scaling) washed content to a flat tint
3. **Intermediate PNG roundtrips** between pipeline stages further corrupted colors
4. **Tint layer as PNG composite input** could flatten variation; use raw RGBA buffer instead

## Fixes (updated)

### `print/script.ts` — `renderPanelGlass()`

- Single sharp pipeline: `extract → removeAlpha → blur → modulate → composite → png`
- `blurSigma = (glassPanelBlurPx * renderScale) / 9` (~12px at 200 DPI)
- Tint via raw RGBA buffer composite (`blend: "over"`) not a generated PNG layer
- No intermediate `.png().toBuffer()` between crop and final output

## Verified

- `kompaktbericht-dark/panel-1.png`: frosted blur of section chips visible through dark panel tint (stdev ~24)
- `kompaktbericht/panel-1.png`: light theme glass with visible content variation (stdev ~49)
