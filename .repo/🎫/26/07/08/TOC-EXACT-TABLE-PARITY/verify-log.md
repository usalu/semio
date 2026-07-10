# Verify Log — TOC Exact Table Parity

## Build

```bash
cd mit-bestand/bericht && bun ./script.ts build
```

Result: `zwischenbericht.pdf` and `zwischenbericht-dark.pdf` (12 pages each), no fatal TeX errors.

## PDF text extraction (pdfjs)

| Check | Result |
|-------|--------|
| Total pages | 12 (light + dark) |
| TOC spans multiple pages | pages 2–3 |
| TOC entry density | page 2 ~117 hierarchy tokens, page 3 ~77 |
| Body starts after TOC | `Netzwerk` on page 4 |
| Glossary register table | page 12 |

## Visual verification

Screenshots in ticket folder:

- `verify-parity-p2-toc.png` — TOC page 1: muted title tab, shaded header row, left/right border, hairline between every row
- `verify-parity-p3-toc.png` — TOC continuation with repeated title + headers, bottom closing rule on last TOC row
- `verify-parity-p4-netzwerk.png` — `Netzwerk` reference table inside `Window` tcolorbox (parity reference)
- `verify-parity-p12-glossary.png` — Glossary register with matching bordered longtable chrome

## Root causes fixed

1. **Column width overflow:** long register colspecs now use `\semio@table@long@inner@w` measured after table body font selection, with `@{}` between columns, `\tabcolsep=0pt` in long mode, and padding/rule widths measured via `\hbox` rather than raw `em` tokens at the wrong font size.
2. **Missing row hairlines:** `\SemioTableRow` routes long-mode rows through `\semio@table@row@long` (`#1\\ \hline`) instead of the `\global`-boolean-gated `\noalign{\hrule}` path. Short `tabular` tables keep the existing before-row separator logic via `\semio@table@row@short`.
3. **Pagination regression:** re-enabling `\noalign{\hrule}` with a `\newif\ifsemio@table@long@mode` guard leaked state into `longtable`'s measurement pass (14 pages, near-empty pages). Literal `\hline` after each row plus a `\semio@table@long@mode` count switch restores 12-page pagination.
4. **Bottom closing rule:** provided by the last data row's `\hline` on the final TOC page (visible on `verify-parity-p3-toc.png`); separate `\endlastfoot` rule removed as redundant.

## Notes

- Minor `Overfull \hbox` (~5.75pt) alignment warnings remain in the log for register tables; visual inspection on page 3 shows right border aligned with the margin and parity with `Netzwerk`.
- `\newif\ifsemio@table@long@mode` inside `\SemioTableRow` caused `Incomplete \iffalse` on short tables; replaced with `\newcount\semio@table@long@mode` and separate row macros.
