# Scene report: excel / 📋️program / `Beat3_Program`

File: `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`. Plan: `storyboard-excel-program.md` (22 clauses; the built scene has 24, listed below). Audit: `audit-excel-program.md`. Keyframes: `T/🗑️generated/frames/📊️excel/Beat3_Program/{en,de}/<key>.png` with `layout.md`.

## What was built

Left: the Excel window (`xl_window`) over the real Nakagin workbook (`workbook(language)`), always showing the part of a sheet that matters. Right: the Nakagin drawing (capsule plan with fittings, interior box, `tower_axon` painted per capsule) or tile and chart panels. Every number comes from the engine (`evaluate`, `shown`, `apply`, `formula_text`) or from `🏢️nakagin`; window text, formulas (`Capsules[Type]` / `Capsules[Typ]`, `ZÄHLENWENN`, `;`, decimal comma), tab names and error words are localized by the engine and toolkit. Cells appear one at a time by clearing computed cells in a staged workbook and morphing them in with `Window.recalculate`.

Colour meaning (one colour per meaning): type A teal, B orange, C green, D violet (scene-local, because red is reserved); towers light/mid grey; reference frames use the toolkit order; error red only for errors (FALSE / FALSCH, 139 ≠ 140, the trailing-space mark); green for TRUE and totals; yellow for dimension lines and the counter in the hook.

Clauses: 1 `hook` ("In our teaching model ...") · 2 `program` · 3 `sheet` · 4 `area` · 5 `fill` · 6 `net` · 7 `free` (B17 occupants framed, B9 and G16 shown in the frame colours of `=B9-G16`) · 8 `volume` (interior box 2.30 x 3.80 x 2.10 m, frames on B9 and B7, cell close-up) · 9 `types` · 10 `table` (table `Capsules`, row frame, capsule lights) · 11 `countif` (`=COUNTIF(Capsules[Type],A4)`; callout `Capsules[Type] = M2:M141` shown once, linking to the references video) · 12 `rest` · 13 `countifs` (`Capsules[Tower]`, `Capsules[Type]`, mixed references) · 14 `pull` · 15 `xlookup` · 16 `sumproduct` (new cell `Program!B9`, window rows 3 to 9 so both operand ranges B4:B7 and C4:C7 are framed) · 17 `chart` · 18 `towers` (`SUMIF(Capsules[Tower],...,Capsules[Net area])`) · 19 `swap` · 20 `update` (dependency arrows drawn along the lower cell edge, not through values) · 21 `check` · 22 `typo` (red mark after the text in M3) · 23 `fix` · 24 `recap`.

## Changes outside the scene file

`📊️excel/🎬️clip/🧱️model/🔣️.json`, Program sheet only: cells A17:B18 (building count, check), A9:B9 (`=SUMPRODUCT(B4:B7,C4:C7)`, labels "Sum m²" / "Summe m²"), `B16` now `=B9`; shorter headers (Count, Area, Total, Volume, Share) so none is clipped; widths A 14, B 15, C 13, D 17, E 18, F 22 (the summary windows show A:E, the matrix A:F); Occupants row on every type sheet. Engine tests: 83 OK after the edits.

## Audit follow-up

Blockers fixed: `###` totals (widths, A:E window, `#,##0` totals fit), volume now dimensions the interior height 2.10 m (box 2.30 x 3.80 x 2.10 m). Majors fixed: clipped headers, visible trailing-space marker, error red separated from type colours (type D violet), neutral tower tints, trace arrows no longer strike through values, SUMPRODUCT operands framed, hook qualified as "teaching model" (and said once), term "interior height" / "Innenhöhe" used in sheet and narration. Minors fixed: total counter colour neutral, legend chip colours equal outline colours, chart and cell precision one decimal, occupants frame. Not changed: recap chips stay neutral (frame colours would reuse type or error colours); the countif frame colours come from the toolkit reference order; delta chips (+1, -1) stay above the bars; type C window 1.6 m and the Type D 5 degree turn are model and sheet matters (not drawn here).

## Keyframe review notes

All 24 clauses were read in en and de in the final capture. Remaining strict findings (22 en, 24 de) are by design: selection and reference frames lying on cell text, the dashed row frame, wrapped formula bar of long formulas laid over the column letters (toolkit behaviour), the `[@...]` formula of the id column in `table`, and a chart gridline through a value label (de update).

## Tests

- `X/🧪️tests/🐍️.py`: 83 tests OK.
- `X/🧪️tests/🎭️presentation/🐍️.py`: scene contract for Beat3_Program passes in en and de (narration keys equal literal `self.step` keys, 24 clauses, native Cairo lifecycle). The full file also reports 4 errors that belong to other scenes (Beat4_Energy: `'Glide' object has no attribute 'animations'`; Beat1_Anatomy: `zip() argument 2 is shorter`).

## Quality round (2026-10-29)

Implemented from `quality-excel-program.md`: the varying quantity is now the free floor per type (net area is the same shell for all types, said once in `pull`); `Program!A20:D27` holds the block (counts, free floor from `'Type X'!B16`, product column, SUMPRODUCT in B26, XLOOKUP row 27) so SUMPRODUCT (630.5 m²), the chart (185.0 / 154.7 / 110.8 / 179.9 m²) and the swap (+1.9 m²) change visibly. The hook shows the four-colour teaser with 36/35/34/35. The single-capsule lead-in is cut to 7 clauses (area and fill merged); a `row` clause separates creating the table (Format as Table, table name `Capsules` / `Kapseln` from the engine, header tint, banding, marquee) from selecting a row. The typo capsule is ringed on a dimmed-by-lit-set tower with the quoted cell text, the fix shows the cleared state, the narration names data validation. `towers` tints the towers differently (light / blue-grey) with capsule counts 54 and 86. `program` and `xlookup` draw the plan, fittings and person glyph; `pull` shows the four plans with their free floor. Scene-local column widths (`_SceneView`) keep the shared Program widths untouched. Final captures en+de: all 24 frames read; engine tests 84 OK; narration/step contract OK.
