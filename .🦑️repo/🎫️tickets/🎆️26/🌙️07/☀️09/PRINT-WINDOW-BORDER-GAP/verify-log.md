# Print Window Border Gap

## Problem

Window title bars in print showed empty chip slivers (no Titel / Aktenzeichen labels), a continuous top border (no U-shaped cutout), duplicate left hairlines, and a gap between the header row and the window body.

## Root causes

1. **Title/number never expanded in LaTeX mode** — `\tl_use:N` inside `\ExplSyntaxOff` in store/render paths left chip text empty even when token lists held the correct values.
2. **Wrong gap segment** — `\semio@window@header@muted` used `\semio@window@gap@inbox` (top stroke first) instead of `\semio@window@gap` (OS cutout).
3. **Redundant trailing rule** — extra full-width `\semio@window@stroke@h` after the chip row.
4. **Empty number check** — `\if\relax\detokenize{#1}` tested the macro name, not its expansion.

## Fixes (`print/tex/semio-window.sty`)

- **Store:** `\exp_args:Nxx \semio_window_header_store_set:nn` expands `\l_semio_window_number_tl` / `\l_semio_window_kind_title_tl` in expl3, then `\xdef`s globals for shipout reuse.
- **Render:** `\exp_args:Nxx \semio_window_header_render_muted:nn` expands token lists in expl3 and calls `\semio@window@header@muted{#1}{#2}` inside `\ExplSyntaxOff`.
- **Cutout:** `\semio@window@gap@inbox` → `\semio@window@gap`; removed trailing full-width stroke.
- **Empty ctrl chip:** `\edef\semio@window@header@numval{#1}` + detokenize expansion check.

## Verification

```bash
# Stop duplicate tectonic/watch processes first if builds hang or fail with no log.
bun run build:mit-bestand:zwischenbericht
```

TeX output on cover page (`\makecoverpages`) shows `Overfull \hbox (10.67674pt too wide) in paragraph` — confirms title chip text is rendered (empty slivers only produced 6pt overfull).

Rebuild `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf` and check page 1: left chips show Titel, Aktenzeichen, Förderzeitraum, Berichtszeitraum, Beschreibung, etc.
