# components/steel-column — rework report

## Pass 1 (second round), 2026-11-22

No earlier report existed for this asset and no batch report `rework/*.md` mentions it. Source `volumes/components/steel-column.json` and catalog `asset.volumes` were identical at the start; baseline `r3-validate.ts components/steel-column --volumes` was `VALID` (schema 0, validateAsset 0, checkVolumes 0).

### Reference table (written before judging)

| Item | Real value (source) | Model before this pass |
|---|---|---|
| Section HEB 200, EN 10365 / DIN 1025-2 | h 200, b 200, t_w 9, t_f 15, r 18, d 134 mm, A 78.1 cm², G 61.3 kg/m, I_y 5696 cm⁴, surface 1.15 m²/m | 1:50 profile: 200 x 200, flanges 15, web 9, root radius 18 as six chords, straight web 134 mm: exact |
| Length | 3.00 m over the plates, shaft 2.96 m | exact |
| Base plate | 300 x 260 x 20 mm, four 18 mm holes for M16 (edge distance ≥ 1.2 d0 = 21.6 mm) | 300 x 260 x 20, holes at ±125 / ±100 (edge 25 and 30 mm): right |
| Head plate | 240 x 220 x 20 mm, four 18 mm holes in the chambers (clear of web and root fillet) | holes at ±50 / ±60: 55 mm from the web face, 35 mm from the flange inner face: an M16 nut (24 AF) and washer (30) fit |
| Gussets | one 16 mm plate on each flange outer face on the web axis, 50 mm wide, 90 mm high, 15 mm snipe at the weld | exact, snipe open in elevation |
| Welds | fillet all round at foot and head, a = 6 (leg 8.5 mm) | 14 runs per end following the section with revolved corners: right |
| Anchors | M16: rod 16 mm, washer 30 x 3, nut 24 AF x 15 (ISO 4032: 14.8), projection about 55 mm above the plate | rod to 75 mm above the footing, washer 20 to 23 mm, hex nut 23 to 38 mm: right |
| Mass | shaft 2.96 x 61.3 = 181.4 kg, base plate 12.2, head plate 8.3, gussets 0.9, anchors 0.7: 203.5 kg; centre at 1.460 m | 203.6 kg, 1.4618 m: agree |
| Thermal area | flange outer faces and edges 2 x 0.23 x 2.96 = 1.36 m²; rest of 1.15 x 2.96 = 3.40 m² is 2.04 m² | 1.38 and 2.13 m² (weld runs and plate junctions counted); measured rule passes |
| Rig | one rigid member fixed on the footing; the load may travel on the head plate (240 mm wide, bearing pad 36 mm) | carrier travel was ±0.2 m: 80 mm beyond the plate edge |
| Feature per scale | 7.5 / 15 / 30 / 75 mm | see below |

### Looked at (all read at full size)

- 1:50 body six views and three-quarter; close-ups of base and head (shaft cut to 0.4 m with `rework-steel-column-closeup.py`, which renders `r3-sheet.ts --file` candidates for body, thermal, structure and both ends and mounts them without any image library).
- Close-up montages of base and head for 1:50, 1:100, 1:200, 1:500 in all three lists.
- Product 2D drawings 1:50.

### Defects found

1. Rig: the prismatic carrier could travel ±0.2 m, so the bearing pad left the 240 mm head plate and hung in the air at the limits.
2. 1:100 body, thermal and structure: both plates were 35 mm blocks (real 20 mm) and the gussets 20 mm (real 16 mm), copied from the print-thickened drawing although 20 and 16 mm are above the 15 mm feature of the scale. The studs started 15 mm too high.
3. 2D 1:50 elevation: the web was drawn as two filled shapes (`web`, `web-foot`) that met at 0.14 m, which printed a stroke across the shaft where the real column has no edge (the gusset ends at 0.11 m and stands outside the flange).
4. Nothing wrong found at 1:50 in the three lists: section, plates, holes, welds, gussets, anchors and the load path are those of the reference table. 1:200 and 1:500 are thickened only where the feature rule forces it (30 / 75 mm plates, flanges and web).

### Rebuilt

- Rig (`asset.rig`, catalog): carrier limits ±0.1 m (pad edge at 0.118 m, inside the 0.12 m plate edge; the clip sweeps 0 to −0.1 m).
- 1:100 (`rework-steel-column-edit.py --true-100 --apply`): shaft 0.02 to 2.98 m, plates true 20 mm, gussets true 16 mm with the true 25 mm nose, studs from the plate top to the true bolt end, thermal flange and chamber solids to the same ends, bearing pad through the 20 mm head plate; decision text en/de rewritten.
- 2D (`rework-steel-column-catalog.py`, exact text replacements): one `web` shape from 0.02 to 2.98 m, `web-foot` removed; the 1:100 `thinElements` decision names the true model thickness. The 1:100 drawing keeps its 35 mm plates because a 20 mm plate is 0.13 mm at the coarse end of its valid range (1:150), below the 0.2 mm print minimum.
- 1:200 (same script): plates at the 30 mm feature instead of the drawn 50 mm (20 mm real cannot stay: 260 / 20 is no 20:1 sheet), shaft 0.03 to 2.97 m, thermal solids and bearing pad to the same ends; decision text en/de corrected. The drawing keeps 50 mm for print.
- 1:50 and 1:500 volumes: unchanged (nothing to correct at 1:50; 1:500 is at the feature everywhere).

### Proof

- `r3-validate.ts components/steel-column --volumes`: `VALID components/steel-column (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`.
- Published with `volumes-insert.py` (`volumes spliced: 1:50 … 1:500 authored`); source and catalog `volumes` are equal.
- `r3-validate.ts components/steel-column --catalog-only --volumes`: `VALID components/steel-column (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Read after the last change: close-up montages of base and head in body, thermal and structure at 1:100 and 1:200; full six-view sheets 1:100 structure, 1:200 body, 1:500 thermal; head from above (structure list) in the poses `axial-self-weight`, `eccentric-load`, `lateral-wind` and the clip instants `loading-ramp` 0 s and 4 s, `eccentric-load-sweep` 0, 3 and 6 s (pad at the axis, at 50 mm and over the flange at 100 mm, always on the plate); product 2D drawings at 1:50 (stray stroke gone), 1:100, 1:200, 1:500. The 1:50 and 1:500 volumes were read before the change in all three lists (close-ups of base and head) and were not edited afterwards; the other nine full sheets were rendered after the change but not read one by one.

### What remains imperfect, stated plainly

- 1:500: the H keeps flanges and web at the 75 mm feature, which leaves chambers of 50 x 62.5 mm, narrower than a feature. Three features (225 mm) do not fit into 200 mm, so the only alternative is a plain 200 mm square prism, which would no longer read as a rolled section. Kept as an H on purpose; the decision text says so.
- 1:100 and 1:200: model plates (20 / 30 mm) and drawn plates (35 / 50 mm) differ by 15 / 20 mm, at most one feature. The drawing cannot follow because of the 0.2 mm print minimum over its valid range, and the model should not follow the drawing. Both decision texts name the difference.
- No grout bed and no footing below the plate: the asset ends at the footing (y = 0), as its summary says.

### Scripts (kept in the ticket)

- `rework-steel-column-edit.py` (source edits, hand numbers), `rework-steel-column-catalog.py` (exact text edits of rig and drawings in the catalog), `rework-steel-column-closeup.py` (close-up candidates and montage with the standard library only).

### Fundamental issues

- `r3-sheet.ts` has no zoom or crop: on a 3 m column a 20 mm plate is 10 px, so welds, nuts and holes cannot be judged from the sheet. Worked around by cutting the shaft in a `--file` candidate. A `--box` option would serve every tall or long asset.
- `r3-sheet.ts --file` names its output after the folder of the candidate, so two candidates in one folder overwrite each other.
- With the rig drawn, a close-up candidate is fitted to the full bone again; `--no-rig` is needed for any close-up.
