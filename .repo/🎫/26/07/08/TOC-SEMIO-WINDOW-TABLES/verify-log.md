# Verify Log — TOC Multi-Page Register Tables

## Build

```bash
cd mit-bestand/bericht && bun ./script.ts build
```

Result: `zwischenbericht.pdf` (11 pages), no `Overfull \vbox` on TOC.

## PDF text extraction (pdfjs)

| Check | Result |
|-------|--------|
| TOC spans multiple pages | pages 2–3 (~67 + ~43 entries) |
| Window title on continuation | `Inhaltsverzeichnis` on page 3 |
| Column header repeated | `Nr` / `Titel` / `Seite` on pages 2–3 |
| Late entries present | `Ergebnisverwertung`, `Verzeichnisse` (section A) on TOC pages |
| Body starts after TOC | `Arbeitspaketzuordnung` on page 4 |

## Root causes fixed

1. Register lists used non-breakable `Table` tcolorbox + `tabular` — replaced with page-level `longtable`.
2. `\ifsemio@table@register@long` was set inside a TeX group — must use `\global\...true`.
3. Malformed `\endfirsthead` marker (empty head before content).
4. Inter-row `\noalign{\hrule}` separators prevented `longtable` page breaks — disabled in long mode.
5. Repeating `SemioTableHeaderRow` (expl3) in `\endhead` forced one row per page — use plain-LaTeX `\semio@table@long@header@repeat@three` on continuation pages.
6. Continuation window title uses longtable-safe `\semio@heading@cap@muted` hbox row, not full `\semio@window@header@muted` inside `\endhead`.
