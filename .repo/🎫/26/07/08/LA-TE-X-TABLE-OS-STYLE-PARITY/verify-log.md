# Verify Log

## Build

```bash
cd print && bun ./script.ts test
```

Result: all 12 template PDFs built (6 templates × light + dark).

## Table spacing fix (2026-07-08)

Root cause of empty row gaps: `\extrarowheight` was set to the full body row height (`7 × spacing unit`), which **adds** extra space below every row on top of the text — doubling visual row height.

Fixes in `semio-table.sty`:
- `\extrarowheight` set to `0pt`
- Row min-heights via `\rule{0pt}{...}` struts only (header `9×`, body `7×` spacing unit)
- `ll` / `lll` shorthand maps to full-width `m` columns with `\raggedright` and vertical centering (matches React `align-middle`)
- Column widths account for `\tabcolsep` so the grid spans `\linewidth` inside the window chrome

## Notes

- `\SemioTableHeaderRow` must use `\newcommand` (not xparse `m` argument) because `\rowcolor` issues `\noalign` and breaks when wrapped in `\NewDocumentCommand`.
- `\SemioTableBegin`/`\SemioTableEnd` commands work inside `\makeworkpackages{...}` macro arguments.
