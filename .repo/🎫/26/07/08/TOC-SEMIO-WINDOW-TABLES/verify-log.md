# Verify Log — TOC Window Table Border Consistency

## Build

```bash
cd mit-bestand/bericht && bun ./script.ts build
```

Result: `zwischenbericht.pdf` and `zwischenbericht-dark.pdf` (12 pages each), no fatal TeX errors.

## PDF text extraction (pdfjs)

| Check | Result |
|-------|--------|
| TOC spans multiple pages | pages 2–3 |
| TOC entry density | page 2 ~134 hierarchy tokens, page 3 ~60 |
| Window title on continuation | `Inhaltsverzeichnis` repeated on page 3 |
| Column header repeated | `Nr` / `Titel` / `Seite` on pages 2–3 |
| Body starts after TOC | `Arbeitspaketzuordnung` / `Netzwerk` on page 4 |
| Glossary uses bordered longtable | page 12 (`Glossar` register table) |

## Visual verification

Screenshots in ticket folder:

- `verify-border-p2.png` — TOC page 1: muted title tab, shaded header row, left/right window border with padding
- `verify-border-p3.png` — TOC continuation with repeated title + headers
- `verify-border-p4.png` — `Netzwerk` reference table inside `Window` tcolorbox
- `verify-border-p12.png` — Glossary register with matching bordered longtable chrome

## Root causes fixed

1. `\semio@window@gap` now requires two arguments (`fill`, `stroke`); continuation chrome in `semio-table.sty` passed only one, causing `xcolor` error `\hbox` on TOC build.
2. Register tables (`SemioTableRegister` / `Reference` / `Glossary`) always render via `longtable` — removed obsolete `\ifsemio@table@register@long` flag and window toggles.
3. `SemioGlossaryListOf` no longer forces short `tabular` path (`\global\semio@table@register@longfalse` removed).
4. Long register tables draw window-like outer border using `@{\hspace{padding}}| ... |@{\hspace{padding}}` colspec (padding + vertical rules on every page).
5. Bottom closing rule via `\semio@table@long@closing@rule` in `\endlastfoot`.
6. Continuation header uses matching bordered `\multicolumn` and two-arg `\semio@window@gap`.
7. Inter-row `\noalign{\hrule}` remains disabled in long mode (`\ifsemio@table@long@mode`) — re-enabling breaks pagination (14+ pages, one row per page).

## Notes

- Per-row hairlines like `Netzwerk` cannot be applied inside `longtable` without breaking page breaks; outer border + header row styling is the compatible match.
- Minor `Overfull \hbox` (~4pt) on register alignments from border padding; acceptable.
