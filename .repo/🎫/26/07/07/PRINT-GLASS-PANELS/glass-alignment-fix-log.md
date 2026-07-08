# Glass Alignment Fix Log

## Issues
- Glass PNG misaligned from border (grey fill offset from white frame)
- No visible frosted content behind panel on dark theme

## Root causes
1. Pass-2 shipout used separate `\includegraphics` + `\fbox` / misaligned `\llap`/`\vspace{-#4}` stacking
2. `tcolorbox` `background image` key does not exist in tcolorbox 5.0.2; `colbackopacity` invalid
3. `tcolorbox` inside `eso-pic` `\put` caused `Extra }` at shipout
4. Glass tint too heavy (`glassPanelAlpha` at full strength + `over` blend) on dark panel color

## Fixes
### `print/tex/semio-window.sty`
- Glass shipout: pre-build panel in `\sbox{\semio@panel@glass@box}` using TikZ node with `path picture` background image + `draw` border on same node (aligned frame + fill)
- `inner sep=\semio@chrome@padding` for text inset; image sized to `#2` x `#4` matching manifest crop
- Added `\Ifthispagestyle` tracker for footer shipout overlay
- Register listof: collect rows in vbox before tabular (`\semio@register@rows`)

### `print/script.ts`
- Reduced tint alpha to `glassPanelAlpha * 0.42`
- Increased blur: `blurSigma = (glassPanelBlurPx * renderScale) / 2`
- Tint composite blend: `soft-light` instead of `over`

## Verified builds (pass 1 + glass + pass 2)
- `paper` / `paper-dark` ✓
- `kompaktbericht` / `kompaktbericht-dark` ✓
- `zwischenbericht` / `zwischenbericht-dark` ✓

## Glass PNG spot-check
- `kompaktbericht-dark/panel-1.png`: shows blurred teal/orange blockquote chips behind frosted tint

## Remaining
- `forschungsbericht` fails at `\makeregisters` (register table preamble; unrelated concurrent register refactor)
