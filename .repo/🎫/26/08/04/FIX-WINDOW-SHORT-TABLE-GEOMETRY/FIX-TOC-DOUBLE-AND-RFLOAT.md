# Fix: TOC Double Mid-Rule + Floating Right Stem

## Bugs
1. **Floating R vertical** above title-only chrome (TOC / glossary) — chrome row painted full-height `border@R` with no number chip.
2. **Double hairline** under column headers on longtable **continuation** pages (TOC p3+) — measured 1.5pt (= 2×0.75pt hairline), page 1 was 0.75pt.

## Root causes
1. `\semio@table@long@title@chrome@row` always used `@{\semio@table@border@R}`. Title-only muted headers clear the number, so the lone R stem floated in the air.
2. Mid-rule as one hbox (inner `\vrule` + smash pillars) measured as a stacked double under `\endhead` reinsertion. Full-width `\hrule` + separate smash pillars restores single hairline on continuations.

## Fix (`print/tex/semio-table.sty`)
- `\semio@table@border@R@chrome` — width phantom only (no ink). Number chip paints its own right wall; body rows supply `border@R` below the baseline.
- Chrome row uses `@{\semio@table@border@R@chrome}`.
- `\semio@table@rule` (owns@sides): `\hrule` + smash L/R pillars (3pt overlap).
- Long body rows use `\\*` before the trailing mid-rule so the rule cannot orphan across a page break.

## Pixel QA (scale 4, after rebuild)
| Page | aboveR in chip band | header mid-rule h |
|------|---------------------|-------------------|
| TOC p2 | 0 | 3 (=0.75pt) |
| TOC p3–p5 | 0 | 3 (=0.75pt) |
| Glossar p121 | 0 | 2–3 |
| Marktplätze p78 | (number chip owns R) | 2 |

Artifacts: `v4-toc2-head.png`, `v4-toc3-head.png`, `v4-toc2-R.png`.
