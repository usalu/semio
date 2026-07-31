# Table Row Border Verify

## Changes

`print/tex/semio-table.sty`:

- Row separators render **between** rows only; the last row has no trailing `\hline`.
- Body cells use `T`/`U` column types with `\semio@spacing@single` padding top and bottom (matching `\tabcolsep` sides).
- Removed `\noalign{\vskip\semio@block@sep@skip}` below row content (was pushing text against the top rule).

`print/tex/semio-window.sty`:

- Added `semio~window~table` tcolorbox style (`left=0pt`, `right=0pt`, `bottom=0pt`) for `semiotable` windows so row rules span the full window width.

## Build

```bash
cd mit-bestand/bericht && bun ./script.ts build zwischenbericht/zwischenbericht.tex
```
