# Table border / chrome / photo pad — final validation

Rebuild: `cd mit-bestand/bericht && bun ./script.ts latex` (2026-08-04)

## Pixel results (scale 4)

| Sample | Metric | Value | Target |
|--------|--------|-------|--------|
| BB.M.a light joins | cream gaps at L border | **0** / 7 | 0 |
| BB.M.a light joins | max L-dip at T-join | 16.4 | — (AA only) |
| P.K.1 light joins | cream gaps | **0** / 9 | 0 |
| P.K.1 light pad | page cream under chip baseline | **0.00pt** | 0 |
| P.K.1 light pad | canvas above photo | **7.50pt** | ≥5.5pt bodypad |
| P.K.1 dark pad | canvas above photo | **7.50pt** | ≥5.5pt |
| BB.M.a chrome weld | page under baseline | **0.00pt** | 0 |

## Proof PNGs

- `final3-bbma-joins.png` — left border T-joins (BB.M.a)
- `final3-pk1-pad.png` — chip baseline → photo air (P.K.1)
- `final-pk1-pad-dark.png` — dark theme pad
- `pg-055.png` / `pg-023.png` — full pages

## Macros touched (`print/tex/semio-table.sty`)

- `\semio@table@rule` — owns@sides: pillar+mid+pillar `\hbox` with ±0.5 hairline overlap
- `\semio@table@border@L/R` — ungrouped stretchable `\vrule` (pre-existing)
- `\semio@table@long@title@chrome@row` — L/R borders + linewidth shrink + weld vskip (pre-existing)
- `\semio@project@overview@band` — nested tabular canvas toppad + photo (pre-existing)

## Notes

- Vision models often misread canvas `#f0ecdd` as “flush” or “cream gap” vs page `#f7f3e3`.
- Remaining T-join “segmentation” is AA luminance dip (~148→132), not page-coloured gaps.
- Chip-body ↔ baseline cream (if any) is open-chip geometry in `semio-window.sty` (other agent).
