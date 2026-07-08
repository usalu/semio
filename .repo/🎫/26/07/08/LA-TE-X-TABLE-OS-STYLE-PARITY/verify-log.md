# Verify Log

## Build

```bash
cd print && bun ./script.ts test
```

Result: all 12 template PDFs built (6 templates × light + dark).

## Notes

- `\SemioTableHeaderRow` must use `\newcommand` (not xparse `m` argument) because `\rowcolor` issues `\noalign` and breaks when wrapped in `\NewDocumentCommand`.
- `\SemioTableBegin`/`\SemioTableEnd` commands work inside `\makeworkpackages{...}` macro arguments.
- Header row: `semio-chrome-window` background, `h-large` (9× spacing unit), `\bfseries`.
- Body rows: hairline horizontal rules only, `h-medium` (7× spacing unit) via `\extrarowheight`.
- Theme-aware colors resolve through existing `semio-chrome-*` aliases in `semio-core.sty`.
