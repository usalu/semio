# Print Footer Bottom Spacing — Verify Log

## Problem
Chrome footer sat too high with ~2.5cm dead space below it; chapter/TOC pages used KOMA `plain` style (bare page number only).

## Fix
- Apply chrome page styles via `\AfterEndPreamble` so KOMA does not override `fancy`
- Mirror chrome header/footer into `plain`, `scrplain`, `scrheadings`
- Reduce bottom margin: `\geometry{bottom=\semio@spacing@single}`
- Anchor footer at physical page bottom with eso-pic `\AtPageLowerLeft` + `\put`
- Skip footer on `empty` pages via `\Ifthispagestyle`

## Verification
Built `print/dist/paper.pdf` and rasterized page 2.

Before: bare centered page number floating mid-lower page.
After: full chrome footer bar flush at page bottom (emblem, author, page number).

Artifacts: `report-p2-before.png`, `report-p2-footer-crop.png`
